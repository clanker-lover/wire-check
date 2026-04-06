# Entity: thiserror

## What it is

A Rust library for deriving `std::error::Error` implementations via `#[derive(Error)]` and `#[error("...")]` attributes. Produces boilerplate-free error types with proper `Display` and `Error` trait implementations.

## How wire-check uses it

Every module with fallible operations defines its own error enum using thiserror:

- `ConfigError` in `config.rs` -- with `#[from]` for `io::Error` and `toml::de::Error`
- `GitError` in `git.rs` -- manual string messages
- `CargoError` in `cargo.rs` -- manual string messages
- `LayerError` in `layers.rs` -- with `#[from]` for `GitError` and `CargoError`

The `#[from]` attribute enables automatic conversion via the `?` operator.

(Source: `src/config.rs:82-90`, `src/git.rs:7-13`, `src/cargo.rs:15-21`, `src/layers.rs:34-40`)

## Version

thiserror 2 (not v1). (Source: `Cargo.toml:29`)

## Cross-references

- [Error handling concept](../concepts/error-handling.md)
- [Cargo.toml summary](../summaries/cargo-toml.md)
