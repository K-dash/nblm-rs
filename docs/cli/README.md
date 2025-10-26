# CLI Overview

Command-line interface for the NotebookLM Enterprise API.

## Command Structure

```bash
nblm [GLOBAL_OPTIONS] <COMMAND> [COMMAND_OPTIONS]
```

## Global Options

Options that can be used with any command:

| Option                           | Description                                 | Required | Default  |
| -------------------------------- | ------------------------------------------- | -------- | -------- |
| `--auth <METHOD>`                | Authentication method: `gcloud` or `env`    | Yes      | -        |
| `--project-number <NUMBER>`      | Google Cloud project number                 | Yes\*    | From env |
| `--location <LOCATION>`          | API location: `global`, `us`, or `eu`       | No       | `global` |
| `--endpoint-location <LOCATION>` | Endpoint location (must match `--location`) | No       | `global` |
| `--json`                         | Output in JSON format                       | No       | false    |
| `--debug-http`                   | Print raw HTTP responses to stderr          | No       | false    |
| `-h, --help`                     | Print help information                      | No       | -        |
| `-V, --version`                  | Print version information                   | No       | -        |

\*Can be set via `NBLM_PROJECT_NUMBER` environment variable.

## Commands

| Command     | Description                | Documentation                |
| ----------- | -------------------------- | ---------------------------- |
| `notebooks` | Manage notebooks           | [notebooks.md](notebooks.md) |
| `sources`   | Manage notebook sources    | [sources.md](sources.md)     |
| `audio`     | Manage audio overviews     | [audio.md](audio.md)         |
| `share`     | Share notebooks with users | [share.md](share.md)         |

## Authentication

Two authentication methods are supported:

### gcloud CLI (Recommended)

```bash
gcloud auth login
nblm notebooks recent
```

### Environment Variable

```bash
export NBLM_ACCESS_TOKEN=$(gcloud auth print-access-token)
nblm --auth env notebooks recent
```

See [Authentication Guide](../getting-started/authentication.md) for details.

## Environment Variables

Reduce command verbosity by setting environment variables:

```bash
export NBLM_PROJECT_NUMBER="123456789012"
export NBLM_LOCATION="global"
export NBLM_ENDPOINT_LOCATION="global"

# Now you can omit these flags
nblm notebooks recent
```

### Raw HTTP Logging

Use the new `--debug-http` flag (or set `NBLM_DEBUG_HTTP=1`) to print the raw JSON payload returned by the API. Logged bodies may contain sensitive data, so enable this only on trusted machines.

## Output Formats

### Human-Readable (Default)

```bash
nblm notebooks recent
```

Output:

```
Title: My Notebook
Notebook ID: abc123
Updated: 2025-10-25T10:30:00Z
```

### JSON Format

```bash
nblm --json notebooks recent
```

Output:

```json
{
  "notebooks": [
    {
      "title": "My Notebook",
      "notebookId": "abc123",
      "updateTime": "2025-10-25T10:30:00Z"
    }
  ]
}
```

The `--json` flag can be placed anywhere in the command:

```bash
# All equivalent
nblm --json notebooks recent
nblm notebooks recent --json
```

## Error Handling

### Exit Codes

| Code | Description          |
| ---- | -------------------- |
| 0    | Success              |
| 1    | General error        |
| 2    | Authentication error |

### Automatic Retries

The CLI automatically retries transient failures (HTTP 429, 500, 502, 503, 504) with exponential backoff.

### Error Messages

Errors are printed to stderr in a human-readable format:

```bash
Error: Failed to create notebook
Cause: API returned 403 Forbidden
```

In JSON mode, errors are also in JSON format:

```json
{
  "error": "Failed to create notebook",
  "cause": "API returned 403 Forbidden"
}
```

## Getting Help

### General Help

```bash
nblm --help
```

### Command-Specific Help

```bash
nblm notebooks --help
nblm sources add --help
```

## Examples

### Quick Start

```bash
# Set up
export NBLM_PROJECT_NUMBER="123456789012"
gcloud auth login

# Create notebook
nblm notebooks create --title "My Notebook"

# List notebooks
nblm notebooks recent

# Add source
nblm sources add \
  --notebook-id abc123 \
  --web-url "https://example.com"
```

### JSON Output with jq

```bash
# Get all notebook titles
nblm --json notebooks recent | jq '.notebooks[].title'

# Get first notebook ID
nblm --json notebooks recent | jq -r '.notebooks[0].notebookId'

# Count notebooks
nblm --json notebooks recent | jq '.notebooks | length'
```

## Next Steps

- [Notebooks Commands](notebooks.md) - Notebook management
- [Sources Commands](sources.md) - Source management
- [Audio Commands](audio.md) - Audio overview operations
- [Advanced Usage](advanced.md) - Scripting and automation
