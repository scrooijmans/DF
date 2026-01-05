# Realtime Node Creation: Adding Nodes to Active Pipeline

## Overview

This document explains the complete end-to-end implementation of realtime node creation in the MudRock application. It describes how clicking a UDF in the Library tab creates a new node in the database, adds it to the selected pipeline's `dag_definition`, and automatically updates the UI in real-time through Supabase Realtime subscriptions.

## Architecture Overview

The node creation system consists of several interconnected components:

```
┌─────────────────────────────────────────────────────────────┐
│                    User Interaction                          │
│  Click UDF in Library Tab → content-library-udf-category-item │
└────────────────────┬────────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────────┐
│              Frontend Service Layer                         │
│  createNodeFromUdfAndAddToPipeline()                        │
│    ├─ createNodeFromUdf() → Insert into nodes table         │
│    └─ addNodeToPipeline() → Update pipeline dag_definition │
└────────────────────┬────────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────────┐
│              PostgreSQL Database                            │
│  INSERT INTO nodes (...)                                    │
│  UPDATE pipelines SET dag_definition = ...                 │
│  ← Publication: supabase_realtime                           │
│  ← RLS policies enabled                                     │
└────────────────────┬────────────────────────────────────────┘
                     │
                     │ Logical Replication (WAL)
                     ▼
┌─────────────────────────────────────────────────────────────┐
│              Supabase Realtime Service                      │
│  Detects INSERT on nodes table                              │
│  Detects UPDATE on pipelines table                          │
│  ← Extension: postgres_cdc_rls                              │
│  ← Verifies RLS policies                                    │
└────────────────────┬────────────────────────────────────────┘
                     │
                     │ WebSocket (ws://)
                     ▼
┌─────────────────────────────────────────────────────────────┐
│              Kong API Gateway                               │
│  Routes /realtime/v1/websocket                              │
│  ← Key-auth plugin (anonymous: anon)                        │
└────────────────────┬────────────────────────────────────────┘
                     │
                     │ HTTPS/WSS
                     ▼
┌─────────────────────────────────────────────────────────────┐
│              Frontend Realtime Services                     │
│  RealtimeNodesService → Updates nodesForCurrentSelectedPipeline │
│  Pipeline Realtime → Reloads nodes when dag_definition changes │
│  ← ContentDagPipelineState (Global State)                   │
└────────────────────┬────────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────────┐
│              UI Components                                  │
│  SF-pipeline-flow.svelte → Visual graph updates             │
│  content-selected-pipeline-blocks.svelte → Node list updates│
└─────────────────────────────────────────────────────────────┘
```

## Complete Call Stack & Data Flow

### Step 1: User Clicks UDF in Library Tab

**Location**: `src/lib/components/pages/home/content-main/content-library-udfs/content-library-udf-category/content-library-udf-category-item/content-library-udf-category-item.svelte`

**Code Flow**:

```svelte
<script lang="ts">
  import { createNodeFromUdfAndAddToPipeline } from '$lib/services/node-service';
  import { getContentDagPipelineState } from '...';
  import { getPostgresProjectsState } from '...';

  async function handleClick() {
    const selectedPipelineId = pipelineState.selectedPipelineId;
    const projectId = projectsState.currentSelectedProjectId;

    // Validation
    if (!selectedPipelineId) {
      alert('Please select a pipeline first');
      return;
    }

    // Create node and add to pipeline
    const node = await createNodeFromUdfAndAddToPipeline({
      udfName: udf.name,
      udfDescription: udf.description || '',
      udfCategory: udf.category,
      pipelineId: selectedPipelineId,
      projectId,
    });
  }
</script>
```

**What Happens**:

1. Validates that a pipeline and project are selected
2. Calls `createNodeFromUdfAndAddToPipeline()` with UDF details
3. Shows loading state (`isAdding = true`) during operation

---

### Step 2: Create Node in Database

**Location**: `src/lib/services/node-service.ts` → `createNodeFromUdf()`

**Code Flow**:

```typescript
export async function createNodeFromUdf(params: {
  udfName: string;
  udfDescription: string;
  udfCategory: string;
  pipelineId: string;
  projectId: string;
}): Promise<NodeRow> {
  const nodeId = crypto.randomUUID();

  // Create operator node config from UDF
  const nodeConfig = {
    type: "operator",
    operator_id: params.udfName,
    operator_version: "1.0.0",
    parameters: {},
    version_strategy: { strategy: "latest" },
  };

  const nodeRow: NodeRow = {
    id: nodeId,
    name: params.udfName,
    node_type: "operator",
    node_config: nodeConfig,
    input_schema: null,
    output_schema: null,
    metadata: {
      description: params.udfDescription,
      tags: [params.udfCategory],
      cache_enabled: false,
    },
    tags: [params.udfCategory],
    is_active: true,
  };

  // Insert into nodes table via Supabase PostgREST
  const { data, error } = await supabase
    .from("nodes")
    .insert({ ...nodeRow })
    .select()
    .single();

  return data as NodeRow;
}
```

