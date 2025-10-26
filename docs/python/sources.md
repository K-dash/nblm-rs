# Sources API

Detailed guide for managing sources in notebooks with the Python SDK.

## Adding Sources

### Add Web Sources

```python
from nblm import NblmClient, GcloudTokenProvider, WebSource

client = NblmClient(
    token_provider=GcloudTokenProvider(),
    project_number="123456789012"
)

# Single web source
response = client.add_sources(
    notebook_id="abc123",
    web_sources=[
        WebSource(url="https://example.com", name="Example Website")
    ]
)

# Multiple web sources
response = client.add_sources(
    notebook_id="abc123",
    web_sources=[
        WebSource(url="https://docs.python.org", name="Python Docs"),
        WebSource(url="https://realpython.com"),  # name is optional
        WebSource(url="https://peps.python.org", name="Python PEPs")
    ]
)
```

### Add Text Sources

```python
from nblm import TextSource

response = client.add_sources(
    notebook_id="abc123",
    text_sources=[
        TextSource(content="Meeting notes from 2025-10-25", name="Meeting Notes"),
        TextSource(content="TODO: Review API changes", name="TODO")
    ]
)
```

### Add Video Sources

```python
from nblm import VideoSource

response = client.add_sources(
    notebook_id="abc123",
    video_sources=[
        VideoSource(url="https://www.youtube.com/watch?v=VIDEO_ID_1"),
        VideoSource(url="https://www.youtube.com/watch?v=VIDEO_ID_2")
    ]
)
```

### Mix Multiple Source Types

```python
from nblm import WebSource, TextSource, VideoSource

response = client.add_sources(
    notebook_id="abc123",
    web_sources=[
        WebSource(url="https://example.com", name="Article")
    ],
    text_sources=[
        TextSource(content="Summary of key points", name="Summary")
    ],
    video_sources=[
        VideoSource(url="https://www.youtube.com/watch?v=VIDEO_ID")
    ]
)

print(f"Added {len(response.sources)} sources")
for source in response.sources:
    print(f"  - {source['name']}")
```

### Validation

The SDK validates inputs before making API calls:

```python
from nblm import NblmError

# Empty text content raises error
try:
    client.add_sources(
        notebook_id="abc123",
        text_sources=[TextSource(content="", name="Empty")]
    )
except NblmError as e:
    print(f"Validation error: {e}")  # "text content cannot be empty"

# No sources provided raises error
try:
    client.add_sources(notebook_id="abc123")
except NblmError as e:
    print(f"Validation error: {e}")  # "at least one source must be provided"
```

## Uploading Files

### Basic Upload

```python
response = client.upload_source_file(
    notebook_id="abc123",
    path="/path/to/document.pdf"
)

print(f"Uploaded: {response.source_id}")
```

### With Custom Content Type

```python
response = client.upload_source_file(
    notebook_id="abc123",
    path="/path/to/file.txt",
    content_type="text/plain"
)
```

### With Display Name

```python
response = client.upload_source_file(
    notebook_id="abc123",
    path="/path/to/research.pdf",
    display_name="Research Paper 2025"
)
```

### Batch Upload

```python
from pathlib import Path

files_dir = Path("/path/to/documents")
uploaded = []

for file_path in files_dir.glob("*.pdf"):
    response = client.upload_source_file(
        notebook_id="abc123",
        path=str(file_path)
    )
    uploaded.append(response.source_id)
    print(f"Uploaded: {file_path.name} -> {response.source_id}")

print(f"Total uploaded: {len(uploaded)}")
```

### File Validation

The SDK validates files before upload:

```python
from nblm import NblmError

# File not found
try:
    client.upload_source_file(
        notebook_id="abc123",
        path="/nonexistent/file.pdf"
    )
except NblmError as e:
    print(f"Error: {e}")  # "file not found"

# Not a file
try:
    client.upload_source_file(
        notebook_id="abc123",
        path="/path/to/directory"
    )
except NblmError as e:
    print(f"Error: {e}")  # "path is not a file"

# Empty file
try:
    client.upload_source_file(
        notebook_id="abc123",
        path="/path/to/empty.txt"
    )
except NblmError as e:
    print(f"Error: {e}")  # "cannot upload empty files"
```

### Supported File Types

The API supports various file types including:

- PDF (.pdf)
- Text files (.txt)
- Word documents (.docx)
- And more

Content type is auto-detected from file extension if not specified.

## Getting Source Details

### Get a Specific Source

```python
source = client.get_source(
    notebook_id="abc123",
    source_id="source-1"
)

print(f"Name: {source.name}")
print(f"Title: {source.title}")

if source.metadata:
    print(f"Word count: {source.metadata.word_count}")
    print(f"Added: {source.metadata.source_added_timestamp}")

if source.settings:
    print(f"Status: {source.settings.status}")
```

### YouTube Metadata

```python
source = client.get_source(notebook_id="abc123", source_id="video-source-1")

if source.metadata and source.metadata.youtube_metadata:
    yt = source.metadata.youtube_metadata
    print(f"Channel: {yt.channel_name}")
    print(f"Video ID: {yt.video_id}")
```

## Deleting Sources

### Delete Single Source

```python
client.delete_sources(
    notebook_id="abc123",
    source_names=[
        "projects/123456789012/locations/global/notebooks/abc123/sources/source-1"
    ]
)
```

