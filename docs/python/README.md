# Python SDK Overview

Python bindings for the NotebookLM Enterprise API, powered by Rust via PyO3.

> **Warning**: This is an unofficial tool and is not affiliated with or endorsed by Google. Use at your own risk.

## Features

- **Type-safe API**: Full type hints for IDE autocomplete and static analysis
- **Fast**: Powered by Rust for high performance
- **Easy to use**: Pythonic API with sensible defaults
- **Comprehensive**: Supports notebooks, sources, and audio overviews

## Supported Operations

| Category           | Operations                                        | Status        |
| ------------------ | ------------------------------------------------- | ------------- |
| **Notebooks**      | Create, list, delete                              | Available     |
| **Sources**        | Add (web, text, video), upload files, get, delete | Available     |
| **Audio Overview** | Create, delete                                    | Available     |
| **Sharing**        | Share with users                                  | Not available |

## Installation

```bash
pip install nblm
```

Or with uv:

```bash
uv add nblm
```

**Requirements**: Python 3.14 or later

## Quick Example

```python
from nblm import NblmClient, GcloudTokenProvider, WebSource

# Initialize client
client = NblmClient(
    token_provider=GcloudTokenProvider(),
    project_number="123456789012"
)

# Create notebook
notebook = client.create_notebook(title="My Notebook")

# Add sources
client.add_sources(
    notebook_id=notebook.notebook_id,
    web_sources=[WebSource(url="https://example.com", name="Example")]
)

# Create audio overview
audio = client.create_audio_overview(notebook.notebook_id)
print(f"Audio status: {audio.status}")
```

## Documentation

### Getting Started

- [Quickstart](quickstart.md) - Get started in 5 minutes
- [Authentication](../getting-started/authentication.md) - Set up authentication
- [Configuration](../getting-started/configuration.md) - Configure project and location

### API Reference

- [API Reference](api-reference.md) - Complete API documentation
- [Notebooks API](notebooks.md) - Notebook operations
- [Sources API](sources.md) - Source operations
- [Audio API](audio.md) - Audio overview operations
- [Error Handling](error-handling.md) - Exception handling

## Authentication Methods

```python
from nblm import (
    GcloudTokenProvider,  # Use gcloud CLI
    EnvTokenProvider,     # Use environment variable
    NblmClient
)

# Method 1: gcloud CLI (recommended)
provider = GcloudTokenProvider()

# Method 2: Environment variable
import os
os.environ["NBLM_ACCESS_TOKEN"] = "your-token"
provider = EnvTokenProvider()

# Create client
client = NblmClient(
    token_provider=provider,
    project_number="123456789012"
)
```

## Debugging HTTP Responses

Set `NBLM_DEBUG_HTTP=1` before importing `nblm` to print the raw JSON bodies returned by the API. The payload can include notebook contents, so only enable this in trusted environments.

```bash
export NBLM_DEBUG_HTTP=1
python monitor_api.py --debug-http
```

## Type Support

The SDK includes full type hints:

```python
from nblm import (
    NblmClient,
    Notebook,
    NotebookSource,
    AudioOverviewResponse,
    ListRecentlyViewedResponse,
    BatchCreateSourcesResponse,
    WebSource,
    TextSource,
    VideoSource,
)

# All operations are fully typed
client: NblmClient
notebook: Notebook = client.create_notebook(title="Title")
sources: BatchCreateSourcesResponse = client.add_sources(...)
audio: AudioOverviewResponse = client.create_audio_overview(...)
```

## Error Handling

```python
from nblm import NblmClient, NblmError

try:
    notebook = client.create_notebook(title="My Notebook")
except NblmError as e:
    print(f"Error: {e}")
```

See [Error Handling](error-handling.md) for details.

## Performance

The Python SDK is powered by Rust, providing:

- **Fast execution**: Native code performance
- **Memory efficiency**: Rust's memory management
- **Thread safety**: Safe concurrent operations

## Limitations

- **Sharing operations**: Not currently supported
- **Google Drive sources**: Not implemented (API returns HTTP 500)
- **Audio configuration**: API only accepts empty request (as of 2025-10-25)

## Next Steps

- [Quickstart](quickstart.md) - Start building with the Python SDK
- [API Reference](api-reference.md) - Explore all available methods
- [Examples](notebooks.md) - See practical examples
