// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use clap::Parser;
use dialoguer::Input;
use iota_wallet::account::AccountHandle;

use crate::commands::account::{
    balance_command, faucet_command, generate_address_command, list_addresses_command, list_transactions_command,
    mint_native_token_command, mint_nft_command, send_command, send_micro_command, send_native_command,
    send_nft_command, sync_account_command, AccountCli, AccountCommands,
};

// loop on the account prompt
pub async fn account_prompt(account_handle: AccountHandle) {
    loop {
        let exit = account_prompt_internal(account_handle.clone()).await;
        if exit {
            break;
        }
    }
}

// loop on the account prompt
pub async fn account_prompt_internal(account_handle: AccountHandle) -> bool {
    let alias = {
        let account = account_handle.read().await;
        account.alias().clone()
    };
    let command: String = Input::new()
        .with_prompt(format!("Account `{}` command (h for help)", alias))
        .interact_text()
        .unwrap();

    match command.as_str() {
        "h" => {
            if let Err(err) = AccountCli::try_parse_from(vec!["Account:", "help"]) {
                let _ = err.print();
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
                Err(e) => {
                    let _ = e.print();
                    return false;
                }
            };
            if let Err(err) = match account_cli.command {
                AccountCommands::Address => generate_address_command(&account_handle).await,
                AccountCommands::Balance => balance_command(&account_handle).await,
                AccountCommands::Exit => {
                    return true;
                }
                AccountCommands::Faucet { url } => faucet_command(&account_handle, url).await,
                AccountCommands::ListAddresses => list_addresses_command(&account_handle).await,
                AccountCommands::ListTransactions => list_transactions_command(&account_handle).await,
                AccountCommands::MintNativeToken {
                    maximum_supply,
                    token_tag,
                    foundry_metadata,
                } => mint_native_token_command(&account_handle, maximum_supply, token_tag, foundry_metadata).await,
                AccountCommands::MintNft {
                    address,
                    immutable_metadata,
                    metadata,
                } => mint_nft_command(&account_handle, address, immutable_metadata, metadata).await,
                AccountCommands::Send { address, amount } => send_command(&account_handle, address, amount).await,
                AccountCommands::SendMicro { address, amount } => {
                    send_micro_command(&account_handle, address, amount).await
                }
                AccountCommands::SendNative {
                    address,
                    token_id,
                    native_token_amount,
                } => send_native_command(&account_handle, address, token_id, native_token_amount).await,
                AccountCommands::SendNft { address, nft_id } => {
                    send_nft_command(&account_handle, address, nft_id).await
                }
                AccountCommands::Sync => sync_account_command(&account_handle).await,
            } {
                println!("Error: {}", err);
            }
        }
    }

    false
}