**Database Operation**:

```sql
INSERT INTO public.nodes (
    id, name, node_type, node_config, input_schema, output_schema,
    metadata, tags, is_active, created_at, updated_at
) VALUES (
    'uuid-generated',
    'udf_name',
    'operator',
    '{"type": "operator", "operator_id": "udf_name", ...}',
    NULL,
    NULL,
    '{"description": "...", "tags": ["category"], ...}',
    ARRAY['category'],
    true,
    NOW(),
    NOW()
);
```

**What Happens**:

1. Generates UUID for new node
2. Converts UDF to operator node configuration
3. Inserts node into `nodes` table via Supabase PostgREST
4. Returns `NodeRow` with created node data

**PostgreSQL WAL**: Write-Ahead Log is updated with INSERT operation

---

### Step 3: Update Pipeline's dag_definition

**Location**: `src/lib/services/node-service.ts` → `addNodeToPipeline()`

**Code Flow**:

```typescript
export async function addNodeToPipeline(params: {
  pipelineId: string;
  nodeId: string;
  sourceNodeId?: string;
}): Promise<void> {
  // Fetch current pipeline
  const { data: pipeline } = await supabase
    .from("pipelines")
    .select("dag_definition")
    .eq("id", params.pipelineId)
    .single();

  const dagDefinition = pipeline.dag_definition || {};

  // Initialize node_ids array if needed
  if (!dagDefinition.node_ids) {
    dagDefinition.node_ids = [];
  }

  // Add node ID if not already present
  if (!dagDefinition.node_ids.includes(params.nodeId)) {
    dagDefinition.node_ids.push(params.nodeId);
  }

  // Initialize edges array if needed
  if (!dagDefinition.edges) {
    dagDefinition.edges = [];
  }

  // Add edge if source node provided
  if (params.sourceNodeId) {
    dagDefinition.edges.push({
      source_node_id: params.sourceNodeId,
      target_node_id: params.nodeId,
      edge_type: "data_flow",
    });
  }

  // Update pipeline
  await supabase
    .from("pipelines")
    .update({
      dag_definition: dagDefinition,
      updated_at: new Date().toISOString(),
    })
    .eq("id", params.pipelineId);
}
```

**Database Operation**:

```sql
-- First: Fetch current dag_definition
SELECT dag_definition FROM pipelines WHERE id = 'pipeline-id';

-- Then: Update with new node_id
UPDATE pipelines
SET dag_definition = jsonb_set(
    dag_definition,
    '{node_ids}',
    (dag_definition->'node_ids') || '["new-node-id"]'::jsonb,
    true
),
updated_at = NOW()
WHERE id = 'pipeline-id';
```

**What Happens**:

1. Fetches current pipeline's `dag_definition`
2. Adds new node ID to `node_ids` array
3. Optionally creates edge if `sourceNodeId` provided
4. Updates pipeline's `dag_definition` JSONB field
5. Updates `updated_at` timestamp

**PostgreSQL WAL**: Write-Ahead Log is updated with UPDATE operation

---

### Step 4: Realtime Service Detects Changes

**Location**: Supabase Realtime Service (Docker container)

**Process**:

```
PostgreSQL WAL (INSERT on nodes table)
    ↓
Logical Replication → Supabase Realtime Service
    ↓
Checks RLS policies (SELECT permission required)
    ↓
Formats event for WebSocket transmission
    ↓
Queues event for WebSocket client
```

**Event Format** (Node INSERT):

```json
{
  "schema": "public",
  "table": "nodes",
  "commit_timestamp": "2025-01-20T10:14:15.444Z",
  "eventType": "INSERT",
  "new": {
    "id": "517eb7c9-21a8-4215-b501-7a2cee8ab428",
    "name": "linear_vsh",
    "node_type": "operator",
    "node_config": {
      "type": "operator",
      "operator_id": "linear_vsh",
      "operator_version": "1.0.0",
      ...
    },
    "metadata": {...},
    "tags": ["subsurface"],
    "is_active": true,
    "created_at": "2025-01-20T10:14:15.441571+00:00",
    ...
  },
  "old": null
}
```

**Event Format** (Pipeline UPDATE):

