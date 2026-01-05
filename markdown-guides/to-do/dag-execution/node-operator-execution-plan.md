# Node Operator Execution Plan

This document outlines the plan for executing operator nodes in MudRock, including loading input curve data, executing UDFs, and writing output curves back to storage.

## Overview

When a user clicks the "Play" button in `content-node-editor.svelte`, the system should:

1. **Load node configuration** from database (`nodes` table)
2. **Load input curve data** from Parquet files (via OpenDAL storage)
3. **Execute operator** (UDF) with input data
4. **Write output curves** (new or overwrite existing) back to Parquet storage
5. **Update database** with new curve records (if creating new curves)

## Architecture

### Current State

**Existing Crates**:

- ✅ `crates/dags/executor` - Basic DAG executor (needs enhancement)
- ✅ `crates/dags/core` - DAG types and structures
- ✅ `crates/dags/storage` - Pipeline storage operations
- ✅ `crates/query-engine/udf-core` - UDF registry and implementations
- ✅ `crates/storage/opendal-storage-adapter` - Unified storage abstraction
- ✅ `crates/storage/project-data-layout` - Path construction for project data

**Existing Components**:

- ✅ `content-node-editor.svelte` - Node editor UI
- ✅ `content-pipeline.svelte.ts` - Pipeline state management
- ✅ `curve-options-provider.ts` - Curve option fetching

### Proposed Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    Frontend (SvelteKit)                     │
│                                                              │
│  content-node-editor.svelte                                  │
│    └─ "Play" button → executeNodeOperator(nodeId)           │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼ (Tauri Command)
┌─────────────────────────────────────────────────────────────┐
│                    Tauri Backend                            │
│                                                              │
│  execute_node_operator(node_id: Uuid)                      │
│    └─ NodeOperatorExecutor::execute(node_id)               │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│         New Crate: node-operator-executor                    │
│                                                              │
│  NodeOperatorExecutor                                        │
│    ├─ load_node_config(node_id)                             │
│    ├─ load_input_curves(input_curve_ids)                   │
│    ├─ execute_operator(operator_id, input_data, params)     │
│    └─ write_output_curves(output_data, mapping)            │
└─────────────────────────────────────────────────────────────┘
                            │
        ┌───────────────────┴───────────────────┐
        │                                       │
        ▼                                       ▼
┌──────────────────────┐          ┌──────────────────────────┐
│ dag-storage          │          │ opendal-storage-adapter   │
│ - Load node from DB  │          │ - Read Parquet files      │
│ - Update curves      │          │ - Write Parquet files     │
└──────────────────────┘          └──────────────────────────┘
        │                                       │
        └───────────────────┬───────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│              PostgreSQL + OpenDAL Storage                    │
│  - nodes table (config, input_curve_ids, mapping)           │
│  - curves table (curve records)                             │
│  - Parquet files (actual curve data)                       │
└─────────────────────────────────────────────────────────────┘
```

## Implementation Plan

### Phase 1: Create Node Operator Executor Crate

**Crate**: `crates/dags/node-operator-executor`

**Structure**:

```
crates/dags/node-operator-executor/
├── Cargo.toml
├── src/
│   ├── lib.rs              # Public API exports
│   ├── executor.rs         # NodeOperatorExecutor struct
│   ├── curve_loader.rs     # Load curves from Parquet
│   ├── curve_writer.rs     # Write curves to Parquet
│   └── error.rs            # Error types
└── README.md
```

**Dependencies** (`Cargo.toml`):

```toml
[dependencies]
# Core dependencies
dag-core = { path = "../core" }
dag-storage = { path = "../storage" }
udf-core = { path = "../../query-engine/udf-core" }
opendal-storage-adapter = { path = "../../storage/opendal-storage-adapter" }
project-data-layout = { path = "../../storage/project-data-layout" }

# Data processing
datafusion = "40"
arrow = "54"
parquet = "54"

# Async runtime
tokio = { version = "1.0", features = ["full"] }

# Error handling
thiserror = "1.0"

# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# Database
tokio-postgres = "0.7"
uuid = { version = "1.0", features = ["v4", "serde"] }

# Observability
tracing = "0.1"
```

### Phase 2: Implement NodeOperatorExecutor

**File**: `crates/dags/node-operator-executor/src/executor.rs`

```rust
use crate::error::{NodeOperatorError, Result};
use crate::curve_loader::CurveLoader;
use crate::curve_writer::CurveWriter;
use dag_storage::PipelineStorage;
use udf_core::UdfRegistry;
use opendal_storage_adapter::EnhancedOpenDALStorageAdapter;
use project_data_layout::ProjectDataLayoutManager;
use datafusion::prelude::*;
use uuid::Uuid;

