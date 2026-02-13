// This file is part of Mimosa, a CLI to manage secrets.
//
// Copyright (C) 2026 Clément DOUIN <pimalaya.org@posteo.net>
//
// This program is free software: you can redistribute it and/or
// modify it under the terms of the GNU Affero General Public License
// as published by the Free Software Foundation, either version 3 of
// the License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public
// License along with this program. If not, see
// <https://www.gnu.org/licenses/>.

use std::io::{self, BufRead};

use anyhow::{Context, Result};
use clap::Parser;
use pimalaya_toolbox::terminal::printer::Printer;

use crate::account::Account;

/// Set a password in the configured backend.
///
/// The password is read from stdin (one line).
#[derive(Parser, Debug)]
pub struct SetPasswordCommand {}

impl SetPasswordCommand {
    pub fn execute(self, printer: &mut impl Printer, account: Account) -> Result<()> {
        let mut password = String::new();

        io::stdin()
            .lock()
            .read_line(&mut password)
            .context("Cannot read password from stdin")?;

        let password = password.trim_end_matches('\n').trim_end_matches('\r');

        account.backend.set_password(password)?;

        printer.out("Password successfully saved")
    }
}
