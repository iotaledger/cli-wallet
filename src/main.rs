// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Wallet CLI
//! Create a new account: `$ cargo run -- new --node http://localhost:14265`

use clap::{arg, AppSettings, Arg, ArgMatches, Command, Parser};
use dialoguer::{console::Term, theme::ColorfulTheme, Password, Select};
use iota_wallet::{
    account::AccountHandle,
    account_manager::AccountManager,
    signing::{stronghold::StrongholdSigner, SignerType},
    ClientOptions,
};
// use notify_rust::Notification;
use std::env::var_os;

mod account;

pub type Result<T> = anyhow::Result<T>;

fn print_error<E: ToString>(e: E) {
    println!("ERROR: {}", e.to_string());
}

fn get_password() -> String {
    let mut prompt = Password::new();
    prompt.with_prompt("What's the stronghold password?");
    // if !manager.storage_path().exists() {
    prompt.with_confirmation("Confirm password", "Password mismatch");
    // }

    let password: String = prompt.interact().unwrap();
    password
}

async fn pick_account(accounts: Vec<AccountHandle>) -> Option<usize> {
    let mut items = Vec::new();
    for account_handle in accounts {
        items.push(account_handle.read().await.alias().clone());
    }
    Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select an account to manipulate")
        .items(&items)
        .default(0)
        .interact_on_opt(&Term::stderr())
        .unwrap_or_default()
}

async fn select_account_command(manager: &AccountManager, matches: &ArgMatches) -> Result<Option<AccountHandle>> {
    if let Some(matches) = matches.subcommand_matches("account") {
        let alias = matches.value_of("alias").unwrap();
        if let Ok(account) = manager.get_account(alias).await {
            return Ok(Some(account));
        } else {
            println!("Account not found");
        }
    }
    Ok(None)
}

async fn store_mnemonic_command(manager: &mut AccountManager, matches: &ArgMatches) -> Result<bool> {
    if let Some(matches) = matches.subcommand_matches("mnemonic") {
        let mnemonic = matches.value_of("mnemonic").unwrap().to_string();
        manager
            .get_signer()
            .lock()
            .await
            .store_mnemonic(std::path::Path::new(""), mnemonic)
            .await?;
        println!("Mnemonic stored successfully");
        return Ok(true);
    }
    Ok(false)
}

async fn new_account_command(manager: &AccountManager, matches: &ArgMatches) -> Result<Option<AccountHandle>> {
    if let Some(matches) = matches.subcommand_matches("new") {
        let mut builder = manager.create_account();
        if let Some(alias) = matches.value_of("alias") {
            builder = builder.with_alias(alias.to_string());
        }
        let account_handle = builder.finish().await?;
        println!("Created account `{}`", account_handle.read().await.alias());
        Ok(Some(account_handle))
    } else {
        Ok(None)
    }
}

async fn sync_accounts_command(manager: &AccountManager, matches: &ArgMatches) -> Result<()> {
    if matches.subcommand_matches("sync").is_some() {
        let total_balance = manager.sync(None).await?;
        println!("Synchronized all accounts: {:?}", total_balance);
    }
    Ok(())
}

async fn run() -> Result<()> {
    // ignore stronghold password clear
    // iota_wallet::set_stronghold_password_clear_interval(Duration::from_millis(0)).await;

    let storage_path = var_os("WALLET_DATABASE_PATH")
        .map(|os_str| os_str.into_string().expect("invalid WALLET_DATABASE_PATH"))
        .unwrap_or_else(|| "./wallet-cli-database".to_string());

    let mut password;

    let signer = loop {
        password = get_password();
        match StrongholdSigner::try_new_signer_handle(&password, std::path::Path::new(&format!("./wallet.stronghold")))
        {
            Ok(signer) => break signer,
            Err(err) => println!("{}", err),
        }
    };

    // let nodes: Vec<&str> = matches.values_of("node").map(|v| v.collect()).unwrap_or_default();

    let mut manager = AccountManager::builder(signer)
        .with_client_options(
            ClientOptions::new()
                .with_node("http://localhost:14265")?
                // .with_nodes(&nodes)?
                .with_node_sync_disabled(),
        )
        .with_storage_folder(&storage_path)
        .finish()
        .await?;

    let matches = Command::new("AccountManager")
        .subcommand(
            Command::new("mnemonic")
                .about("mnemonic to store")
                .arg(Arg::new("mnemonic").takes_value(true))
                .arg_required_else_help(false),
        )
        .subcommand(
            Command::new("account")
                .about("Get an existing account")
                .arg(
                    Arg::new("alias")
                        //.long("--alias")
                        .takes_value(true),
                )
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("new")
                .about("Create a new account")
                .arg(Arg::new("alias").takes_value(true))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("sync")
                .about("Syncs all accounts")
                .arg_required_else_help(false),
        )
        // .help_template(CLI_TEMPLATE)
        .get_matches();

    let set_mnemonic = store_mnemonic_command(&mut manager, &matches).await?;

    let account_cli = Command::new("Account cli")
        .subcommand(
            Command::new("exit")
                .about("Exits from the account prompt.")
                .arg_required_else_help(false),
        )
        .subcommand(
            Command::new("sync")
                .about("Synchronizes the account with the Tangle.")
                .arg_required_else_help(false),
        )
        .subcommand(
            Command::new("address")
                .about("Generates an address.")
                .arg_required_else_help(false),
        )
        .subcommand(
            Command::new("balance")
                .about("Gets the account balance.")
                .arg_required_else_help(false),
        )
        .subcommand(
            Command::new("list-addresses")
                .about("List the account addresses.")
                .arg_required_else_help(false),
        )
        .subcommand(
            Command::new("list-transactions")
                .about("List the account transactions.")
                .arg_required_else_help(false),
        )
        .subcommand(
            Command::new("send")
                .about("Send an amount to a bech32 address: `send atoi1qzt0nhsf38nh6rs4p6zs5knqp6psgha9wsv74uajqgjmwc75ugupx3y7x0r 1000000`")
                .arg(Arg::new("address").takes_value(true))
                .arg_required_else_help(true)
                .arg(Arg::new("amount").takes_value(true))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("faucet")
                .about("Request funds from the faucet to the last address, `faucet_url` is optional, default is `http://localhost:14265/api/plugins/faucet/v1/enqueue`")
                .arg(Arg::new("faucet_url").takes_value(true))
        )
        .after_help(
            "Longer explanation to appear after the options when \
                     displaying the help information from --help or -h",
        );

    if std::env::args().len() == 1 {
        let accounts = manager.get_accounts().await?;
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
    sync_accounts_command(&manager, &matches).await?;

    Ok(())
}

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        print_error(e);
    }
}
