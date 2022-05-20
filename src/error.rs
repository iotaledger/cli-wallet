// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use fern_logger::Error as LoggerError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("logger error: {0}")]
    Logger(#[from] LoggerError),
}
