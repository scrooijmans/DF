# Use Refinery for SQLite Schema Migrations

- **Status**: accepted
- **Date**: 2026-01-05
- **Decision-makers**: Engineering team
- **Consulted**: Rust database migration best practices
- **Informed**: Stakeholders

## Context and Problem Statement

DataForge uses rusqlite for local SQLite storage. As the schema evolves, we need a way to track changes, enable rollbacks, and ensure team members' databases stay in sync. How should we manage SQLite schema migrations?

## Decision Drivers

- **rusqlite compatibility**: Must work with existing rusqlite setup
- **Embedded migrations**: Migrations baked into binary for desktop app deployment
- **Offline-first**: No network dependency for migrations
- **Team collaboration**: Handle non-contiguous migrations from parallel PRs
- **Simplicity**: Minimal boilerplate, easy to understand

## Considered Options

1. **Refinery** (with rusqlite feature)
2. **Diesel migrations** (would require Diesel ORM adoption)
3. **SQLx migrations** (async-focused, not rusqlite compatible)
4. **Manual migrations** (custom migration tracking)

## Decision Outcome

**Chosen option**: "Refinery", because it directly supports rusqlite, can embed migrations in the binary (critical for Tauri desktop app), and uses Flyway-inspired patterns that are industry standard. It's a standalone tool that doesn't require adopting a full ORM.

### Consequences

**Positive:**
- Direct rusqlite integration via feature flag
- Migrations embedded in binary (no external files needed at runtime)
- Supports both SQL and Rust migration files
- Non-contiguous migrations (`U` prefix) handle parallel team development
- Flyway-inspired patterns (industry standard)
- Transaction-wrapped migrations by default

**Negative:**
- No automatic rollback (must write explicit undo migrations)
- No compile-time schema validation
- Learning curve for migration naming conventions

## Confirmation

- Migrations directory at `crates/dataforge-core/migrations/`
- All schema changes tracked as versioned migrations
- Migrations run on app startup before database access
- Integration tests verify migration correctness

## Pros and Cons of Options

### Refinery (with rusqlite feature)

Standalone migration toolkit with embedded migration support.

- **Good**: Direct rusqlite support (`features = ["rusqlite"]`)
- **Good**: `embed_migrations!` macro bakes migrations into binary
- **Good**: Supports SQL and Rust migration files
- **Good**: Non-contiguous migrations for team workflows
- **Good**: Flyway-inspired (proven patterns)
- **Good**: Transaction-per-migration by default
- **Bad**: No automatic rollback generation
- **Bad**: No compile-time schema checking
- **Neutral**: Must rebuild to pick up new migrations

### Diesel Migrations

Full ORM with built-in migration system.

- **Good**: Compile-time schema validation
- **Good**: Automatic rollback support
- **Good**: Mature ecosystem
- **Bad**: Requires adopting Diesel ORM entirely
- **Bad**: Heavier dependency footprint
- **Bad**: Different query patterns than raw rusqlite

### SQLx Migrations

Async SQL toolkit with migration support.

- **Good**: Compile-time query checking
- **Good**: Good CLI tooling
- **Bad**: Async-focused (not rusqlite compatible)
- **Bad**: Would require rewriting database layer
- **Bad**: Overkill for embedded SQLite

### Manual Migrations

Custom migration tracking implementation.

- **Good**: Full control
- **Good**: No external dependencies
- **Bad**: Must implement version tracking
- **Bad**: Must implement transaction handling
- **Bad**: Non-standard patterns
- **Bad**: Time investment for basic functionality

## More Information

### Implementation Pattern

**1. Add dependencies to `Cargo.toml`:**
```toml
[dependencies]
refinery = { version = "0.8", features = ["rusqlite"] }

[build-dependencies]
refinery-cli = "0.8"  # Optional: for CLI commands
```

**2. Create migrations directory:**
```
crates/dataforge-core/
├── migrations/
│   ├── V1__initial_schema.sql
│   ├── V2__add_curve_properties.sql
│   └── V3__add_sync_tables.sql
└── src/
    └── db.rs
```

**3. Embed migrations in code:**
```rust
use refinery::embed_migrations;
use rusqlite::Connection;

embed_migrations!("migrations");

pub fn initialize_database(conn: &mut Connection) -> Result<(), refinery::Error> {
    migrations::runner().run(conn)?;
    Ok(())
}
```

**4. Migration file naming:**
```
V{version}__{description}.sql   # Strictly versioned
U{version}__{description}.sql   # Non-contiguous (team workflows)
```

**5. Example migration (`V1__initial_schema.sql`):**
```sql
-- V1__initial_schema.sql
CREATE TABLE IF NOT EXISTS wells (
    id TEXT PRIMARY KEY,
    workspace_id TEXT NOT NULL,
    name TEXT NOT NULL,
    uwi TEXT,
    depth_unit TEXT NOT NULL DEFAULT 'ft',
    depth_step REAL NOT NULL DEFAULT 0.5,
    depth_origin REAL NOT NULL DEFAULT 0.0,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    deleted_at TEXT,
    version INTEGER NOT NULL DEFAULT 1
);

CREATE INDEX idx_wells_workspace ON wells(workspace_id);
```

### Migration Workflow

1. **Create migration**: Add `V{N}__{description}.sql` to `migrations/`
2. **Rebuild**: Migrations embedded on compile
3. **Run app**: Migrations auto-apply on startup
4. **Team sync**: Non-contiguous (`U` prefix) handles parallel PRs

### Rollback Strategy

Refinery doesn't support automatic rollbacks. To undo a migration:

1. Create new migration: `V{N+1}__undo_previous_change.sql`
2. Write explicit SQL to reverse the change
3. This maintains a clear audit trail

### References

- [Refinery GitHub](https://github.com/rust-db/refinery)
- [Refinery Documentation](https://docs.rs/refinery/)
- [Flyway Migration Patterns](https://documentation.red-gate.com/fd/migrations-184127470.html)
