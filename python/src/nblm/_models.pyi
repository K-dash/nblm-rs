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
