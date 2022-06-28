// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use clap::{Parser, Subcommand};
use iota_wallet::{
    account::{types::AccountAddress, AccountHandle, OutputsToClaim},
    iota_client::{
        bee_block::output::{AliasId, FoundryId, NftId, OutputId, TokenId},
        request_funds_from_faucet,
    },
    AddressAndNftId, AddressNativeTokens, AddressWithAmount, AddressWithMicroAmount, NativeTokenOptions, NftOptions,
    U256,
};

use crate::error::Error;

#[derive(Debug, Parser)]
#[clap(version, long_about = None)]
#[clap(propagate_version = true)]
pub struct AccountCli {
    #[clap(subcommand)]
    pub command: AccountCommand,
}

#[derive(Debug, Subcommand)]
pub enum AccountCommand {
    /// List the account addresses.
    Addresses,
    /// Print the account balance.
    Balance,
    /// Burn a native token: `burn-native-token "0x..." 100`
    BurnNativeToken { token_id: String, amount: String },
    /// Burn an NFT: `burn-nft "0x..."`
    BurnNft { nft_id: String },
    /// Claim outputs with storage deposit return, expiration or timelock unlock conditions.
    Claim,
    /// Consolidate all basic outputs into one address.
    Consolidate,
    /// Destroy an alias: `destroy-alias "0x..."`
    DestroyAlias { alias_id: String },
    /// Destroy a foundry: `destroy-foundry "0x..."`
    DestroyFoundry { foundry_id: String },
    /// Exit from the account prompt.
    Exit,
    /// Request funds from the faucet to the latest address, `url` is optional, default is `http://localhost:8091/api/enqueue`
    Faucet {
        url: Option<String>,
        address: Option<String>,
    },
    /// Melt a native token: `melt-native-token "0x..." 100`
    MeltNativeToken { token_id: String, amount: String },
    /// Mint a native token: `mint-native-token 100 "0x..." (foundry metadata)`
    MintNativeToken {
        maximum_supply: String,
        foundry_metadata: Option<String>,
    },
    /// Mint an NFT to an optional bech32 encoded address: `mint-nft
    /// rms1qztwng6cty8cfm42nzvq099ev7udhrnk0rw8jt8vttf9kpqnxhpsx869vr3 "immutable metadata" "metadata"`
    MintNft {
        address: Option<String>,
        immutable_metadata: Option<String>,
        metadata: Option<String>,
    },
    /// Generate a new address.
    NewAddress,
    /// Send an amount to a bech32 encoded address: `send
    /// rms1qztwng6cty8cfm42nzvq099ev7udhrnk0rw8jt8vttf9kpqnxhpsx869vr3 1000000`
    Send { address: String, amount: u64 },
    /// Send an amount below the storage deposit minimum to a bech32 address: `send
    /// rms1qztwng6cty8cfm42nzvq099ev7udhrnk0rw8jt8vttf9kpqnxhpsx869vr3 1`
    SendMicro { address: String, amount: u64 },
    /// Send native tokens to a bech32 address: `send-native
    /// rms1qztwng6cty8cfm42nzvq099ev7udhrnk0rw8jt8vttf9kpqnxhpsx869vr3
    /// 08e3a2f76cc934bc0cc21575b4610c1d7d4eb589ae0100000000000000000000000000000000 10`
    SendNativeToken {
        address: String,
        token_id: String,
        amount: String,
    },
    /// Send an NFT to a bech32 encoded address
    SendNft { address: String, nft_id: String },
    /// Sync the account with the Tangle.
    Sync,
    /// List the account transactions.
    Transactions,
    /// Display an output.
    Output { output_id: String },
    /// List all outputs.
    Outputs,
    /// List the unspent outputs.
    UnspentOutputs,
}

/// `addresses` command
pub async fn addresses_command(account_handle: &AccountHandle) -> Result<(), Error> {
    let addresses = account_handle.list_addresses().await?;

    if addresses.is_empty() {
        log::info!("No addresses found");
    } else {
        for address in addresses {
            print_address(account_handle, &address).await?;
        }
    }

    Ok(())
}

// `burn-native-token` command
pub async fn burn_native_token_command(
    account_handle: &AccountHandle,
    token_id: String,
    amount: String,
) -> Result<(), Error> {
    log::info!("Burning native token {token_id} {amount}.");

    let transaction_result = account_handle
        .burn_native_token(
            (
                TokenId::from_str(&token_id)?,
                U256::from_dec_str(&amount).map_err(|e| Error::Miscellanous(e.to_string()))?,
            ),
            None,
        )
        .await?;

    log::info!("{transaction_result:?}",);

    Ok(())
}

// `burn-nft` command
pub async fn burn_nft_command(account_handle: &AccountHandle, nft_id: String) -> Result<(), Error> {
    log::info!("Burning nft {nft_id}.");

    let transaction_result = account_handle.burn_nft(NftId::from_str(&nft_id)?, None).await?;

    log::info!("{transaction_result:?}");

    Ok(())
}

