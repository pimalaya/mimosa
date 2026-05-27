// This file is part of Mimosa, a CLI to manage passwords.
//
// Copyright (C) 2026  soywod <pimalaya.org@posteo.net>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

//! TOML schema for mimosa's configuration file: a single `[stores]`
//! table keyed by user-chosen store name, each value selecting one
//! keyring backend (see [`crate::store::dispatch::Store`]).

use std::collections::HashMap;

use anyhow::{anyhow, Result};
use pimalaya_config::toml::TomlConfig;
use serde::{Deserialize, Serialize};

use crate::store::dispatch::Store;

/// Top-level configuration deserialized from the TOML config file.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    /// Every configured store, keyed by the user-chosen name used as
    /// the positional argument of `mimosa password <store>`.
    pub stores: HashMap<String, Store>,
}

impl Config {
    /// Looks up a store by name, returning a clone or an error
    /// pointing at the missing key.
    pub fn get_store(&self, name: &str) -> Result<Store> {
        self.stores
            .get(name)
            .cloned()
            .ok_or_else(|| anyhow!("Store {name:?} not found"))
    }
}

impl TomlConfig for Config {
    type Account = Store;

    fn project_name() -> &'static str {
        env!("CARGO_PKG_NAME")
    }

    fn take_default_account(&mut self) -> Option<(String, Self::Account)> {
        None
    }

    fn take_named_account(&mut self, name: &str) -> Option<(String, Self::Account)> {
        self.stores.remove_entry(name)
    }
}

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]

    use super::Config;
    use crate::store::dispatch::Store;

    /// The shipped `config.sample.toml` must round-trip through the
    /// deserializer under default features, otherwise users who
    /// copy-paste it end up with an unreadable config.
    #[cfg(any(feature = "dbus-secret-service", feature = "zbus-secret-service"))]
    #[test]
    fn config_sample_toml_parses() {
        let toml_text = include_str!("../config.sample.toml");
        let config: Config = toml::from_str(toml_text).expect("config.sample.toml must parse");
        let store = config
            .stores
            .get("example")
            .expect("example store must exist");
        assert!(matches!(store, Store::SecretService(_)));
    }

    #[cfg(feature = "keyutils")]
    #[test]
    fn linux_keyutils_kind_parses() {
        let toml_text = r#"
[stores.example]
store = "linux-keyutils"
linux-keyutils.service = "svc"
linux-keyutils.user = "u"
"#;
        let config: Config = toml::from_str(toml_text).unwrap();
        assert!(matches!(
            config.stores.get("example"),
            Some(Store::Keyutils(_))
        ));
    }

    #[cfg(feature = "apple-keychain")]
    #[test]
    fn apple_native_kind_parses() {
        let toml_text = r#"
[stores.example]
store = "apple-native"
apple-native.service = "svc"
apple-native.user = "u"
"#;
        let config: Config = toml::from_str(toml_text).unwrap();
        assert!(matches!(
            config.stores.get("example"),
            Some(Store::Macos(_))
        ));
    }

    #[cfg(feature = "windows-credential-manager")]
    #[test]
    fn windows_native_kind_parses() {
        let toml_text = r#"
[stores.example]
store = "windows-native"
windows-native.service = "svc"
windows-native.user = "u"
"#;
        let config: Config = toml::from_str(toml_text).unwrap();
        assert!(matches!(
            config.stores.get("example"),
            Some(Store::Windows(_))
        ));
    }
}
