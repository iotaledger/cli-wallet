// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Wallet CLI
//! Create a new account: `$ cargo run -- new --node http://localhost:14265`

use clap::{load_yaml, App, AppSettings, ArgMatches};
use console::Term;
use dialoguer::{theme::ColorfulTheme, Confirm, Select};
use dotenv::dotenv;
use iota_wallet::{
    account::{Account, AccountIdentifier},
    account_manager::AccountManager,
    client::ClientOptionsBuilder,
    signing::SignerType,
    storage::sqlite::SqliteStorageAdapter,
    Result, WalletError,
};
use once_cell::sync::OnceCell;
use tokio::runtime::Runtime;

use std::{
    env::var_os,
    fs::OpenOptions,
    io::Write,
    sync::{Arc, Mutex, RwLock},
};

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

fn pick_account(account_cli: &App<'_>, accounts: Arc<RwLock<Vec<Account>>>) -> Result<()> {
    let items: Vec<String> = {
        let accounts_ = accounts.read().unwrap();
        accounts_.iter().map(|acc| acc.alias().clone()).collect()
    };
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select an account to manipulate")
        .items(&items)
        .default(0)
        .interact_on_opt(&Term::stderr())?;
    if let Some(selected) = selection {
        account::account_prompt(account_cli, accounts.clone(), selected);
        pick_account(account_cli, accounts)?;
    }
    Ok(())
}

fn select_account_command(manager: &AccountManager, matches: &ArgMatches) -> Result<Option<AccountIdentifier>> {
    if let Some(matches) = matches.subcommand_matches("account") {
        let alias = matches.value_of("alias").unwrap();
        if let Some(account) = manager.get_account_by_alias(alias) {
            return Ok(Some(account.id().clone()));
        } else {
            println!("Account not found");
        }
    }
    Ok(None)
}

fn new_account_command(manager: &AccountManager, matches: &ArgMatches) -> Result<Option<Account>> {
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
                builder = builder.mnemonic(mnemonic.to_str().expect("invalid IOTA_WALLET_MNEMONIC env").to_string());
            } else {
                let mnemonic = bip39::Mnemonic::new(bip39::MnemonicType::Words24, bip39::Language::English);
                println!("Your mnemonic is `{:?}`, you must store it on an environment variable called `IOTA_WALLET_MNEMONIC` to use this CLI", mnemonic.phrase());
                if let Ok(flag) = Confirm::new()
                    .with_prompt("Do you want to store the mnemonic in a .env file?")
                    .interact()
                {
                    if flag {
                        let mut file = OpenOptions::new().append(true).create(true).open(".env")?;
                        writeln!(file, r#"IOTA_WALLET_MNEMONIC="{}""#, mnemonic.phrase())?;
                        println!("mnemonic added to {:?}", std::env::current_dir()?.join(".env"));
                    }
                }
                builder = builder.mnemonic(mnemonic.into_phrase());
            }
        }
        let account = builder.initialise()?;
        println!("Created account `{}`", account.alias());
        Ok(Some(account))
    } else {
        Ok(None)
    }
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

