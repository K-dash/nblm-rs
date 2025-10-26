# API Reference

Complete reference for all classes and methods in the nblm Python SDK.

## Client

### NblmClient

Main client class for interacting with the NotebookLM API.

```python
from nblm import NblmClient, GcloudTokenProvider

client = NblmClient(
    token_provider=GcloudTokenProvider(),
    project_number="123456789012",
    location="global",
    endpoint_location="global"
)
```

#### Constructor Parameters

| Parameter           | Type          | Required | Default  | Description                   |
| ------------------- | ------------- | -------- | -------- | ----------------------------- |
| `token_provider`    | TokenProvider | Yes      | -        | Authentication token provider |
| `project_number`    | str           | Yes      | -        | Google Cloud project number   |
| `location`          | str           | No       | "global" | API location                  |
| `endpoint_location` | str           | No       | "global" | Endpoint location             |

#### Methods

##### Notebooks

**`create_notebook(title: str) -> Notebook`**

Create a new notebook.

```python
notebook = client.create_notebook(title="My Notebook")
```

**`list_recently_viewed(page_size: Optional[int] = None) -> ListRecentlyViewedResponse`**

List recently viewed notebooks.

```python
response = client.list_recently_viewed(page_size=10)
```

**`delete_notebooks(notebook_names: List[str]) -> BatchDeleteNotebooksResponse`**

Delete one or more notebooks.

```python
client.delete_notebooks([
    "projects/123/locations/global/notebooks/abc"
])
```

##### Sources

**`add_sources(notebook_id: str, web_sources: Optional[List[WebSource]] = None, text_sources: Optional[List[TextSource]] = None, drive_sources: Optional[List[GoogleDriveSource]] = None, video_sources: Optional[List[VideoSource]] = None) -> BatchCreateSourcesResponse`**

Add sources to a notebook.

```python
from nblm import WebSource, TextSource, GoogleDriveSource

response = client.add_sources(
    notebook_id="abc123",
    web_sources=[WebSource(url="https://example.com", name="Example")],
    text_sources=[TextSource(content="Notes", name="My Notes")],
    drive_sources=[GoogleDriveSource(
        document_id="FILE_ID",
        mime_type="application/vnd.google-apps.presentation",
        name="Team Update",
    )],
)
```

**`upload_source_file(notebook_id: str, path: str, content_type: Optional[str] = None, display_name: Optional[str] = None) -> UploadSourceFileResponse`**

Upload a file as a source.

```python
response = client.upload_source_file(
    notebook_id="abc123",
    path="/path/to/file.pdf",
    content_type="application/pdf",
    display_name="My Document"
)
```

**`get_source(notebook_id: str, source_id: str) -> NotebookSource`**

Get a specific source by ID.

```python
source = client.get_source(
    notebook_id="abc123",
    source_id="source-1"
)
```

**`delete_sources(notebook_id: str, source_names: List[str]) -> BatchDeleteSourcesResponse`**

Delete sources from a notebook.

```python
client.delete_sources(
    notebook_id="abc123",
    source_names=["projects/.../notebooks/abc123/sources/source-1"]
)
```

##### Audio

**`create_audio_overview(notebook_id: str, request: Optional[AudioOverviewRequest] = None) -> AudioOverviewResponse`**

Create an audio overview.

```python
from nblm import AudioOverviewRequest

audio = client.create_audio_overview(
    notebook_id="abc123",
    request=AudioOverviewRequest()
)
```

**`delete_audio_overview(notebook_id: str) -> None`**

Delete an audio overview.

```python
client.delete_audio_overview(notebook_id="abc123")
```

## Token Providers

### GcloudTokenProvider

Use gcloud CLI for authentication.

```python
from nblm import GcloudTokenProvider

# Default (uses 'gcloud' from PATH)
provider = GcloudTokenProvider()

# Custom binary path
provider = GcloudTokenProvider(binary="/usr/local/bin/gcloud")
```

#### Constructor Parameters

| Parameter | Type | Required | Default  | Description           |
| --------- | ---- | -------- | -------- | --------------------- |
| `binary`  | str  | No       | "gcloud" | Path to gcloud binary |

### EnvTokenProvider

Use access token from environment variable.

```python
import os
from nblm import EnvTokenProvider

os.environ["NBLM_ACCESS_TOKEN"] = "your-token"
provider = EnvTokenProvider()

# Or use custom environment variable key
os.environ["MY_TOKEN"] = "your-token"
provider = EnvTokenProvider(key="MY_TOKEN")
```

#### Constructor Parameters

| Parameter | Type | Required | Default             | Description              |
| --------- | ---- | -------- | ------------------- | ------------------------ |
| `key`     | str  | No       | "NBLM_ACCESS_TOKEN" | Environment variable key |

## Models

### Notebook

Represents a NotebookLM notebook.

#### Attributes

| Attribute     | Type          | Description                    |
| ------------- | ------------- | ------------------------------ |
| `name`        | Optional[str] | Full resource name             |
| `notebook_id` | Optional[str] | Notebook identifier            |
| `title`       | Optional[str] | Notebook title                 |
| `create_time` | Optional[str] | Creation timestamp             |
| `update_time` | Optional[str] | Last update timestamp          |
| `extra`       | dict          | Additional API response fields |

### WebSource

Web URL source for adding to notebooks.

```python
from nblm import WebSource

source = WebSource(
    url="https://example.com",
    name="Example Website"  # Optional
)
```

#### Attributes

