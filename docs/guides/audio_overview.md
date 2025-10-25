# Audio Overview Guide

This guide covers how to create and manage audio overviews (podcast-style discussions) from your notebook content.

## Overview

The Audio Overview feature generates an AI-powered podcast discussion based on the sources in your notebook. This creates an engaging audio summary that can help with understanding and retention of the material.

## API Limitations

> [!IMPORTANT]
> As of 2025-10-25, the NotebookLM API has the following limitations for audio overview:
>
> - The API only accepts an empty request body `{}`
> - Configuration options like `sourceIds`, `episodeFocus`, and `languageCode` mentioned in the documentation are not yet supported
> - These settings can only be configured through the NotebookLM UI after creation

## Creating Audio Overview

### Using the CLI

```bash
# Create audio overview for a notebook
nblm audio create --notebook-id NOTEBOOK_ID

# Example output:
# Audio overview created successfully:
#   Name: corpora/abc123/notebooks/xyz789/audioOverviews/default
#   Status: AUDIO_OVERVIEW_STATUS_IN_PROGRESS
```

### Using Python

```python
from nblm import NblmClient, GcloudTokenProvider, AudioOverviewRequest

# Initialize client
token_provider = GcloudTokenProvider()
client = NblmClient(
    token_provider=token_provider,
    project_number="YOUR_PROJECT_NUMBER"
)

# Create audio overview (currently requires empty request)
response = client.create_audio_overview(
    notebook_id="NOTEBOOK_ID",
    request=AudioOverviewRequest()  # Must be empty due to API limitations
)

print(f"Audio overview created: {response.name}")
print(f"Status: {response.status}")
```

## Audio Status Values

The API returns status as an enum-like string with the prefix `AUDIO_OVERVIEW_STATUS_`.
As of 2025-10-25, the Python bindings only surface `AUDIO_OVERVIEW_STATUS_IN_PROGRESS`. Other status values (such as completion or failure states) are not currently exposed through the public API, so you must check the NotebookLM UI to see them. For the authoritative list of status codes, refer to the official [NotebookLM Enterprise API reference](https://cloud.google.com/gemini/enterprise/notebooklm-enterprise/docs/api-audio-overview).

> [!NOTE]
> The API does not provide a way to check the current status after creation. You need to check through the NotebookLM UI.

## Deleting Audio Overview

### Using the CLI

```bash
# Delete audio overview
nblm audio delete --notebook-id NOTEBOOK_ID
```

### Using Python

```python
# Delete audio overview
client.delete_audio_overview(notebook_id="NOTEBOOK_ID")
print("Audio overview deleted successfully")
```

## Best Practices

1. **Wait for Sources to Process**: Ensure all sources in your notebook have been fully processed before creating an audio overview.

2. **Content Requirements**: The notebook should have sufficient content from sources to generate a meaningful discussion.

3. **Language Considerations**: Currently, language selection is only available through the UI, not the API.

4. **Error Handling**: Always handle potential errors when creating audio overviews:

```python
from nblm import NblmError

try:
    audio = client.create_audio_overview(notebook_id)
    print(f"Created: {audio.name}")
except NblmError as e:
    print(f"Failed to create audio: {e}")
```

## Example Workflow

```python
# Complete workflow example
from nblm import (
    NblmClient, GcloudTokenProvider,
    WebSource, AudioOverviewRequest, NblmError
)

token_provider = GcloudTokenProvider()
client = NblmClient(
    token_provider=token_provider,
    project_number="YOUR_PROJECT_NUMBER"
)

# 1. Create notebook
notebook = client.create_notebook("Audio Test Notebook")

# 2. Add sources
client.add_sources(
    notebook_id=notebook.notebook_id,
    web_sources=[
        WebSource(url="https://en.wikipedia.org/wiki/Machine_learning"),
        WebSource(url="https://en.wikipedia.org/wiki/Artificial_intelligence")
    ]
)

# 3. Wait a moment for sources to process (in production, implement proper polling)
import time
time.sleep(10)

# 4. Create audio overview
try:
    audio = client.create_audio_overview(
        notebook_id=notebook.notebook_id,
        request=AudioOverviewRequest()
    )
    print(f"Audio overview created: {audio.name}")
    print(f"Initial status: {audio.status}")
except NblmError as e:
    print(f"Failed to create audio: {e}")

# 5. Clean up when done
client.delete_audio_overview(notebook_id=notebook.notebook_id)
client.delete_notebooks([notebook.name])
```

## Related Commands

- `nblm audio create --notebook-id NOTEBOOK_ID` - Create audio overview
- `nblm audio delete --notebook-id NOTEBOOK_ID` - Delete audio overview
- `nblm notebooks recent` - List notebooks to get IDs

## Troubleshooting

### Audio Creation Fails

If audio creation fails, check:

1. The notebook has at least one processed source
2. The sources contain sufficient content
3. You're using an empty request body (no configuration fields)

### Cannot Configure Audio Settings

Currently, audio settings like language and focus topics can only be configured through the NotebookLM web interface, not via the API.

## Future Enhancements

Once the API supports configuration options, you'll be able to:

- Select specific sources for the audio
- Set the episode focus/topic
- Choose the language for the audio
- Customize other generation parameters

These fields are already defined in the code but commented out, ready for when the API adds support.
