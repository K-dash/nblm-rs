# Error Handling

Exception handling patterns for the NotebookLM Python SDK.

## Exception Hierarchy

```python
from nblm import NblmError

# NblmError is the base exception for all SDK errors
try:
    notebook = client.create_notebook(title="Test")
except NblmError as e:
    print(f"Error: {e}")
```

All errors raised by the SDK are instances of `NblmError`.

## Basic Error Handling

### Try-Except Pattern

```python
from nblm import NblmClient, GcloudTokenProvider, NblmError

client = NblmClient(
    token_provider=GcloudTokenProvider(),
    project_number="123456789012"
)

try:
    notebook = client.create_notebook(title="My Notebook")
    print(f"Success: {notebook.notebook_id}")
except NblmError as e:
    print(f"Failed: {e}")
```

### Multiple Operations

```python
from nblm import WebSource

try:
    # Create notebook
    notebook = client.create_notebook(title="Research")

    # Add sources
    client.add_sources(
        notebook_id=notebook.notebook_id,
        web_sources=[WebSource(url="https://example.com")]
    )

    # Create audio
    audio = client.create_audio_overview(notebook_id=notebook.notebook_id)

    print("All operations succeeded")

except NblmError as e:
    print(f"Operation failed: {e}")
    # Cleanup or retry logic here
```

## Error Categories

### Authentication Errors

```python
try:
    client = NblmClient(
        token_provider=GcloudTokenProvider(),
        project_number="123456789012"
    )
    notebook = client.create_notebook(title="Test")

except NblmError as e:
    error_msg = str(e).lower()
    if "auth" in error_msg or "permission" in error_msg:
        print("Authentication error:")
        print("  1. Run: gcloud auth login")
        print("  2. Verify project access")
    else:
        print(f"Other error: {e}")
```

### Validation Errors

```python
from nblm import TextSource

try:
    # Empty text validation
    client.add_sources(
        notebook_id="abc123",
        text_sources=[TextSource(content="", name="Empty")]
    )
except NblmError as e:
    if "empty" in str(e).lower():
        print("Validation error: Content cannot be empty")
    else:
        print(f"Error: {e}")
```

### API Errors

```python
try:
    client.delete_notebooks(["projects/.../notebooks/nonexistent"])
except NblmError as e:
    if "not found" in str(e).lower():
        print("Resource not found")
    elif "403" in str(e) or "forbidden" in str(e).lower():
        print("Permission denied")
    elif "500" in str(e):
        print("Server error - may be transient, try again")
    else:
        print(f"API error: {e}")
```

## Retry Patterns

### Simple Retry

```python
import time
from nblm import NblmError

def create_notebook_with_retry(
    client: NblmClient,
    title: str,
    max_retries: int = 3
):
    for attempt in range(max_retries):
        try:
            return client.create_notebook(title)
        except NblmError as e:
            if attempt < max_retries - 1:
                wait_time = 2 ** attempt  # Exponential backoff
                print(f"Attempt {attempt + 1} failed, retrying in {wait_time}s...")
                time.sleep(wait_time)
            else:
                raise

# Usage
notebook = create_notebook_with_retry(client, "My Notebook")
```

### Exponential Backoff

```python
import time
from typing import TypeVar, Callable
from nblm import NblmError

T = TypeVar('T')

def retry_with_backoff(
    func: Callable[[], T],
    max_retries: int = 3,
    base_delay: float = 1.0
) -> T:
    """Execute function with exponential backoff retry."""
    for attempt in range(max_retries):
        try:
            return func()
        except NblmError as e:
            if attempt < max_retries - 1:
                delay = base_delay * (2 ** attempt)
                print(f"Retry {attempt + 1}/{max_retries} after {delay}s...")
                time.sleep(delay)
            else:
                raise

# Usage
notebook = retry_with_backoff(
    lambda: client.create_notebook(title="My Notebook"),
    max_retries=3
)
```

## Graceful Degradation

### Fallback Values

```python
from nblm import NblmError

def get_notebook_count(client: NblmClient) -> int:
    """Get notebook count, return 0 on error."""
    try:
        response = client.list_recently_viewed()
        return len(response.notebooks)
    except NblmError as e:
        print(f"Warning: Failed to get notebooks: {e}")
        return 0

count = get_notebook_count(client)
print(f"Notebooks: {count}")
```

### Partial Success Handling

```python
from nblm import WebSource, NblmError

def add_sources_with_partial_success(
    client: NblmClient,
    notebook_id: str,
    urls: list[str]
):
    """Add sources one by one, continue on errors."""
    succeeded = []
    failed = []

    for url in urls:
        try:
            response = client.add_sources(
                notebook_id=notebook_id,
                web_sources=[WebSource(url=url)]
            )
            succeeded.append(url)
        except NblmError as e:
            failed.append((url, str(e)))
            print(f"Failed to add {url}: {e}")

    return succeeded, failed

# Usage
urls = ["https://example.com", "https://invalid", "https://example.org"]
succeeded, failed = add_sources_with_partial_success(client, "abc123", urls)

print(f"Succeeded: {len(succeeded)}")
print(f"Failed: {len(failed)}")
```

