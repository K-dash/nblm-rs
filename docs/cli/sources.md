# Sources Commands

Manage sources (content) within NotebookLM notebooks.

## Available Commands

| Command  | Description                      |
| -------- | -------------------------------- |
| `add`    | Add sources to a notebook        |
| `upload` | Upload a file as a source        |
| `get`    | Get details of a specific source |
| `delete` | Delete sources from a notebook   |

## add

Add one or more sources to a notebook.

### Usage

```bash
nblm sources add --notebook-id <ID> [SOURCE_OPTIONS...]
```

### Options

| Option                       | Description                  | Required | Can Repeat |
| ---------------------------- | ---------------------------- | -------- | ---------- |
| `--notebook-id <ID>`         | Notebook identifier          | Yes      | No         |
| `--web-url <URL>`            | Web page URL                 | No       | Yes        |
| `--web-name <NAME>`          | Display name for web source  | No       | Yes        |
| `--text <CONTENT>`           | Text content                 | No       | Yes        |
| `--text-name <NAME>`         | Display name for text source | No       | Yes        |
| `--video-url <URL>`          | YouTube video URL            | No       | Yes        |
| `--drive-id <ID>`            | Google Drive file ID         | No       | Yes        |
| `--drive-resource-key <KEY>` | Drive resource key           | No       | Yes        |
| `--drive-mime-type <TYPE>`   | Drive file MIME type         | No       | Yes        |

**Note**: At least one source option must be provided.

### Examples

**Add a web URL:**

```bash
nblm sources add \
  --notebook-id abc123 \
  --web-url "https://example.com" \
  --web-name "Example Website"
```

**Add text content:**

```bash
nblm sources add \
  --notebook-id abc123 \
  --text "My research notes" \
  --text-name "Notes"
```

**Add YouTube video:**

```bash
nblm sources add \
  --notebook-id abc123 \
  --video-url "https://www.youtube.com/watch?v=dQw4w9WgXcQ"
```

**Add multiple sources at once:**

```bash
nblm sources add \
  --notebook-id abc123 \
  --web-url "https://docs.python.org" \
  --web-name "Python Docs" \
  --text "Sample notes" \
  --text-name "My Notes" \
  --video-url "https://www.youtube.com/watch?v=VIDEO_ID"
```

**Add Google Drive file (currently not working):**

```bash
# WARNING: This returns HTTP 500 as of 2025-10-25
nblm sources add \
  --notebook-id abc123 \
  --drive-id "FILE_ID" \
  --drive-resource-key "RESOURCE_KEY" \
  --drive-mime-type "application/pdf"
```

**JSON output:**

```bash
nblm --json sources add \
  --notebook-id abc123 \
  --web-url "https://example.com"
```

Output:

```json
{
  "sources": [
    {
      "name": "projects/123456789012/locations/global/notebooks/abc123/sources/source-1",
      "title": "Example Website",
      "createTime": "2025-10-25T10:30:00Z"
    }
  ]
}
```

### Notes

- Web URLs are fetched and indexed automatically
- Text content must not be empty
- Video URLs currently only support YouTube (`youtubeUrl` field)
- **Google Drive sources return HTTP 500 error** (API issue as of 2025-10-25)
- The `--web-name` and `--text-name` options are optional; if not provided, defaults are used

## upload

Upload a local file as a notebook source.

### Usage

```bash
nblm sources upload --notebook-id <ID> --file <PATH> [OPTIONS]
```

### Options

| Option                  | Description                   | Required |
| ----------------------- | ----------------------------- | -------- |
| `--notebook-id <ID>`    | Notebook identifier           | Yes      |
| `--file <PATH>`         | Path to file to upload        | Yes      |
| `--content-type <TYPE>` | HTTP Content-Type (MIME type) | No       |
| `--display-name <NAME>` | Display name for the source   | No       |

### Examples

**Upload a PDF:**

```bash
nblm sources upload \
  --notebook-id abc123 \
  --file document.pdf
```

**Upload with custom content type:**

```bash
nblm sources upload \
  --notebook-id abc123 \
  --file report.txt \
  --content-type "text/plain"
```

**Upload with display name:**

```bash
nblm sources upload \
  --notebook-id abc123 \
  --file research.pdf \
  --display-name "Research Paper 2025"
```

**JSON output:**

```bash
nblm --json sources upload \
  --notebook-id abc123 \
  --file document.pdf
```

Output:

