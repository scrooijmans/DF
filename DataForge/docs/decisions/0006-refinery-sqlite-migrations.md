# Use Refinery for SQLite Schema Migrations

- **Status**: implemented
- **Date**: 2026-01-05 (Updated: 2026-01-08)
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
- **SQLite limitations**: ALTER TABLE doesn't support IF NOT EXISTS for columns

## Considered Options

1. **Refinery** (with rusqlite feature)
2. **Diesel migrations** (would require Diesel ORM adoption)
3. **SQLx migrations** (async-focused, not rusqlite compatible)
4. **Manual migrations** (custom migration tracking)

## Decision Outcome

**Chosen option**: "Refinery with Hybrid Approach", because SQLite's ALTER TABLE limitations require Rust code for conditional column additions, while Refinery provides version tracking and the `embed_migrations!` macro for desktop deployment.

### Implementation Notes (2026-01-08)

Due to SQLite's limitation that ALTER TABLE doesn't support `IF NOT EXISTS` for columns, we implemented a **hybrid approach**:

1. **Rust migrations** (`run_migrations()` in db.rs): Handle conditional ALTER TABLE operations with explicit column existence checks
2. **Refinery SQL files**: Serve as documentation and version tracking (mostly no-ops since Rust handles DDL)
3. **Bootstrap mechanism**: Detects already-applied migrations for existing databases transitioning to Refinery

### Consequences

**Positive:**
- Direct rusqlite integration via feature flag
- Migrations embedded in binary (no external files needed at runtime)
- Supports both SQL and Rust migration files
- Version tracking via `refinery_schema_history` table
- Bootstrap mechanism handles existing database transition
- Idempotent migrations (safe to run multiple times)

**Negative:**
- No automatic rollback (must write explicit undo migrations)
- No compile-time schema validation
- Hybrid approach adds complexity (Rust + SQL migrations)
- SQL files are mostly documentation rather than executable

## Confirmation

- ✅ Migrations directory at `crates/dataforge-core/migrations/`
- ✅ All schema changes tracked as versioned migrations (V1-V8)
- ✅ Migrations run on app startup before database access
- ✅ `refinery_schema_history` table tracks applied versions
- ✅ Bootstrap mechanism handles existing databases
- ✅ Both fresh and existing databases work correctly

## Current Migration Inventory

| Version | Name | Purpose |
|---------|------|---------|
| V1 | baseline_schema | Baseline marker (SCHEMA constant handles creation) |
| V2 | add_conflict_strategy | Add conflict_strategy column to sync_state |
| V3 | curves_dual_storage | Recreate curves table for native/gridded storage |
| V4 | curves_quality_fields | Add quality fields to curves table |
| V5 | curve_properties_osdu | Add OSDU columns to curve_properties |
| V6 | markers_well_name | Add well_name column to markers |
| V7 | schema_version_columns | Add schema_version to work product tables |
| V8 | osdu_schema_alignment | Add wellbores table and OSDU columns |

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

### Actual Implementation Pattern

**1. Dependencies in workspace `Cargo.toml`:**
```toml
[workspace.dependencies]
refinery = { version = "0.8", features = ["rusqlite"] }
```

**2. Migrations directory structure:**
```
crates/dataforge-core/
├── migrations/
│   ├── V1__baseline_schema.sql
│   ├── V2__add_conflict_strategy.sql
│   ├── V3__curves_dual_storage.sql
│   ├── V4__curves_quality_fields.sql
│   ├── V5__curve_properties_osdu.sql
│   ├── V6__markers_well_name.sql
│   ├── V7__schema_version_columns.sql
│   └── V8__osdu_schema_alignment.sql
└── src/
    └── db.rs
```

**3. Database initialization flow (db.rs):**
```rust
use refinery::embed_migrations;

embed_migrations!("migrations");

pub fn init_db(conn: &Connection) -> SqliteResult<()> {
    // 1. Bootstrap Refinery history for existing databases
    bootstrap_refinery_history(conn)?;

    // 2. Run Rust migrations (conditional ALTER TABLE)
    run_migrations(conn)?;

    // 3. Run Refinery migrations (version tracking)
    run_refinery_migrations(conn)?;

    // 4. Execute SCHEMA batch (CREATE TABLE/INDEX IF NOT EXISTS)
    conn.execute_batch(SCHEMA)?;

    // 5. Seed reference data
    insert_default_reference_data(conn)?;

    Ok(())
}
```

**4. Helper functions for SQLite limitations:**
```rust
fn table_exists(conn: &Connection, table_name: &str) -> bool { ... }
fn column_exists(conn: &Connection, table_name: &str, column_name: &str) -> bool { ... }

// Example conditional migration
if table_exists(conn, "wells") && !column_exists(conn, "wells", "operator") {
    conn.execute("ALTER TABLE wells ADD COLUMN operator TEXT", [])?;
}
```

**5. Bootstrap mechanism for existing databases:**
```rust
fn bootstrap_refinery_history(conn: &Connection) -> SqliteResult<()> {
    // Detect which migrations are already applied based on schema state
    // Insert records into refinery_schema_history with "bootstrapped" checksum
}
```

### Migration File Naming

```
V{version}__{description}.sql   # Strictly versioned
U{version}__{description}.sql   # Non-contiguous (team workflows)
```

### Adding a New Migration

1. Create `V{N}__{description}.sql` in `migrations/` directory
2. If it requires ALTER TABLE, add Rust code to `run_migrations()` in db.rs
3. Add detection logic to `bootstrap_refinery_history()` for existing DBs
4. Update the migrations list in `run_refinery_migrations()`
5. Rebuild - migrations embedded on compile
6. Run app - migrations auto-apply on startup

### Rollback Strategy

Refinery doesn't support automatic rollbacks. To undo a migration:

1. Create new migration: `V{N+1}__undo_previous_change.sql`
2. Add corresponding Rust code for conditional reversal
3. This maintains a clear audit trail

### References

- [Refinery GitHub](https://github.com/rust-db/refinery)
- [Refinery Documentation](https://docs.rs/refinery/)
- [Flyway Migration Patterns](https://documentation.red-gate.com/fd/migrations-184127470.html)
