# AGENTS.md

`bg3-stats-calc` is a Rust CLI that simulates Baldur's Gate 3 combat math (attack rolls, damage rolls, armor class). Running it prints a scripted sequence of attacks; rolls use `rand`, so output varies between runs.

Standard commands (see `Cargo.toml`): `cargo build`, `cargo run`, `cargo test`, `cargo clippy`.

## Cursor Cloud specific instructions

- The crate is `edition = "2024"`, which requires Rust >= 1.85. The base VM image may leave an older toolchain (e.g. 1.83) as the rustup default, which fails at manifest parse with `feature 'edition2024' is required`. The startup update script installs and defaults to `stable`; if you still hit that error, run `rustup default stable`.
- `cargo run` is a non-interactive, self-contained simulation (no args, no input, no network); it exits on its own.
- There are no automated tests yet, so `cargo test` reports `0 tests` — that is expected, not a setup failure.
