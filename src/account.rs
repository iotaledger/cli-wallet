// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use clap::Parser;
use dialoguer::Input;
use iota_wallet::account::AccountHandle;

use crate::{
    command::account::{
        addresses_command, balance_command, claim_command, consolidate_command, faucet_command,
        mint_native_token_command, mint_nft_command, new_address_command, send_command, send_micro_command,
        send_native_token_command, send_nft_command, sync_command, transactions_command, AccountCli, AccountCommand,
    },
    error::Error,
};

// loop on the account prompt
pub async fn account_prompt(account_handle: AccountHandle) -> Result<(), Error> {
    loop {
        if account_prompt_internal(account_handle.clone()).await? {
            return Ok(());
        }
    }
}

// loop on the account prompt
pub async fn account_prompt_internal(account_handle: AccountHandle) -> Result<bool, Error> {
    let alias = {
        let account = account_handle.read().await;
        account.alias().clone()
    };
    let command: String = Input::new()
        .with_prompt(format!("Account \"{}\"", alias))
        .interact_text()?;

    match command.as_str() {
        "h" => {
            if let Err(err) = AccountCli::try_parse_from(vec!["Account:", "help"]) {
                println!("{err}");
            }
        }
        "clear" => {
            // Clear console
            let _ = std::process::Command::new("clear").status();
        }
        _ => {
            // Prepend `Account: ` so the parsing will be correct
            let command = format!("Account: {}", command.trim());
            let account_cli = match AccountCli::try_parse_from(command.split(' ')) {
                Ok(account_cli) => account_cli,
                Err(err) => {
                    println!("{err}");
                    return Ok(false);
                }
            };
            if let Err(err) = match account_cli.command {
                AccountCommand::NewAddress => new_address_command(&account_handle).await,
                AccountCommand::Balance => balance_command(&account_handle).await,
                AccountCommand::Claim => claim_command(&account_handle).await,
                AccountCommand::Consolidate => consolidate_command(&account_handle).await,
                AccountCommand::Exit => {
                    return Ok(true);
                }
                AccountCommand::Faucet { url, address } => faucet_command(&account_handle, url, address).await,
                AccountCommand::Addresses => addresses_command(&account_handle).await,
                AccountCommand::Transactions => transactions_command(&account_handle).await,
                AccountCommand::MintNativeToken {
                    maximum_supply,
                    foundry_metadata,
                } => mint_native_token_command(&account_handle, maximum_supply, foundry_metadata).await,
                AccountCommand::MintNft {
                    address,
                    immutable_metadata,
                    metadata,
                } => mint_nft_command(&account_handle, address, immutable_metadata, metadata).await,
                AccountCommand::Send { address, amount } => send_command(&account_handle, address, amount).await,
                AccountCommand::SendMicro { address, amount } => {
                    send_micro_command(&account_handle, address, amount).await
                }
                AccountCommand::SendNativeToken {
                    address,
                    token_id,
                    amount,
                } => send_native_token_command(&account_handle, address, token_id, amount).await,
                AccountCommand::SendNft { address, nft_id } => send_nft_command(&account_handle, address, nft_id).await,
                AccountCommand::Sync => sync_command(&account_handle).await,
            } {
                log::error!("{}", err);
            }
        }
    }

    Ok(false)
}
