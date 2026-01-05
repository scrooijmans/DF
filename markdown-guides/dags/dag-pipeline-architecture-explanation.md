# DAG Architecture: Storage, Type Safety, and DataFusion Integration

This document explains how DAGs are stored in your self-hosted Supabase database, how type safety ensures compatibility between nodes, and how DataFusion DataFrames are used for execution.

## 📦 Storage Architecture

### Database Schema (Executed: `02-pipeline-schema.sql`)

DAGs are stored in your **self-hosted Supabase PostgreSQL** database (via Hetzner VPS) using a **dual-storage approach**:

#### 1. **Main DAG Definition (JSONB)**

```
pipelines table
├── id (UUID) - Primary key
├── project_id (UUID) - Foreign key to projects
├── name, description, version
├── dag_definition (JSONB) ← Complete DagDefinition serialized as JSON
├── created_by, created_at, updated_at
└── tags, is_active, is_public
```

**Purpose**: Stores the **complete DAG structure** as JSONB for:

- Quick loading of entire DAGs
- Version control (via `version` field)
- Backup and restore

**Example JSONB Structure**:

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "name": "GR to VShale Pipeline",
  "project_id": "project-uuid",
  "version": "1.0.0",
  "nodes": [
    {
      "id": "node-1-uuid",
      "name": "Ingest Well Logs",
      "node_type": {
        "type": "ingestion",
        "source_type": { "source": "well_logs", ... },
        "output_schema": { "schema_id": "well_logs_standard", ... }
      },
      "dependencies": []
    },
    {
      "id": "node-2-uuid",
      "name": "GR → VShale",
      "node_type": {
        "type": "operator",
        "operator_id": "shale_volume_larionov",
        "input_schema": { ... },
        "output_schema": { ... }
      },
      "dependencies": ["node-1-uuid"]
    }
  ]
}
```

#### 2. **Normalized Node Storage** (For Querying)

```
pipeline_nodes table
├── pipeline_id (UUID) - Foreign key to pipelines
├── node_id (UUID) - Node UUID within DAG
├── node_name, node_type ('ingestion' | 'operator' | 'output')
├── node_config (JSONB) - NodeType enum as JSON
├── input_schema (JSONB) - SchemaDefinition
├── output_schema (JSONB) - SchemaDefinition
├── dependencies (JSONB) - Array of node UUIDs
├── position_x, position_y (for UI)
└── metadata (JSONB) - Cache settings, timeouts, tags
```

**Purpose**: Enables **efficient querying** without parsing entire JSON:

- Find all operators in a pipeline: `SELECT * FROM pipeline_nodes WHERE pipeline_id = $1 AND node_type = 'operator'`
- Query by schema compatibility: `SELECT * FROM pipeline_nodes WHERE output_schema->>'schema_id' = 'well_logs_standard'`
- Visual editor position storage: `position_x, position_y`

#### 3. **Explicit Edge Storage** (Graph Queries)

```
pipeline_edges table
├── pipeline_id (UUID)
├── source_node_id (UUID)
├── target_node_id (UUID)
└── edge_type ('data_flow' | 'conditional' | 'error')
```

**Purpose**: Enables **graph-based queries**:

- Find all nodes that depend on a node: `SELECT target_node_id FROM pipeline_edges WHERE source_node_id = $1`
- Detect cycles: Graph algorithms on edges
- Visual graph rendering: Direct edge-to-node mapping

#### 4. **Execution History**

```
pipeline_executions table
├── pipeline_id (UUID)
├── status ('pending' | 'running' | 'completed' | 'failed')
├── execution_time_ms, error_message
├── metrics (JSONB) - {rows_in, rows_out, bytes_scanned, ...}
├── content_hash (TEXT) - For cache lookup
└── execution_log (JSONB) - Per-node execution details
```

**Purpose**: Track execution history, metrics, and enable caching via `content_hash`.

### Storage Implementation (`dag-storage` Crate)

**File**: `crates/dags/storage/src/pipeline_storage.rs`

```rust
// Save pipeline: Dual-write strategy
pub async fn save_pipeline(
    &mut self,
    dag: &DagDefinition,
    user_id: Option<Uuid>,
) -> Result<Uuid, PipelineStorageError> {
    let mut tx = self.client.transaction().await?;

    // 1. Insert/Update main pipeline record (JSONB)
    tx.execute(
        "INSERT INTO pipelines (id, ..., dag_definition) VALUES ($1, ..., $2)",
        &[..., &Json(serde_json::to_value(dag)?)]
    ).await?;

    // 2. Delete existing normalized nodes/edges (for updates)
    tx.execute("DELETE FROM pipeline_nodes WHERE pipeline_id = $1", ...).await?;

    // 3. Insert normalized nodes
    for node in &dag.nodes {
        let (input_schema, output_schema) = Self::extract_schemas(&node.node_type);
        tx.execute(
            "INSERT INTO pipeline_nodes (..., input_schema, output_schema) VALUES (...)",
            &[..., &Json(input_schema), &Json(output_schema)]
        ).await?;

        // 4. Insert edges
        for dep_id in &node.dependencies {
            tx.execute(
                "INSERT INTO pipeline_edges (source_node_id, target_node_id) VALUES ($1, $2)",
                &[&dep_id, &node.id]
            ).await?;
        }
    }

    tx.commit().await?;
    Ok(dag.id)
}
```

**Benefits of Dual Storage**:

- ✅ **Fast loading**: Load entire DAG from JSONB in one query
- ✅ **Efficient querying**: Query normalized tables for specific nodes/schemas
- ✅ **Graph operations**: Use edges table for dependency analysis
- ✅ **Schema validation**: GIN indexes on JSONB schemas for compatibility checks

---

## 🔒 Type Safety & Schema Compatibility

Type safety is enforced at **multiple layers** to prevent runtime errors:

### Layer 1: Compile-Time Type Safety (Rust)

**File**: `crates/dags/core/src/node_types.rs`

```rust
pub enum NodeType {
    Ingestion {
        source_type: IngestionSourceType,
        filters: IngestionFilters,
        output_schema: SchemaDefinition,  // ← Output schema defined at compile time
    },
    Operator {
        operator_id: String,
        input_schema: SchemaDefinition,   // ← Required input schema
        output_schema: SchemaDefinition,   // ← Expected output schema
        parameters: HashMap<String, serde_json::Value>,
    },
    Output {
        input_schema: SchemaDefinition,    // ← Required input schema
        output_type: OutputType,
    },
}
```

**How it works**:

- **Ingestion nodes** define `output_schema` - what columns they produce
- **Operator nodes** define both `input_schema` and `output_schema` - what they need and produce
- **Output nodes** define `input_schema` - what they expect to receive

### Layer 2: Schema Definition (`SchemaDefinition`)

**File**: `crates/dags/core/src/schema.rs`

```rust
pub struct SchemaDefinition {
    pub schema_id: String,                    // e.g., "well_logs_standard"
    pub arrow_schema_json: String,             // Arrow schema as JSON
    pub required_columns: Vec<String>,         // Must be present
    pub optional_columns: Vec<String>,         // May be present
}

