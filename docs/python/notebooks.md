# Notebooks API

Detailed guide for notebook operations in the Python SDK.

## Creating Notebooks

### Basic Creation

```python
from nblm import NblmClient, GcloudTokenProvider

client = NblmClient(
    token_provider=GcloudTokenProvider(),
    project_number="123456789012"
)

notebook = client.create_notebook(title="My Research Notebook")
```

### Accessing Notebook Information

```python
notebook = client.create_notebook(title="Test Notebook")

print(f"Title: {notebook.title}")
print(f"Notebook ID: {notebook.notebook_id}")
print(f"Full name: {notebook.name}")
print(f"Created: {notebook.create_time}")
print(f"Updated: {notebook.update_time}")
```

### Error Handling

```python
from nblm import NblmError

try:
    notebook = client.create_notebook(title="My Notebook")
except NblmError as e:
    print(f"Failed to create notebook: {e}")
```

## Listing Notebooks

### List All Recent Notebooks

```python
response = client.list_recently_viewed()

for notebook_data in response.notebooks:
    print(f"Title: {notebook_data.get('title')}")
    print(f"ID: {notebook_data.get('notebookId')}")
    print(f"Updated: {notebook_data.get('updateTime')}")
    print("---")
```

### Limit Results

```python
# Get only 10 most recent notebooks
response = client.list_recently_viewed(page_size=10)
```

### Response Structure

```python
response = client.list_recently_viewed()

# response.notebooks is a list of dictionaries
for notebook_data in response.notebooks:
    # Each notebook_data is a dictionary with these keys:
    # - title: str
    # - notebookId: str
    # - name: str (full resource name)
    # - createTime: str
    # - updateTime: str
    pass
```

!!! note "API Limitations"
    - Notebooks are sorted by most recently updated first
    - The `page_size` parameter is accepted but **pagination is not implemented by the API** (as of 2025-10-25)
    - All notebooks are returned regardless of the `page_size` value

## Deleting Notebooks

### Delete Single Notebook

```python
notebook_name = "projects/123456789012/locations/global/notebooks/abc123"
client.delete_notebooks([notebook_name])
```

### Delete Multiple Notebooks

```python
notebook_names = [
    "projects/123456789012/locations/global/notebooks/abc123",
    "projects/123456789012/locations/global/notebooks/def456",
    "projects/123456789012/locations/global/notebooks/ghi789"
]

response = client.delete_notebooks(notebook_names)
```

### Get Notebook Name from Response

```python
# Create a notebook
notebook = client.create_notebook(title="Temporary Notebook")

# Use the full name for deletion
client.delete_notebooks([notebook.name])
```

!!! warning "Deletion Limitations"
    - **Important**: Despite the API being named "batchDelete", it only accepts one notebook at a time (as of 2025-10-25)
    - The SDK handles this limitation by calling the API sequentially for each notebook
    - Deletion is permanent and cannot be undone
    - All sources and content within the notebook are also deleted

## Common Patterns

### Create, Use, and Clean Up

```python
from nblm import NblmClient, GcloudTokenProvider, WebSource

client = NblmClient(
    token_provider=GcloudTokenProvider(),
    project_number="123456789012"
)

# Create notebook
notebook = client.create_notebook(title="Temporary Analysis")
notebook_id = notebook.notebook_id

try:
    # Use the notebook
    client.add_sources(
        notebook_id=notebook_id,
        web_sources=[WebSource(url="https://example.com")]
    )

    # Do analysis...

finally:
    # Clean up
    client.delete_notebooks([notebook.name])
```

### Filter and Process Notebooks

```python
response = client.list_recently_viewed()

# Filter notebooks by title
research_notebooks = [
    nb for nb in response.notebooks
    if "research" in nb.get("title", "").lower()
]

print(f"Found {len(research_notebooks)} research notebooks")
```

### Batch Create

```python
titles = ["Project A", "Project B", "Project C"]
created_notebooks = []

for title in titles:
    notebook = client.create_notebook(title=title)
    created_notebooks.append(notebook)
    print(f"Created: {notebook.notebook_id}")
```

### Delete Old Notebooks

```python
from datetime import datetime, timedelta

response = client.list_recently_viewed()

# Calculate cutoff date (30 days ago)
cutoff = datetime.now() - timedelta(days=30)

old_notebooks = []
for nb_data in response.notebooks:
    update_time = datetime.fromisoformat(nb_data["updateTime"].replace("Z", "+00:00"))
    if update_time < cutoff:
        old_notebooks.append(nb_data["name"])

if old_notebooks:
    print(f"Deleting {len(old_notebooks)} old notebooks...")
    client.delete_notebooks(old_notebooks)
```

## Error Handling

### Common Errors

**Authentication failure:**

```python
from nblm import NblmError

try:
    notebook = client.create_notebook(title="Test")
except NblmError as e:
    if "authentication" in str(e).lower():
        print("Authentication failed. Check your credentials.")
    else:
        print(f"Error: {e}")
```

**Notebook not found:**

```python
try:
    client.delete_notebooks(["projects/.../notebooks/nonexistent"])
except NblmError as e:
    print(f"Notebook not found: {e}")
```

**Permission denied:**

```python
try:
    client.delete_notebooks(["projects/.../notebooks/abc123"])
except NblmError as e:
    if "permission" in str(e).lower():
        print("You don't have permission to delete this notebook")
```

## Best Practices

### Save Notebook IDs

```python
# Save for later use
notebook = client.create_notebook(title="Important Notebook")

# Store the ID
notebook_id = notebook.notebook_id
with open("notebook_id.txt", "w") as f:
    f.write(notebook_id)

# Use later
with open("notebook_id.txt", "r") as f:
    saved_id = f.read().strip()

client.add_sources(
    notebook_id=saved_id,
    web_sources=[WebSource(url="https://example.com")]
)
```

### Validate Before Operations

```python
def create_notebook_safely(client: NblmClient, title: str) -> Optional[Notebook]:
    if not title or not title.strip():
        print("Error: Title cannot be empty")
        return None

    try:
        return client.create_notebook(title=title)
    except NblmError as e:
        print(f"Failed to create notebook: {e}")
        return None
```

### Use Context Managers

```python
from contextlib import contextmanager
from typing import Generator

@contextmanager
def temporary_notebook(
    client: NblmClient,
    title: str
) -> Generator[Notebook, None, None]:
    """Create a notebook and automatically delete it when done."""
    notebook = client.create_notebook(title=title)
    try:
        yield notebook
    finally:
        client.delete_notebooks([notebook.name])

# Usage
with temporary_notebook(client, "Temp Analysis") as notebook:
    client.add_sources(
        notebook_id=notebook.notebook_id,
        web_sources=[WebSource(url="https://example.com")]
    )
    # Notebook is automatically deleted after this block
```

## Next Steps

- [Sources API](sources.md) - Add and manage sources
- [Audio API](audio.md) - Create audio overviews
- [API Reference](api-reference.md) - Complete API reference
- [Error Handling](error-handling.md) - Exception handling patterns
