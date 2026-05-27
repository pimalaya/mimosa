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

//! The `password` family of subcommands: read, write, remove. Every
//! variant carries a store name that must match a `[stores.<name>]`
//! block in the configuration file.

pub mod read;
pub mod remove;
pub mod write;

use anyhow::Result;
use clap::Subcommand;
use mimosa::config::Config;
use pimalaya_cli::printer::Printer;

use crate::password::{
    read::ReadPasswordCommand, remove::RemovePasswordCommand, write::WritePasswordCommand,
};

/// Read, write, or remove a password in a configured store.
#[derive(Subcommand, Debug)]
pub enum PasswordCommand {
    #[command(visible_aliases = ["get", "show"])]
    Read(ReadPasswordCommand),
    #[command(visible_aliases = ["set", "update", "edit"])]
    Write(WritePasswordCommand),
    #[command(visible_aliases = ["rm", "delete", "del"])]
    Remove(RemovePasswordCommand),
}

impl PasswordCommand {
    pub fn execute(self, printer: &mut impl Printer, config: &Config) -> Result<()> {
        match self {
            Self::Read(cmd) => cmd.execute(printer, config),
            Self::Write(cmd) => cmd.execute(printer, config),
            Self::Remove(cmd) => cmd.execute(printer, config),
        }
    }
}
