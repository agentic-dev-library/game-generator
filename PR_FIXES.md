# PR Workflow Failure Resolution Guide

## Summary

All open PRs (#2, #3, #4) are experiencing CI failures because they were created before the GitHub Actions workflow was properly configured with system dependencies. This document provides instructions for fixing each PR.

## Root Cause Analysis

### Issue 1: Missing ALSA Dependencies (All PRs)
**Symptom**: Build failures with error:
```
The system library `alsa` required by crate `alsa-sys` was not found.
```

**Cause**: The feature branches don't have the `.github/workflows/rust.yml` file that was added to `main` branch. This workflow file contains the necessary steps to install ALSA development libraries.

**Impact**: Affects all CI jobs (Check, Test, Clippy, Documentation)

### Issue 2: Formatting Errors (PR #4 only)
**Symptom**: Format job failing with whitespace/newline differences

**Files affected**:
- `vintage_game_generator/tests/test_harness.rs`
- `vintage_game_generator/tests/test_harness_macos.rs`

**Cause**: Code not formatted with `rustfmt`

## Resolution Steps

### For PR #2: Add vintage_blending_core crate
```bash
# Checkout the PR branch
git checkout feature/add-blending-core

# Rebase against main to get the workflow file
git fetch origin
git rebase origin/main

# Force push (since rebase rewrites history)
git push --force-with-lease origin feature/add-blending-core
```

### For PR #3: Add vintage_build_tools crate
```bash
# Checkout the PR branch
git checkout feature/add-build-tools

# Rebase against main to get the workflow file
git fetch origin
git rebase origin/main

# Force push (since rebase rewrites history)
git push --force-with-lease origin feature/add-build-tools
```

### For PR #4: Add complete vintage_game_generator workspace
```bash
# Checkout the PR branch
git checkout feature/add-generator-app

# Rebase against main to get the workflow file
git fetch origin
git rebase origin/main

# Fix formatting issues
cargo fmt --all

# Commit the formatting fixes
git add .
git commit -m "fix: apply rustfmt formatting"

# Force push (since rebase rewrites history)
git push --force-with-lease origin feature/add-generator-app
```

## What the Workflow Fix Does

The updated `.github/workflows/rust.yml` file on `main` branch includes:

```yaml
- name: Install system dependencies
  run: |
    sudo apt-get update
    sudo apt-get install -y libasound2-dev libudev-dev
```

This installs:
- `libasound2-dev`: ALSA (Advanced Linux Sound Architecture) development files
- `libudev-dev`: udev library development files

These are required because:
1. The `bevy` game engine (used by `vintage_game_generator`) depends on audio capabilities
2. The `alsa-sys` crate provides Rust bindings to ALSA
3. Without the development headers, the build system cannot link to the ALSA library

## Verification

After applying the fixes, the CI workflows should:
1. ✅ Install system dependencies successfully
2. ✅ Build all crates without ALSA-related errors
3. ✅ Pass all Clippy checks
4. ✅ Pass formatting checks (after running `cargo fmt`)
5. ✅ Generate documentation successfully
6. ✅ Run all tests successfully

## Alternative Approach (If Rebase is Problematic)

If rebasing causes conflicts, you can manually copy the workflow file:

```bash
# Checkout your feature branch
git checkout feature/add-XXX

# Get the workflow file from main
git checkout origin/main -- .github/workflows/rust.yml

# Commit it
git add .github/workflows/rust.yml
git commit -m "chore: add CI workflow with system dependencies"

# Push
git push origin feature/add-XXX
```

## Notes

- The workflow file is already correctly configured on `main` branch
- This is a one-time fix - once the branches are updated, future PRs will inherit the correct workflow
- The WASM build job is intentionally disabled because the crates use tokio/reqwest which require native networking

## Questions?

If you encounter issues applying these fixes, please comment on your PR or reach out for assistance.
