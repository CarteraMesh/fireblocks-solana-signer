#!/bin/bash

# Local release script for testing cargo-release configuration

set -e

BUMP_TYPE=${1:-patch}

if [[ ! "$BUMP_TYPE" =~ ^(patch|minor|major)$ ]]; then
    echo "❌ Invalid bump type: $BUMP_TYPE"
    echo "Usage: $0 [patch|minor|major]"
    exit 1
fi

echo "🔍 Running local release test with '$BUMP_TYPE' bump"
echo "=================================================="

# Show current version info
./scripts/version-info.sh

echo ""
echo "🧪 Running pre-release checks..."

# Check that it compiles
echo "  ✅ Checking compilation..."
cargo check --all-features --quiet

# Check formatting
echo "  ✅ Checking formatting..."
cargo +nightly fmt --all -- --check

# Run clippy
echo "  ✅ Running clippy..."
cargo clippy --all-features --quiet -- -D warnings

# Run only doc tests (fast)
echo "  ✅ Running doc tests..."
cargo test --doc --quiet

echo ""
echo "ℹ️  Note: Skipping integration tests (use 'cargo test' manually if needed)"

echo ""
echo "🚀 Running cargo-release dry run..."
cargo release --dry-run --no-confirm $BUMP_TYPE

echo ""
echo "✅ Dry run completed successfully!"
echo "To actually release, run: cargo release --execute --no-confirm $BUMP_TYPE"