```json
{
  "schema": "public",
  "table": "pipelines",
  "commit_timestamp": "2025-01-20T10:14:15.500Z",
  "eventType": "UPDATE",
  "new": {
    "id": "24cffc55-328b-4410-a266-05ac5ad8378b",
    "dag_definition": {
      "node_ids": [
        "517eb7c9-21a8-4215-b501-7a2cee8ab428",
        "a5af6d6a-1bb6-466e-9ffa-4de750b9065d"
      ],
      "edges": [...],
      ...
    },
    "updated_at": "2025-01-20T10:14:15.500Z",
    ...
  },
  "old": {
    "dag_definition": {
      "node_ids": ["a5af6d6a-1bb6-466e-9ffa-4de750b9065d"],
      ...
    },
    ...
  }
}
```

**Configuration**:

- **Publication**: `supabase_realtime` includes both `public.nodes` and `public.pipelines` tables
- **RLS Policies**: SELECT policies for `anon` and `authenticated` roles (see `005-enable-realtime-nodes.sql`)
- **Extension**: `postgres_cdc_rls` with RLS verification enabled

---

### Step 5: Frontend Receives Node INSERT Event

**Location**: `src/lib/services/realtime-nodes-service.ts` → `handleNodeInsert()`

**Code Flow**:

```typescript
private async handleNodeInsert(newNode: any) {
  const pipelineId = this.pipelineState?.selectedPipelineId;
  if (!pipelineId) {
    return; // No pipeline selected, skip
  }

  // Fetch pipeline to verify node belongs to it
  const { data: pipeline } = await this.supabase
    .from("pipelines")
    .select("dag_definition")
    .eq("id", pipelineId)
    .single();

  const dagDefinition = pipeline.dag_definition || {};
  const nodeIds = dagDefinition.node_ids || [];

  // Only update if node is in pipeline's node_ids
  if (!nodeIds.includes(newNode.id)) {
    return; // Node not in this pipeline, skip
  }

  // Fetch position info from pipeline_node_references
  const references = await getPipelineNodeReferences(pipelineId);
  const ref = references.find((r) => r.node_id === newNode.id);
  const position = ref?.position_x !== null && ref?.position_y !== null
    ? [ref.position_x, ref.position_y] as [number, number]
    : null;

  // Convert NodeRow to DagNode
  const dagNode = nodeRowToDagNode(newNode, position);

  // Update global state
  const exists = this.pipelineState.nodesForCurrentSelectedPipeline.some(
    (n: DagNode) => n.id === dagNode.id
  );

  if (exists) {
    // Update existing node
    this.pipelineState.nodesForCurrentSelectedPipeline =
      this.pipelineState.nodesForCurrentSelectedPipeline.map((n: DagNode) =>
        n.id === dagNode.id ? dagNode : n
      );
  } else {
    // Add new node (preserve order from node_ids)
    const orderedNodes = [...this.pipelineState.nodesForCurrentSelectedPipeline];
    const nodeIndex = nodeIds.indexOf(dagNode.id);
    if (nodeIndex >= 0) {
      orderedNodes.splice(nodeIndex, 0, dagNode);
    } else {
      orderedNodes.push(dagNode);
    }
    this.pipelineState.nodesForCurrentSelectedPipeline = orderedNodes;
  }

  // Invalidate cache
  await invalidateTableCache("nodes");
}
```

**What Happens**:

1. Checks if a pipeline is selected
2. Fetches pipeline's `dag_definition` to verify node belongs to it
3. Only processes if node ID is in `dag_definition.node_ids`
4. Fetches position info from `pipeline_node_references` table
5. Converts `NodeRow` to `DagNode` format
6. Updates `ContentDagPipelineState.nodesForCurrentSelectedPipeline` array
7. Invalidates PostgREST cache for "nodes" table

**State Update**:

```typescript
// Before
nodesForCurrentSelectedPipeline = [
  { id: "node-1", name: "Load Data", ... }
]

// After
nodesForCurrentSelectedPipeline = [
  { id: "node-1", name: "Load Data", ... },
  { id: "new-node-id", name: "linear_vsh", ... }  // ← New node added
]
```

---

### Step 6: Frontend Receives Pipeline UPDATE Event

**Location**: `src/lib/components/pages/home/content-main/content-dag-editor/content-dag-editor-pipelines/content-dag-editor-pipelines.svelte`

**Code Flow**:

