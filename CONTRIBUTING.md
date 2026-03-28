# Contributing

Contributions are welcome. Here's how.

## Reporting Issues

Open an issue on GitHub with:
- What you expected
- What happened instead
- Steps to reproduce
- Your Rust toolchain version (`rustc --version`)

## Pull Requests

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Ensure all checks pass:
   ```bash
   cargo fmt --check
   cargo clippy --all-targets -- -D warnings
   cargo test
   ```
5. Open a pull request with a clear description of the change

## Code Style

- `cargo fmt` for formatting
- `cargo clippy -- -D warnings` must pass
- No `unwrap()` or `expect()` outside test code
- Default to `pub(crate)` visibility
- Every public function needs at least one test

## License

By contributing, you agree that your contributions will be licensed under both
MIT and Apache 2.0, matching the project's dual license.
