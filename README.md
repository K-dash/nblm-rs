# nblm-rs

Unofficial command-line interface for NotebookLM Enterprise API written in Rust.

> [!WARNING]
> This is an unofficial tool and is not affiliated with or endorsed by Google. Use at your own risk.

## Features

| Feature | Command | Status | Notes |
|---------|---------|--------|-------|
| **Notebooks** | | | |
| Create notebook | `notebooks create` | Working | |
| List recent notebooks | `notebooks recent` | Working | Pagination not implemented by API |
| Delete notebook(s) | `notebooks delete` | Working | Sequential deletion (API limitation) |
| **Sources** | | | |
| Add web URL | `sources add --web-url` | Working | |
| Add text content | `sources add --text` | Working | |
| Add video (YouTube) | `sources add --video-url` | Working | Uses `youtubeUrl` field |
| Add Google Drive | `sources add --drive-*` | Not Working | API returns HTTP 500 |
| Delete source(s) | `sources delete` | Working | |
| **Audio Overview** | | | |
| Create audio overview | `audio create` | Working | Config fields not supported |
| Delete audio overview | `audio delete` | Working | |
| **Sharing** | | | |
| Share notebook | `share add` | Untested | Requires additional users |

## Installation

### Prerequisites

- Rust 1.90.0 or later
- Google Cloud project with NotebookLM API enabled
- Authentication credentials (gcloud, service account, or access token)

### Build from source

```bash
git clone https://github.com/yourusername/nblm-rs.git
cd nblm-rs
cargo build --release
```

The binary will be available at `target/release/nblm`.

## Authentication

### Method 1: gcloud CLI

```bash
gcloud auth login
gcloud config set project YOUR_PROJECT_ID

nblm --auth gcloud \
  --project-number PROJECT_NUMBER \
  --location global \
  --endpoint-location us \
  notebooks recent
```

### Method 2: Service Account

```bash
# Create service account key
gcloud iam service-accounts keys create ~/sa-key.json \
  --iam-account=your-sa@project.iam.gserviceaccount.com

# Use with environment variable
export GOOGLE_APPLICATION_CREDENTIALS="$HOME/sa-key.json"
nblm --auth sa \
  --project-number PROJECT_NUMBER \
  notebooks recent

# Or specify key file directly
nblm --auth sa \
  --sa-key ~/sa-key.json \
  --project-number PROJECT_NUMBER \
  notebooks recent
```

> [!IMPORTANT]
> Service account requires `roles/editor` permission.

### Method 3: Environment Variable Token

```bash
export NBLM_ACCESS_TOKEN=$(gcloud auth print-access-token)
nblm --auth env \
  --project-number PROJECT_NUMBER \
  notebooks recent
```

## Configuration

You can use environment variables to avoid repeating common options:

```bash
export NBLM_PROJECT_NUMBER="123456789"
export NBLM_LOCATION="global"
export NBLM_ENDPOINT_LOCATION="us"

# Now you can run commands without these flags
nblm notebooks recent
```

## Usage Examples

### Notebooks

```bash
# Create a new notebook
nblm notebooks create --title "My Research Notebook"

# List recently viewed notebooks
nblm notebooks recent

# List with pagination (note: API doesn't implement pagination yet)
nblm notebooks recent --page-size 10

# Delete a notebook
nblm notebooks delete \
  --notebook-name "projects/PROJECT_NUMBER/locations/global/notebooks/NOTEBOOK_ID"

# Delete multiple notebooks (executed sequentially due to API limitation)
nblm notebooks delete \
  --notebook-name "projects/.../notebooks/ID1" \
  --notebook-name "projects/.../notebooks/ID2"
```

### Sources

