# Node Execution End-to-End Flow

## Overview

This document describes the complete end-to-end process of executing a node operator from the frontend to the backend, including how input curves are loaded, operators are executed, output curves are created, and how realtime updates propagate back to the frontend.

## Execution Types

The system supports two types of executions, both tracked in the `node_executions` table:

1. **Individual Node Execution** (`execution_type = 'node'`): A single node is executed independently
2. **Pipeline Execution** (`execution_type = 'dag'`): A node is executed as part of a full pipeline/DAG execution

Both execution types use the same `node_executions` table with an `execution_type` field to distinguish between them. Pipeline executions can link multiple node executions via the `parent_execution_id` field (references `pipeline_executions.id`).

## Architecture Flow

```
┌─────────────────┐
│   Frontend      │
│   (Svelte)      │ ← User clicks "Execute Node" button
└────────┬────────┘
         │
         │ Tauri Command: execute_node_operator
         ▼
┌─────────────────┐
│   Tauri         │
│   Backend       │ ← pipeline_commands.rs
└────────┬────────┘
         │
         │ Initialize Components
         │ - PostgreSQL Connection
         │ - Storage Adapter (MinIO)
         │ - Project Layout Manager
         │ - UDF Registry
         │ - NodeOperatorExecutor
         ▼
┌─────────────────┐
│   NodeOperator  │
│   Executor      │ ← node-operator-executor crate
└────────┬────────┘
         │
         │ 1. Load Node Config
         │ 2. Load Input Curves
         │ 3. Execute Operator (UDF)
         │ 4. Write Output Curves
         ▼
┌─────────────────┐
│   PostgreSQL    │
│   Database      │ ← INSERT/UPDATE curves table
└────────┬────────┘
         │
         │ Realtime Event (INSERT)
         ▼
┌─────────────────┐
│   Supabase      │
│   Realtime      │ ← WebSocket event
└────────┬────────┘
         │
         │ RealtimeCurvesService
         ▼
┌─────────────────┐
│   Frontend      │
│   (Svelte)      │ ← PostgresCurvesState updated
└─────────────────┘
```

## Step-by-Step Execution Flow

### 1. Frontend: User Initiates Execution

**Location**: `src/lib/components/pages/home/content-main/content-node-editor/content-node-editor.svelte`

**Process**:

- User clicks "Execute Node" button
- Frontend gets current user ID via `supabase.auth.getUser()` for audit trail
- `handleExecuteNode()` function is called
- Validates that a node is selected (`selectedNodeId`)
- Calls Tauri command `execute_node_operator` with `nodeId` and `userId`

```typescript
async function handleExecuteNode() {
  if (!selectedNodeId) {
    console.warn("No node selected, cannot execute");
    return;
  }

  try {
    isExecuting = true;
    executionError = null;

    // Get current user ID for audit trail
    let userId: string | null = null;
    try {
      const {
        data: { user },
      } = await supabase.auth.getUser();
      userId = user?.id || null;
    } catch (error) {
      console.warn(
        "[ContentNodeEditor] Failed to get user ID for audit trail:",
        error,
      );
    }

    const result = await invoke<{
      node_id: string;
      status: string;
      message?: string;
      output_curves?: Array<{ id: string; name: string }>;
      execution_id?: string;
    }>("execute_node_operator", {
      nodeId: selectedNodeId,
      userId: userId,
    });

    if (result.status === "success") {
      console.log(`Node ${selectedNodeId} executed successfully`);
      // Increment suffixes for "create_new" mappings
      // Show success toast notification
      // Realtime updates propagate new curves
    } else {
      executionError = result.message || "Node execution failed";
      // Show error toast notification
    }
  } catch (error) {
    console.error("Error executing node:", error);
    executionError =
      error instanceof Error ? error.message : "Failed to execute node";
  } finally {
    isExecuting = false;
  }
}
```

**Key Points**:

- Execution is asynchronous
- Error handling displays user-friendly messages
- Loading state (`isExecuting`) prevents multiple simultaneous executions

