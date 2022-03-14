// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Wallet CLI
//! Create a new account: `$ cargo run -- new --node http://localhost:14265`

use clap::Parser;
use iota_wallet::{account_manager::AccountManager, signing::stronghold::StrongholdSigner, ClientOptions};
use std::env::var_os;

mod account;
mod account_manager;
mod commands;
mod helpers;
use account_manager::match_account_manager_command;
use commands::account_manager::AccountManagerCli;
use helpers::{get_password, pick_account};

pub type Result<T> = anyhow::Result<T>;

async fn run() -> Result<()> {
    let storage_path = var_os("WALLET_DATABASE_PATH")
        .map(|os_str| os_str.into_string().expect("invalid WALLET_DATABASE_PATH"))
        .unwrap_or_else(|| "./stardust-cli-wallet-db".to_string());

    let stronghold_path = std::path::Path::new("./stardust-cli-wallet.stronghold");
    let signer = loop {
        let password = get_password(stronghold_path);
        match StrongholdSigner::try_new_signer_handle(&password, stronghold_path) {
            Ok(signer) => break signer,
            Err(err) => println!("{}", err),
        }
    };

    let account_manager = AccountManager::builder(signer)
        .with_client_options(
            ClientOptions::new()
                .with_node("http://localhost:14265")?
                .with_node_sync_disabled(),
        )
        .with_storage_folder(&storage_path)
        .finish()
        .await?;

    if let Ok(account_manager_cli) = AccountManagerCli::try_parse() {
        match_account_manager_command(&account_manager, account_manager_cli).await?;
    }

    match std::env::args().len() {
        1 => {
            // Show the account selector
            if let Some(index) = pick_account(account_manager.get_accounts().await?).await {
                account::account_prompt(account_manager.get_account(index as u32).await?).await;
            }
        }
        2 => {
            // If only one argument from the user is provided, try to use it as identifier
            let mut iter = std::env::args();
            // The first element is traditionally the path of the executable
            iter.next();
            if let Some(identifier) = iter.next() {
                if let Ok(account_handle) = account_manager.get_account(identifier).await {
                    account::account_prompt(account_handle).await;
                }
            }
        }
        _ => {}
    }

    // This will print the help message if parsing fails
    AccountManagerCli::parse();

    let accounts = account_manager.get_accounts().await?;
    if !accounts.is_empty() {
        loop {
            // Show the account selector
            if let Some(index) = pick_account(accounts.clone()).await {
                account::account_prompt(account_manager.get_account(index as u32).await?).await;
            }
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        println!("ERROR: {e}");
    }
}
