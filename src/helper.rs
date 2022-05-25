// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::path::Path;

use dialoguer::{console::Term, theme::ColorfulTheme, Password, Select};
use iota_wallet::account::AccountHandle;

use crate::error::Error;

pub fn get_password(path: &Path) -> Result<String, Error> {
    let mut prompt = Password::new();

    prompt.with_prompt("What's the stronghold password?");

    // Check if the stronghold exists already
    if !path.exists() {
        prompt.with_confirmation("Confirm password", "Password mismatch");
    }

    Ok(prompt.interact()?)
}

pub async fn pick_account(accounts: Vec<AccountHandle>) -> Option<usize> {
    let mut items = Vec::new();

    for account_handle in accounts {
        items.push(account_handle.read().await.alias().clone());
    }

    Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select an account to manipulate")
        .items(&items)
        .default(0)
        .interact_on_opt(&Term::stderr())
        .unwrap_or_default()
}
