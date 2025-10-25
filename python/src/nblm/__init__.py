"""
NotebookLM Enterprise API client for Python

This package provides Python bindings for the NotebookLM Enterprise API.
"""

from typing import TYPE_CHECKING

from .nblm import (
    DEFAULT_ENV_TOKEN_KEY,
    DEFAULT_GCLOUD_BINARY,
    BatchDeleteNotebooksResponse,
    EnvTokenProvider,
    GcloudTokenProvider,
    ListRecentlyViewedResponse,
    NblmClient,
    NblmError,
    Notebook,
    NotebookMetadata,
    NotebookSource,
    NotebookSourceId,
    NotebookSourceMetadata,
    NotebookSourceSettings,
    NotebookSourceYoutubeMetadata,
)

__version__ = "0.1.0"

__all__ = [
    "DEFAULT_ENV_TOKEN_KEY",
    "DEFAULT_GCLOUD_BINARY",
    "BatchDeleteNotebooksResponse",
    "EnvTokenProvider",
    "GcloudTokenProvider",
    "ListRecentlyViewedResponse",
    "NblmClient",
    "NblmError",
    "Notebook",
    "NotebookMetadata",
    "NotebookSource",
    "NotebookSourceId",
    "NotebookSourceMetadata",
    "NotebookSourceSettings",
    "NotebookSourceYoutubeMetadata",
]
