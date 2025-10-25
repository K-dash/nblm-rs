"""Data models for nblm"""

from typing import Any

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
