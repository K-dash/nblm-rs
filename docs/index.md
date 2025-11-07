# nblm-rs Documentation

Complete documentation for the NotebookLM Enterprise API client (CLI & Python SDK).

!!! important "Unofficial Project"
    This project is not affiliated with, sponsored, or endorsed by Google. nblm-rs is an independent, unofficial tool. It is provided "as is" without any warranty.

## Getting Started

**New to nblm-rs?** Start here:

- [Installation](getting-started/installation.md) - Install CLI or Python SDK
- [Authentication](getting-started/authentication.md) - Set up authentication with gcloud
- [Configuration](getting-started/configuration.md) - Project numbers, locations, environment variables

## Features

!!! note "API Status"
    The NotebookLM API is currently in **alpha**. Some features may not work as documented due to API limitations. See [API Limitations](api/limitations.md) for details.

### Notebooks

| Feature               | CLI | Python | Status  | Notes                                |
| --------------------- | --- | ------ | ------- | ------------------------------------ |
| Create notebook       | ✅  | ✅     | Working |                                      |
| List recent notebooks | ✅  | ✅     | Working | Pagination not implemented by API    |
| Delete notebook(s)    | ✅  | ✅     | Working | Sequential deletion (API limitation) |

### Sources

| Feature             | CLI | Python | Status  | Notes                       |
| ------------------- | --- | ------ | ------- | --------------------------- |
| Add web URL         | ✅  | ✅     | Working |                             |
| Add text content    | ✅  | ✅     | Working |                             |
| Add video (YouTube) | ✅  | ✅     | Working | Uses `youtubeUrl` field     |
| Add Google Drive    | ✅  | ✅     | Working | Requires Drive-enabled auth |
| Upload file         | ✅  | ✅     | Working |                             |
| Delete source(s)    | ✅  | ✅     | Working |                             |
| Get source by ID    | ✅  | ✅     | Working |                             |

### Audio Overview

| Feature               | CLI | Python | Status  | Notes                       |
| --------------------- | --- | ------ | ------- | --------------------------- |
| Create audio overview | ✅  | ✅     | Working | Config fields not supported |
| Delete audio overview | ✅  | ✅     | Working |                             |

### Sharing

| Feature        | CLI | Python | Status   | Notes                     |
| -------------- | --- | ------ | -------- | ------------------------- |
| Share notebook | ✅  | ❌     | Untested | Requires additional users |

## CLI Reference

Complete command-line interface documentation:

- [CLI Overview](cli/README.md) - Command structure and common options
- [Notebooks Commands](cli/notebooks.md) - Create, list, and delete notebooks
- [Sources Commands](cli/sources.md) - Add, upload, and manage sources
- [Audio Commands](cli/audio.md) - Create and delete audio overviews
- [Doctor Command](cli/doctor.md) - Run environment diagnostics

## Python SDK Reference

Python bindings documentation:

- [Python SDK Overview](python/README.md) - Installation and basic usage
- [Quickstart](python/quickstart.md) - Get started in 5 minutes
- [API Reference](python/api-reference.md) - All classes and methods
- [Source Management](python/sources.md) - Source operations in detail
- [Notebooks API](python/notebooks.md) - Notebook operations in detail
- [Audio API](python/audio.md) - Audio overview operations
- [Error Handling](python/error-handling.md) - Exception handling patterns

## Rust SDK

Rust library documentation:

- [Getting Started](rust/getting-started.md) - Rust SDK setup and usage

!!! note "Work in Progress"
    The Rust SDK is currently being refactored. The Getting Started guide will be updated once the new core APIs are finalized.

## Guides

Additional guides and tutorials:

- [Troubleshooting](guides/troubleshooting.md) - Common issues and solutions

## API Information

- [API Limitations](api/limitations.md) - Known limitations and workarounds
- [NotebookLM API Documentation](https://cloud.google.com/gemini/enterprise/notebooklm-enterprise/docs/overview) - Official API docs

## Contributing

- [Contributing Guide](https://github.com/K-dash/nblm-rs/blob/main/CONTRIBUTING.md) - Development setup and guidelines

---

!!! note
    The `investigation/` directory contains internal research notes and experiments with the NotebookLM API.
