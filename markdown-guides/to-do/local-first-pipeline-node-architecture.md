# Local-First Pipeline/Node Architecture with Automerge CRDT

## Executive Summary

This document describes the refactoring plan to migrate the current pipeline/node architecture from a "sync-first" Supabase Realtime approach to a "local-first" Automerge CRDT approach, following the same patterns being implemented for charts.

**Current State**: Direct database writes via Supabase PostgREST with Realtime subscriptions for UI updates. Network-dependent, no offline support.

**Target State**: Local-first architecture where:

- Edits apply immediately to local CRDT document
- UI updates instantly (optimistic)
- Sync happens in background via Tauri IPC to Rust backend
- Offline support with automatic conflict resolution
- Same patterns as ChartSpec implementation

---

## Architecture Comparison

### Current Architecture (Sync-First)

```
┌─────────────────────────────────────────────────────────────┐
│                    User Interaction                          │
│  Click UDF → Create Node → Add to Pipeline                   │
└────────────────────┬────────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────────┐
│              Frontend Service Layer                          │
│  createNodeFromUdfAndAddToPipeline()                         │
│    ├─ supabase.from('nodes').insert() ← BLOCKS until done   │
│    └─ supabase.from('pipelines').update() ← BLOCKS          │
└────────────────────┬────────────────────────────────────────┘
                     │ Network Required ⚠️
                     ▼
┌─────────────────────────────────────────────────────────────┐
│              PostgreSQL Database                             │
│  ← Publication: supabase_realtime                            │
│  ← UI waits for Realtime event to update                     │
└────────────────────┬────────────────────────────────────────┘
                     │ WebSocket
                     ▼
┌─────────────────────────────────────────────────────────────┐
│              Frontend Realtime Services                      │
│  RealtimeNodesService → Eventually updates UI                │
│  Pipeline Realtime → Reloads nodes                          │
└─────────────────────────────────────────────────────────────┘
```

**Problems**:

- Network latency delays UI feedback
- No offline support
- Complex Realtime subscription management
- Multiple network roundtrips per operation
- Race conditions between INSERT/UPDATE events

### Target Architecture (Local-First with Automerge)

```
┌─────────────────────────────────────────────────────────────┐
│                    User Interaction                          │
│  Click UDF → Create Node → Add to Pipeline                   │
└────────────────────┬────────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────────┐
│              Frontend Store (ContentPipelineState)           │
│  1. Apply optimistic update (UI updates INSTANTLY)          │
│  2. Fire-and-forget sync to CRDT backend                    │
│  ← No blocking, no waiting for network                       │
└────────────────────┬────────────────────────────────────────┘
                     │ Tauri IPC (async, non-blocking)
                     ▼
┌─────────────────────────────────────────────────────────────┐
│              Rust Backend (Automerge)                        │
│  1. Apply patch to CRDT document                             │
│  2. Persist to local storage / Postgres                      │
│  3. Queue sync to remote peers                               │
│  ← Works offline, syncs when connected                       │
└─────────────────────────────────────────────────────────────┘
```

**Benefits**:

- Zero-latency UI updates (optimistic)
- Full offline support
- Automatic conflict resolution
- Simpler frontend code (no Realtime subscriptions)
- Consistent with ChartSpec pattern

---

## Data Model Design

### 1. PipelineSpec (CRDT Document)

Following the ChartSpec pattern, we define `PipelineSpec` as the CRDT document structure:

```typescript
// src/lib/types/PipelineSpec.ts

export interface PipelineSpec {
  version: number; // Schema version for migrations
  id: string; // UUID
  name: string;
  projectId: string;
  description?: string;

  // DAG Structure
  nodes: Record<string, PipelineNodeSpec>; // Map of nodeId -> node spec
  edges: PipelineEdge[];

  // Layout
  layout: {
    positions: Record<string, [number, number]>; // nodeId -> [x, y]
  };

  // Metadata
  metadata: PipelineMetadata;

  // Execution
  execution?: PipelineExecutionConfig;
}

export interface PipelineNodeSpec {
  id: string;
  name: string;
  nodeType: "ingestion" | "operator" | "output";
  nodeConfig: NodeTypeConfig; // Operator details
  inputCurveIds?: string[]; // References to curves
  inputToOutputCurveMapping?: Record<string, string | null>;
  metadata?: NodeMetadata;
}

export interface PipelineEdge {
  id: string; // Edge ID for stable identity
  sourceNodeId: string;
  targetNodeId: string;
  edgeType: "data_flow" | "conditional" | "error";
}

export interface PipelineMetadata {
  description?: string;
  tags?: string[];
  createdAt: string;
  updatedAt: string;
  createdBy?: string;
}
```

### 2. NodeSpec vs PipelineNodeSpec

**Key Decision**: Nodes are embedded in PipelineSpec, not stored separately.

**Rationale**:

- **CRDT Simplicity**: Single document for the entire pipeline (easier sync)
- **Atomic Operations**: Adding node + edge is one change, not two
- **Offline Consistency**: No need to sync multiple tables
- **Matches Chart Pattern**: ChartSpec embeds series config, not separate table

**Migration Path**:

- Existing `nodes` table becomes a **template/library** of reusable node definitions
- Pipeline-specific instances are embedded in `PipelineSpec.nodes`
- Position info moves from `pipeline_node_references` to `PipelineSpec.layout.positions`

---

## Frontend State Management

### 3. Updated ContentPipelineState (Optimistic Updates)

