# Concept: Error Handling Strategy

## What it is

wire-check uses a layered error handling strategy: each module defines its own error type using `thiserror`, and `main.rs` handles all errors at the top level with `eprintln!` + `process::exit(2)`.

## Per-module error types

| Module | Error type | Variants |
|--------|-----------|----------|
| `config.rs` | `ConfigError` | `NotFound(PathBuf)`, `Read(io::Error)`, `Parse(toml::de::Error)` |
| `git.rs` | `GitError` | `NotARepo(PathBuf)`, `CommandFailed(String)` |
| `cargo.rs` | `CargoError` | `CommandFailed(String)`, `ParseFailed(String)` |
| `layers.rs` | `LayerError` | `Git(GitError)`, `Cargo(CargoError)` |

All use `thiserror::Error` derive with `#[error("...")]` attributes for Display. `LayerError` uses `#[from]` for automatic conversion from `GitError` and `CargoError`.

(Source: `src/config.rs:82-90`, `src/git.rs:7-13`, `src/cargo.rs:15-21`, `src/layers.rs:34-40`)

## Top-level error handling

`main.rs` does NOT return `Result` from `main()`. Instead, it uses `match`/`if let` at each step and calls `eprintln!("{error message}")` followed by `process::exit(2)`. This produces clean, user-facing error messages without stack traces or error chain formatting.

(Source: `src/main.rs:43-50`, `src/main.rs:69-74`, `src/main.rs:100-105`)

## Exit code semantics

- 0: all checks passed
- 1: at least one diagnostic failure (expected condition, not an error)
- 2: configuration or runtime error (unexpected condition)

This is a common CLI convention: 0 = success, 1 = expected failure, 2 = unexpected error.

(Source: `src/main.rs`, `README.md:52`)

## Infallible vs fallible layers

Layers 1 and 2 are infallible (return `Vec<Diagnostic>` directly). If file reads fail, they return empty diagnostics. Layers 3 and 4 are fallible (return `Result<Vec<Diagnostic>, LayerError>`) because they depend on git and cargo commands that can fail.

(Source: `src/layers.rs`)

## No unwrap policy

The project enforces no `unwrap()` or `expect()` outside test code. This is both a methodology rule and a CLAUDE.md constraint.

(Source: `docs/METHODOLOGY.md:36`, `CLAUDE.md:59`, `CONTRIBUTING.md:30`)

## Cross-references

- [Orchestrator pattern concept](orchestrator-pattern.md)
- [thiserror entity](../entities/thiserror.md)
- [Development methodology concept](development-methodology.md)
