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

use std::collections::HashMap;

use anyhow::{anyhow, Result};
use pimalaya_toolbox::config::TomlConfig;
use serde::{Deserialize, Serialize};

use crate::store::Store;

/// The main configuration.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    /// The configuration of all the stores.
    pub stores: HashMap<String, Store>,
}

impl Config {
    pub fn get_store(&self, name: &str) -> Result<Store> {
        self.stores
            .get(name)
            .cloned()
            .ok_or_else(|| anyhow!("store {name:?} not found"))
    }
}

impl TomlConfig for Config {
    type Account = Store;

    fn project_name() -> &'static str {
        env!("CARGO_PKG_NAME")
    }

    fn find_default_account(&self) -> Option<(String, Self::Account)> {
        None
    }

    fn find_account(&self, name: &str) -> Option<(String, Self::Account)> {
        self.stores
            .get(name)
            .map(|store| (name.to_owned(), store.clone()))
    }
}
