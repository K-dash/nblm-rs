# NotebookLM API Known Limitations

This document tracks verified API limitations and workarounds implemented in nblm-rs.

> **Last Updated**: 2025-10-24

## Batch Delete Notebooks

**Discovered**: 2025-10-19
**Status**: Confirmed API limitation
**Severity**: Medium

### Issue

The `batchDeleteNotebooks` API endpoint accepts an array of notebook names in the request body, but only successfully processes a single notebook at a time. Attempting to delete multiple notebooks in one request results in HTTP 400 error.

**API Endpoint**: `POST /v1alpha1/projects/{project}/locations/{location}/notebooks:batchDelete`

**Request Format**:
```json
{
  "names": [
    "projects/123/locations/global/notebooks/abc",
    "projects/123/locations/global/notebooks/def"
  ]
}
```

**Behavior**:
- ✓ Works: Array with 1 element
- ✗ Fails: Array with 2+ elements (HTTP 400)

### Workaround

nblm-rs implements sequential deletion:
```rust
pub async fn delete_notebooks(&self, notebook_names: Vec<String>) -> Result<...> {
    for name in &notebook_names {
        let request = BatchDeleteNotebooksRequest {
            names: vec![name.clone()],  // Single item only
        };
        self.batch_delete_notebooks(request).await?;
    }
    Ok(...)
}
```

### Impact

- Multiple deletions take longer (sequential API calls)
- Cannot leverage true batch operation benefits
- Retry logic applies to each individual deletion

## Pagination Not Implemented

**Discovered**: 2025-10-19 (per README)
**Status**: Confirmed API limitation
**Severity**: Low

### Issue

The `listRecentlyViewed` API accepts `pageSize` and `pageToken` parameters but never returns `nextPageToken` in responses, indicating pagination is not currently implemented.

**API Endpoint**: `GET /v1alpha1/projects/{project}/locations/{location}/notebooks:listRecentlyViewed`

### Behavior

- `pageSize` parameter is accepted but may not be honored
- `nextPageToken` is never returned in responses
- All accessible notebooks appear to be returned in single response

### Workaround

None needed. The API returns all results in one call.

### Impact

- Cannot paginate through large notebook lists
- May cause performance issues with very large notebook collections (untested)

## Google Drive Source Addition Requirements

**Discovered**: 2025-10-19 (per README)  
**Status**: Works with prerequisites  
**Severity**: Medium

### Issue

The API returns 404/500 errors when the authenticated principal cannot access the target Drive document. Early testing interpreted this as an API limitation, but it is caused by insufficient Drive permissions.

**API Endpoint**: `POST /v1alpha1/.../sources:batchCreate`

### Behavior

```bash
$ nblm sources add --notebook-id ID \
    --drive-document-id FILE_ID \
    --drive-mime-type application/pdf
Error: http error 404 Not Found: Requested entity was not found.
```

### Workaround

- Authenticate with Drive-enabled credentials: `gcloud auth login --enable-gdrive-access`
- Ensure the Drive document is shared with (or owned by) the authenticated account
- Provide the correct MIME type reported by the Drive API

Once these prerequisites are satisfied, Drive sources ingest successfully.

### Impact

- Users must configure Drive access explicitly before automation works
- Helpful to document authentication instructions for contributors

## Audio Overview Configuration Fields Not Supported

**Discovered**: 2025-10-19 (per README)
**Status**: Confirmed API limitation
**Severity**: Low

### Issue

API documentation mentions configuration fields (`languageCode`, `sourceIds`, `episodeFocus`), but the API rejects all of these fields with "Unknown name" errors. Only empty request body `{}` is accepted.

**API Endpoint**: `POST /v1alpha1/.../audioOverviews`

### Behavior

**Documented (but rejected)**:
```json
{
  "languageCode": "en",
  "sourceIds": [...],
  "episodeFocus": "..."
}
```

**Actually accepted**:
```json
{}
```

### Workaround

Create audio overview with empty request, then configure settings through NotebookLM web UI.

### Impact

- Cannot specify language or source selection via API
- Cannot focus episode on specific topics via API
- Audio overview creation is "fire and forget" from API perspective

## Summary

| Limitation | Severity | Workaround | Status |
|------------|----------|------------|--------|
| Batch delete only accepts 1 item | Medium | Sequential deletion | Implemented |
| Pagination not working | Low | None needed | Noted |
| Google Drive sources require Drive access | Medium | Enable gcloud --enable-gdrive-access and share document | Documented |
| Audio config fields rejected | Low | Use web UI for config | Documented |