```typescript
// src/lib/components/pages/home/content-main/content-dag-editor/content-pipelines/content-pipeline.svelte.ts

import { setContext, getContext, untrack } from "svelte";
import type { PipelineSpec, PipelineNodeSpec } from "$lib/types/PipelineSpec";

// Tauri invoke wrapper (fire-and-forget pattern)
async function tauriInvoke(
  command: string,
  args: Record<string, any>,
): Promise<void> {
  try {
    const { invoke } = await import("@tauri-apps/api/core");
    await invoke(command, args);
  } catch (err) {
    console.error(`[tauriInvoke] ${command} failed:`, err);
    // Don't throw - fire-and-forget for background sync
  }
}

export class ContentPipelineState {
  // ============================================
  // LOCAL STATE (Optimistic, Immediate Updates)
  // ============================================

  private _pipelines = $state<Map<string, PipelineSpec>>(new Map());
  private _selectedPipelineId = $state<string | null>(null);
  private _selectedNodeId = $state<string | null>(null);
  private _isSyncing = $state(false);
  private _error = $state<string | null>(null);

  // Derived state for reactivity
  get pipelines(): PipelineSpec[] {
    return Array.from(this._pipelines.values());
  }

  get selectedPipeline(): PipelineSpec | null {
    return this._selectedPipelineId
      ? (this._pipelines.get(this._selectedPipelineId) ?? null)
      : null;
  }

  get nodesForSelectedPipeline(): PipelineNodeSpec[] {
    const pipeline = this.selectedPipeline;
    if (!pipeline) return [];
    return Object.values(pipeline.nodes);
  }

  get selectedNode(): PipelineNodeSpec | null {
    const pipeline = this.selectedPipeline;
    if (!pipeline || !this._selectedNodeId) return null;
    return pipeline.nodes[this._selectedNodeId] ?? null;
  }

  get isSyncing(): boolean {
    return this._isSyncing;
  }

  get error(): string | null {
    return this._error;
  }

  // ============================================
  // PIPELINE OPERATIONS (Optimistic + CRDT Sync)
  // ============================================

  /**
   * Load pipelines from CRDT backend (initial load or refresh)
   */
  async loadPipelines(projectId: string): Promise<void> {
    try {
      const { invoke } = await import("@tauri-apps/api/core");
      const specs = await invoke<PipelineSpec[]>("get_project_pipelines", {
        projectId,
      });

      this._pipelines = new Map(specs.map((s) => [s.id, s]));
    } catch (err) {
      this._error = err instanceof Error ? err.message : String(err);
      console.error("[ContentPipelineState] Failed to load pipelines:", err);
    }
  }

  /**
   * Create a new pipeline (optimistic + CRDT sync)
   */
  async createPipeline(params: {
    projectId: string;
    name: string;
    description?: string;
  }): Promise<PipelineSpec | null> {
    const id = crypto.randomUUID();
    const now = new Date().toISOString();

    const spec: PipelineSpec = {
      version: 1,
      id,
      name: params.name,
      projectId: params.projectId,
      description: params.description,
      nodes: {},
      edges: [],
      layout: { positions: {} },
      metadata: {
        createdAt: now,
        updatedAt: now,
      },
    };

    // 1. OPTIMISTIC UPDATE (immediate UI feedback)
    this._pipelines.set(id, spec);

    // 2. CRDT SYNC (fire-and-forget)
    void this.syncToCRDT("create_pipeline_spec", { spec });

    return spec;
  }

  /**
   * Add a node to the selected pipeline (optimistic + CRDT sync)
   */
  addNodeToPipeline(params: {
    nodeSpec: PipelineNodeSpec;
    position?: [number, number];
    sourceNodeId?: string; // Create edge if provided
  }): PipelineNodeSpec | null {
    const pipeline = this.selectedPipeline;
    if (!pipeline) {
      this._error = "No pipeline selected";
      return null;
    }

    // 1. OPTIMISTIC UPDATE
    const updatedNodes = {
      ...pipeline.nodes,
      [params.nodeSpec.id]: params.nodeSpec,
    };
    const updatedPositions = { ...pipeline.layout.positions };

    if (params.position) {
      updatedPositions[params.nodeSpec.id] = params.position;
    }

    const updatedEdges = [...pipeline.edges];
    if (params.sourceNodeId) {
      updatedEdges.push({
        id: crypto.randomUUID(),
        sourceNodeId: params.sourceNodeId,
        targetNodeId: params.nodeSpec.id,
        edgeType: "data_flow",
      });
    }

    const updatedPipeline: PipelineSpec = {
      ...pipeline,
      nodes: updatedNodes,
      edges: updatedEdges,
      layout: { ...pipeline.layout, positions: updatedPositions },
      metadata: { ...pipeline.metadata, updatedAt: new Date().toISOString() },
    };

    this._pipelines.set(pipeline.id, updatedPipeline);

    // 2. CRDT SYNC (fire-and-forget)
    void this.syncToCRDT("update_pipeline_spec", {
      pipelineId: pipeline.id,
      spec: updatedPipeline,
    });

    return params.nodeSpec;
  }

  /**
   * Update a node in the pipeline (optimistic + CRDT sync)
   */
  updateNode<T extends PipelineNodeSpec>(
    nodeId: string,
    updater: (node: T) => T,
  ): PipelineNodeSpec | null {
    const pipeline = this.selectedPipeline;
    if (!pipeline) return null;

    const node = pipeline.nodes[nodeId] as T | undefined;
    if (!node) return null;

    // 1. OPTIMISTIC UPDATE
    const updatedNode = updater(node);
    const updatedPipeline: PipelineSpec = {
      ...pipeline,
      nodes: { ...pipeline.nodes, [nodeId]: updatedNode },
      metadata: { ...pipeline.metadata, updatedAt: new Date().toISOString() },
    };

    this._pipelines.set(pipeline.id, updatedPipeline);

    // 2. CRDT SYNC (fire-and-forget)
    void this.syncToCRDT("update_pipeline_spec", {
      pipelineId: pipeline.id,
      spec: updatedPipeline,
    });

    return updatedNode;
  }

  /**
   * Remove a node from the pipeline (optimistic + CRDT sync)
   */
  removeNode(nodeId: string): boolean {
    const pipeline = this.selectedPipeline;
    if (!pipeline) return false;

    // 1. OPTIMISTIC UPDATE
    const { [nodeId]: removed, ...remainingNodes } = pipeline.nodes;
    const { [nodeId]: removedPos, ...remainingPositions } =
      pipeline.layout.positions;
    const filteredEdges = pipeline.edges.filter(
      (e) => e.sourceNodeId !== nodeId && e.targetNodeId !== nodeId,
    );

    const updatedPipeline: PipelineSpec = {
      ...pipeline,
      nodes: remainingNodes,
      edges: filteredEdges,
      layout: { ...pipeline.layout, positions: remainingPositions },
      metadata: { ...pipeline.metadata, updatedAt: new Date().toISOString() },
    };

    this._pipelines.set(pipeline.id, updatedPipeline);

    // Clear selection if removed node was selected
    if (this._selectedNodeId === nodeId) {
      this._selectedNodeId = null;
    }

    // 2. CRDT SYNC (fire-and-forget)
    void this.syncToCRDT("update_pipeline_spec", {
      pipelineId: pipeline.id,
      spec: updatedPipeline,
    });

    return true;
  }

  /**
   * Update node position (optimistic + CRDT sync)
   */
  updateNodePosition(nodeId: string, position: [number, number]): void {
    const pipeline = this.selectedPipeline;
    if (!pipeline) return;

    // 1. OPTIMISTIC UPDATE
    const updatedPipeline: PipelineSpec = {
      ...pipeline,
      layout: {
        ...pipeline.layout,
        positions: { ...pipeline.layout.positions, [nodeId]: position },
      },
    };

    this._pipelines.set(pipeline.id, updatedPipeline);

    // 2. CRDT SYNC (fire-and-forget, debounced for drag operations)
    void this.syncToCRDT("update_pipeline_spec", {
      pipelineId: pipeline.id,
      spec: updatedPipeline,
    });
  }

  // ============================================
  // SELECTION
  // ============================================

  selectPipeline(pipelineId: string | null): void {
    this._selectedPipelineId = pipelineId;
    this._selectedNodeId = null; // Clear node selection when pipeline changes
  }

  selectNode(nodeId: string | null): void {
    this._selectedNodeId = nodeId;
  }

  // ============================================
  // CRDT SYNC (Fire-and-Forget)
  // ============================================

  private async syncToCRDT(
    command: string,
    args: Record<string, any>,
  ): Promise<void> {
    this._isSyncing = true;
    try {
      await tauriInvoke(command, args);
    } catch (err) {
      console.error(
        `[ContentPipelineState] CRDT sync failed (${command}):`,
        err,
      );
      // Don't throw - UI already updated optimistically
      // Could implement retry logic or error queue here
    } finally {
      this._isSyncing = false;
    }
  }

  // ============================================
  // REMOTE UPDATE HANDLING (From Other Collaborators)
  // ============================================

  /**
   * Setup CRDT listener for remote updates
   */
  async setupCRDTListener(): Promise<() => void> {
    const { listen } = await import("@tauri-apps/api/event");

    const unlisten = await listen<{ pipelineId: string; spec: PipelineSpec }>(
      "pipeline-updated",
      ({ payload: { pipelineId, spec } }) => {
        console.log(
          "[ContentPipelineState] Remote update received:",
          pipelineId,
        );
        this.handleRemoteUpdate(pipelineId, spec);
      },
    );

    return unlisten;
  }

  /**
   * Handle remote CRDT update from collaborator
   */
  handleRemoteUpdate(pipelineId: string, spec: PipelineSpec): void {
    // Only update if not the currently editing pipeline to avoid conflicts
    // In practice, Automerge handles conflicts automatically
    this._pipelines.set(pipelineId, spec);
  }
}

// ============================================
// CONTEXT API
// ============================================

const CONTENT_PIPELINE_STATE_KEY = Symbol("content-pipeline-state");

export function setContentPipelineState() {
  const state = new ContentPipelineState();
  setContext(CONTENT_PIPELINE_STATE_KEY, state);
  return state;
}

export function getContentPipelineState() {
  const state = getContext<ContentPipelineState>(CONTENT_PIPELINE_STATE_KEY);
  if (!state) {
    throw new Error(
      "ContentPipelineState not found. Did you call setContentPipelineState() in the parent layout?",
    );
  }
  return state;
}
```

