"""
Tests for notebook operations

Note: These tests only verify that the types and interfaces are available.
Integration tests with actual API calls would require:
- Valid Google Cloud credentials
- A real project number
- Network access to NotebookLM API

For unit testing Rust-backed PyO3 classes, mocking is not feasible.
Consider using integration tests or end-to-end tests instead.
"""

from nblm import (
    BatchDeleteNotebooksResponse,
    EnvTokenProvider,
    GcloudTokenProvider,
    ListRecentlyViewedResponse,
    NblmClient,
    Notebook,
)


def test_notebook_type_imports() -> None:
    """Test that notebook-related types can be imported"""

    assert Notebook is not None
    assert ListRecentlyViewedResponse is not None
    assert BatchDeleteNotebooksResponse is not None


def test_client_methods_exist() -> None:
    """Test that NblmClient has the expected methods"""

    # Verify method signatures exist (without calling them)
    assert hasattr(NblmClient, "create_notebook")
    assert hasattr(NblmClient, "list_recently_viewed")
    assert hasattr(NblmClient, "delete_notebooks")
    assert hasattr(NblmClient, "add_sources")
    assert hasattr(NblmClient, "delete_sources")


def test_token_provider_types() -> None:
    """Test that token provider types can be imported"""

    assert GcloudTokenProvider is not None
    assert EnvTokenProvider is not None