### 2. Tauri Backend: Command Handler

**Location**: `src-tauri/src/tauri_commands/pipeline_commands.rs`

**Process**:

1. **Parse Node ID**: Converts string UUID to `Uuid` type
2. **Load Configuration**: Reads `AppConfig` with database connection details
3. **Connect to PostgreSQL**: Establishes connection using `database_url` from config
4. **Find Project ID**: Queries `pipelines` and `pipeline_node_references` tables to find the project associated with the node
5. **Load Node Info**: Queries `nodes` table to get:
   - `node_type` (TEXT: 'operator', 'ingestion', 'output')
   - `node_config` (JSONB: contains `operator_id`, `operator_version`, `parameters`)
   - `input_curve_ids` (UUID array)
   - `input_to_output_curve_mapping` (JSONB)
6. **Extract Operator Info**: Extracts `operator_id` and `operator_version` from `node_config` JSONB (not from `node_type` TEXT)
7. **Get User Email**: Queries `auth.users` table to get user email for audit trail
8. **Create Execution Record**: Inserts a new record in `node_executions` table with:
   - `execution_type`: 'node' (individual) or 'dag' (if `pipeline_id` is present)
   - `status`: 'running'
   - `operator_id`, `operator_version`
   - `input_curve_ids`, `input_curve_count`
   - `output_curve_mapping` (snapshot)
   - `node_config_snapshot`: Full node configuration at execution time
   - `operator_config_snapshot`: Operator parameters used
   - `executed_by`: User ID from frontend
   - `executed_by_email`: User email from database
   - `started_at`: Current timestamp
9. **Initialize Storage Adapter**: Creates `EnhancedOpenDALStorageAdapter` for MinIO with project-specific bucket (`project-{project_id}`)
10. **Initialize Layout Manager**: Creates `ProjectDataLayoutManager` for managing project data paths
11. **Initialize UDF Registry**: Creates registry with all available UDFs (23 default UDFs)
12. **Create Executor**: Instantiates `NodeOperatorExecutor` with all initialized components
13. **Execute Node**: Calls `executor.execute_node(node_uuid)` to perform the actual execution
14. **Update Execution Record**: Updates `node_executions` record with:
    - **On Success**:
      - `status = 'completed'`
      - `completed_at = now()`
      - `execution_time_ms = duration`
      - `output_curve_ids[] = [created curve IDs]`
      - `output_curve_count = count`
      - `result_summary = { execution metrics }`
      - `metrics`: JSONB with performance metrics
    - **On Failure**:
      - `status = 'failed'`
      - `completed_at = now()`
      - `execution_time_ms = duration`
      - `error_message = error message`
      - `error_details = { error_type, error_message }`
15. **Return Result**: Serializes `NodeExecutionResult` to JSON and returns to frontend

**Key Configuration**:

- Database URL: Constructed from environment variables (`DB_HOST`, `DB_PORT`, `DB_USER`, `POSTGRES_PASSWORD`, `DB_NAME`)
- Storage Backend: MinIO at `http://91.99.166.223:9000`
- Bucket: `project-{project_id}` (project-specific storage)
- Cache: Moka cache with 1000 capacity, 3600s TTL

**Error Handling**:

- PostgreSQL connection errors are caught and returned with masked connection strings
- All errors are logged with detailed context for debugging

### 3. NodeOperatorExecutor: Load Node Configuration

**Location**: `crates/dags/node-operator-executor/src/executor.rs`

**Process**:

1. **Query Database**: Loads node record from `nodes` table
2. **Parse Node Type**: Extracts `operator_id` from `node_type` JSONB field
3. **Load Input Curves**: Reads `input_curve_ids` array from node record
4. **Load Mapping**: Reads `input_to_output_curve_mapping` JSONB field containing:
   ```json
   {
     "input_curve_id": {
       "mode": "create_new" | "overwrite",
       "output_curve_id": "uuid" | null,
       "suffix": "string" | null
     }
   }
   ```
