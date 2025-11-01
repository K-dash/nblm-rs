#!/usr/bin/env bash
# Updates the Homebrew tap Formula/nblm.rb and opens a PR via gh.
#
# Example usage:
#   ./scripts/update_homebrew.sh \
#     --version 0.3.0 \
#     --tap-dir ../homebrew-nblm

set -euo pipefail

usage() {
  cat <<'USAGE'
Usage: ./scripts/update_homebrew.sh \
  --version <version> \
  [--tap-dir <path>] \
  [--remote <remote>] \
  [--base-branch <branch>] \
  [--pr-branch <branch>] \
  [--no-pr]

The script fetches release assets from GitHub, calculates SHA256,
updates Formula/nblm.rb, commits, pushes, and opens a PR via gh.
USAGE
}

TAP_DIR="../homebrew-nblm"
REMOTE="origin"
BASE_BRANCH="main"
PR_BRANCH=""
CREATE_PR=1

VERSION=""

while [[ $# -gt 0 ]]; do
  case "$1" in
    --version)
      VERSION="$2"
      shift 2
      ;;
    --tap-dir)
      TAP_DIR="$2"
      shift 2
      ;;
    --remote)
      REMOTE="$2"
      shift 2
      ;;
    --base-branch)
      BASE_BRANCH="$2"
      shift 2
      ;;
    --pr-branch)
      PR_BRANCH="$2"
      shift 2
      ;;
    --no-pr)
      CREATE_PR=0
      shift 1
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "Unknown argument: $1" >&2
      usage >&2
      exit 1
      ;;
  esac
done

if [[ -z "$VERSION" ]]; then
  echo "Error: --version is required" >&2
  usage >&2
  exit 1
fi

if [[ -z "$PR_BRANCH" ]]; then
  PR_BRANCH="update-nblm-v${VERSION}"
fi

for cmd in git gh python3; do
  if ! command -v "$cmd" >/dev/null 2>&1; then
    echo "Error: required command '$cmd' not found" >&2
    exit 1
  fi
done

if ! gh auth status >/dev/null 2>&1; then
  echo "Error: gh is not authenticated. Run 'gh auth login' first." >&2
  exit 1
fi

if [[ ! -d "$TAP_DIR/.git" ]]; then
  echo "Error: tap directory '$TAP_DIR' is not a git repository" >&2
  exit 1
fi

echo "Fetching release info for tag v${VERSION}..."
RELEASE_JSON=$(gh api "repos/K-dash/nblm-rs/releases/tags/v${VERSION}")

