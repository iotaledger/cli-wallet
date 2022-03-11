// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::print_error;

use anyhow::Result;
use clap::{App, ArgMatches};
use dialoguer::Input;
use iota_wallet::{
    account::{
        types::{AccountAddress, Transaction},
        AccountHandle,
    },
    ClientOptions,
};

use std::{num::NonZeroU64, process::Command, str::FromStr};

fn print_transaction(transaction: &Transaction) {
    println!("TRANSACTION {:?}", transaction);
    // if let Some(MessagePayload::Transaction(tx)) = message.payload() {
    //     let TransactionEssence::Regular(essence) = tx.essence();
    //     println!("--- Value: {:?}", essence.value());
    // }
    // println!("--- Timestamp: {:?}", message.timestamp());
    // println!(
    //     "--- Broadcasted: {}, confirmed: {}",
    //     message.broadcasted(),
    //     match message.confirmed() {
    //         Some(c) => c.to_string(),
    //         None => "unknown".to_string(),
    //     }
    // );
}

async fn print_address(account_handle: &AccountHandle, address: &AccountAddress) {
    println!("ADDRESS {:?}", address.address().to_bech32());
    // println!("Address balance: {}", address.balance());
    println!("--- Index: {}", address.key_index());
    println!("--- Change address: {}", address.internal());
    // println!("--- Address outputs: {}", address.output_ids());
}

// `list-messages` command
async fn list_messages_command(account_handle: &AccountHandle, matches: &ArgMatches) -> Result<()> {
    if let Some(matches) = matches.subcommand_matches("list-messages") {
        // if let Some(id) = matches.value_of("id") {
        //     if let Ok(message_id) = MessageId::from_str(id) {
        //         let account = account_handle.read().await;
        //         if let Some(message) = account.get_message(&message_id).await {
        //             print_message(&message);
        //         } else {
        //             println!("Message not found");
        //         }
        //     } else {
        //         println!("Message id must be a hex string of length 64");
        //     }
        // } else {

        let transactions = account_handle.list_transactions().await?;
        if transactions.is_empty() {
            println!("No transactions found");
        } else {
            transactions.iter().for_each(|m| print_transaction(m));
        }
        // }
    }
    Ok(())
}

// `list-addresses` command
async fn list_addresses_command(account_handle: &AccountHandle, matches: &ArgMatches) {
    if matches.subcommand_matches("list-addresses").is_some() {
        let addresses = account_handle.list_addresses().await.unwrap();
        if addresses.is_empty() {
            println!("No addresses found");
        } else {
            for address in addresses {
                print_address(account_handle, &address).await;
            }
        }
    }
}

// `sync` command
async fn sync_account_command(account_handle: &AccountHandle, matches: &ArgMatches) -> Result<()> {
    if let Some(matches) = matches.subcommand_matches("sync") {
        let sync = account_handle.sync(None).await?;
        println!("Synced: {:?}", sync);
    }
    Ok(())
}

// `address` command
async fn generate_address_command(account_handle: &AccountHandle, matches: &ArgMatches) -> Result<()> {
    if matches.subcommand_matches("address").is_some() {
        let address = account_handle.generate_addresses(1, None).await?;
        print_address(account_handle, &address[0]).await;
    }
    Ok(())
}

// `balance` command
async fn balance_command(account_handle: &AccountHandle, matches: &ArgMatches) -> Result<()> {
    if matches.subcommand_matches("balance").is_some() {
        println!("{:?}", account_handle.balance().await?);
    }
    Ok(())
}

// `transfer` command
async fn transfer_command(account_handle: &AccountHandle, matches: &ArgMatches) -> Result<()> {
    if let Some(matches) = matches.subcommand_matches("transfer") {
        let address = matches.value_of("address").unwrap().to_string();
        let amount = matches.value_of("amount").unwrap();
        // if let Ok(address) = iota_wallet::address::parse(address) {
        //     if let Ok(amount) = amount.parse::<u64>() {
        //         let transfer = Transfer::builder(
        //             address,
        //             NonZeroU64::new(amount).ok_or_else(|| anyhow::anyhow!("amount can't be zero"))?,
        //             None,
        //         )
        //         .finish();

        //         let message = account_handle.transfer(transfer).await?;
        //         print_message(&message);
        //     } else {
        //         return Err(anyhow::anyhow!("Amount must be a number"));
        //     }
        // } else {
        //     return Err(anyhow::anyhow!("Address must be a bech32 string"));
        // }
    }
    Ok(())
}

// `set-node` command
// async fn set_node_command(account_handle: &AccountHandle, matches: &ArgMatches) -> Result<()> {
//     if let Some(matches) = matches.subcommand_matches("set-node") {
//         let node = matches.value_of("node").unwrap();
//         account_handle
//             .set_client_options(ClientOptionsBuilder::new().with_nodes(&[node])?.build()?)
//             .await?;
//     }
//     Ok(())
// }

// `set-alias` command
// async fn set_alias_command(account_handle: &AccountHandle, matches: &ArgMatches) -> Result<()> {
//     if let Some(matches) = matches.subcommand_matches("set-alias") {
//         let alias = matches.value_of("alias").unwrap();
//         account_handle.set_alias(alias).await?;
//     }
//     Ok(())
// }

// account prompt commands
async fn account_commands(account_handle: &AccountHandle, matches: &ArgMatches) -> Result<()> {
    list_messages_command(account_handle, matches).await?;
    list_addresses_command(account_handle, matches).await;
    sync_account_command(account_handle, matches).await?;
    generate_address_command(account_handle, matches).await?;
    balance_command(account_handle, matches).await?;
    transfer_command(account_handle, matches).await?;
    // set_node_command(account_handle, matches).await?;
    // set_alias_command(account_handle, matches).await?;
    Ok(())
}

// loop on the account prompt
pub async fn account_prompt(account_cli: &App<'_>, account_handle: AccountHandle) {
    loop {
        let exit = account_prompt_internal(account_cli, account_handle.clone()).await;
        if exit {
            break;
        }
    }
}

// loop on the account prompt
pub async fn account_prompt_internal(account_cli: &App<'_>, account_handle: AccountHandle) -> bool {
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
            let mut cli = account_cli.clone();
            cli.print_help().unwrap();
        }
        "clear" => {
            let _ = Command::new("clear").status();
        }
        _ => {
            match account_cli
                .clone()
                .try_get_matches_from(command.split(' ').collect::<Vec<&str>>())
            {
                Ok(matches) => {
                    if matches.subcommand_matches("exit").is_some() {
                        return true;
                    }

                    if let Err(e) = account_commands(&account_handle, &matches).await {
                        print_error(e);
                    }
                }
                Err(e) => {
                    println!("{}", e.to_string());
                }
            }
        }
    }

    false
}
