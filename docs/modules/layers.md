# layers

## What it does

Contains the four verification checks that form wire-check's core logic.
Each layer is a public function returning `Vec<Diagnostic>`.

Layer 1 (annotation_ban) scans the crate root for `#[allow(dead_code)]` on
mod declarations — a pattern that hides unwired modules from the compiler.

Layer 2 (cross_reference) verifies every mod declared in the crate root is
actually imported from outside its own directory. Test modules are exempt.

Layer 3 (dead_code_ratchet) runs cargo check with force-warn dead_code,
then filters warnings to only newly added files. Existing dead code is
grandfathered; new code must be live.

Layer 4 (test_requirement) checks that when new source files are added,
at least one test file is also modified.

## Key types

- `Layer` — enum identifying which layer produced a diagnostic
- `Diagnostic` — layer, file path, and message
- `LayerError` — wraps GitError and CargoError

## Dependencies

Uses `git` module for file listings and `cargo` module for dead code detection.