---

## Database Migrations

### 4. SQL for CRDT Support

Following the pattern from `crdt_architecture.md`, we need two new tables:

```sql
-- Migration: Add CRDT support for pipelines
-- File: YYYYMMDDHHMMSS-add-crdt-pipeline-support.sql

-- 1. Add pipeline_spec JSONB column to pipelines table (for snapshot storage)
ALTER TABLE public.pipelines
ADD COLUMN IF NOT EXISTS pipeline_spec JSONB DEFAULT '{}';

-- Comment for documentation
COMMENT ON COLUMN public.pipelines.pipeline_spec IS
  'Complete PipelineSpec CRDT document stored as JSONB. Source of truth for pipeline structure including nodes, edges, and layout.';

-- 2. Create pipeline snapshots table (for CRDT state persistence)
CREATE TABLE IF NOT EXISTS public.pipeline_snapshots (
  pipeline_id UUID PRIMARY KEY REFERENCES public.pipelines(id) ON DELETE CASCADE,
  snapshot BYTEA NOT NULL,  -- Automerge binary snapshot
  actor_id TEXT NOT NULL,   -- Last actor who modified
  version BIGINT NOT NULL DEFAULT 1,
  created_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
  updated_at TIMESTAMPTZ DEFAULT NOW() NOT NULL
);

-- Index for version queries
CREATE INDEX IF NOT EXISTS idx_pipeline_snapshots_version
ON public.pipeline_snapshots(version DESC);

-- Comment for documentation
COMMENT ON TABLE public.pipeline_snapshots IS
  'Stores Automerge CRDT snapshots for pipelines. Binary format for efficient sync.';

-- 3. Create pipeline operations log (for incremental sync)
CREATE TABLE IF NOT EXISTS public.pipeline_ops (
  id BIGSERIAL NOT NULL,
  pipeline_id UUID NOT NULL REFERENCES public.pipelines(id) ON DELETE CASCADE,
  actor_id TEXT NOT NULL,
  op_data BYTEA NOT NULL,   -- Automerge change binary
  created_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
  PRIMARY KEY (pipeline_id, id)
);

-- Index for efficient op retrieval by pipeline and ordering
CREATE INDEX IF NOT EXISTS idx_pipeline_ops_pipeline_created
ON public.pipeline_ops(pipeline_id, created_at DESC);

-- Comment for documentation
COMMENT ON TABLE public.pipeline_ops IS
  'Stores incremental Automerge operations for pipeline collaboration. Used for sync between clients.';

-- 4. Add RLS policies for CRDT tables
ALTER TABLE public.pipeline_snapshots ENABLE ROW LEVEL SECURITY;
ALTER TABLE public.pipeline_ops ENABLE ROW LEVEL SECURITY;

-- Snapshots: Read access for authenticated users
CREATE POLICY "Users can view pipeline snapshots for their projects"
ON public.pipeline_snapshots FOR SELECT
TO authenticated
USING (
  pipeline_id IN (
    SELECT p.id FROM public.pipelines p
    WHERE p.project_id IN (
      SELECT id FROM public.projects WHERE owner_id = auth.uid()
    )
  )
);

-- Snapshots: Write access for authenticated users
CREATE POLICY "Users can update pipeline snapshots for their projects"
ON public.pipeline_snapshots FOR ALL
TO authenticated
USING (
  pipeline_id IN (
    SELECT p.id FROM public.pipelines p
    WHERE p.project_id IN (
      SELECT id FROM public.projects WHERE owner_id = auth.uid()
    )
  )
);

-- Ops: Read access for authenticated users
CREATE POLICY "Users can view pipeline ops for their projects"
ON public.pipeline_ops FOR SELECT
TO authenticated
USING (
  pipeline_id IN (
    SELECT p.id FROM public.pipelines p
    WHERE p.project_id IN (
      SELECT id FROM public.projects WHERE owner_id = auth.uid()
    )
  )
);

-- Ops: Write access for authenticated users
CREATE POLICY "Users can insert pipeline ops for their projects"
ON public.pipeline_ops FOR INSERT
TO authenticated
WITH CHECK (
  pipeline_id IN (
    SELECT p.id FROM public.pipelines p
    WHERE p.project_id IN (
      SELECT id FROM public.projects WHERE owner_id = auth.uid()
    )
  )
);

-- 5. Create function to migrate existing dag_definition to pipeline_spec
CREATE OR REPLACE FUNCTION migrate_pipeline_to_spec(p_id UUID)
RETURNS JSONB AS $$
DECLARE
  pipeline_row RECORD;
  node_ids TEXT[];
  node_row RECORD;
  nodes_obj JSONB := '{}';
  positions_obj JSONB := '{}';
BEGIN
  -- Get pipeline data
  SELECT * INTO pipeline_row FROM public.pipelines WHERE id = p_id;

  IF pipeline_row IS NULL THEN
    RETURN NULL;
  END IF;

  -- Get node_ids from dag_definition
  node_ids := ARRAY(
    SELECT jsonb_array_elements_text(
      COALESCE(pipeline_row.dag_definition->'node_ids', '[]'::jsonb)
    )
  );

  -- Build nodes object from nodes table
  FOR node_row IN
    SELECT n.*, pnr.position_x, pnr.position_y
    FROM public.nodes n
    LEFT JOIN public.pipeline_node_references pnr
      ON pnr.node_id = n.id AND pnr.pipeline_id = p_id
    WHERE n.id = ANY(node_ids)
  LOOP
    nodes_obj := nodes_obj || jsonb_build_object(
      node_row.id::text,
      jsonb_build_object(
        'id', node_row.id,
        'name', node_row.name,
        'nodeType', node_row.node_type,
        'nodeConfig', node_row.node_config,
        'inputCurveIds', COALESCE(node_row.input_curve_ids, ARRAY[]::UUID[]),
        'inputToOutputCurveMapping', COALESCE(node_row.input_to_output_curve_mapping, '{}'::jsonb),
        'metadata', node_row.metadata
      )
    );

    -- Add position if available
    IF node_row.position_x IS NOT NULL AND node_row.position_y IS NOT NULL THEN
      positions_obj := positions_obj || jsonb_build_object(
        node_row.id::text,
        jsonb_build_array(node_row.position_x, node_row.position_y)
      );
    END IF;
  END LOOP;

  -- Return PipelineSpec structure
  RETURN jsonb_build_object(
    'version', 1,
    'id', pipeline_row.id,
    'name', pipeline_row.name,
    'projectId', pipeline_row.project_id,
    'description', pipeline_row.description,
    'nodes', nodes_obj,
    'edges', COALESCE(pipeline_row.dag_definition->'edges', '[]'::jsonb),
    'layout', jsonb_build_object('positions', positions_obj),
    'metadata', jsonb_build_object(
      'description', pipeline_row.description,
      'tags', COALESCE(pipeline_row.tags, ARRAY[]::TEXT[]),
      'createdAt', pipeline_row.created_at,
      'updatedAt', pipeline_row.updated_at
    )
  );
END;
$$ LANGUAGE plpgsql;

-- 6. Migrate all existing pipelines
UPDATE public.pipelines
SET pipeline_spec = migrate_pipeline_to_spec(id)
WHERE pipeline_spec = '{}'::jsonb OR pipeline_spec IS NULL;

-- 7. Add trigger to keep pipeline_spec in sync (during migration period)
CREATE OR REPLACE FUNCTION sync_pipeline_spec_on_update()
RETURNS TRIGGER AS $$
BEGIN
  -- Only update pipeline_spec if dag_definition changed and pipeline_spec is empty
  -- This preserves CRDT updates while allowing legacy updates during migration
  IF NEW.dag_definition IS DISTINCT FROM OLD.dag_definition
     AND (NEW.pipeline_spec = '{}'::jsonb OR NEW.pipeline_spec IS NULL) THEN
    NEW.pipeline_spec := migrate_pipeline_to_spec(NEW.id);
  END IF;
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_sync_pipeline_spec
BEFORE UPDATE ON public.pipelines
FOR EACH ROW
EXECUTE FUNCTION sync_pipeline_spec_on_update();
```

