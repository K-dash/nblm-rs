from typing import Any

class WebSource:
    """Source type for adding web URLs to a notebook."""

    url: str
    name: str | None

    def __init__(self, url: str, name: str | None = None) -> None:
        """
        Create a WebSource.

        Args:
            url: Web URL to add
            name: Optional display name for the source
        """

class TextSource:
    """Source type for adding text content to a notebook."""

    content: str
    name: str | None

    def __init__(self, content: str, name: str | None = None) -> None:
        """
        Create a TextSource.

        Args:
            content: Text content to add
            name: Optional display name for the source
        """

class GoogleDriveSource:
    """Source type for adding Google Drive documents to a notebook."""

    document_id: str
    mime_type: str
    name: str | None

    def __init__(
        self,
        document_id: str,
        mime_type: str,
        name: str | None = None,
    ) -> None:
        """
        Create a GoogleDriveSource.

        Args:
            document_id: Google Drive document ID
            mime_type: MIME type returned by the Drive API
            name: Optional display name for the source
        """

class VideoSource:
    """Source type for adding YouTube videos to a notebook."""

    url: str

    def __init__(self, url: str) -> None:
        """
        Create a VideoSource.

        Args:
            url: YouTube video URL to add
        """

class BatchCreateSourcesResponse:
    """Response from adding sources to a notebook."""

    sources: list[NotebookSource]
    error_count: int | None

class BatchDeleteSourcesResponse:
    """Response from deleting sources from a notebook."""

    extra: dict[str, Any]

class UploadSourceFileResponse:
    """Response from uploading a file source to a notebook."""

    source_id: NotebookSourceId | None
    extra: dict[str, Any]

"""Data models for nblm"""

class NotebookSourceYoutubeMetadata:
    """Metadata for YouTube sources that were ingested into a notebook."""

    channel_name: str | None
    video_id: str | None
    extra: dict[str, Any]

class NotebookSourceSettings:
    """Source-level ingestion settings returned by the API."""

    status: str | None
    extra: dict[str, Any]

class NotebookSourceId:
    """Internal identifier for a notebook source."""

    id: str | None
    extra: dict[str, Any]

class NotebookSourceMetadata:
    """Timestamps and other attributes describing a notebook source."""

    source_added_timestamp: str | None
    word_count: int | None
    youtube_metadata: NotebookSourceYoutubeMetadata | None
    extra: dict[str, Any]

class NotebookSource:
    """A single source that has been added to a notebook."""

    name: str
    title: str | None
    metadata: NotebookSourceMetadata | None
    settings: NotebookSourceSettings | None
    source_id: NotebookSourceId | None
    extra: dict[str, Any]

class NotebookMetadata:
    """Top-level metadata describing a notebook."""

    create_time: str | None
    is_shareable: bool | None
    is_shared: bool | None
    last_viewed: str | None
    extra: dict[str, Any]

class Notebook:
    """Represents a NotebookLM notebook with structured fields."""

    name: str | None
    title: str
    notebook_id: str | None
    emoji: str | None
    metadata: NotebookMetadata | None
    sources: list[NotebookSource]
    extra: dict[str, Any]

class ListRecentlyViewedResponse:
    """Response from listing recently viewed notebooks."""

    notebooks: list[Notebook]

class BatchDeleteNotebooksResponse:
    """Aggregated results from batch notebook deletion."""

    deleted_notebooks: list[str]
    failed_notebooks: list[str]

class AudioOverviewRequest:
    """Request for creating an audio overview.

    Note: As of the current API version, this request must be empty.
    All fields are reserved for future use.
    """

    def __init__(self) -> None:
        """Create an empty AudioOverviewRequest."""

class AudioOverviewResponse:
    """Response from creating or getting an audio overview."""

    audio_overview_id: str | None
    name: str | None
    status: str | None
    generation_options: Any
    extra: dict[str, Any]
