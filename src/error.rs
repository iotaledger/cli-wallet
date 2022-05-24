// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use fern_logger::Error as LoggerError;
use iota_wallet::{
    error::Error as WalletError,
    iota_client::{bee_block::Error as BlockError, error::Error as ClientError},
};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("block error: {0}")]
    Block(#[from] BlockError),
    #[error("client error: {0}")]
    Client(#[from] ClientError),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("logger error: {0}")]
    Logger(#[from] LoggerError),
    #[error("{0}")]
    Miscellanous(String),
    #[error("generate at least one address before using the faucet")]
    NoAddressForFaucet,
    #[error("wallet error: {0}")]
    Wallet(#[from] WalletError),
}
