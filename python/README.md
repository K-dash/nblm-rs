# nblm - Python SDK for NotebookLM Enterprise API

Python bindings for the NotebookLM Enterprise API, powered by Rust via PyO3.

> **Warning**: This project is not affiliated with, sponsored, or endorsed by Google. nblm-rs is an independent, unofficial tool. It is provided "as is" without any warranty.

## Installation

```bash
pip install nblm
```

Or with uv:

```bash
uv add nblm
```

**Requirements**: Python 3.14 or later

## Quick Start

```python
from nblm import NblmClient, GCloudTokenProvider

# Initialize client
client = NblmClient(
    token_provider=GCloudTokenProvider(),
    project_number="123456789012"
)

# Create a notebook
notebook = client.create_notebook("My Notebook")
print(f"Created: {notebook.title}")

# Add sources
from nblm import WebSource

client.add_sources(
    notebook_id=notebook.notebook_id,
    web_sources=[WebSource(url="https://example.com", name="Example")]
)

# Create audio overview
audio = client.create_audio_overview(notebook.notebook_id)
print(f"Audio status: {audio.status}")
```

## Features

- **Notebooks**: Create, list, and delete notebooks
- **Sources**: Add web, text, video sources; upload files; manage sources
- **Audio Overviews**: Create podcast-style discussions from notebook content
- **Type Safety**: Full type hints for IDE autocomplete and static analysis
- **Fast**: Powered by Rust for high performance

## Authentication

### gcloud CLI (Recommended)

```python
from nblm import NblmClient, GCloudTokenProvider

client = NblmClient(
    token_provider=GCloudTokenProvider(),
    project_number="123456789012"
)
```

### Environment Variable

```python
import os
from nblm import NblmClient, EnvTokenProvider

os.environ["NBLM_ACCESS_TOKEN"] = "your-access-token"

client = NblmClient(
    token_provider=EnvTokenProvider(),
    project_number="123456789012"
)
```

### OAuth2 (Read-only)

> **Experimental:** OAuth2 support currently requires setting `NBLM_PROFILE_EXPERIMENT=1`. Users must create an OAuth 2.0 **Desktop application** client ID in Google Cloud Console, including the client secret, before running this flow.

First, run the CLI once to complete the browser flow and store a refresh token:

```bash
export NBLM_OAUTH_CLIENT_ID="123-abc.apps.googleusercontent.com"
export NBLM_OAUTH_CLIENT_SECRET="your-desktop-client-secret"
export NBLM_PROFILE_EXPERIMENT=1
nblm --auth user-oauth --project-number 123456 notebooks list
```

Then reuse that token from Python without re-authenticating:

```python
from nblm import NblmClient

client = NblmClient.with_user_oauth(
    project_number=123456,
    location="us-central1",
)
```

You can also instantiate the provider directly:

```python
from nblm import UserOAuthProvider

provider = UserOAuthProvider.from_file(project_number=123456)
```

## Documentation

**Complete Python SDK documentation:**

- [Getting Started Guide](https://github.com/K-dash/nblm-rs/blob/main/docs/getting-started/installation.md)
- [Quickstart Tutorial](https://github.com/K-dash/nblm-rs/blob/main/docs/python/quickstart.md)
- [API Reference](https://github.com/K-dash/nblm-rs/blob/main/docs/python/api-reference.md)
- [Notebooks API](https://github.com/K-dash/nblm-rs/blob/main/docs/python/notebooks.md)
- [Sources API](https://github.com/K-dash/nblm-rs/blob/main/docs/python/sources.md)
- [Audio API](https://github.com/K-dash/nblm-rs/blob/main/docs/python/audio.md)
- [Error Handling](https://github.com/K-dash/nblm-rs/blob/main/docs/python/error-handling.md)

## Examples

### Create Notebook and Add Sources

```python
from nblm import NblmClient, GCloudTokenProvider, WebSource, TextSource

client = NblmClient(
    token_provider=GCloudTokenProvider(),
    project_number="123456789012"
)

# Create notebook
notebook = client.create_notebook("Research: Python Best Practices")

# Add sources
client.add_sources(
    notebook_id=notebook.notebook_id,
    web_sources=[
        WebSource(url="https://peps.python.org/pep-0008/", name="PEP 8"),
        WebSource(url="https://docs.python-guide.org/")
    ],
    text_sources=[
        TextSource(content="Focus on code quality", name="Notes")
    ]
)
```

### Upload Files

```python
# Upload a PDF
response = client.upload_source_file(
    notebook_id=notebook.notebook_id,
    path="/path/to/document.pdf",
    display_name="Research Paper"
)
print(f"Uploaded: {response.source_id}")
```

### Error Handling

```python
from nblm import NblmError

try:
    notebook = client.create_notebook("My Notebook")
except NblmError as e:
    print(f"Error: {e}")
```

## Type Hints

The library includes full type hints:

```python
from nblm import (
    NblmClient,
    Notebook,
    NotebookSource,
    AudioOverviewResponse,
    WebSource,
    TextSource,
    VideoSource,
)

# All operations are fully typed
client: NblmClient
notebook: Notebook = client.create_notebook("Title")
audio: AudioOverviewResponse = client.create_audio_overview("abc123")
```

## Supported Operations

| Category           | Operations                                        | Status        |
| ------------------ | ------------------------------------------------- | ------------- |
| **Notebooks**      | Create, list, delete                              | Available     |
| **Sources**        | Add (web, text, video), upload files, get, delete | Available     |
| **Audio Overview** | Create, delete                                    | Available     |
| **Sharing**        | Share with users                                  | Not available |

## Links

- [GitHub Repository](https://github.com/K-dash/nblm-rs)
- [Full Documentation](https://github.com/K-dash/nblm-rs/tree/main/docs)
- [CLI Tool](https://crates.io/crates/nblm-cli)
- [Issue Tracker](https://github.com/K-dash/nblm-rs/issues)

## Contributing

See [CONTRIBUTING.md](https://github.com/K-dash/nblm-rs/blob/main/CONTRIBUTING.md) for development setup and guidelines.

## License

MIT
