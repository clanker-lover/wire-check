# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-03-27

### Added

- Initial release
- Four verification layers: annotation ban, cross-reference, dead code ratchet, test requirement
- Dead code ratchet pattern: grandfathers existing dead code, enforces on new files only
- Auto-detection of crate root (main.rs/lib.rs) and git base branch (main/master)
- TOML-based per-project configuration (all optional, sensible defaults)
- Human-readable and JSON output formats
- Single-layer mode (`--layer`) for targeted checks
- Runs on stable Rust (no nightly required)