/// Executes a single operator node with input/output curve handling
pub struct NodeOperatorExecutor {
    storage: Arc<dyn PipelineStorage>,
    udf_registry: Arc<UdfRegistry>,
    storage_adapter: Arc<EnhancedOpenDALStorageAdapter>,
    layout_manager: Arc<ProjectDataLayoutManager>,
    curve_loader: CurveLoader,
    curve_writer: CurveWriter,
}

impl NodeOperatorExecutor {
    /// Create a new executor
    pub fn new(
        storage: Arc<dyn PipelineStorage>,
        udf_registry: Arc<UdfRegistry>,
        storage_adapter: Arc<EnhancedOpenDALStorageAdapter>,
        layout_manager: Arc<ProjectDataLayoutManager>,
    ) -> Self {
        let curve_loader = CurveLoader::new(
            Arc::clone(&storage_adapter),
            Arc::clone(&layout_manager),
        );
        let curve_writer = CurveWriter::new(
            Arc::clone(&storage_adapter),
            Arc::clone(&layout_manager),
        );

        Self {
            storage,
            udf_registry,
            storage_adapter,
            layout_manager,
            curve_loader,
            curve_writer,
        }
    }

    /// Execute an operator node
    ///
    /// Steps:
    /// 1. Load node configuration from database
    /// 2. Load input curve data from Parquet files
    /// 3. Execute operator (UDF) with input data
    /// 4. Write output curves (new or overwrite) to Parquet
    /// 5. Update database with new curve records (if creating new)
    pub async fn execute(&self, node_id: Uuid) -> Result<NodeExecutionResult> {
        // 1. Load node configuration
        let node = self.load_node_config(node_id).await?;

        // 2. Extract operator ID and parameters
        let operator_id = self.extract_operator_id(&node)?;
        let parameters = self.extract_parameters(&node)?;

        // 3. Load input curves
        let input_curves = self.load_input_curves(&node).await?;

        // 4. Create DataFusion context and register UDFs
        let ctx = SessionContext::new();
        self.udf_registry.register_all_udfs(&ctx)?;

        // 5. Build input DataFrame from curves
        let input_df = self.build_input_dataframe(&ctx, input_curves).await?;

        // 6. Execute operator
        let output_df = self.execute_operator(&ctx, operator_id, input_df, parameters).await?;

        // 7. Write output curves
        let output_curves = self.write_output_curves(&node, output_df).await?;

        Ok(NodeExecutionResult {
            node_id,
            input_curve_count: input_curves.len(),
            output_curve_count: output_curves.len(),
            output_curves,
        })
    }

    /// Load node configuration from database
    async fn load_node_config(&self, node_id: Uuid) -> Result<NodeConfig> {
        // Query nodes table for node_id
        // Extract: node_config (JSONB), input_curve_ids, input_to_output_curve_mapping
        todo!()
    }

    /// Load input curve data from Parquet files
    async fn load_input_curves(&self, node: &NodeConfig) -> Result<Vec<LoadedCurve>> {
        // For each input_curve_id:
        //   1. Query curves table for curve metadata (parquet_file_path, parquet_column_name)
        //   2. Load Parquet file via OpenDAL
        //   3. Extract column data
        //   4. Return LoadedCurve with well_id, depth, values
        self.curve_loader.load_curves(node.input_curve_ids.clone()).await
    }

    /// Build input DataFrame from loaded curves
    async fn build_input_dataframe(
        &self,
        ctx: &SessionContext,
        curves: Vec<LoadedCurve>,
    ) -> Result<DataFrame> {
        // Combine curves into single DataFrame
        // Columns: well_id, depth, gr, gr_clean, gr_shale, ... (based on operator requirements)
        // Handle multiple wells (group by well_id)
        todo!()
    }

    /// Execute operator (UDF) with input DataFrame
    async fn execute_operator(
        &self,
        ctx: &SessionContext,
        operator_id: &str,
        input_df: DataFrame,
        parameters: HashMap<String, serde_json::Value>,
    ) -> Result<DataFrame> {
        // Build SQL query: SELECT operator_id(col1, col2, ...) FROM input_df
        // Execute via DataFusion
        // Return output DataFrame with output curve columns
        todo!()
    }

    /// Write output curves to Parquet files
    async fn write_output_curves(
        &self,
        node: &NodeConfig,
        output_df: DataFrame,
    ) -> Result<Vec<Uuid>> {
        // For each input curve in input_to_output_curve_mapping:
        //   - If mapping is null: Create new curve (with suffix)
        //   - If mapping is curve_id: Overwrite existing curve
        // Write Parquet files via OpenDAL
        // Update curves table with new records
        self.curve_writer.write_curves(node, output_df).await
    }
}
```

### Phase 3: Implement Curve Loader

**File**: `crates/dags/node-operator-executor/src/curve_loader.rs`

```rust
use opendal_storage_adapter::EnhancedOpenDALStorageAdapter;
use project_data_layout::ProjectDataLayoutManager;
use datafusion::prelude::*;
use arrow::record_batch::RecordBatch;
use uuid::Uuid;

