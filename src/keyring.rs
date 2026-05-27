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

//! Thin wrapper around [`keyring_core::Entry`] mapping its errors to
//! `anyhow::Error` with a human-readable context. Every backend in
//! [`crate::store`] resolves to a single [`Entry`] after installing
//! its concrete store via [`keyring_core::set_default_store`].

use anyhow::{anyhow, Context, Result};
use keyring_core::Error;
use secrecy::{ExposeSecret, SecretString};

/// Handle to a `(service, user)` entry on the currently installed
/// keyring store.
pub struct Entry(keyring_core::Entry);

impl Entry {
    /// Resolves `(service, user)` against the currently installed
    /// keyring store, returning a handle to operate on.
    pub fn new(service: &str, user: &str) -> Result<Self> {
        keyring_core::Entry::new(service, user)
            .map(Self)
            .map_err(|err| anyhow!(err))
            .context("Cannot create keyring entry")
    }

    /// Reads the secret currently held by the entry.
    pub fn read(&self) -> Result<SecretString> {
        let password = self
            .0
            .get_password()
            .map_err(|err| anyhow!(err))
            .context("Cannot read password from keyring")?;

        Ok(SecretString::from(password))
    }

    /// Writes (or overwrites) the secret held by the entry.
    pub fn write(&self, secret: SecretString) -> Result<()> {
        self.0
            .set_password(secret.expose_secret())
            .map_err(|err| anyhow!(err))
            .context("Cannot write password to keyring")
    }

    /// Removes the entry. Returns `Ok(true)` when a credential was
    /// deleted, `Ok(false)` when there was nothing to delete, and
    /// `Err` for any other backend failure.
    pub fn remove(&self) -> Result<bool> {
        match self.0.delete_credential() {
            Ok(()) => Ok(true),
            Err(Error::NoEntry) => Ok(false),
            Err(err) => Err(anyhow!(err).context("Cannot remove password from keyring")),
        }
    }
}