impl SchemaDefinition {
    /// Validate compatibility between two schemas
    pub fn validate_compatible(
        &self,        // Source schema (output of upstream node)
        other: &SchemaDefinition,  // Target schema (input of downstream node)
    ) -> Result<(), SchemaValidationError> {
        // Rule 1: All required columns in 'other' must exist in 'self'
        let self_all_columns = self.all_columns();
        for required_col in &other.required_columns {
            if !self_all_columns.contains(required_col) {
                return Err(SchemaValidationError::MissingRequiredColumn(...));
            }
        }

        // Rule 2: Column types should match (when arrow_schema_json is parsed)
        // TODO: Full type validation

        Ok(())
    }

    /// Validate actual Arrow schema matches definition (at runtime)
    pub fn validate_schema(&self, actual_schema: &Schema) -> Result<(), ...> {
        // Check required columns exist
        for required_col in &self.required_columns {
            actual_schema.field_with_name(required_col)
                .map_err(|_| SchemaValidationError::MissingRequiredColumn(...))?;
        }
        Ok(())
    }
}
```

### Layer 3: Frontend Type Checking (Before Save)

**Location**: `src/lib/components/pipelines/` (planned)

```typescript
// Frontend validates node connections before sending to backend
async function validateNodeConnection(
  sourceNode: DagNode,
  targetNode: DagNode,
): Promise<boolean> {
  // Get schemas from node types
  const sourceOutputSchema = sourceNode.node_type.output_schema;
  const targetInputSchema = targetNode.node_type.input_schema;

  // Check compatibility
  const compatible = sourceOutputSchema.validate_compatible(targetInputSchema);

  // Visual feedback: Green edge = compatible, Red edge = incompatible
  return compatible;
}
```

### Layer 4: DAG Compiler Validation (Before Execution)

**Location**: `crates/dags/executor/src/compiler.rs` (planned)

```rust
pub struct DagCompiler;

