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

#[cfg(feature = "command")]
use std::{
    io::{pipe, Write},
    process::Output,
};

#[allow(unused)]
use anyhow::{anyhow, bail, Context, Result};
#[cfg(feature = "command")]
use io_process::{
    command::Command,
    coroutines::spawn_then_wait_with_output::{
        SpawnThenWaitWithOutput, SpawnThenWaitWithOutputResult,
    },
    runtimes::std::handle as handle_process,
};
use serde::{Deserialize, Serialize};

use super::de;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(try_from = "de::Backend", rename_all = "kebab-case")]
pub enum Backend {
    #[cfg(feature = "keyring")]
    Keyring(KeyringBackend),
    #[cfg(feature = "command")]
    Command(CommandBackend),
}

#[cfg(feature = "keyring")]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct KeyringBackend {
    pub service: String,
    pub user: String,
}

#[cfg(feature = "command")]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct CommandBackend {
    pub get: Command,
    pub set: Command,
    pub delete: Command,
}

impl Backend {
    pub fn get_password(&self) -> Result<String> {
        match self {
            #[cfg(feature = "keyring")]
            Self::Keyring(config) => {
                init_keyring_store()?;

                let entry = keyring_core::Entry::new(&config.service, &config.user)
                    .map_err(|err| anyhow!("{err}"))
                    .context("Cannot create keyring entry")?;

                entry
                    .get_password()
                    .map_err(|err| anyhow!("{err}"))
                    .context("Cannot get password from keyring")
            }
            #[cfg(feature = "command")]
            Self::Command(config) => {
                let mut spawn = SpawnThenWaitWithOutput::new(config.get.clone());
                let mut arg = None;

                let Output {
                    status,
                    stdout,
                    stderr,
                } = loop {
                    match spawn.resume(arg.take()) {
                        SpawnThenWaitWithOutputResult::Ok(output) => break output,
                        SpawnThenWaitWithOutputResult::Io(io) => arg = Some(handle_process(io)?),
                        SpawnThenWaitWithOutputResult::Err(err) => {
                            let ctx = "Spawn get password command error";
                            return Err(anyhow!("{err}").context(ctx));
                        }
                    }
                };

                if !status.success() {
                    let bytes = if stdout.is_empty() { stderr } else { stdout };
                    let err = anyhow!("{}", String::from_utf8_lossy(&bytes));
                    return Err(err.context("Get password via command error"));
                }

                let password = String::from_utf8(stdout)
                    .context("Password command output is not valid UTF-8")?;

                Ok(password.trim_end_matches('\n').to_owned())
            }
        }
    }

    pub fn set_password(&self, password: &str) -> Result<()> {
        match self {
            #[cfg(feature = "keyring")]
            Self::Keyring(config) => {
                init_keyring_store()?;

                let entry = keyring_core::Entry::new(&config.service, &config.user)
                    .map_err(|err| anyhow!("{err}"))
                    .context("Cannot create keyring entry")?;

                entry
                    .set_password(password)
                    .map_err(|err| anyhow!("{err}"))
                    .context("Cannot set password in keyring")
            }
            #[cfg(feature = "command")]
            Self::Command(config) => {
                let mut cmd = config.set.clone();
                let data = password.as_bytes();
                let (reader, mut writer) = pipe()?;

                cmd.stdin(reader);
                writer.write_all(data)?;
                drop(writer);

                let mut spawn = SpawnThenWaitWithOutput::new(cmd);
                let mut arg = None;

                let Output {
                    status,
                    stdout,
                    stderr,
                } = loop {
                    match spawn.resume(arg.take()) {
                        SpawnThenWaitWithOutputResult::Ok(output) => break output,
                        SpawnThenWaitWithOutputResult::Io(io) => arg = Some(handle_process(io)?),
                        SpawnThenWaitWithOutputResult::Err(err) => {
                            let ctx = "Spawn set password command error";
                            return Err(anyhow!("{err}").context(ctx));
                        }
                    }
                };

                if !status.success() {
                    let bytes = if stdout.is_empty() { stderr } else { stdout };
                    if bytes.is_empty() {
                        bail!("Set password via command error");
                    }
                    let err = anyhow!("{}", String::from_utf8_lossy(&bytes));
                    return Err(err.context("Set password via command error"));
                }

                Ok(())
            }
        }
    }

    pub fn delete_password(&self) -> Result<()> {
        match self {
            #[cfg(feature = "keyring")]
            Self::Keyring(config) => {
                init_keyring_store()?;

                let entry = keyring_core::Entry::new(&config.service, &config.user)
                    .map_err(|err| anyhow!("{err}"))
                    .context("Cannot create keyring entry")?;

                entry
                    .delete_credential()
                    .map_err(|err| anyhow!("{err}"))
                    .context("Cannot delete password from keyring")
            }
            #[cfg(feature = "command")]
            Self::Command(config) => {
                let mut spawn = SpawnThenWaitWithOutput::new(config.delete.clone());
                let mut arg = None;

                let Output {
                    status,
                    stdout,
                    stderr,
                } = loop {
                    match spawn.resume(arg.take()) {
                        SpawnThenWaitWithOutputResult::Ok(output) => break output,
                        SpawnThenWaitWithOutputResult::Io(io) => arg = Some(handle_process(io)?),
                        SpawnThenWaitWithOutputResult::Err(err) => {
                            let ctx = "Spawn delete password command error";
                            return Err(anyhow!("{err}").context(ctx));
                        }
                    }
                };

                if !status.success() {
                    let bytes = if stdout.is_empty() { stderr } else { stdout };
                    if bytes.is_empty() {
                        bail!("Delete password via command error");
                    }
                    let err = anyhow!("{}", String::from_utf8_lossy(&bytes));
                    return Err(err.context("Delete password via command error"));
                }

                Ok(())
            }
        }
    }
}

#[cfg(feature = "keyring")]
fn init_keyring_store() -> Result<()> {
    #[cfg(feature = "dbus-secret-service")]
    {
        let store = dbus_secret_service_keyring_store::Store::new()
            .map_err(|err| anyhow!("{err}"))
            .context("Cannot create D-Bus Secret Service store")?;
        keyring_core::set_default_store(store);
        return Ok(());
    }

    #[cfg(feature = "apple-native")]
    {
        let store = apple_native_keyring_store::keychain::Store::new()
            .map_err(|err| anyhow!("{err}"))
            .context("Cannot create Apple Keychain store")?;
        keyring_core::set_default_store(store);
        return Ok(());
    }

    #[cfg(feature = "windows-native")]
    {
        let store = windows_native_keyring_store::Store::new()
            .map_err(|err| anyhow!("{err}"))
            .context("Cannot create Windows Credential store")?;
        keyring_core::set_default_store(store);
        return Ok(());
    }

    #[allow(unreachable_code)]
    bail!(
        "No keyring store available: enable a store feature \
         (dbus-secret-service, apple-native, or windows-native)"
    )
}
