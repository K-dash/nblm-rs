#!/usr/bin/env bash
set -euo pipefail

# Usage: ./scripts/bump-version.sh

# Fetch latest tags from remote
echo "Fetching latest tags from remote..."
git fetch --tags --quiet

# Get the latest tag version
LATEST_TAG=$(git tag -l 'v*' | sort -V | tail -n 1)
if [ -z "$LATEST_TAG" ]; then
  echo "No existing tags found."
  LATEST_VERSION="none"
else
  LATEST_VERSION="${LATEST_TAG#v}"
  echo "Current latest tag: ${LATEST_TAG} (${LATEST_VERSION})"
fi

# Prompt for new version
echo ""
read -p "Enter new version: " NEW_VERSION

if [ -z "$NEW_VERSION" ]; then
  echo "Error: Version cannot be empty"
  exit 1
fi

echo "Bumping version to ${NEW_VERSION}..."

# Rust crates
echo "Updating Rust crates..."
sed -i.bak "s/^version = \".*\"/version = \"${NEW_VERSION}\"/" Cargo.toml
sed -i.bak "s/^version = \".*\"/version = \"${NEW_VERSION}\"/" crates/nblm-core/Cargo.toml
sed -i.bak "s/^version = \".*\"/version = \"${NEW_VERSION}\"/" crates/nblm-cli/Cargo.toml
sed -i.bak "s/^version = \".*\"/version = \"${NEW_VERSION}\"/" crates/nblm-python/Cargo.toml

# Update nblm-core dependency version in nblm-cli
echo "Updating nblm-core dependency in nblm-cli..."
sed -i.bak "s/^nblm-core = { version = \"[^\"]*\"/nblm-core = { version = \"${NEW_VERSION}\"/" crates/nblm-cli/Cargo.toml

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

