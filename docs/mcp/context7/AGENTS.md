# DataForge Tech Stack & Context7 Documentation Guide

You have access to Context7 MCP server, which provides up-to-date, version-specific documentation and code examples for all libraries in the DataForge tech stack. This ensures you always have current APIs and best practices.

## Overview

**Context7** brings up-to-date, version-specific documentation and code examples directly into your AI coding assistant. That means no more outdated code or hallucinated APIs.

### ❌ Without Context7
- LLMs rely on outdated or generic information about the libraries you use
- Code examples based on old training data
- Hallucinated APIs that don't even exist
- Generic answers for old package versions

### ✅ With Context7
- Context7 MCP pulls up-to-date, version-specific documentation and code examples straight from the source
- Grounds your LLM with current documentation
- Ensures high-quality, working code with current APIs

## How to Use Context7

### Basic Usage

**Add `use context7` to your prompt** when asking about any library in the tech stack:

```
Create a Tauri command that reads a Parquet file using DuckDB. use context7
```

```
Implement Automerge for real-time collaborative editing. use context7
```

```
Set up a SciChart surface with multiple series and custom axes. use context7
```

### Workflow

1. **Write your prompt naturally** - describe what you want to build
2. **Add `use context7`** to your prompt
3. **Get working code** with current APIs

No tab-switching, no hallucinated APIs that don't exist, no outdated code generation.

## DataForge Tech Stack

### Core Framework

- **SvelteKit + Svelte 5** - Frontend framework
  - Use Context7 for: **Svelte 5 runes**, SvelteKit routing, load functions
  - Prefer `$state`, `$derived`, and **`$effect()` instead of `onMount()`** for lifecycle‑style logic
    in new Svelte code.
  - Example: `How do I migrate onMount logic to Svelte 5 $effect()? use context7`

- **TypeScript** - Type-safe development
  - Use Context7 for: TypeScript patterns, type definitions, advanced types

- **Vite** - Build tooling
  - Use Context7 for: Vite configuration, plugin setup, build optimization

- **Tauri 2** - Desktop application wrapper
  - Use Context7 for: Tauri commands, window management, native dialogs, plugins
  - Example: `Create a Tauri command that opens a file dialog. use context7`

### Styling & UI

- **Tailwind CSS 4** - Utility-first styling
  - Use Context7 for: Tailwind v4 features, configuration, custom utilities
  - Example: `Configure Tailwind CSS 4 with custom theme colors. use context7`

- **Bits UI** - Headless component library
  - Use Context7 for: Component patterns, accessibility, composition

- **Lucide Icons** - Icon system
  - Use Context7 for: Icon usage, sizing, styling

### Data Visualization

- **SciChart** - Advanced scientific charting (WASM-powered)
  - Use Context7 for: Chart configuration, series types, performance optimization
  - Example: `Create a SciChart surface with multiple XY series and custom axes. use context7`

- **AG Grid** - Enterprise data grid
  - Use Context7 for: Grid configuration, column definitions, cell renderers
  - Example: `Set up AG Grid with custom cell renderers and sorting. use context7`

### Backend & Database

- **SQLite (rusqlite)** - Local database
  - Use Context7 for: SQLite queries, schema management, transactions
  - Example: `Create a rusqlite query with prepared statements. use context7`

- **DuckDB** - Parquet file querying (Tauri backend)
  - Use Context7 for: DuckDB queries, Parquet reading, performance optimization
  - Example: `Query Parquet files with DuckDB in Rust. use context7`

### Real-time Collaboration

- **Automerge** - CRDT-based real-time sync
  - Use Context7 for: CRDT patterns, conflict resolution, document merging
  - Example: `Implement Automerge for collaborative document editing. use context7`

### Forms & Validation

- **Superforms + Zod** - Form handling and schema validation
  - Use Context7 for: Form setup, validation, error handling
  - Example: `Create a Superforms form with Zod validation. use context7`

### Authentication

- **Local Auth (Argon2)** - Password hashing and session management
  - Self-hosted authentication following ColaNode's account-workspace-user model

### Desktop Features

- **Tauri plugins** - Native dialogs, file system, shell
  - Use Context7 for: Plugin configuration, native APIs, desktop integration

## Available Context7 MCP Tools

### 1. resolve-library-id

**Use this FIRST** to resolve a library/package name to a Context7-compatible library ID.

- Searches for matching libraries based on name
- Returns library ID needed for documentation retrieval
- Example: Resolve "svelte" or "tauri" to their Context7 library IDs

### 2. get-library-docs

Retrieves up-to-date documentation for a specific library.

