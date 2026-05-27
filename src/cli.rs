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

//! Top-level clap definitions: the [`Cli`] parser and the
//! [`MimosaCommand`] subcommand dispatch that routes to the
//! `password` family plus the `manuals` / `completions` helpers
//! provided by [`pimalaya_cli`].

use std::path::PathBuf;

use anyhow::{bail, Result};
use clap::{CommandFactory, Parser, Subcommand};
use mimosa::config::Config;
use pimalaya_cli::{
    clap::{
        args::{JsonFlag, LogFlags},
        commands::{CompletionCommand, ManualCommand},
        parsers::path_parser,
    },
    long_version,
    printer::Printer,
};
use pimalaya_config::toml::TomlConfig;

use crate::password::PasswordCommand;

#[derive(Parser, Debug)]
#[command(name = env!("CARGO_PKG_NAME"))]
#[command(author, version, about)]
#[command(long_version = long_version!())]
#[command(propagate_version = true, infer_subcommands = true)]
#[command(arg_required_else_help = true)]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Command,

    /// Override the default configuration file path.
    ///
    /// The given paths are shell-expanded then canonicalized (if
    /// applicable). If the first path does not point to a valid file,
    /// the command bails. Other paths are merged with the first one,
    /// which allows you to separate your public config from your
    /// private(s) one(s). Multiple paths can also be provided by
    /// delimiting them with `:` (like `$PATH` in a POSIX shell).
    #[arg(short, long = "config", global = true, env = "MIMOSA_CONFIG")]
    #[arg(value_name = "PATH", value_parser = path_parser, value_delimiter = ':')]
    pub config_paths: Vec<PathBuf>,
    #[command(flatten)]
    pub json: JsonFlag,
    #[command(flatten)]
    pub log: LogFlags,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    #[command(subcommand)]
    Password(PasswordCommand),
    Manuals(ManualCommand),
    Completions(CompletionCommand),
}

impl Command {
    pub fn execute(self, printer: &mut impl Printer, config_paths: &[PathBuf]) -> Result<()> {
        match self {
            Self::Password(cmd) => {
                let Some(config) = Config::from_paths_or_default(config_paths)? else {
                    bail!(
                        "Cannot find configuration file, create one at {}",
                        Config::target_path(config_paths)?.display(),
                    );
                };
                cmd.execute(printer, &config)
            }
            Self::Manuals(cmd) => cmd.execute(printer, Cli::command()),
            Self::Completions(cmd) => cmd.execute(printer, Cli::command()),
        }
    }
}
