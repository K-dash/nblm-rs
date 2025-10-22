"""Type stubs for nblm Python bindings"""

__version__: str
DEFAULT_GCLOUD_BINARY: str
DEFAULT_ENV_TOKEN_KEY: str
DEFAULT_SERVICE_ACCOUNT_SCOPES: list[str]

def default_service_account_scopes() -> list[str]:
    """Return the default OAuth scopes used for service account authentication."""

class NblmError(Exception):
    """Base exception for nblm errors"""

class GcloudTokenProvider:
    """Token provider that uses gcloud CLI for authentication"""

    def __init__(self, binary: str = DEFAULT_GCLOUD_BINARY) -> None:
        """
        Create a new GcloudTokenProvider

        Args:
            binary: Path to gcloud binary (default: DEFAULT_GCLOUD_BINARY)
        """

class EnvTokenProvider:
    """Token provider that reads access token from environment variable"""

    def __init__(self, key: str = DEFAULT_ENV_TOKEN_KEY) -> None:
        """
        Create a new EnvTokenProvider

        Args:
            key: Environment variable name (default: DEFAULT_ENV_TOKEN_KEY)
        """

class ServiceAccountTokenProvider:
    """Token provider that uses service account key for authentication"""

    @staticmethod
    def from_file(
        path: str,
        scopes: list[str] = DEFAULT_SERVICE_ACCOUNT_SCOPES,
    ) -> ServiceAccountTokenProvider:
        """
        Create a new ServiceAccountTokenProvider from a JSON key file

        Args:
            path: Path to service account JSON key file
            scopes: OAuth scopes (default: DEFAULT_SERVICE_ACCOUNT_SCOPES)

        Returns:
            ServiceAccountTokenProvider instance

        Raises:
            NblmError: If the key file cannot be read or parsed
        """

    @staticmethod
    def from_json(
        json_data: str,
        scopes: list[str] = DEFAULT_SERVICE_ACCOUNT_SCOPES,
    ) -> ServiceAccountTokenProvider:
        """
        Create a new ServiceAccountTokenProvider from JSON string

        Args:
            json_data: Service account key as JSON string
            scopes: OAuth scopes (default: DEFAULT_SERVICE_ACCOUNT_SCOPES)

        Returns:
            ServiceAccountTokenProvider instance

        Raises:
            NblmError: If the JSON cannot be parsed
        """

TokenProvider = GcloudTokenProvider | EnvTokenProvider | ServiceAccountTokenProvider

class NblmClient:
    """NotebookLM Enterprise API client"""

    def __init__(
        self,
        token_provider: TokenProvider,
        project_number: str,
        location: str = "global",
        endpoint_location: str = "global",
    ) -> None:
        """
        Create a new NblmClient

        Args:
            token_provider: Token provider for authentication
            project_number: Google Cloud project number
            location: NotebookLM location (default: "global")
            endpoint_location: API endpoint location (default: "global")

        Raises:
            NblmError: If the client cannot be created
        """

__all__ = [
    "DEFAULT_ENV_TOKEN_KEY",
    "DEFAULT_GCLOUD_BINARY",
    "DEFAULT_SERVICE_ACCOUNT_SCOPES",
    "EnvTokenProvider",
    "GcloudTokenProvider",
    "NblmClient",
    "NblmError",
    "ServiceAccountTokenProvider",
    "default_service_account_scopes",
]
