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

//! Synchronous std client behind the `mimosa` CLI: load a TOML
//! configuration with [`config::Config`], then read / write / remove
//! secrets through [`store::dispatch::Store`] which dispatches to one
//! of the four OS-native keyring backends (Secret Service, Linux
//! keyutils, Apple Keychain, Windows Credential Manager).
//!
//! There is intentionally no I/O-free core: every backend bottoms out
//! in [`keyring_core`], which is already a thin synchronous wrapper
//! over the platform store, so there is nothing meaningful to
//! abstract beneath it. The library exists so that other tools
//! (himalaya, neverest, …) can resolve a `[stores.<name>]` block to a
//! [`secrecy::SecretString`] without reimplementing the dispatch.

pub mod config;
pub mod store;

mod keyring;
