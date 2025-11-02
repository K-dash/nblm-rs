# Audio API

Create and manage audio overviews (podcast-style discussions) for notebooks.

## Creating Audio Overviews

### Basic Creation

```python
from nblm import NblmClient, GcloudTokenProvider, AudioOverviewRequest

client = NblmClient(
    token_provider=GcloudTokenProvider(),
    project_number="123456789012"
)

# Create audio overview
audio = client.create_audio_overview(
    notebook_id="abc123",
    request=AudioOverviewRequest()
)

print(f"Audio overview ID: {audio.audio_overview_id}")
print(f"Name: {audio.name}")
print(f"Status: {audio.status}")
```

### Without Request Object

The request parameter is optional and defaults to an empty request:

```python
# Simplified version
audio = client.create_audio_overview(notebook_id="abc123")
```

### Response Attributes

```python
audio = client.create_audio_overview(notebook_id="abc123")

# Available attributes
print(f"ID: {audio.audio_overview_id}")
print(f"Full name: {audio.name}")
print(f"Status: {audio.status}")
print(f"Generation options: {audio.generation_options}")
print(f"Extra fields: {audio.extra}")
```

## Deleting Audio Overviews

### Basic Deletion

```python
client.delete_audio_overview(notebook_id="abc123")
```

### With Error Handling

```python
from nblm import NblmError

try:
    client.delete_audio_overview(notebook_id="abc123")
    print("Audio overview deleted successfully")
except NblmError as e:
    print(f"Failed to delete audio overview: {e}")
```

## Common Patterns

### Create Notebook with Audio

Complete workflow from creating a notebook to generating audio:

```python
from nblm import (
    NblmClient,
    GcloudTokenProvider,
    WebSource,
    AudioOverviewRequest
)

client = NblmClient(
    token_provider=GcloudTokenProvider(),
    project_number="123456789012"
)

# 1. Create notebook
notebook = client.create_notebook(title="Tutorial Analysis")
notebook_id = notebook.notebook_id

# 2. Add sources
client.add_sources(
    notebook_id=notebook_id,
    web_sources=[
        WebSource(url="https://example.com/tutorial-1"),
        WebSource(url="https://example.com/tutorial-2")
    ]
)

# 3. Create audio overview
audio = client.create_audio_overview(
    notebook_id=notebook_id,
    request=AudioOverviewRequest()
)

print(f"Audio overview created: {audio.audio_overview_id}")
print(f"Status: {audio.status}")
print("Check NotebookLM web UI for completion status")
```

### Recreate Audio Overview

```python
notebook_id = "abc123"

# Delete existing audio
try:
    client.delete_audio_overview(notebook_id=notebook_id)
    print("Deleted existing audio overview")
except NblmError:
    print("No existing audio overview")

# Create new audio
audio = client.create_audio_overview(notebook_id=notebook_id)
print(f"Created new audio overview: {audio.status}")
```

### Batch Create Audio Overviews

```python
from nblm import NblmError

notebook_ids = ["abc123", "def456", "ghi789"]
created = []
failed = []

for notebook_id in notebook_ids:
    try:
        audio = client.create_audio_overview(notebook_id=notebook_id)
        created.append(notebook_id)
        print(f"Created audio for {notebook_id}: {audio.status}")
    except NblmError as e:
        failed.append(notebook_id)
        print(f"Failed for {notebook_id}: {e}")

print(f"\nSummary: {len(created)} succeeded, {len(failed)} failed")
```

## Audio Overview Status

!!! important "Status Checking Limitation"
    As of now, there is no API to retrieve audio overview status. You must check the NotebookLM web UI in your browser to see when audio generation is complete or if it has failed.

### Status Values

When creating an audio overview, the initial status is:

| Status                                | Description               |
| ------------------------------------- | ------------------------- |
| `AUDIO_OVERVIEW_STATUS_IN_PROGRESS`   | Audio is being generated  |

### Checking Status

```python
audio = client.create_audio_overview(notebook_id="abc123")

print(f"Status: {audio.status}")
# Output: Status: AUDIO_OVERVIEW_STATUS_IN_PROGRESS
print("Audio generation in progress...")
print("Check NotebookLM web UI for completion")
print(f"URL: https://notebooklm.google.com/notebook/{notebook_id}")
```

!!! note "Status Values"
    The API only returns `AUDIO_OVERVIEW_STATUS_IN_PROGRESS` upon creation. Status values like `AUDIO_OVERVIEW_STATUS_COMPLETED` or `AUDIO_OVERVIEW_STATUS_FAILED` cannot be obtained via the API and must be checked in the browser.

## Error Handling

### Common Errors

**Notebook has no sources:**

```python
from nblm import NblmError

try:
    # Empty notebook
    audio = client.create_audio_overview(notebook_id="empty_notebook")
except NblmError as e:
    print(f"Error: {e}")
    # "Notebook must have at least one source"
```

**Audio already exists:**

```python
try:
    audio = client.create_audio_overview(notebook_id="abc123")
except NblmError as e:
    print(f"Error: {e}")
    # "Audio overview already exists"
    # Solution: Delete existing audio first
```

**Notebook not found:**

```python
try:
    audio = client.create_audio_overview(notebook_id="nonexistent")
except NblmError as e:
    print(f"Error: {e}")
    # "Notebook not found"
```

**Audio not found when deleting:**

```python
try:
    client.delete_audio_overview(notebook_id="abc123")
except NblmError as e:
    print(f"Error: {e}")
    # "Audio overview not found"
```

## API Limitations

As of 2025-10-25, the audio overview API has the following limitations:

### Configuration Not Supported

The API documentation mentions configuration fields like:

- `languageCode` - Audio language
- `sourceIds` - Specific sources to include
- `episodeFocus` - Topic focus

**However, these are not actually supported**. The API only accepts an empty request:

```python
# This is all you can do
request = AudioOverviewRequest()
audio = client.create_audio_overview(notebook_id="abc123", request=request)

# Or simply
audio = client.create_audio_overview(notebook_id="abc123")
```

Language and other settings must be configured through the NotebookLM web UI.

### No Status Polling

There is no API method to check audio generation progress:

```python
# This does NOT exist
# status = client.get_audio_overview_status(notebook_id="abc123")  # Not available

# You must check the web UI for status
```

### One Audio Per Notebook

Only one audio overview can exist per notebook:

```python
# First call succeeds
audio1 = client.create_audio_overview(notebook_id="abc123")

# Second call fails (audio already exists)
try:
    audio2 = client.create_audio_overview(notebook_id="abc123")
except NblmError as e:
    print("Must delete existing audio first")
```

## Best Practices

### Always Check for Sources

```python
from nblm import WebSource

def create_audio_safely(client: NblmClient, notebook_id: str):
    # Ensure notebook has sources
    client.add_sources(
        notebook_id=notebook_id,
        web_sources=[WebSource(url="https://example.com")]
    )

    # Then create audio
    audio = client.create_audio_overview(notebook_id=notebook_id)
    return audio
```

### Handle Existing Audio

```python
def recreate_audio(client: NblmClient, notebook_id: str):
    # Try to delete existing audio (ignore if doesn't exist)
    try:
        client.delete_audio_overview(notebook_id=notebook_id)
    except NblmError:
        pass

    # Create new audio
    audio = client.create_audio_overview(notebook_id=notebook_id)
    return audio
```

## Next Steps

- [Notebooks API](notebooks.md) - Create and manage notebooks
- [Sources API](sources.md) - Add sources before creating audio
- [API Reference](api-reference.md) - Complete API reference
- [Error Handling](error-handling.md) - Exception handling patterns
