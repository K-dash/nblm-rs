"""NblmClient for NotebookLM API operations"""

import os

from ._auth import TokenProvider
from ._models import (
    AudioOverviewRequest,
    AudioOverviewResponse,
    BatchCreateSourcesResponse,
    BatchDeleteNotebooksResponse,
    BatchDeleteSourcesResponse,
    GoogleDriveSource,
    ListRecentlyViewedResponse,
    Notebook,
    NotebookSource,
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
        project_number: str | None = ...,
        location: str = "global",
        endpoint_location: str = "global",
        profile: str = "enterprise",
    ) -> None:
        """
        Create a new NblmClient

        Args:
            token_provider: Token provider for authentication
            project_number: Google Cloud project number (required for the enterprise profile)
            location: NotebookLM location (default: "global")
            endpoint_location: API endpoint location (default: "global")
            profile: API profile selection (experimental, default: "enterprise")

        Raises:
            NblmError: If the client cannot be created
        """

    @staticmethod
    def with_user_oauth(
        project_number: int | None = ...,
        location: str = "global",
        user: str | None = ...,
        endpoint_location: str | None = ...,
    ) -> NblmClient:
        """
        Create a client that reuses refresh tokens produced by `nblm --auth user-oauth`.

        Args:
            project_number: Project number used when bootstrapping the token
            location: Notebook location (e.g., "us-central1")
            user: Optional user hint when multiple credentials exist
            endpoint_location: Override the API endpoint location (defaults to "global")

        Returns:
            NblmClient: Configured client that uses the stored refresh token
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
        drive_sources: list[GoogleDriveSource] | None = ...,
        video_sources: list[VideoSource] | None = ...,
    ) -> BatchCreateSourcesResponse:
        """
        Add sources to a notebook.

        Args:
            notebook_id: Notebook identifier (notebook resource ID, e.g. "abc123")
            web_sources: Optional list of WebSource objects
            text_sources: Optional list of TextSource objects
            drive_sources: Optional list of GoogleDriveSource objects
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

    def get_source(
        self,
        notebook_id: str,
        source_id: str,
    ) -> NotebookSource:
        """
        Get a single source by its ID.

        Args:
            notebook_id: Notebook identifier (notebook resource ID, e.g. "abc123")
            source_id: Source identifier to retrieve

        Returns:
            NotebookSource: The requested source information

        Raises:
            NblmError: If the request fails
        """

    def create_audio_overview(
        self,
        notebook_id: str,
        request: AudioOverviewRequest | None = None,
    ) -> AudioOverviewResponse:
        """
        Create an audio overview for a notebook.

        Creates an audio overview (podcast-style discussion) from the notebook's sources.

        Note: The current API only accepts an empty request. Configuration options
        like source selection and language are not yet supported via the API.

        Args:
            notebook_id: Notebook identifier (notebook resource ID, e.g. "abc123")
            request: Audio overview request (currently must be empty)

        Returns:
            AudioOverviewResponse: Response containing the audio overview name and status

        Raises:
            NblmError: If the request fails
        """

    def delete_audio_overview(self, notebook_id: str) -> None:
        """
        Delete the audio overview for a notebook.

        Args:
            notebook_id: Notebook identifier (notebook resource ID, e.g. "abc123")

        Returns:
            None

        Raises:
            NblmError: If the request fails
        """
