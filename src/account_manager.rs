// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::env::var_os;

use clap::Parser;
use iota_wallet::{
    account_manager::AccountManager,
    secret::{stronghold::StrongholdSecretManager, SecretManager},
    ClientOptions,
};

use crate::{
    command::account_manager::{
        init_command, new_command, select_command, set_node_command, sync_command, AccountManagerCli,
        AccountManagerCommand,
    },
    error::Error,
    helper::get_password,
};

pub async fn new_account_manager() -> Result<AccountManager, Error> {
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
            let account_manager = AccountManager::builder()
                .with_secret_manager(secret_manager)
                .with_client_options(
                    ClientOptions::new()
                        .with_node("http://localhost:14265")?
                        .with_node_sync_disabled(),
                )
                .with_storage_path(&storage_path)
                .finish()
                .await?;

            match account_manager_cli.command {
                AccountManagerCommand::Init(mnemonic_url) => init_command(&account_manager, mnemonic_url).await,
                AccountManagerCommand::New { alias } => new_command(&account_manager, alias).await,
                AccountManagerCommand::Select { identifier } => select_command(&account_manager, identifier).await,
                AccountManagerCommand::SetNode { url } => set_node_command(&account_manager, url).await,
                AccountManagerCommand::Sync => sync_command(&account_manager).await,
            }?;

            Ok(account_manager)
        }
        Err(e) => {
            println!("{e}");
            Err(Error::Help)
        }
    }
}
