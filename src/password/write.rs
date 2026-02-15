// This file is part of Mimosa, a CLI to manage passwords.
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

use std::{
    fmt, fs,
    io::{self, Read},
    path::Path,
};

use anyhow::{Context, Result};
use clap::Parser;
use pimalaya_toolbox::terminal::printer::Printer;
use secrecy::{ExposeSecret, SecretString};
use serde::Serialize;

use crate::{config::Config, store::StoreExt};

/// Set a secret in the store.
///
/// If PASSWORD is given and points to an existing file, its content
/// is used as the secret. Otherwise the argument itself is used as
/// the secret. When no argument is provided, the secret is read from
/// stdin (supports both piping and file redirection).
#[derive(Parser, Debug)]
pub struct WritePasswordCommand {
    /// Name of the store in the configuration file.
    pub store: String,

    /// The secret, or a path to a file containing the secret.
    pub password: Option<SecretString>,
}

impl WritePasswordCommand {
    pub fn execute(self, printer: &mut impl Printer, config: &Config) -> Result<()> {
        let store = config.get_store(&self.store)?;

        let password = match self.password {
            Some(ref val) if Path::new(val.expose_secret()).is_file() => {
                let contents = fs::read_to_string(val.expose_secret())
                    .context("Cannot read secret from file")?;
                contents
                    .trim_end_matches('\n')
                    .trim_end_matches('\r')
                    .into()
            }
            Some(val) => val,
            None => {
                let mut buf = String::new();
                io::stdin()
                    .read_to_string(&mut buf)
                    .context("Cannot read secret from stdin")?;
                buf.trim_end_matches('\n').trim_end_matches('\r').into()
            }
        };

        store.write(password)?;

        printer.out(PasswordWritten { store: self.store })
    }
}

#[derive(Serialize)]
struct PasswordWritten {
    store: String,
}

impl fmt::Display for PasswordWritten {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Password successfully written to {}", self.store)
    }
}