```bash
# Add web URL
nblm sources add \
  --notebook-id NOTEBOOK_ID \
  --web-url "https://example.com" \
  --web-name "Example Website"

# Add text content
nblm sources add \
  --notebook-id NOTEBOOK_ID \
  --text "Your text content here" \
  --text-name "My Notes"

# Add YouTube video
nblm sources add \
  --notebook-id NOTEBOOK_ID \
  --video-url "https://www.youtube.com/watch?v=VIDEO_ID"

# Add multiple sources at once
nblm sources add \
  --notebook-id NOTEBOOK_ID \
  --web-url "https://docs.python.org" \
  --web-name "Python Docs" \
  --text "Sample text" \
  --text-name "Notes"

# Delete sources
nblm sources delete \
  --notebook-id NOTEBOOK_ID \
  --source-name "projects/.../notebooks/NB_ID/sources/SOURCE_ID"
```

> [!WARNING]
> Google Drive source addition currently returns HTTP 500 Internal Server Error. This is an API-side issue as of 2025-10-19.

### Audio Overview

```bash
# Create audio overview
nblm audio create --notebook-id NOTEBOOK_ID

# Delete audio overview
nblm audio delete --notebook-id NOTEBOOK_ID
```

> [!NOTE]
> Despite API documentation mentioning `languageCode`, `sourceIds`, and `episodeFocus` fields, the API only accepts empty request body as of 2025-10-19. Language and other settings must be configured through the NotebookLM UI.

### Sharing

```bash
# Share notebook with a user (untested)
nblm share add \
  --notebook-id NOTEBOOK_ID \
  --email user@example.com \
  --role reader

# Share with multiple users
nblm share add \
  --notebook-id NOTEBOOK_ID \
  --email user1@example.com \
  --email user2@example.com \
  --role writer
```

Available roles: `owner`, `writer`, `reader`, `not-shared`

### JSON Output

All commands support `--json` flag for machine-readable output. The flag can be placed anywhere in the command:

```bash
# All of these work
nblm --json notebooks recent
nblm notebooks --json recent
nblm notebooks recent --json

# Parse with jq
nblm --json notebooks recent | jq '.notebooks[].title'
```

## Known API Issues

> [!WARNING]
> The following issues have been discovered through testing and are documented in the code:

### Google Drive Sources (HTTP 500)

As of 2025-10-19, adding Google Drive sources returns HTTP 500 Internal Server Error. This occurs even with proper authentication (`gcloud auth login --enable-gdrive-access`) and correct IAM permissions. The CLI includes warnings when attempting to use this feature.

### Audio Overview Configuration Fields

The API documentation mentions `languageCode`, `sourceIds`, and `episodeFocus` fields, but the actual API rejects all of these fields with "Unknown name" errors. Only empty request body `{}` is accepted. These fields are commented out in the code for future use when the API implements them.

### Notebook Batch Deletion

Despite the API endpoint being named `batchDelete` and accepting an array of notebook names, it only works with a single notebook at a time. The CLI works around this by calling the API sequentially for each notebook.

### Pagination Not Implemented

The `notebooks recent` command accepts `--page-size` and `--page-token` parameters, but the NotebookLM API never returns `nextPageToken` in responses, indicating pagination is not currently implemented.

All of these have been corrected in the implementation.

## Development

### Running Tests

```bash
# Run all tests (unit + integration)
cargo test --all

# Run only unit tests
cargo test --lib

# Run only integration tests
cargo test --test '*'

# Run specific test file
cargo test --test notebooks_delete
```

All 60 tests should pass:
- 29 unit tests
- 31 integration tests

## Contributing

Contributions are welcome! Please note:

1. All tests must pass before submitting PR
2. Add integration tests for new features
3. Document any new API issues discovered
4. Follow existing code style

## License

MIT

## Acknowledgments

This project uses the NotebookLM Enterprise API. See the [official documentation](https://cloud.google.com/gemini/enterprise/notebooklm-enterprise/docs) for more information.

## Related Resources

- [NotebookLM API Documentation](https://cloud.google.com/gemini/enterprise/notebooklm-enterprise/docs)
- [NotebookLM API - Notebooks](https://cloud.google.com/gemini/enterprise/notebooklm-enterprise/docs/api-notebooks)
- [NotebookLM API - Sources](https://cloud.google.com/gemini/enterprise/notebooklm-enterprise/docs/api-notebooks-sources)
- [NotebookLM API - Audio Overview](https://cloud.google.com/gemini/enterprise/notebooklm-enterprise/docs/api-audio-overview)
