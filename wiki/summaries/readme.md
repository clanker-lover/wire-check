# README.md -- Project Documentation

## What it does

Primary user-facing documentation for wire-check. Explains what the tool does, why it exists, how to install and use it, configuration options, CI integration, and known limitations. (Source: `README.md`)

## Key content

**Problem statement:** Every growing Rust project accumulates modules that compile but are not wired into anything. Existing tools (cargo clippy, cargo-modules orphans) either warn about all dead code project-wide or only find missing module tree entries. wire-check's innovation is the dead code ratchet: existing dead code is grandfathered, new code must be live. (Source: `README.md:9-16`)

**Installation:** `cargo install wire-check`. (Source: `README.md:20`)

**CLI options:** project_root (positional), `--config`, `--format` (human/json), `--base-ref`, `--layer`. (Source: `README.md:103-117`)

**Configuration:** `wire-check.toml` with `[project]`, `[layers]`, `[filters]` sections. All optional. (Source: `README.md:77-98`)

**Exit codes:** 0 = pass, 1 = fail, 2 = error. (Source: `README.md:52`)

**Known limitations:** Text-pattern cross-reference (no AST), requires git history, single-crate only. (Source: `README.md:151-154`)

**Related tools comparison:** cargo clippy, cargo-modules orphans, cargo-shear, cargo-udeps. (Source: `README.md:156-163`)

## FACTS

- Credits eslint-seatbelt by justjake and qntm's ratchet pattern article as inspiration.
- Doc-drift is used to verify module documentation against source code.
- GitHub Actions example included.

## INFERENCES

- The README positions wire-check as a CI gate tool, not an interactive development aid.

## OPEN QUESTIONS

- None identified.

## Cross-references

- [Dead code ratchet concept](../concepts/dead-code-ratchet.md)
- [Four layers concept](../concepts/four-layers.md)
- [eslint-seatbelt entity](../entities/eslint-seatbelt.md)
- [doc-drift entity](../entities/doc-drift.md)
