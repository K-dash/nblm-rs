# Contributing to nblm-rs

Thank you for your interest in contributing to nblm-rs!

## Prerequisites

- Rust 1.90.0 or later
- [cargo-make](https://github.com/sagiegurari/cargo-make) - Install with `cargo install cargo-make`

## Development Workflow

### 1. Fork and Clone

```bash
git clone https://github.com/yourusername/nblm-rs.git
cd nblm-rs
```

### 2. Make Your Changes

Create a new branch for your changes:

```bash
git checkout -b feature/your-feature-name
```

### 3. Run All Checks

Before submitting a pull request, ensure that all checks pass:

```bash
cargo make all
```

This command runs:
- `cargo fmt --all` - Format code
- `cargo clippy --all-targets --all-features -- -D warnings` - Lint code (zero warnings required)
- `cargo test --all` - Run all tests (60 tests should pass)

> [!IMPORTANT]
> All pull requests must pass `cargo make all` before being merged.

### 4. Additional Commands

```bash
# Format code only
cargo make fmt

# Run linter only
cargo make lint

# Run tests only
cargo make test

# Run CI checks (used in GitHub Actions)
cargo make ci

# Generate coverage report
cargo make coverage
```

## Testing

### Running Tests

```bash
# Run all tests (unit + integration)
cargo test --all

# Run only unit tests
cargo test --lib

# Run only integration tests
cargo test --test '*'

# Run specific test file
cargo test --test notebooks_delete
```

All 60 tests should pass:
- 29 unit tests
- 31 integration tests

### Adding New Tests

When adding new features:

1. Add unit tests in the same file as your implementation
2. Add integration tests in `crates/nblm-cli/tests/`
3. Use the test helpers in `crates/nblm-cli/tests/_helpers/`
4. Follow existing test patterns for consistency

## Code Style

- Follow Rust standard formatting (`cargo fmt`)
- Keep clippy warnings at zero (`cargo clippy`)
- Use meaningful variable and function names
- Add documentation comments for public APIs
- Document any API issues or limitations you discover

## Documenting API Issues

If you discover issues with the NotebookLM API:

1. Add comments in the code explaining the issue
2. Add warnings in CLI help text if it affects user experience
3. Document in README.md's "Known API Issues" section
4. Include verification date in your documentation

Example:
```rust
/// Delete notebooks.
///
/// # Known Issues (as of 2025-10-19)
///
/// Despite the API being named "batchDelete", it only accepts one notebook
/// at a time. This function works around this limitation by calling the API
/// sequentially.
```

## Pull Request Process

1. Ensure `cargo make all` passes
2. Update tests.md if you've added new test cases
3. Update README.md if you've added new features
4. Write clear commit messages
5. Reference any related issues in your PR description

## Project Structure

```
nblm-rs/
├── crates/
│   ├── nblm-core/          # Core API client and models
│   │   ├── src/
│   │   │   ├── auth.rs     # Authentication methods
│   │   │   ├── client.rs   # HTTP client and API calls
│   │   │   ├── models.rs   # Request/response types
│   │   │   └── lib.rs
│   │   └── tests/          # Unit tests
│   └── nblm-cli/           # CLI interface
│       ├── src/
│       │   ├── args.rs     # CLI argument definitions
│       │   ├── ops/        # Command implementations
│       │   └── main.rs
│       └── tests/          # Integration tests
├── Makefile.toml           # cargo-make tasks
└── README.md
```

## Getting Help

- Check existing issues for similar problems
- Read the [NotebookLM API documentation](https://cloud.google.com/gemini/enterprise/notebooklm-enterprise/docs)
- Ask questions in issue discussions

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
