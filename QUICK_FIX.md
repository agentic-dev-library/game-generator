# Quick Fix Guide for Failing PRs

> **TL;DR**: All open PRs need to rebase against `main` to get the CI workflow file.

## The Problem

PRs #2, #3, and #4 are failing CI because they don't have the `.github/workflows/rust.yml` file that was added after those branches were created.

## The Solution

### For PR #2 and #3
```bash
git fetch origin
git rebase origin/main
git push --force-with-lease
```

### For PR #4 (needs formatting fix too)
```bash
git fetch origin
git rebase origin/main
cargo fmt --all
git add .
git commit -m "fix: apply rustfmt formatting"
git push --force-with-lease
```

## Verify Before Pushing (Optional)

```bash
./scripts/verify-ci-setup.sh
```

This script checks:
- ✓ Workflow file exists
- ✓ ALSA deps configured
- ✓ Code formatted correctly
- ✓ Build succeeds
- ✓ Clippy passes

## What Changed?

The `main` branch now has a complete CI workflow (`.github/workflows/rust.yml`) that:
1. Installs ALSA development libraries (`libasound2-dev`, `libudev-dev`)
2. Runs all checks (build, test, clippy, docs, formatting)
3. Uses Rust caching for faster builds

## Need More Details?

See `PR_FIXES.md` for comprehensive instructions and troubleshooting.

## Questions?

Post a comment on your PR or contact the maintainers.

---

**Last Updated**: 2025-12-18  
**Affected PRs**: #2, #3, #4  
**Fix Time**: ~5 minutes per PR
