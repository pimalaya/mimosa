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
use serde::Serialize;

use crate::{config::Config, store::StoreExt};

/// Remove a password from the store.
#[derive(Parser, Debug)]
pub struct RemovePasswordCommand {
    /// Name of the store in the configuration file.
    pub store: String,
}

impl RemovePasswordCommand {
    pub fn execute(self, printer: &mut impl Printer, config: &Config) -> Result<()> {
        let removed = config.get_store(&self.store)?.remove()?;

        printer.out(PasswordRemoved {
            store: self.store,
            removed,
        })
    }
}

#[derive(Serialize)]
struct PasswordRemoved {
    store: String,
    removed: bool,
}

impl fmt::Display for PasswordRemoved {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = &self.store;

        if self.removed {
            write!(f, "Password successfully removed from {s}")
        } else {
            write!(f, "No password found in {s}, nothing was removed")
        }
    }
}