5. **Validate**: Ensures node is an operator and has input curves configured

**NodeConfig Structure**:

```rust
pub struct NodeConfig {
    pub id: Uuid,
    pub name: String,
    pub node_type: NodeType,
    pub input_curve_ids: Vec<Uuid>,
    pub input_to_output_curve_mapping: HashMap<Uuid, OutputCurveMapping>,
    pub metadata: HashMap<String, serde_json::Value>,
}
```

### 4. NodeOperatorExecutor: Load Input Curves

**Location**: `crates/dags/node-operator-executor/src/curve_loader.rs`

**Process**:

1. **Query Database**: For each `input_curve_id`, loads curve metadata from `curves` table:
   - `well_id`
   - `parquet_file_path`
   - `parquet_column_name`
   - `curve_metadata_id`
2. **Download Parquet Files**: Uses `EnhancedOpenDALStorageAdapter` to download Parquet files from MinIO
3. **Read Parquet Data**: Parses Parquet files using DataFusion to extract:
   - Depth column (`DEPT`)
   - Curve values (from `parquet_column_name`)
4. **Create LoadedCurve**: Builds `LoadedCurve` objects with:
   - `curve_id`: UUID of the curve
   - `well_id`: Integer well ID
   - `depth`: Vector of depth values
   - `values`: Vector of curve values
   - `parquet_file_path`: Path to Parquet file in storage
   - `parquet_column_name`: Column name in Parquet file

**LoadedCurve Structure**:

```rust
pub struct LoadedCurve {
    pub curve_id: Uuid,
    pub well_id: i32,
    pub depth: Vec<f64>,
    pub values: Vec<f64>,
    pub parquet_file_path: String,
    pub parquet_column_name: String,
}
```

**Error Handling**:

- Missing curves return `CurveLoadFailed` error
- Storage errors return `StorageError` with details
- Parquet parsing errors return `ParquetError`

### 5. NodeOperatorExecutor: Execute Operator (UDF)

**Location**: `crates/dags/node-operator-executor/src/executor.rs`

**Process**:

1. **Get Operator Definition**: Looks up operator in `UdfRegistry` by `operator_id`
2. **Convert to RecordBatch**: Transforms `LoadedCurve[]` into Arrow `RecordBatch` format
3. **Extract Parameters**: Reads operator parameters from `node.node_type` JSONB
4. **Execute UDF**: Calls the actual UDF function with:
   - Input data (RecordBatch)
   - Operator parameters (from node config)
5. **Return Output DataFrame**: Returns DataFusion `DataFrame` with output curve data

**Current Implementation**:

- Uses placeholder UDF execution (returns modified input data)
- Future: Will call actual UDF functions from `udf-core` crate

**Output Format**:

- DataFusion `DataFrame` with columns:
  - `well_id`: Well ID for grouping
  - Output column(s): Computed curve values

### 6. CurveWriter: Write Output Curves

**Location**: `crates/dags/node-operator-executor/src/curve_writer.rs`

**Process**:

1. **Group by Well**: Groups output DataFrame by `well_id` to process each well separately
2. **Process Each Mapping**: For each entry in `input_to_output_curve_mapping`:

   **Mode: "overwrite"**:
   - Downloads existing Parquet file from storage
   - Updates the specified column (`parquet_column_name`) with new values
   - Uploads updated Parquet file back to storage
   - Returns existing `output_curve_id`

   **Mode: "create_new"**:
   - Constructs new curve name: `{input_curve_name}{suffix}` (e.g., `RHOB_processed`)
   - Checks if Parquet file exists:
     - **If exists**: Appends new column to existing file
     - **If not exists**: Creates new Parquet file with `DEPT` and output column
   - Uploads Parquet file to storage
   - **Inserts new curve record** into `curves` table:
     ```sql
     INSERT INTO curves (
       id, well_id, curve_id, curve_name,
       curve_metadata_id, parquet_file_path, parquet_column_name
     )
     VALUES (
       $1, $2, $3, $4, $5, $6, $7
     )
     ```
   - Returns new `curve_id` UUID

