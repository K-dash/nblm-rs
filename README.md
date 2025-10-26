<div align="center">

  # _nblm-rs_

  **Unofficial NotebookLM Enterprise API client**

  🦀 **Rust CLI**: Command-line tool for shell scripting and automation <br/>
  🐍 **Python SDK**: Python bindings for integration in Python applications

  [![Crates.io](https://img.shields.io/crates/v/nblm-cli.svg)](https://crates.io/crates/nblm-cli)
  [![PyPI](https://img.shields.io/pypi/v/nblm.svg)](https://pypi.org/project/nblm/)
  [![codecov](https://codecov.io/gh/K-dash/nblm-rs/graph/badge.svg?token=OhxeTdnxTw)](https://codecov.io/gh/K-dash/nblm-rs)

  <img width="512" height="512" alt="logo" src="https://github.com/user-attachments/assets/5ecea7aa-a771-4455-9851-c35f20430faa" />

</div>

> [!IMPORTANT]
> This project targets the **NotebookLM Enterprise API** only. Google hasn’t published an API for the consumer edition or general Google Workspace tenants as of 2025-10-25.

## Known API Issues

> [!NOTE]
> The NotebookLM API is currently in **alpha** and has several known limitations. See [API Limitations](docs/api/limitations.md) for details.

## Motivation

In September 2025, Google released the [NotebookLM Enterprise API](https://cloud.google.com/gemini/enterprise/notebooklm-enterprise/docs/overview), enabling programmatic access to NotebookLM features for the first time.

While you can interact with the API using simple `curl` commands, this approach has several limitations that this project addresses:

### Challenges with Direct API Calls

- **Authentication complexity**
  - **Problem**: Managing OAuth tokens, handling token refresh, and ensuring secure credential storage
  - **Solution**: Seamless `gcloud` CLI integration with automatic token caching and refresh

- **Manual request construction**
  - **Problem**: Writing JSON payloads by hand, managing resource names, and handling API versioning
  - **Solution**: Type-safe CLI flags and Python SDK with intelligent defaults and validation

- **Error handling**
  - **Problem**: Cryptic HTTP error codes without context or recovery suggestions
  - **Solution**: Clear, actionable error messages with automatic retries for transient failures

- **Batch operations**
  - **Problem**: Writing loops to process multiple items, managing API call sequences
  - **Solution**: Built-in batch commands with simplified syntax for multiple operations

- **Output parsing**
  - **Problem**: Manual JSON parsing and extracting specific fields from responses
  - **Solution**: Structured output formats and JSON mode for easy integration with `jq` and other tools

### Project Goals

This project provides production-ready tools that make the NotebookLM API accessible and reliable:

- **Rust CLI**: Fast, cross-platform binary for shell scripting and automation
- **Python SDK**: Idiomatic Python bindings for application integration
- **Type safety**: Compile-time checks prevent common API usage errors
- **Developer experience**: Intuitive commands and clear documentation


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
| Add Google Drive    | ◯   | ◯      | Working     | Requires Drive-enabled auth |
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
nblm notebooks create --title "My Notebook"

# 4. Add a source
nblm sources add \
  --notebook-id YOUR_NOTEBOOK_ID \
  --web-url "https://example.com" \
  --web-name "Example"
```

### Python

```python
from nblm import NblmClient, GcloudTokenProvider, WebSource

# Initialize client
client = NblmClient(
    token_provider=GcloudTokenProvider(),
    project_number="123456789012"
)

# Create a notebook
notebook = client.create_notebook(title="My Notebook")

# Add sources
response = client.add_sources(
    notebook_id=notebook.notebook_id,
    web_sources=[WebSource(url="https://example.com", name="Example")]
)
```

## Platform Support

| Platform | CLI | Python SDK |
|----------|-----|------------|
| Linux    | [![Linux CLI supported](https://img.shields.io/badge/support-%E2%9C%85-green)](https://shields.io) | [![Linux Python SDK supported](https://img.shields.io/badge/support-%E2%9C%85-green)](https://shields.io) |
| macOS    | [![macOS CLI supported](https://img.shields.io/badge/support-%E2%9C%85-green)](https://shields.io) | [![macOS Python SDK supported](https://img.shields.io/badge/support-%E2%9C%85-green)](https://shields.io) |
| Windows  | [![Windows CLI not supported](https://img.shields.io/badge/support-%E2%9D%8C-red)](https://shields.io) | [![Windows Python SDK not supported](https://img.shields.io/badge/support-%E2%9D%8C-red)](https://shields.io) |


## Documentation

**Complete guides and API references:**

- [Getting Started](docs/getting-started/installation.md) - Installation, authentication, configuration
- [CLI Reference](docs/cli/README.md) - All commands, options, and examples
- [Python SDK Reference](docs/python/README.md) - API reference and usage patterns
- [API Limitations](docs/api/limitations.md) - Known issues and workarounds


## Related Resources

- [NotebookLM API Documentation](https://cloud.google.com/gemini/enterprise/notebooklm-enterprise/docs/overview) - Official API documentation
- [NotebookLM API Reference](https://cloud.google.com/gemini/enterprise/notebooklm-enterprise/docs/api-notebooks) - API reference

## Contributing

Contributions are welcome! See [CONTRIBUTING.md](CONTRIBUTING.md) for development setup and guidelines.

> [!IMPORTANT]
> All pull requests must pass `cargo make all` (Rust) and `cargo make py-all` (Python) before being merged.

## License

MIT
