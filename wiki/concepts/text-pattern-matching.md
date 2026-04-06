# Concept: Text-Pattern Matching (vs AST Parsing)

## What it is

wire-check uses text-based string searching rather than AST (Abstract Syntax Tree) parsing to detect code references. This is a deliberate design trade-off affecting Layers 1 and 2.

## Where it appears

**Layer 1 (annotation ban):** Checks for `#[allow(dead_code)]` and `#![allow(dead_code)]` using `starts_with` and `contains` on trimmed lines. (Source: `src/layers.rs:60-71`)

**Layer 2 (cross-reference):** Searches for `crate::{mod}::`, `use crate::{mod}`, and `{mod}::` using `String::contains()` across all source files. (Source: `src/layers.rs:124-133`)

**Mod name parsing:** Uses `strip_prefix` on `"mod "`, `"pub mod "`, `"pub(crate) mod "` prefixes rather than parsing a full Rust syntax tree. (Source: `src/layers.rs:149-173`)

## Trade-offs

**Advantages:**
- No dependency on `syn`, `rust-analyzer`, or any Rust parser library
- Fast -- simple string operations
- Easy to understand and modify
- Zero nightly requirements

**Disadvantages:**
- Can produce false positives from references in comments or string literals
- Can miss references through re-exports (`pub use`)
- The bare path pattern (`{mod}::`) could match unrelated identifiers
- Cannot distinguish between different items with the same module name in different contexts

(Source: `README.md:152-153`)

## Why this trade-off was made

The README states this explicitly: "This is sufficient for typical Rust projects and keeps the tool dependency-free (no syn, no rust-analyzer), but it's worth knowing." The tool was designed as a lightweight CI gate, not a comprehensive static analysis tool.

(Source: `README.md:152`)

## Cross-references

- [layers.rs summary](../summaries/layers-rs.md)
- [Four layers concept](four-layers.md)
- [Fix cross-reference bare paths summary](../summaries/fix-cross-reference-bare-paths.md)