```svelte
<script lang="ts">
  // Realtime updates for pipelines table
  $effect(() => {
    const channel = supabase
      .channel("pipelines-changes")
      .on(
        "postgres_changes",
        { event: "*", schema: "public", table: "pipelines" },
        (payload: any) => {
          const selectedPipelineId = pipelineState.selectedPipelineId;

          // Reload pipelines list
          if (pid) void loadPipelines();

          // If selected pipeline's dag_definition changed, reload nodes
          if (selectedPipelineId && payload.new?.id === selectedPipelineId) {
            console.log("Selected pipeline's dag_definition changed, reloading nodes");
            void loadNodesForPipeline(selectedPipelineId);
          }
        },
      )
      .subscribe();

    return () => {
      try { supabase.removeChannel(channel); } catch (_) {}
    };
  });
</script>
```

**What Happens**:

1. Listens for UPDATE events on `pipelines` table
2. When selected pipeline's `dag_definition` changes:
   - Calls `loadNodesForPipeline()` to fetch all nodes
   - Updates `nodesForCurrentSelectedPipeline` with complete node list
   - Ensures UI reflects latest state

**Why Both Events?**:

- **Node INSERT**: Immediate update when node is created (optimistic)
- **Pipeline UPDATE**: Ensures consistency after `dag_definition` is updated (defensive)

---

### Step 7: UI Components React to State Changes

**Location**: Multiple components using `ContentDagPipelineState`

**Components Affected**:

1. **SF-pipeline-flow.svelte** (Visual Graph):

   ```svelte
   <script lang="ts">
     const pipelineState = getContentDagPipelineState();
     let nodes = $derived(pipelineState.nodesForCurrentSelectedPipeline);
   </script>

   <SvelteFlow {nodes} {edges} />
   ```

   - Automatically re-renders when `nodesForCurrentSelectedPipeline` changes
   - New node appears in visual graph

2. **content-selected-pipeline-blocks.svelte** (Node List):

   ```svelte
   <script lang="ts">
     const pipelineState = getContentDagPipelineState();
     let nodes = $derived(pipelineState.nodesForCurrentSelectedPipeline);
   </script>

   {#each nodes as node}
     <ContentSelectedPipelineBlockItem {node} />
   {/each}
   ```

   - Automatically updates node list when state changes
   - New node appears in sidebar

3. **node-editor-sidebar.svelte** (Node Details):

   ```svelte
   <script lang="ts">
     const pipelineState = getContentDagPipelineState();
     let selectedNode = $derived.by(() => {
       const nodeId = pipelineState.currentSelectedNodeForSelectedPipeline;
       return pipelineState.nodesForCurrentSelectedPipeline.find(n => n.id === nodeId);
     });
   </script>
   ```

   - Automatically updates when selected node changes

**Svelte Reactivity**:

- `$state` runes trigger automatic re-renders
- `$derived` runes recompute when dependencies change
- Components automatically update without manual refresh

---

## Complete Call Stack Summary

```
1. User clicks UDF
   ↓
2. content-library-udf-category-item.svelte::handleClick()
   ↓
3. node-service.ts::createNodeFromUdfAndAddToPipeline()
   ├─ createNodeFromUdf()
   │   └─ supabase.from('nodes').insert() → PostgreSQL INSERT
   │       └─ PostgreSQL WAL updated
   │
   └─ addNodeToPipeline()
       └─ supabase.from('pipelines').update() → PostgreSQL UPDATE
           └─ PostgreSQL WAL updated
   ↓
4. Supabase Realtime Service detects changes
   ├─ Node INSERT event → WebSocket message
   └─ Pipeline UPDATE event → WebSocket message
   ↓
5. Frontend receives events
   ├─ RealtimeNodesService::handleNodeInsert()
   │   └─ Updates ContentDagPipelineState.nodesForCurrentSelectedPipeline
   │
   └─ content-dag-editor-pipelines.svelte::pipeline UPDATE handler
       └─ Calls loadNodesForPipeline() → Ensures consistency
   ↓
6. UI Components react
   ├─ SF-pipeline-flow.svelte → Visual graph updates
   ├─ content-selected-pipeline-blocks.svelte → Node list updates
   └─ node-editor-sidebar.svelte → Node details update
```

---

## Key Files Reference

| File                                       | Purpose                                        |
| ------------------------------------------ | ---------------------------------------------- |
| `content-library-udf-category-item.svelte` | User interaction handler for UDF clicks        |
| `node-service.ts`                          | Node creation and pipeline update functions    |
| `realtime-nodes-service.ts`                | Realtime subscription service for nodes table  |
| `content-dag-editor-pipeline.svelte.ts`    | Global state management for pipeline and nodes |
| `content-dag-editor-pipelines.svelte`      | Pipeline list component with realtime updates  |
| `005-enable-realtime-nodes.sql`            | SQL migration for realtime on nodes table      |

