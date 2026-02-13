// This file is part of Mimosa, a CLI to manage secrets.
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
use anyhow::{bail, Error};
use serde::Deserialize;

#[cfg(feature = "command")]
use super::CommandBackend;
#[cfg(feature = "keyring")]
use super::KeyringBackend;

#[cfg(not(feature = "keyring"))]
pub type KeyringBackend = ();
#[cfg(not(feature = "command"))]
pub type CommandBackend = ();

#[allow(unused)]
use pimalaya_toolbox::feat;

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Backend {
    #[cfg_attr(not(feature = "keyring"), serde(deserialize_with = "keyring"))]
    Keyring(KeyringBackend),
    #[cfg_attr(not(feature = "command"), serde(deserialize_with = "command"))]
    Command(CommandBackend),
}

impl TryFrom<Backend> for super::Backend {
    type Error = Error;

    fn try_from(backend: Backend) -> Result<Self, Self::Error> {
        match backend {
            #[cfg(feature = "keyring")]
            Backend::Keyring(cfg) => Ok(Self::Keyring(cfg)),
            #[cfg(not(feature = "keyring"))]
            Backend::Keyring(_) => bail!(feat!("keyring")),

            #[cfg(feature = "command")]
            Backend::Command(cfg) => Ok(Self::Command(cfg)),
            #[cfg(not(feature = "command"))]
            Backend::Command(_) => bail!(feat!("command")),
        }
    }
}

#[cfg(not(feature = "keyring"))]
pub fn keyring<'de, T, D: serde::Deserializer<'de>>(_: D) -> Result<T, D::Error> {
    Err(serde::de::Error::custom(feat!("keyring")))
}

#[cfg(not(feature = "command"))]
pub fn command<'de, T, D: serde::Deserializer<'de>>(_: D) -> Result<T, D::Error> {
    Err(serde::de::Error::custom(feat!("command")))
}
