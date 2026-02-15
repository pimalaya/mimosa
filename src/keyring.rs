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

use anyhow::{anyhow, Context, Result};
use keyring_core::{Entry, Error};
use secrecy::{ExposeSecret, SecretString};

fn new_entry(service: &str, user: &str) -> Result<keyring_core::Entry> {
    Entry::new(service, user)
        .map_err(|err| anyhow!(err))
        .context("Cannot create keyring entry")
}

pub fn exists(service: &str, user: &str) -> Result<bool> {
    match new_entry(service, user)?.get_password() {
        Ok(_) => Ok(true),
        Err(Error::NoEntry) => Ok(false),
        Err(err) => Err(err.into()),
    }
}

pub fn read(service: &str, user: &str) -> Result<SecretString> {
    let password = new_entry(service, user)?
        .get_password()
        .map_err(|err| anyhow!(err))
        .context("Cannot read password from keyring")?;

    Ok(SecretString::from(password))
}

pub fn write(service: &str, user: &str, secret: SecretString) -> Result<()> {
    new_entry(service, user)?
        .set_password(secret.expose_secret())
        .map_err(|err| anyhow!(err))
        .context("Cannot write password to keyring")
}

pub fn remove(service: &str, user: &str) -> Result<bool> {
    match new_entry(service, user)?.delete_credential() {
        Ok(()) => Ok(true),
        Err(Error::NoEntry) => Ok(false),
        Err(err) => Err(anyhow!(err).context("Cannot remove password from keyring")),
    }
}
