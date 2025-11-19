# Auth Commands

Commands for managing authentication with Google Cloud.

## Overview

The `auth` command simplifies the authentication process by wrapping the Google Cloud SDK (`gcloud`) authentication flow. It allows you to log in and check your authentication status directly from the `nblm` CLI.

## Commands

### `login`

Log in to Google Cloud using the `gcloud` CLI. This command opens a browser window to authenticate with your Google account.

```bash
nblm auth login
```

**Behavior:**

1.  Executes `gcloud auth login` in the background.
2.  Opens your default web browser for Google authentication.
3.  Saves credentials to the standard `gcloud` configuration location.

**Exit Codes:**

- `0`: Authentication successful.
- `1`: Authentication failed (e.g., user cancelled, network error).

### `status`

Check the current authentication status.

```bash
nblm auth status
```

**Output (Authenticated):**

```text
Authenticated
Account: user@example.com
Backend: gcloud
```

**Output (Not Authenticated):**

```text
Not authenticated.
Run 'nblm auth login' to log in.
```

**Exit Codes:**

- `0`: User is authenticated.
- `1`: User is NOT authenticated.

## Examples

### Initial Setup

```bash
# 1. Log in
nblm auth login

# 2. Verify status
nblm auth status

# 3. Start using nblm
nblm notebooks recent
```

### Scripting

You can use the exit code of `nblm auth status` to check if the user is logged in before running other commands.

```bash
if ! nblm auth status > /dev/null 2>&1; then
  echo "Please log in first."
  exit 1
fi

nblm notebooks recent
```
