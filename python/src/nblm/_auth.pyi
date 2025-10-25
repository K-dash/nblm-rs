"""Authentication providers for nblm"""

DEFAULT_GCLOUD_BINARY: str
DEFAULT_ENV_TOKEN_KEY: str

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

TokenProvider = GcloudTokenProvider | EnvTokenProvider
