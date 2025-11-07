# Troubleshooting

This guide collects the most common issues reported for the NotebookLM Enterprise clients (CLI and Python SDK) and outlines quick checks to resolve them.

## Authentication errors

- Confirm that `GOOGLE_APPLICATION_CREDENTIALS` points to the correct service-account JSON file.
- If you rely on the gcloud CLI, refresh Application Default Credentials with `gcloud auth application-default login`.
- Verify that the notebook region and the project number used for authentication match the resources you are operating against.

## 403 or 404 responses from the API

- Run `nblm-cli doctor` to double-check the `--project-number` and `--location` values.
- Ensure the NotebookLM Enterprise API is enabled for the target project in Cloud Console.

## Upload timeouts

- The CLI uses a default timeout of a few minutes. Increase it with `--timeout-seconds` and re-run the command.
- For the Python SDK, pass a higher `timeout` value to `client.upload_source` and enable the retry policy.
- Consider compressing or splitting very large files before uploading them.

## When you need more help

- Launch commands with `--debug` to capture verbose logs.
- When filing a GitHub Issue, include the command that failed, environment details, and sanitized log snippets (omit sensitive data).
- For contribution guidelines, refer to [CONTRIBUTING.md](https://github.com/K-dash/nblm-rs/blob/main/CONTRIBUTING.md).
