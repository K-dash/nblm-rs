"""Type stubs for nblm Python bindings"""

from ._auth import (
    DEFAULT_ENV_TOKEN_KEY,
    DEFAULT_GCLOUD_BINARY,
    EnvTokenProvider,
    GcloudTokenProvider,
    NblmError,
    UserOAuthProvider,
)
from ._client import NblmClient
from ._models import (
    AudioOverviewRequest,
    AudioOverviewResponse,
    BatchCreateSourcesResponse,
    BatchDeleteNotebooksResponse,
    BatchDeleteSourcesResponse,
    GoogleDriveSource,
    ListRecentlyViewedResponse,
    Notebook,
    NotebookMetadata,
    NotebookSource,
    NotebookSourceId,
    NotebookSourceMetadata,
    NotebookSourceSettings,
    NotebookSourceYoutubeMetadata,
    TextSource,
    UploadSourceFileResponse,
    VideoSource,
    WebSource,
)

__version__: str

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