**Curve Name Generation**:

- For "create_new" mode: `{input_curve.parquet_column_name}{suffix}`
- Example: Input curve `RHOB` with suffix `_processed` → `RHOB_processed`
- **Important**: Curve name must be unique per `well_id` + `curve_metadata_id` combination

**Database Insert**:

- Gets `curve_metadata_id` from `operator_registry.output_schema.curve_metadata_type`
- Looks up `curve_metadata` table to find matching `curve_mnemonic`
- Inserts new curve with generated UUID, well_id, curve_name, and metadata reference

### 7. PostgreSQL: Curve Insert Triggers Realtime

**Location**: `sql/schemas/curves.sql`

**Process**:

1. **INSERT Statement**: New curve record inserted into `curves` table
2. **PostgreSQL WAL**: Write-Ahead Log records the INSERT event
3. **Logical Replication**: PostgreSQL sends change to Supabase Realtime service
4. **Realtime Processing**: Realtime service validates RLS policies and formats event
5. **WebSocket Event**: Event sent to connected frontend clients via WebSocket

**Event Format**:

```json
{
  "schema": "public",
  "table": "curves",
  "commit_timestamp": "2025-12-01T13:25:14.792Z",
  "eventType": "INSERT",
  "new": {
    "id": "52cd8eb2-3bf0-4ab8-b174-f7eb797cbb1d",
    "well_id": 45,
    "curve_id": "RHOB_processedOutput",
    "curve_name": "RHOB_processedOutput",
    "curve_metadata_id": "42c3d8cc-75fc-4f60-9e2a-6c790d324506",
    "parquet_file_path": "project-xxx/wells/45/curves/data.parquet",
    "parquet_column_name": "RHOB_processedOutput",
    "created_at": "2025-12-01T13:25:14.790575+00:00",
    "updated_at": "2025-12-01T13:25:14.790575+00:00"
  },
  "old": null
}
```

### 8. Frontend: RealtimeCurvesService Receives Event

**Location**: `src/lib/services/realtime-curves-service.ts`

**Process**:

1. **WebSocket Message**: Supabase client receives INSERT event
2. **Event Handler**: `handleCurvesChange()` processes the event
3. **Convert to Curve Type**: Transforms database record to `Curve` interface
4. **Update Global State**: Adds curve to `PostgresCurvesState.curves` array
5. **Cache Invalidation**: Calls `invalidateTableCache("curves")` to ensure fresh data on next fetch

**Code Flow**:

```typescript
private async handleCurveInsert(newCurve: any) {
  // Convert database record to Curve type
  const curve: Curve = {
    id: newCurve.id,
    well_id: newCurve.well_id,
    curve_id: newCurve.curve_id,
    curve_name: newCurve.curve_name,
    curve_metadata_id: newCurve.curve_metadata_id,
    parquet_file_path: newCurve.parquet_file_path ?? null,
    parquet_column_name: newCurve.parquet_column_name ?? null,
    created_at: newCurve.created_at ?? new Date().toISOString(),
    updated_at: newCurve.updated_at ?? new Date().toISOString(),
  };

  // Add to global state (avoid duplicates)
  const exists = this.curvesState.curves.some((c: Curve) => c.id === curve.id);
  if (!exists) {
    this.curvesState.curves = [...this.curvesState.curves, curve];
  }

  // Invalidate cache
  await invalidateTableCache("curves");
}
```

### 9. Frontend: UI Updates Reactively

**Location**: `src/lib/components/pages/home/content-main/content-node-editor/output-curves/output-curves.svelte`

**Process**:

1. **Reactive State**: `PostgresCurvesState.curves` is a `$state` rune, triggering reactivity
2. **Component Updates**: Components using `getPostgresCurvesState()` automatically re-render
3. **Output Curve Options**: `getOutputCurveOptionsForOperatorByWell()` queries `curves_view` which includes newly created curves
4. **UI Refresh**: Output curve tables automatically show new curves without manual refresh

