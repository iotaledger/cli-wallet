// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::Result;
use clap::{Parser, Subcommand};
use iota_wallet::{
    account::{
        types::{AccountAddress, Transaction},
        AccountHandle, OutputsToCollect, SyncOptions,
    },
    iota_client::{
        bee_message::output::{NftId, TokenId, TokenTag},
        request_funds_from_faucet,
    },
    AddressAndNftId, AddressNativeTokens, AddressWithAmount, AddressWithMicroAmount, NativeTokenOptions, NftOptions,
    U256,
};

use std::str::FromStr;

#[derive(Parser)]
#[clap(version, long_about = None)]
#[clap(propagate_version = true)]
pub struct AccountCli {
    #[clap(subcommand)]
    pub command: AccountCommands,
}

#[derive(Subcommand)]
pub enum AccountCommands {
    /// Generate a new address.
    Address,
    /// Print the account balance.
    Balance,
    /// Request funds from the faucet to the latest address, `url` is optional, default is `http://localhost:14265/api/plugins/faucet/v1/enqueue`
    Faucet { url: Option<String> },
    /// List the account addresses.
    ListAddresses,
    /// List the account transactions.
    ListTransactions,
    /// Mint an nft to an optional bech32 encoded address: `mint-nft
    /// atoi1qzt0nhsf38nh6rs4p6zs5knqp6psgha9wsv74uajqgjmwc75ugupx3y7x0r "immutable metadata" "metadata"`
    MintNft {
        address: Option<String>,
        immutable_metadata: Option<String>,
        metadata: Option<String>,
    },
    /// Mint a native token: `mint-native-token 100 "tokentag"`
    MintNativeToken {
        maximum_supply: String,
        token_tag: Option<String>,
    },
    /// Send an amount to a bech32 encoded address: `send
    /// atoi1qzt0nhsf38nh6rs4p6zs5knqp6psgha9wsv74uajqgjmwc75ugupx3y7x0r 1000000`
    Send { address: String, amount: u64 },
    /// Send an amount below the storage deposit minimum to a bech32 address: `send
    /// atoi1qzt0nhsf38nh6rs4p6zs5knqp6psgha9wsv74uajqgjmwc75ugupx3y7x0r 1`
    SendMicro { address: String, amount: u64 },
    /// Send native tokens to a bech32 address: `send-native
    /// atoi1qzt0nhsf38nh6rs4p6zs5knqp6psgha9wsv74uajqgjmwc75ugupx3y7x0r
    /// 08e3a2f76cc934bc0cc21575b4610c1d7d4eb589ae0100000000000000000000000000000000 10`
    SendNative {
        address: String,
        token_id: String,
        native_token_amount: String,
    },
    /// Send an nft to a bech32 encoded address
    SendNft { address: String, nft_id: String },
    /// Sync the account with the Tangle.
    Sync,
    /// Exit from the account prompt.
    Exit,
}

/// `list-transactions` command
pub async fn list_transactions_command(account_handle: &AccountHandle) -> Result<()> {
    let transactions = account_handle.list_transactions().await?;
    if transactions.is_empty() {
        println!("No transactions found");
    } else {
        transactions.iter().for_each(print_transaction);
    }
    Ok(())
}

/// `list-addresses` command
pub async fn list_addresses_command(account_handle: &AccountHandle) -> Result<()> {
    let addresses = account_handle.list_addresses().await.unwrap();
    if addresses.is_empty() {
        println!("No addresses found");
    } else {
        for address in addresses {
            print_address(account_handle, &address).await;
        }
    }
    Ok(())
}

// `mint-nft` command
pub async fn mint_nft_command(
    account_handle: &AccountHandle,
    address: Option<String>,
    immutable_metadata: Option<String>,
    metadata: Option<String>,
) -> Result<()> {
    let immutable_metadata = immutable_metadata.map(|immutable_metadata| immutable_metadata.as_bytes().to_vec());
    let metadata = metadata.map(|metadata| metadata.as_bytes().to_vec());
    let nft_options = vec![NftOptions {
        address,
        immutable_metadata,
        metadata,
    }];

    let transfer_result = account_handle.mint_nfts(nft_options, None).await?;
    println!("Minting transaction sent: {:?}", transfer_result);
    Ok(())
}

