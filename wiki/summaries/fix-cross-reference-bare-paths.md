# docs/specs/fix-cross-reference-bare-paths.md -- Bare Path Fix Spec

## What it does

Specification for fixing Layer 2 (cross-reference check) to recognize bare module path references like `module::function()` used in main.rs without the `crate::` prefix. (Source: `docs/specs/fix-cross-reference-bare-paths.md`)

## Problem

Layer 2 originally searched only for `crate::module::` and `use crate::module`, missing bare `module::` references. In main.rs (the crate root), the `crate::` prefix is optional, so `layers::check_annotation_ban()` is a valid reference that was not being detected. (Source: `docs/specs/fix-cross-reference-bare-paths.md:3-6`)

## Fix

Add a third search pattern: `{mod_name}::`. Pass all three patterns to `find_reference_outside_module`. (Source: `docs/specs/fix-cross-reference-bare-paths.md:8-15`)

## FACTS

- This fix has been implemented in the current codebase (`src/layers.rs:127`).
- The fix is a single line addition plus threading the new pattern through.

## INFERENCES

- This is a natural consequence of the text-pattern matching approach -- new reference patterns need new search strings.

## OPEN QUESTIONS

- None identified.

## Cross-references

- [layers.rs summary](layers-rs.md) -- where the fix was applied
- [Text-pattern matching concept](../concepts/text-pattern-matching.md)
