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

//! Linux kernel keyutils backend, gated by the `keyutils` cargo
//! feature. Entries are wiped at reboot.

#[allow(unused)]
use anyhow::{anyhow, bail, Context, Result};
use secrecy::SecretString;
use serde::{Deserialize, Serialize};

use crate::{keyring, store::dispatch::StoreExt};

/// Linux keyutils store entry.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct KeyutilsStore {
    /// Description prefix of the kernel key; combined with `user` to
    /// form the unique lookup key.
    pub service: String,
    /// Description suffix of the kernel key, usually the user
    /// identifier or email.
    pub user: String,
}

impl StoreExt for KeyutilsStore {
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

impl KeyutilsStore {
    fn entry(&self) -> Result<keyring::Entry> {
        self.init()?;
        keyring::Entry::new(&self.service, &self.user)
    }

    #[cfg(target_os = "linux")]
    #[cfg(feature = "keyutils")]
    fn init(&self) -> Result<()> {
        let store = linux_keyutils_keyring_store::Store::new()
            .map_err(|err| anyhow!("{err}"))
            .context("Cannot create Linux keyutils store")?;
        keyring_core::set_default_store(store);
        Ok(())
    }

    #[cfg(target_os = "linux")]
    #[cfg(not(feature = "keyutils"))]
    fn init(&self) -> Result<()> {
        bail!("Feature `keyutils` is missing");
    }

    #[cfg(not(target_os = "linux"))]
    fn init(&self) -> Result<()> {
        bail!("Feature `keyutils` is not available on this platform");
    }
}
