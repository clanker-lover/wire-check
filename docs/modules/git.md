# git

## What it does

Runs git commands as child processes to detect the base branch and list
new or modified files. `detect_base_ref` tries main, then master, falls
back to HEAD~1. `new_files` returns .rs files added since the base ref
(excluding test files and main.rs). `modified_files` returns all changed
files for the test requirement check.

## Key types

- `GitError` — not a repo, or command failed

## Dependencies

Uses `std::process::Command` to run git. No git library dependency.
