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

//! Apple Keychain backend (macOS), gated by the `apple-keychain`
//! cargo feature.

#[allow(unused)]
use anyhow::{anyhow, bail, Context, Result};
use secrecy::SecretString;
use serde::{Deserialize, Serialize};

use crate::{keyring, store::dispatch::StoreExt};

/// Apple Keychain store entry.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct MacosStore {
    /// `kSecAttrService` of the Keychain item; combined with `user`
    /// to form the unique lookup key.
    pub service: String,
    /// `kSecAttrAccount` of the Keychain item.
    pub user: String,
}

impl StoreExt for MacosStore {
    fn read(&self) -> Result<SecretString> {
        self.entry()?.read()
    }

    fn write(&self, secret: SecretString) -> Result<()> {
        self.entry()?.write(secret)
    }

    fn remove(&self) -> Result<bool> {
        self.entry()?.remove()
    }
}

impl MacosStore {
    fn entry(&self) -> Result<keyring::Entry> {
        self.init()?;
        keyring::Entry::new(&self.service, &self.user)
    }

    #[cfg(target_os = "macos")]
    #[cfg(feature = "apple-keychain")]
    fn init(&self) -> Result<()> {
        let store = apple_native_keyring_store::keychain::Store::new()
            .map_err(|err| anyhow!("{err}"))
            .context("Cannot create Apple Keychain store")?;
        keyring_core::set_default_store(store);
        Ok(())
    }

    #[cfg(target_os = "macos")]
    #[cfg(not(feature = "apple-keychain"))]
    fn init(&self) -> Result<()> {
        bail!("Feature `apple-keychain` is missing");
    }

    #[cfg(not(target_os = "macos"))]
    fn init(&self) -> Result<()> {
        bail!("Feature `apple-keychain` is not available on this platform");
    }
}
