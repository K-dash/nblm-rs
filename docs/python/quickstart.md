# Python SDK Quickstart

Get started with the NotebookLM Python SDK in 5 minutes.

## Installation

```bash
pip install nblm
```

## Prerequisites

- Python 3.14 or later
- Google Cloud project with NotebookLM API enabled
- gcloud CLI installed and authenticated

## Basic Setup

### 1. Authenticate with gcloud

```bash
gcloud auth login
```

### 2. Get your project number

```bash
gcloud projects describe YOUR_PROJECT_ID --format="value(projectNumber)"
```

Example output: `123456789012`

### 3. Create your first notebook

```python
from nblm import NblmClient, GCloudTokenProvider

# Initialize client
client = NblmClient(
    token_provider=GCloudTokenProvider(),
    project_number="123456789012"
)

# Create a notebook
notebook = client.create_notebook("My First Notebook")
print(f"Created: {notebook.title}")
print(f"Notebook ID: {notebook.notebook_id}")
```

## Complete Example

Here's a complete workflow from creating a notebook to generating an audio overview:

```python
from nblm import (
    NblmClient,
    GCloudTokenProvider,
    WebSource,
    TextSource,
    AudioOverviewRequest
)

# 1. Initialize client
client = NblmClient(
    token_provider=GCloudTokenProvider(),
    project_number="123456789012"
)

# 2. Create a notebook
notebook = client.create_notebook("Python Tutorial Notebook")
notebook_id = notebook.notebook_id
print(f"Created notebook: {notebook_id}")

# 3. Add sources
response = client.add_sources(
    notebook_id=notebook_id,
    web_sources=[
        WebSource(url="https://docs.python.org/3/", name="Python Docs"),
        WebSource(url="https://realpython.com/")
    ],
    text_sources=[
        TextSource(content="My learning notes", name="Notes")
    ]
)
print(f"Added {len(response.sources)} sources")

# 4. Upload a file
upload_response = client.upload_source_file(
    notebook_id=notebook_id,
    path="tutorial.pdf"
)
print(f"Uploaded file: {upload_response.source_id}")

# 5. Create audio overview
audio = client.create_audio_overview(
    notebook_id=notebook_id,
    request=AudioOverviewRequest()
)
print(f"Audio overview created: {audio.status}")

# 6. List your notebooks
notebooks = client.list_recently_viewed(page_size=10)
print(f"Total notebooks: {len(notebooks.notebooks)}")
```

## Authentication Options

### Option 1: gcloud CLI (Recommended)

```python
from nblm import NblmClient, GCloudTokenProvider

client = NblmClient(
    token_provider=GCloudTokenProvider(),
    project_number="123456789012"
)
```

### Option 2: Environment Variable

```python
import os
from nblm import NblmClient, EnvTokenProvider

# Set access token
os.environ["NBLM_ACCESS_TOKEN"] = "your-access-token"

client = NblmClient(
    token_provider=EnvTokenProvider(),
    project_number="123456789012"
)
```

### Option 3: Custom gcloud Binary Path

```python
from nblm import NblmClient, GCloudTokenProvider

client = NblmClient(
    token_provider=GCloudTokenProvider(binary="/custom/path/gcloud"),
    project_number="123456789012"
)
```

## Common Operations

### Create and populate a notebook

```python
from nblm import NblmClient, GCloudTokenProvider, WebSource

client = NblmClient(
    token_provider=GCloudTokenProvider(),
    project_number="123456789012"
)

# Create
notebook = client.create_notebook("Research Notebook")

# Add multiple web sources
urls = [
    "https://example.com/article1",
    "https://example.com/article2",
    "https://example.com/article3"
]

client.add_sources(
    notebook_id=notebook.notebook_id,
    web_sources=[WebSource(url=url) for url in urls]
)
```

### Error handling

```python
from nblm import NblmClient, GCloudTokenProvider, NblmError

client = NblmClient(
    token_provider=GCloudTokenProvider(),
    project_number="123456789012"
)

try:
    notebook = client.create_notebook("Test Notebook")
    print(f"Success: {notebook.notebook_id}")
except NblmError as e:
    print(f"Failed: {e}")
```

## Configuration

### Using environment variables

```python
import os
from nblm import NblmClient, GCloudTokenProvider

# Set once
os.environ["NBLM_PROJECT_NUMBER"] = "123456789012"
os.environ["NBLM_LOCATION"] = "global"
os.environ["NBLM_ENDPOINT_LOCATION"] = "global"

# Create client (will use environment variables)
client = NblmClient(
    token_provider=GCloudTokenProvider(),
    project_number=os.environ["NBLM_PROJECT_NUMBER"]
)
```

### Custom locations

```python
from nblm import NblmClient, GCloudTokenProvider

# Use US location (for compliance requirements)
client = NblmClient(
    token_provider=GCloudTokenProvider(),
    project_number="123456789012",
    location="us",
    endpoint_location="us"
)
```

## Next Steps

- [API Reference](api-reference.md) - Complete API documentation
- [Notebooks API](notebooks.md) - Detailed notebook operations
- [Sources API](sources.md) - Detailed source operations
- [Error Handling](error-handling.md) - Exception handling patterns