impl DagCompiler {
    pub fn validate(&self, dag: &DagDefinition) -> Result<(), CompilationError> {
        // 1. Check for cycles (using petgraph)
        self.check_acyclic(dag)?;

        // 2. Validate all node connections are type-compatible
        for node in &dag.nodes {
            for dep_id in &node.dependencies {
                let dep_node = dag.get_node(dep_id)?;

                // Get schemas
                let dep_output = dep_node.node_type.output_schema().ok_or(...)?;
                let node_input = node.node_type.input_schema().ok_or(...)?;

                // Validate compatibility
                dep_output.validate_compatible(node_input)?;
            }
        }

        // 3. Ensure all operators exist in registry
        for node in &dag.nodes {
            if let NodeType::Operator { operator_id, operator_version, .. } = &node.node_type {
                self.operator_registry.get_operator(operator_id, operator_version)?;
            }
        }

        Ok(())
    }
}
```

### Layer 5: Runtime Schema Validation (During Execution)

**Location**: `crates/dags/executor/src/executor.rs` (planned)

```rust
async fn execute_node(
    &self,
    node: &DagNode,
    input_data: Option<RecordBatch>,  // ← Arrow RecordBatch from upstream
) -> Result<RecordBatch, DagExecutionError> {
    match &node.node_type {
        NodeType::Operator { input_schema, output_schema, .. } => {
            // Validate input schema matches operator's expected input
            let input = input_data.ok_or(DagExecutionError::MissingInput(...))?;
            input_schema.validate_schema(input.schema())?;  // ← Runtime check

            // Execute operator
            let result = self.execute_operator(...).await?;

            // Validate output schema matches expected
            output_schema.validate_schema(result.schema())?;  // ← Runtime check

            Ok(result)
        }
        // ... other node types
    }
}
```

### Example: Type Safety Flow

**Scenario**: Connect "Ingest Well Logs" → "GR → VShale Operator"

```
1. Ingestion Node Output Schema:
   {
     schema_id: "well_logs_standard",
     required_columns: ["well_id", "depth"],
     optional_columns: ["gr", "rhob"]
   }

2. Operator Input Schema:
   {
     schema_id: "well_logs_standard",
     required_columns: ["well_id", "depth", "gr"],  // ← Needs "gr"
     optional_columns: []
   }

3. Compatibility Check (validate_compatible):
   ✅ "well_id" exists in output (required)
   ✅ "depth" exists in output (required)
   ✅ "gr" exists in output (optional) → OK (optional can satisfy required)
   → Connection is VALID

4. If Operator Required "sonic" instead:
   ❌ "sonic" NOT in output → Connection is INVALID
   → Frontend shows red edge, backend rejects save
```

---

## ⚙️ DataFusion Integration

DataFusion is used as the **execution engine** for DAG nodes. Data flows as **Arrow RecordBatches** between nodes, and operations use **DataFusion DataFrames**.

### Architecture

```
┌─────────────────────────────────────────────────────────┐
│              DAG Executor (petgraph)                    │
│                                                           │
│  ┌──────────────────────────────────────────────────┐  │
│  │  1. Topological Sort (petgraph)                   │  │
│  │     → Determine node execution order               │  │
│  └──────────────────────────────────────────────────┘  │
│                        │                                  │
│                        ▼                                  │
│  ┌──────────────────────────────────────────────────┐  │
│  │  2. Node Execution (DataFusion SessionContext)     │  │
│  │     - Ingestion: Read Parquet → DataFrame          │  │
│  │     - Operator: Transform DataFrame → DataFrame   │  │
│  │     - Output: DataFrame → Arrow RecordBatch         │  │
│  └──────────────────────────────────────────────────┘  │
│                        │                                  │
│                        ▼                                  │
│  ┌──────────────────────────────────────────────────┐  │
│  │  3. RecordBatch Streaming                         │  │
│  │     - Node outputs → RecordBatch                  │  │
│  │     - RecordBatch → Next node input               │  │
│  │     - Caching: RecordBatch → Moka cache           │  │
│  └──────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────┘
```

### DataFusion Usage

#### 1. **Session Context Setup**

```rust
// crates/dags/executor/src/executor.rs (planned)
use datafusion::prelude::*;

