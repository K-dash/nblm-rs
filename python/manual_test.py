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
    AudioOverviewRequest,
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
    source_ids = []
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
            # Extract source ID from the source name (format: .../sources/SOURCE_ID)
            if source.name and "/sources/" in source.name:
                source_id = source.name.split("/sources/")[-1]
                source_ids.append(source_id)
        print()
    except NblmError as e:
        print(f"✗ Failed to add sources: {e}\n")

    # Test 4: Get source by ID
    print("Test 4: Getting source details...")
    if source_ids:
        # Get the first source that was added
        test_source_id = source_ids[0]
        try:
            source = client.get_source(
                notebook_id=notebook.notebook_id,
                source_id=test_source_id
            )
            print(f"✓ Retrieved source: {source.name}")
            print(f"  Title: {source.title}")
            if source.source_id and source.source_id.id:
                print(f"  Source ID: {source.source_id.id}")
            if source.metadata:
                if source.metadata.source_added_timestamp:
                    print(f"  Added: {source.metadata.source_added_timestamp}")
                if source.metadata.word_count:
                    print(f"  Word Count: {source.metadata.word_count}")
                if source.metadata.youtube_metadata:
                    youtube_meta = source.metadata.youtube_metadata
                    if youtube_meta.channel_name:
                        print(f"  YouTube Channel: {youtube_meta.channel_name}")
                    if youtube_meta.video_id:
                        print(f"  YouTube Video ID: {youtube_meta.video_id}")
            if source.settings and source.settings.status:
                print(f"  Status: {source.settings.status}")
            print()
        except NblmError as e:
            print(f"✗ Failed to get source: {e}\n")
    else:
        print("  No sources available to get details\n")

    # Test 5: Create audio overview
    print("Test 5: Creating audio overview...")
    try:
        # Create audio overview with empty request (current API requirement)
        audio_response = client.create_audio_overview(
            notebook_id=notebook.notebook_id,
            request=AudioOverviewRequest()
        )
        print(f"✓ Audio overview created:")
        
        # Debug information
        print(f"  [DEBUG] audio_overview_id: {audio_response.audio_overview_id}")
        print(f"  [DEBUG] name: {audio_response.name}")
        print(f"  [DEBUG] status: {audio_response.status}")
        print(f"  [DEBUG] generation_options: {audio_response.generation_options}")
        print(f"  [DEBUG] extra keys: {list(audio_response.extra.keys()) if hasattr(audio_response.extra, 'keys') else 'N/A'}")
        print(f"  [DEBUG] full repr: {repr(audio_response)}")
        
        # Original display
        if audio_response.audio_overview_id:
            print(f"  Audio Overview ID: {audio_response.audio_overview_id}")
        if audio_response.name:
            print(f"  Name: {audio_response.name}")
        if audio_response.status:
            print(f"  Status: {audio_response.status}")
        print()
        
        # Test deleting the audio overview
        print("Test 5b: Deleting audio overview...")
        try:
            client.delete_audio_overview(notebook_id=notebook.notebook_id)
            print("✓ Audio overview deleted successfully\n")
        except NblmError as e:
            print(f"✗ Failed to delete audio overview: {e}\n")
    except NblmError as e:
        print(f"✗ Failed to create audio overview: {e}\n")

    # Test 6: Upload a local file as a source (optional)
    upload_path = os.getenv("NBLM_UPLOAD_FILE")
    if upload_path:
        print("Test 5: Uploading file to the notebook...")
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
            "Test 6 skipped: set NBLM_UPLOAD_FILE to exercise upload_source_file manually.\n"
        )

    # # Test 7: Delete the created notebook
    # print("Test 7: Deleting the test notebook...")
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

    print("All tests completed!")


if __name__ == "__main__":
    main()
