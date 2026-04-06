# doc-drift.toml -- Documentation Drift Configuration

## What it does

Configures doc-drift, a tool that verifies module documentation stays in sync with source code. Maps source files in `src/` to their corresponding docs in `docs/modules/`. (Source: `doc-drift.toml`)

## Configuration

- **Sources:** `src/` directory, excluding `main.rs`
- **Docs:** `docs/modules/` directory
- **Required sections:** "What it does" (every module doc must have this)
- **Min word count:** 30 words per doc
- **Freshness checking:** disabled
- **Excluded attributes:** `cfg(test)` modules
- **Identifier whitelist:** `self`, `Self`
- **Mapping style:** flat, separator "-"

(Source: `doc-drift.toml`)

## FACTS

- `main.rs` is excluded from documentation requirements.
- Each source module must have a corresponding doc with at least 30 words and a "What it does" section.
- The mapping style "flat" with separator "-" means `src/foo_bar.rs` maps to `docs/modules/foo-bar.md`.

## INFERENCES

- Doc-drift is another tool by the same author (repository referenced in README.md). Wire-check's documentation structure was modeled after doc-drift's approach.

## OPEN QUESTIONS

- None identified.

## Cross-references

- [doc-drift entity](../entities/doc-drift.md)
- [Development methodology concept](../concepts/development-methodology.md)
