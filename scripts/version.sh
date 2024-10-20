#!/usr/bin/env bash

# ================================================================= #
#  Version Script: Bump the project version, create a git tag and push the changes:
#  - check repo is clean
#  - get latest version from git tags
#  - increment version based on VERSION_TYPE
#  - update version in Cargo.toml & Cargo.lock
#  - tag commit & push changes
# ================================================================= #

set -euo pipefail

if [ -n "$(git status --porcelain)" ]; then
  printf "\nError: repo has uncommitted changes\n\n"
  exit 1
fi

# =================================================================
# Get the latest git tag version and increment it
# =================================================================

GIT_TAG_LATEST=$(git tag --sort=v:refname | tail -n 1)

# If no tags found, default to v0.0.0
if [ -z "$GIT_TAG_LATEST" ]; then
  GIT_TAG_LATEST="v0.0.0"
fi

GIT_TAG_LATEST=$(echo "$GIT_TAG_LATEST" | sed 's/^v//') # Remove prefix "v" from tag

VERSION_TYPE="${1-}" # From the first argument passed
VERSION_NEXT=""

if [ "$VERSION_TYPE" = "patch" ]; then
  VERSION_NEXT="$(echo "$GIT_TAG_LATEST" | awk -F. '{$NF++; print $1"."$2"."$NF}')"
elif [ "$VERSION_TYPE" = "minor" ]; then
  VERSION_NEXT="$(echo "$GIT_TAG_LATEST" | awk -F. '{$2++; $3=0; print $1"."$2"."$3}')"
elif [ "$VERSION_TYPE" = "major" ]; then
  VERSION_NEXT="$(echo "$GIT_TAG_LATEST" | awk -F. '{$1++; $2=0; $3=0; print $1"."$2"."$3}')"
else
  printf "\nError: invalid VERSION_TYPE arg passed, must be patch, minor or major\n\n"
  exit 1
fi

# =================================================================
# Update version in Cargo.toml and commit it
# =================================================================

# echo "Next: $VERSION_NEXT"
# sed -i "s/^version = .*/version = \"$VERSION_NEXT\"/" Cargo.toml
sed -i '' "s/^version = .*/version = \"$VERSION_NEXT\"/" Cargo.toml
cargo generate-lockfile # Update version in Cargo.lock
git add .
git commit -m "build: bump version to v$VERSION_NEXT"

# =================================================================
# Create new git tag and push it
# =================================================================

echo "Tagging Commit & Pushing Changes: v$VERSION_NEXT"
git tag -a "v$VERSION_NEXT" -m "Release: v$VERSION_NEXT"
git push -u origin main --follow-tags