### Delete Multiple Sources

```python
source_names = [
    "projects/123/locations/global/notebooks/abc123/sources/source-1",
    "projects/123/locations/global/notebooks/abc123/sources/source-2",
    "projects/123/locations/global/notebooks/abc123/sources/source-3"
]

client.delete_sources(
    notebook_id="abc123",
    source_names=source_names
)
```

### Get Source Names from Add Response

```python
from nblm import WebSource

# Add sources
response = client.add_sources(
    notebook_id="abc123",
    web_sources=[
        WebSource(url="https://example.com"),
        WebSource(url="https://example.org")
    ]
)

# Extract source names
source_names = [source["name"] for source in response.sources]

# Delete them
client.delete_sources(
    notebook_id="abc123",
    source_names=source_names
)
```

## Common Patterns

### Create Notebook and Add Sources

```python
from nblm import NblmClient, GcloudTokenProvider, WebSource, TextSource

client = NblmClient(
    token_provider=GcloudTokenProvider(),
    project_number="123456789012"
)

# Create notebook
notebook = client.create_notebook(title="Research: Python Best Practices")

# Add initial sources
client.add_sources(
    notebook_id=notebook.notebook_id,
    web_sources=[
        WebSource(url="https://peps.python.org/pep-0008/", name="PEP 8"),
        WebSource(url="https://docs.python-guide.org/", name="Python Guide")
    ],
    text_sources=[
        TextSource(
            content="Focus on code quality and readability",
            name="Project Goals"
        )
    ]
)

print(f"Notebook ready: {notebook.notebook_id}")
```

### Incremental Source Addition

```python
notebook_id = "abc123"

# Add web sources first
client.add_sources(
    notebook_id=notebook_id,
    web_sources=[WebSource(url="https://example.com")]
)

# Add text notes later
client.add_sources(
    notebook_id=notebook_id,
    text_sources=[TextSource(content="Notes", name="Notes")]
)

# Upload files separately
client.upload_source_file(
    notebook_id=notebook_id,
    path="document.pdf"
)
```

### Adding Google Drive Documents

```python
from nblm import GoogleDriveSource

notebook_id = "abc123"

# Authenticate with Drive-enabled credentials:
#   gcloud auth login --enable-gdrive-access
#   export NBLM_ACCESS_TOKEN=$(gcloud auth print-access-token)

client.add_sources(
    notebook_id=notebook_id,
    drive_sources=[
        GoogleDriveSource(
            document_id="FILE_ID",
            mime_type="application/vnd.google-apps.presentation",
            name="Team Update Slides",
        )
    ],
)
```

> **Tip:** The authenticated account must have view access to the Drive document. Use the Drive web UI to confirm you can open the file before ingesting it. `FILE_ID` can be extracted from the Drive URL at `/d/<ID>/` (e.g., `https://drive.google.com/file/d/<ID>/xxx`).

### Bulk Upload from Directory

```python
from pathlib import Path

def upload_directory(client: NblmClient, notebook_id: str, directory: str):
    """Upload all files from a directory to a notebook."""
    path = Path(directory)
    uploaded_count = 0

    for file_path in path.iterdir():
        if file_path.is_file():
            try:
                response = client.upload_source_file(
                    notebook_id=notebook_id,
                    path=str(file_path)
                )
                print(f"Uploaded: {file_path.name}")
                uploaded_count += 1
            except Exception as e:
                print(f"Failed to upload {file_path.name}: {e}")

    return uploaded_count

# Usage
count = upload_directory(client, "abc123", "/path/to/documents")
print(f"Uploaded {count} files")
```

### Extract Source IDs

```python
# Add sources and save IDs
response = client.add_sources(
    notebook_id="abc123",
    web_sources=[WebSource(url="https://example.com")]
)

# Extract source ID from full name
# Format: "projects/.../notebooks/.../sources/SOURCE_ID"
for source in response.sources:
    full_name = source["name"]
    source_id = full_name.split("/")[-1]
    print(f"Source ID: {source_id}")
```

## Error Handling

### Validation Errors

```python
from nblm import NblmError

# Empty text validation
try:
    client.add_sources(
        notebook_id="abc123",
        text_sources=[TextSource(content="   ", name="Empty")]
    )
except NblmError as e:
    print(f"Validation failed: {e}")

# No sources provided
try:
    client.add_sources(notebook_id="abc123")
except NblmError as e:
    print(f"Validation failed: {e}")
```

### API Errors

```python
# Notebook not found
try:
    client.add_sources(
        notebook_id="nonexistent",
        web_sources=[WebSource(url="https://example.com")]
    )
except NblmError as e:
    print(f"API error: {e}")

# Invalid URL
try:
    client.add_sources(
        notebook_id="abc123",
        web_sources=[WebSource(url="not-a-valid-url")]
    )
except NblmError as e:
    print(f"API error: {e}")
```

## Limitations

- **Source listing**: No API method to list all sources in a notebook
- **Source updates**: Cannot update existing sources, only add or delete

## Next Steps

- [Notebooks API](notebooks.md) - Create and manage notebooks
- [Audio API](audio.md) - Create audio overviews
- [API Reference](api-reference.md) - Complete API reference
- [Error Handling](error-handling.md) - Exception handling
