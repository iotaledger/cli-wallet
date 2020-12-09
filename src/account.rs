// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{block_on, print_error};

use clap::{App, ArgMatches};
use dialoguer::Input;
use iota::message::prelude::MessageId;
use iota_wallet::{
    account::Account,
    message::{Message, MessageType, Transfer},
    Result,
};

use std::str::FromStr;

fn print_message(message: &Message) {
    println!("{}", serde_json::to_string_pretty(message).unwrap());
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
fn list_addresses_command(account: &Account, matches: &ArgMatches) {
    if matches.subcommand_matches("list-addresses").is_some() {
        let mut addresses = account.list_addresses(false);
        addresses.extend(account.list_addresses(true));
        if addresses.is_empty() {
            println!("No addresses found");
        } else {
            addresses
                .iter()
                .for_each(|a| println!("{}", serde_json::to_string_pretty(a).unwrap()));
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
        println!("{}", address.address().to_bech32());
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
    Ok(())
}

// loop on the account prompt
pub fn account_prompt(account_cli: &App<'_>, mut account: Account) {
    let command: String = Input::new()
        .with_prompt(format!("Account `{}` command (h for help)", account.alias()))
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

    account_prompt(account_cli, account)
}