---

## Database Schema

### Nodes Table

```sql
CREATE TABLE public.nodes (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL,
    node_type TEXT NOT NULL, -- 'ingestion', 'operator', 'output'
    node_config JSONB NOT NULL,
    input_schema JSONB,
    output_schema JSONB,
    metadata JSONB DEFAULT '{}',
    tags TEXT[] DEFAULT '{}',
    is_active BOOLEAN DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL
);
```

### Pipelines Table (dag_definition field)

```json
{
  "id": "pipeline-uuid",
  "name": "Pipeline Name",
  "node_ids": [
    "node-uuid-1",
    "node-uuid-2"
  ],
  "edges": [
    {
      "source_node_id": "node-uuid-1",
      "target_node_id": "node-uuid-2",
      "edge_type": "data_flow"
    }
  ],
  "version": "1.0.0",
  "metadata": {...}
}
```

---

## Realtime Configuration

### SQL Migration (`005-enable-realtime-nodes.sql`)

1. **Add to Publication**:

   ```sql
   ALTER PUBLICATION supabase_realtime ADD TABLE public.nodes;
   ```

2. **Enable RLS**:

   ```sql
   ALTER TABLE public.nodes ENABLE ROW LEVEL SECURITY;
   ```

3. **Create RLS Policies**:
   - SELECT policies for `anon` and `authenticated`
   - INSERT, UPDATE, DELETE policies for both roles

### Frontend Initialization

**Location**: `content-dag-editor-pipeline.svelte.ts`

```typescript
export class ContentDagPipelineState {
  constructor() {
    // Initialize realtime service
    requestAnimationFrame(() => {
      setTimeout(() => {
        realtimeNodesService.initialize(this);
        realtimeNodesService.connect();
      }, 100);
    });
  }
}
```

---

## Benefits

### ✅ **Real-time Updates**

- UI updates automatically when nodes are created
- No manual refresh required
- Consistent state across all components

### ✅ **Optimistic Updates**

- Node appears immediately via INSERT event
- Pipeline update ensures consistency
- Dual-event approach provides redundancy

### ✅ **Type Safety**

- TypeScript types ensure correct data structures
- `DagNode` type matches backend `NodeType` enum
- Compile-time validation prevents errors

### ✅ **Performance**

- Cache invalidation ensures fresh data
- Realtime subscriptions avoid polling
- Efficient WebSocket communication

### ✅ **Scalability**

- Same pattern can be extended to other tables
- Realtime service is reusable
- RLS policies ensure security

---

## Troubleshooting

### Node Not Appearing in UI

1. **Check Realtime Connection**:

   ```typescript
   console.log(realtimeNodesService.getConnectionStatus());
   ```

2. **Verify Node in Pipeline**:

   ```sql
   SELECT dag_definition->'node_ids' FROM pipelines WHERE id = 'pipeline-id';
   ```

3. **Check RLS Policies**:
   ```sql
   SELECT * FROM pg_policies WHERE tablename = 'nodes';
   ```

### Realtime Not Working

1. **Verify Publication**:

   ```sql
   SELECT * FROM pg_publication_tables WHERE tablename = 'nodes';
   ```

2. **Check Realtime Service Logs**:
   - Look for "SUBSCRIBED" status in console
   - Check for RLS policy errors

3. **Verify Tenant Configuration**:
   - Ensure tenant exists in `_realtime.tenants`
   - Check JWT secret encryption

---

## Future Enhancements

### 🚀 **Edge Creation**

- Automatically create edges when nodes are added
- Support for connecting to multiple source nodes

### 🚀 **Position Management**

- Auto-position new nodes in visual graph
- Save positions to `pipeline_node_references` table

### 🚀 **Batch Operations**

- Support adding multiple nodes at once
- Optimize realtime updates for bulk operations

### 🚀 **Conflict Resolution**

- Handle concurrent node additions
- Merge changes from multiple users

---

## Summary

The realtime node creation system provides a seamless, reactive experience for adding nodes to pipelines. By leveraging Supabase Realtime subscriptions, the UI automatically updates when nodes are created, ensuring users always see the latest state without manual refresh.

**Key Design Decisions**:

- ✅ **Dual-event approach**: Node INSERT + Pipeline UPDATE for redundancy
- ✅ **RLS policies**: Secure access control for realtime subscriptions
- ✅ **State management**: Global state ensures consistency across components
- ✅ **Type safety**: TypeScript types match backend structures
- ✅ **Performance**: Cache invalidation + realtime subscriptions

This architecture ensures type safety, performance, and maintainability throughout the node creation flow.