**Reactivity Chain**:

```
PostgresCurvesState.curves updated
    ↓
Svelte $state rune triggers reactivity
    ↓
$derived values recalculate
    ↓
Components re-render with new data
    ↓
Output curve options include newly created curves
```

## Key Data Structures

### Input to Output Curve Mapping

**Database Format** (JSONB in `nodes.input_to_output_curve_mapping`):

```json
{
  "input_curve_id_1": {
    "mode": "create_new",
    "output_curve_id": null,
    "suffix": "_processed"
  },
  "input_curve_id_2": {
    "mode": "overwrite",
    "output_curve_id": "uuid-of-existing-curve",
    "suffix": null
  }
}
```

**Rust Type**:

```rust
pub struct OutputCurveMapping {
    pub mode: String, // "create_new" | "overwrite"
    pub output_curve_id: Option<Uuid>,
    pub suffix: Option<String>,
}
```

### Node Execution Result

**Returned to Frontend**:

```json
{
  "node_id": "c933f94f-ec86-4da0-8f6e-00fcbb5192d3",
  "status": "success",
  "execution_time_ms": 770,
  "input_curve_count": 1,
  "output_curve_count": 1,
  "output_curves": ["52cd8eb2-3bf0-4ab8-b174-f7eb797cbb1d"]
}
```

**Rust Type**:

```rust
pub struct NodeExecutionResult {
    pub node_id: Uuid,
    pub input_curve_count: usize,
    pub output_curve_count: usize,
    pub output_curves: Vec<Uuid>,
    pub execution_time_ms: u64,
}
```

## Error Handling

### Frontend Errors

- **No Node Selected**: Warning logged, execution prevented
- **Tauri Command Failure**: Error message displayed to user
- **Network Errors**: Connection timeout errors shown

### Backend Errors

- **PostgreSQL Connection**: Detailed error with masked connection string
- **Node Not Found**: `NodeNotFound` error returned
- **Operator Not Found**: `OperatorNotFound` error returned
- **Missing Input Curves**: `MissingInputCurves` error returned
- **Curve Load Failed**: `CurveLoadFailed` error with curve ID and message
- **Storage Errors**: `StorageError` with detailed message
- **Parquet Errors**: `ParquetError` with parsing details
- **Invalid Mapping**: `InvalidCurveMapping` error with description

## Performance Considerations

### Execution Time

- **Typical Execution**: 500-1000ms for single input curve
- **Factors**:
  - Parquet file download time (network latency)
  - Parquet parsing time (file size)
  - UDF execution time (complexity)
  - Parquet file upload time (network latency)
  - Database insert time (minimal)

### Optimization Opportunities

1. **Parallel Curve Loading**: Load multiple input curves concurrently
2. **Batch Parquet Operations**: Group multiple column writes into single Parquet update
3. **Caching**: Cache frequently accessed Parquet files
4. **Streaming**: Stream large Parquet files instead of loading entirely into memory

## Realtime Updates

### Automatic UI Updates

When a node executes successfully and creates new curves:

1. **Backend**: Inserts curve record into `curves` table
2. **PostgreSQL**: WAL event triggers logical replication
3. **Realtime**: WebSocket event sent to frontend
4. **Frontend**: `RealtimeCurvesService` receives INSERT event
5. **State Update**: `PostgresCurvesState.curves` array updated
6. **UI Refresh**: Components automatically re-render with new curves

### Benefits

- ✅ **No Manual Refresh**: New curves appear immediately
- ✅ **Multi-User Sync**: Multiple users see changes in real-time
- ✅ **Consistent State**: Global state stays synchronized with database
- ✅ **Cache Invalidation**: PostgREST cache automatically invalidated

## Validation Rules

### Curve Name Uniqueness

**Rule**: Curve names must be unique per `well_id` + `curve_metadata_id` combination.

**Rationale**:

