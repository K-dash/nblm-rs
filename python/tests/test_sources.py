"""Tests for sources operations bindings."""

import nblm


def test_sources_response_types_available() -> None:
    """Ensure source-related response classes are accessible."""

    assert nblm.BatchCreateSourcesResponse is not None
    assert nblm.BatchDeleteSourcesResponse is not None
    assert nblm.UploadSourceFileResponse is not None
    assert nblm.NotebookSource is not None
    assert nblm.WebSource is not None
    assert nblm.TextSource is not None
    assert nblm.VideoSource is not None


def test_client_sources_methods_exist() -> None:
    """Verify NblmClient exposes sources methods without instantiation."""

    assert hasattr(nblm.NblmClient, "add_sources")
    assert hasattr(nblm.NblmClient, "delete_sources")
    assert hasattr(nblm.NblmClient, "upload_source_file")
    assert hasattr(nblm.NblmClient, "get_source")
