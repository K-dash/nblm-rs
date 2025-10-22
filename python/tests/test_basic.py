"""
Basic tests for nblm Python bindings
"""


def test_import() -> None:
    """Test that the module can be imported"""
    import nblm

    assert nblm.__version__ == "0.1.0"


def test_classes_available() -> None:
    """Test that all main classes are available"""
    from nblm import (
        DEFAULT_ENV_TOKEN_KEY,
        DEFAULT_GCLOUD_BINARY,
        DEFAULT_SERVICE_ACCOUNT_SCOPES,
        EnvTokenProvider,
        GcloudTokenProvider,
        NblmClient,
        NblmError,
        ServiceAccountTokenProvider,
    )

    assert NblmClient is not None
    assert GcloudTokenProvider is not None
    assert ServiceAccountTokenProvider is not None
    assert EnvTokenProvider is not None
    assert NblmError is not None
    assert DEFAULT_GCLOUD_BINARY == "gcloud"
    assert DEFAULT_ENV_TOKEN_KEY == "NBLM_ACCESS_TOKEN"
    assert DEFAULT_SERVICE_ACCOUNT_SCOPES == ["https://www.googleapis.com/auth/cloud-platform"]


def test_gcloud_token_provider_creation() -> None:
    """Test creating a GcloudTokenProvider"""
    from nblm import GcloudTokenProvider

    provider = GcloudTokenProvider()
    assert provider is not None

    provider_custom = GcloudTokenProvider(binary="/usr/bin/gcloud")
    assert provider_custom is not None


def test_env_token_provider_creation() -> None:
    """Test creating an EnvTokenProvider"""
    from nblm import EnvTokenProvider

    provider = EnvTokenProvider()
    assert provider is not None

    provider_custom = EnvTokenProvider(key="MY_CUSTOM_TOKEN")
    assert provider_custom is not None


def test_client_creation() -> None:
    """Test creating an NblmClient"""
    from nblm import GcloudTokenProvider, NblmClient

    provider = GcloudTokenProvider()
    client = NblmClient(
        token_provider=provider,
        project_number="123456789012",
        location="global",
        endpoint_location="global",
    )
    assert client is not None


def test_default_scope_helper_returns_copy() -> None:
    """Ensure default scope helper returns a fresh list"""
    from nblm import DEFAULT_SERVICE_ACCOUNT_SCOPES, default_service_account_scopes

    scopes = default_service_account_scopes()
    assert scopes == DEFAULT_SERVICE_ACCOUNT_SCOPES
    assert scopes is not DEFAULT_SERVICE_ACCOUNT_SCOPES
