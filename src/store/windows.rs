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
pub struct WindowsStore {
    pub service: String,
    pub user: String,
}

impl StoreExt for WindowsStore {
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

impl WindowsStore {
    #[cfg(target_os = "windows")]
    #[cfg(feature = "windows-credential-manager")]
    fn init(&self) -> Result<()> {
        let store = windows_native_keyring_store::Store::new()
            .map_err(|err| anyhow!("{err}"))
            .context("Cannot create Windows Credential store")?;
        keyring_core::set_default_store(store);
        Ok(())
    }

    #[cfg(target_os = "windows")]
    #[cfg(not(feature = "windows-credential-manager"))]
    fn init(&self) -> Result<()> {
        bail!("Feature `windows-credential-manager` is missing");
    }

    #[cfg(not(target_os = "windows"))]
    fn init(&self) -> Result<()> {
        bail!("Feature `windows-credential-manager` is not available on this platform");
    }
}
