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

### Context Manager for Cleanup

```python
from contextlib import contextmanager
from typing import Generator

@contextmanager
def temporary_audio(
    client: NblmClient,
    notebook_id: str
) -> Generator[AudioOverviewResponse, None, None]:
    """Create audio overview and delete it when done."""
    audio = client.create_audio_overview(notebook_id=notebook_id)
    try:
        yield audio
    finally:
        client.delete_audio_overview(notebook_id=notebook_id)

# Usage
with temporary_audio(client, "abc123") as audio:
    print(f"Testing audio: {audio.audio_overview_id}")
    # Audio is automatically deleted after this block
```

## Audio Overview Status

### Status Values

Audio overviews can have the following statuses:

| Status       | Description               |
| ------------ | ------------------------- |
| `PROCESSING` | Audio is being generated  |
| `COMPLETED`  | Audio generation complete |
| `FAILED`     | Audio generation failed   |

### Checking Status

```python
audio = client.create_audio_overview(notebook_id="abc123")

if audio.status == "PROCESSING":
    print("Audio is being generated...")
    print("Check NotebookLM web UI for completion")
elif audio.status == "COMPLETED":
    print("Audio is ready!")
elif audio.status == "FAILED":
    print("Audio generation failed")
```

> **Note**: The SDK does not provide a method to poll for status updates. You must check the NotebookLM web UI to see when processing is complete.

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

### Wait for Completion

```python
import time

def wait_for_audio_message(notebook_id: str):
    """Print message about waiting for audio generation."""
    print(f"Audio overview created for notebook: {notebook_id}")
    print("Generation in progress (typically 3-5 minutes)...")
    print("Check NotebookLM web UI for completion status")
    print(f"URL: https://notebooklm.google.com/notebook/{notebook_id}")

# Usage
audio = client.create_audio_overview(notebook_id="abc123")
wait_for_audio_message("abc123")
```

## Next Steps

- [Notebooks API](notebooks.md) - Create and manage notebooks
- [Sources API](sources.md) - Add sources before creating audio
- [API Reference](api-reference.md) - Complete API reference
- [Error Handling](error-handling.md) - Exception handling patterns
