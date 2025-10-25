#!/usr/bin/env python3
"""
Manual test script for nblm Python bindings

This script demonstrates notebook creation, retrieval, and deletion.
Run with: python manual_test.py
"""

import os
import sys
from nblm import NblmClient, GcloudTokenProvider, NblmError


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

    # Test 3: Delete the created notebook
    print("Test 3: Deleting the test notebook...")
    try:
        response = client.delete_notebooks([notebook.name])
        print(f"✓ Deleted {len(response.deleted_notebooks)} notebook(s)")
        for name in response.deleted_notebooks:
            print(f"  - {name}")
        if response.failed_notebooks:
            print(f"  Failed: {response.failed_notebooks}")
        print()
    except NblmError as e:
        print(f"✗ Failed to delete notebook: {e}\n")

    print("All tests completed!")


if __name__ == "__main__":
    main()
