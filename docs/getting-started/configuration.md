# Configuration

Configure the NotebookLM Enterprise API client with your Google Cloud project.

## Project Number

The NotebookLM API requires a Google Cloud project number (not project ID).

### Get Your Project Number

```bash
gcloud projects describe YOUR_PROJECT_ID --format="value(projectNumber)"
```

Example output: `123456789012`

### Difference Between Project ID and Project Number

- **Project ID**: Human-readable identifier (e.g., `my-project-2024`)
- **Project Number**: Unique numerical identifier (e.g., `123456789012`)

The API requires the **project number**.

## Locations

The NotebookLM API supports the following multi-region locations:

| Location | Description                   | Recommendation              |
| -------- | ----------------------------- | --------------------------- |
| `global` | Best performance and features | **Recommended**             |
| `us`     | United States only            | For compliance requirements |
| `eu`     | European Union only           | For compliance requirements |

!!! important "Location consistency"
    `location` and `endpoint_location` must always be set to the same value. The API treats them as a pair, and mismatched values result in `INVALID_ARGUMENT` errors.

## Environment Variables

Set environment variables to avoid repeating options in every command.

### Debug Logging

Set `NBLM_DEBUG_HTTP=1` to emit full HTTP response bodies for every API call. This works for both the CLI and Python SDK and is handy when you need to inspect raw JSON during contract changes.

```bash
# Enable verbose HTTP logging
export NBLM_DEBUG_HTTP=1
```

!!! warning "Sensitive data"
    The full response payload can contain sensitive information. Only enable debug logging in trusted environments and disable it once you finish troubleshooting.

### CLI

```bash
# Required
export NBLM_PROJECT_NUMBER="123456789012"

# Recommended
export NBLM_LOCATION="global"
export NBLM_ENDPOINT_LOCATION="global"

# Optional (for specific authentication methods)
export NBLM_ACCESS_TOKEN="your-access-token"
```

### Python SDK

```python
import os

# Set before creating client
os.environ["NBLM_PROJECT_NUMBER"] = "123456789012"
os.environ["NBLM_LOCATION"] = "global"
os.environ["NBLM_ENDPOINT_LOCATION"] = "global"
```

Or pass directly to the client:

```python
from nblm import NblmClient, GcloudTokenProvider

client = NblmClient(
    token_provider=GcloudTokenProvider(),
    project_number="123456789012",
    location="global",
    endpoint_location="global"
)
```

## Configuration File

### CLI

The CLI does not currently support a configuration file. Use environment variables instead.

### Python SDK

You can create a configuration wrapper:

```python
# config.py
import os
from nblm import NblmClient, GcloudTokenProvider

def create_client():
    return NblmClient(
        token_provider=GcloudTokenProvider(),
        project_number=os.getenv("NBLM_PROJECT_NUMBER", "123456789012"),
        location=os.getenv("NBLM_LOCATION", "global"),
        endpoint_location=os.getenv("NBLM_ENDPOINT_LOCATION", "global"),
    )
```

Then use it in your code:

```python
from config import create_client

client = create_client()
notebook = client.create_notebook(title="My Notebook")
```

## Verification

### CLI

```bash
# Should work without additional flags if environment variables are set
nblm notebooks recent
```

### Python SDK

```python
from nblm import NblmClient, GcloudTokenProvider

client = NblmClient(
    token_provider=GcloudTokenProvider(),
    project_number="123456789012"
)

# Should successfully list notebooks
response = client.list_recently_viewed()
print(f"Found {len(response.notebooks)} notebooks")
```

!!! tip "Validate with doctor"
    Once your configuration variables are in place, run [`nblm doctor`](../cli/doctor.md) to verify authentication, project bindings, and location settings before moving to production.

## Next Steps

- [CLI Overview](../cli/README.md) - Start using the CLI
- [Python Quickstart](../python/quickstart.md) - Start using Python SDK
- [Troubleshooting](../guides/troubleshooting.md) - Common configuration issues
