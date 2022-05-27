// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::env::var_os;

use clap::Parser;
use iota_wallet::{
    account_manager::AccountManager,
    secret::{stronghold::StrongholdSecretManager, SecretManager},
};

use crate::{
    command::account_manager::{
        init_command, new_command, set_node_command, sync_command, AccountManagerCli, AccountManagerCommand,
    },
    error::Error,
    helper::get_password,
};

pub async fn new_account_manager() -> Result<(AccountManager, Option<String>), Error> {
    let storage_path = var_os("WALLET_DATABASE_PATH")
        .map(|os_str| os_str.into_string().expect("invalid WALLET_DATABASE_PATH"))
        .unwrap_or_else(|| "./stardust-cli-wallet-db".to_string());
    let stronghold_path = std::path::Path::new("./stardust-cli-wallet.stronghold");

    match AccountManagerCli::try_parse() {
        Ok(account_manager_cli) => {
            let password = get_password(stronghold_path)?;
            let secret_manager = SecretManager::Stronghold(
                StrongholdSecretManager::builder()
                    .password(&password)
                    .snapshot_path(stronghold_path.to_path_buf())
                    .build(),
            );

            let account_manager = if let Some(command) = account_manager_cli.command {
                if let AccountManagerCommand::Init(mnemonic_url) = command {
                    init_command(secret_manager, storage_path, mnemonic_url).await?
                } else {
                    let account_manager = AccountManager::builder()
                        .with_secret_manager(secret_manager)
                        .with_storage_path(&storage_path)
                        .finish()
                        .await?;

                    match command {
                        // PANIC: this will never happen because of the if/else.
                        AccountManagerCommand::Init(_) => unreachable!(),
                        AccountManagerCommand::New { alias } => new_command(&account_manager, alias).await,
                        AccountManagerCommand::SetNode { url } => set_node_command(&account_manager, url).await,
                        AccountManagerCommand::Sync => sync_command(&account_manager).await,
                    }?;

                    account_manager
                }
            } else {
                AccountManager::builder()
                    .with_secret_manager(secret_manager)
                    .with_storage_path(&storage_path)
                    .finish()
                    .await?
            };

            Ok((account_manager, account_manager_cli.account))
        }
        Err(e) => {
            println!("{e}");
            Err(Error::Help)
        }
    }
}
