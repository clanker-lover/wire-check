# src/git.rs -- Git Operations

## What it does

Runs git commands as child processes to detect the base branch and list new or modified files. Three public functions: detect the default branch, list newly added `.rs` files (for the ratchet), and list all modified files (for the test requirement check). (Source: `src/git.rs`, `docs/modules/git.md`)

## Public API

- `detect_base_ref(project_root: &Path) -> Result<String, GitError>` -- tries "main", then "master", falls back to "HEAD~1". (Source: `src/git.rs:16-34`)
- `new_files(project_root, base_ref, test_modules, test_files) -> Result<Vec<PathBuf>, GitError>` -- `.rs` files added since base ref, excluding main.rs and test paths. Uses `git diff --diff-filter=A`. (Source: `src/git.rs:37-72`)
- `modified_files(project_root, base_ref) -> Result<Vec<PathBuf>, GitError>` -- all files changed since base ref. (Source: `src/git.rs:92-115`)

## Key types

- `GitError` -- enum: `NotARepo(PathBuf)`, `CommandFailed(String)`. Uses `thiserror`. (Source: `src/git.rs:7-13`)

## Internal functions

- `is_test_path(path, test_modules, test_files) -> bool` -- checks if a path matches configured test module names or test file patterns. Uses `/{module}.rs` and `/{module}/` patterns to avoid substring false positives (e.g., "attestation.rs" does not match module "tests"). (Source: `src/git.rs:75-89`)

## Dependencies

- `std::process::Command` for running git
- `thiserror` for error types
- No git library -- shells out to the `git` binary

(Source: `src/git.rs`, `docs/modules/git.md`)

## Architecture role

One of two modules allowed to run external commands (the other is `cargo.rs`). Called by `layers.rs` for Layers 3 and 4. (Source: `docs/ARCHITECTURE.md`)

## Tests (7 tests)

- `detect_base_ref_finds_master` -- works in a fresh git repo
- `detect_base_ref_fails_on_non_repo` -- returns `NotARepo` error
- `new_files_returns_empty_when_no_changes` -- no additions = empty
- `modified_files_returns_empty_when_no_changes` -- no changes = empty
- `is_test_path_matches_module_name` -- matches `connection_tests.rs` and subdirectories
- `is_test_path_does_not_match_substrings` -- "attestation.rs" does not match "tests"
- `is_test_path_matches_test_files` -- matches configured test file paths

(Source: `src/git.rs:117-223`)

## FACTS

- `new_files` uses `git diff --diff-filter=A -- src/**/*.rs` to find only added files.
- `main.rs` is always excluded from new file lists.
- Test path matching uses `/{module}.rs` prefix to avoid substring matches -- this was a bug fix documented in `docs/specs/fix-review-findings.md`.

## INFERENCES

- The `HEAD~1` fallback in `detect_base_ref` means wire-check works even on repos without a main/master branch (e.g., repos using "trunk" or "develop").

## OPEN QUESTIONS

- The `HEAD~1` fallback only compares against the previous commit, which may miss files added in earlier commits on a feature branch. This is a known limitation for non-standard branch naming.

## Cross-references

- [layers.rs summary](layers-rs.md) -- consumer of git functions
- [Dead code ratchet concept](../concepts/dead-code-ratchet.md)
- [External command isolation concept](../concepts/external-command-isolation.md)
- [Test path filtering concept](../concepts/test-path-filtering.md)
