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

//! Runtime dispatch enum for the four keyring backends. Built by the
//! [`super::de`] serde shim and consumed by every `password`
//! subcommand through the [`StoreExt`] trait.

use anyhow::Result;
use secrecy::SecretString;
use serde::{Deserialize, Serialize};

use crate::store::{
    de, keyutils::KeyutilsStore, macos::MacosStore, secret_service::SecretServiceStore,
    windows::WindowsStore,
};

/// CRUD operations every keyring backend must implement.
pub trait StoreExt {
    /// Reads the secret currently held by the backend.
    fn read(&self) -> Result<SecretString>;
    /// Writes (or overwrites) the secret held by the backend.
    fn write(&self, secret: SecretString) -> Result<()>;
    /// Removes the secret. Returns `Ok(true)` if an entry was
    /// deleted, `Ok(false)` if there was nothing to delete.
    fn remove(&self) -> Result<bool>;
}

/// Runtime store, selected at deserialization time by the `store`
/// field of the TOML schema. Every variant is unreachable when the
/// corresponding backend feature is disabled (the serde shim in
/// [`super::de`] `bail!`s before construction), hence the
/// `dead_code` allow for `--no-default-features` builds.
#[allow(dead_code)]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(try_from = "de::Store", into = "de::Store")]
pub enum Store {
    SecretService(SecretServiceStore),
    Keyutils(KeyutilsStore),
    Macos(MacosStore),
    Windows(WindowsStore),
}

impl StoreExt for Store {
    fn read(&self) -> Result<SecretString> {
        match self {
            Self::SecretService(s) => s.read(),
            Self::Keyutils(s) => s.read(),
            Self::Macos(s) => s.read(),
            Self::Windows(s) => s.read(),
        }
    }

    fn write(&self, secret: SecretString) -> Result<()> {
        match self {
            Self::SecretService(s) => s.write(secret),
            Self::Keyutils(s) => s.write(secret),
            Self::Macos(s) => s.write(secret),
            Self::Windows(s) => s.write(secret),
        }
    }

    fn remove(&self) -> Result<bool> {
        match self {
            Self::SecretService(s) => s.remove(),
            Self::Keyutils(s) => s.remove(),
            Self::Macos(s) => s.remove(),
            Self::Windows(s) => s.remove(),
        }
    }
}