- Same curve type (e.g., `VSH`) can exist for multiple wells
- Same well can have multiple curve types (e.g., `GR`, `RHOB`, `VSH`)
- But same well cannot have duplicate curve names for the same type

**Validation Points**:

1. **Before Save Mapping**: Check if suffix would create duplicate name
2. **After Execution**: If "create_new" mode creates duplicate, execution succeeds but name conflict exists
3. **UI Feedback**: Show warning/error if duplicate detected

**Example**:

- ✅ Well A + VSH + `VSH_processed` → Allowed
- ✅ Well B + VSH + `VSH_processed` → Allowed (different well)
- ✅ Well A + GR + `GR_processed` → Allowed (different curve type)
- ❌ Well A + VSH + `VSH_processed` (duplicate) → Not allowed

## Audit Trail Architecture

### Design Philosophy

Inspired by GitHub Actions, Airflow, and other DAG systems, our audit trail follows these principles:

1. **Granular Tracking**: Individual node executions are tracked separately from pipeline executions
2. **Complete History**: Every execution is recorded with full context (inputs, outputs, timing, user)
3. **Reproducibility**: Snapshots of node config, operator config, and input data enable exact reproduction
4. **Future-Proof**: Architecture supports both single node execution (current) and full DAG execution (future)
5. **User Attribution**: Every execution is linked to the user who triggered it

### Database Schema: `node_executions` Table

The `node_executions` table provides granular tracking of individual node executions:

**Key Fields:**

- `id`: Unique execution ID
- `node_id`: Reference to the executed node
- `pipeline_id`: Optional reference to parent pipeline (NULL for standalone executions)
- `execution_type`: `'node'` (single) or `'dag'` (part of pipeline)
- `parent_execution_id`: Links to `pipeline_executions.id` if part of DAG
- `operator_id` & `operator_version`: Which operator was executed
- `status`: `'pending'`, `'running'`, `'completed'`, `'failed'`, `'cancelled'`
- `started_at` & `completed_at`: Execution timing
- `execution_time_ms`: Duration in milliseconds
- `executed_by` & `executed_by_email`: User attribution
- `input_curve_ids[]`: Array of input curve UUIDs
- `output_curve_ids[]`: Array of output curve UUIDs created/updated
- `output_curve_mapping`: Full `input_to_output_curve_mapping` snapshot
- `result_summary`: JSONB with execution results
- `error_message` & `error_details`: Error information if failed
- `metrics`: JSONB with performance metrics
- `node_config_snapshot`: Snapshot of node configuration at execution time
- `operator_config_snapshot`: Snapshot of operator parameters used
- `input_data_snapshot`: Snapshot of input curve metadata
- `otel_trace_id`: OpenTelemetry trace ID for distributed tracing
- `logs_uri`: S3/MinIO path to execution logs

### Relationship Between Tables

```
pipeline_executions (high-level)
    ├─ node_execution_ids[] → [node_executions.id, ...]
    └─ Links to multiple node_executions

node_executions (granular)
    ├─ parent_execution_id → pipeline_executions.id (if part of DAG)
    ├─ pipeline_id → pipelines.id (if part of pipeline)
    └─ Standalone if execution_type = 'node'
```

### Comparison with Other Systems

**GitHub Actions**:

- **Workflow Runs**: Similar to `pipeline_executions`
- **Job Runs**: Similar to `node_executions` (granular)
- **Step Logs**: Similar to our `logs_uri` and `error_details`
- **Artifacts**: Similar to our `output_curve_ids[]`

**Apache Airflow**:

- **DAG Runs**: Similar to `pipeline_executions`
- **Task Instances**: Similar to `node_executions`
- **XComs**: Similar to our `result_summary` JSONB
- **Task Logs**: Similar to our `logs_uri`

**Key Differences**:

- **Our System**: Tracks curve-level inputs/outputs (domain-specific)
- **Our System**: Stores operator version for reproducibility
- **Our System**: Links to Supabase Auth for user attribution
- **Our System**: Uses PostgreSQL JSONB for flexible metadata

