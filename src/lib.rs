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

#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
#![doc = include_str!("../README.md")]

#[cfg(not(any(feature = "keyring", feature = "command")))]
compile_error!(
    "At least one backend feature must be enabled: \
     keyring (dbus-secret-service, apple-native, windows-native) or command"
);

pub mod account;
pub mod backend;
pub mod cli;
pub mod config;
pub mod password;
