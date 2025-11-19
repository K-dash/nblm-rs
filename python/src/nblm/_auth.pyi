"""Authentication providers for nblm"""

DEFAULT_GCLOUD_BINARY: str
DEFAULT_ENV_TOKEN_KEY: str

def login(drive_access: bool = False, force: bool = False) -> None:
    """
    Log in via Google Cloud SDK (gcloud auth login).

    Args:
        drive_access: If True, requests Google Drive access.
        force: If True, forces re-authentication.
    """

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

class UserOAuthProvider:
    """Token provider that reuses refresh tokens created via the CLI's user-oauth flow"""

    @staticmethod
    def from_file(
        project_number: int | None = ...,
        location: str = "global",
        user: str | None = ...,
        endpoint_location: str | None = ...,
    ) -> UserOAuthProvider:
        """Load refresh tokens from the shared credentials file."""

    @property
    def endpoint_location(self) -> str:
        """Return the endpoint location associated with the stored token."""

TokenProvider = GcloudTokenProvider | EnvTokenProvider | UserOAuthProvider
