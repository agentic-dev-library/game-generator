# Scripts Directory

This directory contains utility scripts for development and CI verification.

## Available Scripts

### verify-ci-setup.sh

Verifies that your branch has the correct CI configuration and will pass GitHub Actions workflows.

**Usage:**
```bash
./scripts/verify-ci-setup.sh
```

**What it checks:**
1. ✓ Workflow file exists (`.github/workflows/rust.yml`)
2. ✓ ALSA dependencies configured in workflow
3. ✓ System libraries installed (local builds only)
4. ✓ Code formatting with rustfmt
5. ✓ Cargo check passes
6. ✓ Clippy lints pass

**When to use:**
- Before pushing changes to a feature branch
- After rebasing against main
- When debugging CI failures
- Before opening a new PR

**Example output:**
```
=== Vintage Game Generator CI Setup Verification ===

Checking for workflow file... ✓ Found
Checking for ALSA dependency installation... ✓ Configured
Checking for ALSA development libraries... ⚠ Not installed (OK in CI, needed for local builds)
  To install: sudo apt-get install -y libasound2-dev libudev-dev
Checking code formatting... ✓ Properly formatted
Running cargo check... ✓ No build errors
Running clippy... ✓ No clippy warnings

=== All checks passed! ===
Your branch should pass CI workflows.
```

**Common issues and fixes:**

| Issue | Fix |
|-------|-----|
| Workflow file missing | `git rebase origin/main` |
| ALSA not configured | `git rebase origin/main` |
| Formatting errors | `cargo fmt --all` |
| Build errors | Review error messages and fix code |
| Clippy warnings | `cargo clippy --all-features --fix` |

## Adding New Scripts

When adding new scripts:
1. Make them executable: `chmod +x scripts/your-script.sh`
2. Add a shebang line: `#!/usr/bin/env bash`
3. Document them in this README
4. Use meaningful error messages
5. Follow existing script conventions
