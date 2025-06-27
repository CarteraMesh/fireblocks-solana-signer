#!/bin/bash

# Script to show current version and what the next versions would be

set -e

echo "🔍 Version Information"
echo "====================="

# Get current version using cargo metadata
CURRENT_VERSION=$(cargo metadata --no-deps --format-version 1 | jq -r '.packages[0].version')
echo "📦 Current version: $CURRENT_VERSION"

# Parse version components
IFS='.' read -r major minor patch <<< "$CURRENT_VERSION"

# Calculate potential new versions
PATCH_VERSION="$major.$minor.$((patch + 1))"
MINOR_VERSION="$major.$((minor + 1)).0"
MAJOR_VERSION="$((major + 1)).0.0"

echo ""
echo "🚀 Potential releases:"
echo "  patch: $CURRENT_VERSION → $PATCH_VERSION"
echo "  minor: $CURRENT_VERSION → $MINOR_VERSION"
echo "  major: $CURRENT_VERSION → $MAJOR_VERSION"

echo ""
echo "📋 Alternative methods to get version:"
echo "  grep: $(grep '^version' Cargo.toml | head -1)"
echo "  cargo pkgid: $(cargo pkgid | cut -d'#' -f2)"
