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

//! Serde shim for the runtime [`super::dispatch::Store`] enum.
//!
//! Used as the `try_from` / `into` target so the user's TOML schema
//! (`store = "<kind>"` discriminator + one per-backend block) can be
//! resolved at deserialization time into the variant-with-payload
//! shape the rest of the codebase consumes.

#[allow(unused)]
use anyhow::{bail, Context, Error};
use serde::{Deserialize, Serialize};

use crate::store::{
    keyutils::KeyutilsStore, macos::MacosStore, secret_service::SecretServiceStore,
    windows::WindowsStore,
};

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct Store {
    pub store: StoreKind,
    pub secret_service: Option<SecretServiceStore>,
    pub linux_keyutils: Option<KeyutilsStore>,
    pub apple_native: Option<MacosStore>,
    pub windows_native: Option<WindowsStore>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum StoreKind {
    SecretService,
    LinuxKeyutils,
    AppleNative,
    WindowsNative,
}

impl TryFrom<Store> for super::dispatch::Store {
    type Error = Error;

    fn try_from(entry: Store) -> Result<Self, Self::Error> {
        match entry.store {
            #[cfg(any(feature = "dbus-secret-service", feature = "zbus-secret-service"))]
            StoreKind::SecretService => {
                let store = entry
                    .secret_service
                    .context("Missing `secret-service` configuration")?;
                Ok(Self::SecretService(store))
            }
            #[cfg(not(any(feature = "dbus-secret-service", feature = "zbus-secret-service")))]
            StoreKind::SecretService => {
                bail!("Missing feature; enable `dbus-secret-service` or `zbus-secret-service`");
            }

            #[cfg(feature = "keyutils")]
            StoreKind::LinuxKeyutils => {
                let store = entry
                    .linux_keyutils
                    .context("Missing `keyutils` configuration")?;
                Ok(Self::Keyutils(store))
            }
            #[cfg(not(feature = "keyutils"))]
            StoreKind::LinuxKeyutils => bail!("Missing `keyutils` cargo feature"),

            #[cfg(feature = "apple-keychain")]
            StoreKind::AppleNative => {
                let store = entry
                    .apple_native
                    .context("Missing `apple-keychain` configuration")?;
                Ok(Self::Macos(store))
            }
            #[cfg(not(feature = "apple-keychain"))]
            StoreKind::AppleNative => bail!("Missing `apple-keychain` cargo feature"),

            #[cfg(feature = "windows-credential-manager")]
            StoreKind::WindowsNative => {
                let store = entry
                    .windows_native
                    .context("Missing `windows-credential-manager` configuration")?;
                Ok(Self::Windows(store))
            }
            #[cfg(not(feature = "windows-credential-manager"))]
            StoreKind::WindowsNative => bail!("Missing `windows-credential-manager` cargo feature"),
        }
    }
}

impl From<super::dispatch::Store> for Store {
    fn from(store: super::dispatch::Store) -> Self {
        match store {
            super::dispatch::Store::SecretService(s) => Self {
                store: StoreKind::SecretService,
                secret_service: Some(s),
                linux_keyutils: None,
                apple_native: None,
                windows_native: None,
            },
            super::dispatch::Store::Keyutils(s) => Self {
                store: StoreKind::LinuxKeyutils,
                secret_service: None,
                linux_keyutils: Some(s),
                apple_native: None,
                windows_native: None,
            },
            super::dispatch::Store::Macos(s) => Self {
                store: StoreKind::AppleNative,
                secret_service: None,
                linux_keyutils: None,
                apple_native: Some(s),
                windows_native: None,
            },
            super::dispatch::Store::Windows(s) => Self {
                store: StoreKind::WindowsNative,
                secret_service: None,
                linux_keyutils: None,
                apple_native: None,
                windows_native: Some(s),
            },
        }
    }
}
