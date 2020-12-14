// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{block_on, print_error};

use clap::{App, ArgMatches};
use dialoguer::Input;
use iota::message::prelude::MessageId;
use iota_wallet::{
    account::Account,
    address::Address,
    client::ClientOptionsBuilder,
    message::{Message, MessageType, Transfer},
    Result,
};

use std::{
    str::FromStr,
    sync::{Arc, RwLock},
};

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
fn list_messages_command(account: &Account, matches: &ArgMatches) {
    if let Some(matches) = matches.subcommand_matches("list-messages") {
        if let Some(id) = matches.value_of("id") {
            if let Ok(message_id) = MessageId::from_str(id) {
                if let Some(message) = account.get_message(&message_id) {
                    print_message(message);
                } else {
                    println!("Message not found");
                }
            } else {
                println!("Message id must be a hex string of length 64");
            }
        } else {
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
fn list_addresses_command(account: &mut Account, matches: &ArgMatches) {
    if matches.subcommand_matches("list-addresses").is_some() {
        let addresses = account.addresses_mut();
        if addresses.is_empty() {
            println!("No addresses found");
        } else {
            addresses.sort_by_key(|a| *a.key_index());
            addresses.iter().for_each(|a| print_address(a));
        }
    }
}

// `sync` command
fn sync_account_command(account: &mut Account, matches: &ArgMatches) -> Result<()> {
    if matches.subcommand_matches("sync").is_some() {
        block_on(async move { account.sync().execute().await })?;
    }
    Ok(())
}

// `address` command
fn generate_address_command(account: &mut Account, matches: &ArgMatches) -> Result<()> {
    if matches.subcommand_matches("address").is_some() {
        let address = account.generate_address()?;
        print_address(&address);
    }
    Ok(())
}

// `balance` command
fn balance_command(account: &Account, matches: &ArgMatches) {
    if matches.subcommand_matches("balance").is_some() {
        let balance = account.addresses().iter().fold(0, |acc, addr| acc + *addr.balance());
        println!("{}", balance);
    }
}

// `transfer` command
fn transfer_command(account: &mut Account, matches: &ArgMatches) -> Result<()> {
    if let Some(matches) = matches.subcommand_matches("transfer") {
        let address = matches.value_of("address").unwrap().to_string();
        let amount = matches.value_of("amount").unwrap();
        if let Ok(address) = iota_wallet::address::parse(address) {
            if let Ok(amount) = amount.parse::<u64>() {
                let transfer = Transfer::new(address, amount);
                let res: Result<()> = block_on(async move {
                    let synced = account.sync().execute().await?;
                    let transfer_metadata = synced.transfer(transfer).await?;
                    *account = transfer_metadata.account;
                    print_message(&transfer_metadata.message);
                    Ok(())
                });
                res?;
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
fn replay_message(account: &mut Account, action: ReplayAction, message_id: &str) -> Result<()> {
    if let Ok(message_id) = MessageId::from_str(message_id) {
        let res: Result<()> = block_on(async move {
            let synced = account.sync().execute().await?;
            let message = match action {
                ReplayAction::Promote => synced.promote(&message_id).await?,
                ReplayAction::Retry => synced.retry(&message_id).await?,
                ReplayAction::Reattach => synced.reattach(&message_id).await?,
            };
            print_message(&message);
            Ok(())
        });
        res?;
    } else {
        println!("Message id must be a hex string of length 64");
    }
    Ok(())
}

// `promote` command
fn promote_message_command(account: &mut Account, matches: &ArgMatches) -> Result<()> {
    if let Some(matches) = matches.subcommand_matches("promote") {
        let message_id = matches.value_of("id").unwrap();
        replay_message(account, ReplayAction::Promote, message_id)?;
    }
    Ok(())
}

// `retry` command
fn retry_message_command(account: &mut Account, matches: &ArgMatches) -> Result<()> {
    if let Some(matches) = matches.subcommand_matches("retry") {
        let message_id = matches.value_of("id").unwrap();
        replay_message(account, ReplayAction::Retry, message_id)?;
    }
    Ok(())
}

// `reattach` command
fn reattach_message_command(account: &mut Account, matches: &ArgMatches) -> Result<()> {
    if let Some(matches) = matches.subcommand_matches("reattach") {
        let message_id = matches.value_of("id").unwrap();
        replay_message(account, ReplayAction::Reattach, message_id)?;
    }
    Ok(())
}

// `set-node` command
fn set_node_command(account: &mut Account, matches: &ArgMatches) -> Result<()> {
    if let Some(matches) = matches.subcommand_matches("set-node") {
        let node = matches.value_of("node").unwrap();
        account.set_client_options(ClientOptionsBuilder::node(node)?.build());
    }
    Ok(())
}

// `set-alias` command
fn set_alias_command(account: &mut Account, matches: &ArgMatches) -> Result<()> {
    if let Some(matches) = matches.subcommand_matches("set-alias") {
        let alias = matches.value_of("alias").unwrap();
        account.set_alias(alias);
    }
    Ok(())
}

// account prompt commands
fn account_commands(account: &mut Account, matches: &ArgMatches) -> Result<()> {
    list_messages_command(account, &matches);
    list_addresses_command(account, &matches);
    sync_account_command(account, &matches)?;
    generate_address_command(account, &matches)?;
    balance_command(account, &matches);
    transfer_command(account, &matches)?;
    promote_message_command(account, &matches)?;
    retry_message_command(account, &matches)?;
    reattach_message_command(account, &matches)?;
    set_node_command(account, &matches)?;
    set_alias_command(account, &matches)?;
    Ok(())
}

// loop on the account prompt
pub fn account_prompt(account_cli: &App<'_>, accounts: Arc<RwLock<Vec<Account>>>, index: usize) {
    let alias = {
        let accounts_ = accounts.read().unwrap();
        accounts_[index].alias().clone()
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
            let _ = std::process::Command::new("clear").status();
        }
        _ => {
            match account_cli
                .clone()
                .try_get_matches_from(command.split(' ').collect::<Vec<&str>>())
            {
                Ok(matches) => {
                    if matches.subcommand_matches("exit").is_some() {
                        return;
                    }

                    let mut accounts_ = accounts.write().unwrap();
                    let mut account = accounts_.get_mut(index).unwrap();
                    if let Err(e) = account_commands(&mut account, &matches) {
                        print_error(e);
                    }
                }
                Err(e) => {
                    println!("{}", e.to_string());
                }
            }
        }
    }

    account_prompt(account_cli, accounts, index)
}