pub struct CurveLoader {
    storage_adapter: Arc<EnhancedOpenDALStorageAdapter>,
    layout_manager: Arc<ProjectDataLayoutManager>,
}

impl CurveLoader {
    pub fn new(
        storage_adapter: Arc<EnhancedOpenDALStorageAdapter>,
        layout_manager: Arc<ProjectDataLayoutManager>,
    ) -> Self {
        Self {
            storage_adapter,
            layout_manager,
        }
    }

    /// Load curves from Parquet files
    pub async fn load_curves(&self, curve_ids: Vec<Uuid>) -> Result<Vec<LoadedCurve>> {
        // For each curve_id:
        //   1. Query curves table: SELECT parquet_file_path, parquet_column_name, well_id
        //   2. Load Parquet file via storage_adapter.read(parquet_file_path)
        //   3. Extract column data (parquet_column_name)
        //   4. Return LoadedCurve { well_id, depth_column, value_column }
        todo!()
    }
}

pub struct LoadedCurve {
    pub curve_id: Uuid,
    pub well_id: i32,
    pub depth: Vec<f64>,  // Depth column (DEPT)
    pub values: Vec<f64>, // Curve values
}
```

### Phase 4: Implement Curve Writer

**File**: `crates/dags/node-operator-executor/src/curve_writer.rs`

```rust
use opendal_storage_adapter::EnhancedOpenDALStorageAdapter;
use project_data_layout::ProjectDataLayoutManager;
use datafusion::prelude::*;
use uuid::Uuid;

pub struct CurveWriter {
    storage_adapter: Arc<EnhancedOpenDALStorageAdapter>,
    layout_manager: Arc<ProjectDataLayoutManager>,
}

impl CurveWriter {
    pub fn new(
        storage_adapter: Arc<EnhancedOpenDALStorageAdapter>,
        layout_manager: Arc<ProjectDataLayoutManager>,
    ) -> Self {
        Self {
            storage_adapter,
            layout_manager,
        }
    }

    /// Write output curves to Parquet files
    pub async fn write_curves(
        &self,
        node: &NodeConfig,
        output_df: DataFrame,
    ) -> Result<Vec<Uuid>> {
        // For each input curve in input_to_output_curve_mapping:
        //   - Extract output column from DataFrame (grouped by well_id)
        //   - If mapping is null: Create new curve
        //     - Generate curve_id, curve_name (with suffix)
        //     - Write to new Parquet file (or append to existing)
        //     - Insert into curves table
        //   - If mapping is curve_id: Overwrite existing
        //     - Update Parquet file column
        //     - Update curves table
        todo!()
    }
}
```

### Phase 5: Add Tauri Command

**File**: `src-tauri/src/commands/node_execution.rs`

```rust
use crate::state::AppState;
use node_operator_executor::NodeOperatorExecutor;
use uuid::Uuid;

#[tauri::command]
pub async fn execute_node_operator(
    node_id: Uuid,
    state: tauri::State<'_, AppState>,
) -> Result<NodeExecutionResult, String> {
    let executor = NodeOperatorExecutor::new(
        state.pipeline_storage.clone(),
        state.udf_registry.clone(),
        state.storage_adapter.clone(),
        state.layout_manager.clone(),
    );

    executor.execute(node_id).await
        .map_err(|e| e.to_string())
}
```

### Phase 6: Add Frontend Integration

**File**: `src/lib/components/pages/home/content-main/content-node-editor/content-node-editor.svelte`

```svelte
<script lang="ts">
  import { Play } from '@lucide/svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { getContentPipelineState } from '../content-dag-editor/content-pipelines/content-pipeline.svelte.ts';

  const pipelineState = getContentPipelineState();
  let selectedNodeId = $derived(pipelineState.currentSelectedNodeForSelectedPipeline);
  let isExecuting = $state(false);
  let executionError = $state<string | null>(null);

  async function handleExecute() {
    if (!selectedNodeId) return;

    try {
      isExecuting = true;
      executionError = null;

      const result = await invoke('execute_node_operator', {
        nodeId: selectedNodeId,
      });

      console.log('Node execution result:', result);
      // TODO: Show success message or update UI
    } catch (error) {
      console.error('Node execution failed:', error);
      executionError = error instanceof Error ? error.message : 'Execution failed';
    } finally {
      isExecuting = false;
    }
  }
</script>