pub struct DagExecutor {
    session_ctx: SessionContext,
    storage_adapter: Arc<EnhancedOpenDALStorageAdapter>,
}

impl DagExecutor {
    async fn create_session_context(&self, project_id: &str) -> SessionContext {
        let ctx = SessionContext::new();

        // Register OpenDAL object store for S3/MinIO access
        let object_store = self.storage_adapter.create_object_store().await?;
        ctx.runtime_env().register_object_store(
            "s3",
            "project-bucket",
            Arc::new(object_store),
        );

        // Register project Parquet files as tables
        // e.g., CREATE EXTERNAL TABLE well_123 STORED AS PARQUET LOCATION 's3://...'
        self.register_project_tables(&ctx, project_id).await?;

        ctx
    }
}
```

#### 2. **Ingestion Node Execution** (Parquet → DataFrame)

```rust
async fn execute_ingestion(
    &self,
    source_type: &IngestionSourceType,
) -> Result<RecordBatch, DagExecutionError> {
    match source_type {
        IngestionSourceType::WellLogs { project_id, well_ids, curve_names, .. } => {
            let ctx = self.session_ctx.clone();

            // Read Parquet files using DataFusion
            // Option 1: Direct Parquet scan
            let df = ctx
                .read_parquet(&format!("s3://project-{}/wells/{}/logs_composite.parquet",
                                     project_id, well_ids[0]))
                .await?;

            // Option 2: Register as table, then query
            // ctx.sql("CREATE EXTERNAL TABLE well_123 STORED AS PARQUET LOCATION 's3://...'").await?;
            // let df = ctx.sql("SELECT well_id, depth, gr, rhob FROM well_123 WHERE depth BETWEEN 1000 AND 2000").await?;

            // Apply filters (pushdown to Parquet)
            let filtered_df = df
                .filter(col("depth").gt_eq(lit(1000.0)))
                .filter(col("depth").lt_eq(lit(2000.0)))
                .select( curve_names.iter().map(|c| col(c)).collect() );

            // Execute and collect as RecordBatch
            let results = filtered_df.collect().await?;
            let batch = results[0].clone();  // First batch

            Ok(batch)
        }
        // ... other source types
    }
}
```

#### 3. **Operator Node Execution** (DataFrame Transformations)

```rust
async fn execute_operator(
    &self,
    operator_id: &str,
    parameters: &HashMap<String, serde_json::Value>,
    input: RecordBatch,  // ← From upstream node
) -> Result<RecordBatch, DagExecutionError> {
    // Convert RecordBatch to DataFrame
    let ctx = self.session_ctx.clone();
    let df = ctx.read_batch(input)?;

    match operator_id {
        "shale_volume_larionov" => {
            // Apply transformation using DataFusion expressions
            // GR → VShale: VSh = (GR - GRmin) / (GRmax - GRmin)
            let gr_min = parameters.get("gr_min").and_then(|v| v.as_f64()).unwrap_or(0.0);
            let gr_max = parameters.get("gr_max").and_then(|v| v.as_f64()).unwrap_or(200.0);

            let vsh = (col("gr") - lit(gr_min)) / (lit(gr_max) - lit(gr_min));

            let result_df = df.with_column("vsh", vsh)?;

            // Convert back to RecordBatch
            let results = result_df.collect().await?;
            Ok(results[0].clone())
        }

        "despike" => {
            // Despike using window functions
            let window = Window::new(vec![], vec![], Sort::new(vec![col("depth").sort(true, true)]));
            let vsh_mean = col("vsh").over(window.clone());
            let vsh_std = stddev(col("vsh")).over(window);

            let threshold = parameters.get("threshold").and_then(|v| v.as_f64()).unwrap_or(0.1);
            let is_outlier = (col("vsh") - vsh_mean).abs().gt(vsh_std * lit(threshold));

            let result_df = df.filter(not(is_outlier))?;
            let results = result_df.collect().await?;
            Ok(results[0].clone())
        }

        // ... other operators
    }
}
```

#### 4. **Node-to-Node Data Flow** (RecordBatch Streaming)

```rust
pub async fn execute(
    &self,
    dag: &DagDefinition,
) -> Result<ExecutionResult, DagExecutionError> {
    // 1. Topological sort (petgraph)
    let execution_order = self.topological_sort(dag)?;

    // 2. Execute nodes in order
    let mut node_results: HashMap<Uuid, RecordBatch> = HashMap::new();

    for node_id in execution_order {
        let node = dag.get_node(&node_id)?;

        // Collect input data from dependencies
        let input_data = if node.dependencies.is_empty() {
            None  // Ingestion node
        } else {
            // Combine multiple dependencies into single RecordBatch
            let mut input_batches = Vec::new();
            for dep_id in &node.dependencies {
                let dep_batch = node_results.get(dep_id)
                    .ok_or(DagExecutionError::MissingInput(...))?;
                input_batches.push(dep_batch.clone());
            }
            Some(self.combine_batches(input_batches)?)
        };

        // Execute node (returns RecordBatch)
        let output = self.execute_node(node, input_data).await?;

        // Store result for downstream nodes
        node_results.insert(node_id, output);
    }

    // 3. Return final result
    let output_node = dag.output_nodes().first().ok_or(...)?;
    let final_result = node_results.get(&output_node.id).ok_or(...)?;

    Ok(ExecutionResult { data: final_result.clone(), ... })
}
```

### DataFusion Benefits

1. **Automatic Optimization**:
   - Predicate pushdown: Filters pushed to Parquet scan
   - Projection pruning: Only selected columns read
   - Join reordering: Optimal join plans

2. **Arrow Native**:
   - Zero-copy: RecordBatches passed between nodes
   - Columnar format: Efficient for analytical workloads
   - Memory efficient: Lazy evaluation until `collect()`

3. **Parquet Integration**:
   - Direct Parquet reading: `ctx.read_parquet("s3://...")`
   - Statistics-based pruning: Uses Parquet row group statistics
   - Column projection: Only reads needed columns

4. **SQL Support** (Optional):
   - Can use SQL for complex queries: `ctx.sql("SELECT ...")`
   - Useful for joining multiple wells or complex aggregations

### Current Implementation Status

**Existing Code**:

- ✅ `crates/dags/executor/src/node.rs` - Basic DataFrame execution
- ✅ `datafusion` dependency in `Cargo.toml`
- ✅ Node execution structure with DataFrame support

**Planned**:

- 🔄 Full integration with `dag-core` node types
- 🔄 OpenDAL storage adapter integration
- 🔄 Operator registry execution
- 🔄 Complete DAG execution flow

---

## 📊 Data Flow Summary

```
Frontend DAG Builder
    ↓ (save_dag JSON)
