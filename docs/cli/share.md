# Share Commands

Share notebooks with other users.

> **Warning**: This command has **not been tested** in a real environment as it requires multiple Google accounts. The documentation is based on the API specification and should work as described, but functionality is not guaranteed. Use at your own risk.
>
> **Official Documentation**: For detailed information about sharing permissions and requirements, see [Share notebooks - Google Cloud](https://cloud.google.com/gemini/enterprise/notebooklm-enterprise/docs/share-notebooks).
>
> **Contributions Welcome**: If you have access to a multi-user environment and can test this functionality, we would greatly appreciate your contributions. Please see [CONTRIBUTING.md](../../CONTRIBUTING.md) for how to contribute test results or improvements.

## Available Commands

| Command | Description                 |
| ------- | --------------------------- |
| `add`   | Share a notebook with users |

## add

Share a notebook with one or more users by email.

### Usage

```bash
nblm share add --notebook-id <ID> --email <EMAIL> [--email <EMAIL>...] --role <ROLE>
```

### Options

| Option               | Description                          | Required |
| -------------------- | ------------------------------------ | -------- |
| `--notebook-id <ID>` | Notebook identifier                  | Yes      |
| `--email <EMAIL>`    | User email address (can be repeated) | Yes      |
| `--role <ROLE>`      | Permission role                      | Yes      |

### Roles

| Role         | Permissions                           |
| ------------ | ------------------------------------- |
| `owner`      | Full control (manage sharing, delete) |
| `writer`     | Edit notebook and sources             |
| `reader`     | Read-only access                      |
| `not-shared` | Remove user access                    |

### Examples

**Share with a single user:**

```bash
nblm --auth gcloud share add \
  --notebook-id abc123 \
  --email user@example.com \
  --role reader
```

**Share with multiple users:**

```bash
nblm --auth gcloud share add \
  --notebook-id abc123 \
  --email user1@example.com \
  --email user2@example.com \
  --role writer
```

**Grant ownership:**

```bash
nblm --auth gcloud share add \
  --notebook-id abc123 \
  --email collaborator@example.com \
  --role owner
```

**Remove user access:**

```bash
nblm --auth gcloud share add \
  --notebook-id abc123 \
  --email user@example.com \
  --role not-shared
```

**JSON output:**

```bash
nblm --auth gcloud --json share add \
  --notebook-id abc123 \
  --email user@example.com \
  --role reader
```

Output:

```json
{
  "success": true
}
```

### Notes

- Users must have Google accounts with the specified email addresses
- Users will receive a notification when a notebook is shared with them
- The `owner` role allows users to delete the notebook and manage sharing
- Use `not-shared` role to revoke access


## Limitations

- Cannot list current sharing settings via CLI (use NotebookLM web UI)
- Cannot retrieve audit logs of sharing actions via CLI

## Next Steps

- [Notebooks Commands](notebooks.md) - Create and manage notebooks
- [Sources Commands](sources.md) - Add content to notebooks
