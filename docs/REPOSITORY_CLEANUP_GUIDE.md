# Repository Cleanup Guide

This document outlines the cleanup performed on the Gatherraa repository to reduce clone size and improve developer experience.

## Overview

The repository underwent a comprehensive cleanup to:
- Remove lock files from git history
- Remove node_modules and build artifacts from history
- Improve .gitignore configuration
- Reorganize documentation and tests

## Changes Made

### 1. **History Cleanup** ✅
- **Removed from git history:**
  - All lock files (package-lock.json, pnpm-lock.yaml, yarn.lock, Cargo.lock)
  - node_modules directories (from contract folder)
  - Build outputs (dist/, build/, .next/, artifacts/, cache/)
  - Generated and cache files

- **Repository size reduction:**
  - Before: 770MB
  - After: 619MB
  - **Saved: 151MB (~20% reduction)**

### 2. **File Organization** ✅
Moved to appropriate directories:
- Documentation: Root `*.md` files → `docs/`
- Load tests: Root `.js` files → `tests/k6-load-tests/`
- Docker configs: Alternative configs → `ops/docker-configs/`

### 3. **.gitignore Files Enhanced** ✅
Updated with comprehensive rules for:
- All lock files across all package managers
- Build outputs and compilation artifacts
- IDE and editor files
- Environment files and secrets
- Temporary files and caches
- Logs and diagnostic reports
- OS-specific files

### 4. **Clean Directory Structure** ✅
- Root directory: 28 files/folders (cleaned up from ~40)
- Proper separation of concerns
- Documentation consolidated

## What Contributors Need to Know

### Before Cloning

**IMPORTANT:** The commit hashes have changed due to history rewriting. If you have an existing clone:

```bash
# Option 1: Fresh clone (recommended)
rm -rf Gatherraa
git clone https://github.com/Gatheraa/Gatherraa.git
cd Gatherraa

# Option 2: Sync existing clone
git fetch origin main
git reset --hard origin/main
```

### Initial Setup

When you clone the repository, **YOU MUST REGENERATE lock files**:

```bash
# Frontend
cd app/frontend
npm install  # or: pnpm install / yarn install

# Backend
cd app/backend
npm install  # or: pnpm install / yarn install

# Contracts
cd contract
npm install  # or: pnpm install / yarn install
cargo build  # For Rust contracts
```

**Location:** Each folder has its own `package.json` or `Cargo.toml`. Lock files will be created locally but NOT committed.

### What's NOT Committed Now

These are regenerated during setup:
- ✗ `package-lock.json` (frontend, backend)
- ✗ `pnpm-lock.yaml`
- ✗ `yarn.lock`
- ✗ `Cargo.lock`
- ✗ `node_modules/`
- ✗ Build outputs (dist/, build/, .next/)

### How to Install Dependencies

```bash
# Install all dependencies at once
npm ci --workspaces  # For the whole monorepo

# Or install separately
cd app/backend && npm install
cd app/frontend && npm install
cd contract && npm install
```

## Best Practices Going Forward

### ✅ DO:
1. **Commit only source code** (`.ts`, `.tsx`, `.js`, `.rs`, `.sol`, `.md`)
2. **Regenerate lock files during setup** (or use CI/CD pinned versions)
3. **Use environment files** (.env.example) not .env files
4. **Run builds locally** before committing
5. **Review .gitignore** before adding new dependencies

### ❌ DON'T:
1. **Commit lock files** - they're auto-generated
2. **Commit node_modules** or dependencies
3. **Commit build outputs** (dist/, .next/, artifacts/)
4. **Commit environment files with secrets** (.env, .env.local)
5. **Commit logs or temporary files**

## For Maintainers

### Preventing Future Bloat

The enhanced `.gitignore` prevents most common issues. To add new patterns:

```bash
# Global patterns (root .gitignore)
echo "new_pattern/" >> .gitignore

# Frontend specific
echo "new_pattern/" >> app/frontend/.gitignore

# Backend specific
echo "new_pattern/" >> app/backend/.gitignore

# Contracts specific
echo "new_pattern/" >> contract/.gitignore
```

### Checking Repository Health

```bash
# Show git repository size
du -sh .git

# Count objects in history
git count-objects -v

# List largest files in working directory
find . -type f -size +10M

# Check for accidentally committed node_modules
git ls-files | grep node_modules
```

### Future Cleanup

If node_modules or build artifacts get committed:

```bash
# Install git-filter-repo
pip install git-filter-repo

# Create patterns file (patterns.txt)
echo "node_modules/" > patterns.txt
echo "dist/" >> patterns.txt

# Run the filter (WARNING: rewrites history)
git-filter-repo --invert-paths --paths-from-file patterns.txt --force

# Garbage collection
git gc --aggressive --prune=now
```

## Summary

| Metric | Before | After | Change |
|--------|--------|-------|--------|
| Repository Size | 770MB | 619MB | -151MB (-20%) |
| Root Files/Folders | ~40 | 28 | -12 (-30%) |
| Lock Files | 8 | 0 | removed ✅ |
| Documentation | Root scattered | docs/ | organized ✅ |
| Tests | Root scattered | tests/ | organized ✅ |

## Related Documentation

- [README.md](../README.md) - Main project documentation
- [docs/](../docs/) - Comprehensive guides and documentation
- [tests/k6-load-tests/README.md](../tests/k6-load-tests/README.md) - Load testing setup
- [.gitignore](../.gitignore) - File patterns to exclude

---

**Last Updated:** March 21, 2026  
**Cleanup Tool:** git-filter-repo  
**Git Version:** 2.x  
**Status:** ✅ Complete
