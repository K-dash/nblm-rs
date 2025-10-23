# nblm - Python Bindings for NotebookLM Enterprise API

Python bindings for the NotebookLM Enterprise API client written in Rust.

> [!WARNING]
> This is an unofficial tool and is not affiliated with or endorsed by Google. Use at your own risk.

> [!NOTE]
> **Under Active Development**: This library is currently in development. APIs may change, and additional features are being added. Currently supports notebook operations (create, list, delete). Source management and audio generation features are planned.

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

### 2. Service Account

```python
from nblm import ServiceAccountTokenProvider, NblmClient

# From file
provider = ServiceAccountTokenProvider.from_file("/path/to/key.json")

# Or from JSON string
provider = ServiceAccountTokenProvider.from_json(json_string)

client = NblmClient(
    token_provider=provider,
    project_number="123456789012",
)
```

> [!IMPORTANT]
> Service account requires `roles/editor` permission.

### 3. Environment Variable Token

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
# List recently viewed notebooks
response = client.list_recently_viewed()
for notebook_data in response.notebooks:
    print(f"Title: {notebook_data['title']}")
    print(f"ID: {notebook_data.get('notebookId', 'N/A')}")

# With pagination parameters (note: API doesn't implement pagination yet)
response = client.list_recently_viewed(page_size=10)
```

> [!NOTE]
> As of 2025-10-19, the NotebookLM API does not implement pagination.
> The `page_size` parameter is accepted but ignored, and `next_page_token`
> is never returned in responses.

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

### Pagination

The `list_recently_viewed` method accepts `page_size` and `page_token` parameters, but the NotebookLM API does not currently implement pagination. These parameters are included for future compatibility.

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

- [NotebookLM API Documentation](https://cloud.google.com/gemini/enterprise/notebooklm-enterprise/docs)
- [Main Repository](https://github.com/K-dash/nblm-rs)
