# Pipeline-Node Relationship Architecture

## Problem Statement

Nodes can be assigned to multiple pipelines. We need to efficiently query which pipelines contain a given node without:
- Creating data duplication
- Traversing JSONB fields
- Adding denormalized columns

## Solution: Use `pipeline_node_references` Table

### Why This Is The Correct Approach

**✅ DO: Use `pipeline_node_references` table**

The `pipeline_node_references` table is the **source of truth** for which pipelines contain which nodes. This is the correct relational design because:

1. **Normalized Design**: Follows proper database normalization (many-to-many relationship)
2. **Indexed Queries**: `node_id` column is indexed (`idx_pipeline_node_references_node_id`)
3. **No Data Duplication**: Node data stored once, referenced by ID
4. **Efficient Queries**: Direct index lookup, no JSONB traversal needed
5. **Scalable**: Performance doesn't degrade as pipelines grow

### Query Pattern

```typescript
// Efficient query using indexed foreign key
const { data } = await supabase
  .from("pipeline_node_references")
  .select(`
    pipeline_id,
    pipelines (
      id,
      name,
      description,
      project_id
    )
  `)
  .eq("node_id", nodeId);
```

**Performance**: O(log n) with index lookup, scales to millions of pipelines.

### ❌ DO NOT: Add Column to Nodes Table

**Why not?**
- Violates normalization (node can be in many pipelines)
- Requires array column (`pipeline_ids UUID[]`) which is harder to query
- Data duplication risk (pipeline info stored in multiple places)
- Harder to maintain consistency

### ❌ DO NOT: Traverse JSONB `dag_definition`

**Why not?**
- Requires full table scan of `pipelines` table
- JSONB parsing overhead for every pipeline
- Performance degrades linearly with number of pipelines
- No index support for JSONB array queries

### ❌ DO NOT: Use Edge Functions

**Why not?**
- Unnecessary complexity for a simple relational query
- Adds latency (function invocation overhead)
- Database can handle this efficiently with proper indexes

## Implementation

### Service Function

```typescript
// src/lib/services/node-service.ts
export async function getPipelinesContainingNode(
  nodeId: string,
): Promise<Array<{ id: string; name: string; description: string | null; project_id: string }>>
```

### Usage in Components

```typescript
// Load pipelines when node is selected
$effect(() => {
  if (selectedNodeId) {
    loadPipelinesContainingNode();
  }
});
```

## Database Schema

```sql
-- Junction table (many-to-many relationship)
CREATE TABLE pipeline_node_references (
    id UUID PRIMARY KEY,
    pipeline_id UUID REFERENCES pipelines(id),
    node_id UUID REFERENCES nodes(id),
    position_x FLOAT,
    position_y FLOAT,
    pipeline_specific_config JSONB,
    UNIQUE (pipeline_id, node_id)
);

-- Index for efficient node lookups
CREATE INDEX idx_pipeline_node_references_node_id 
  ON pipeline_node_references(node_id);
```

## Benefits

1. **Query Efficiency**: Indexed lookup, no full table scans
2. **Data Integrity**: Foreign key constraints ensure referential integrity
3. **Maintainability**: Single source of truth, no sync issues
4. **Scalability**: Performance remains constant as data grows
5. **Flexibility**: Easy to add pipeline-specific metadata (position, config)

## Related Tables

- `nodes`: Node definitions (shared across pipelines)
- `pipelines`: Pipeline definitions with `dag_definition` JSONB
- `pipeline_node_references`: Junction table linking pipelines to nodes

## Notes

- The `dag_definition` JSONB field in `pipelines` table is used for execution/runtime
- The `pipeline_node_references` table is used for queries and relationships
- Both are kept in sync when nodes are added/removed from pipelines