- Requires library ID from `resolve-library-id`
- Supports two modes:
  - `mode: "code"` (default) - API references and code examples
  - `mode: "info"` - Conceptual guides and architectural information
- Can focus on specific topics (e.g., "hooks", "routing", "authentication")
- Supports pagination for large documentation sets

### Usage Pattern

```typescript
// 1. Resolve library name to ID
resolve-library-id("svelte")

// 2. Get documentation
get-library-docs({
  context7CompatibleLibraryID: "/sveltejs/svelte",
  mode: "code",
  topic: "runes",
  page: 1
})
```

## Best Practices

### When to Use Context7

✅ **DO use Context7 for:**
- Library-specific API questions
- Version-specific features
- Code examples and patterns
- Configuration setup
- Migration guides
- Best practices

❌ **DON'T use Context7 for:**
- General programming concepts
- Project-specific business logic
- Code that's already in the codebase

### Design & Reuse Expectations for Generated Code

Whenever Context7 is used to help design or implement **new frontend or backend components**, favor
high-quality, reusable designs aligned with the software design checklists in:

- `markdown-guides/SOFTWARE-DESIGN/new_project_kickoff_checklist.md`
- `markdown-guides/SOFTWARE-DESIGN/software_project_quality_checklist.md`

Concretely, new components should:

- **Be reusable by default**:
  - Prefer generic, composable building blocks over one-off, page-specific code
  - Keep concerns separated (UI vs domain vs infrastructure)
  - Co-locate related logic, but avoid god-modules or grab-bag utilities
- **Have clear boundaries and responsibilities**:
  - Single, well-named responsibility per component or module
  - Explicit inputs/outputs (props, function parameters, return types)
  - No hidden global state; dependencies passed in or imported from well-defined boundaries
- **Be easy to change safely**:
  - Designed so that medium-sized changes don’t ripple through the entire codebase
  - Tested or at least structured to be testable without heavy infrastructure
  - Naming and structure that matches intent, so future engineers can understand and refactor

### Prompt Examples

**Good prompts:**
```
Create a Tauri command that reads a Parquet file using DuckDB and returns the results. use context7
```

```
Implement Supabase realtime subscription with conflict resolution using Automerge. use context7
```

```
Set up a SciChart surface with multiple XY series, custom axes, and zoom/pan functionality. use context7
```

```
Create a Superforms form with Zod validation and error handling. use context7
```

**Less effective prompts:**
```
How do I write a function? use context7
```

```
What's in my codebase? use context7
```

## Documentation Storage

When Context7 is used to retrieve library documentation:

- **All Context7-generated content** must be stored in `docs/context7/` directory
- **Organize by library** in subfolders: `docs/context7/{library-name}/`
- **Use descriptive filenames** within the library subfolder (e.g., `automerge-repo-indexeddb.md`, `tauri-commands.md`, `svelte-runes.md`)
- **Include relevant code examples**, API references, and usage patterns
- **Update existing documentation** if Context7 provides more current information

### Storage Structure Examples

```
docs/context7/
├── automerge/
│   ├── automerge-repo-indexeddb.md
│   └── automerge-crdt-patterns.md
├── tauri/
│   ├── tauri-commands.md
│   └── tauri-window-management.md
├── svelte/
│   ├── svelte-runes.md
│   └── sveltekit-routing.md
├── vite/
│   └── vite-configuration.md
└── ...
```

### Library Name Mapping

When determining the subfolder name, use the library's package name (lowercase, hyphenated):
- `automerge` → `docs/context7/automerge/`
- `@sveltejs/kit` → `docs/context7/sveltekit/` or `docs/context7/svelte/`
- `@tauri-apps/api` → `docs/context7/tauri/`
- `yjs` → `docs/context7/yjs/`
- `scichart` → `docs/context7/scichart/`

## Architecture Notes

### Project Structure

- **Frontend**: SvelteKit code in `/src` directory
- **Backend**: Rust backend code in `src-tauri`
- **Internal Crates**: `crates/dataforge-core`, `crates/dataforge-sync`, `crates/dataforge-storage`

### Key Patterns

- **Authentication**: ColaNode-style account-workspace-user model
- **State Management**: Svelte 5 runes for component state
- **Error Handling**: `thiserror` for Rust, Zod for frontend validation
- **Storage**: Local SQLite + Parquet files for curve data
- **Real-time**: Automerge CRDTs for collaborative editing

## Next Steps

1. **Install Context7** - Already configured in MCP settings
2. **Start using it** - Add `use context7` to your prompts
3. **Check documentation** - Review `docs/context7/{library-name}/` for retrieved documentation organized by library

For more information, see: [Context7 Documentation](https://context7.com/docs/overview)