```json
{
  "sourceId": "source-abc123"
}
```

### Notes

- Content type is auto-detected from file extension if not specified
- Supported file types include: PDF, TXT, DOCX, and more
- File must exist and be readable
- Empty files cannot be uploaded
- Maximum file size may be limited by the API

## get

Get details of a specific source.

### Usage

```bash
nblm sources get --notebook-id <ID> --source-id <SOURCE_ID>
```

### Options

| Option                    | Description         | Required |
| ------------------------- | ------------------- | -------- |
| `--notebook-id <ID>`      | Notebook identifier | Yes      |
| `--source-id <SOURCE_ID>` | Source identifier   | Yes      |

### Examples

**Get source details:**

```bash
nblm sources get \
  --notebook-id abc123 \
  --source-id source-1
```

**JSON output:**

```bash
nblm --json sources get \
  --notebook-id abc123 \
  --source-id source-1
```

Output:

```json
{
  "name": "projects/123456789012/locations/global/notebooks/abc123/sources/source-1",
  "title": "Example Website",
  "metadata": {
    "wordCount": 1500,
    "sourceAddedTimestamp": "2025-10-25T10:30:00Z"
  },
  "settings": {
    "status": "ACTIVE"
  }
}
```

### Notes

- Use this to verify source details after adding
- Useful for checking processing status
- The `source-id` can be extracted from the full source name

## delete

Delete one or more sources from a notebook.

### Usage

```bash
nblm sources delete --notebook-id <ID> --source-name <NAME> [--source-name <NAME>...]
```

### Options

| Option                 | Description                                 | Required |
| ---------------------- | ------------------------------------------- | -------- |
| `--notebook-id <ID>`   | Notebook identifier                         | Yes      |
| `--source-name <NAME>` | Full source resource name (can be repeated) | Yes      |

### Examples

**Delete a single source:**

```bash
nblm sources delete \
  --notebook-id abc123 \
  --source-name "projects/123456789012/locations/global/notebooks/abc123/sources/source-1"
```

**Delete multiple sources:**

```bash
nblm sources delete \
  --notebook-id abc123 \
  --source-name "projects/.../notebooks/abc123/sources/source-1" \
  --source-name "projects/.../notebooks/abc123/sources/source-2"
```

**Get source names from notebook and delete:**

```bash
# List sources and extract names (requires getting notebook details first)
SOURCE_NAME="projects/123456789012/locations/global/notebooks/abc123/sources/source-1"

nblm sources delete \
  --notebook-id abc123 \
  --source-name "$SOURCE_NAME"
```

### Notes

- Deletion is permanent and cannot be undone
- The full source resource name is required (not just the source ID)
- Multiple sources can be deleted in a single command

## Common Patterns

### Add and verify source

```bash
# Add source
RESULT=$(nblm --json sources add \
  --notebook-id abc123 \
  --web-url "https://example.com")

# Extract source name
SOURCE_NAME=$(echo "$RESULT" | jq -r '.sources[0].name')

# Extract source ID from name
SOURCE_ID=$(echo "$SOURCE_NAME" | awk -F'/' '{print $NF}')

# Get source details
nblm sources get \
  --notebook-id abc123 \
  --source-id "$SOURCE_ID"
```

### Bulk upload files

```bash
# Upload all PDFs in a directory
for file in *.pdf; do
  echo "Uploading $file..."
  nblm sources upload \
    --notebook-id abc123 \
    --file "$file"
done
```

### Add sources from a list

```bash
# urls.txt contains one URL per line
while IFS= read -r url; do
  echo "Adding $url..."
  nblm sources add \
    --notebook-id abc123 \
    --web-url "$url"
done < urls.txt
```

## Error Handling

### Common Errors

**Empty text content:**

```
Error: Text content cannot be empty
Cause: The --text option was provided with empty string
```

**File not found:**

```
Error: File not found: /path/to/file
Cause: The specified file does not exist or is not readable
```

**Google Drive API error:**

```
Error: Failed to add source
Cause: API returned 500 Internal Server Error
Note: Google Drive sources are currently not working (as of 2025-10-25)
```

**Invalid notebook ID:**

```
Error: Notebook not found
Cause: The specified notebook does not exist or you don't have access
```

## Next Steps

- [Notebooks Commands](notebooks.md) - Create and manage notebooks
- [Audio Commands](audio.md) - Create audio overviews from sources
- [Advanced Usage](advanced.md) - Scripting and automation
