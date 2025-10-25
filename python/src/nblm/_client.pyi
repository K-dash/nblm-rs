"""NblmClient for NotebookLM API operations"""

import os

from ._auth import TokenProvider
from ._models import (
    BatchCreateSourcesResponse,
    BatchDeleteNotebooksResponse,
    BatchDeleteSourcesResponse,
    ListRecentlyViewedResponse,
    Notebook,
    TextSource,
    UploadSourceFileResponse,
    VideoSource,
    WebSource,
)

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

    def create_notebook(self, title: str) -> Notebook:
        """
        Create a new notebook with the given title

        Args:
            title: The title of the notebook

        Returns:
            Notebook: The created notebook

        Raises:
            NblmError: If the notebook creation fails
        """

    def list_recently_viewed(self, page_size: int | None = None) -> ListRecentlyViewedResponse:
        """
        List recently viewed notebooks

        Args:
            page_size: Maximum number of notebooks to return (1-500, default: 500)

        Returns:
            ListRecentlyViewedResponse: Response containing notebooks list

        Raises:
            NblmError: If the request fails
        """

    def delete_notebooks(self, notebook_names: list[str]) -> BatchDeleteNotebooksResponse:
        """
        Delete one or more notebooks

        Args:
            notebook_names: List of full notebook resource names to delete

        Returns:
            BatchDeleteNotebooksResponse: Response (typically empty)

        Raises:
            NblmError: If deletion fails

        Note:
            Despite the underlying API being named "batchDelete", it only accepts
            one notebook at a time (as of 2025-10-19). This method works around
            this limitation by calling the API sequentially for each notebook.
        """

    def add_sources(
        self,
        notebook_id: str,
        web_sources: list[WebSource] | None = ...,
        text_sources: list[TextSource] | None = ...,
        video_sources: list[VideoSource] | None = ...,
    ) -> BatchCreateSourcesResponse:
        """
        Add sources to a notebook.

        Args:
            notebook_id: Notebook identifier (notebook resource ID, e.g. "abc123")
            web_sources: Optional list of WebSource objects
            text_sources: Optional list of TextSource objects
            video_sources: Optional list of VideoSource objects

        Returns:
            BatchCreateSourcesResponse: Results for each processed source

        Raises:
            NblmError: If validation fails or the API request fails
        """

    def upload_source_file(
        self,
        notebook_id: str,
        path: str | os.PathLike[str],
        *,
        content_type: str | None = ...,
        display_name: str | None = ...,
    ) -> UploadSourceFileResponse:
        """
        Upload a local file as a notebook source.

        Args:
            notebook_id: Notebook identifier (resource ID, e.g. "abc123")
            path: Path to the file to upload
            content_type: Optional HTTP Content-Type header value
            display_name: Optional display name to attach to the source
                (NotebookLM currently rejects custom names; kept for future use)

        Returns:
            UploadSourceFileResponse: Response containing the created source ID

        Raises:
            NblmError: If validation fails or the API request fails
        """

    def delete_sources(
        self,
        notebook_id: str,
        source_names: list[str],
    ) -> BatchDeleteSourcesResponse:
        """
        Delete sources from a notebook.

        Args:
            notebook_id: Notebook identifier (notebook resource ID, e.g. "abc123")
            source_names: Fully qualified source resource names

        Returns:
            BatchDeleteSourcesResponse: API response payload (typically empty)

        Raises:
            NblmError: If the API request fails
        """