### Why PostgreSQL (Not Arrow/DataFusion/Iceberg)?

**Decision**: Use PostgreSQL for execution audit trail.

**Rationale**:

1. **Structured Metadata**: Execution records are structured metadata (not large datasets)
2. **Query Performance**: PostgreSQL excels at querying structured data with indexes
3. **User Attribution**: Easy joins with `auth.users` table
4. **Real-time Updates**: Supabase Realtime works seamlessly with PostgreSQL
5. **ACID Transactions**: Ensures execution records are atomic
6. **JSONB Flexibility**: Allows flexible metadata without schema migrations

**When to Use Arrow/DataFusion/Iceberg**:

- **Large Execution Results**: If we need to store large result datasets
- **Analytics Queries**: If we need complex analytics on execution history
- **Time-Series Data**: If execution metrics become time-series data

**Current Approach**:

- PostgreSQL for execution metadata (who, what, when, status)
- Parquet files for actual curve data (via OpenDAL)
- JSONB snapshots for reproducibility (node config, operator config)

### Usage Examples

**Query Execution History for a Node**:

```sql
SELECT
    id,
    started_at,
    completed_at,
    status,
    execution_time_ms,
    executed_by_email,
    operator_id,
    operator_version,
    input_curve_count,
    output_curve_count
FROM node_executions
WHERE node_id = '...'
ORDER BY started_at DESC
LIMIT 50;
```

**Query Execution History for a Pipeline**:

```sql
SELECT
    pe.id AS pipeline_execution_id,
    ne.id AS node_execution_id,
    ne.node_id,
    n.name AS node_name,
    ne.started_at,
    ne.completed_at,
    ne.status,
    ne.execution_time_ms,
    ne.operator_id,
    ne.operator_version
FROM pipeline_executions pe
INNER JOIN node_executions ne ON ne.parent_execution_id = pe.id
INNER JOIN nodes n ON n.id = ne.node_id
WHERE pe.pipeline_id = '...'
ORDER BY ne.started_at DESC;
```

**Query Failed Executions**:

```sql
SELECT
    id,
    node_id,
    started_at,
    error_message,
    error_details,
    executed_by_email
FROM node_executions
WHERE status = 'failed'
ORDER BY started_at DESC
LIMIT 20;
```

## Future Enhancements

### Planned Improvements

1. **Actual UDF Execution**: Replace placeholder with real UDF calls
2. **Batch Execution**: Execute multiple nodes in parallel
3. **Progress Tracking**: Show execution progress for long-running operations
4. **Rollback Support**: Undo failed executions
5. **Execution History UI**: Display execution history component
6. **Execution Metrics Dashboard**: Visualize execution metrics
7. **Execution Retry**: Track retry attempts in `node_executions`
8. **Execution Caching**: Use `content_hash` to detect duplicate executions
9. **Execution Scheduling**: Add `scheduled_at` and `triggered_by` fields
10. **Execution Dependencies**: Track which executions depend on others
11. **Execution Rollback**: Store rollback information for failed executions
12. **Validation UI**: Real-time validation feedback for curve name conflicts

### DAG Execution Support

When implementing full DAG execution:

1. **Create `pipeline_executions` record** with status `'running'`
2. **For each node in DAG**:
   - Create `node_executions` record with `parent_execution_id` pointing to pipeline execution
   - Execute node
   - Update node execution record
3. **Update `pipeline_executions` record**:
   - `status = 'completed'` or `'failed'`
   - `node_execution_ids[] = [all node execution IDs]`
   - Aggregate metrics from all node executions

### Implementation Status

✅ **Completed**:

- SQL migration for `node_executions` table
- Backend execution logging (create record before execution, update after)
- Frontend user ID passing
- Success/error toast notifications
- Suffix auto-increment after successful execution
- Validation skip logic to prevent false errors

🔄 **Future Enhancements**:

- Execution history UI component
- Execution metrics dashboard
- Execution retry functionality
- Execution caching based on content hash
- Full DAG execution support
