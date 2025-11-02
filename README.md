<div align="center">

  <img width="256" alt="logo" src="https://github.com/user-attachments/assets/37f0f882-65ca-436e-8053-3db8c18cac59" />

  # _nblm-rs_

  **Unofficial NotebookLM Enterprise API client**

  🦀 **Rust CLI**: Command-line tool for shell scripting and automation <br/>
  🐍 **Python SDK**: Python bindings for integration in Python applications

  [![Crates.io](https://img.shields.io/crates/v/nblm-cli.svg)](https://crates.io/crates/nblm-cli)
  [![Crates.io](https://img.shields.io/crates/d/nblm-cli.svg?color=orange&label=downloads)](https://crates.io/crates/nblm-cli)
  <br/>
  [![PyPI](https://img.shields.io/pypi/v/nblm.svg?color=blue)](https://pypi.org/project/nblm/)
  [![PyPI Downloads](https://static.pepy.tech/personalized-badge/nblm?period=total&units=INTERNATIONAL_SYSTEM&left_color=GRAY&right_color=BLUE&left_text=downloads)](https://pepy.tech/projects/nblm)
  [![Python versions](https://img.shields.io/pypi/pyversions/nblm.svg)](https://pypi.org/project/nblm/)
  [![uv](https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/astral-sh/uv/main/assets/badge/v0.json)](https://github.com/astral-sh/uv)
  [![Ruff](https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/astral-sh/ruff/main/assets/badge/v2.json)](https://github.com/astral-sh/ruff)
  [![mypy](https://img.shields.io/badge/mypy-checked-blue.svg)](https://mypy-lang.org/)
  <br/>
  [![DeepWiki](https://img.shields.io/badge/DeepWiki-K--dash%2Fnblm--rs-blue.svg?logo=data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAACwAAAAyCAYAAAAnWDnqAAAAAXNSR0IArs4c6QAAA05JREFUaEPtmUtyEzEQhtWTQyQLHNak2AB7ZnyXZMEjXMGeK/AIi+QuHrMnbChYY7MIh8g01fJoopFb0uhhEqqcbWTp06/uv1saEDv4O3n3dV60RfP947Mm9/SQc0ICFQgzfc4CYZoTPAswgSJCCUJUnAAoRHOAUOcATwbmVLWdGoH//PB8mnKqScAhsD0kYP3j/Yt5LPQe2KvcXmGvRHcDnpxfL2zOYJ1mFwrryWTz0advv1Ut4CJgf5uhDuDj5eUcAUoahrdY/56ebRWeraTjMt/00Sh3UDtjgHtQNHwcRGOC98BJEAEymycmYcWwOprTgcB6VZ5JK5TAJ+fXGLBm3FDAmn6oPPjR4rKCAoJCal2eAiQp2x0vxTPB3ALO2CRkwmDy5WohzBDwSEFKRwPbknEggCPB/imwrycgxX2NzoMCHhPkDwqYMr9tRcP5qNrMZHkVnOjRMWwLCcr8ohBVb1OMjxLwGCvjTikrsBOiA6fNyCrm8V1rP93iVPpwaE+gO0SsWmPiXB+jikdf6SizrT5qKasx5j8ABbHpFTx+vFXp9EnYQmLx02h1QTTrl6eDqxLnGjporxl3NL3agEvXdT0WmEost648sQOYAeJS9Q7bfUVoMGnjo4AZdUMQku50McDcMWcBPvr0SzbTAFDfvJqwLzgxwATnCgnp4wDl6Aa+Ax283gghmj+vj7feE2KBBRMW3FzOpLOADl0Isb5587h/U4gGvkt5v60Z1VLG8BhYjbzRwyQZemwAd6cCR5/XFWLYZRIMpX39AR0tjaGGiGzLVyhse5C9RKC6ai42ppWPKiBagOvaYk8lO7DajerabOZP46Lby5wKjw1HCRx7p9sVMOWGzb/vA1hwiWc6jm3MvQDTogQkiqIhJV0nBQBTU+3okKCFDy9WwferkHjtxib7t3xIUQtHxnIwtx4mpg26/HfwVNVDb4oI9RHmx5WGelRVlrtiw43zboCLaxv46AZeB3IlTkwouebTr1y2NjSpHz68WNFjHvupy3q8TFn3Hos2IAk4Ju5dCo8B3wP7VPr/FGaKiG+T+v+TQqIrOqMTL1VdWV1DdmcbO8KXBz6esmYWYKPwDL5b5FA1a0hwapHiom0r/cKaoqr+27/XcrS5UwSMbQAAAABJRU5ErkJggg==)](https://deepwiki.com/K-dash/nblm-rs)
  [![codecov](https://codecov.io/gh/K-dash/nblm-rs/graph/badge.svg?token=OhxeTdnxTw)](https://codecov.io/gh/K-dash/nblm-rs)


  
</div>

> [!IMPORTANT]
> This project targets the **NotebookLM Enterprise API** only. Google hasn’t published an API for the consumer edition or general Google Workspace tenants as of 2025-10-25.

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

- **Repeated operations**

  - **Problem**: Writing boilerplate loops for fetch/add/delete sequences
  - **Solution**: Higher-level client helpers and CLI flags that wrap single API calls (with retries built in) so scripts stay concise

- **Output parsing**
  - **Problem**: Manual JSON parsing and extracting specific fields from responses
  - **Solution**: Structured response objects in the Python SDK and `--json` output in the CLI for easy integration with tools like `jq`

### Project Goals

This project provides production-ready tools that make the NotebookLM API accessible and reliable:

- **Rust CLI**: Fast, cross-platform binary for shell scripting and automation
- **Python SDK**: Idiomatic Python bindings for application integration
- **Type safety**: Compile-time checks prevent common API usage errors
- **Developer experience**: Intuitive commands and clear documentation

## Installation

### CLI

```bash
# macOS
brew tap k-dash/nblm https://github.com/K-dash/homebrew-nblm
brew install k-dash/nblm/nblm

# Linux (prebuilt binaries)
# Download from Releases page: https://github.com/K-dash/nblm-rs/releases

# From source
cargo install nblm-cli
```

### Python SDK

```bash
pip install nblm
# or
uv add nblm
```

> Prerequisite: a Google Cloud project with the NotebookLM Enterprise API enabled and either `gcloud auth login` or an OAuth token ready for `NBLM_ACCESS_TOKEN`.

For detailed installation instructions and troubleshooting, see the [Installation Guide](https://k-dash.github.io/nblm-rs/getting-started/installation/).

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

## Features

> [!NOTE]
> The NotebookLM API is currently in **alpha**. Some features may not work as documented due to API limitations. See the [complete feature list](https://k-dash.github.io/nblm-rs/#features) in the documentation.

nblm-rs supports the following NotebookLM API operations:

- **Notebooks**: Create, list, and delete notebooks
- **Sources**: Add web URLs, text, videos (YouTube), Google Drive files, and upload files
- **Audio Overview**: Create and delete audio overviews
- **Sharing**: Share notebooks with users (CLI only, untested)

For detailed feature status and limitations, see the [Features documentation](https://k-dash.github.io/nblm-rs/#features).

## Documentation

**Complete guides and API references:**

📖 **[Full Documentation](https://k-dash.github.io/nblm-rs/)** - Complete guides, API references, and examples

- [Getting Started](https://k-dash.github.io/nblm-rs/getting-started/installation/) - Installation, authentication, configuration
- [CLI Reference](https://k-dash.github.io/nblm-rs/cli/) - All commands, options, and examples
- [Python SDK Reference](https://k-dash.github.io/nblm-rs/python/) - API reference and usage patterns

## Known API Issues

> [!NOTE]
> The NotebookLM API is currently in **alpha** and has several known limitations. See [API Limitations](https://k-dash.github.io/nblm-rs/api/limitations/) for details.

## Related Resources

- [NotebookLM API Documentation](https://cloud.google.com/gemini/enterprise/notebooklm-enterprise/docs/overview) - Official API documentation
- [NotebookLM API Reference](https://cloud.google.com/gemini/enterprise/notebooklm-enterprise/docs/api-notebooks) - API reference

## Contributing

Contributions are welcome! See [CONTRIBUTING.md](CONTRIBUTING.md) for development setup and guidelines.

## License

MIT