---

## Rust Backend Commands

### 5. Tauri Commands for CRDT Operations

The Rust backend needs these Tauri commands:

```rust
// src-tauri/src/pipeline_commands.rs

use automerge::{AutoCommit, transaction::Transactable};
use serde::{Deserialize, Serialize};
use tauri::{command, State};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineSpec {
    pub version: i32,
    pub id: String,
    pub name: String,
    #[serde(rename = "projectId")]
    pub project_id: String,
    pub description: Option<String>,
    pub nodes: std::collections::HashMap<String, PipelineNodeSpec>,
    pub edges: Vec<PipelineEdge>,
    pub layout: PipelineLayout,
    pub metadata: PipelineMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineNodeSpec {
    pub id: String,
    pub name: String,
    #[serde(rename = "nodeType")]
    pub node_type: String,
    #[serde(rename = "nodeConfig")]
    pub node_config: serde_json::Value,
    #[serde(rename = "inputCurveIds")]
    pub input_curve_ids: Option<Vec<String>>,
    #[serde(rename = "inputToOutputCurveMapping")]
    pub input_to_output_curve_mapping: Option<std::collections::HashMap<String, Option<String>>>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineEdge {
    pub id: String,
    #[serde(rename = "sourceNodeId")]
    pub source_node_id: String,
    #[serde(rename = "targetNodeId")]
    pub target_node_id: String,
    #[serde(rename = "edgeType")]
    pub edge_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineLayout {
    pub positions: std::collections::HashMap<String, [f64; 2]>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineMetadata {
    pub description: Option<String>,
    pub tags: Option<Vec<String>>,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    #[serde(rename = "createdBy")]
    pub created_by: Option<String>,
}

// ============================================
// TAURI COMMANDS
// ============================================

#[command]
pub async fn get_project_pipelines(
    project_id: String,
    state: State<'_, AppState>,
) -> Result<Vec<PipelineSpec>, String> {
    let client = state.db_client.lock().await;

    let rows = client
        .query(
            "SELECT pipeline_spec FROM pipelines WHERE project_id = $1 AND is_active = true",
            &[&uuid::Uuid::parse_str(&project_id).map_err(|e| e.to_string())?],
        )
        .await
        .map_err(|e| e.to_string())?;

    let specs: Vec<PipelineSpec> = rows
        .iter()
        .filter_map(|row| {
            let spec_json: serde_json::Value = row.get("pipeline_spec");
            serde_json::from_value(spec_json).ok()
        })
        .collect();

    Ok(specs)
}

#[command]
pub async fn get_pipeline_spec(
    pipeline_id: String,
    state: State<'_, AppState>,
) -> Result<PipelineSpec, String> {
    let client = state.db_client.lock().await;

    let row = client
        .query_one(
            "SELECT pipeline_spec FROM pipelines WHERE id = $1",
            &[&uuid::Uuid::parse_str(&pipeline_id).map_err(|e| e.to_string())?],
        )
        .await
        .map_err(|e| e.to_string())?;

    let spec_json: serde_json::Value = row.get("pipeline_spec");
    serde_json::from_value(spec_json).map_err(|e| e.to_string())
}

#[command]
pub async fn create_pipeline_spec(
    spec: PipelineSpec,
    state: State<'_, AppState>,
    app: tauri::AppHandle,
) -> Result<(), String> {
    let client = state.db_client.lock().await;

    let spec_json = serde_json::to_value(&spec).map_err(|e| e.to_string())?;

    // Insert into pipelines table with pipeline_spec
    client
        .execute(
            r#"
            INSERT INTO pipelines (id, name, description, project_id, pipeline_spec, dag_definition)
            VALUES ($1, $2, $3, $4, $5, $5)
            "#,
            &[
                &uuid::Uuid::parse_str(&spec.id).map_err(|e| e.to_string())?,
                &spec.name,
                &spec.description,
                &uuid::Uuid::parse_str(&spec.project_id).map_err(|e| e.to_string())?,
                &spec_json,
            ],
        )
        .await
        .map_err(|e| e.to_string())?;

    // Emit event for other listeners
    app.emit_all("pipeline-created", PipelineCreatedEvent {
        pipeline_id: spec.id.clone(),
        spec: spec.clone(),
    }).map_err(|e| e.to_string())?;

    Ok(())
}

#[command]
pub async fn update_pipeline_spec(
    pipeline_id: String,
    spec: PipelineSpec,
    state: State<'_, AppState>,
    app: tauri::AppHandle,
) -> Result<(), String> {
    let mut store = state.pipeline_store.lock().await;
    let client = state.db_client.lock().await;

    // 1. Apply CRDT change (if using Automerge document store)
    // store.apply_change(&pipeline_id, &spec)?;

    let spec_json = serde_json::to_value(&spec).map_err(|e| e.to_string())?;

    // 2. Persist to PostgreSQL
    client
        .execute(
            r#"
            UPDATE pipelines
            SET pipeline_spec = $1, updated_at = NOW()
            WHERE id = $2
            "#,
            &[
                &spec_json,
                &uuid::Uuid::parse_str(&pipeline_id).map_err(|e| e.to_string())?,
            ],
        )
        .await
        .map_err(|e| e.to_string())?;

    // 3. Emit event for other collaborators
    app.emit_all("pipeline-updated", PipelineUpdateEvent {
        pipeline_id,
        spec: spec.clone(),
    }).map_err(|e| e.to_string())?;

    Ok(())
}

#[command]
pub async fn delete_pipeline_spec(
    pipeline_id: String,
    state: State<'_, AppState>,
    app: tauri::AppHandle,
) -> Result<(), String> {
    let client = state.db_client.lock().await;

    // Soft delete
    client
        .execute(
            "UPDATE pipelines SET is_active = false, updated_at = NOW() WHERE id = $1",
            &[&uuid::Uuid::parse_str(&pipeline_id).map_err(|e| e.to_string())?],
        )
        .await
        .map_err(|e| e.to_string())?;

    // Emit event
    app.emit_all("pipeline-deleted", PipelineDeletedEvent {
        pipeline_id,
    }).map_err(|e| e.to_string())?;

    Ok(())
}

// Event types
#[derive(Debug, Clone, Serialize)]
struct PipelineCreatedEvent {
    pipeline_id: String,
    spec: PipelineSpec,
}

#[derive(Debug, Clone, Serialize)]
struct PipelineUpdateEvent {
    pipeline_id: String,
    spec: PipelineSpec,
}

#[derive(Debug, Clone, Serialize)]
struct PipelineDeletedEvent {
    pipeline_id: String,
}
```

