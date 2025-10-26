# Authentication Guide

This guide covers all authentication methods supported by nblm-rs for accessing the NotebookLM Enterprise API.

## Overview

nblm-rs supports two authentication methods:

| Method | Use Case | Operations Support | Recommended |
|--------|----------|-------------------|-------------|
| gcloud CLI | Local development, interactive use | Full (read + write) | ✓ Yes |
| Environment Variable | CI/CD, automation, production | Full (read + write) | For automation |

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

**Pros**:
- ✓ Easy setup for developers
- ✓ Uses existing gcloud credentials
- ✓ Full API access (read + write)
- ✓ Automatic token refresh

**Cons**:
- ✗ Requires gcloud CLI installed
- ✗ Interactive login needed initially
- ✗ Not suitable for unattended automation

## Method 2: Environment Variable

Uses an access token from an environment variable. Suitable for CI/CD pipelines.

### Setup

```bash
# Get access token from your authenticated account
export NBLM_ACCESS_TOKEN=$(gcloud auth print-access-token)

# Or use any other source of valid access token
export NBLM_ACCESS_TOKEN="ya29...."
```

> **Note**: Tokens expire after 1 hour. You'll need to refresh them periodically.

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

### Pros & Cons

**Pros**:
- ✓ No gcloud CLI required at runtime
- ✓ Works in containerized environments
- ✓ Suitable for CI/CD pipelines
- ✓ Full API access (read + write)

**Cons**:
- ✗ Tokens expire after 1 hour
- ✗ Manual token refresh needed
- ✗ Token must be obtained from authenticated source

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

> **Important**: `NBLM_LOCATION` and `NBLM_ENDPOINT_LOCATION` must be set to the same value.

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
