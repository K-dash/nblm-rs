#!/usr/bin/env bash
set -euo pipefail

# Usage: ./scripts/bump-version.sh 0.1.1

if [ $# -ne 1 ]; then
  echo "Usage: $0 <version>"
  echo "Example: $0 0.1.1"
  exit 1
fi

NEW_VERSION="$1"

echo "Bumping version to ${NEW_VERSION}..."

# Rust crates
echo "Updating Rust crates..."
sed -i.bak "s/^version = \".*\"/version = \"${NEW_VERSION}\"/" Cargo.toml
sed -i.bak "s/^version = \".*\"/version = \"${NEW_VERSION}\"/" crates/nblm-core/Cargo.toml
sed -i.bak "s/^version = \".*\"/version = \"${NEW_VERSION}\"/" crates/nblm-cli/Cargo.toml
sed -i.bak "s/^version = \".*\"/version = \"${NEW_VERSION}\"/" crates/nblm-python/Cargo.toml

# Python package
echo "Updating Python package..."
sed -i.bak "s/^version = \".*\"/version = \"${NEW_VERSION}\"/" python/pyproject.toml
(cd python && uv sync)

# Clean up backup files
find . -name "*.bak" -delete

# Update Cargo.lock
echo "Updating Cargo.lock..."
cargo check --quiet

echo "✅ Version bumped to ${NEW_VERSION}"
echo ""
echo "Next steps:"
echo "  1. Review changes: git diff"
echo "  2. Commit: git add -A && git commit -m 'chore: bump version to ${NEW_VERSION}'"
echo "  3. Tag: git tag v${NEW_VERSION}"
echo "  4. Push: git push origin main && git push origin v${NEW_VERSION}"

