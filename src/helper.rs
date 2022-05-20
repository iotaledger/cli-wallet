// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::path::Path;

use clap::Parser;
use dialoguer::{console::Term, theme::ColorfulTheme, Password, Select};
use iota_wallet::account::AccountHandle;

use crate::AccountManagerCli;

pub fn get_password(path: &Path) -> String {
    let mut prompt = Password::new();
    prompt.with_prompt("What's the stronghold password?");
    // Check if the stronghold exists already
    if !path.exists() {
        prompt.with_confirmation("Confirm password", "Password mismatch");
    }

    let password: String = prompt.interact().unwrap();
    password
}

pub async fn pick_account(accounts: Vec<AccountHandle>) -> Option<usize> {
    let mut items = Vec::new();
    for account_handle in accounts {
        println!("{}", account_handle.read().await.index());
        items.push(account_handle.read().await.alias().clone());
    }
    Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select an account to manipulate")
        .items(&items)
        .default(0)
        .interact_on_opt(&Term::stderr())
        .unwrap_or_default()
}

pub fn help_command() {
    if let Err(r) = AccountManagerCli::try_parse() {
        // If only one argument from the user is provided, try to use it as identifier
        let mut iter = std::env::args();
        // The first element is traditionally the path of the executable
        iter.next();
        if let Some(input) = iter.next() {
            if input == "help" {
                // this prints the help output
                r.print().expect("Error writing Error");
                std::process::exit(0);
            }
        }
    }
}
