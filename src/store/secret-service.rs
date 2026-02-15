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

#[allow(unused)]
use anyhow::{anyhow, bail, Context, Result};
use secrecy::SecretString;
use serde::{Deserialize, Serialize};

use crate::{keyring, store::StoreExt};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct SecretServiceStore {
    pub service: String,
    pub user: String,
    #[serde(default)]
    pub flavour: Option<Flavour>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Flavour {
    Dbus,
    Zbus,
}

impl StoreExt for SecretServiceStore {
    fn read(&self) -> Result<SecretString> {
        self.init()?;
        keyring::read(&self.service, &self.user)
    }

    fn write(&self, secret: SecretString) -> Result<()> {
        self.init()?;
        keyring::write(&self.service, &self.user, secret)
    }

    fn remove(&self) -> Result<bool> {
        self.init()?;
        keyring::remove(&self.service, &self.user)
    }
}

impl SecretServiceStore {
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
