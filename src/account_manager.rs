// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_wallet::{account_manager::AccountManager, ClientOptions};

use crate::{
    command::account_manager::{
        init_command, new_account_command, select_account_command, sync_accounts_command, AccountManagerCli,
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
        AccountManagerCommand::Sync => {
            sync_accounts_command(account_manager).await?;
        }
        AccountManagerCommand::SetNode { url } => {
            account_manager
                .set_client_options(ClientOptions::new().with_node(&url)?.with_node_sync_disabled())
                .await?;
        }
        AccountManagerCommand::New { alias } => {
            new_account_command(account_manager, alias).await?;
        }
        AccountManagerCommand::Get { identifier } => {
            select_account_command(account_manager, identifier).await?;
        }
    }

    Ok(())
}
