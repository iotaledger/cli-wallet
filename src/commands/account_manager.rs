// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{account::account_prompt, Result};
use clap::{Args, Parser, Subcommand};
use iota_wallet::{account_manager::AccountManager, iota_client::utils::generate_mnemonic, ClientOptions};

#[derive(Parser)]
#[clap(version, long_about = None)]
#[clap(propagate_version = true)]
pub struct AccountManagerCli {
    #[clap(subcommand)]
    pub command: AccountManagerCommands,
}

#[derive(Subcommand)]
pub enum AccountManagerCommands {
    /// Get an existing account with the alias or account index
    Get { identifier: String },
    /// Initialize the wallet with a mnemonic and node url, if nothing is provided, a new mnemonic will be generated and "http://localhost:14265" used
    Init(MnemonicAndUrl),
    /// Create a new account with an optional alias
    New { alias: Option<String> },
    /// Set the node to use
    SetNode { url: String },
    /// Sync all accounts
    Sync,
}

#[derive(Args)]
pub struct MnemonicAndUrl {
    #[clap(short, long)]
    pub mnemonic: Option<String>,
    #[clap(short, long)]
    pub node: Option<String>,
}

pub async fn select_account_command(manager: &AccountManager, identifier: String) -> Result<()> {
    if let Ok(account) = manager.get_account(identifier).await {
        account_prompt(account).await;
        return Ok(());
    } else {
        println!("Account not found");
    }
    Ok(())
}

pub async fn init_command(manager: &AccountManager, mnemonic_url: MnemonicAndUrl) -> Result<()> {
    if let Some(node) = mnemonic_url.node {
        manager
            .set_client_options(ClientOptions::new().with_node(&node)?)
            .await?;
    }

    let mnemonic = match mnemonic_url.mnemonic {
        Some(mnemonic) => mnemonic,
        None => generate_mnemonic()?,
    };
    println!(
        "**Important** write this mnemonic phrase in a safe place.
        It is the only way to recover your account if you ever forget your password/lose the .stronghold file."
    );
    println!("////////////////////////////\n");
    println!("{}", mnemonic);
    println!("\n////////////////////////////");

    manager
        .get_signer()
        .lock()
        .await
        .store_mnemonic(std::path::Path::new(""), mnemonic)
        .await?;
    println!("Mnemonic stored successfully");
    Ok(())
}

pub async fn new_account_command(manager: &AccountManager, alias: Option<String>) -> Result<()> {
    let mut builder = manager.create_account();
    if let Some(alias) = alias {
        builder = builder.with_alias(alias);
    }
    let account_handle = builder.finish().await?;
    println!("Created account `{}`", account_handle.read().await.alias());
    account_prompt(account_handle).await;
    Ok(())
}

pub async fn sync_accounts_command(manager: &AccountManager) -> Result<()> {
    let total_balance = manager.sync(None).await?;
    println!("Synchronized all accounts: {:?}", total_balance);
    Ok(())
}