// `mint-native-token` command
pub async fn mint_native_token_command(
    account_handle: &AccountHandle,
    // todo: enable this when there is support to mint additional tokens for an existing token
    // circulating_supply: String,
    maximum_supply: String,
    token_tag: Option<String>,
) -> Result<()> {
    let token_tag = token_tag.map(|token_tag| token_tag.as_bytes().to_vec());
    let token_tag = if let Some(mut token_tag) = token_tag {
        if token_tag.len() > 12 {
            return Err(anyhow::anyhow!("Token tag is too long in bytes {}/12", token_tag.len()));
        } else {
            // Fill remaining bytes with zeros
            token_tag.resize(12, 0u8);
            let bytes: [u8; 12] = token_tag.try_into().expect("Invalid token tag byte length");
            TokenTag::new(bytes)
        }
    } else {
        TokenTag::new([0u8; 12])
    };
    let native_token_options = NativeTokenOptions {
        account_address: None,
        token_tag,
        circulating_supply: U256::from_dec_str(&maximum_supply)?,
        maximum_supply: U256::from_dec_str(&maximum_supply)?,
    };

    let transfer_result = account_handle.mint_native_token(native_token_options, None).await?;

    println!("Minting transaction sent: {:?}", transfer_result);
    Ok(())
}

// `sync` command
pub async fn sync_account_command(account_handle: &AccountHandle) -> Result<()> {
    let sync = account_handle
        .sync(Some(SyncOptions {
            try_collect_outputs: OutputsToCollect::All,
            ..Default::default()
        }))
        .await?;
    println!("Synced: {:?}", sync);
    Ok(())
}

// `address` command
pub async fn generate_address_command(account_handle: &AccountHandle) -> Result<()> {
    let address = account_handle.generate_addresses(1, None).await?;
    print_address(account_handle, &address[0]).await;
    Ok(())
}

// `balance` command
pub async fn balance_command(account_handle: &AccountHandle) -> Result<()> {
    println!("{:?}", account_handle.balance().await?);
    Ok(())
}

// `send` command
pub async fn send_command(account_handle: &AccountHandle, address: String, amount: u64) -> Result<()> {
    let outputs = vec![AddressWithAmount { address, amount }];
    let transfer_result = account_handle.send_amount(outputs, None).await?;
    println!("Transaction created: {:?}", transfer_result);
    Ok(())
}

// `send-micro` command
pub async fn send_micro_command(account_handle: &AccountHandle, address: String, amount: u64) -> Result<()> {
    let outputs = vec![AddressWithMicroAmount {
        address,
        amount,
        return_address: None,
        expiration: None,
    }];
    let transfer_result = account_handle.send_micro_transaction(outputs, None).await?;
    println!("Micro transaction created: {:?}", transfer_result);
    Ok(())
}

// `send-native` command
pub async fn send_native_command(
    account_handle: &AccountHandle,
    address: String,
    token_id: String,
    native_token_amount: String,
) -> Result<()> {
    let outputs = vec![AddressNativeTokens {
        address,
        native_tokens: vec![(TokenId::from_str(&token_id)?, U256::from_dec_str(&native_token_amount)?)],
        ..Default::default()
    }];
    let transfer_result = account_handle.send_native_tokens(outputs, None).await?;
    println!("Transaction created: {:?}", transfer_result);
    Ok(())
}

// `send-nft` command
pub async fn send_nft_command(account_handle: &AccountHandle, address: String, nft_id: String) -> Result<()> {
    let outputs = vec![AddressAndNftId {
        address,
        nft_id: NftId::from_str(&nft_id)?,
    }];
    let transfer_result = account_handle.send_nft(outputs, None).await?;
    println!("Transaction created: {:?}", transfer_result);
    Ok(())
}

// `faucet` command
pub async fn faucet_command(account_handle: &AccountHandle, url: Option<String>) -> Result<()> {
    let address = match account_handle.list_addresses().await?.last() {
        Some(address) => address.clone(),
        None => return Err(anyhow::anyhow!("Generate an address first!")),
    };
    let faucet_url = match &url {
        Some(faucet_url) => faucet_url,
        None => "http://localhost:14265/api/plugins/faucet/v1/enqueue",
    };
    println!(
        "{}",
        request_funds_from_faucet(faucet_url, &address.address().to_bech32()).await?
    );
    Ok(())
}

// `set-alias` command
// pub async fn set_alias_command(account_handle: &AccountHandle) -> Result<()> {
//     if let Some(matches) = matches.subcommand_matches("set-alias") {
//         let alias = matches.value_of("alias").unwrap();
//         account_handle.set_alias(alias).await?;
//     }
//     Ok(())
// }

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

pub async fn print_address(account_handle: &AccountHandle, address: &AccountAddress) {
    println!("ADDRESS {:?}", address.address().to_bech32());
    println!("--- Index: {}", address.key_index());
    if *address.internal() {
        println!("--- Change address: {}", address.internal());
    }
    let addresses_with_balance = account_handle.list_addresses_with_unspent_outputs().await.unwrap();
    if let Ok(index) = addresses_with_balance.binary_search_by_key(&(address.key_index(), address.internal()), |a| {
        (a.key_index(), a.internal())
    }) {
        println!("--- Address balance: {}", addresses_with_balance[index].amount());
        println!("--- Address outputs: {:#?}", addresses_with_balance[index].output_ids());
    }
}