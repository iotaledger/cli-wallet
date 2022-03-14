// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{
    commands::account_manager::{
        init_command, new_account_command, select_account_command, sync_accounts_command, AccountManagerCli,
        AccountManagerCommands,
    },
    Result,
};
use iota_wallet::{account_manager::AccountManager, ClientOptions};

pub async fn match_account_manager_command(
    account_manager: &AccountManager,
    account_manager_cli: AccountManagerCli,
) -> Result<()> {
    match account_manager_cli.command {
        AccountManagerCommands::Init(mnemonic_url) => {
            init_command(account_manager, mnemonic_url).await?;
        }
        AccountManagerCommands::Sync => {
            sync_accounts_command(account_manager).await?;
        }
        AccountManagerCommands::SetNode { url } => {
            account_manager
                .set_client_options(ClientOptions::new().with_node(&url)?)
                .await?;
        }
        AccountManagerCommands::New { alias } => {
            new_account_command(account_manager, alias).await?;
        }
        AccountManagerCommands::Get { identifier } => {
            select_account_command(account_manager, identifier).await?;
        }
    }
    Ok(())
}
