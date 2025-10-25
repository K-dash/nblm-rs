# nblm - Python Bindings for NotebookLM Enterprise API

Python bindings for the NotebookLM Enterprise API client written in Rust.

> [!WARNING]
> This is an unofficial tool and is not affiliated with or endorsed by Google. Use at your own risk.

> [!NOTE]
> **Under Active Development**: This library is currently in development. APIs may change, and additional features are being added. Currently supports notebook operations (create, list, delete) and source management (add/delete). Audio generation features are planned.

## Installation

```bash
pip install nblm
```

Or with uv:

```bash
uv add nblm
```

## Authentication

The library supports three authentication methods:

### 1. gcloud CLI

```python
from nblm import GcloudTokenProvider, NblmClient

provider = GcloudTokenProvider()  # Uses default 'gcloud' binary
# or specify custom path
# provider = GcloudTokenProvider(binary="/usr/local/bin/gcloud")

client = NblmClient(
    token_provider=provider,
    project_number="123456789012",
    location="global",
    endpoint_location="global",
)
```


### 2. Environment Variable Token

```python
import os
from nblm import EnvTokenProvider, NblmClient

os.environ["NBLM_ACCESS_TOKEN"] = "your-access-token"
provider = EnvTokenProvider()  # Uses NBLM_ACCESS_TOKEN by default
# or specify custom key
# provider = EnvTokenProvider(key="MY_CUSTOM_TOKEN")

client = NblmClient(
    token_provider=provider,
    project_number="123456789012",
)
```

## Notebook Operations

### Creating Notebooks

```python
from nblm import GcloudTokenProvider, NblmClient

provider = GcloudTokenProvider()
client = NblmClient(
    token_provider=provider,
    project_number="123456789012",
)

# Create a new notebook
notebook = client.create_notebook("My Research Notebook")
print(f"Created notebook: {notebook.title}")
print(f"Notebook ID: {notebook.notebook_id}")
```

### Listing Recent Notebooks

```python
# List recently viewed notebooks (default: 500)
response = client.list_recently_viewed()
for notebook_data in response.notebooks:
    print(f"Title: {notebook_data['title']}")
    print(f"ID: {notebook_data.get('notebookId', 'N/A')}")

# Limit results with page_size (1-500)
response = client.list_recently_viewed(page_size=10)
```

### Deleting Notebooks

```python
# Delete a single notebook
notebook_name = "projects/123456789012/locations/global/notebooks/abc123"
client.delete_notebooks([notebook_name])

# Delete multiple notebooks
notebook_names = [
    "projects/123456789012/locations/global/notebooks/abc123",
    "projects/123456789012/locations/global/notebooks/def456",
]
client.delete_notebooks(notebook_names)
```

> [!NOTE]
> Despite the underlying API being named "batchDelete", it only accepts
> one notebook at a time (as of 2025-10-19). The `delete_notebooks` method
> works around this limitation by calling the API sequentially for each notebook.

### Managing Sources

```python
from nblm import GcloudTokenProvider, NblmClient, WebSource, TextSource, VideoSource

provider = GcloudTokenProvider()
client = NblmClient(token_provider=provider, project_number="123456789012")

# Add different source types to a notebook
response = client.add_sources(
    notebook_id="abc123",
    web_sources=[
        WebSource(url="https://example.com", name="Example Website"),
        WebSource(url="https://python.org"),  # name is optional
    ],
    text_sources=[
        TextSource(content="Inline memo", name="Notes"),
    ],
    video_sources=[
        VideoSource(url="https://youtube.com/watch?v=123"),
    ],
)

for result in response.sources:
    print(result.name)  # Full source resource name

# Delete sources using full resource names
client.delete_sources(
    notebook_id="abc123",
    source_names=[
        "projects/123456789012/locations/global/notebooks/abc123/sources/source-1",
    ],
)
```

## Configuration

### Project Number

Get your Google Cloud project number:

```bash
gcloud projects describe YOUR_PROJECT_ID --format="value(projectNumber)"
```

### Locations

The NotebookLM API supports the following multi-region locations:

- `global` - **Recommended** by Google for best performance
- `us` - United States (for compliance requirements)
- `eu` - European Union (for compliance requirements)

```python
# Using global location (recommended)
client = NblmClient(
    token_provider=provider,
    project_number="123456789012",
    location="global",
    endpoint_location="global",
)
```

> [!NOTE]
> `location` and `endpoint_location` must be set to the same value.

## Error Handling

```python
from nblm import NblmClient, NblmError, GcloudTokenProvider

provider = GcloudTokenProvider()
client = NblmClient(
    token_provider=provider,
    project_number="123456789012",
)

try:
    notebook = client.create_notebook("My Notebook")
except NblmError as e:
    print(f"Error: {e}")
```

## API Limitations

### Delete Operations

The batch delete API only accepts one notebook at a time. The library handles this automatically by calling the API sequentially for each notebook.

## Type Hints

The library includes full type hints for better IDE support:

```python
from nblm import (
    NblmClient,
    Notebook,
    ListRecentlyViewedResponse,
    BatchDeleteNotebooksResponse,
)

client: NblmClient = ...
notebook: Notebook = client.create_notebook("Title")
response: ListRecentlyViewedResponse = client.list_recently_viewed()
```

## Development

See the main [CONTRIBUTING.md](../CONTRIBUTING.md) for development setup.

## License

MIT

## Related Resources

- [NotebookLM API Documentation](https://cloud.google.com/gemini/enterprise/notebooklm-enterprise/docs/overview)
- [Main Repository](https://github.com/K-dash/nblm-rs)
