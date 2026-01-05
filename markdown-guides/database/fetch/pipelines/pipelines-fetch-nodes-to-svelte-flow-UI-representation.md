# Pipeline DAG Visualization: Frontend to Backend Flow

This document describes the complete end-to-end process of how a pipeline's DAG definition is fetched from PostgreSQL and rendered in the Svelte Flow visual editor.

## 📊 Overview

**Flow**: User Selection → State Management → Data Fetching → Type Conversion → Layout Application → Visual Rendering

```
PostgreSQL (JSONB) → Supabase PostgREST → Frontend Service → Type Converter → Svelte Flow Component → Dagre Layout → Visual Graph
```

## 🔄 Complete Data Flow

### 1. **Pipeline Selection** (User Interaction)

**Location**: `src/lib/components/pages/home/content-main/content-dag-editor/content-dag-editor-pipelines/content-dag-editor-pipeline-item/content-dag-editor-pipeline-item.svelte`

**Action**: User clicks on a pipeline item in the sidebar

**Code Flow**:

```svelte
<script lang="ts">
  import { getContentDagPipelineState } from '../content-dag-editor-pipeline.svelte.ts';

  const pipelineState = getContentDagPipelineState();

  function handleClick() {
    pipelineState.setSelectedPipelineId(props.pipeline.id);
  }
</script>

<button onclick={handleClick}>
  {props.pipeline.name}
</button>
```

**Result**: `ContentDagPipelineState.selectedPipelineId` is updated with the selected pipeline UUID.

---

### 2. **Global State Management** (Reactive State)

**Location**: `src/lib/components/pages/home/content-main/content-dag-editor/content-dag-editor-pipelines/content-dag-editor-pipeline.svelte.ts`

**State Class**:

```typescript
export class ContentDagPipelineState {
  selectedPipelineId = $state<string | null>(null);

  setSelectedPipelineId(id: string | null) {
    this.selectedPipelineId = id;
  }

  getSelectedPipelineId(): string | null {
    return this.selectedPipelineId;
  }
}
```

**Context Setup**: Initialized in `src/routes/home/+layout.svelte`:

```typescript
import { setContentDagPipelineState } from "$lib/components/.../content-dag-editor-pipeline.svelte.ts";

// In layout component
setContentDagPipelineState();
```

**Result**: Global reactive state accessible throughout the DAG editor component tree.

---

### 3. **Reactive Pipeline ID Reading** (Component State)

**Location**: `src/lib/components/pages/home/content-main/content-dag-editor/content-dag-editor.svelte`

**Code**:

```typescript
const pipelineState = getContentDagPipelineState();
let selectedId = $derived(pipelineState.getSelectedPipelineId());
```

**Result**: `selectedId` automatically updates whenever `pipelineState.selectedPipelineId` changes.

---

### 4. **Effect-Based Data Loading** (Automatic Fetch)

**Location**: `src/lib/components/pages/home/content-main/content-dag-editor/content-dag-editor.svelte`

**Code**:

```typescript
$effect(() => {
  const id = selectedId; // Capture current value
  if (id) {
    void loadPipeline();
  }
});
```

**Why `const id = selectedId`?**: Captures the current value to prevent infinite loops. The effect only runs when `selectedId` changes, not on every reactive update.

**Result**: `loadPipeline()` is called automatically whenever a new pipeline is selected.

---

### 5. **Pipeline Data Fetching** (Supabase PostgREST)

**Location**: `src/lib/services/pipeline-service.ts`

**Function**: `getPipelineById(id: string)`

**Code**:

```typescript
export async function getPipelineById(id: string): Promise<PipelineRow | null> {
  const { data, error } = await supabase
    .from("pipelines")
    .select(
      `id, name, description, project_id, version, tags, created_at, updated_at, dag_definition`,
    )
    .eq("id", id)
    .single();

  if (error) throw new Error(error.message);
  return (data as PipelineRow) ?? null;
}
```

**Database Query**:

```sql
SELECT
  id, name, description, project_id, version, tags,
  created_at, updated_at, dag_definition
FROM pipelines
WHERE id = $1
LIMIT 1;
```

**Result**: Returns `PipelineRow` with `dag_definition` JSONB containing the complete DAG structure:

```json
{
  "id": "pipeline-uuid",
  "name": "Sample Pipeline",
  "project_id": "project-uuid",
  "version": "1.0.0",
  "nodes": [
    {
      "id": "node-uuid",
      "name": "Load Well Logs",
      "node_type": { "type": "ingestion", ... },
      "dependencies": [],
      "metadata": { "position": [100, 100], ... }
    },
    ...
  ],
  "metadata": { ... }
}
```

---