// `balance` command
pub async fn balance_command(account_handle: &AccountHandle) -> Result<(), Error> {
    log::info!("{:?}", account_handle.balance().await?);

    Ok(())
}

// `claim` command
pub async fn claim_command(account_handle: &AccountHandle) -> Result<(), Error> {
    log::info!("Claiming outputs.");

    let claiming_txs = account_handle.try_claim_outputs(OutputsToClaim::All).await?;

    for claim_tx in claiming_txs {
        log::info!("Claim transaction sent: {claim_tx:?}");
    }

    Ok(())
}

// `consolidate` command
pub async fn consolidate_command(account_handle: &AccountHandle) -> Result<(), Error> {
    log::info!("Consolidating outputs.");

    let consolidation_txs = account_handle.consolidate_outputs(true, None).await?;

    for consolidation_tx in consolidation_txs {
        log::info!("Consolidation transaction sent: {consolidation_tx:?}");
    }

    Ok(())
}

// `destroy-alias` command
pub async fn destroy_alias_command(account_handle: &AccountHandle, alias_id: String) -> Result<(), Error> {
    log::info!("Destroying alias {alias_id}.");

    let transaction_result = account_handle
        .destroy_alias(AliasId::from_str(&alias_id)?, None)
        .await?;

    log::info!("{transaction_result:?}");

    Ok(())
}

// `destroy-foundry` command
pub async fn destroy_foundry_command(account_handle: &AccountHandle, foundry_id: String) -> Result<(), Error> {
    log::info!("Destroying foundry {foundry_id}.");

    let transaction_result = account_handle
        .destroy_foundry(FoundryId::from_str(&foundry_id)?, None)
        .await?;

    log::info!("{transaction_result:?}");

    Ok(())
}

// `faucet` command
pub async fn faucet_command(
    account_handle: &AccountHandle,
    url: Option<String>,
    address: Option<String>,
) -> Result<(), Error> {
    let address = if let Some(address) = address {
        address
    } else {
        match account_handle.list_addresses().await?.last() {
            Some(address) => address.address().to_bech32(),
            None => return Err(Error::NoAddressForFaucet),
        }
    };
    let faucet_url = match &url {
        Some(faucet_url) => faucet_url,
        None => "http://localhost:8091/api/enqueue",
    };

    log::info!("{}", request_funds_from_faucet(faucet_url, &address).await?);

    Ok(())
}

// `melt-native-token` command
pub async fn melt_native_token_command(
    account_handle: &AccountHandle,
    token_id: String,
    amount: String,
) -> Result<(), Error> {
    let transaction_result = account_handle
        .melt_native_token(
            (
                TokenId::from_str(&token_id)?,
                U256::from_dec_str(&amount).map_err(|e| Error::Miscellanous(e.to_string()))?,
            ),
            None,
        )
        .await?;

    log::info!("Native token melting transaction sent: {transaction_result:?}");

    Ok(())
}

// `mint-native-token` command
pub async fn mint_native_token_command(
    account_handle: &AccountHandle,
    // todo: enable this when there is support to mint additional tokens for an existing token
    // circulating_supply: String,
    maximum_supply: String,
    foundry_metadata: Option<String>,
) -> Result<(), Error> {
    let native_token_options = NativeTokenOptions {
        account_address: None,
        circulating_supply: U256::from_dec_str(&maximum_supply).map_err(|e| Error::Miscellanous(e.to_string()))?,
        maximum_supply: U256::from_dec_str(&maximum_supply).map_err(|e| Error::Miscellanous(e.to_string()))?,
        foundry_metadata: foundry_metadata
            .map(|s| prefix_hex::decode(&s))
            .transpose()
            .map_err(|e| Error::Miscellanous(e.to_string()))?,
    };

    let transaction_result = account_handle.mint_native_token(native_token_options, None).await?;

    log::info!("Native token minting transaction sent: {transaction_result:?}");

    Ok(())
}

// `mint-nft` command
pub async fn mint_nft_command(
    account_handle: &AccountHandle,
    address: Option<String>,
    immutable_metadata: Option<String>,
    metadata: Option<String>,
) -> Result<(), Error> {
    let immutable_metadata = immutable_metadata.map(|immutable_metadata| immutable_metadata.as_bytes().to_vec());
    let metadata = metadata.map(|metadata| metadata.as_bytes().to_vec());
    let nft_options = vec![NftOptions {
        address,
        immutable_metadata,
        metadata,
    }];
    let transaction_result = account_handle.mint_nfts(nft_options, None).await?;

    log::info!("NFT minting transaction sent: {transaction_result:?}");

    Ok(())
}

// `new-address` command
pub async fn new_address_command(account_handle: &AccountHandle) -> Result<(), Error> {
    let address = account_handle.generate_addresses(1, None).await?;

    print_address(account_handle, &address[0]).await?;

    Ok(())
}

