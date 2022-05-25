// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Stardust CLI Wallet
//! Create a new account: `cargo run init --node http://node.url:port --mnemonic MNEMONIC`

mod account;
mod account_manager;
mod command;
mod error;
mod helper;

use fern_logger::{LoggerConfigBuilder, LoggerOutputConfigBuilder};
use log::LevelFilter;

use self::{account_manager::new_account_manager, error::Error, helper::pick_account};

async fn run() -> Result<(), Error> {
    let account_manager = new_account_manager().await?;

    match std::env::args().len() {
        1 => {
            // Show the account selector
            if let Some(index) = pick_account(account_manager.get_accounts().await?).await {
                account::account_prompt(account_manager.get_account(index as u32).await?).await?;
            }
        }
        2 => {
            // If only one argument from the user is provided, try to use it as identifier
            let mut iter = std::env::args();
            // The first element is traditionally the path of the executable
            iter.next();
            if let Some(identifier) = iter.next() {
                if let Ok(account_handle) = account_manager.get_account(identifier).await {
                    account::account_prompt(account_handle).await?;
                }
            }
        }
        _ => {}
    }

    let accounts = account_manager.get_accounts().await?;
    if !accounts.is_empty() {
        loop {
            // Show the account selector
            if let Some(index) = pick_account(accounts.clone()).await {
                account::account_prompt(account_manager.get_account(index as u32).await?).await?;
            }
        }
    }

    Ok(())
}

fn logger_init() -> Result<(), Error> {
    let target_exclusions = ["rustls"];
    let stdout = LoggerOutputConfigBuilder::default()
        .name("stdout")
        .level_filter(LevelFilter::Debug)
        .target_exclusions(&target_exclusions)
        .color_enabled(true);
    let config = LoggerConfigBuilder::default().with_output(stdout).finish();

    fern_logger::logger_init(config)?;

    Ok(())
}

#[tokio::main]
async fn main() {
    if let Err(e) = logger_init() {
        println!("{e}");
        return;
    }

    match run().await {
        Ok(_) | Err(Error::Help) => {}
        Err(e) => log::error!("{e}"),
    }
}
