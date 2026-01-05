# Best Practices for Maintaining Import Paths in JS/TS Projects

This document outlines best practices for managing import paths in JavaScript and TypeScript projects, with a focus on handling file moves and refactoring.

## Table of Contents

1. [Path Aliases vs Relative Paths](#path-aliases-vs-relative-paths)
2. [Configuration Best Practices](#configuration-best-practices)
3. [Handling File Moves](#handling-file-moves)
4. [Tooling and Automation](#tooling-and-automation)
5. [Project-Specific Recommendations](#project-specific-recommendations)

---

## Path Aliases vs Relative Paths

### When to Use Path Aliases

**Use path aliases for:**
- ✅ Shared utilities and libraries (`$lib`, `@/utils`, `@/components`)
- ✅ Frequently accessed directories
- ✅ Cross-cutting concerns (constants, types, configs)
- ✅ Stable, well-defined project structure

**Use relative paths for:**
- ✅ Files in the same directory or nearby directories
- ✅ Co-located files (components and their styles/tests)
- ✅ Temporary or experimental code
- ✅ Files that are likely to move together

### Example Comparison

```typescript
// ❌ BAD: Deep relative paths are fragile
import { Button } from '../../../../components/ui/button'
import { formatDate } from '../../../utils/date'

// ✅ GOOD: Path aliases are stable
import { Button } from '$lib/components/ui/button'
import { formatDate } from '$lib/utils/date'

// ✅ ALSO GOOD: Relative paths for nearby files
import { helper } from './helper'
import { types } from '../types'
```

---

## Configuration Best Practices

### 1. TypeScript Configuration (`tsconfig.json`)

Always configure path aliases in `tsconfig.json` for IDE support and type checking:

```json
{
  "compilerOptions": {
    "baseUrl": ".",
    "moduleResolution": "bundler", // or "nodenext" for Node.js projects
    "paths": {
      "$lib": ["./src/lib"],
      "$lib/*": ["./src/lib/*"],
      "@/*": ["./src/*"],
      "@components/*": ["./src/lib/components/*"],
      "@utils/*": ["./src/lib/utils/*"]
    }
  }
}
```

**Key Points:**
- `baseUrl` is required for path mapping to work
- Use wildcard patterns (`*`) for subdirectory matching
- Match patterns in both TypeScript and build tool configs

### 2. Vite Configuration (`vite.config.ts`)

Configure matching aliases in Vite for runtime resolution:

```typescript
import path from 'path'
import { defineConfig } from 'vite'

export default defineConfig({
  resolve: {
    alias: {
      '$lib': path.resolve(__dirname, './src/lib'),
      '@': path.resolve(__dirname, './src'),
      '@components': path.resolve(__dirname, './src/lib/components'),
      '@utils': path.resolve(__dirname, './src/lib/utils')
    }
  }
})
```

**Important:** The aliases in `vite.config.ts` must match those in `tsconfig.json` for consistency.

### 3. SvelteKit Configuration (`svelte.config.js`)

For SvelteKit projects, also configure aliases in `svelte.config.js`:

```javascript
const config = {
  kit: {
    alias: {
      $lib: './src/lib',
      '$lib/*': './src/lib/*',
      '@': './src',
      '@/*': './src/*'
    }
  }
}
```

---

## Handling File Moves

### Strategy 1: Use Path Aliases (Recommended)

**Before moving:**
```typescript
// some/path/to/folder/file.ts
import { helper } from './helper'
import { utils } from '../../utils'
```

**After moving to `another/path/in/another/folder/file.ts`:**

If using path aliases:
```typescript
// No changes needed if using aliases!
import { helper } from './helper'  // Still works (relative)
import { utils } from '@utils'     // Still works (alias)
```

If using relative paths:
```typescript
// ❌ BROKEN: Relative paths need manual updates
import { helper } from './helper'
import { utils } from '../../utils'  // Wrong path now!

// ✅ FIXED: Update relative paths manually
import { helper } from './helper'
import { utils } from '../../../utils'  // Updated path
```

### Strategy 2: Automated Refactoring Tools

#### VS Code / Cursor

1. **Rename Symbol (F2)**: Automatically updates imports when renaming files
2. **Move File**: Right-click → "Move to..." updates imports automatically
3. **Find All References (Shift+F12)**: Find all import locations before moving

#### TypeScript Language Server

The TypeScript language server automatically tracks imports and can update them when:
- Files are renamed via the IDE
- Using "Rename Symbol" (F2)
- Using "Move Symbol" refactoring

#### ESLint with Import Rules

Configure ESLint to catch broken imports:

```json
{
  "extends": [
    "plugin:import/recommended",
    "plugin:import/typescript"
  ],
  "rules": {
    "import/no-unresolved": "error",
    "import/no-relative-parent-imports": "warn"
  },
  "settings": {
    "import/resolver": {
      "typescript": {
        "alwaysTryTypes": true,
        "project": "./tsconfig.json"
      }
    }
  }
}
```

### Strategy 3: Pre-Move Checklist

Before moving a file:

1. ✅ **Find all imports** of the file:
   ```bash
   # Using grep/ripgrep
   grep -r "from.*file" src/
   
   # Or use IDE "Find All References"
   ```

2. ✅ **Check for re-exports**:
   ```typescript
   // Check if file is re-exported from index files
   export * from './some/path/to/folder/file'
   ```

3. ✅ **Update barrel exports** (index.ts files):
   ```typescript
   // Before
   export * from './some/path/to/folder/file'
   
   // After
   export * from './another/path/in/another/folder/file'
   ```

4. ✅ **Update test files**:
   ```typescript
   // Update test imports
   import { functionToTest } from './another/path/in/another/folder/file'
   ```

5. ✅ **Run type checking**:
   ```bash
   # TypeScript
   npx tsc --noEmit
   
   # Or with your build tool
   npm run type-check
   ```

---

## Tooling and Automation

### 1. TypeScript Project References

For monorepos, use TypeScript project references:

```json
// tsconfig.json (root)
{
  "compilerOptions": {
    "composite": true
  },
  "references": [
    { "path": "./packages/core" },
    { "path": "./packages/utils" }
  ]
}
```

### 2. Path Mapping Tools

#### `tsconfig-paths`

For Node.js projects, use `tsconfig-paths` to resolve paths at runtime:

```bash
npm install --save-dev tsconfig-paths
```

```typescript
// Register at entry point
import 'tsconfig-paths/register'
```

#### `vite-tsconfig-paths` Plugin

For Vite projects, use the plugin to automatically sync paths:

```bash
npm install --save-dev vite-tsconfig-paths
```

```typescript
import tsconfigPaths from 'vite-tsconfig-paths'

export default defineConfig({
  plugins: [tsconfigPaths()]
})
```

### 3. Import Sorting and Organization

Use tools to maintain consistent import order:

```bash
# ESLint plugin
npm install --save-dev eslint-plugin-import

# Or use Prettier with import sorting
npm install --save-dev @trivago/prettier-plugin-sort-imports
```

### 4. Automated Import Updates

#### Using `jscodeshift` (Facebook's Codemod Tool)

Create codemods to automate bulk import updates:

```javascript
// codemod.js
export default function transformer(file, api) {
  const j = api.jscodeshift
  return j(file.source)
    .find(j.ImportDeclaration)
    .forEach(path => {
      // Update import paths
      if (path.value.source.value.includes('old/path')) {
        path.value.source.value = path.value.source.value.replace(
          'old/path',
          'new/path'
        )
      }
    })
    .toSource()
}
```

---

## Project-Specific Recommendations

### For DataForge Project

Based on your current setup:

#### Current Configuration

Your project uses:
- **SvelteKit** with TypeScript
- **Vite** as build tool
- **Path alias**: `$lib` → `./src/lib`
- **shadcn-svelte** aliases: `$lib/components`, `$lib/utils`, etc.

#### Recommended Practices

1. **Extend Path Aliases** (if needed):

   ```json
   // tsconfig.json
   {
     "compilerOptions": {
       "baseUrl": ".",
       "paths": {
         "$lib": ["./src/lib"],
         "$lib/*": ["./src/lib/*"],
         "@/*": ["./src/*"],  // Add this for broader access
         "@components/*": ["./src/lib/components/*"],
         "@utils/*": ["./src/lib/utils/*"]
       }
     }
   }
   ```

   ```typescript
   // vite.config.ts
   import path from 'path'
   
   export default defineConfig({
     resolve: {
       alias: {
         '$lib': path.resolve(__dirname, './src/lib'),
         '@': path.resolve(__dirname, './src')
       }
     }
   })
   ```

2. **File Organization Strategy**:

   ```
   src/
   ├── lib/
   │   ├── components/     # Use $lib/components
   │   ├── utils/          # Use $lib/utils
   │   └── types/          # Use $lib/types
   ├── routes/             # Use relative paths (SvelteKit routes)
   └── app.css
   ```

3. **When Moving Files**:

   - **Within `src/lib/`**: Use path aliases - no updates needed
   - **Between routes**: Use relative paths - update manually or via IDE
   - **From routes to lib**: Update to use `$lib/*` aliases
   - **From lib to routes**: Consider if file should stay in lib

4. **Migration Checklist**:

   When moving `some/path/to/folder/file.ts` to `another/path/in/another/folder/file.ts`:

   ```bash
   # 1. Find all imports
   grep -r "some/path/to/folder/file" src/
   
   # 2. Check for re-exports
   grep -r "export.*from.*some/path" src/
   
   # 3. Use IDE refactoring (F2 or Move File)
   # 4. Run type check
   npm run check  # or your type-check command
   
   # 5. Run linter
   npm run lint
   ```

---

## Common Pitfalls and Solutions

### Pitfall 1: Mismatched Aliases

**Problem**: Aliases in `tsconfig.json` don't match `vite.config.ts`

**Solution**: Keep them in sync or use `vite-tsconfig-paths` plugin

### Pitfall 2: Circular Dependencies

**Problem**: Files importing each other create circular dependencies

**Solution**: 
- Use barrel exports (`index.ts`) carefully
- Extract shared code to separate modules
- Use dependency injection patterns

### Pitfall 3: Runtime Path Resolution

**Problem**: Path aliases work in TypeScript but fail at runtime

**Solution**: 
- Ensure Vite/SvelteKit aliases are configured
- For Node.js, use `tsconfig-paths/register`
- Test imports in both dev and build modes

### Pitfall 4: Test File Imports

**Problem**: Test files can't resolve path aliases

**Solution**: Configure test runners (Vitest, Jest) with path mapping:

```typescript
// vitest.config.ts
import { defineConfig } from 'vitest/config'
import path from 'path'

export default defineConfig({
  resolve: {
    alias: {
      '$lib': path.resolve(__dirname, './src/lib')
    }
  }
})
```

---

## Summary: Quick Reference

### ✅ DO

- Use path aliases for stable, shared code
- Keep `tsconfig.json` and build tool configs in sync
- Use IDE refactoring tools (F2, Move File)
- Run type checks after moving files
- Use relative paths for co-located files
- Document your path alias conventions

### ❌ DON'T

- Use deep relative paths (`../../../../`)
- Mix alias and relative paths inconsistently
- Forget to update test files
- Ignore barrel export files (`index.ts`)
- Move files without checking all references
- Use aliases for files that move frequently

### 🔧 Tools

- **IDE**: VS Code / Cursor refactoring (F2, Move File)
- **Type Checking**: `tsc --noEmit`
- **Linting**: ESLint with `import` plugin
- **Build Tools**: `vite-tsconfig-paths` plugin
- **Search**: `grep` or IDE "Find All References"

---

## Additional Resources

- [TypeScript Module Resolution](https://www.typescriptlang.org/docs/handbook/module-resolution.html)
- [Vite Path Aliases](https://vitejs.dev/config/shared-options.html#resolve-alias)
- [SvelteKit Path Aliases](https://kit.svelte.dev/docs/configuration#alias)
- [ESLint Import Plugin](https://github.com/import-js/eslint-plugin-import)

