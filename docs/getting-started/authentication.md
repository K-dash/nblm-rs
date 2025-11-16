# Authentication Guide

This guide covers all authentication methods supported by nblm-rs for accessing the NotebookLM Enterprise API.

## Overview

nblm-rs supports three authentication methods:

| Method | Use Case | Operations Support | Recommended |
|--------|----------|-------------------|-------------|
| gcloud CLI | Local development, interactive use | Full (read + write) | ✓ Yes |
| Environment Variable | CI/CD, automation, production | Full (read + write) | For automation |
| OAuth2 (CLI + Python) | End-user scenarios requiring OAuth consent | CLI: Full, Python: Read-only | For NotebookLM users |

## Method 1: gcloud CLI (Recommended)

Uses your Google Cloud user account credentials via the gcloud CLI.

### Prerequisites

```bash
# Install gcloud CLI
# See: https://cloud.google.com/sdk/docs/install

# Authenticate
gcloud auth login

# Set project (optional)
gcloud config set project YOUR_PROJECT_ID
```

### CLI Usage

```bash
# Using gcloud authentication (default)
nblm --project-number PROJECT_NUMBER \
  --location global \
  --endpoint-location global \
  notebooks recent

# Using gcloud authentication
#  (when you specify --auth gcloud, the CLI automatically runs `gcloud auth print-access-token` internally to obtain the access token for authentication)
nblm --auth gcloud \
  --project-number PROJECT_NUMBER \
  --location global \
  --endpoint-location global \
  notebooks recent

```

### Python Usage

```python
from nblm import NblmClient, GcloudTokenProvider

# Initialize with gcloud auth
token_provider = GcloudTokenProvider()
client = NblmClient(
    project_number="YOUR_PROJECT_NUMBER",
    location="global",
    endpoint_location="global",
    token_provider=token_provider,
)

# Use the client
notebook = client.create_notebook(title="My Notebook")
print(f"Created: {notebook.name}")
```

### Custom gcloud Binary Path

```python
# If gcloud is not in PATH
token_provider = GcloudTokenProvider("/custom/path/to/gcloud")
```

### Pros & Cons

<div class="grid cards" markdown>

-   :material-thumb-up-outline: **Pros**

    - Easy setup for developers
    - Uses existing gcloud credentials
    - Full API access (read + write)
    - Automatic token refresh

-   :material-thumb-down-outline: **Cons**

    - Requires gcloud CLI installed
    - Interactive login needed initially
    - Not suitable for unattended automation

</div>

## Method 2: Environment Variable

Uses an access token from an environment variable. Suitable for CI/CD pipelines.

### Setup

```bash
# Get access token from your authenticated account
export NBLM_ACCESS_TOKEN=$(gcloud auth print-access-token)

# Or use any other source of valid access token
export NBLM_ACCESS_TOKEN="ya29...."
```

!!! note "Token lifetime"
    Tokens acquired via `gcloud auth print-access-token` — and any environment variables that reuse them — expire after roughly one hour. Refresh them regularly and, for unattended workloads, favor tokens issued from a dedicated service account or Workload Identity Federation with automatic rotation.

### CLI Usage

```bash
nblm --auth env \
  --project-number PROJECT_NUMBER \
  notebooks recent
```

### Python Usage

```python
from nblm import NblmClient, EnvTokenProvider

# Initialize with environment variable
token_provider = EnvTokenProvider("NBLM_ACCESS_TOKEN")
client = NblmClient(
    project_number="YOUR_PROJECT_NUMBER",
    location="global",
    endpoint_location="global",
    token_provider=token_provider,
)

notebook = client.create_notebook(title="My Notebook")
```

### Custom Environment Variable Name

```python
# Use different variable name
export MY_CUSTOM_TOKEN=$(gcloud auth print-access-token)

token_provider = EnvTokenProvider("MY_CUSTOM_TOKEN")
```

!!! note "Drive-specific requirement"
    If you intend to ingest Google Drive sources, the token must include the `https://www.googleapis.com/auth/drive.file` (or broader `drive`) scope. The CLI and SDK validate this scope before uploading Drive documents.

### Pros & Cons

<div class="grid cards" markdown>

-   :material-thumb-up-outline: **Pros**

    - No gcloud CLI required at runtime
    - Works in containerized environments
    - Suitable for CI/CD pipelines
    - Full API access (read + write)

-   :material-thumb-down-outline: **Cons**

    - Tokens expire after 1 hour
    - Manual token refresh needed
    - Token must be obtained from authenticated source

</div>

## Method 3: OAuth2 (User Authentication)

Use this when you need to act as a specific Google user. The CLI completes the OAuth2 browser flow and stores a refresh token in `~/.config/nblm-rs/credentials.json`. Python can then reuse that token in a read-only fashion.

!!! warning "Experimental feature"
    OAuth2 support is still considered experimental. Set `NBLM_PROFILE_EXPERIMENT=1` before using `--auth user-oauth`, and be prepared for breaking changes while the implementation stabilizes.

### Prerequisites

1. In Google Cloud Console, create an OAuth 2.0 **Desktop application** client ID for your project.
2. Copy both the **client ID** and the generated **client secret**; they must be provided via `NBLM_OAUTH_CLIENT_ID` and `NBLM_OAUTH_CLIENT_SECRET`.
3. Ensure the redirect URI `http://127.0.0.1:4317` is allowed (add it if necessary).

### Setup

```bash
export NBLM_OAUTH_CLIENT_ID="YOUR_CLIENT_ID"
nblm --auth user-oauth --project-number PROJECT_NUMBER notebooks list
```

### Python Usage

```python
from nblm import NblmClient

client = NblmClient.with_user_oauth(
    project_number=PROJECT_NUMBER,
    location="global",
)
```

If you need direct access to the provider (for dependency injection or advanced scenarios), instantiate it explicitly:

```python
from nblm import UserOAuthProvider

provider = UserOAuthProvider.from_file(project_number=PROJECT_NUMBER)
```

!!! note "Python scope"
    The Python SDK currently supports read-only access via OAuth2. All write operations still require the CLI.


## Configuration

### Environment Variables

All methods can use these environment variables to avoid repeating parameters:

```bash
export NBLM_PROJECT_NUMBER="123456789012"
export NBLM_LOCATION="global"
export NBLM_ENDPOINT_LOCATION="global"
```

Then:
```bash
# No need to specify --project-number, --location, --endpoint-location
nblm notebooks recent
```

### Location Options

NotebookLM API supports these multi-region locations:
- `global` - **Recommended** by Google for best performance
- `us` - United States (for compliance requirements)
- `eu` - European Union (for compliance requirements)

!!! important "Location consistency"
    `NBLM_LOCATION` and `NBLM_ENDPOINT_LOCATION` must always be set to the same value.

## Troubleshooting

### "gcloud command not found"

**Solution**: Install gcloud CLI or use a different authentication method.

### "Failed to get access token from gcloud"

**Solution**: Run `gcloud auth login` to authenticate.

### "Token expired" error

**Solution**:
- For gcloud: Authentication is automatic
- For env token: Re-run `export NBLM_ACCESS_TOKEN=$(gcloud auth print-access-token)`

## Recommendation Summary

| Scenario | Recommended Method |
|----------|-------------------|
| Local development | gcloud CLI |
| CI/CD pipelines | Environment Variable |
| Production automation | Environment Variable |
| Server applications | Environment Variable |
