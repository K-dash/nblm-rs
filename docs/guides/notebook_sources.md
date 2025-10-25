# Notebook Sources

This guide explains how to manage NotebookLM notebook sources using the nblm Python bindings.

> [!NOTE]
> Source management (add/delete) requires write access to the target notebook. Ensure your access token or gcloud account has appropriate permissions.

## Adding Sources

Use `NblmClient.add_sources()` to attach web pages, inline text, or YouTube videos to an existing notebook.

```python
from nblm import GcloudTokenProvider, NblmClient, WebSource, TextSource, VideoSource

provider = GcloudTokenProvider()
client = NblmClient(token_provider=provider, project_number="123456789012")

response = client.add_sources(
    notebook_id="abc123",  # Notebook resource ID (not the full name)
    web_sources=[
        WebSource(url="https://example.com/article", name="Reference Article"),
        WebSource(url="https://python.org"),  # name is optional
    ],
    text_sources=[
        TextSource(content="Inline notes about the topic", name="Research Notes"),
    ],
    video_sources=[
        VideoSource(url="https://youtube.com/watch?v=VIDEO_ID"),
    ],
)

for result in response.sources:
    print(f"Source created: {result.name}")
```

### Source Types

- **`WebSource(url, name=None)`**: Web URL with optional display name
- **`TextSource(content, name=None)`**: Inline text with optional display name
  - Text content must be non-empty after trimming
- **`VideoSource(url)`**: YouTube video URL
  - Video sources currently don't support custom names (API limitation)

At least one source type must be provided.

`BatchCreateSourcesResponse.error_count` reflects the number of ingestion failures reported by the API.

## Uploading Files

You can upload local files to a notebook via the CLI or Python bindings. The API currently returns the created `sourceId` and any additional metadata provided by NotebookLM.

### CLI example

```bash
nblm sources upload \
  --notebook-id abc123 \
  --file ./docs/brief.pdf \
  --content-type application/pdf
```

The command prints the notebook ID, file metadata, and the returned source ID. When `--content-type` is omitted, the CLI guesses it from the file extension and falls back to `application/octet-stream`.

> [!NOTE]
> As of 2025-10-25 the NotebookLM API rejects custom display names for uploads (returns HTTP 400). The CLI keeps `--display-name` for forward compatibility but currently ignores it.

### Python example

```python
from pathlib import Path

from nblm import GcloudTokenProvider, NblmClient

provider = GcloudTokenProvider()
client = NblmClient(token_provider=provider, project_number="123456789012")

result = client.upload_source_file(
    notebook_id="abc123",
    path=Path("./docs/brief.pdf"),
    content_type="application/pdf",  # optional
    display_name="Product Brief",    # optional
)

print(result.source_id.id if result.source_id else "<missing>")
```

Empty files are rejected client-side. If the API fails to ingest the file, an `nblm.NblmError` is raised with the server-provided message.

## Deleting Sources

Use `NblmClient.delete_sources()` to remove previously added sources by their full resource names.

```python
response = client.delete_sources(
    notebook_id="abc123",
    source_names=[
        "projects/123456789012/locations/global/notebooks/abc123/sources/source-1",
    ],
)

print(response.extra)  # Typically empty, includes API metadata if present
```

## Error Handling

- Parameter validation errors raise `nblm.NblmError` with message `validation error: ...`.
- API failures raise `nblm.NblmError` with details from the NotebookLM service.

## Related Commands

- CLI equivalent: `nblm sources add ...` and `nblm sources delete ...`
- See `docs/guides/authentication.md` for token setup.
