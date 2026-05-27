# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed

- Migrated build/runtime deps from `pimalaya-toolbox` to `pimalaya-cli` (terminal + build) and `pimalaya-config` (toml). The `TomlConfig` trait method names changed: `find_default_account` / `find_account` became `take_default_account` / `take_named_account`, and `from_paths_or_default` now returns `Option<Self>`.
- Replaced the `--quiet` / `--debug` / `--trace` boolean flags with the unified `--log-level <LEVEL>` and `--log-file <PATH>` parameters from `pimalaya-cli::clap::args::LogFlags`. `RUST_LOG` is still honoured when `--log-level` is omitted.
- Renamed `-c`/`--config` to accept a `:`-separated list of paths (first is the base, rest are deep-merged) and added the `MIMOSA_CONFIG` environment variable as a fallback.
- Bumped keyring-store dependencies (`dbus-secret-service-keyring-store`, `zbus-secret-service-keyring-store`, `linux-keyutils-keyring-store`, `apple-native-keyring-store`, `windows-native-keyring-store`) and `keyring-core` to the `1.x` line.
- Renamed `src/store/store.rs` to `src/store/dispatch.rs`; the dispatch `Store` enum and `StoreExt` trait are now reached via `mimosa::store::dispatch::{Store, StoreExt}`.
- Split the crate into a `std` library (`src/lib.rs`, exposing `config`, `store`) and a binary (`src/main.rs`, declaring CLI-only `cli`, `password` modules). The library is what `himalaya`, `neverest` and friends can depend on to resolve a `[stores.<name>]` block to a `secrecy::SecretString`. There is intentionally no I/O-free core: every backend bottoms out in `keyring_core`, so nothing meaningful sits below.
- Refactored the `keyring` module from three free functions taking `(service, user)` to a `keyring::Entry` struct with `new` / `read` / `write` / `remove` methods; each backend now exposes a private `fn entry(&self) -> Result<keyring::Entry>` and dispatches through it.
- Aligned author attribution and `src/**/*.rs` license headers on `soywod <pimalaya.org@posteo.net>`.
- Aligned the README with the Pimalaya CLI layout (matching neverest's section order: Pre-built binary / Cargo / Nix / Sources, then Configuration, Usage, Social, Sponsoring). Header keeps the 🔑 emoji and plain-markdown badge run (Documentation / Matrix / Mastodon).
- Rewrote `config.sample.toml` so the `store` discriminator values match what the deserializer actually accepts (`secret-service`, `linux-keyutils`, `apple-native`, `windows-native`) and documented every per-backend block.

### Fixed

- `Store::Keyutils` variant was unreachable on `--features keyutils` builds (the serde shim used a non-existent `Self::LinuxKeyutils` constructor).

### Added

- `deny.toml` with the canonical `[sources]` / `[licenses]` allowlists.
- `rust-version = "1.87"` to `Cargo.toml`.
- Inline `///` documentation on every public type, trait method, and per-backend struct field; module-level `//!` headers on every `*.rs` file under `src/`.
- Inline unit tests covering `config.sample.toml` deserialization and the `Store` ⇄ `de::Store` round-trip.

## [1.0.0] - 2026-02-15

### Added

- Init repository, taking inspiration from Ortie CLI

[unreleased]: https://github.com/pimalaya/mimosa/compare/v1.0.0...master
[1.0.0]: https://github.com/pimalaya/mimosa/compare/root...v1.0.0