MAC_ARM_URL=$(echo "$RELEASE_JSON" | python3 -c "
import sys, json
data = json.load(sys.stdin)
for asset in data.get('assets', []):
    if 'nblm-macos-aarch64.tar.gz' in asset['name']:
        print(asset['browser_download_url'])
        break
")

MAC_INTEL_URL=$(echo "$RELEASE_JSON" | python3 -c "
import sys, json
data = json.load(sys.stdin)
for asset in data.get('assets', []):
    if 'nblm-macos-x86_64.tar.gz' in asset['name']:
        print(asset['browser_download_url'])
        break
")

if [[ -z "$MAC_ARM_URL" ]]; then
  echo "Error: could not find macOS ARM64 asset in release v${VERSION}" >&2
  exit 1
fi

if [[ -z "$MAC_INTEL_URL" ]]; then
  echo "Error: could not find macOS Intel asset in release v${VERSION}" >&2
  exit 1
fi

echo "Found ARM64 binary: $MAC_ARM_URL"
echo "Found Intel binary: $MAC_INTEL_URL"

TEMP_DIR=$(mktemp -d)
trap "rm -rf $TEMP_DIR" EXIT

echo "Downloading ARM64 binary..."
curl -sSL "$MAC_ARM_URL" -o "$TEMP_DIR/arm64.tar.gz"
MAC_ARM_SHA=$(shasum -a 256 "$TEMP_DIR/arm64.tar.gz" | awk '{print $1}')
echo "ARM64 SHA256: $MAC_ARM_SHA"

echo "Downloading Intel binary..."
curl -sSL "$MAC_INTEL_URL" -o "$TEMP_DIR/intel.tar.gz"
MAC_INTEL_SHA=$(shasum -a 256 "$TEMP_DIR/intel.tar.gz" | awk '{print $1}')
echo "Intel SHA256: $MAC_INTEL_SHA"

pushd "$TAP_DIR" >/dev/null

if [[ -n $(git status --porcelain) ]]; then
  echo "Error: tap repository has uncommitted changes. Commit or stash them first." >&2
  popd >/dev/null
  exit 1
fi

git fetch "$REMOTE" "$BASE_BRANCH" --quiet
git checkout "$BASE_BRANCH" >/dev/null
git pull --ff-only "$REMOTE" "$BASE_BRANCH"

if git rev-parse --verify --quiet "$PR_BRANCH" >/dev/null; then
  echo "Error: branch '$PR_BRANCH' already exists locally. Pick a different --pr-branch or delete it." >&2
  popd >/dev/null
  exit 1
fi

if git ls-remote --exit-code --heads "$REMOTE" "$PR_BRANCH" >/dev/null 2>&1; then
  echo "Error: branch '$PR_BRANCH' already exists on remote '$REMOTE'. Choose a different --pr-branch." >&2
  popd >/dev/null
  exit 1
fi

git checkout -b "$PR_BRANCH" "$REMOTE/$BASE_BRANCH" >/dev/null

FORMULA_PATH="Formula/nblm.rb"
if [[ ! -f "$FORMULA_PATH" ]]; then
  echo "Error: formula file '$FORMULA_PATH' not found" >&2
  popd >/dev/null
  exit 1
fi

export FORMULA_PATH VERSION MAC_ARM_URL MAC_ARM_SHA MAC_INTEL_URL MAC_INTEL_SHA

python3 <<'PYTHON'
import os
import pathlib
import re

formula_path = pathlib.Path(os.environ["FORMULA_PATH"])
text = formula_path.read_text()

version = os.environ["VERSION"]
mac_arm_url = os.environ["MAC_ARM_URL"]
mac_arm_sha = os.environ["MAC_ARM_SHA"]
mac_intel_url = os.environ["MAC_INTEL_URL"]
mac_intel_sha = os.environ["MAC_INTEL_SHA"]

lines = text.splitlines()

def with_indent(original_line: str, content: str) -> str:
    indent = re.match(r"^\s*", original_line).group(0)
    return f"{indent}{content}"

found_version = False
found_arm = False
found_intel = False

for idx, line in enumerate(lines):
    stripped = line.strip()
    if stripped.startswith("version ") and not found_version:
        lines[idx] = with_indent(line, f'version "{version}"')
        found_version = True
    elif "url " in stripped and "nblm-macos-aarch64" in stripped and not found_arm:
        lines[idx] = with_indent(line, f'url "{mac_arm_url}"')
        # find sha line that follows the url
        sha_idx = idx + 1
        while sha_idx < len(lines) and lines[sha_idx].strip() == "":
            sha_idx += 1
        if sha_idx >= len(lines) or not lines[sha_idx].strip().startswith("sha256"):
            raise SystemExit("Failed to locate sha256 line for macOS arm64 block")
        lines[sha_idx] = with_indent(lines[sha_idx], f'sha256 "{mac_arm_sha}"')
        found_arm = True
    elif "url " in stripped and "nblm-macos-x86_64" in stripped and not found_intel:
        lines[idx] = with_indent(line, f'url "{mac_intel_url}"')
        sha_idx = idx + 1
        while sha_idx < len(lines) and lines[sha_idx].strip() == "":
            sha_idx += 1
        if sha_idx >= len(lines) or not lines[sha_idx].strip().startswith("sha256"):
            raise SystemExit("Failed to locate sha256 line for macOS intel block")
        lines[sha_idx] = with_indent(lines[sha_idx], f'sha256 "{mac_intel_sha}"')
        found_intel = True


if not found_version:
    raise SystemExit("Failed to update version line")
if not found_arm:
    raise SystemExit("Failed to update macOS arm block")
if not found_intel:
    raise SystemExit("Failed to update macOS intel block")

updated_text = "\n".join(lines)
if text.endswith("\n"):
    updated_text += "\n"

formula_path.write_text(updated_text)
PYTHON

git add "$FORMULA_PATH"

if git diff --cached --quiet; then
  echo "Error: no changes detected after editing formula" >&2
  popd >/dev/null
  exit 1
fi

COMMIT_MSG="chore: update nblm formula to v${VERSION}"
git commit -m "$COMMIT_MSG"

git push -u "$REMOTE" "$PR_BRANCH"

if [[ "$CREATE_PR" -eq 1 ]]; then
  gh pr create \
    --base "$BASE_BRANCH" \
    --head "$PR_BRANCH" \
    --title "Update nblm formula to v${VERSION}" \
    --body "Update the nblm Homebrew formula to version v${VERSION}."
else
  echo "Skipping PR creation (--no-pr provided)."
fi

popd >/dev/null

echo "✅ Formula update complete"
