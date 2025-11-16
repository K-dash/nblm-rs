import json
import tempfile
from collections.abc import Iterator
from pathlib import Path

import pytest

from nblm import UserOAuthProvider


@pytest.fixture()
def temp_config_dir(monkeypatch: pytest.MonkeyPatch) -> Iterator[Path]:
    temp_dir = tempfile.TemporaryDirectory()
    monkeypatch.setenv("NBLM_CONFIG_DIR", temp_dir.name)
    try:
        yield Path(temp_dir.name)
    finally:
        temp_dir.cleanup()


def write_credentials(config_dir: Path, key: str) -> None:
    config_dir.mkdir(parents=True, exist_ok=True)
    payload = {
        "version": 1,
        "entries": {
            key: {
                "refresh_token": "fake_refresh_token_xyz",
                "scopes": ["https://www.googleapis.com/auth/cloud-platform"],
                "expires_at": None,
                "token_type": "Bearer",
                "updated_at": "2025-01-01T00:00:00Z",
            }
        },
    }
    (config_dir / "credentials.json").write_text(json.dumps(payload))


def token_store_key(project_number: int, endpoint_location: str, user: str | None = None) -> str:
    parts = ["enterprise", f"project={project_number}", f"location={endpoint_location}"]
    if user:
        parts.append(f"user={user}")
    return ":".join(parts)


def test_user_oauth_provider_from_file(
    temp_config_dir: Path, monkeypatch: pytest.MonkeyPatch
) -> None:
    monkeypatch.setenv("NBLM_OAUTH_CLIENT_ID", "fake-client-id")
    write_credentials(temp_config_dir, token_store_key(123456, "global"))

    provider = UserOAuthProvider.from_file(
        project_number=123456,
        location="us-central1",
    )

    assert provider is not None
    assert provider.endpoint_location == "global"


def test_user_oauth_provider_missing_client_id(
    monkeypatch: pytest.MonkeyPatch, temp_config_dir: Path
) -> None:
    monkeypatch.delenv("NBLM_OAUTH_CLIENT_ID", raising=False)
    write_credentials(temp_config_dir, token_store_key(999999, "global"))

    with pytest.raises(ValueError):
        UserOAuthProvider.from_file(project_number=999999)


def test_user_oauth_provider_missing_credentials(
    monkeypatch: pytest.MonkeyPatch, _temp_config_dir: Path
) -> None:
    monkeypatch.setenv("NBLM_OAUTH_CLIENT_ID", "fake-client-id")

    with pytest.raises(ValueError, match="No refresh token found"):
        UserOAuthProvider.from_file(project_number=42)
