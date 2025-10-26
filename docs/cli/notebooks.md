# Notebooks Commands

Manage NotebookLM notebooks from the command line.

## Available Commands

| Command  | Description                    |
| -------- | ------------------------------ |
| `create` | Create a new notebook          |
| `recent` | List recently viewed notebooks |
| `delete` | Delete one or more notebooks   |

## create

Create a new notebook with a title.

### Usage

```bash
nblm notebooks create --title <TITLE>
```

### Options

| Option            | Description    | Required |
| ----------------- | -------------- | -------- |
| `--title <TITLE>` | Notebook title | Yes      |

### Examples

**Basic usage:**

```bash
nblm notebooks create --title "My Research Notebook"
```

**JSON output:**

```bash
nblm --json notebooks create --title "Project Documentation"
```

Output:

```json
{
  "title": "Project Documentation",
  "notebookId": "abc123def456",
  "name": "projects/123456789012/locations/global/notebooks/abc123def456",
  "createTime": "2025-10-25T10:30:00Z",
  "updateTime": "2025-10-25T10:30:00Z"
}
```

### Notes

- The notebook is created in your Google Cloud project
- The `notebookId` is needed for subsequent operations (adding sources, etc.)
- Newly created notebooks are empty and have no sources

## recent

List recently viewed notebooks.

### Usage

```bash
nblm notebooks recent [--page-size <SIZE>]
```

### Options

| Option               | Description                                   | Required | Default |
| -------------------- | --------------------------------------------- | -------- | ------- |
| `--page-size <SIZE>` | Maximum number of notebooks to return (1-500) | No       | 500     |

### Examples

**List all recent notebooks:**

```bash
nblm notebooks recent
```

**Limit results:**

```bash
nblm notebooks recent --page-size 10
```

**JSON output:**

```bash
nblm --json notebooks recent
```

Output:

```json
{
  "notebooks": [
    {
      "title": "My Research Notebook",
      "notebookId": "abc123",
      "name": "projects/123456789012/locations/global/notebooks/abc123",
      "createTime": "2025-10-20T09:00:00Z",
      "updateTime": "2025-10-25T15:30:00Z"
    },
    {
      "title": "Project Documentation",
      "notebookId": "def456",
      "name": "projects/123456789012/locations/global/notebooks/def456",
      "createTime": "2025-10-25T10:30:00Z",
      "updateTime": "2025-10-25T10:30:00Z"
    }
  ]
}
```

**Extract specific fields with jq:**

```bash
# Get all notebook titles
nblm --json notebooks recent | jq '.notebooks[].title'

# Get all notebook IDs
nblm --json notebooks recent | jq '.notebooks[].notebookId'

# Get the most recently updated notebook
nblm --json notebooks recent | jq '.notebooks[0]'
```

### Notes

- Notebooks are sorted by most recently updated first
- The `--page-size` option is accepted but pagination is not implemented by the API (as of 2025-10-25)
- All notebooks are returned regardless of `--page-size` value

## delete

Delete one or more notebooks.

### Usage

```bash
nblm notebooks delete --notebook-name <NAME> [--notebook-name <NAME>...]
```

### Options

| Option                   | Description                                   | Required |
| ------------------------ | --------------------------------------------- | -------- |
| `--notebook-name <NAME>` | Full notebook resource name (can be repeated) | Yes      |

### Examples

**Delete a single notebook:**

```bash
nblm notebooks delete \
  --notebook-name "projects/123456789012/locations/global/notebooks/abc123"
```

**Delete multiple notebooks:**

```bash
nblm notebooks delete \
  --notebook-name "projects/123456789012/locations/global/notebooks/abc123" \
  --notebook-name "projects/123456789012/locations/global/notebooks/def456"
```

**Get notebook name from recent list:**

```bash
# Get the full notebook name
NOTEBOOK_NAME=$(nblm --json notebooks recent | jq -r '.notebooks[0].name')

# Delete it
nblm notebooks delete --notebook-name "$NOTEBOOK_NAME"
```

### Notes

- **Important**: Despite the API being named "batchDelete", it only accepts one notebook at a time (as of 2025-10-25)
- The CLI automatically handles this limitation by calling the API sequentially for each notebook
- Deletion is permanent and cannot be undone
- All sources and content within the notebook are also deleted

## Common Patterns

### Create and save notebook ID

```bash
# Create notebook and extract ID
NOTEBOOK_ID=$(nblm --json notebooks create --title "My Notebook" | jq -r '.notebookId')

echo "Created notebook: $NOTEBOOK_ID"

# Use the ID in subsequent commands
nblm sources add --notebook-id "$NOTEBOOK_ID" --web-url "https://example.com"
```

### List and filter notebooks

```bash
# Find notebooks by title
nblm --json notebooks recent | jq '.notebooks[] | select(.title | contains("Research"))'

# Count notebooks
nblm --json notebooks recent | jq '.notebooks | length'

# Get notebooks created today
TODAY=$(date +%Y-%m-%d)
nblm --json notebooks recent | jq ".notebooks[] | select(.createTime | startswith(\"$TODAY\"))"
```

### Delete all notebooks (dangerous)

```bash
# WARNING: This deletes ALL notebooks!
nblm --json notebooks recent | \
  jq -r '.notebooks[].name' | \
  xargs -I {} nblm notebooks delete --notebook-name {}
```

## Error Handling

### Common Errors

**Notebook not found:**

```
Error: Notebook not found
Cause: The specified notebook does not exist or has been deleted
```

**Permission denied:**

```
Error: Permission denied
Cause: Your account does not have access to this notebook or project
```

**Invalid notebook name format:**

```
Error: Invalid notebook name format
Cause: Notebook name must be in format: projects/PROJECT_NUMBER/locations/LOCATION/notebooks/NOTEBOOK_ID
```

## Next Steps

- [Sources Commands](sources.md) - Add content to notebooks
- [Audio Commands](audio.md) - Create audio overviews
- [Share Commands](share.md) - Share notebooks with others
- [Advanced Usage](advanced.md) - Scripting and automation
