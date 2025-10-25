#!/usr/bin/env python3
"""
Manual test script for nblm Python bindings

This script demonstrates notebook creation, retrieval, and deletion.
Run with: python manual_test.py
"""

import os
import sys
from pathlib import Path

from nblm import (
    GcloudTokenProvider,
    NblmClient,
    NblmError,
    TextSource,
    VideoSource,
    WebSource,
)


def main() -> None:
    # Get project number from environment
    project_number = os.getenv("NBLM_PROJECT_NUMBER")
    if not project_number:
        print("Error: NBLM_PROJECT_NUMBER environment variable not set")
        sys.exit(1)

    location = os.getenv("NBLM_LOCATION", "global")
    endpoint_location = os.getenv("NBLM_ENDPOINT_LOCATION", "global")

    print(f"Project Number: {project_number}")
    print(f"Location: {location}")
    print(f"Endpoint Location: {endpoint_location}\n")

    # Initialize client with gcloud auth
    try:
        token_provider = GcloudTokenProvider()
        client = NblmClient(
            project_number=project_number,
            location=location,
            endpoint_location=endpoint_location,
            token_provider=token_provider,
        )
        print("✓ Client initialized\n")
    except NblmError as e:
        print(f"✗ Failed to initialize client: {e}")
        sys.exit(1)

    # Test 1: Create a notebook
    print("Test 1: Creating a notebook...")
    try:
        notebook = client.create_notebook(title="Python Test Notebook")
        print(f"✓ Notebook created: {notebook.name}")
        print(f"  Title: {notebook.title}\n")
    except NblmError as e:
        print(f"✗ Failed to create notebook: {e}\n")
        sys.exit(1)

    # Test 2: List recently viewed notebooks
    print("Test 2: Listing recently viewed notebooks...")
    try:
        response = client.list_recently_viewed()
        print(f"✓ Found {len(response.notebooks)} notebook(s)")
        for nb in response.notebooks[:3]:  # Show first 3
            print(f"  - {nb.title} ({nb.name})")
        print()
    except NblmError as e:
        print(f"✗ Failed to list notebooks: {e}\n")

    # Test 3: Add sources to the notebook
    print("Test 3: Adding sources to the notebook...")
    try:
        response = client.add_sources(
            notebook_id=notebook.notebook_id,
            web_sources=[
                WebSource(url="https://www.python.org/", name="Python Official Site"),
                WebSource(url="https://docs.python.org/"),
            ],
            text_sources=[
                TextSource(content="This is a test text content.", name="Test Note"),
            ],
            video_sources=[
                VideoSource(url="https://www.youtube.com/watch?v=dQw4w9WgXcQ"),
            ],
        )
        print(f"✓ Added sources (error_count: {response.error_count})")
        for source in response.sources:
            print(f"  - {source.name}")
        print()
    except NblmError as e:
        print(f"✗ Failed to add sources: {e}\n")

    # Test 4: Upload a local file as a source (optional)
    upload_path = os.getenv("NBLM_UPLOAD_FILE")
    if upload_path:
        print("Test 4: Uploading file to the notebook...")
        path_obj = Path(upload_path)
        if not path_obj.exists() or not path_obj.is_file():
            print(f"✗ Upload path is not a file: {path_obj}")
        else:
            content_type = os.getenv("NBLM_UPLOAD_CONTENT_TYPE")
            display_name = os.getenv("NBLM_UPLOAD_DISPLAY_NAME")
            try:
                response = client.upload_source_file(
                    notebook_id=notebook.notebook_id,
                    path=path_obj,
                    content_type=content_type,
                    display_name=display_name,
                )

                source_id_obj = response.source_id
                if source_id_obj and source_id_obj.id:
                    print(f"✓ Uploaded source ID: {source_id_obj.id}")
                else:
                    print("✓ Upload accepted (source ID unavailable)")

                extra = getattr(response, "extra", {})
                if isinstance(extra, dict):
                    print(f"  Extra metadata keys: {len(extra)}\n")
                else:
                    print("  Extra metadata: <unavailable>\n")
            except NblmError as e:
                print(f"✗ Failed to upload file: {e}\n")
                print(f"Error type: {type(e).__name__}")
                print(f"Error message: {e}")
                print(f"Error details: {e.args}")
                import traceback
                traceback.print_exc()
    else:
        print(
            "Test 4 skipped: set NBLM_UPLOAD_FILE to exercise upload_source_file manually.\n"
        )

    # # Test 4: Delete the created notebook
    # print("Test 4: Deleting the test notebook...")
    # try:
    #     response = client.delete_notebooks([notebook.name])
    #     print(f"✓ Deleted {len(response.deleted_notebooks)} notebook(s)")
    #     for name in response.deleted_notebooks:
    #         print(f"  - {name}")
    #     if response.failed_notebooks:
    #         print(f"  Failed: {response.failed_notebooks}")
    #     print()
    # except NblmError as e:
    #     print(f"✗ Failed to delete notebook: {e}\n")

    # print("All tests completed!")


if __name__ == "__main__":
    main()
