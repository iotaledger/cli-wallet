// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Stardust CLI Wallet
//! Create a new account: `cargo run init --node http://node.url:port --mnemonic MNEMONIC`

use anyhow::Result;
use clap::Parser;
use iota_wallet::{account_manager::AccountManager, signing::stronghold::StrongholdSigner, ClientOptions};
use std::env::var_os;

mod account;
mod account_manager;
mod commands;
mod helpers;
use account_manager::match_account_manager_command;
use commands::account_manager::AccountManagerCli;
use helpers::{get_password, help_command, pick_account};

async fn run() -> Result<()> {
    // Print help overview and exit before showing the password prompt
    help_command();

    let storage_path = var_os("WALLET_DATABASE_PATH")
        .map(|os_str| os_str.into_string().expect("invalid WALLET_DATABASE_PATH"))
        .unwrap_or_else(|| "./stardust-cli-wallet-db".to_string());

    let stronghold_path = std::path::Path::new("./stardust-cli-wallet.stronghold");
    let password = get_password(stronghold_path);
    let signer = StrongholdSigner::builder()
        .password(&password)
        .snapshot_path(stronghold_path.to_path_buf())
        .build();

    let account_manager = AccountManager::builder()
        .with_signer(signer.into())
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
