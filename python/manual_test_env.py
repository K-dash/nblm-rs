#!/usr/bin/env python3
"""
Manual test script for nblm Python bindings using EnvTokenProvider

This script demonstrates authentication using an environment variable token.
Run with: python manual_test_env.py

Prerequisites:
    export NBLM_ACCESS_TOKEN=$(gcloud auth print-access-token)
    export NBLM_PROJECT_NUMBER="your_project_number"
"""

import os
import sys
from nblm import NblmClient, EnvTokenProvider, NblmError


def main() -> None:
    # Get configuration from environment
    access_token = os.getenv("NBLM_ACCESS_TOKEN")
    if not access_token:
        print("Error: NBLM_ACCESS_TOKEN environment variable not set")
        print("Run: export NBLM_ACCESS_TOKEN=$(gcloud auth print-access-token)")
        sys.exit(1)

    project_number = os.getenv("NBLM_PROJECT_NUMBER")
    if not project_number:
        print("Error: NBLM_PROJECT_NUMBER environment variable not set")
        sys.exit(1)

    location = os.getenv("NBLM_LOCATION", "global")
    endpoint_location = os.getenv("NBLM_ENDPOINT_LOCATION", "global")

    print("=== EnvTokenProvider Test ===")
    print(f"Project Number: {project_number}")
    print(f"Location: {location}")
    print(f"Endpoint Location: {endpoint_location}")
    print(f"Token: {access_token[:20]}...\n")

    # Initialize client with environment token provider
    try:
        token_provider = EnvTokenProvider("NBLM_ACCESS_TOKEN")
        client = NblmClient(
            project_number=project_number,
            location=location,
            endpoint_location=endpoint_location,
            token_provider=token_provider,
        )
        print("✓ Client initialized with EnvTokenProvider\n")
    except NblmError as e:
        print(f"✗ Failed to initialize client: {e}")
        sys.exit(1)

    # Test: Create a notebook
    print("Test: Creating a notebook...")
    try:
        notebook = client.create_notebook(title="EnvToken Test Notebook")
        print(f"✓ Notebook created: {notebook.name}")
        print(f"  Title: {notebook.title}\n")
    except NblmError as e:
        print(f"✗ Failed to create notebook: {e}\n")
        sys.exit(1)

    # Test: List recently viewed notebooks
    print("Test: Listing recently viewed notebooks...")
    try:
        response = client.list_recently_viewed()
        print(f"✓ Found {len(response.notebooks)} notebook(s)")
        for nb in response.notebooks[:3]:  # Show first 3
            print(f"  - {nb.title} ({nb.name})")
        print()
    except NblmError as e:
        print(f"✗ Failed to list notebooks: {e}\n")

    # Test: Delete the created notebook
    print("Test: Deleting the test notebook...")
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

    print("All tests completed with EnvTokenProvider!")


if __name__ == "__main__":
    main()
