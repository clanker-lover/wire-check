# src/layers.rs -- The Four Verification Layers

## What it does

Contains the four verification checks that form wire-check's core logic. Each layer is a public function returning `Vec<Diagnostic>` (or `Result<Vec<Diagnostic>, LayerError>` for layers needing git/cargo). This is the business logic center of the tool. (Source: `src/layers.rs`, `docs/modules/layers.md`)

## Public API

- `check_annotation_ban(crate_root: &Path) -> Vec<Diagnostic>` -- Layer 1. (Source: `src/layers.rs:46-94`)
- `check_cross_references(project_root, crate_root, test_modules) -> Vec<Diagnostic>` -- Layer 2. (Source: `src/layers.rs:99-146`)
- `check_dead_code_ratchet(project_root, base_ref, test_modules, test_files) -> Result<Vec<Diagnostic>, LayerError>` -- Layer 3. (Source: `src/layers.rs:239-265`)
- `check_test_requirement(project_root, base_ref, test_modules, test_files) -> Result<Vec<Diagnostic>, LayerError>` -- Layer 4. (Source: `src/layers.rs:270-300`)

## Key types

- `Layer` -- enum: `AnnotationBan`, `CrossReference`, `DeadCodeRatchet`, `TestRequirement`. (Source: `src/layers.rs:9-15`)
- `Diagnostic` -- struct: `file: PathBuf`, `message: String`. (Source: `src/layers.rs:19-22`)
- `LayerResult` -- enum: `Ran(Vec<Diagnostic>)`, `Skipped`. (Source: `src/layers.rs:25-31`)
- `LayerError` -- wraps `GitError` and `CargoError` via `From`. (Source: `src/layers.rs:34-40`)

## Internal functions

- `parse_mod_name(line: &str) -> Option<String>` -- extracts module name from `mod foo;`, `pub mod bar;`, `pub(crate) mod baz;`. Skips `cfg(test)` modules. (Source: `src/layers.rs:149-173`)
- `find_reference_outside_module(src_dir, mod_name, patterns, test_modules) -> bool` -- searches all `.rs` files outside the module's own directory/file for any of the given patterns. (Source: `src/layers.rs:176-217`)
- `collect_rs_files(dir, files)` -- recursive directory walk collecting `.rs` files. (Source: `src/layers.rs:220-234`)

## Layer details

**Layer 1 (annotation_ban):** Scans crate root file line-by-line for `#![allow(...dead_code...)]` (crate-level) and `#[allow(...dead_code...)]` followed by a `mod` declaration (item-level). Text-pattern matching, not AST parsing. (Source: `src/layers.rs:46-94`)

**Layer 2 (cross_reference):** Parses `mod <name>` declarations from crate root. For each module (excluding test modules), searches all `.rs` files outside that module's directory for three patterns: `crate::{mod}::`, `use crate::{mod}`, `{mod}::`. No match = module is unwired. (Source: `src/layers.rs:99-146`)

**Layer 3 (dead_code_ratchet):** Gets newly added files from git, runs cargo check with force-warn, filters warnings to only new files. Empty new files list = automatic pass. (Source: `src/layers.rs:239-265`)

**Layer 4 (test_requirement):** Gets newly added files from git, checks if any configured test files were modified in the same diff. No test file updates = failure. (Source: `src/layers.rs:270-300`)

## Dependencies

- `crate::cargo` -- for dead code warnings (Layer 3)
- `crate::git` -- for file listings (Layers 3, 4)

(Source: `src/layers.rs:5-6`)

## Tests (9 tests)

- Layer 1: `annotation_ban_detects_crate_level`, `annotation_ban_detects_mod_level`, `annotation_ban_passes_clean_file`, `annotation_ban_ignores_non_mod_allow`
- Layer 2: `parse_mod_name_basic`, `parse_mod_name_skips_cfg_test`, `parse_mod_name_rejects_non_mod`, `cross_reference_detects_unwired_module`, `cross_reference_passes_wired_module`, `cross_reference_skips_test_modules`

(Source: `src/layers.rs:302-417`)

## FACTS

- Layer 2 uses text-pattern matching, not AST parsing. This is a deliberate trade-off documented in `README.md` Known Limitations.
- Layer 3 short-circuits (returns empty) if git reports zero new files.
- Layer 4 only checks if test files were modified, not whether the tests are meaningful.
- The `Diagnostic` struct does not carry a `Layer` field -- the layer is tracked externally via `(Layer, LayerResult)` tuples.

## INFERENCES

- The text-pattern approach for Layer 2 keeps the tool dependency-free (no syn, no rust-analyzer) but could produce false positives from references in comments or string literals.

## OPEN QUESTIONS

- Layer 2's bare path pattern (`{mod}::`) could match unrelated identifiers in large codebases. The README acknowledges this.

## Cross-references

- [main.rs summary](main-rs.md) -- orchestrates layer execution
- [cargo.rs summary](cargo-rs.md) -- provides dead code warnings
- [git.rs summary](git-rs.md) -- provides file listings
- [report.rs summary](report-rs.md) -- formats layer results
- [Four layers concept](../concepts/four-layers.md)
- [Dead code ratchet concept](../concepts/dead-code-ratchet.md)
- [Text-pattern matching concept](../concepts/text-pattern-matching.md)
