"""Tests for audio operations bindings."""

import nblm


def test_audio_types_available() -> None:
    """Ensure audio-related types are accessible."""

    assert nblm.AudioOverviewRequest is not None
    assert nblm.AudioOverviewResponse is not None


def test_audio_request_instantiation() -> None:
    """Test creating AudioOverviewRequest instances."""

    # Should create successfully with no arguments
    request = nblm.AudioOverviewRequest()
    assert request is not None


def test_client_audio_methods_exist() -> None:
    """Verify NblmClient exposes audio methods without instantiation."""

    assert hasattr(nblm.NblmClient, "create_audio_overview")
    assert hasattr(nblm.NblmClient, "delete_audio_overview")