### 6. **Type Conversion: Backend → Frontend** (DagDefinition → Svelte Flow)

**Location**: `src/lib/services/dag-converter.ts`

**Function**: `dagDefinitionToSvelteFlow(dag: DagDefinition)`

**Conversion Process**:

#### 6.1. **Node Conversion**

**Input** (Backend `DagNode`):

```typescript
{
  id: "node-uuid",
  name: "Load Well Logs",
  node_type: { type: "ingestion", source_type: {...}, ... },
  dependencies: ["parent-node-uuid"],
  metadata: { position: [100, 100], cache_enabled: true }
}
```

**Output** (Svelte Flow `SvelteFlowNode`):

```typescript
{
  id: "node-uuid",
  type: "default",
  position: { x: 100, y: 100 },
  data: {
    label: "Load Well Logs\n(ingestion)",
    nodeType: "ingestion",
    nodeConfig: { type: "ingestion", ... },
    nodeName: "Load Well Logs",
    metadata: { position: [100, 100], ... }
  },
  sourcePosition: "bottom",
  targetPosition: "top"
}
```

**Key Transformations**:

- `node.metadata.position` (array `[x, y]`) → `position` (object `{x, y}`)
- `node.node_type.type` → `data.nodeType` for styling
- `node.name` + `nodeType` → `data.label` for display
- Full `node.node_type` → `data.nodeConfig` for preservation

#### 6.2. **Edge Conversion**

**Input** (Backend `DagNode.dependencies`):

```typescript
node.dependencies = ["parent-node-uuid"];
```

**Output** (Svelte Flow `SvelteFlowEdge[]`):

```typescript
[
  {
    id: "e-parent-node-uuid-node-uuid",
    source: "parent-node-uuid",
    target: "node-uuid",
    type: "smoothstep",
    animated: false,
  },
];
```

**Key Transformations**:

- `node.dependencies` (array of upstream node IDs) → `SvelteFlowEdge[]` (explicit source/target pairs)
- Each dependency becomes an edge: `dependency → node.id`
- Unique edge IDs generated: `e-{sourceId}-{targetId}`

**Code**:

```typescript
export function dagDefinitionToSvelteFlow(dag: DagDefinition | null): {
  nodes: SvelteFlowNode[];
  edges: SvelteFlowEdge[];
} {
  if (!dag || !dag.nodes || dag.nodes.length === 0) {
    return { nodes: [], edges: [] };
  }

  // Convert nodes
  const nodes: SvelteFlowNode[] = dag.nodes.map((node) => {
    const position = node.metadata?.position
      ? { x: node.metadata.position[0], y: node.metadata.position[1] }
      : { x: 0, y: 0 }; // Default: will be laid out by Dagre

    // Determine node type for styling
    let nodeType: "ingestion" | "operator" | "output" = "operator";
    if (node.node_type && typeof node.node_type === "object") {
      if ("type" in node.node_type) {
        const nt = node.node_type as { type: string };
        if (nt.type === "ingestion") nodeType = "ingestion";
        else if (nt.type === "output") nodeType = "output";
        else if (nt.type === "operator") nodeType = "operator";
      }
    }

    return {
      id: typeof node.id === "string" ? node.id : node.id.toString(),
      type: "default",
      position,
      data: {
        label: `${node.name}\n(${nodeType})`,
        nodeType,
        nodeConfig: node.node_type,
        nodeName: node.name,
        metadata: node.metadata,
      },
      sourcePosition: "bottom",
      targetPosition: "top",
    };
  });

  // Convert dependencies to edges
  const edges: SvelteFlowEdge[] = [];
  dag.nodes.forEach((node) => {
    node.dependencies.forEach((depId) => {
      const sourceId = typeof depId === "string" ? depId : depId.toString();
      const targetId =
        typeof node.id === "string" ? node.id : node.id.toString();

      edges.push({
        id: `e-${sourceId}-${targetId}`,
        source: sourceId,
        target: targetId,
        type: "smoothstep",
        animated: false,
      });
    });
  });

  return { nodes, edges };
}
```

**Result**: `{ nodes: SvelteFlowNode[], edges: SvelteFlowEdge[] }` ready for Svelte Flow rendering.

---

### 7. **State Update in Editor Component**

**Location**: `src/lib/components/pages/home/content-main/content-dag-editor/content-dag-editor.svelte`

**Code**:

