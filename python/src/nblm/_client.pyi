"""NblmClient for NotebookLM API operations"""

from ._auth import TokenProvider
from ._models import BatchDeleteNotebooksResponse, ListRecentlyViewedResponse, Notebook

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
