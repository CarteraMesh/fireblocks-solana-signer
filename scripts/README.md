# Release Scripts

This directory contains helper scripts for managing releases.

## Scripts

### `version-info.sh`
Shows current version and potential next versions for patch, minor, and major bumps.

```bash
./scripts/version-info.sh
```

### `local-release.sh`
Runs a local dry-run of the release process to test everything works before triggering the GitHub Action.

```bash
# Test patch release (default)
./scripts/local-release.sh

# Test minor release
./scripts/local-release.sh minor

# Test major release
./scripts/local-release.sh major
```

## GitHub Action Release

To trigger a release via GitHub Actions:

1. Go to the "Actions" tab in your GitHub repository
2. Select the "Release" workflow
3. Click "Run workflow"
4. Choose your version bump type (patch/minor/major)
5. Optionally enable "dry run" to test without publishing

## Manual Release

If you prefer to release manually:

```bash
# Run tests and checks first
./scripts/local-release.sh patch

# Then execute the actual release
cargo release --execute --no-confirm patch
```