---

## Migration Strategy

### 6. Phase-Based Migration Plan

#### Phase 1: Database Schema (Week 1)

- [ ] Add `pipeline_spec` column to `pipelines` table
- [ ] Create `pipeline_snapshots` and `pipeline_ops` tables
- [ ] Create migration function to convert `dag_definition` → `pipeline_spec`
- [ ] Run migration on existing pipelines
- [ ] Add sync trigger for backward compatibility

#### Phase 2: Rust Backend (Week 2)

- [ ] Implement `PipelineSpec` structs in Rust
- [ ] Add Tauri commands for CRDT operations
- [ ] Implement Automerge document store (optional, can start with JSONB)
- [ ] Add event emission for collaboration

#### Phase 3: Frontend State (Week 3)

- [ ] Refactor `ContentPipelineState` to optimistic pattern
- [ ] Update node-service.ts to use new CRDT commands
- [ ] Remove Supabase Realtime subscriptions
- [ ] Add Tauri event listeners for remote updates

#### Phase 4: UI Components (Week 4)

- [ ] Update `content-library-udf-category-item.svelte` to use new state
- [ ] Update `SF-pipeline-flow.svelte` to use new state
- [ ] Update `content-selected-pipeline-blocks.svelte`
- [ ] Test offline scenarios

