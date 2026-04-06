# Entity: doc-drift

## What it is

A documentation verification tool that checks module docs stay in sync with source code. Created by the same author as wire-check (Brandon / clanker-lover). Repository: https://github.com/clanker-lover/doc-drift

## Relationship to wire-check

- wire-check's MVP spec states the project is "modeled after doc-drift's structure and patterns." (Source: `docs/specs/wire-check-mvp.md:4`)
- wire-check uses doc-drift to verify its own module documentation via `doc-drift.toml`. (Source: `doc-drift.toml`, `README.md:148-149`)
- Both tools share the same author and similar design philosophy (lightweight CLI, text-based analysis, zero-config defaults).

## Configuration in wire-check

The `doc-drift.toml` maps `src/` files (excluding `main.rs`) to `docs/modules/` with required "What it does" sections and minimum 30-word docs. (Source: `doc-drift.toml`)

## Cross-references

- [doc-drift.toml summary](../summaries/doc-drift-toml.md)
- [MVP spec summary](../summaries/mvp-spec.md)
- [README summary](../summaries/readme.md)