| Attribute | Type          | Description  |
| --------- | ------------- | ------------ |
| `url`     | str           | Web page URL |
| `name`    | Optional[str] | Display name |

### TextSource

Text content source for adding to notebooks.

```python
from nblm import TextSource

source = TextSource(
    content="My notes",
    name="Notes"  # Optional
)
```

#### Attributes

| Attribute | Type          | Description  |
| --------- | ------------- | ------------ |
| `content` | str           | Text content |
| `name`    | Optional[str] | Display name |

### GoogleDriveSource

Google Drive document source for adding to notebooks.

```python
from nblm import GoogleDriveSource

source = GoogleDriveSource(
    document_id="FILE_ID",
    mime_type="application/vnd.google-apps.presentation",
    name="Team Update Slides",  # Optional
)
```

> **Prerequisite:** Authenticate with Drive access enabled (`gcloud auth login --enable-gdrive-access`) and ensure the document is shared with the authenticated account.
> **Tip:** `document_id` can be extracted from the Drive URL at `/d/<ID>/` (e.g., `https://drive.google.com/file/d/<ID>/xxx`).

#### Attributes

| Attribute     | Type          | Description                          |
| ------------- | ------------- | ------------------------------------ |
| `document_id` | str           | Google Drive document ID             |
| `mime_type`   | str           | MIME type returned by the Drive API  |
| `name`        | Optional[str] | Display name shown in NotebookLM     |

### VideoSource

YouTube video source for adding to notebooks.

```python
from nblm import VideoSource

source = VideoSource(url="https://www.youtube.com/watch?v=VIDEO_ID")
```

#### Attributes

| Attribute | Type | Description       |
| --------- | ---- | ----------------- |
| `url`     | str  | YouTube video URL |

### AudioOverviewRequest

Request for creating an audio overview.

```python
from nblm import AudioOverviewRequest

request = AudioOverviewRequest()
```

> **Note**: The API currently only accepts empty requests. Configuration fields are not supported.

### NotebookSource

Represents a source within a notebook.

#### Attributes

| Attribute   | Type                             | Description        |
| ----------- | -------------------------------- | ------------------ |
| `name`      | str                              | Full resource name |
| `title`     | Optional[str]                    | Source title       |
| `metadata`  | Optional[NotebookSourceMetadata] | Source metadata    |
| `settings`  | Optional[NotebookSourceSettings] | Source settings    |
| `source_id` | Optional[NotebookSourceId]       | Source ID          |
| `extra`     | dict                             | Additional fields  |

### AudioOverviewResponse

Response from creating or getting an audio overview.

#### Attributes

| Attribute            | Type          | Description                    |
| -------------------- | ------------- | ------------------------------ |
| `audio_overview_id`  | Optional[str] | Audio overview identifier      |
| `name`               | Optional[str] | Full resource name             |
| `status`             | Optional[str] | Processing status              |
| `generation_options` | Any           | Generation options             |
| `extra`              | dict          | Additional API response fields |

## Response Objects

### ListRecentlyViewedResponse

Response from listing notebooks.

#### Attributes

| Attribute   | Type       | Description                    |
| ----------- | ---------- | ------------------------------ |
| `notebooks` | List[dict] | List of notebook data          |
| `extra`     | dict       | Additional API response fields |

### BatchCreateSourcesResponse

Response from adding sources.

#### Attributes

| Attribute | Type       | Description                    |
| --------- | ---------- | ------------------------------ |
| `sources` | List[dict] | List of created sources        |
| `extra`   | dict       | Additional API response fields |

### BatchDeleteNotebooksResponse

Response from deleting notebooks.

#### Attributes

| Attribute | Type | Description                    |
| --------- | ---- | ------------------------------ |
| `extra`   | dict | Additional API response fields |

### BatchDeleteSourcesResponse

Response from deleting sources.

#### Attributes

| Attribute | Type | Description                    |
| --------- | ---- | ------------------------------ |
| `extra`   | dict | Additional API response fields |

### UploadSourceFileResponse

Response from uploading a file.

#### Attributes

| Attribute   | Type          | Description       |
| ----------- | ------------- | ----------------- |
| `source_id` | Optional[str] | Created source ID |
| `extra`     | dict          | Additional fields |

## Exceptions

### NblmError

Base exception for all nblm errors.

```python
from nblm import NblmError

try:
    notebook = client.create_notebook(title="Test")
except NblmError as e:
    print(f"Error: {e}")
```

See [Error Handling](error-handling.md) for detailed exception handling patterns.

## Type Hints

All classes and methods include complete type hints for IDE support:

```python
from nblm import (
    NblmClient,
    Notebook,
    NotebookSource,
    ListRecentlyViewedResponse,
    BatchCreateSourcesResponse,
    WebSource,
    TextSource,
    VideoSource,
    AudioOverviewRequest,
    AudioOverviewResponse,
)

# Type checking with mypy
def create_and_populate(client: NblmClient, title: str) -> Notebook:
    notebook: Notebook = client.create_notebook(title=title)
    response: BatchCreateSourcesResponse = client.add_sources(
        notebook_id=notebook.notebook_id,
        web_sources=[WebSource(url="https://example.com")]
    )
    return notebook
```

## Next Steps

- [Quickstart](quickstart.md) - Get started quickly
- [Notebooks API](notebooks.md) - Notebook operations guide
- [Sources API](sources.md) - Source operations guide
- [Error Handling](error-handling.md) - Exception handling