#### Phase 5: Cleanup (Week 5)

- [ ] Remove `RealtimeNodesService`
- [ ] Deprecate direct Supabase calls in node-service.ts
- [ ] Update documentation
- [ ] Performance testing

---

## Backward Compatibility

### 7. Dual-Write Strategy

During migration, maintain backward compatibility:

```typescript
// Temporary dual-write: Both Supabase and CRDT
async function addNodeToPipeline(params: {...}): Promise<void> {
  // 1. Optimistic CRDT update (new pattern)
  pipelineState.addNodeToPipeline({...});

  // 2. Legacy Supabase update (for backward compatibility)
  if (featureFlags.legacySupabaseWrites) {
    await supabase.from('pipelines').update({...});
    await supabase.from('pipeline_node_references').upsert({...});
  }
}
```

---

## Benefits Summary

| Aspect                  | Current (Supabase Realtime) | Target (Automerge CRDT) |
| ----------------------- | --------------------------- | ----------------------- |
| **UI Latency**          | Waits for DB roundtrip      | Instant (optimistic)    |
| **Offline Support**     | ❌ None                     | ✅ Full offline         |
| **Conflict Resolution** | Manual/race conditions      | ✅ Automatic (CRDT)     |
| **Network Dependency**  | Required for all ops        | Background sync only    |
| **Code Complexity**     | High (Realtime + REST)      | Lower (single pattern)  |
| **Consistency**         | Eventual (events)           | Strong (CRDT merge)     |
| **Collaboration**       | Real-time events            | CRDT sync               |

