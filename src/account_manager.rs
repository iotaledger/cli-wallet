// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::env::var_os;

use iota_wallet::{
    account_manager::AccountManager,
    secret::{stronghold::StrongholdSecretManager, SecretManager},
};

use crate::{
    command::account_manager::{
        backup_command, change_password_command, init_command, new_command, restore_command, set_node_command,
        sync_command, AccountManagerCli, AccountManagerCommand,
    },
    error::Error,
    helper::get_password,
};

pub async fn new_account_manager(cli: AccountManagerCli) -> Result<(AccountManager, Option<String>), Error> {
    let storage_path = var_os("WALLET_DATABASE_PATH").map_or_else(
        || "./stardust-cli-wallet-db".to_string(),
        |os_str| os_str.into_string().expect("invalid WALLET_DATABASE_PATH"),
    );
    let stronghold_path = std::path::Path::new("./stardust-cli-wallet.stronghold");

    let password = get_password("Stronghold password", !stronghold_path.exists())?;
    let secret_manager = SecretManager::Stronghold(
        StrongholdSecretManager::builder()
            .password(&password)
            .try_build(stronghold_path.to_path_buf())?,
    );

    let (account_manager, account) = if let Some(command) = cli.command {
        if let AccountManagerCommand::Init(mnemonic_url) = command {
            (init_command(secret_manager, storage_path, mnemonic_url).await?, None)
        } else {
            let account_manager = AccountManager::builder()
                .with_secret_manager(secret_manager)
                .with_storage_path(&storage_path)
                .finish()
                .await?;
            let mut account = None;

            match command {
                AccountManagerCommand::Backup { path } => backup_command(&account_manager, path).await?,
                AccountManagerCommand::ChangePassword => change_password_command(&account_manager, &password).await?,
                // PANIC: this will never happen because of the if/else.
                AccountManagerCommand::Init(_) => unreachable!(),
                AccountManagerCommand::New { alias } => account = Some(new_command(&account_manager, alias).await?),
                AccountManagerCommand::Restore { path } => restore_command(&account_manager, path).await?,
                AccountManagerCommand::SetNode { url } => set_node_command(&account_manager, url).await?,
                AccountManagerCommand::Sync => sync_command(&account_manager).await?,
            };

            (account_manager, account)
        }
    } else {
        (
            AccountManager::builder()
                .with_secret_manager(secret_manager)
                .with_storage_path(&storage_path)
                .finish()
                .await?,
            None,
        )
    };

    Ok((account_manager, account))
}
