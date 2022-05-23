// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_wallet::account_manager::AccountManager;

use crate::{
    command::account_manager::{
        init_command, new_command, select_command, set_node_command, sync_command, AccountManagerCli,
        AccountManagerCommand,
    },
    error::Error,
};

pub async fn match_account_manager_command(
    account_manager: &AccountManager,
    account_manager_cli: AccountManagerCli,
) -> Result<(), Error> {
    match account_manager_cli.command {
        AccountManagerCommand::Init(mnemonic_url) => {
            init_command(account_manager, mnemonic_url).await?;
        }
        AccountManagerCommand::New { alias } => {
            new_command(account_manager, alias).await?;
        }
        AccountManagerCommand::Select { identifier } => {
            select_command(account_manager, identifier).await?;
        }
        AccountManagerCommand::SetNode { url } => {
            set_node_command(account_manager, url).await?;
        }
        AccountManagerCommand::Sync => {
            sync_command(account_manager).await?;
        }
    }

    Ok(())
}
