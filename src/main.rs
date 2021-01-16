// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Wallet CLI
//! Create a new account: `$ cargo run -- new --node http://localhost:14265`

use clap::{load_yaml, App, AppSettings, ArgMatches};
use dialoguer::{console::Term, theme::ColorfulTheme, Password, Select};
use iota_wallet::{
    account::AccountHandle, account_manager::AccountManager, client::ClientOptionsBuilder, signing::SignerType,
};
use once_cell::sync::OnceCell;
use tokio::runtime::Runtime;

use std::{env::var_os, path::PathBuf, sync::Mutex, time::Duration};

mod account;

const CLI_TEMPLATE: &str = "\
  {before-help}{bin} {version}\n\
  {about-with-newline}\n\
  {usage-heading}\n    {usage}\n\
  \n\
  {all-args}{after-help}\
";

const ACCOUNT_CLI_TEMPLATE: &str = "\
  {all-args}{after-help}\
";

pub type Result<T> = anyhow::Result<T>;

fn print_error<E: ToString>(e: E) {
    println!("ERROR: {}", e.to_string());
}

static RUNTIME: OnceCell<Mutex<Runtime>> = OnceCell::new();

fn get_password(manager: &AccountManager) -> String {
    let mut prompt = Password::new();
    prompt.with_prompt("What's the stronghold password?");
    if !manager.storage_path().exists() {
        prompt.with_confirmation("Confirm password", "Password mismatch");
    }

    let password: String = prompt.interact().unwrap();
    password
}

async fn pick_account(accounts: Vec<AccountHandle>) -> Option<usize> {
    let mut items = Vec::new();
    for account_handle in accounts {
        items.push(account_handle.alias().await);
    }
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select an account to manipulate")
        .items(&items)
        .default(0)
        .interact_on_opt(&Term::stderr())
        .unwrap_or_default();
    if let Some(selected) = selection {
        Some(selected)
    } else {
        None
    }
}

async fn select_account_command(manager: &AccountManager, matches: &ArgMatches) -> Result<Option<AccountHandle>> {
    if let Some(matches) = matches.subcommand_matches("account") {
        let alias = matches.value_of("alias").unwrap();
        if let Some(account) = manager.get_account_by_alias(alias).await {
            return Ok(Some(account));
        } else {
            println!("Account not found");
        }
    }
    Ok(None)
}

async fn store_mnemonic_command(manager: &mut AccountManager, matches: &ArgMatches) -> Result<()> {
    if let Some(matches) = matches.subcommand_matches("mnemonic") {
        let mnemonic = matches.value_of("mnemonic").unwrap().to_string();
        manager.store_mnemonic(SignerType::Stronghold, Some(mnemonic)).await?;
    }
    Ok(())
}

async fn new_account_command(manager: &AccountManager, matches: &ArgMatches) -> Result<Option<AccountHandle>> {
    if let Some(matches) = matches.subcommand_matches("new") {
        let nodes: Vec<&str> = matches
            .values_of("node")
            .expect("at least a node must be provided")
            .collect();
        let mut builder = manager
            .create_account(
                ClientOptionsBuilder::nodes(&nodes)?
                    .local_pow(matches.value_of("pow").unwrap_or("local") == "local")
                    .build()
                    .unwrap(),
            )
            .signer_type(SignerType::Stronghold);
        if let Some(alias) = matches.value_of("alias") {
            builder = builder.alias(alias);
        }
        let account = builder.initialise().await?;
        println!("Created account `{}`", account.alias().await);
        Ok(Some(account))
    } else {
        Ok(None)
    }
}

async fn delete_account_command(manager: &AccountManager, matches: &ArgMatches) -> Result<()> {
    if let Some(matches) = matches.subcommand_matches("delete") {
        let account_alias = matches.value_of("alias").unwrap();
        if let Some(account) = manager.get_account_by_alias(account_alias).await {
            manager.remove_account(&account.id().await).await?;
            println!("Account removed");
        } else {
            println!("Account not found");
        }
    }
    Ok(())
}

async fn sync_accounts_command(manager: &AccountManager, matches: &ArgMatches) -> Result<()> {
    if matches.subcommand_matches("sync").is_some() {
        let synced = manager.sync_accounts().await?;
        println!("Synchronized {} accounts", synced.len());
    }
    Ok(())
}

async fn backup_command(manager: &AccountManager, matches: &ArgMatches) -> Result<()> {
    if let Some(matches) = matches.subcommand_matches("backup") {
        let destination = matches.value_of("path").unwrap();
        let full_path = manager.backup(destination).await?;
        println!("Backup stored at {:?}", full_path);
    }
    Ok(())
}

async fn import_command(manager: &mut AccountManager, matches: &ArgMatches) -> Result<()> {
    if let Some(matches) = matches.subcommand_matches("backup") {
        let source = matches.value_of("path").unwrap();
        let password = get_password(&manager);
        manager.import_accounts(source, password).await?;
        println!("Backup successfully imported");
    }
    Ok(())
}

async fn run() -> Result<()> {
    let runtime = Runtime::new().expect("Failed to create async runtime");
    RUNTIME.set(Mutex::new(runtime)).expect("Failed to store async runtime");

    // ignore stronghold password clear
    iota_wallet::set_stronghold_password_clear_interval(Duration::from_millis(0)).await;

    let storage_path = var_os("WALLET_DATABASE_PATH")
        .map(|os_str| os_str.into_string().expect("invalid WALLET_DATABASE_PATH"))
        .unwrap_or_else(|| "./wallet-cli-database".to_string());
    let mut manager = AccountManager::builder()
        .with_storage_path(&storage_path)
        .finish()
        .await?;

    let password = get_password(&manager);

    manager.set_stronghold_password(password).await?;

    // on first run, we generate a random mnemonic and store it
    if !PathBuf::from(storage_path).join("wallet.stronghold").exists() {
        manager.store_mnemonic(SignerType::Stronghold, None).await?;
    }

    let yaml = load_yaml!("account-cli.yml");
    let account_cli = App::from(yaml)
        .help_template(ACCOUNT_CLI_TEMPLATE)
        .setting(AppSettings::DisableVersion)
        .setting(AppSettings::NoBinaryName);

    if std::env::args().len() == 1 {
        let accounts = manager.get_accounts().await;
        match accounts.len() {
            0 => {}
            1 => {
                account::account_prompt(&account_cli, accounts.first().unwrap().clone()).await;
                return Ok(());
            }
            _ => {
                while let Some(index) = pick_account(accounts.clone()).await {
                    account::account_prompt(&account_cli, accounts[index].clone()).await;
                }
            }
        }
    }

    let yaml = load_yaml!("cli.yml");
    let matches = App::from(yaml)
        .help_template(CLI_TEMPLATE)
        .setting(AppSettings::ColoredHelp)
        .setting(AppSettings::ArgRequiredElseHelp)
        .get_matches();

    store_mnemonic_command(&mut manager, &matches).await?;

    match select_account_command(&manager, &matches).await {
        Ok(Some(account)) => {
            account::account_prompt(&account_cli, account).await;
        }
        Ok(None) => {}
        Err(e) => return Err(e),
    };
    match new_account_command(&manager, &matches).await {
        Ok(Some(new_account_handle)) => {
            account::account_prompt(&account_cli, new_account_handle).await;
        }
        Ok(None) => {}
        Err(e) => return Err(e),
    };
    delete_account_command(&manager, &matches).await?;
    sync_accounts_command(&manager, &matches).await?;
    backup_command(&manager, &matches).await?;
    import_command(&mut manager, &matches).await?;

    Ok(())
}

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        print_error(e);
    }
}
