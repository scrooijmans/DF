# DAG Schema Analysis & Recommendations

**Date**: 2025-11-01  
**Context**: Evaluating current pipeline schema (`02-pipeline-schema.sql`) for Svelte Flow integration

## 📊 Current Schema Overview

### Tables in Use
1. ✅ **`pipelines`** - Primary table with `dag_definition` JSONB (actively used)
2. ⚠️ **`pipeline_nodes`** - Normalized nodes (written by backend, not read by frontend)
3. ⚠️ **`pipeline_edges`** - Normalized edges (written by backend, not read by frontend)
4. 📋 **`pipeline_executions`** - Execution history (future use)
5. 📋 **`pipeline_lineage`** - Transformation tracking (future use)
6. ✅ **`operator_registry`** - Operator catalog (actively used)

## 🎯 Recommendation: **Keep JSONB, Make Normalized Tables Optional**

### Why JSONB is Perfect for DAGs

1. **Flexibility**: Matches `DagDefinition` structure exactly
2. **Svelte Flow Compatibility**: Direct conversion to `Node[]/Edge[]` arrays
3. **Single Source of Truth**: No sync issues between JSONB and normalized tables
4. **PostgreSQL JSONB Performance**: 
   - GIN indexes for efficient queries
   - Native JSON operators (`->`, `->>`, `@>`)
   - Supports complex nested structures
5. **No External Format Needed**: 
   - Substrait/other formats add complexity
   - JSONB is human-readable and debuggable
   - Works seamlessly with Rust `serde_json`

### When Normalized Tables Are Useful

**Keep them for future advanced queries:**

1. **Schema Compatibility Queries**:
   ```sql
   -- Find all pipelines with nodes that output 'well_logs_standard' schema
   SELECT DISTINCT pipeline_id 
   FROM pipeline_nodes 
   WHERE output_schema->>'schema_id' = 'well_logs_standard';
   ```

2. **Graph Analysis**:
   ```sql
   -- Find all nodes that depend on a specific node
   SELECT target_node_id 
   FROM pipeline_edges 
   WHERE source_node_id = 'node-uuid';
   ```

3. **Position Queries** (for UI):
   ```sql
   -- Get all node positions for a pipeline
   SELECT node_id, position_x, position_y 
   FROM pipeline_nodes 
   WHERE pipeline_id = 'pipeline-uuid';
   ```

**However**: These queries can also be done on JSONB with proper indexes:
```sql
-- Same query using JSONB
SELECT id FROM pipelines 
WHERE dag_definition->'nodes' @> '[{"node_type": {"type": "operator", "output_schema": {"schema_id": "well_logs_standard"}}}]';
```

## 🔧 Recommended Schema Changes

### Option 1: **Keep Current Schema** (Recommended for MVP)
- ✅ Keep all tables as-is
- ✅ Backend continues dual-write (JSONB + normalized)
- ✅ Frontend uses JSONB only
- ✅ Normalized tables available for future queries
- **Action**: No changes needed

### Option 2: **Simplify to JSONB Only** (If normalized tables aren't needed)
- ❌ Remove `pipeline_nodes` and `pipeline_edges` tables
- ✅ Keep `dag_definition` JSONB as single source of truth
- ✅ Simplify `PipelineStorage` to only write JSONB
- **Trade-off**: Lose ability to do efficient normalized queries

### Option 3: **Make Normalized Tables Optional** (Hybrid)
- ✅ Keep tables but mark as "optional"
- ✅ Add a flag: `use_normalized_tables BOOLEAN DEFAULT false`
- ✅ Only write to normalized tables if flag is enabled
- **Action**: Add conditional logic to `PipelineStorage::save_pipeline`

## 📝 Current Implementation Status

### Backend (`crates/dags/storage/src/pipeline_storage.rs`)
- ✅ Writes to `dag_definition` JSONB
- ✅ Writes to `pipeline_nodes` (normalized)
- ✅ Writes to `pipeline_edges` (normalized)
- ❌ **Never reads from normalized tables** (always uses JSONB)

### Frontend (`src/lib/services/pipeline-service.ts`)
- ✅ Reads `dag_definition` JSONB only
- ✅ Converts JSONB → Svelte Flow format
- ❌ Never queries normalized tables

## 🎨 Svelte Flow Integration Impact

**Current Flow**:
```
Postgres (JSONB) → Frontend (DagDefinition) → Converter → Svelte Flow (Node[]/Edge[])
```

**With Normalized Tables** (if we wanted to use them):
```
Postgres (normalized) → Frontend (reconstruct DagDefinition) → Converter → Svelte Flow
```

**Verdict**: JSONB is simpler and more efficient for Svelte Flow.

## ✅ Final Recommendation

**Keep the current schema as-is** with these clarifications:

1. **Primary Storage**: `dag_definition` JSONB is the **source of truth**
2. **Normalized Tables**: Optional denormalization for **future advanced queries**
3. **No External Format**: JSONB is sufficient, no need for Substrait/other formats
4. **Backend**: Continue dual-write for now (minimal overhead)
5. **Frontend**: Continue using JSONB only

**Future Optimization**: If normalized tables remain unused after 6 months, consider removing them or making them truly optional with a feature flag.

## 🔮 Alternative Storage Options (Not Recommended)

### Why Not Substrait?
- ❌ Adds serialization/deserialization overhead
- ❌ Less human-readable than JSONB
- ❌ Requires additional libraries
- ❌ Overkill for our use case

### Why Not External Graph Database (Neo4j, etc.)?
- ❌ Adds infrastructure complexity
- ❌ Requires separate deployment
- ❌ JSONB + PostgreSQL is sufficient for our graph size
- ❌ PostgreSQL has excellent JSONB support

### Why Not Parquet/Arrow Files?
- ❌ Not suitable for DAG definitions (textual structure)
- ❌ Would require separate storage layer
- ❌ JSONB is more queryable in Postgres

## 📚 References

- [PostgreSQL JSONB Documentation](https://www.postgresql.org/docs/current/datatype-json.html)
- [Svelte Flow Data Format](https://svelteflow.dev/)
- Current schema: `db/init/02-pipeline-schema.sql`
- Storage implementation: `crates/dags/storage/src/pipeline_storage.rs`

