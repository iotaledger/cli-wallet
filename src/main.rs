//! Wallet CLI
//! Create a new account: `$ cargo run -- new --node http://localhost:14265`

use clap::{load_yaml, App, AppSettings, ArgMatches};
use console::Term;
use dialoguer::{theme::ColorfulTheme, Select};
use iota_wallet::{
    account::Account, account_manager::AccountManager, client::ClientOptionsBuilder,
    signing::SignerType, storage::sqlite::SqliteStorageAdapter, Result, WalletError,
};
use once_cell::sync::OnceCell;
use tokio::runtime::Runtime;

use std::{env::var_os, sync::Mutex};

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

fn print_error(e: WalletError) {
    println!("ERROR: {}", e.to_string());
}

static RUNTIME: OnceCell<Mutex<Runtime>> = OnceCell::new();

pub fn block_on<C: futures::Future>(cb: C) -> C::Output {
    let runtime = RUNTIME.get().unwrap();
    runtime.lock().unwrap().block_on(cb)
}

fn pick_account(account_cli: &App<'_>, accounts: Vec<Account>) -> Result<()> {
    let items: Vec<&String> = accounts.iter().map(|acc| acc.alias()).collect();
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select an account to manipulate")
        .items(&items)
        .default(0)
        .interact_on_opt(&Term::stderr())?;
    if let Some(selected) = selection {
        account::account_prompt(account_cli, accounts[selected].clone());
        pick_account(account_cli, accounts)?;
    }
    Ok(())
}

fn select_account_command(account_cli: &App<'_>, manager: &AccountManager, matches: &ArgMatches) {
    if let Some(matches) = matches.subcommand_matches("account") {
        let alias = matches.value_of("alias").unwrap();
        if let Some(account) = manager.get_account_by_alias(alias) {
            account::account_prompt(account_cli, account);
        } else {
            println!("Account not found");
        }
    }
}

fn new_account_command(
    account_cli: &App<'_>,
    manager: &AccountManager,
    matches: &ArgMatches,
) -> Result<()> {
    if let Some(matches) = matches.subcommand_matches("new") {
        let nodes: Vec<&str> = matches
            .values_of("node")
            .expect("at least a node must be provided")
            .collect();
        let accounts = manager.get_accounts()?;
        let mut builder = manager
            .create_account(ClientOptionsBuilder::nodes(&nodes)?.build().unwrap())
            .signer_type(SignerType::EnvMnemonic);
        if let Some(alias) = matches.value_of("alias") {
            builder = builder.alias(alias);
        }
        if let Some(mnemonic) = matches.value_of("mnemonic") {
            builder = builder.mnemonic(mnemonic);
        } else if accounts.is_empty() {
            if let Some(mnemonic) = var_os("IOTA_WALLET_MNEMONIC") {
                builder = builder.mnemonic(
                    mnemonic
                        .to_str()
                        .expect("invalid IOTA_WALLET_MNEMONIC env")
                        .to_string(),
                );
            } else {
                let mnemonic =
                    bip39::Mnemonic::new(bip39::MnemonicType::Words24, bip39::Language::English);
                println!("Your mnemonic is `{:?}`, you must store it on an environment variable called `IOTA_WALLET_MNEMONIC` to use this CLI", mnemonic.phrase());
                builder = builder.mnemonic(mnemonic.into_phrase());
            }
        }
        let account = builder.initialise()?;
        println!("Created account `{}`", account.alias());
        account::account_prompt(account_cli, account);
    }
    Ok(())
}

fn delete_account_command(manager: &AccountManager, matches: &ArgMatches) -> Result<()> {
    if let Some(matches) = matches.subcommand_matches("delete") {
        let account_alias = matches.value_of("alias").unwrap();
        if let Some(account) = manager.get_account_by_alias(account_alias) {
            manager.remove_account(account.id().into())?;
            println!("Account removed");
        } else {
            println!("Account not found");
        }
    }
    Ok(())
}

fn sync_accounts_command(manager: &AccountManager, matches: &ArgMatches) -> Result<()> {
    if matches.subcommand_matches("sync").is_some() {
        let synced = block_on(async move { manager.sync_accounts().await })?;
        println!("Synchronized {} accounts", synced.len());
    }
    Ok(())
}

fn backup_command(manager: &AccountManager, matches: &ArgMatches) -> Result<()> {
    if let Some(matches) = matches.subcommand_matches("backup") {
        let destination = matches.value_of("path").unwrap();
        let full_path = manager.backup(destination)?;
        println!("Backup stored at {:?}", full_path);
    }
    Ok(())
}

fn import_command(manager: &AccountManager, matches: &ArgMatches) -> Result<()> {
    if let Some(matches) = matches.subcommand_matches("backup") {
        let source = matches.value_of("path").unwrap();
        manager.import_accounts(source)?;
        println!("Backup successfully imported");
    }
    Ok(())
}

fn run() -> Result<()> {
    let runtime = Runtime::new().expect("Failed to create async runtime");
    RUNTIME
        .set(Mutex::new(runtime))
        .expect("Failed to store async runtime");

    let storage_path = var_os("WALLET_DATABASE_PATH")
        .map(|os_str| os_str.into_string().expect("invalid WALLET_DATABASE_PATH"))
        .unwrap_or_else(|| "./wallet-cli-database".to_string());
    let mut manager = AccountManager::with_storage_adapter(
        &storage_path,
        SqliteStorageAdapter::new(&storage_path, "accounts")?,
    )?;

    let yaml = load_yaml!("account-cli.yml");
    let account_cli = App::from(yaml)
        .help_template(ACCOUNT_CLI_TEMPLATE)
        .setting(AppSettings::DisableVersion)
        .setting(AppSettings::NoBinaryName);

    if std::env::args().len() == 1 {
        let accounts = manager.get_accounts()?;
        match accounts.len() {
            0 => {}
            1 => account::account_prompt(&account_cli, accounts.first().unwrap().clone()),
            _ => pick_account(&account_cli, accounts)?,
        }
    }

    let yaml = load_yaml!("cli.yml");
    let matches = App::from(yaml)
        .help_template(CLI_TEMPLATE)
        .setting(AppSettings::ColoredHelp)
        .setting(AppSettings::ArgRequiredElseHelp)
        .get_matches();

    select_account_command(&account_cli, &manager, &matches);
    new_account_command(&account_cli, &manager, &matches)?;
    delete_account_command(&manager, &matches)?;
    sync_accounts_command(&manager, &matches)?;
    backup_command(&manager, &matches)?;
    import_command(&manager, &matches)?;

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        print_error(e);
    }
}
