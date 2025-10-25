[![Crates.io](https://img.shields.io/crates/v/nblm-cli.svg)](https://crates.io/crates/nblm-cli)
[![PyPI](https://img.shields.io/pypi/v/nblm.svg)](https://pypi.org/project/nblm/)
[![codecov](https://codecov.io/gh/K-dash/nblm-rs/graph/badge.svg?token=OhxeTdnxTw)](https://codecov.io/gh/K-dash/nblm-rs)

# nblm-rs

Unofficial NotebookLM Enterprise API client written in Rust, available as:

- 🦀 **Rust CLI**: Command-line tool for shell scripting and automation
- 🐍 **Python SDK**: Python bindings for integration in Python applications

> [!WARNING]
> This is an unofficial tool and is not affiliated with or endorsed by Google. Use at your own risk.

## Motivation

Why use this CLI instead of direct API calls or web UI?

- **Credential caching**: Leverages gcloud authentication cache - no need to specify API keys or tokens repeatedly
- **Type safety**: Rust's type system catches errors at compile time, preventing runtime failures
- **Batch operations**: Add multiple sources to a notebook in a single command
- **Better error handling**: Clear, actionable error messages with helpful guidance
- **Automatic retries**: Built-in retry logic with exponential backoff for transient failures
- **JSON output**: Machine-readable format for automation and scripting workflows
- **Cross-platform**: Single binary works on Linux and macOS
- **Developer-friendly**: Command-line interface integrates seamlessly with shell scripts and CI/CD pipelines

> [!NOTE]
> Windows is not supported.

## Features (Verified as of 2025-10-25)

> [!NOTE]
> The NotebookLM API is currently in alpha. Some features may not work as documented due to API limitations. See [Known API Issues](#known-api-issues) for details.

### Notebooks

| Feature               | CLI | Python | Status  | Notes                                |
| --------------------- | --- | ------ | ------- | ------------------------------------ |
| Create notebook       | ◯   | ◯      | Working |                                      |
| List recent notebooks | ◯   | ◯      | Working | Pagination not implemented by API    |
| Delete notebook(s)    | ◯   | ◯      | Working | Sequential deletion (API limitation) |

### Sources

| Feature             | CLI | Python | Status      | Notes                   |
| ------------------- | --- | ------ | ----------- | ----------------------- |
| Add web URL         | ◯   | ◯      | Working     |                         |
| Add text content    | ◯   | ◯      | Working     |                         |
| Add video (YouTube) | ◯   | ◯      | Working     | Uses `youtubeUrl` field |
| Add Google Drive    | ◯   | ✗      | Not Working | API returns HTTP 500    |
| Upload file         | ◯   | ◯      | Working     |                         |
| Delete source(s)    | ◯   | ◯      | Working     |                         |
| Get source by ID    | ◯   | ◯      | Working     |                         |

### Audio Overview

| Feature               | CLI | Python | Status  | Notes                       |
| --------------------- | --- | ------ | ------- | --------------------------- |
| Create audio overview | ◯   | ◯      | Working | Config fields not supported |
| Delete audio overview | ◯   | ◯      | Working |                             |

### Sharing

| Feature        | CLI | Python | Status   | Notes                     |
| -------------- | --- | ------ | -------- | ------------------------- |
| Share notebook | ◯   | ✗      | Untested | Requires additional users |

## Installation

### CLI (Rust)

```bash
# From crates.io
cargo install nblm-cli

# Or build from source
git clone https://github.com/K-dash/nblm-rs.git
cd nblm-rs
cargo build --release
```

### Python SDK

```bash
pip install nblm
```

**Prerequisites**: Google Cloud project with NotebookLM API enabled

> **Detailed Installation Guide**: See [Installation Documentation](docs/) for platform-specific instructions and troubleshooting.

## Quick Start

### CLI

```bash
# 1. Authenticate
gcloud auth login

# 2. Set environment variables
export NBLM_PROJECT_NUMBER="123456789012"  # Get from GCP console
export NBLM_LOCATION="global"
export NBLM_ENDPOINT_LOCATION="global"

# 3. Create a notebook
nblm --auth gcloud notebooks create --title "My Notebook"

# 4. Add a source
nblm --auth gcloud sources add \
  --notebook-id YOUR_NOTEBOOK_ID \
  --web-url "https://example.com" \
  --web-name "Example"
```

### Python

```python
from nblm import NblmClient, GCloudTokenProvider

# Initialize client
client = NblmClient(
    token_provider=GCloudTokenProvider(),
    project_number="123456789012"
)

# Create a notebook
notebook = client.create_notebook("My Notebook")

# Add sources
client.add_sources(
    notebook_id=notebook.notebook_id,
    web_sources=[{"url": "https://example.com", "name": "Example"}]
)
```

## Documentation

**Complete guides and API references:**

- [Getting Started](docs/getting-started/installation.md) - Installation, authentication, configuration
- [CLI Reference](docs/cli/README.md) - All commands, options, and examples
- [Python SDK Reference](docs/python/README.md) - API reference and usage patterns
- [API Limitations](docs/api/limitations.md) - Known issues and workarounds

## Known API Issues

> [!NOTE]
> The NotebookLM API is currently in **alpha** and has several known limitations. See [API Limitations](docs/api/limitations.md) for details.

## Related Resources

- [NotebookLM API Documentation](https://cloud.google.com/gemini/enterprise/notebooklm-enterprise/docs/overview) - Official API documentation
- [NotebookLM API Reference](https://cloud.google.com/gemini/enterprise/notebooklm-enterprise/docs/api-notebooks) - API reference

## Contributing

Contributions are welcome! See [CONTRIBUTING.md](CONTRIBUTING.md) for development setup and guidelines.

> [!IMPORTANT]
> All pull requests must pass `cargo make all` (Rust) and `cargo make py-all` (Python) before being merged.

## License

MIT
