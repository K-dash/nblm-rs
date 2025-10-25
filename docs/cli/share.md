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

## Common Patterns

### Share with a team

```bash
# Share notebook with multiple team members
NOTEBOOK_ID="abc123"
TEAM_EMAILS=(
  "alice@example.com"
  "bob@example.com"
  "charlie@example.com"
)

for email in "${TEAM_EMAILS[@]}"; do
  echo "Sharing with $email..."
  nblm --auth gcloud share add \
    --notebook-id "$NOTEBOOK_ID" \
    --email "$email" \
    --role writer
done

echo "Notebook shared with all team members."
```

### Share from a list

```bash
# emails.txt contains one email per line
NOTEBOOK_ID="abc123"
ROLE="reader"

while IFS= read -r email; do
  echo "Sharing with $email..."
  nblm --auth gcloud share add \
    --notebook-id "$NOTEBOOK_ID" \
    --email "$email" \
    --role "$ROLE"
done < emails.txt
```

### Update user role

```bash
# Change user's role from reader to writer
NOTEBOOK_ID="abc123"
USER_EMAIL="user@example.com"

echo "Updating role to writer..."
nblm --auth gcloud share add \
  --notebook-id "$NOTEBOOK_ID" \
  --email "$USER_EMAIL" \
  --role writer

echo "Role updated."
```

### Revoke access for multiple users

```bash
# Remove access for multiple users
NOTEBOOK_ID="abc123"
USERS_TO_REMOVE=(
  "former-member@example.com"
  "contractor@example.com"
)

for email in "${USERS_TO_REMOVE[@]}"; do
  echo "Revoking access for $email..."
  nblm --auth gcloud share add \
    --notebook-id "$NOTEBOOK_ID" \
    --email "$email" \
    --role not-shared
done

echo "Access revoked for all specified users."
```

## Error Handling

### Common Errors

**Invalid email address:**

```
Error: Invalid email address
Cause: The provided email is not a valid format
```

**Notebook not found:**

```
Error: Notebook not found
Cause: The specified notebook does not exist or you don't have access
```

**Permission denied:**

```
Error: Permission denied
Cause: You don't have permission to share this notebook
Note: Only owners can manage sharing
```

**User not found:**

```
Error: User not found
Cause: No Google account exists with the specified email address
```

## Permissions and Roles

### Role Comparison

| Action                | Owner | Writer | Reader |
| --------------------- | ----- | ------ | ------ |
| View notebook         | Yes   | Yes    | Yes    |
| Edit title            | Yes   | Yes    | No     |
| Add sources           | Yes   | Yes    | No     |
| Delete sources        | Yes   | Yes    | No     |
| Create audio overview | Yes   | Yes    | No     |
| Share with others     | Yes   | No     | No     |
| Delete notebook       | Yes   | No     | No     |

### Best Practices

- **Principle of least privilege**: Start with `reader` role and upgrade as needed
- **Owner role**: Be cautious when granting owner role as it allows deletion
- **Regular audits**: Periodically review who has access to sensitive notebooks
- **Remove access promptly**: Use `not-shared` role when users no longer need access

## Limitations

- Cannot list current sharing settings via CLI (use NotebookLM web UI)
- Cannot retrieve audit logs of sharing actions via CLI

## Next Steps

- [Notebooks Commands](notebooks.md) - Create and manage notebooks
- [Sources Commands](sources.md) - Add content to notebooks
- [Advanced Usage](advanced.md) - Scripting and automation