fn watch_accounts(accounts: Arc<RwLock<Vec<Account>>>) {
    let accounts_ = accounts.clone();
    iota_wallet::event::on_balance_change(move |event| {
        let mut accounts_ = accounts_.write().unwrap();
        let account = accounts_.iter_mut().find(|a| &a.id() == event.account_id()).unwrap();
        let account_address = account
            .addresses_mut()
            .iter_mut()
            .find(|a| a == event.address())
            .unwrap();
        account_address.set_balance(*event.balance());
    });

    let accounts_ = accounts.clone();
    iota_wallet::event::on_new_transaction(move |event| {
        let mut accounts_ = accounts_.write().unwrap();
        let account = accounts_.iter_mut().find(|a| &a.id() == event.account_id()).unwrap();
        account.append_messages(vec![event.cloned_message()]);
    });

    let accounts_ = accounts.clone();
    iota_wallet::event::on_confirmation_state_change(move |event| {
        let mut accounts_ = accounts_.write().unwrap();
        let account = accounts_.iter_mut().find(|a| &a.id() == event.account_id()).unwrap();
        if let Some(message) = account.messages_mut().iter_mut().find(|m| m == event.message()) {
            message.set_confirmed(*event.confirmed());
        }
    });

    let accounts_ = accounts.clone();
    iota_wallet::event::on_reattachment(move |event| {
        let mut accounts_ = accounts_.write().unwrap();
        let account = accounts_.iter_mut().find(|a| &a.id() == event.account_id()).unwrap();
        account.append_messages(vec![event.cloned_message()]);
    });

    let accounts_ = accounts.clone();
    iota_wallet::event::on_broadcast(move |event| {
        let mut accounts_ = accounts_.write().unwrap();
        let account = accounts_.iter_mut().find(|a| &a.id() == event.account_id()).unwrap();
        if let Some(message) = account.messages_mut().iter_mut().find(|m| m == event.message()) {
            message.set_broadcasted(true);
        }
    });
}

fn run() -> Result<()> {
    let runtime = Runtime::new().expect("Failed to create async runtime");
    RUNTIME.set(Mutex::new(runtime)).expect("Failed to store async runtime");

    let storage_path = var_os("WALLET_DATABASE_PATH")
        .map(|os_str| os_str.into_string().expect("invalid WALLET_DATABASE_PATH"))
        .unwrap_or_else(|| "./wallet-cli-database".to_string());
    let mut manager =
        AccountManager::with_storage_adapter(&storage_path, SqliteStorageAdapter::new(&storage_path, "accounts")?)?;

    manager.start_background_sync();

    let yaml = load_yaml!("account-cli.yml");
    let account_cli = App::from(yaml)
        .help_template(ACCOUNT_CLI_TEMPLATE)
        .setting(AppSettings::DisableVersion)
        .setting(AppSettings::NoBinaryName);

    let accounts = manager.get_accounts()?;
    let accounts = Arc::new(RwLock::new(accounts));

    watch_accounts(accounts.clone());

    if std::env::args().len() == 1 {
        let len = {
            let a = accounts.read().unwrap();
            a.len()
        };
        match len {
            0 => {}
            1 => account::account_prompt(&account_cli, accounts.clone(), 0),
            _ => pick_account(&account_cli, accounts.clone())?,
        }
    }

    let yaml = load_yaml!("cli.yml");
    let matches = App::from(yaml)
        .help_template(CLI_TEMPLATE)
        .setting(AppSettings::ColoredHelp)
        .setting(AppSettings::ArgRequiredElseHelp)
        .get_matches();

    match select_account_command(&manager, &matches) {
        Ok(Some(selected_account_id)) => {
            let index = {
                let accounts_ = accounts.read().unwrap();
                accounts_.iter().position(|a| a.id() == &selected_account_id).unwrap()
            };
            account::account_prompt(&account_cli, accounts.clone(), index);
        }
        Ok(None) => {}
        Err(e) => return Err(e),
    };
    match new_account_command(&manager, &matches) {
        Ok(Some(new_account)) => {
            let index = {
                let mut accounts_ = accounts.write().unwrap();
                accounts_.push(new_account);
                accounts_.len() - 1
            };
            account::account_prompt(&account_cli, accounts.clone(), index);
        }
        Ok(None) => {}
        Err(e) => return Err(e),
    };
    delete_account_command(&manager, &matches)?;
    sync_accounts_command(&manager, &matches)?;
    backup_command(&manager, &matches)?;
    import_command(&manager, &matches)?;

    Ok(())
}

fn main() {
    if let Ok(p) = dotenv() {
        println!("loaded dotenv from {:?}", p);
    }
    if let Err(e) = run() {
        print_error(e);
    }
}