## Logging

### Basic Logging

```python
import logging
from nblm import NblmClient, GcloudTokenProvider, NblmError

# Configure logging
logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s - %(levelname)s - %(message)s'
)
logger = logging.getLogger(__name__)

client = NblmClient(
    token_provider=GcloudTokenProvider(),
    project_number="123456789012"
)

try:
    logger.info("Creating notebook...")
    notebook = client.create_notebook(title="Test Notebook")
    logger.info(f"Created notebook: {notebook.notebook_id}")

except NblmError as e:
    logger.error(f"Failed to create notebook: {e}", exc_info=True)
    raise
```

### Structured Logging

```python
import json
import logging
from nblm import NblmError

logger = logging.getLogger(__name__)

def log_error(operation: str, error: NblmError, **context):
    """Log error with structured context."""
    log_data = {
        "operation": operation,
        "error": str(error),
        "context": context
    }
    logger.error(json.dumps(log_data))

# Usage
try:
    client.create_notebook(title="Test")
except NblmError as e:
    log_error("create_notebook", e, title="Test", project="123456789012")
```

## Type Checking

### Using Type Hints

```python
from typing import Optional
from nblm import NblmClient, Notebook, NblmError

def create_notebook_safe(
    client: NblmClient,
    title: str
) -> Optional[Notebook]:
    """Create notebook, return None on error."""
    try:
        return client.create_notebook(title)
    except NblmError as e:
        print(f"Error: {e}")
        return None

# Usage with type checking
notebook = create_notebook_safe(client, "Test")
if notebook is not None:
    print(f"Created: {notebook.notebook_id}")
else:
    print("Failed to create notebook")
```

### Result Types

```python
from typing import Union
from dataclasses import dataclass
from nblm import Notebook, NblmError

@dataclass
class Success:
    notebook: Notebook

@dataclass
class Failure:
    error: str

Result = Union[Success, Failure]

def create_notebook_result(
    client: NblmClient,
    title: str
) -> Result:
    """Create notebook and return typed result."""
    try:
        notebook = client.create_notebook(title)
        return Success(notebook=notebook)
    except NblmError as e:
        return Failure(error=str(e))

# Usage
result = create_notebook_result(client, "Test")
match result:
    case Success(notebook):
        print(f"Created: {notebook.notebook_id}")
    case Failure(error):
        print(f"Failed: {error}")
```

## Cleanup Patterns

### Automatic Cleanup

```python
from nblm import NblmClient, GcloudTokenProvider, WebSource

client = NblmClient(
    token_provider=GcloudTokenProvider(),
    project_number="123456789012"
)

notebook = None
try:
    # Create resources
    notebook = client.create_notebook(title="Temporary")

    # Use resources
    client.add_sources(
        notebook_id=notebook.notebook_id,
        web_sources=[WebSource(url="https://example.com")]
    )

    # Process...

except Exception as e:
    print(f"Error during processing: {e}")
    raise
finally:
    # Always cleanup
    if notebook is not None:
        try:
            client.delete_notebooks([notebook.name])
            print("Cleanup successful")
        except NblmError as cleanup_error:
            print(f"Cleanup failed: {cleanup_error}")
```

### Resource Manager

```python
from typing import Optional
from contextlib import contextmanager

class NotebookManager:
    def __init__(self, client: NblmClient):
        self.client = client
        self.notebooks: list[str] = []

    def create(self, title: str) -> Notebook:
        """Create and track notebook."""
        notebook = self.client.create_notebook(title)
        self.notebooks.append(notebook.name)
        return notebook

    def cleanup(self):
        """Delete all tracked notebooks."""
        if self.notebooks:
            try:
                self.client.delete_notebooks(self.notebooks)
                print(f"Cleaned up {len(self.notebooks)} notebooks")
                self.notebooks.clear()
            except NblmError as e:
                print(f"Cleanup error: {e}")

# Usage
manager = NotebookManager(client)
try:
    nb1 = manager.create("Notebook 1")
    nb2 = manager.create("Notebook 2")
    # Do work...
finally:
    manager.cleanup()
```

## Debugging

### Enable Debug Output

```python
import logging

# Enable debug logging for troubleshooting
logging.basicConfig(level=logging.DEBUG)

# Your code here
client = NblmClient(...)
```

### Inspect Error Details

```python
from nblm import NblmError

try:
    client.create_notebook(title="Test")
except NblmError as e:
    print(f"Error type: {type(e)}")
    print(f"Error message: {str(e)}")
    print(f"Error repr: {repr(e)}")
```

## Next Steps

- [API Reference](api-reference.md) - Complete API documentation
- [Notebooks API](notebooks.md) - Notebook operations
- [Sources API](sources.md) - Source operations
- [Audio API](audio.md) - Audio overview operations