---

## Automerge-Repo Integration (2024-12-10)

### Architecture Overview

The pipeline system now uses `automerge-repo` with `automerge-repo-svelte-store` for true CRDT persistence:

```
┌─────────────────────────────────────────────────────────────────────┐
│  Frontend (SvelteKit)                                               │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │  Automerge Repo (pipeline-repo.ts)                           │   │
│  │  - IndexedDB storage (immediate persistence)                 │   │
│  │  - BroadcastChannel (tab-to-tab sync)                        │   │
│  └─────────────────────────────────────────────────────────────┘   │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │  ContentPipelineState (content-pipeline.svelte.ts)           │   │
│  │  - Uses AutomergeDocumentStore for each pipeline             │   │
│  │  - doc.change() for CRDT mutations (instant)                 │   │
│  │  - Automatic reactivity via Svelte store subscription        │   │
│  └─────────────────────────────────────────────────────────────┘   │
└───────────────────────────┬─────────────────────────────────────────┘
                            │ Tauri IPC (URL registry)
┌───────────────────────────▼─────────────────────────────────────────┐
│  Backend (Rust)                                                     │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │  PostgreSQL URL Registry                                     │   │
│  │  - Stores automerge_url for document discovery               │   │
│  │  - pipelines.automerge_url column                            │   │
│  │  - project_automerge_indexes table                           │   │
│  └─────────────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────────────┘
```

### Key Files

| File                                        | Purpose                                                    |
| ------------------------------------------- | ---------------------------------------------------------- |
| `$lib/pipelines/repo/pipeline-repo.ts`      | Automerge-repo singleton with IndexedDB + BroadcastChannel |
| `$lib/pipelines/repo/pipeline-store.ts`     | Typed wrapper around automerge-repo-svelte-store           |
| `$lib/pipelines/types/pipeline-doc-type.ts` | PipelineDocument CRDT type definitions                     |
| `$lib/pipelines/index.ts`                   | Module exports                                             |
| `content-pipeline.svelte.ts`                | Updated to use automerge document stores                   |

### Data Flow

1. **Create Pipeline**:
   - `createPipeline()` → `automergeStore.create()` → IndexedDB (instant)
   - `registerPipelineUrl()` → Tauri IPC → PostgreSQL (background)

2. **Modify Pipeline**:
   - `doc.change((d) => { ... })` → IndexedDB (instant)
   - BroadcastChannel syncs to other tabs automatically

3. **Load Pipelines**:
   - `get_project_pipeline_urls` → PostgreSQL → automerge URLs
   - `automergeStore.find(url)` → IndexedDB → reactive document

### URL Registry SQL Migration

New migration `003-automerge-url-registry.sql` adds:

- `pipelines.automerge_url` column
- `charts.automerge_url` column
- `project_automerge_indexes` table
- Helper functions and RLS policies

---

## Implementation Files Created

The following files have been created/updated as part of this implementation:

### 1. SQL Migration

- **File**: `sql/migrations/002-crdt-pipeline-node-architecture.sql`
- **Purpose**: Adds CRDT support columns and tables
- **Key Changes**:
  - `pipelines.pipeline_spec` JSONB column
  - `nodes.node_spec` JSONB column
  - `pipeline_snapshots` table (Automerge binary snapshots)
  - `pipeline_ops` table (operation log for incremental sync)
  - `node_snapshots` and `node_ops` tables
  - `pipeline_actors` table (presence tracking)
  - `crdt_version` columns for optimistic locking
  - Helper functions and triggers for auto-incrementing versions

### 2. Rust Backend

- **Module**: `src-tauri/src/pipeline_spec/`
  - `mod.rs` - Module exports
  - `types.rs` - PipelineSpec, NodeSpec, PipelineEdge types
  - `commands.rs` - Tauri commands for CRDT operations

- **Tauri Commands**:
  - `get_pipeline_spec` - Get single pipeline
  - `get_project_pipelines` - Get all pipelines for project
  - `create_pipeline_spec` - Create new pipeline
  - `update_pipeline_spec` - Update pipeline
  - `delete_pipeline_spec` - Soft delete pipeline
  - `add_node_to_pipeline` - Add node to pipeline
  - `remove_node_from_pipeline` - Remove node
  - `update_node_in_pipeline` - Update node
  - `add_edge_to_pipeline` - Add edge
  - `remove_edge_from_pipeline` - Remove edge
  - `update_node_position` - Update single node position
  - `batch_update_node_positions` - Batch position updates

### 3. Frontend State

- **File**: `src/lib/components/pages/home/content-main/content-dag-editor/content-pipelines/content-pipeline.svelte.ts`
- **Class**: `ContentPipelineState`
- **Key Features**:
  - Optimistic local updates (instant UI feedback)
  - Fire-and-forget CRDT sync via Tauri IPC
  - Event listeners for remote updates (`pipeline-updated`, `node-updated`)
  - Debounced position sync for drag operations
  - TypeScript types matching Rust backend

### 4. Main.rs Updates

- **File**: `src-tauri/src/main.rs`
- **Changes**:
  - Added `mod pipeline_spec;` import
  - Registered all pipeline CRDT commands in `invoke_handler`

### 5. UI Component Updates (2024-12-10)

The following UI components were updated to use the new Automerge-based state:

#### content-pipelines.svelte

