"""Data models for nblm"""

from typing import Any

class Notebook:
    """Represents a NotebookLM notebook"""

    name: str | None
    title: str
    notebook_id: str | None
    extra: dict[str, Any]

class ListRecentlyViewedResponse:
    """Response from listing recently viewed notebooks"""

    notebooks: list[dict[str, Any]]

class BatchDeleteNotebooksResponse:
    """Response from batch deleting notebooks"""

    extra: dict[str, Any]
