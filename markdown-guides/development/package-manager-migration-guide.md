# Package Manager Migration Guide: pnpm → bun

## 📋 Overview

This guide explains how to migrate your MudRock Tauri + SvelteKit project from **pnpm** to **bun**, including what needs to change and the differences between package managers.

---

## ✅ Can You Switch? **YES!**

**Short answer**: Yes, you can switch from pnpm to bun with minimal changes. All three package managers (npm, pnpm, bun) read the same `package.json` file, so the migration is straightforward.

---

## 🔄 What Needs to Change?

### **1. Lock Files** ✅ **AUTOMATIC**

**Current state:**
- `pnpm-lock.yaml` (pnpm lock file)
- `package-lock.json` (may exist from npm usage)

**After migration:**
- `bun.lockb` (bun's binary lock file - automatically generated)
- You can delete `pnpm-lock.yaml` after confirming bun works

**Action**: Just run `bun install` - bun will generate its own lock file automatically.

### **2. `.gitignore`** ⚠️ **MINOR UPDATE**

**Current entry:**
```gitignore
.pnpm-store/
```

**Add for bun:**
```gitignore
.pnpm-store/
.bun/              # Bun's cache directory
bun.lockb          # Optional: if you want to ignore lock file (not recommended)
```

**Note**: Unlike pnpm's `.pnpm-store`, bun's `.bun` directory is typically smaller and can be ignored.

### **3. Scripts in `package.json`** ✅ **NO CHANGES NEEDED**

Your current scripts work with bun:
```json
{
  "scripts": {
    "dev": "vite dev",           // ✅ Works with bun
    "build": "vite build",       // ✅ Works with bun
    "tauri:dev": "tauri dev",    // ✅ Works with bun
    "check": "svelte-kit sync && svelte-check --tsconfig ./tsconfig.json"  // ✅ Works with bun
  }
}
```

**Bun can execute npm scripts directly** - no changes needed!

### **4. CI/CD Files** ⚠️ **CHECK IF YOU HAVE THEM**

If you have GitHub Actions, GitLab CI, or other CI/CD pipelines, update them:

**Before (pnpm):**
```yaml
- run: pnpm install
- run: pnpm run build
```

**After (bun):**
```yaml
- uses: oven-sh/setup-bun@v1
  with:
    bun-version: latest
- run: bun install
- run: bun run build
```

**Note**: I didn't find any CI/CD files in your repo, so this may not apply.

### **5. Documentation** ⚠️ **UPDATE IF EXISTS**

Update any README files or documentation that mention pnpm:

**Files to check:**
- `README.md` - Update installation instructions
- `MVP.md` - Update if it mentions pnpm specifically
- Any other `.md` files with installation instructions

**Example update:**
```markdown
# Before
pnpm install
pnpm run dev

# After
bun install
bun run dev
```

### **6. Rust/Tauri Integration** ✅ **NO CHANGES**

Your Rust backend (`src-tauri/Cargo.toml`) is completely independent - no changes needed.

---

## 📊 Package Manager Comparison

### **npm** (Node Package Manager)
- **Speed**: Slowest (sequential installs, no parallelization)
- **Disk Usage**: Highest (duplicates packages in `node_modules`)
- **Lock File**: `package-lock.json` (JSON, human-readable)
- **Workspace Support**: Basic (npm workspaces)
- **Native Tools**: None (pure JavaScript)
- **Best For**: Standard projects, maximum compatibility

### **pnpm** (Performant npm)
- **Speed**: Fast (parallel installs, content-addressable storage)
- **Disk Usage**: Lowest (hard links, shared store at `~/.pnpm-store`)
- **Lock File**: `pnpm-lock.yaml` (YAML, human-readable)
- **Workspace Support**: Excellent (pnpm workspaces)
- **Native Tools**: None (pure JavaScript)
- **Best For**: Monorepos, disk space optimization

### **bun** (All-in-one JavaScript runtime)
- **Speed**: Fastest (native implementation, parallel installs, built-in bundler)
- **Disk Usage**: Medium (similar to npm, but faster installs)
- **Lock File**: `bun.lockb` (binary, not human-readable)
- **Workspace Support**: Good (bun workspaces)
- **Native Tools**: ✅ **Runtime** (can run `.ts`/`.js` files directly), ✅ **Bundler**, ✅ **Test runner**, ✅ **Package manager**
- **Best For**: Fast development, TypeScript-first projects, modern tooling

---

## 🚀 Migration Steps

### **Step 1: Install Bun**

```bash
# macOS/Linux
curl -fsSL https://bun.sh/install | bash

# Windows (PowerShell)
powershell -c "irm bun.sh/install.ps1 | iex"

# Or via npm (if you have it)
npm install -g bun
```

### **Step 2: Backup Current State** (Optional but Recommended)

```bash
# Create a backup branch
git checkout -b backup/pnpm-state

# Commit current state
git add pnpm-lock.yaml
git commit -m "Backup: pnpm lock file before migration to bun"

# Return to main branch
git checkout main
```

### **Step 3: Remove pnpm Lock File**

```bash
# Remove pnpm lock file (bun will create its own)
rm pnpm-lock.yaml

# Optional: Remove pnpm store cache
rm -rf .pnpm-store
```

### **Step 4: Install Dependencies with Bun**

```bash
# Install all dependencies (bun will create bun.lockb)
bun install

# Verify installation
bun run dev  # Test that everything works
```

### **Step 5: Update `.gitignore`**

Add bun-specific entries:
```gitignore
# Bun
.bun/
```

**Note**: `bun.lockb` should typically be **committed** (like `package-lock.json`), so don't ignore it.

### **Step 6: Update Documentation**

Update `README.md` and any other docs:

```markdown
## Installation

```bash
# Install dependencies
bun install

# Run development server
bun run dev

# Build for production
bun run build
```

### **Step 7: Test Everything**

```bash
# Test development
bun run dev

# Test build
bun run build

# Test Tauri
bun run tauri:dev

# Test type checking
bun run check
```

---

## 🎯 Key Differences & Benefits

### **Speed Comparison** (Typical install times)

| Package Manager | Install Time (100 packages) | Cold Start |
|----------------|----------------------------|------------|
| npm            | ~30-60s                    | ~200ms     |
| pnpm           | ~10-20s                    | ~200ms     |
| bun            | ~5-10s                     | ~50ms      |

### **Bun Advantages for Your Project**

1. **⚡ Faster Installs**: Bun installs packages 2-3x faster than pnpm
2. **🚀 Faster Runtime**: Bun can run TypeScript directly (no transpilation step)
3. **🔧 Built-in Tools**: Bun includes bundler, test runner, and package manager in one
4. **📦 Smaller Footprint**: Single binary, no Node.js required for package management
5. **🎯 TypeScript Native**: Can run `.ts` files directly without `ts-node`

### **Potential Considerations**

1. **Lock File Format**: `bun.lockb` is binary (not human-readable like `pnpm-lock.yaml`)
2. **Ecosystem Maturity**: Bun is newer (but very stable for package management)
3. **CI/CD**: May need to update CI scripts (but bun has official GitHub Actions)
4. **Team Adoption**: Team members need to install bun (but it's a one-time setup)

---

## 🔍 Compatibility Check

### **✅ Fully Compatible**

- ✅ `package.json` scripts
- ✅ SvelteKit
- ✅ Vite
- ✅ Tauri CLI
- ✅ TypeScript
- ✅ All npm packages

### **⚠️ May Need Attention**

- ⚠️ **Postinstall Scripts**: Some packages use postinstall hooks - test these
- ⚠️ **Native Modules**: Bun handles these well, but test if you have any
- ⚠️ **Workspace Scripts**: If you use pnpm-specific workspace features, check compatibility

### **Your Specific Stack**

Based on your `package.json`:
- ✅ **SvelteKit**: Fully compatible
- ✅ **Vite**: Fully compatible
- ✅ **Tauri**: Fully compatible (uses npm scripts)
- ✅ **TypeScript**: Fully compatible (bun runs TS natively!)
- ✅ **Tailwind CSS**: Fully compatible
- ✅ **All dependencies**: Should work without issues

---

## 📝 Recommended Migration Checklist

- [ ] Install bun globally
- [ ] Backup current state (optional)
- [ ] Remove `pnpm-lock.yaml`
- [ ] Run `bun install`
- [ ] Update `.gitignore` (add `.bun/`)
- [ ] Test `bun run dev`
- [ ] Test `bun run build`
- [ ] Test `bun run tauri:dev`
- [ ] Update `README.md` with bun commands
- [ ] Update `MVP.md` if it mentions pnpm
- [ ] Commit `bun.lockb` to git
- [ ] Update team documentation

---

## 🎓 Quick Reference: Command Equivalents

| pnpm Command | bun Command | Notes |
|--------------|-------------|-------|
| `pnpm install` | `bun install` | Same behavior |
| `pnpm add <pkg>` | `bun add <pkg>` | Same behavior |
| `pnpm add -D <pkg>` | `bun add -d <pkg>` | Note: `-d` instead of `-D` |
| `pnpm remove <pkg>` | `bun remove <pkg>` | Same behavior |
| `pnpm run <script>` | `bun run <script>` | Same behavior |
| `pnpm exec <cmd>` | `bunx <cmd>` | Bun's equivalent to `npx` |
| `pnpm update` | `bun update` | Same behavior |

---

## 💡 Pro Tips

1. **Keep Both Temporarily**: You can keep both pnpm and bun installed during migration
2. **Test Thoroughly**: Run all your scripts and commands before fully switching
3. **Team Communication**: Let your team know about the switch
4. **CI/CD First**: If you have CI/CD, update it first to catch issues early
5. **Lock File**: Commit `bun.lockb` to ensure consistent installs across team

---

## 🚨 Rollback Plan

If you need to rollback:

```bash
# Restore pnpm lock file from git
git checkout backup/pnpm-state -- pnpm-lock.yaml

# Reinstall with pnpm
pnpm install

# Continue using pnpm
pnpm run dev
```

---

## 📚 Additional Resources

- **Bun Documentation**: https://bun.sh/docs
- **Bun Installation**: https://bun.sh/docs/installation
- **Bun vs npm/pnpm**: https://bun.sh/docs/install#comparison

---

## ✅ Conclusion

**Migration is straightforward** - bun reads the same `package.json` file, so you mainly need to:
1. Install bun
2. Remove `pnpm-lock.yaml`
3. Run `bun install`
4. Update documentation

**Benefits**: Faster installs, faster runtime, built-in TypeScript support, and a modern toolchain.

**Risk**: Low - bun is production-ready and widely used. Your stack (SvelteKit + Tauri) is fully compatible.

