"""Type stubs for nblm Python bindings"""

from ._auth import (
    DEFAULT_ENV_TOKEN_KEY,
    DEFAULT_GCLOUD_BINARY,
    DEFAULT_SERVICE_ACCOUNT_SCOPES,
    EnvTokenProvider,
    GcloudTokenProvider,
    NblmError,
    ServiceAccountTokenProvider,
    default_service_account_scopes,
)
from ._client import NblmClient
from ._models import (
    BatchDeleteNotebooksResponse,
    ListRecentlyViewedResponse,
    Notebook,
)

__version__: str

__all__ = [
    "DEFAULT_ENV_TOKEN_KEY",
    "DEFAULT_GCLOUD_BINARY",
    "DEFAULT_SERVICE_ACCOUNT_SCOPES",
    "BatchDeleteNotebooksResponse",
    "EnvTokenProvider",
    "GcloudTokenProvider",
    "ListRecentlyViewedResponse",
    "NblmClient",
    "NblmError",
    "Notebook",
    "ServiceAccountTokenProvider",
    "default_service_account_scopes",
]
