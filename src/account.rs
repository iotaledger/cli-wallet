// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::print_error;

use anyhow::Result;
use clap::{App, ArgMatches};
use dialoguer::Input;
use iota::message::prelude::MessageId;
use iota_wallet::{
    account::AccountHandle,
    address::Address,
    client::ClientOptionsBuilder,
    message::{Message, MessageType, Transfer},
};

use std::{num::NonZeroU64, process::Command, str::FromStr};

fn print_message(message: &Message) {
    println!("MESSAGE {}", message.id());
    println!("--- Value: {:?}", message.value());
    println!("--- Timestamp: {:?}", message.timestamp());
    println!(
        "--- Broadcasted: {}, confirmed: {}",
        message.broadcasted(),
        match message.confirmed() {
            Some(c) => c.to_string(),
            None => "unknown".to_string(),
        }
    );
}

fn print_address(address: &Address) {
    println!("ADDRESS {:?}", address.address().to_bech32());
    println!("--- Balance: {}", address.balance());
    println!("--- Index: {}", address.key_index());
    println!("--- Change address: {}", address.internal());
}

// `list-messages` command
async fn list_messages_command(account_handle: &AccountHandle, matches: &ArgMatches) {
    if let Some(matches) = matches.subcommand_matches("list-messages") {
        if let Some(id) = matches.value_of("id") {
            if let Ok(message_id) = MessageId::from_str(id) {
                let account = account_handle.read().await;
                if let Some(message) = account.get_message(&message_id) {
                    print_message(message);
                } else {
                    println!("Message not found");
                }
            } else {
                println!("Message id must be a hex string of length 64");
            }
        } else {
            let account = account_handle.read().await;
            let message_type = if let Some(message_type) = matches.value_of("type") {
                match message_type {
                    "received" => Some(MessageType::Received),
                    "sent" => Some(MessageType::Sent),
                    "failed" => Some(MessageType::Failed),
                    "unconfirmed" => Some(MessageType::Unconfirmed),
                    "value" => Some(MessageType::Value),
                    _ => panic!("unexpected message type"),
                }
            } else {
                None
            };

            let messages = account.list_messages(0, 0, message_type);
            if messages.is_empty() {
                println!("No messages found");
            } else {
                messages.iter().for_each(|m| print_message(m));
            }
        }
    }
}

// `list-addresses` command
async fn list_addresses_command(account_handle: &AccountHandle, matches: &ArgMatches) {
    if matches.subcommand_matches("list-addresses").is_some() {
        let account = account_handle.read().await;
        let addresses = account.addresses();
        if addresses.is_empty() {
            println!("No addresses found");
        } else {
            addresses.iter().for_each(|a| print_address(a));
        }
    }
}

// `sync` command
async fn sync_account_command(account_handle: &AccountHandle, matches: &ArgMatches) -> Result<()> {
    if matches.subcommand_matches("sync").is_some() {
        account_handle.sync().await.execute().await?;
    }
    Ok(())
}

// `address` command
async fn generate_address_command(account_handle: &AccountHandle, matches: &ArgMatches) -> Result<()> {
    if matches.subcommand_matches("address").is_some() {
        let address = account_handle.generate_address().await?;
        print_address(&address);
    }
    Ok(())
}

// `balance` command
async fn balance_command(account_handle: &AccountHandle, matches: &ArgMatches) {
    if matches.subcommand_matches("balance").is_some() {
        let account = account_handle.read().await;
        let balance = account.addresses().iter().fold(0, |acc, addr| acc + *addr.balance());
        println!("{}", balance);
    }
}

// `transfer` command
async fn transfer_command(account_handle: &AccountHandle, matches: &ArgMatches) -> Result<()> {
    if let Some(matches) = matches.subcommand_matches("transfer") {
        let address = matches.value_of("address").unwrap().to_string();
        let amount = matches.value_of("amount").unwrap();
        if let Ok(address) = iota_wallet::address::parse(address) {
            if let Ok(amount) = amount.parse::<u64>() {
                let transfer = Transfer::builder(
                    address,
                    NonZeroU64::new(amount).ok_or_else(|| anyhow::anyhow!("amount can't be zero"))?,
                )
                .finish();

                let synced = account_handle.sync().await.execute().await?;
                let message = synced.transfer(transfer).await?;
                print_message(&message);
            } else {
                println!("Amount must be a number");
            }
        } else {
            println!("Address must be a bech32 string");
        }
    }
    Ok(())
}

enum ReplayAction {
    Promote,
    Retry,
    Reattach,
}

// promotes, retries or reattaches a message
async fn replay_message(account_handle: &AccountHandle, action: ReplayAction, message_id: &str) -> Result<()> {
    if let Ok(message_id) = MessageId::from_str(message_id) {
        let synced = account_handle.sync().await.execute().await?;
        let message = match action {
            ReplayAction::Promote => synced.promote(&message_id).await?,
            ReplayAction::Retry => synced.retry(&message_id).await?,
            ReplayAction::Reattach => synced.reattach(&message_id).await?,
        };
        print_message(&message);
    } else {
        println!("Message id must be a hex string of length 64");
    }
    Ok(())
}

// `promote` command
async fn promote_message_command(account_handle: &AccountHandle, matches: &ArgMatches) -> Result<()> {
    if let Some(matches) = matches.subcommand_matches("promote") {
        let message_id = matches.value_of("id").unwrap();
        replay_message(account_handle, ReplayAction::Promote, message_id).await?;
    }
    Ok(())
}

// `retry` command
async fn retry_message_command(account_handle: &AccountHandle, matches: &ArgMatches) -> Result<()> {
    if let Some(matches) = matches.subcommand_matches("retry") {
        let message_id = matches.value_of("id").unwrap();
        replay_message(account_handle, ReplayAction::Retry, message_id).await?;
    }
    Ok(())
}

// `reattach` command
async fn reattach_message_command(account_handle: &AccountHandle, matches: &ArgMatches) -> Result<()> {
    if let Some(matches) = matches.subcommand_matches("reattach") {
        let message_id = matches.value_of("id").unwrap();
        replay_message(account_handle, ReplayAction::Reattach, message_id).await?;
    }
    Ok(())
}

// `set-node` command
async fn set_node_command(account_handle: &AccountHandle, matches: &ArgMatches) -> Result<()> {
    if let Some(matches) = matches.subcommand_matches("set-node") {
        let node = matches.value_of("node").unwrap();
        account_handle
            .set_client_options(ClientOptionsBuilder::node(node)?.build())
            .await;
    }
    Ok(())
}

// `set-alias` command
async fn set_alias_command(account_handle: &AccountHandle, matches: &ArgMatches) -> Result<()> {
    if let Some(matches) = matches.subcommand_matches("set-alias") {
        let alias = matches.value_of("alias").unwrap();
        account_handle.set_alias(alias).await;
    }
    Ok(())
}

// account prompt commands
async fn account_commands(account_handle: &AccountHandle, matches: &ArgMatches) -> Result<()> {
    list_messages_command(account_handle, &matches).await;
    list_addresses_command(account_handle, &matches).await;
    sync_account_command(account_handle, &matches).await?;
    generate_address_command(account_handle, &matches).await?;
    balance_command(account_handle, &matches).await;
    transfer_command(account_handle, &matches).await?;
    promote_message_command(account_handle, &matches).await?;
    retry_message_command(account_handle, &matches).await?;
    reattach_message_command(account_handle, &matches).await?;
    set_node_command(account_handle, &matches).await?;
    set_alias_command(account_handle, &matches).await?;
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
    let alias = account_handle.alias().await;
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