Backend: PipelineStorage.save_pipeline()
    ↓
PostgreSQL:
    ├── pipelines.dag_definition (JSONB) ← Complete DAG
    ├── pipeline_nodes (normalized) ← For querying
    └── pipeline_edges (graph) ← For dependencies
    ↓ (load_pipeline)
DAG Compiler (validate + optimize)
    ↓ (execute)
DAG Executor:
    ├── Topological sort (petgraph)
    ├── Execute nodes (DataFusion)
    │   ├── Ingestion: Parquet → DataFrame → RecordBatch
    │   ├── Operator: RecordBatch → DataFrame → RecordBatch
    │   └── Output: RecordBatch → Frontend/Storage
    └── Cache intermediate results (Moka)
    ↓
Execution Results:
    ├── RecordBatch → Frontend (AG-Grid)
    └── Metrics → pipeline_executions table
```

---

## 🎯 Key Takeaways

1. **Storage**: Dual-storage in PostgreSQL (JSONB for complete DAGs, normalized tables for querying)
2. **Type Safety**: 5 layers of validation (compile-time → runtime)
3. **DataFusion**: Execution engine using DataFrames and Arrow RecordBatches
4. **Integration**: OpenDAL for storage, DataFusion for computation, Arrow for transport

This architecture ensures **type-safe DAG execution** while leveraging the performance of DataFusion's optimized query engine and Arrow's columnar format.