```typescript
async function loadPipeline() {
  if (!selectedId) {
    dagDefinition = null;
    svelteFlowNodes = [];
    svelteFlowEdges = [];
    isDirty = false;
    return;
  }

  try {
    isLoading = true;
    error = null;

    // Fetch from Supabase
    const pipeline = await getPipelineById(selectedId);
    dagDefinition = (pipeline?.dag_definition as DagDefinition) ?? null;

    if (dagDefinition) {
      // Convert to Svelte Flow format
      const converted = dagDefinitionToSvelteFlow(dagDefinition);
      svelteFlowNodes = converted.nodes;
      svelteFlowEdges = converted.edges;
    } else {
      svelteFlowNodes = [];
      svelteFlowEdges = [];
    }

    isDirty = false;
  } catch (e: any) {
    error = e?.toString?.() ?? "Failed to load pipeline";
    console.error("Failed to load pipeline:", e);
  } finally {
    isLoading = false;
  }
}
```

**State Variables**:

- `dagDefinition: DagDefinition | null` - Original backend format (for saving)
- `svelteFlowNodes: SvelteFlowNode[]` - Converted nodes for rendering
- `svelteFlowEdges: SvelteFlowEdge[]` - Converted edges for rendering
- `isLoading: boolean` - Loading state
- `error: string | null` - Error state

**Result**: Component state updated with converted Svelte Flow data.

---

### 8. **Props Passing to Svelte Flow Component**

**Location**: `src/lib/components/pages/home/content-main/content-dag-editor/content-dag-editor.svelte`

**Template**:

```svelte
{#if selectedId && svelteFlowNodes.length > 0}
  <SFPipelineFlow nodes={svelteFlowNodes} edges={svelteFlowEdges} />
{:else if selectedId}
  <div>Empty Pipeline</div>
{:else}
  <div>No Pipeline Selected</div>
{/if}
```

**Result**: `SF-pipeline-flow.svelte` receives nodes and edges as props.

---

### 9. **Svelte Flow Component: Prop Reception**

**Location**: `src/lib/components/SF/SF-pipeline-flow.svelte`

**Code**:

```typescript
interface Props {
  nodes?: SvelteFlowNode[];
  edges?: SvelteFlowEdge[];
}

let { nodes: propNodes = [], edges: propEdges = [] }: Props = $props();

// Internal state for Svelte Flow
let nodes = $state.raw<Node[]>([]);
let edges = $state.raw<Edge[]>([]);
```

**Result**: Props received and stored in internal reactive state.

---

### 10. **Reactive Prop Updates** (Effect-Based Sync)

**Location**: `src/lib/components/SF/SF-pipeline-flow.svelte`

**Code**:

```typescript
$effect(() => {
  const propNodesLength = propNodes.length;
  const propEdgesLength = propEdges.length;

  // Check if props actually changed
  const propsChanged =
    nodes.length !== propNodesLength ||
    edges.length !== propEdgesLength ||
    (propNodesLength > 0 &&
      nodes.length > 0 &&
      nodes[0]?.id !== propNodes[0]?.id);

  if (propsChanged) {
    nodes = propNodes as Node[];
    edges = propEdges as Edge[];
    hasAppliedLayout = false;

    // Auto-apply layout if nodes don't have positions
    const needsLayout = nodes.some(
      (n) => !n.position || (n.position.x === 0 && n.position.y === 0),
    );
    if (needsLayout) {
      requestAnimationFrame(() => {
        applyLayout();
      });
    }
  }
});
```

**Key Features**:

- **Change Detection**: Compares lengths and first node ID to avoid unnecessary updates
- **Layout Flag**: `hasAppliedLayout` prevents re-layout loops
- **Deferred Layout**: `requestAnimationFrame` prevents ResizeObserver errors

**Result**: Internal `nodes` and `edges` updated when props change.

---

### 11. **Dagre Layout Application** (Automatic Positioning)

**Location**: `src/lib/components/SF/SF-pipeline-flow.svelte`

**Function**: `applyLayout()`

**Code**:

```typescript
function applyLayout() {
  if (nodes.length === 0) return;

  const dagreGraph = new dagre.graphlib.Graph();
  dagreGraph.setDefaultEdgeLabel(() => ({}));
  dagreGraph.setGraph({ rankdir: layoutDirection }); // 'TB' or 'LR'

  const nodeWidth = 200;
  const nodeHeight = 50;

  // Add nodes to dagre graph
  nodes.forEach((node) => {
    dagreGraph.setNode(node.id, { width: nodeWidth, height: nodeHeight });
  });

  // Add edges to dagre graph
  edges.forEach((edge) => {
    dagreGraph.setEdge(edge.source, edge.target);
  });

  // Compute layout
  dagre.layout(dagreGraph);

  // Update node positions
  const updatedNodes = nodes.map((node) => {
    const nodeWithPosition = dagreGraph.node(node.id);
    return {
      ...node,
      position: {
        x: nodeWithPosition.x - nodeWidth / 2,
        y: nodeWithPosition.y - nodeHeight / 2,
      },
      sourcePosition:
        layoutDirection === "LR" ? Position.Right : Position.Bottom,
      targetPosition: layoutDirection === "LR" ? Position.Left : Position.Top,
    } as Node;
  });

  nodes = updatedNodes;
  hasAppliedLayout = true;
}
```

