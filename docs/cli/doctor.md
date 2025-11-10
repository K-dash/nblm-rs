# Doctor Command

Run NotebookLM CLI diagnostics to verify that your environment is ready before calling other commands.

## Usage

```bash
nblm doctor
```

No additional flags are required. Global options such as `--project-number` are ignored; the command relies on environment variables instead.

### Options

| Flag                | Description                                                              |
| ------------------- | ------------------------------------------------------------------------ |
| `--skip-api-check`  | Skip the API connectivity check (useful for offline environments or CI)  |

## What It Checks

The doctor command runs a series of health checks and prints the result of each one with a status marker:

| Status  | Meaning                                                              |
| ------- | -------------------------------------------------------------------- |
| `[ok]`  | Check passed                                                         |
| `[warn]`| Non-blocking issue was detected (command exits with status code `1`) |
| `[error]`| Blocking issue was detected (command exits with status code `2`)     |

### Environment Variables

| Variable                 | Required | Passing Condition                                     | Failure Result                     |
| ------------------------ | -------- | ----------------------------------------------------- | ---------------------------------- |
| `NBLM_PROJECT_NUMBER`    | Yes      | Variable is set to a non-empty value                  | `[error]` with export suggestion   |
| `NBLM_ENDPOINT_LOCATION` | No       | Variable is set (defaults to `global` when missing)   | `[warn]` with suggested default    |
| `NBLM_LOCATION`          | No       | Variable is set (defaults to `global` when missing)   | `[warn]` with suggested default    |
| `NBLM_ACCESS_TOKEN`      | No       | Variable is set to a non-empty value (value hidden)   | `[warn]` suggesting token export   |

Values for sensitive variables (such as `NBLM_ACCESS_TOKEN`) are never printed. You will only see `set (value hidden)` in the output.

### Google Drive Access

If `NBLM_ACCESS_TOKEN` is set, the doctor command validates that the token includes a Drive scope (`drive` or `drive.file`).

- `[ok]` — token includes the required Drive scope
- `[warn]` — scope is missing or cannot be confirmed. The command prints:
  - A recommendation to run `gcloud auth login --enable-gdrive-access`
  - The original environment variable remains untouched

You can skip this check by omitting `NBLM_ACCESS_TOKEN`. This is useful if you never upload Drive files.

### Command Availability

The doctor command currently verifies that the Google Cloud CLI (`gcloud`) is installed. Missing commands produce warnings with download links.

### API Connectivity

The doctor command performs a connectivity check to verify access to the NotebookLM API by calling `list_recently_viewed` with a minimal page size.

**Behavior:**
- Automatically skipped if `NBLM_PROJECT_NUMBER` is missing (to avoid duplicate error reporting)
- Automatically skipped if `gcloud` is not available and `NBLM_ACCESS_TOKEN` is not set (to avoid interactive prompts)
- Can be manually skipped using the `--skip-api-check` flag

**Possible Results:**

| Status    | Message                                     | Meaning                                                          |
| --------- | ------------------------------------------- | ---------------------------------------------------------------- |
| `[ok]`    | Successfully connected to NotebookLM API    | API is accessible with current credentials                       |
| `[error]` | Authentication failed (401 Unauthorized)    | Credentials are missing or invalid                               |
| `[error]` | Permission denied (403 Forbidden)           | Account lacks NotebookLM API access or required IAM roles        |
| `[error]` | Resource not found (404)                    | Project number may be incorrect or NotebookLM is not enabled     |
| `[error]` | Network error                               | Connection timeout or network issues                             |
| `[error]` | API error                                   | Other API errors with details in the message                     |

Each error includes a suggestion for resolution, such as:
- Running `gcloud auth login` for authentication issues
- Verifying IAM roles for permission errors
- Checking the project number for 404 errors

## Exit Codes

| Code | Meaning                                 |
| ---- | --------------------------------------- |
| 0    | All checks passed                       |
| 1    | Only warnings were encountered          |
| 2    | At least one blocking error was found   |

Use the exit code from CI pipelines or shell scripts to block deployments when required variables are missing.

## Example Output

### Successful Check

```text
Running NotebookLM environment diagnostics...

   [ok] NBLM_PROJECT_NUMBER=123456789012
   [ok] NBLM_ENDPOINT_LOCATION=global
   [ok] NBLM_LOCATION=global
   [ok] NBLM_ACCESS_TOKEN set (value hidden)
   [ok] NBLM_ACCESS_TOKEN grants Google Drive access
   [ok] gcloud is installed (Google Cloud SDK 544.0.0)
   [ok] Successfully connected to NotebookLM API

Summary: All 7 checks passed.

All critical checks passed. You're ready to use nblm.
```

### Warning Examples

Warnings appear inline when a check fails:

```text
 [warn] NBLM_ACCESS_TOKEN lacks Google Drive scope
       Suggestion: Run `gcloud auth login --enable-gdrive-access` and refresh NBLM_ACCESS_TOKEN
```

### API Error Example

```text
[error] Authentication failed (401 Unauthorized)
       Suggestion: Run `gcloud auth login` or `gcloud auth application-default login`
```

## Troubleshooting

- **Missing Drive scope**: Re-authenticate with `gcloud auth login --enable-gdrive-access`, then refresh `NBLM_ACCESS_TOKEN` using `gcloud auth print-access-token`.
- **Project number missing**: Export `NBLM_PROJECT_NUMBER` or pass `--project-number` to other CLI commands once the doctor checks succeed.
- **gcloud not found**: Install the Google Cloud CLI from <https://cloud.google.com/sdk/docs/install>.
- **API connectivity errors**:
  - For authentication issues (401): Run `gcloud auth login` or set a valid `NBLM_ACCESS_TOKEN`
  - For permission errors (403): Verify your account has the necessary IAM roles (e.g., `aiplatform.user`)
  - For resource not found (404): Double-check your `NBLM_PROJECT_NUMBER` and ensure NotebookLM is enabled for the project
  - For network errors: Check your internet connection and firewall settings
- **Skip API check**: Use `--skip-api-check` flag if you want to perform diagnostics in offline environments or CI pipelines without API access.

