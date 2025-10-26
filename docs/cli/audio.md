# Audio Commands

Manage audio overviews (podcast-style discussions) for notebooks.

## Available Commands

| Command  | Description              |
| -------- | ------------------------ |
| `create` | Create an audio overview |
| `delete` | Delete an audio overview |

## create

Create an audio overview from a notebook's sources.

### Usage

```bash
nblm audio create --notebook-id <ID>
```

### Options

| Option               | Description         | Required |
| -------------------- | ------------------- | -------- |
| `--notebook-id <ID>` | Notebook identifier | Yes      |

### Examples

**Create audio overview:**

```bash
nblm audio create --notebook-id abc123
```

**JSON output:**

```bash
nblm --json audio create --notebook-id abc123
```

Output:

```json
{
  "audioOverviewId": "audio-abc123",
  "name": "projects/123456789012/locations/global/notebooks/abc123/audioOverviews/audio-abc123",
  "status": "PROCESSING"
}
```

### Notes

- The notebook must have at least one source before creating an audio overview
- Audio generation takes several minutes to complete
- Only one audio overview can exist per notebook
- **Configuration options are not supported**: Despite API documentation mentioning `languageCode`, `sourceIds`, and `episodeFocus` fields, the API only accepts an empty request body (as of 2025-10-25)
- Language and other settings must be configured through the NotebookLM web UI

### Processing Status

Audio overviews go through the following states:

1. **PROCESSING** - Audio is being generated
2. **COMPLETED** - Audio is ready
3. **FAILED** - Generation failed

To check the status, you would need to use the NotebookLM web UI or fetch the notebook details.

## delete

Delete the audio overview from a notebook.

### Usage

```bash
nblm audio delete --notebook-id <ID>
```

### Options

| Option               | Description         | Required |
| -------------------- | ------------------- | -------- |
| `--notebook-id <ID>` | Notebook identifier | Yes      |

### Examples

**Delete audio overview:**

```bash
nblm audio delete --notebook-id abc123
```

**JSON output:**

```bash
nblm --json audio delete --notebook-id abc123
```

Output:

```json
{}
```

### Notes

- Deletion is permanent and cannot be undone
- You can create a new audio overview after deletion
- The audio file itself is deleted, but the notebook and sources remain

## Common Patterns

### Create and wait for completion

```bash
# Create audio overview
nblm audio create --notebook-id abc123

echo "Audio overview created. Check status in NotebookLM web UI."
echo "Generation typically takes 3-5 minutes."

# Note: The CLI does not currently support polling for completion status
```

### Recreate audio overview

```bash
NOTEBOOK_ID="abc123"

# Delete existing audio overview
echo "Deleting existing audio overview..."
nblm audio delete --notebook-id "$NOTEBOOK_ID"

# Wait a moment
sleep 2

# Create new audio overview
echo "Creating new audio overview..."
nblm audio create --notebook-id "$NOTEBOOK_ID"

echo "Done. Check NotebookLM web UI for generation status."
```

### Batch create audio overviews

```bash
# Create audio overviews for multiple notebooks
NOTEBOOKS=("abc123" "def456" "ghi789")

for notebook_id in "${NOTEBOOKS[@]}"; do
  echo "Creating audio overview for notebook: $notebook_id"
  nblm audio create --notebook-id "$notebook_id"
done

echo "All audio overviews created. Check web UI for completion status."
```

## Error Handling

### Common Errors

**Notebook has no sources:**

```
Error: Failed to create audio overview
Cause: Notebook must have at least one source before creating audio overview
```

**Audio overview already exists:**

```
Error: Failed to create audio overview
Cause: Audio overview already exists for this notebook
Solution: Delete the existing audio overview first, then create a new one
```

**Notebook not found:**

```
Error: Notebook not found
Cause: The specified notebook does not exist or you don't have access
```

**Audio overview not found:**

```
Error: Audio overview not found
Cause: No audio overview exists for this notebook
```

## API Limitations

As of 2025-10-25, the audio overview API has the following limitations:

1. **No configuration options**: Cannot specify language, source selection, or episode focus via the API
2. **No status polling**: Cannot check generation status or progress via the CLI
3. **One per notebook**: Only one audio overview can exist per notebook
4. **No download**: Audio files cannot be downloaded via the API

These settings must be managed through the NotebookLM web UI.

## Workflow

The typical workflow for audio overviews:

1. **Create notebook** with `nblm notebooks create`
2. **Add sources** with `nblm sources add` or `nblm sources upload`
3. **Create audio overview** with `nblm audio create`
4. **Check status** in NotebookLM web UI
5. **Listen to audio** in NotebookLM web UI
6. **(Optional) Delete** with `nblm audio delete` if you want to regenerate

## Next Steps

- [Notebooks Commands](notebooks.md) - Create and manage notebooks
- [Sources Commands](sources.md) - Add content to notebooks
- [Advanced Usage](advanced.md) - Scripting and automation