<div class="h-full w-full flex flex-col">
  <!-- Play button above tabs -->
  <div class="flex items-center justify-between {STYLE_CONSTANTS.SPACING.PADDING.SMALL} border-b">
    <Button
      onclick={handleExecute}
      disabled={!selectedNodeId || isExecuting}
      class="flex items-center {STYLE_CONSTANTS.SPACING.GAP.SMALL}"
    >
      <Play class="h-4 w-4" />
      <span>{isExecuting ? 'Executing...' : 'Execute Node'}</span>
    </Button>
    {#if executionError}
      <p class="text-red-500 {STYLE_CONSTANTS.FONT_SIZE.DEFAULT}">{executionError}</p>
    {/if}
  </div>

  <!-- Existing tabs... -->
</div>
```

## Execution Flow

### Step-by-Step Execution

1. **User clicks "Play" button**
   - Frontend calls `execute_node_operator(nodeId)` Tauri command

2. **Load node configuration**
   - Query `nodes` table for `node_id`
   - Extract: `node_config` (JSONB), `input_curve_ids`, `input_to_output_curve_mapping`
   - Extract `operator_id` from `node_config`

3. **Load input curves**
   - For each `input_curve_id`:
     - Query `curves` table: `SELECT parquet_file_path, parquet_column_name, well_id`
     - Load Parquet file via OpenDAL: `storage_adapter.read(parquet_file_path)`
     - Extract column data: `parquet_column_name`
     - Return `LoadedCurve { well_id, depth, values }`

4. **Build input DataFrame**
   - Combine curves into single DataFrame
   - Columns: `well_id`, `depth`, `gr`, `gr_clean`, `gr_shale`, ... (based on operator requirements)
   - Handle multiple wells (group by `well_id`)

5. **Execute operator**
   - Create DataFusion `SessionContext`
   - Register UDFs: `udf_registry.register_all_udfs(&ctx)`
   - Build SQL query: `SELECT operator_id(col1, col2, ...) FROM input_df`
   - Execute via DataFusion: `ctx.sql(query).await`
   - Return output DataFrame with output curve columns

6. **Write output curves**
   - For each input curve in `input_to_output_curve_mapping`:
     - Extract output column from DataFrame (grouped by `well_id`)
     - **If mapping is null** (create new):
       - Generate `curve_id`, `curve_name` (with suffix from node config)
       - Write to Parquet file (append to existing or create new)
       - Insert into `curves` table
     - **If mapping is curve_id** (overwrite):
       - Update Parquet file column
       - Update `curves` table metadata

7. **Return result**
   - Return `NodeExecutionResult` with:
     - `node_id`
     - `input_curve_count`
     - `output_curve_count`
     - `output_curves` (list of created/updated curve IDs)

## Error Handling

**Error Types** (`error.rs`):

```rust
#[derive(Debug, Error)]
pub enum NodeOperatorError {
    #[error("Node not found: {0}")]
    NodeNotFound(Uuid),

    #[error("Operator not found: {0}")]
    OperatorNotFound(String),

    #[error("Failed to load curve {0}: {1}")]
    CurveLoadFailed(Uuid, String),

    #[error("Failed to execute operator: {0}")]
    OperatorExecutionFailed(String),

    #[error("Failed to write output curve: {0}")]
    CurveWriteFailed(String),

    #[error("Database error: {0}")]
    DatabaseError(#[from] tokio_postgres::Error),

    #[error("Storage error: {0}")]
    StorageError(#[from] opendal::Error),

    #[error("DataFusion error: {0}")]
    DataFusionError(#[from] datafusion::error::DataFusionError),
}
```

## Performance Considerations

1. **Parallel Curve Loading**: Load multiple Parquet files in parallel
2. **Caching**: Cache loaded curves in memory (Moka cache)
3. **Batch Writing**: Write multiple output curves in batch operations
4. **Streaming**: Use DataFusion streaming for large datasets

## Testing Strategy

1. **Unit Tests**: Test individual functions (curve loading, DataFrame building)
2. **Integration Tests**: Test full execution flow with mock data
3. **End-to-End Tests**: Test with real Parquet files and database

## Migration Path

1. **Week 1**: Create crate structure and basic executor
2. **Week 2**: Implement curve loader and writer
3. **Week 3**: Integrate with Tauri and frontend
4. **Week 4**: Testing and optimization

## Future Enhancements

1. **Progress Reporting**: Stream execution progress to frontend
2. **Cancellation**: Allow users to cancel long-running executions
3. **Validation**: Pre-execution validation (check input curves exist, operator available)
4. **Caching**: Cache execution results based on input curve content hashes
5. **Parallel Execution**: Execute multiple nodes in parallel (when dependencies allow)