```svelte
<script lang="ts">
  import { getContentPipelineState, type PipelineWithNodes } from "./content-pipeline.svelte.ts";

  const pipelineState = getContentPipelineState();

  // Derived: pipelines from Automerge state (reactive)
  let pipelines = $derived(pipelineState.allPipelinesWithNodes);
  let isLoading = $derived(pipelineState.isSyncing);
  let error = $derived(pipelineState.syncError);
</script>
```

**Changes:**

- Removed `supabase` import and Realtime subscription
- Removed `listPipelinesByProject()` calls
- Now uses `pipelineState.allPipelinesWithNodes` (reactive Automerge state)
- Loading and error states derived from `pipelineState.isSyncing` and `pipelineState.syncError`

#### content-pipelines-overview.svelte

```svelte
<script lang="ts">
  import { getContentPipelineState } from "../content-pipeline.svelte.ts";

  const pipelineState = getContentPipelineState();
  let pipelines = $derived(pipelineState.allPipelinesWithNodes);
</script>
```

**Changes:**

- Removed `listPipelinesByProject()` fetch call
- Now derives pipelines from `pipelineState.allPipelinesWithNodes`
- Added `(p.id)` key to `#each` loop for proper reactivity

#### content-pipeline-item.svelte

```svelte
<script lang="ts">
  import { getContentPipelineState, type PipelineWithNodes } from "$lib/components/...";

  interface Props { pipeline: PipelineWithNodes; }
  let { pipeline }: Props = $props();

  function handleClick() {
    pipelineState.setSelectedPipelineId(pipeline.id);
  }
</script>

<button type="button" onclick={handleClick} class="...">
  <div>{pipeline.name}</div>
  <div>{pipeline.nodes.length} nodes</div>
</button>
```

**Changes:**

- Changed from `PipelineRow` type to `PipelineWithNodes` type
- Added click handler to set selected pipeline via `pipelineState.setSelectedPipelineId()`
- Changed from `<div>` to `<button>` for proper accessibility
- Now displays node count instead of version number

---

## Running the Migration

```bash
# 1. Run the SQL migration
psql -d your_database -f sql/migrations/002-crdt-pipeline-node-architecture.sql

# 2. Build the Tauri app (includes new Rust commands)
cd src-tauri && cargo build

# 3. The frontend state is already updated - no additional steps needed
```

---

## Testing the Implementation

```typescript
// Example usage in a Svelte component
import { getContentPipelineState } from "$lib/components/pages/home/content-main/content-dag-editor/content-pipelines/content-pipeline.svelte";

const pipelineState = getContentPipelineState();

// Create a new pipeline (optimistic)
const newPipeline = await pipelineState.createPipeline(
  "My Pipeline",
  "Description",
);

// Add a node (optimistic)
await pipelineState.addNode(newPipeline.id, {
  id: crypto.randomUUID(),
  name: "My Node",
  nodeType: "operator",
  config: { operator_id: "some_operator" },
  metadata: {},
  inputCurveIds: [],
  inputToOutputCurveMapping: {},
  dependencies: [],
});

// Update node position (debounced sync)
pipelineState.updateNodePosition(newPipeline.id, nodeId, 200, 300);
```

---

## Next Steps

1. ✅ **Database Schema** - SQL migration created
2. ✅ **Rust Backend** - Tauri commands implemented
3. ✅ **Frontend State** - ContentPipelineState refactored
4. ✅ **Run Migration** - SQL migration executed on database (2024-12-10)
5. ✅ **Automerge-Repo Integration** - Frontend automerge-repo for true CRDT persistence (2024-12-10)
   - [x] Create automerge-repo singleton (`$lib/pipelines/repo/pipeline-repo.ts`)
   - [x] Create PipelineSpec document type (`$lib/pipelines/types/pipeline-doc-type.ts`)
   - [x] Create automerge store wrapper (`$lib/pipelines/repo/pipeline-store.ts`)
   - [x] Update ContentPipelineState to use automerge-repo stores
   - [x] Add automerge URL storage SQL migration (`003-automerge-url-registry.sql`)
   - [x] Add Tauri commands for URL registry (`get_project_pipeline_urls`, `register_pipeline_url`, `delete_pipeline_url`)
6. ✅ **Run URL Registry Migration** - SQL migration executed on database (2024-12-10)
7. ✅ **UI Components** - Updated components to use new Automerge state (2024-12-10)
   - [x] Updated `content-pipelines.svelte` - Now uses `pipelineState.allPipelinesWithNodes` (reactive Automerge state)
   - [x] Updated `content-pipelines-overview.svelte` - Now uses `pipelineState.allPipelinesWithNodes`
   - [x] Updated `content-pipeline-item.svelte` - Now uses `PipelineWithNodes` type and `pipelineState.setSelectedPipelineId()`
   - [x] Removed Supabase Realtime subscriptions from pipeline list components
   - [x] Removed direct `listPipelinesByProject()` calls (now derived from Automerge state)
8. ⏳ **Remove Legacy Code** - Deprecate remaining Supabase Realtime subscriptions
   - [ ] Remove `RealtimeNodesService`
   - [ ] Deprecate direct Supabase calls in node-service.ts
   - [ ] Remove unused imports from updated components
9. ⏳ **Testing** - Test offline scenarios and collaboration
   - [ ] Test offline pipeline creation
   - [ ] Test tab-to-tab sync via BroadcastChannel
   - [ ] Test data persistence after browser close/reopen
10. ⏳ **Medium-term: WebSocket Sync** - Add cross-device collaboration

- [ ] Consider adding `@automerge/automerge-repo-network-websocket`
- [ ] Implement sync server or use `automerge-repo-sync-server`
- [ ] Update PostgreSQL URL registry for cross-device discovery
