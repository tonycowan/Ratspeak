#!/usr/bin/env bash
# Dispatch desktop/mobile release workflows without publishing a GitHub Release.
# Requires: gh auth login (repo scope on tonycowan/Ratspeak or your fork).
set -euo pipefail

REPO="${RATSPEAK_GITHUB_REPO:-tonycowan/Ratspeak}"
REF="${RATSPEAK_RELEASE_REF:-feat/codec2-support}"

workflows=(
  "Release Linux"
  "Release macOS"
  "Release Windows"
  "Release Android"
)

if ! command -v gh >/dev/null 2>&1; then
  echo "gh CLI is required. Install from https://cli.github.com/" >&2
  exit 1
fi

if ! gh auth status >/dev/null 2>&1; then
  echo "Run: gh auth login" >&2
  exit 1
fi

for name in "${workflows[@]}"; do
  echo "Dispatching: $name (ref=$REF)"
  gh workflow run "$name" \
    --repo "$REPO" \
    --ref "$REF" \
    -f publish_github_release=false
done

echo
echo "Watch progress: gh run list --repo $REPO --limit 10"
