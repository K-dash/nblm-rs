# Contributing to nblm-rs

Thank you for your interest in contributing to nblm-rs!

## Prerequisites

### Rust Development

- Rust 1.90.0 or later
- Task runner: [cargo-make](https://github.com/sagiegurari/cargo-make)

```bash
# Option 1: cargo-make (traditional)
cargo install cargo-make

# Option 2: makers (faster alternative)
cargo install makers
```

We rely on `cargo-make` for every task (locally and in CI), so please install it before running any project commands.

### Python Development

- Python 3.14
- [uv](https://docs.astral.sh/uv/) - Python package manager

```bash
# Install uv
curl -LsSf https://astral.sh/uv/install.sh | sh
```

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

#### Rust Projects

```bash
# Using cargo-make
cargo make all

# Or using makers
makers all
```

This command runs:

- `cargo fmt --all` - Format code
- `cargo clippy --all-targets --all-features -- -D warnings` - Lint code (zero warnings required)
- `cargo test --all` - Run all tests

> [!IMPORTANT]
> All pull requests must pass `cargo make all` or `makers all` before being merged.

#### Python Package

```bash
# Run all Python checks
cargo make py-all    # or: makers py-all

# Build Python package
cargo make py-build  # or: makers py-build
```

The `py-all` command runs:

- `ruff format` - Format Python code
- `ruff check --fix` - Lint and auto-fix issues
- `mypy` - Type checking

> [!IMPORTANT]
> If you modify Python bindings (`crates/nblm-python/`) or Python code (`python/`), ensure all Python checks pass.

### 4. Additional Commands

#### Rust Commands

```bash
# Format code only
cargo make fmt    # or: makers fmt

# Run linter only
cargo make lint   # or: makers lint

# Run tests only
cargo make test   # or: makers test

# Run CI checks (used in GitHub Actions)
cargo make ci     # or: makers ci

# Generate coverage report
cargo make coverage   # or: makers coverage
```

#### Python Commands

```bash
# Format Python code
cargo make py-fmt           # or: makers py-fmt

# Check formatting (CI)
cargo make py-fmt-check     # or: makers py-fmt-check

# Lint Python code
cargo make py-lint          # or: makers py-lint

# Lint and fix
cargo make py-lint-fix      # or: makers py-lint-fix

# Type checking
cargo make py-type          # or: makers py-type

# Run Python tests
cargo make py-test          # or: makers py-test

# Build Python package with maturin
cargo make py-build         # or: makers py-build
```

### Adding New Tests

#### Rust Tests

When adding new Rust features:

1. Add unit tests in the same file as your implementation
2. Add integration tests in `crates/nblm-cli/tests/`
3. Use the test helpers in `crates/nblm-cli/tests/_helpers/`
4. Follow existing test patterns for consistency

#### Python Tests

When adding new Python features:

1. Add tests in `python/tests/` directory
2. Use pytest conventions and fixtures
3. Test files should be named `test_*.py`
4. Run tests with `cargo make py-test` or `makers py-test`

## Code Style

### Rust

- Follow Rust standard formatting (`cargo fmt`)
- Keep clippy warnings at zero (`cargo clippy`)
- Use meaningful variable and function names
- Add documentation comments for public APIs
- Document any API issues or limitations you discover

### Python

- Follow PEP 8 style guide (enforced by `ruff format`)
- Type hints are required for all public functions (`mypy`)
- Keep `ruff` linting warnings at zero
- Use descriptive variable and function names
- Add docstrings for public APIs

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

1. Ensure all relevant checks pass:
   - Rust: `cargo make all` or `makers all`
   - Python (if modified): `cargo make py-all` and `cargo make py-build`
2. Update documentation if you've added new features
3. Add or update tests for your changes
4. Write clear commit messages
5. Reference any related issues in your PR description

## Project Structure

```
nblm-rs/
├── crates/
│   ├── nblm-core/          # Core API client and models
│   ├── nblm-cli/           # CLI interface
│   └── nblm-python/        # Python bindings (PyO3)
├── python/                 # Python package (tests, config, generated .so)
├── .github/workflows/      # CI/CD workflows
├── docs/                   # Documentation
└── Makefile.toml           # cargo-make tasks
```

## Getting Help

- Check existing issues for similar problems
- Read the [NotebookLM API documentation](https://cloud.google.com/gemini/enterprise/notebooklm-enterprise/docs)
- Ask questions in issue discussions

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
