# Concept: The Four Verification Layers

wire-check runs four independent verification layers, each catching a different class of "compiles but isn't connected" problem. They execute sequentially but are logically independent -- each can be enabled/disabled individually via config or the `--layer` CLI flag.

## Layer 1: Annotation Ban

**What it catches:** `#[allow(dead_code)]` on module declarations in the crate root, and crate-level `#![allow(dead_code)]`. These annotations hide unwired modules from the compiler's dead code detection.

**How it works:** Text-pattern scan of the crate root file. Checks for `#![allow(...dead_code...)]` anywhere and `#[allow(...dead_code...)]` immediately before `mod` declarations. No external commands needed.

(Source: `src/layers.rs:42-94`, `README.md:58`)

## Layer 2: Cross-Reference Check

**What it catches:** Modules declared in the crate root (`mod foo;`) but never imported or referenced from outside their own directory. These modules exist in the module tree but nothing uses them.

**How it works:** Parses `mod <name>` from the crate root, then searches all `.rs` files outside each module's directory for three patterns: `crate::{mod}::`, `use crate::{mod}`, `{mod}::`. Text-based, not AST-based. Test modules are exempt.

(Source: `src/layers.rs:96-217`, `README.md:59`, `docs/specs/fix-cross-reference-bare-paths.md`)

## Layer 3: Dead Code Ratchet

**What it catches:** Dead code in newly added files only. Existing dead code is grandfathered. This is the key innovation -- see [Dead code ratchet concept](dead-code-ratchet.md).

**How it works:** Gets newly added `.rs` files from git diff, runs `cargo check` with `--force-warn dead_code`, filters warnings to only those in new files. Zero new files = automatic pass.

(Source: `src/layers.rs:236-265`, `src/cargo.rs`, `src/git.rs:37-72`)

## Layer 4: Test Requirement

**What it catches:** New source files added without corresponding test file updates. Ensures new code comes with tests.

**How it works:** Gets newly added files from git diff, checks if any configured test files were modified in the same diff. Only checks that test files were touched, not that the tests are meaningful.

(Source: `src/layers.rs:267-300`, `src/git.rs:92-115`)

## Shared patterns

All layers return `Vec<Diagnostic>` (empty = pass). Layers 3 and 4 return `Result<Vec<Diagnostic>, LayerError>` because they can fail (git/cargo errors). Layers 1 and 2 are infallible (file read failures return empty diagnostics).

(Source: `src/layers.rs`, `docs/ARCHITECTURE.md:57-58`)

## Cross-references

- [layers.rs summary](../summaries/layers-rs.md)
- [Dead code ratchet concept](dead-code-ratchet.md)
- [Text-pattern matching concept](text-pattern-matching.md)
- [Test path filtering concept](test-path-filtering.md)
