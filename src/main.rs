// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod account;
mod account_manager;
mod command;
mod error;
mod helper;

use fern_logger::{LoggerConfigBuilder, LoggerOutputConfigBuilder};
use log::LevelFilter;

use self::{account_manager::new_account_manager, error::Error, helper::pick_account};

fn logger_init() -> Result<(), Error> {
    let stdout = LoggerOutputConfigBuilder::default()
        .name("stdout")
        .level_filter(LevelFilter::Debug)
        .target_exclusions(&["rustls"])
        .color_enabled(true);
    let archive = LoggerOutputConfigBuilder::default()
        .name("archive.log")
        .level_filter(LevelFilter::Debug)
        .target_exclusions(&["mnemonic", "rustls"])
        .color_enabled(false);
    let config = LoggerConfigBuilder::default()
        .with_output(stdout)
        .with_output(archive)
        .finish();

    fern_logger::logger_init(config)?;

    Ok(())
}

async fn run() -> Result<(), Error> {
    let (account_manager, account) = new_account_manager().await?;

    match account {
        Some(account) => account::account_prompt(account_manager.get_account(account).await?).await?,
        None => {
            if let Some(account) = pick_account(&account_manager).await? {
                account::account_prompt(account_manager.get_account(account).await?).await?;
            }
        }
    }

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
