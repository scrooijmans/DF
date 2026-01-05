# CLAUDE.md

## Code Style

- Tabs for indentation
- Single quotes, no trailing commas
- 100 character print width
- Prettier with svelte and tailwindcss plugins

## Project Structure

- `src/routes/` - SvelteKit file-based routing
- `src/lib/` - Shared code, importable via `$lib` alias
- `src/lib/assets/` - Static assets imported in components
- `static/` - Public static files served at root
- `src-tauri/` - Rust backend code (Tauri)
- `crates/` - Internal Rust crates (dataforge-core, dataforge-sync, dataforge-storage)

## Context7

**Trigger**: User says "use context7" (for library documentation questions).

**Usage**: Add `use context7` to your prompt when asking about any library in the DataForge tech stack.

**Available Libraries**:

### Core Framework

- `svelte` - Svelte 5 framework
  - When generating Svelte examples, prefer **Svelte 5 runes** (`$state`, `$derived`, `$effect()`) and
    avoid deprecated lifecycle APIs like `onMount()` in new code. Lifecycle‑style logic that used to
    live in `onMount()` should instead be expressed via `$effect()`.
- `sveltekit` - SvelteKit framework
- `tauri` - Desktop application wrapper
- `typescript` - TypeScript language
- `vite` - Build tooling

### Styling & UI

- `tailwindcss` - Tailwind CSS 4
- `bits-ui` - Headless component library
- `lucide` - Icon system

### Data Visualization

- `ag-grid` - Enterprise data grid

### Database

- `rusqlite` - SQLite for Rust
- `duckdb` - Parquet file querying

### Real-time & Forms

- `automerge` - CRDT-based real-time sync
- `superforms` - Form handling
- `zod` - Schema validation

**Examples**:

```
Create a Tauri command that reads a Parquet file using DuckDB. use context7
```

```
Show how to use Zod for form validation in SvelteKit. use context7
```

```
Set up a SciChart surface with multiple series and custom axes. use context7
```

**Documentation Storage**: All Context7-generated documentation is stored in `docs/context7/` organized
by library in subfolders. For example, Automerge documentation would be stored in
`docs/context7/automerge/automerge-repo-indexeddb.md`.

**Design & Reuse Expectations**: When asking for new frontend or backend components, design them to
be reusable, composable, and aligned with our software design checklists:

- `markdown-guides/SOFTWARE-DESIGN/new_project_kickoff_checklist.md`
- `markdown-guides/SOFTWARE-DESIGN/software_project_quality_checklist.md`

That means:

- Prefer generic, well-named components/hooks/stores over ad-hoc page-specific code
- Keep domain logic separate from presentation and infrastructure
- Choose file locations and names that reflect ownership and responsibility

**Complete Guide**: See `docs/mcp/context7/AGENTS.md` for complete guide.