**Layout Process**:

1. **Create Dagre Graph**: Initialize graph with direction (`TB` = top-to-bottom, `LR` = left-to-right)
2. **Add Nodes**: Register all nodes with dimensions (200x50)
3. **Add Edges**: Register all edges (source → target)
4. **Compute Layout**: Dagre calculates hierarchical positions
5. **Update Positions**: Apply calculated positions to nodes
6. **Set Connection Points**: Configure `sourcePosition`/`targetPosition` for edges

**Result**: Nodes positioned in hierarchical layout (top-to-bottom or left-to-right).

---

### 12. **Svelte Flow Rendering** (Visual Graph)

**Location**: `src/lib/components/SF/SF-pipeline-flow.svelte`

**Template**:

```svelte
<SvelteFlow
  nodes={nodes}
  edges={edges}
  fitView
  connectionLineType={ConnectionLineType.SmoothStep}
  defaultEdgeOptions={{ type: 'smoothstep', animated: false }}
  class="w-full h-full"
>
  <Panel position="top-right">
    <button onclick={toggleLayout}>
      Layout: {layoutDirection === 'TB' ? 'Vertical' : 'Horizontal'}
    </button>
  </Panel>
  <Background />
  <Controls />
  <MiniMap />
</SvelteFlow>
```

**Svelte Flow Features**:

- **`fitView`**: Automatically zooms to fit all nodes
- **`connectionLineType={SmoothStep}`**: Curved edges for better visualization
- **`Background`**: Grid pattern for visual reference
- **`Controls`**: Zoom/pan controls
- **`MiniMap`**: Overview of entire graph
- **`Panel`**: Custom UI controls (layout toggle)

**Result**: Interactive DAG visualization rendered in the browser.

---

## 🔄 Reverse Flow: Saving Changes

When the user modifies the graph (moves nodes, adds/removes edges), the reverse conversion happens:

### 1. **Svelte Flow → DagDefinition Conversion**

**Location**: `src/lib/services/dag-converter.ts`

**Function**: `svelteFlowToDagDefinition(nodes, edges, baseDag)`

**Process**:

- Maps `SvelteFlowNode.position` → `DagNode.metadata.position`
- Reconstructs `DagNode.dependencies` from `SvelteFlowEdge[]`
- Preserves original `node_type` configuration from `baseDag`

### 2. **Save to PostgreSQL**

**Location**: `src/lib/services/pipeline-service.ts`

**Function**: `updatePipelineDagDefinition(id, dagDefinition)`

**Code**:

```typescript
await supabase
  .from("pipelines")
  .update({
    dag_definition: dagDefinition,
    updated_at: new Date().toISOString(),
  })
  .eq("id", id);
```

**Result**: Updated `dag_definition` JSONB saved to PostgreSQL.

---

## 📋 Key Files Reference

| File                                    | Purpose                                        |
| --------------------------------------- | ---------------------------------------------- |
| `content-dag-editor-pipeline.svelte.ts` | Global state for selected pipeline ID          |
| `content-dag-editor.svelte`             | Main editor component, orchestrates data flow  |
| `pipeline-service.ts`                   | Supabase PostgREST API calls                   |
| `dag-converter.ts`                      | Bidirectional conversion between formats       |
| `dag-types.ts`                          | TypeScript types matching Rust `DagDefinition` |
| `SF-pipeline-flow.svelte`               | Svelte Flow visual graph component             |

---

## 🎯 Summary

**Complete Flow**:

1. User selects pipeline → `ContentDagPipelineState` updated
2. `$derived` reads selected ID → `$effect` triggers `loadPipeline()`
3. `getPipelineById()` → Supabase PostgREST → PostgreSQL JSONB
4. `dagDefinitionToSvelteFlow()` → Converts to Svelte Flow format
5. Props passed to `SF-pipeline-flow.svelte`
6. `$effect` syncs props → Internal state updated
7. `applyLayout()` → Dagre calculates positions
8. `SvelteFlow` component → Renders interactive graph

**Key Design Decisions**:

- ✅ **JSONB Storage**: Single source of truth in PostgreSQL
- ✅ **Type Conversion**: Clean separation between backend and frontend formats
- ✅ **Reactive State**: Automatic updates via Svelte 5 runes
- ✅ **Layout Automation**: Dagre handles positioning automatically
- ✅ **Position Preservation**: Manual positions saved in `metadata.position`

This architecture ensures type safety, performance, and maintainability throughout the data flow.
