# Release Process

This document describes the automated release process for AI Context Generator.

## Overview

The project uses GitHub Actions to automate:
- ✅ **GitHub Releases**: Automatic creation with changelog
- ✅ **Binary Building**: Multiple Linux platforms (x86_64, musl, aarch64)
- ✅ **crates.io Publishing**: Automatic package publication
- ✅ **Version Verification**: Ensures tag and Cargo.toml match

## How to Release

### 1. Update Version

Update the version in these files:
- `Cargo.toml` (version field)
- `README.md` (dependency examples and changelog)
- `src/lib.rs` (if there are version references)

### 2. Update Changelog

Add a new section in `README.md` under `## 📝 Changelog`:

```markdown
### v0.1.1

- ✨ **Feature**: Description of new features
- 🔧 **Fix**: Bug fixes
- 📚 **Docs**: Documentation improvements
- 🧹 **Refactor**: Code improvements
```

### 3. Commit Changes

```bash
git add Cargo.toml Cargo.lock README.md src/lib.rs
git commit -m "chore: bump version to 0.1.1"
git push origin main
```

### 4. Create Release Tag

```bash
git tag v0.1.1
git push origin v0.1.1
```

**That's it!** The automation will handle the rest.

## What Happens Automatically

### 1. Tests and Validation
- Runs full test suite
- Checks code formatting
- Runs clippy lints
- Verifies version consistency

### 2. Binary Building
- Builds for multiple Linux platforms:
  - `x86_64-unknown-linux-gnu` (standard Linux)
  - `x86_64-unknown-linux-musl` (static binary)
  - `aarch64-unknown-linux-gnu` (ARM64)

### 3. GitHub Release
- Creates a new GitHub release
- Extracts changelog from README.md
- Attaches all binary assets
- Includes installation instructions

### 4. crates.io Publication
- Publishes the new version to crates.io
- Makes it available for `cargo install`

## Setup Requirements

### Secrets Configuration

The repository needs these GitHub secrets configured:

1. **CRATES_IO_TOKEN**: Token for publishing to crates.io
   - Go to [crates.io/me](https://crates.io/me)
   - Generate a new API token
   - Add it to GitHub repository secrets

### Setting up secrets:
1. Go to GitHub repository → Settings → Secrets and variables → Actions
2. Click "New repository secret"
3. Add: `CRATES_IO_TOKEN` with your crates.io API token

## Manual Override

If you need to skip automation or handle something manually:

### Skip crates.io publication:
```bash
# Create tag without triggering full release
git tag v0.1.1-skip-crates
git push origin v0.1.1-skip-crates
```

### Manual crates.io publication:
```bash
cargo publish
```

### Manual GitHub release:
- Go to GitHub releases
- Click "Create a new release"
- Choose the tag and fill in details

## Troubleshooting

### Version mismatch error
- Ensure `Cargo.toml` version matches the git tag
- Tag should be `v0.1.1` for version `0.1.1`

### Build failures
- Check if all tests pass locally: `cargo test`
- Verify formatting: `cargo fmt --check`
- Run clippy: `cargo clippy`

### Publication failures
- Verify CRATES_IO_TOKEN is set correctly
- Check if version already exists on crates.io
- Ensure all required fields are in Cargo.toml

## Benefits of This Automation

- 🚀 **Fast releases**: One command triggers everything
- 🔒 **Consistent process**: No manual steps to forget
- 📦 **Multiple formats**: Binaries for different platforms
- 📝 **Automatic documentation**: Changelog extraction
- ✅ **Quality gates**: Tests must pass before release
- 🔄 **Atomic releases**: All or nothing deployment
