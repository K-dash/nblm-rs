"""
NotebookLM Enterprise API client for Python

This package provides Python bindings for the NotebookLM Enterprise API.
"""

from importlib.metadata import PackageNotFoundError, version
from typing import TYPE_CHECKING

from .nblm import (
    DEFAULT_ENV_TOKEN_KEY,
    DEFAULT_GCLOUD_BINARY,
    AudioOverviewRequest,
    AudioOverviewResponse,
    BatchCreateSourcesResponse,
    BatchDeleteNotebooksResponse,
    BatchDeleteSourcesResponse,
    EnvTokenProvider,
    GcloudTokenProvider,
    GoogleDriveSource,
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
    TextSource,
    UploadSourceFileResponse,
    UserOAuthProvider,
    VideoSource,
    WebSource,
)

try:
    __version__ = version("nblm")
except PackageNotFoundError:
    # Package metadata not available (running from source without installation)
    __version__ = "0.0.0"

__all__ = [
    "DEFAULT_ENV_TOKEN_KEY",
    "DEFAULT_GCLOUD_BINARY",
    "AudioOverviewRequest",
    "AudioOverviewResponse",
    "BatchCreateSourcesResponse",
    "BatchDeleteNotebooksResponse",
    "BatchDeleteSourcesResponse",
    "EnvTokenProvider",
    "GcloudTokenProvider",
    "GoogleDriveSource",
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
    "TextSource",
    "UploadSourceFileResponse",
    "UserOAuthProvider",
    "VideoSource",
    "WebSource",
]
