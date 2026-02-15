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

use anyhow::Result;
use secrecy::SecretString;
use serde::{Deserialize, Serialize};

use crate::store::{
    keyutils::KeyutilsStore, macos::MacosStore, secret_service::SecretServiceStore,
    windows::WindowsStore,
};

use super::de;

/// The contract every store must satisfy.
pub trait StoreExt {
    fn read(&self) -> Result<SecretString>;
    fn write(&self, secret: SecretString) -> Result<()>;
    fn remove(&self) -> Result<bool>;
}

/// A store configuration, resolved from the TOML config.
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
