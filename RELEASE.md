# Release Process

This document explains how to release new versions of `fireblocks-solana-signer`.

## Prerequisites

- You have `cargo-release` installed: `cargo install cargo-release`
- You have push access to the repository
- You have a `CARGO_REGISTRY_TOKEN` configured (for crates.io publishing)

## Release Methods

### 1. GitHub Actions (Recommended)

The easiest way to release is via GitHub Actions:

1. Go to the **Actions** tab in your GitHub repository
2. Select the **Release** workflow
3. Click **Run workflow**
4. Choose your version bump type:
   - **patch**: Bug fixes (0.1.0 → 0.1.1)
   - **minor**: New features (0.1.0 → 0.2.0) 
   - **major**: Breaking changes (0.1.0 → 1.0.0)
5. Optionally enable **dry run** to test without publishing
6. Click **Run workflow**

The workflow will:
- Run all tests and checks
- Update version numbers in README.md
- Create a git tag
- Publish to crates.io
- Create a GitHub release

### 2. Local Release

For local releases or testing:

```bash
# Test the release process (recommended first)
./scripts/local-release.sh patch

# If everything looks good, commit your changes first
git add .
git commit -m "chore: prepare for release"

# Then run the actual release
cargo release --execute --no-confirm patch
```

### 3. Manual Steps

If you prefer full control:

```bash
# 1. Run pre-release checks
./scripts/local-release.sh patch

# 2. Commit any changes
git add .
git commit -m "chore: prepare for release"

# 3. Update version and publish
cargo release --execute --no-confirm patch

# 4. Push changes and tags
git push origin main --tags
```

## What Happens During Release

1. **Pre-release checks**: Tests, formatting, clippy
2. **Version bump**: Updates Cargo.toml version
3. **Documentation updates**: Updates version numbers in README.md
4. **Git operations**: Creates commit and tag
5. **Publishing**: Uploads to crates.io
6. **GitHub release**: Creates release with notes

## Version Strategy

- **Patch** (0.1.0 → 0.1.1): Bug fixes, documentation updates
- **Minor** (0.1.0 → 0.2.0): New features, non-breaking changes
- **Major** (0.1.0 → 1.0.0): Breaking changes, API changes

## Troubleshooting

### Tests Take Too Long
The integration tests interact with Fireblocks and can be slow. The local release script skips them by default. Run full tests manually if needed:

```bash
cargo test --all-features
```

### Formatting Issues
Make sure to use nightly Rust for formatting:

```bash
cargo +nightly fmt --all
```

### Uncommitted Changes
Commit all changes before releasing:

```bash
git add .
git commit -m "chore: prepare for release"
```

### Missing Metadata
Ensure Cargo.toml has all required fields:
- description
- license
- repository
- documentation

## Configuration Files

- **release.toml**: cargo-release configuration
- **.github/workflows/release.yml**: GitHub Actions workflow
- **scripts/**: Helper scripts for local development
