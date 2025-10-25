# Installation

Install the NotebookLM Enterprise API client as a CLI tool or Python SDK.

## Prerequisites

- Google Cloud project with NotebookLM API enabled
- Google Cloud authentication (gcloud CLI recommended)

## CLI Installation

### From crates.io

```bash
cargo install nblm-cli
```

### From Source

```bash
git clone https://github.com/K-dash/nblm-rs.git
cd nblm-rs
cargo build --release
```

The binary will be available at `target/release/nblm`.

**Optional**: Add to PATH

```bash
# Linux/macOS
sudo cp target/release/nblm /usr/local/bin/

# Or add to your shell profile
export PATH="$PATH:/path/to/nblm-rs/target/release"
```

### Verify Installation

```bash
nblm --version
```

## Python SDK Installation

### With pip

```bash
pip install nblm
```

### With uv

```bash
uv add nblm
```

### From Source

```bash
git clone https://github.com/K-dash/nblm-rs.git
cd nblm-rs
cd python
pip install maturin
maturin develop
```

### Verify Installation

```python
import nblm
print(nblm.__version__)
```

## Platform Support

| Platform              | CLI           | Python SDK    |
| --------------------- | ------------- | ------------- |
| Linux (x86_64)        | Supported     | Supported     |
| Linux (aarch64)       | Supported     | Supported     |
| macOS (Intel)         | Supported     | Supported     |
| macOS (Apple Silicon) | Supported     | Supported     |
| Windows               | Not Supported | Not Supported |

> **Note**: Windows support is not available. Consider using WSL (Windows Subsystem for Linux) as a workaround.

## Next Steps

- [Authentication Setup](authentication.md) - Configure authentication
- [Configuration](configuration.md) - Set up project numbers and locations
- [CLI Overview](../cli/README.md) - Start using the CLI
- [Python Quickstart](../python/quickstart.md) - Start using the Python SDK