// `send` command
pub async fn send_command(account_handle: &AccountHandle, address: String, amount: u64) -> Result<(), Error> {
    let outputs = vec![AddressWithAmount { address, amount }];
    let transaction_result = account_handle.send_amount(outputs, None).await?;

    log::info!("Transaction created: {transaction_result:?}");

    Ok(())
}

// `send-micro` command
pub async fn send_micro_command(account_handle: &AccountHandle, address: String, amount: u64) -> Result<(), Error> {
    let outputs = vec![AddressWithMicroAmount {
        address,
        amount,
        return_address: None,
        expiration: None,
    }];

    let transaction_result = account_handle.send_micro_transaction(outputs, None).await?;

    log::info!("Micro transaction created: {transaction_result:?}");

    Ok(())
}

// `send-native-token` command
pub async fn send_native_token_command(
    account_handle: &AccountHandle,
    address: String,
    token_id: String,
    amount: String,
) -> Result<(), Error> {
    let outputs = vec![AddressNativeTokens {
        address,
        native_tokens: vec![(
            TokenId::from_str(&token_id)?,
            U256::from_dec_str(&amount).map_err(|e| Error::Miscellanous(e.to_string()))?,
        )],
        ..Default::default()
    }];
    let transaction_result = account_handle.send_native_tokens(outputs, None).await?;

    log::info!("Transaction created: {transaction_result:?}");

    Ok(())
}

// `send-nft` command
pub async fn send_nft_command(account_handle: &AccountHandle, address: String, nft_id: String) -> Result<(), Error> {
    let outputs = vec![AddressAndNftId {
        address,
        nft_id: NftId::from_str(&nft_id)?,
    }];
    let transaction_result = account_handle.send_nft(outputs, None).await?;

    log::info!("Transaction created: {transaction_result:?}");

    Ok(())
}

// `sync` command
pub async fn sync_command(account_handle: &AccountHandle) -> Result<(), Error> {
    let sync = account_handle.sync(None).await?;

    log::info!("Synced: {sync:?}");

    Ok(())
}

/// `transactions` command
pub async fn transactions_command(account_handle: &AccountHandle) -> Result<(), Error> {
    let transactions = account_handle.list_transactions().await?;

    if transactions.is_empty() {
        log::info!("No transactions found");
    } else {
        // Format to not take too much space with pretty printing but still have a clear separation between transactions
        let txs = transactions.iter().map(|tx| format!("{tx:?}")).collect::<Vec<String>>();
        log::info!("{txs:#?}");
    }

    Ok(())
}

/// `output` command
pub async fn output_command(account_handle: &AccountHandle, output_id: String) -> Result<(), Error> {
    let output = account_handle.get_output(&OutputId::from_str(&output_id)?).await;

    if let Some(output) = output {
        log::info!("{output:#?}");
    } else {
        log::info!("Output not found");
    }

    Ok(())
}

/// `outputs` command
pub async fn outputs_command(account_handle: &AccountHandle) -> Result<(), Error> {
    let outputs = account_handle.list_outputs().await?;

    if outputs.is_empty() {
        log::info!("No outputs found");
    } else {
        let output_ids: Vec<OutputId> = outputs.iter().map(|o| o.output_id).collect();
        log::info!("Outputs: {output_ids:#?}");
    }

    Ok(())
}

/// `unspent-outputs` command
pub async fn unspent_outputs_command(account_handle: &AccountHandle) -> Result<(), Error> {
    let outputs = account_handle.list_unspent_outputs().await?;

    if outputs.is_empty() {
        log::info!("No outputs found");
    } else {
        let output_ids: Vec<OutputId> = outputs.iter().map(|o| o.output_id).collect();
        log::info!("Unspent outputs: {output_ids:#?}");
    }

    Ok(())
}

// `set-alias` command
// pub async fn set_alias_command(account_handle: &AccountHandle) -> Result<()> {
//     if let Some(matches) = matches.subcommand_matches("set-alias") {
//         let alias = matches.value_of("alias")?;
//         account_handle.set_alias(alias).await?;
//     }
//     Ok(())
// }

pub async fn print_address(account_handle: &AccountHandle, address: &AccountAddress) -> Result<(), Error> {
    let mut log = format!("Address {}: {}", address.key_index(), address.address().to_bech32());

    if *address.internal() {
        log = format!("{log}\nChange address");
    }

    let addresses_with_balance = account_handle.list_addresses_with_unspent_outputs().await?;

    if let Ok(index) = addresses_with_balance.binary_search_by_key(&(address.key_index(), address.internal()), |a| {
        (a.key_index(), a.internal())
    }) {
        log = format!("{log}\nBalance: {}", addresses_with_balance[index].amount());
        log = format!("{log}\nOutputs: {:#?}", addresses_with_balance[index].output_ids());
    }

    log::info!("{log}");

    Ok(())
}
