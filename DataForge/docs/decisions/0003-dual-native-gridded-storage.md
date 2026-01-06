# Use Dual Native/Gridded Storage for Curve Data

- **Status**: accepted
- **Date**: 2024-12-14
- **Decision-makers**: Engineering team
- **Consulted**: Petrophysics domain experts
- **Informed**: Stakeholders

## Context and Problem Statement

Well log curves from LAS files have varying sample rates (0.5 ft, 0.1 ft, etc.) and depth units (feet, meters). To enable cross-curve analysis and correlation views, curves need to be on a common depth grid. However, we must also preserve the original data for audit and regulatory compliance. How should we store curve data?

## Decision Drivers

- **Data provenance**: Original data must be preserved for audit
- **Analysis ready**: Curves need common grid for correlation/crossplots
- **Unit consistency**: Support both feet and meters
- **Storage efficiency**: Avoid excessive duplication
- **Traceability**: Clear distinction between source and derived data

## Considered Options

1. **Dual storage** (native + gridded Parquet files)
2. **Native only** (resample on-the-fly)
3. **Gridded only** (discard original sampling)
4. **Virtual views** (store native, compute gridded via SQL)

## Decision Outcome

**Chosen option**: "Dual storage", because it preserves original data for compliance while providing pre-computed gridded data for efficient analysis. The storage overhead is acceptable given the importance of both use cases.

### Consequences

**Positive:**
- Original data preserved (regulatory compliance, audit trail)
- Fast analysis (no resampling needed at query time)
- Clear provenance (native vs gridded explicitly tracked)
- Resample method recorded for reproducibility
- Unit conversion tracked

**Negative:**
- Storage overhead (~2x for curve data)
- Must keep both in sync (managed by immutability)
- More complex ingestion pipeline

## Confirmation

- Ingestion tests verify both Parquet files created
- Schema includes `native_parquet_hash` and `gridded_parquet_hash`
- Resample method and unit conversion tracked in metadata

## Pros and Cons of Options

### Dual Storage (Native + Gridded)

Store two Parquet files per curve: original sampling and resampled to well grid.

- **Good**: Preserves original data for compliance
- **Good**: Fast queries on gridded data
- **Good**: Clear source vs derived distinction
- **Good**: Resample parameters recorded
- **Bad**: ~2x storage for curve data
- **Bad**: More complex ingestion
- **Neutral**: Both files are immutable (content-addressed)

### Native Only (Resample On-The-Fly)

Store only original sampling, compute gridded at query time.

- **Good**: Minimal storage
- **Good**: Single source of truth
- **Bad**: Slow queries (resample every time)
- **Bad**: Resampling must be deterministic
- **Bad**: Complex query logic

### Gridded Only (Discard Original)

Resample during ingestion, discard original sampling.

- **Good**: Simple storage
- **Good**: Fast queries
- **Bad**: Original data lost
- **Bad**: Cannot audit original values
- **Bad**: Regulatory compliance issues
- **Bad**: Information loss from resampling

### Virtual Views (SQL Computation)

Store native, create virtual views that resample via DuckDB.

- **Good**: Single storage
- **Good**: Flexible resampling
- **Bad**: Query performance depends on DuckDB
- **Bad**: Complex SQL for interpolation
- **Bad**: Harder to optimize

## More Information

**Storage schema:**
```sql
curves (
    ...
    native_parquet_hash TEXT,    -- Original sampling
    gridded_parquet_hash TEXT,   -- Resampled to well grid
    resample_method TEXT,        -- 'linear', 'nearest', etc.
    was_unit_converted INTEGER,  -- 1 if units converted
    ...
)
```

**Well grid configuration:**
```sql
wells (
    ...
    depth_unit TEXT,    -- 'ft' or 'm'
    depth_step REAL,    -- e.g., 0.5
    depth_origin REAL,  -- typically 0.0
    ...
)
```

**Implementation files:**
- `crates/dataforge-core/src/wellgrid.rs` - Resampling algorithms
- `crates/dataforge-core/src/parquet.rs` - Parquet generation
- `crates/dataforge-core/src/ingest.rs` - Dual storage creation

**Domain context:**
- OSDU (Open Subsurface Data Universe) uses similar patterns
- Industry standard to preserve original wireline data
- Regulatory requirements (e.g., mineral rights) require original data retention
