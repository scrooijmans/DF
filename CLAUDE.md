# CLAUDE.md

## Project Overview

**DataForge Platform** - Unified geoscience data platform with two Tauri desktop applications:

- **DataForge** - Offline-first data management with sync capabilities
- **DataForge-Compute** - Compute and analysis workbench

Both applications share the same tech stack and code conventions.

## Code Style

- Tabs for indentation
- Single quotes, no trailing commas
- 100 character print width
- Prettier with svelte and tailwindcss plugins

## Project Structure

```
.
├── DataForge/                 # Main data management app
│   ├── src/                   # SvelteKit frontend
│   ├── src-tauri/             # Tauri Rust backend
│   ├── crates/                # Rust workspace crates
│   └── docs/decisions/        # Architecture Decision Records
│
├── DataForge-Compute/         # Compute workbench app
│   ├── src/                   # SvelteKit frontend
│   ├── src-tauri/             # Tauri Rust backend
│   └── docs/                  # Compute documentation
│
├── docs/                      # Shared documentation
│   ├── architecture/          # Architecture docs
│   ├── best-practices/        # Development guides
│   └── context7/              # Library documentation cache
│
└── markdown-guides/           # Software design guides
```

### Common App Structure (both apps)

```
src/
├── routes/           # SvelteKit file-based routing
├── lib/
│   ├── components/   # Reusable Svelte components
│   ├── stores/       # Svelte stores for state management
│   └── types/        # TypeScript type definitions
└── app.css           # Global styles

src-tauri/
├── src/              # Rust backend code
└── tauri.conf.json   # Tauri configuration
```

## Tech Stack

### Core Framework

- **Svelte 5** - Frontend framework with runes
- **SvelteKit** - Application framework
- **Tauri 2** - Desktop application wrapper
- **TypeScript** - Type safety
- **Vite** - Build tooling

### Styling & UI

- **Tailwind CSS 4** - Utility-first CSS
- **bits-ui** - Headless component library (DataForge)
- **Lucide** - Icon system

### Data & Visualization

- **AG Grid** - Enterprise data grid
- **ECharts** - Charting library (DataForge-Compute)
- **DuckDB** - Parquet file querying
- **SQLite/rusqlite** - Local database

### Sync & Collaboration

- **Automerge** - CRDT-based real-time sync
- **Superforms + Zod** - Form handling and validation

## Svelte 5 Conventions

**Use Svelte 5 runes** - Avoid deprecated lifecycle APIs in new code.

### Runes (no import needed)

```typescript
$state()      // Reactive state
$derived()    // Computed values
$effect()     // Side effects and lifecycle
$props()      // Component props
```

### Lifecycle Pattern

Replace `onMount`/`onDestroy` with `$effect()`:

```typescript
// OLD - Don't use
import { onMount, onDestroy } from 'svelte'
onMount(() => { ... })
onDestroy(() => { ... })

// NEW - Use this pattern
$effect(() => {
  // Setup code runs when dependencies change
  const cleanup = initializeSomething()

  return () => {
    // Cleanup runs on unmount or before re-run
    cleanup()
  }
})
```

### Avoiding Infinite Loops in $effect

When initializing third-party libraries (AG Grid, ECharts, etc.):

```typescript
import { untrack } from 'svelte'

// Use regular variables for non-reactive state
let instance = null
let isInitialized = false

$effect(() => {
  if (!container || isInitialized) return
  isInitialized = true

  // Wrap initialization in untrack to avoid tracking dependencies
  untrack(() => {
    instance = createInstance(container, options)
  })

  return () => {
    instance?.destroy()
    instance = null
    isInitialized = false
  }
})
```

## Design Principles

### Local-First Architecture

See `LOCAL_FIRST_GUIDELINES.md` for complete guidelines. Key principles:

1. **Fast** - All operations work from local data first
2. **Works Offline** - Full functionality without network
3. **Multi-Device** - Seamless sync across devices
4. **User Ownership** - Data stored in user-controlled locations
5. **Long-Term** - Open formats, no vendor lock-in

### Code Quality

See `Principal_Engineer_Critical_Evaluation_Checklist.md` for evaluation criteria:

- Clear architecture and component boundaries
- Domain logic separated from UI and infrastructure
- Explicit state management
- Comprehensive error handling
- API contracts defined and documented

### Component Design

- Prefer generic, well-named components over ad-hoc page-specific code
- Keep domain logic separate from presentation
- Choose file locations and names that reflect ownership
- Design for reusability and composability

## Architecture Decision Records

DataForge uses ADRs in `DataForge/docs/decisions/` (MADR 4.0 format):

- `0001-git-like-sync-model.md` - Pull-based sync over real-time CRDT
- `0002-content-addressed-blob-storage.md` - SHA-256 hash storage
- `0003-dual-native-gridded-storage.md` - Native + gridded Parquet
- `0004-url-path-api-versioning.md` - /api/v1/ versioning
- `0005-utoipa-openapi-generation.md` - OpenAPI spec generation
- `0006-refinery-sqlite-migrations.md` - SQLite migrations

## Context7 Usage

**Trigger**: Add "use context7" to prompts for library documentation.

**Examples**:

```
Create a Tauri command that reads a Parquet file using DuckDB. use context7
```

```
Set up AG Grid with custom cell renderers. use context7
```

```
Implement a Svelte 5 component with $state and $effect. use context7
```

**Documentation Storage**: Generated docs stored in `docs/context7/` organized by library.

## Development Commands

```bash
# DataForge
cd DataForge
bun install
bun tauri dev

# DataForge-Compute
cd DataForge-Compute
bun install
bun tauri dev
```

## Key Documentation

| Document | Purpose |
|----------|---------|
| `LOCAL_FIRST_GUIDELINES.md` | Local-first architecture principles |
| `Principal_Engineer_Critical_Evaluation_Checklist.md` | Code quality checklist |
| `docs/architecture/` | System architecture docs |
| `docs/best-practices/` | Development guides |
| `markdown-guides/SOFTWARE-DESIGN/` | Design checklists |
