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

use std::fmt;

use anyhow::Result;
use clap::Parser;
use pimalaya_toolbox::terminal::printer::Printer;
use secrecy::{ExposeSecret, SecretString};
use serde::{ser::SerializeStruct, Serialize, Serializer};

use crate::{config::Config, store::StoreExt};

/// Read a password from the store.
///
/// The raw password is printed to stdout, making it easy to pipe into
/// other commands.
#[derive(Parser, Debug)]
pub struct ReadPasswordCommand {
    /// Name of the store in the configuration file.
    pub store: String,
}

impl ReadPasswordCommand {
    pub fn execute(self, printer: &mut impl Printer, config: &Config) -> Result<()> {
        let password = config.get_store(&self.store)?.read()?;
        printer.out(Password(password))
    }
}

struct Password(SecretString);

impl Serialize for Password {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut s = serializer.serialize_struct("Password", 1)?;
        s.serialize_field("password", self.0.expose_secret())?;
        s.end()
    }
}

impl fmt::Display for Password {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0.expose_secret())
    }
}
