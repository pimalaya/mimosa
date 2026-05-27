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

//! Secret Service backend (Linux, FreeBSD), available via either the
//! `dbus-secret-service` or `zbus-secret-service` cargo feature.

#[allow(unused)]
use anyhow::{anyhow, bail, Context, Result};
use secrecy::SecretString;
use serde::{Deserialize, Serialize};

use crate::{keyring, store::dispatch::StoreExt};

/// Secret Service store entry.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct SecretServiceStore {
    /// `service` attribute of the Secret Service item; combined with
    /// `user` to form the unique lookup key.
    pub service: String,
    /// `account`/`user` attribute of the Secret Service item.
    pub user: String,
    /// Which Secret Service client implementation to use. When
    /// omitted, the best available is picked at runtime (dbus first,
    /// then zbus).
    #[serde(default)]
    pub flavour: Option<Flavour>,
}

/// Secret Service client implementation.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Flavour {
    /// Synchronous, libdbus-based client.
    Dbus,
    /// Pure-Rust, async-io-based client.
    Zbus,
}

impl StoreExt for SecretServiceStore {
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

impl SecretServiceStore {
    fn entry(&self) -> Result<keyring::Entry> {
        self.init()?;
        keyring::Entry::new(&self.service, &self.user)
    }

    fn init(&self) -> Result<()> {
        match &self.flavour {
            Some(Flavour::Dbus) => self.init_dbus(),
            Some(Flavour::Zbus) => self.init_zbus(),
            None => self.init_default(),
        }
    }

    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
    #[cfg(feature = "dbus-secret-service")]
    fn init_dbus(&self) -> Result<()> {
        let store = dbus_secret_service_keyring_store::Store::new()
            .map_err(|err| anyhow!("{err}"))
            .context("Cannot create D-Bus Secret Service store")?;

        keyring_core::set_default_store(store);

        Ok(())
    }

    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
    #[cfg(not(feature = "dbus-secret-service"))]
    fn init_dbus(&self) -> Result<()> {
        bail!("Feature `dbus-secret-service` is missing");
    }

    #[cfg(not(any(target_os = "linux", target_os = "freebsd")))]
    fn init_dbus(&self) -> Result<()> {
        bail!("Secret Service is not available on this platform");
    }

    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
    #[cfg(feature = "zbus-secret-service")]
    fn init_zbus(&self) -> Result<()> {
        let store = zbus_secret_service_keyring_store::Store::new()
            .map_err(|err| anyhow!("{err}"))
            .context("Cannot create zbus Secret Service store")?;
        keyring_core::set_default_store(store);
        Ok(())
    }

    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
    #[cfg(not(feature = "zbus-secret-service"))]
    fn init_zbus(&self) -> Result<()> {
        bail!("Feature `zbus-secret-service` is missing");
    }

    #[cfg(not(any(target_os = "linux", target_os = "freebsd")))]
    fn init_zbus(&self) -> Result<()> {
        bail!("Secret Service is not available on this platform");
    }

    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
    #[cfg(feature = "dbus-secret-service")]
    fn init_default(&self) -> Result<()> {
        self.init_dbus()
    }

    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
    #[cfg(not(feature = "dbus-secret-service"))]
    #[cfg(feature = "zbus-secret-service")]
    fn init_default(&self) -> Result<()> {
        self.init_zbus()
    }

    #[cfg(not(all(
        any(target_os = "linux", target_os = "freebsd"),
        feature = "dbus-secret-service"
    )))]
    #[cfg(not(all(
        any(target_os = "linux", target_os = "freebsd"),
        feature = "zbus-secret-service"
    )))]
    fn init_default(&self) -> Result<()> {
        bail!("no Secret Service implementation available");
    }
}
