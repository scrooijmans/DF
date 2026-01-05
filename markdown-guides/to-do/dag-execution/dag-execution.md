# DAG Execution Architecture Plan - MudRock

This document outlines a modular, type-safe approach to building DAGs (Directed Acyclic Graphs) for data extraction and processing, integrating frontend selections with backend execution through Arrow/DataFusion schemas.

## ✅ Modularization Strategy (mirrors Storage/OpenDAL architecture)

To ensure clean separation of concerns (like `storage-layer.md` and `opendal-integration.md`), we will split DAG functionality into dedicated crates with clear ownership and minimal coupling.

```
crates/dags/
├── core/            # Types only: DagDefinition, DagNode, NodeType, SchemaDefinition
├── validator/       # Graph + type validation (petgraph-powered)
├── storage/         # PipelineStorage (Postgres I/O, JSONB + normalized tables)
├── operators/       # OperatorRegistry + schemars param schemas
├── executor/        # Execution engine (tokio+mpsc, DataFusion, OpenTelemetry)
└── bridge/          # (Optional) Frontend bridge helpers, DTO adapters
```

Design rules:

- `core` has no deps on other DAG crates (types-only). All other crates depend on `core`.
- `validator` depends on `core` and `petgraph` (no DB, no execution code). Unit tests live here.
- `storage` depends on `core` (no `petgraph`, no execution). All SQL/serde live here.
- `operators` depends on `core`, `schemars`, and owns parameter schemas/registry.
- `executor` depends on `core`, `operators` and uses DataFusion/Arrow, tokio channels, tracing/OTel.

Public APIs (key files/functions):

- `crates/dags/validator/src/lib.rs`
  - `pub fn validate_structure(dag: &DagDefinition) -> Result<(), DagValidationError>`
  - `pub fn validate_acyclic(dag: &DagDefinition) -> Result<(), DagValidationError>`
  - `pub fn validate_orphans(dag: &DagDefinition) -> Result<(), DagValidationError>`
  - `pub fn validate_types(dag: &DagDefinition) -> Result<(), DagValidationError>`
  - `pub fn validate_connectivity(dag: &DagDefinition) -> Result<(), DagValidationError>`

- `crates/dags/storage/src/pipeline_storage.rs`
  - `pub async fn save_pipeline(&mut self, dag: &DagDefinition, user_id: Option<Uuid>) -> Result<Uuid, PipelineStorageError>`
  - `pub async fn load_pipeline(&self, id: Uuid) -> Result<DagDefinition, PipelineStorageError>`
  - `pub async fn list_pipelines(&self, project_id: Uuid) -> Result<Vec<PipelineSummary>, PipelineStorageError>`

- `crates/dags/operators/src/registry.rs`
  - `pub fn parameters_schema<T: JsonSchema>() -> serde_json::Value`
  - `pub fn get_operator(&self, id: &str, version: &str) -> Option<OperatorDefinition>`
  - Operators can be `implementation_type = "udf"` (backed by DataFusion UDFs) or `native_rust` (direct Rust implementations).
  - See `markdown-guides/datafusion/udfs/register-udfs.md` and `.../datafusion-udfs-best-practices.md` for UDF registry details.

- `crates/dags/executor/src/executor.rs`
  - `pub async fn execute(&self, dag: &DagDefinition) -> Result<ExecutionResult, DagExecutionError>`

Integration points:

- Tauri command `save_pipeline` calls `dag_validator::validate_structure()` first, then `PipelineStorage::save_pipeline()`.
- Tauri command `execute_pipeline` loads DAG via `PipelineStorage`, validates, then `DagExecutor::execute()`.
- Tauri command `get_available_operators` merges UDF-backed operators and built-in/native operators for the DAG editor palette.

Answer to "Do we need a specific petgraph validation crate?"

- Yes, internally create `crates/dags/validator` to encapsulate all `petgraph`-based validations and keep the executor and storage crates focused. No need for an external crate; a dedicated internal crate improves testability and keeps the surface area small.

---

## 📚 **Third-Party Libraries & Integration Strategy**

### **Core Libraries (Already Integrated)** ✅

**Graph & Execution**:

- ✅ **`petgraph`** - DAG structure, topological sort, cycle detection (`crates/dags/validator`, `crates/dags/executor`)
- ✅ **`tokio`** - Async runtime with bounded channels for backpressure (`crates/dags/executor`)

**Data Processing**:

- ✅ **`datafusion`** - SQL/DataFrame engine, UDF support, Parquet scanning (`crates/dags/executor`, `crates/dags/operators`)
- ✅ **`arrow`** - Columnar in-memory format, RecordBatch transport (`crates/dags/core`, `crates/dags/executor`)

**Storage**:

- ✅ **`opendal`** - Unified storage abstraction (S3/MinIO/Local) via `opendal-storage-adapter` (`crates/dags/executor`)

**Observability**:

- ✅ **`tracing`** + **`opentelemetry`** - Distributed tracing, metrics, spans (`crates/dags/executor`)

**Schema & Validation**:

- ✅ **`schemars`** - Auto-generate JSON Schema from Rust structs for UI forms (`crates/dags/operators`)
- ✅ **`serde`** / **`serde_json`** - JSON serialization/deserialization (all crates)

**Database**:

- ✅ **`tokio-postgres`** / **`sqlx`** - PostgreSQL access (`crates/dags/storage`)

**Frontend Visualization**:

- ✅ **`@xyflow/svelte`** (Svelte Flow) - Visual DAG editor (replaces Cytoscape.js recommendation)
- ✅ **`@dagrejs/dagre`** - Automatic hierarchical graph layout

### **Optional Libraries (Future Enhancements)** 🔮

**Expression Parsing** (for SQL Block nodes):

- **`sqlparser-rs`** - Parse SQL expressions into DataFusion `Expr` AST
  - **Use Case**: Allow users to write SQL predicates in operator nodes
  - **When**: If implementing "SQL Block" node type (optional advanced feature)
  - **Alternative**: Use DataFusion's built-in SQL parser (`ctx.sql()`)

**Geospatial Operations** (for spatial DAG nodes):

- **`geoarrow-rs`** - Arrow-native geometry arrays and spatial predicates
- **`rstar`** - R-tree spatial index for fast point-in-polygon queries
  - **Use Case**: Spatial operations (e.g., "wells_in_polygon", "point_in_polygon")
  - **When**: If adding geospatial operator nodes (future enhancement)
  - **Integration**: Execute geo subgraphs separately, expose results as MemTable to DataFusion

**Form Generation** (for operator parameter forms):

- **`@rjsf/core`** (React JSON Schema Form) or **`zod`** - Generate forms from JSON Schema
  - **Use Case**: Auto-generate parameter forms for operators from `parameters_schema`
  - **When**: If building advanced operator configuration UI
  - **Current**: Using shadcn-svelte forms (sufficient for MVP)

**Portable Query Plans** (for engine portability):

- **`Substrait`** - Portable IR for relational query plans
  - **Use Case**: Compile DAGs to Substrait for engine-agnostic execution (DataFusion today, DuckDB/Spark later)
  - **When**: If needing multi-engine support or long-term plan stability
  - **Strategy**: Keep JSON DAG as user-facing spec; emit Substrait as internal IR for DF-only subgraphs
  - **Note**: Not exposed to users; internal optimization target

**Visualization & Debugging**:

- **DOT Language** - Graph description language for query plan visualization
  - **Use Case**: Export DAGs as DOT for visualization/debugging in external tools (Graphviz)
  - **When**: If adding "Export Plan" feature for debugging
  - **Library**: `petgraph` can export to DOT format natively

**Distributed Execution** (for multi-user/multi-node):

- **`ballista`** - Distributed DataFusion execution
  - **Use Case**: Multi-user collaboration, resource management, distributed analytics
  - **When**: If scaling beyond single-node execution
  - **⚠️ Limitation**: DataFusion compatibility gap exists; community working to close it
  - **Alternative**: Use standalone DataFusion for MVP; add Ballista later if needed
  - **See**: `markdown-guides/to-do/rust-dag/distributed-query-execution.md` for detailed plan

**Sandboxed Execution** (for user-provided UDFs):

- **`wasmtime`** - WebAssembly runtime for sandboxed UDF execution
  - **Use Case**: Allow users to write custom UDFs safely (sandboxed, cannot crash system)
  - **When**: If implementing user-defined UDFs feature
  - **Note**: Built-in UDFs remain native Rust (fastest); WASM only for untrusted user code

**Iceberg Integration** (for advanced table management):

- **`iceberg-rs`** - Apache Iceberg table format support
  - **Use Case**: Time travel, schema evolution, branching (dev/prod)
  - **When**: If migrating from PostgreSQL control tables to Iceberg
  - **Current**: Using PostgreSQL for control plane (sufficient for MVP)
  - **Note**: Data outputs already stored as Parquet (Iceberg-compatible)

### **Library Selection Rationale**

**Why Svelte Flow over Cytoscape.js?**

- ✅ **Better Svelte integration** - Native Svelte component, no wrapper needed
- ✅ **Modern API** - Designed for React/Svelte, better TypeScript support
- ✅ **Active development** - More recent updates and community support
- ✅ **Dagre integration** - Built-in support for automatic layouts
- ✅ **Node handles** - Better support for connection validation (`isValidConnection`)

**Why PostgreSQL over Iceberg Control Tables?**

- ✅ **Simpler MVP** - PostgreSQL is already in use, no additional infrastructure
- ✅ **Queryable** - JSONB supports indexing and queries (`@>`, `->`, `->>`)
- ✅ **Normalized tables** - `pipeline_nodes` and `pipeline_edges` provide efficient graph queries
- ✅ **Future-proof** - Can migrate to Iceberg later if needed (Parquet outputs already compatible)

**Why DataFusion over Polars/DuckDB?**

- ✅ **Already integrated** - DataFusion is core to query engine
- ✅ **Arrow-native** - Zero-copy data transport between nodes
- ✅ **UDF support** - Built-in UDF registration and execution
- ✅ **SQL + DataFrame API** - Flexible query construction
- ✅ **Parquet support** - Native Parquet scanning with predicate pushdown

**Why OpenDAL over direct S3/MinIO clients?**

- ✅ **Unified abstraction** - Single API for S3/MinIO/Local/other backends
- ✅ **Already integrated** - Used throughout storage layer
- ✅ **Caching layer** - Moka cache integration for 10-100x performance improvement
- ✅ **Future-proof** - Easy to add new storage backends (GCS, Azure Blob, etc.)

### **Dependency Management**

**Backend Dependencies** (`crates/dags/*/Cargo.toml`):

```toml
# Core (required)
petgraph = "0.6"
tokio = { version = "1.0", features = ["full"] }
datafusion = { workspace = true }
arrow = { workspace = true }
schemars = { version = "0.8", features = ["derive"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tracing = "0.1"
opentelemetry = "0.21"
opentelemetry-otlp = "0.14"
tracing-opentelemetry = "0.21"

# Optional (future)
# sqlparser-rs = "0.40"  # For SQL expression parsing
# geoarrow-rs = "0.1"    # For geospatial operations
# rstar = "0.11"         # For spatial indexing
# wasmtime = "20.0"      # For sandboxed UDFs
```

**Frontend Dependencies** (`package.json`):

```json
{
  "dependencies": {
    "@xyflow/svelte": "^1.0.0", // ✅ Installed - Visual DAG editor
    "@dagrejs/dagre": "^0.8.5" // ✅ Installed - Automatic layout
    // "@rjsf/core": "^5.0.0",       // Optional - JSON Schema forms
    // "zod": "^3.22.0"               // Optional - Type validation
  }
}
```

### **Integration Patterns**

**Subgraph Partitioning** (from `operator_builder_plan.md`):

- **DF-only subgraphs**: Purely relational/UDF nodes → Compile into single DataFusion plan
- **Geo subgraphs**: Spatial operations → Execute with `geoarrow-rs`, expose as MemTable to DF
- **Hybrid**: Combine both patterns for complex workflows

**Content-Addressed Caching** (from `dataframe_storage_implementation_plan.md`):

- **Cache key**: `SHA256(dag_json + params + input_snapshots + udf_versions)`
- **Storage**: Parquet files in MinIO cache bucket, indexed by hash
- **Lookup**: Check `pipeline_executions.content_hash` before execution

**Expression Building** (from `operator_builder_plan.md`):

- **Safe predicates**: Parse user-provided SQL expressions into DataFusion `Expr` (avoid string concat)
- **Use**: `sqlparser-rs` to parse SQL → map AST to DataFusion expressions
- **Alternative**: Use DataFusion's built-in SQL parser (`ctx.sql()`) for simpler cases

---

## 📈 **Progress Summary**

**Last Updated**: 2025-01-30  
**Status**: Phase 1 Complete ✅, Phase 2 Backend Complete ✅, Frontend creation/listing wired ✅, Phase 1 Optimizations Complete ✅

### ✅ **Completed**

- **Phase 1.1**: Core Node Types (`crates/dags/core`) ✅ **OPTIMIZED**
  - NodeType enum (Ingestion, Operator, Output)
  - SchemaDefinition with validation
  - DagDefinition, DagNode, NodeMetadata structures
  - VersionStrategy enum for operator version management
  - Optional schema support for reference-based operator storage
  - **Performance**: Optimized `has_column()` to avoid unnecessary `String` allocations
  - **Performance**: Added capacity hints to `all_columns()` HashSet construction

- **Phase 1.2**: Database Schema (`db/init/02-pipeline-schema.sql`) ✅
  - All pipeline tables created (pipelines, pipeline_nodes, pipeline_edges, pipeline_executions, operator_registry, pipeline_lineage)
  - Operator versioning support (multiple versions per operator_id)
  - Composite unique constraint `(operator_id, version)`
  - Version management columns (`is_latest`, `deprecated_at`, `replacement_version`, `schema_compatible`)
  - Indexes for efficient queries

- **Phase 1.3**: Rust Best Practices & Performance Optimizations ✅ **NEW**
  - Updated `.cursor/rules/global.mdc` with comprehensive Rust best practices:
    - Memory & performance patterns (reduce allocations, use `Arc`, `bytes::Bytes`)
    - Async patterns (`tokio`, `async_trait`, `tokio::sync::RwLock`)
    - Error handling (`thiserror::Error` with `#[from]` attributes)
    - Storage layer patterns (OpenDAL, caching with `moka`)
    - Database patterns (`tokio_postgres`, `postgres_types::Json`)
    - Arrow/DataFusion patterns (`Arc<Schema>`, `Arc<RecordBatch>`)
    - Type safety (`uuid::Uuid`, `chrono::DateTime`)
    - Concurrency (`Arc`, bounded channels)
  - **Performance Optimizations**:
    - `crates/dags/core/src/schema.rs`: Eliminated unnecessary `String` allocations in `has_column()`
    - `crates/dags/validator/src/lib.rs`: Added capacity hints to `Graph` and `HashMap` construction, optimized return type
    - `crates/dags/storage/src/pipeline_storage.rs`: Added capacity hint for `node_schemas` Vec, fixed unused `mut` warning
  - **Code Quality**: All crates compile successfully with no warnings or errors

- **Phase 2.1**: Well Logs Ingestion Executor (`crates/dags/executor/src/ingestion/well_logs.rs`) ✅ **NEW**
  - `WellLogsIngestionExecutor` implementation with OpenDAL integration
  - DataFusion-based Parquet to RecordBatch conversion
  - Depth range filtering (`filter_depth_range()`)
  - Curve projection (`project_curves()`)
  - Multi-well data combination (`combine_batches()`)
  - Well ID column addition (`add_well_id_column()`)
  - Uses `EnhancedOpenDALStorageAdapter` for storage access (with caching)
  - Uses `ProjectDataLayoutManager` for path construction
  - Integrates with `dag-core` types (`IngestionSourceType`, `IngestionFilters`)

- **Phase 2.2**: Database Access Layer (`crates/dags/storage`) ✅
  - PipelineStorage implementation
  - OperatorRegistryLoader for loading operators from registry
  - Reference-based schema lookup from operator_registry
  - Support for VersionStrategy (Pinned, Latest, Compatible)
  - Backward compatibility with inline schemas

- **Phase 2.3**: Tauri Commands and Validation ✅
  - Created `crates/dags/validator` (petgraph-based) and integrated into Tauri
  - Implemented `save_pipeline` with validator + Postgres persistence via `PipelineStorage`
  - Implemented `load_pipeline` using `PipelineStorage`
  - Added workspace wiring for `dag-core`, `dag-validator`, `dag-storage`

- **Frontend Pipeline Creation & Listing**
  - New UI: `new-pipeline.svelte` creates pipelines against Supabase (PostgREST) using the global `PostgresProjectsState` selected project ✅
  - Service: `pipeline-service.ts` inserts a minimal non-null `dag_definition` stub and lists pipelines by project ✅
  - DAG Editor: `content-dag-editor-pipelines.svelte` lists pipelines for the selected project and auto-refreshes via Supabase Realtime ✅
  - Realtime SQL: `enable_realtime_all_public_tables.sql` added; idempotent, enables publication and basic SELECT policies for pipelines ✅
  - Backend command `save_pipeline` relaxed to allow empty DAGs (no nodes) for draft creation when used via Tauri (kept for future save-path) ✅
  - Sidebar selection: selectable pipeline items update global `ContentDagPipelineState.selectedPipelineId`; selection displayed in editor header ✅

### 🔄 **In Progress**

- **Svelte Flow Integration**: Visual DAG editor with Dagre layout
  - Type conversion utilities (`dag-converter.ts`, `dag-types.ts`) ✅
  - Component architecture: `content-dag-editor.svelte` fetches → converts → passes to `SF-pipeline-flow.svelte` ⏳
  - Dagre automatic layout with position preservation ⏳
  - Bidirectional conversion: `DagDefinition` ↔ Svelte Flow `Node[]/Edge[]` ⏳

### 📋 **Next Steps**

**Recommended Architecture Enhancements** (from `dag-execution-architecture-analysis.md`):

1. ✅ **Tokio Bounded Channels** - Parallel execution with backpressure handling
2. ✅ **OpenTelemetry** - Industry-standard observability and metrics
3. ✅ **Svelte Flow** (`@xyflow/svelte`) - Production-ready graph editor (modern, Svelte-native, handles large graphs)
4. ✅ **schemars** - Auto-generate JSON Schema from Rust structs for UI forms

- Short-term (Editor CRUD)
  1. Load pipeline into editor: fetch row (Supabase) and show editable `dag_definition` JSON in editor header. ✅
  2. Save updates: `updatePipelineDagDefinition(pipelineId, dagDefinition)` implemented with Supabase `.update(...)`. Save button wired. ✅
  3. Keep Tauri `save_pipeline` for server-side validation/execution path; add a "Validate & Save (Backend)" button that posts to Tauri for strict validation when needed. ⏭️

- Operator UX 4) Render operator catalog (already listing) with category filters and a basic parameter form scaffold using `parameters_schema` (JSON Schema → simple form).  
  5) Insert Operator node into editor graph and connect to upstream selection, persisting in `dag_definition` state.

- Ingestion UX 6) Reuse existing global states (wells, curves, markers, trajectories, tables, surfaces) to instantiate Ingestion nodes with derived schemas.  
  7) Add schema preview panel and connection validation hints in the editor (red/green edges).

- Validation/Execution 8) Frontend: lightweight validation (no cycles, single output, required fields).  
  9) Backend: implement `execute_pipeline` plumbing (load → validate via `dag-validator` → executor stub returning summary).

- Persistence / Realtime 10) Add Supabase Realtime subscription for `pipelines` updates to auto-refresh the open editor if the currently edited pipeline changes elsewhere (optimistic lock via updated_at).

---

## 🎯 Immediate Action Items (Post-compile)

1. Editor open/save flow

- Add open handler and load `dag_definition` into editor state
- Implement `updatePipelineDagDefinition` service + Save button

2. Operators crate bootstrap

- Create `crates/dags/operators` with `schemars` and a minimal set of parameter structs
- Expose `OperatorRegistry` API to support `get_available_operators`

3. Execution engine helpers

- Add `build_petgraph(dag)` and `merge_dependencies(...)` in `crates/dags/executor`
- Prepare `execute_pipeline` Tauri command to call `DagExecutor::execute`

4. Frontend editor foundation

- Add basic Svelte Flow editor component and map `DagDefinition <-> Svelte Flow nodes/edges`
- Wire Save/Load buttons to the new Tauri commands

### 🔮 **Future Enhancements (Optional)**

- **Operator Channels**: Replace `is_latest` with channels (`stable`, `beta`, `canary`) for gradual rollouts
- **Operator Families**: Group related operators semantically (e.g., `vshale` family with Larionov, Clavier, etc.)
- **DAG Revisions**: Git-like versioning with branches and immutable revisions
- **Semver Management**: Explicit semantic versioning with compatibility checks
- **Execution Hints**: Node-level optimization metadata (resources, ordering, caching)
- **Resolved Operators**: Explicit tracking of operator versions used per run

**Note**: See "Future Enhancements: Advanced Operator Management Patterns" section below for detailed analysis and migration paths.

## 🎯 **Executive Summary**

Users create DAGs by:

1. **Selecting data sources** from Postgres metadata (wells, curves) via frontend state
2. **Building filters/selections** (e.g., "Wells A & B, GR curve, depth 1000-2000m")
3. **Applying operators** (e.g., "GR → VShale", "despike", "filter outliers")
4. **Storing DAGs** in PostgreSQL with type-safe schema validation
5. **Executing DAGs** using DataFusion/Arrow with automatic type checking

**Key Innovation**: Type-safe node connections ensure operators receive compatible input schemas, preventing runtime errors through compile-time and execution-time validation.

---

## 🏗️ **Architecture Overview**

```
┌─────────────────────────────────────────────────────────────────┐
│                    FRONTEND (SvelteKit)                          │
├─────────────────────────────────────────────────────────────────┤
│  Global State                                                    │
│  ┌──────────────────┐  ┌──────────────────┐                   │
│  │ PostgresWells     │  │ CurveMetadata     │                   │
│  │ State (wells[])   │  │ State (curves[])  │                   │
│  └──────────────────┘  └──────────────────┘                   │
│           │                       │                             │
│           └───────────────────────┼───────────────────────┐     │
│                                   │                       │     │
│  ┌─────────────────────────────────▼───────────────────────▼─┐   │
│  │              DAG Builder UI Component                   │   │
│  │  ┌─────────────────────────────────────────────────┐   │   │
│  │  │ Node 1: Data Ingestion                          │   │   │
│  │  │  - Select wells: [Well A, Well B]                │   │   │
│  │  │  - Select curves: [GR, RHOB]                    │   │   │
│  │  │  - Depth range: 1000-2000m                       │   │   │
│  │  │  Output Schema: {well_id, depth, gr, rhob}       │   │   │
│  │  └─────────────────────────────────────────────────┘   │   │
│  │                           │                             │   │
│  │  ┌─────────────────────────▼───────────────────────┐   │   │
│  │  │ Node 2: Operator (GR → VShale)                │   │   │
│  │  │  Input Schema: {well_id, depth, gr, ...}      │   │   │
│  │  │  Operator: shale_volume_larionov(gr, ...)      │   │   │
│  │  │  Output Schema: {well_id, depth, gr, vsh}      │   │   │
│  │  └─────────────────────────────────────────────────┘   │   │
│  │                           │                             │   │
│  │  ┌─────────────────────────▼───────────────────────┐   │   │
│  │  │ Node 3: Operator (Despike)                      │   │   │
│  │  │  Input Schema: {well_id, depth, gr, vsh, ...}   │   │   │
│  │  │  Operator: despike(vsh, threshold=0.1)          │   │   │
│  │  │  Output Schema: {well_id, depth, gr, vsh, ...} │   │   │
│  │  └─────────────────────────────────────────────────┘   │   │
│  └─────────────────────────────────────────────────────────┘   │
│                           │                                     │
│                           │ Tauri Command                      │
│                           │ save_dag(json_dag_definition)      │
│                           ▼                                     │
└─────────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────────┐
│                  BACKEND (Rust)                                  │
├─────────────────────────────────────────────────────────────────┤
│  ┌─────────────────────────────────────────────────────────────┐ │
│  │              DAG Storage (PostgreSQL)                       │ │
│  │  ┌─────────────────────────────────────────────────────┐   │ │
│  │  │  Table: pipelines (DAG definitions)                 │   │ │
│  │  │  Table: pipeline_nodes (Node configurations)        │   │ │
│  │  │  Table: pipeline_edges (Node connections)            │   │ │
│  │  │  Table: pipeline_executions (Execution history)      │   │ │
│  │  └─────────────────────────────────────────────────────┘   │ │
│  └─────────────────────────────────────────────────────────────┘ │
│                            │                                     │
│                            ▼                                     │
│  ┌─────────────────────────────────────────────────────────────┐ │
│  │              DAG Compiler & Validator                         │ │
│  │  ┌─────────────────────────────────────────────────────┐   │ │
│  │  │  1. Schema Validation                                │   │ │
│  │  │     - Validate input/output schemas for each node    │   │ │
│  │  │     - Check operator compatibility                  │   │ │
│  │  │  2. Type Safety Checking                            │   │ │
│  │  │     - Ensure node connections are type-compatible    │   │ │
│  │  │  3. Optimization                                    │   │ │
│  │  │     - Fuse compatible operators                     │   │ │
│  │  │     - Pushdown filters to storage layer             │   │ │
│  │  └─────────────────────────────────────────────────────┘   │ │
│  └─────────────────────────────────────────────────────────────┘ │
│                            │                                     │
│                            ▼                                     │
│  ┌─────────────────────────────────────────────────────────────┐ │
│  │              DAG Executor (DataFusion-based)                │ │
│  │  ┌─────────────────────────────────────────────────────┐   │ │
│  │  │  Node Execution Engine                              │   │ │
│  │  │  - Ingestion Nodes: Fetch from OpenDAL storage      │   │ │
│  │  │  - Operator Nodes: Apply transformations            │   │ │
│  │  │  - Output Nodes: Return/export results              │   │ │
│  │  └─────────────────────────────────────────────────────┘   │ │
│  └─────────────────────────────────────────────────────────────┘ │
│                            │                                     │
│                            ▼                                     │
│  ┌─────────────────────────────────────────────────────────────┐ │
│  │              Storage Layer Integration                      │ │
│  │  ┌──────────────────┐  ┌──────────────────┐                 │ │
│  │  │ OpenDAL         │  │ DataFusion       │                 │ │
│  │  │ (Parquet fetch) │  │ (SQL execution)  │                 │ │
│  │  └──────────────────┘  └──────────────────┘                 │ │
│  └─────────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
```

---

## 📋 **Phase 1: Node Type System & Schema Definitions**

### **1.1 Core Node Type Enum**

```rust
// crates/dags/core/src/node_types.rs

use arrow::datatypes::Schema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Core node types in the DAG system
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum NodeType {
    /// Data ingestion node - fetches data from storage
    Ingestion {
        source_type: IngestionSourceType,
        filters: IngestionFilters,
        /// Expected output schema after ingestion
        output_schema: SchemaDefinition,
    },

    /// Operator node - applies transformations
    Operator {
        operator_id: String,
        operator_version: String,
        parameters: HashMap<String, serde_json::Value>,
        /// Required input schema
        input_schema: SchemaDefinition,
        /// Expected output schema
        output_schema: SchemaDefinition,
    },

    /// Output node - exports/returns results
    Output {
        output_type: OutputType,
        /// Required input schema
        input_schema: SchemaDefinition,
    },
}

/// Types of data sources for ingestion nodes
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "source", rename_all = "snake_case")]
pub enum IngestionSourceType {
    /// Fetch well log data from Parquet storage
    WellLogs {
        project_id: String,
        well_ids: Vec<String>,
        curve_names: Vec<String>,
        depth_range: Option<(f64, f64)>,
    },

    /// Fetch surface data
    Surface {
        project_id: String,
        surface_name: String,
    },

    /// Fetch well markers/tops
    WellMarkers {
        project_id: String,
        well_ids: Vec<String>,
    },

    /// Fetch from PostgreSQL table
    PostgresTable {
        table_name: String,
        filters: Option<HashMap<String, serde_json::Value>>,
    },
}

/// Filters for ingestion nodes
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IngestionFilters {
    /// Depth range filter (min_depth, max_depth)
    pub depth_range: Option<(f64, f64)>,

    /// Curve value filters (min, max per curve)
    pub curve_filters: HashMap<String, CurveValueFilter>,

    /// Quality flags to include/exclude
    pub quality_flags: Option<QualityFlagFilter>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurveValueFilter {
    pub min_value: Option<f64>,
    pub max_value: Option<f64>,
    pub exclude_nulls: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityFlagFilter {
    pub include_flags: Vec<String>,
    pub exclude_flags: Vec<String>,
}

/// Output node types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum OutputType {
    /// Return data to frontend (as Arrow RecordBatch)
    Frontend,

    /// Export to Parquet file
    Parquet {
        storage_path: String,
    },

    /// Write to PostgreSQL table
    PostgresTable {
        table_name: String,
    },

    /// Export to CSV
    Csv {
        storage_path: String,
    },
}

/// Schema definition for type-safe node connections
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SchemaDefinition {
    /// Schema identifier (e.g., "well_logs_standard")
    pub schema_id: String,

    /// Arrow schema JSON representation
    pub arrow_schema_json: String,

    /// Required columns (must be present)
    pub required_columns: Vec<String>,

    /// Optional columns (may be present)
    pub optional_columns: Vec<String>,
}

impl SchemaDefinition {
    /// Validate that an actual Arrow schema matches this definition
    pub fn validate_schema(&self, actual_schema: &Schema) -> Result<(), SchemaValidationError> {
        // Check required columns exist
        for required_col in &self.required_columns {
            if actual_schema.field_with_name(required_col).is_err() {
                return Err(SchemaValidationError::MissingRequiredColumn(
                    required_col.clone()
                ));
            }
        }

        // Check column types match
        // TODO: Implement type matching logic
        Ok(())
    }
}
```

### **1.2 DAG Node Structure**

```rust
// crates/dags/core/src/node.rs

use arrow::datatypes::Schema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Complete DAG node definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DagNode {
    /// Unique node identifier
    pub id: Uuid,

    /// Human-readable node name
    pub name: String,

    /// Node type and configuration
    pub node_type: NodeType,

    /// IDs of nodes this node depends on (upstream nodes)
    pub dependencies: Vec<Uuid>,

    /// Additional node metadata
    pub metadata: NodeMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeMetadata {
    /// Position in visual editor (x, y coordinates)
    pub position: Option<(f64, f64)>,

    /// Whether this node's output should be cached
    pub cache_enabled: bool,

    /// Timeout for node execution (seconds)
    pub timeout_seconds: Option<u64>,

    /// Tags for categorization
    pub tags: Vec<String>,

    /// Description/user notes
    pub description: Option<String>,
}

/// DAG definition (complete pipeline)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DagDefinition {
    /// Unique DAG identifier
    pub id: Uuid,

    /// DAG name
    pub name: String,

    /// Project this DAG belongs to
    pub project_id: Uuid,

    /// All nodes in the DAG
    pub nodes: Vec<DagNode>,

    /// DAG version (for version control)
    pub version: String,

    /// DAG metadata
    pub metadata: DagMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DagMetadata {
    /// DAG description
    pub description: Option<String>,

    /// Author/user who created this DAG
    pub created_by: String,

    /// Creation timestamp
    pub created_at: chrono::DateTime<chrono::Utc>,

    /// Last modification timestamp
    pub updated_at: chrono::DateTime<chrono::Utc>,

    /// Tags for search/categorization
    pub tags: Vec<String>,
}
```

---

## 📊 **Phase 2: Database Schema for DAG Storage** ✅ **COMPLETED**

### **2.1 PostgreSQL Tables** ✅ **COMPLETED**

```sql
-- Table: pipelines (stores DAG definitions)
CREATE TABLE pipelines (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    project_id UUID NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    description TEXT,
    version TEXT NOT NULL DEFAULT '1.0.0',
    dag_definition JSONB NOT NULL, -- Complete DagDefinition JSON
    created_by UUID NOT NULL REFERENCES users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    tags TEXT[] DEFAULT '{}',
    is_active BOOLEAN DEFAULT true,
    is_public BOOLEAN DEFAULT false,

    -- Indexes
    CONSTRAINT pipelines_project_name_version_unique UNIQUE (project_id, name, version)
);

CREATE INDEX idx_pipelines_project_id ON pipelines(project_id);
CREATE INDEX idx_pipelines_created_by ON pipelines(created_by);
CREATE INDEX idx_pipelines_tags ON pipelines USING GIN(tags);

-- Table: pipeline_nodes (normalized node storage for querying)
CREATE TABLE pipeline_nodes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    pipeline_id UUID NOT NULL REFERENCES pipelines(id) ON DELETE CASCADE,
    node_id UUID NOT NULL, -- Node ID within the DAG
    node_name TEXT NOT NULL,
    node_type TEXT NOT NULL, -- 'ingestion', 'operator', 'output'
    node_config JSONB NOT NULL, -- NodeType enum as JSON
    input_schema JSONB, -- SchemaDefinition JSON
    output_schema JSONB, -- SchemaDefinition JSON
    dependencies JSONB NOT NULL DEFAULT '[]'::jsonb, -- Array of node UUIDs
    position_x FLOAT,
    position_y FLOAT,
    metadata JSONB DEFAULT '{}'::jsonb,

    -- Indexes
    CONSTRAINT pipeline_nodes_pipeline_node_unique UNIQUE (pipeline_id, node_id)
);

CREATE INDEX idx_pipeline_nodes_pipeline_id ON pipeline_nodes(pipeline_id);
CREATE INDEX idx_pipeline_nodes_node_type ON pipeline_nodes(node_type);
CREATE INDEX idx_pipeline_nodes_output_schema ON pipeline_nodes USING GIN(output_schema);
CREATE INDEX idx_pipeline_nodes_input_schema ON pipeline_nodes USING GIN(input_schema);

-- Table: pipeline_edges (explicit edge storage for querying)
CREATE TABLE pipeline_edges (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    pipeline_id UUID NOT NULL REFERENCES pipelines(id) ON DELETE CASCADE,
    source_node_id UUID NOT NULL, -- Source node UUID
    target_node_id UUID NOT NULL, -- Target node UUID
    edge_type TEXT NOT NULL DEFAULT 'data_flow', -- 'data_flow', 'conditional', 'error'

    CONSTRAINT pipeline_edges_source_target_unique UNIQUE (pipeline_id, source_node_id, target_node_id)
);

CREATE INDEX idx_pipeline_edges_pipeline_id ON pipeline_edges(pipeline_id);
CREATE INDEX idx_pipeline_edges_source ON pipeline_edges(source_node_id);
CREATE INDEX idx_pipeline_edges_target ON pipeline_edges(target_node_id);

-- Table: pipeline_executions (execution history)
-- Enhanced with concepts from dataframe_storage_implementation_plan.md
CREATE TABLE pipeline_executions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    pipeline_id UUID NOT NULL REFERENCES pipelines(id) ON DELETE CASCADE,
    status TEXT NOT NULL, -- 'pending', 'running', 'completed', 'failed', 'cancelled'
    started_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,
    execution_time_ms BIGINT,
    error_message TEXT,
    result_summary JSONB, -- {row_count, column_count, sample_data}
    executed_by UUID REFERENCES users(id),
    input_parameters JSONB, -- Any runtime parameters passed
    execution_log JSONB, -- Per-node execution logs

    -- Enhanced metrics (from dataframe_storage_implementation_plan.md)
    metrics JSONB, -- {rows_in, rows_out, bytes_scanned, bytes_written, cpu_ms, wall_ms, mem_peak_bytes}

    -- Content addressing for caching (from dataframe_storage_implementation_plan.md)
    content_hash TEXT, -- Hash of DAG + params + input snapshots for cache lookup

    -- Reproducibility (from dataframe_storage_implementation_plan.md)
    engine_versions JSONB, -- {app_version, datafusion_version, arrow_version}
    input_snapshots JSONB, -- [{table, snapshot_id}] - pinned input data versions

    -- Observability (from dataframe_storage_implementation_plan.md)
    otel_trace_id TEXT, -- OpenTelemetry trace ID (optional)
    logs_uri TEXT, -- S3 path to execution logs (optional)

    CONSTRAINT pipeline_executions_status_check
        CHECK (status IN ('pending', 'running', 'completed', 'failed', 'cancelled'))
);

CREATE INDEX idx_pipeline_executions_pipeline_id ON pipeline_executions(pipeline_id);
CREATE INDEX idx_pipeline_executions_status ON pipeline_executions(status);
CREATE INDEX idx_pipeline_executions_started_at ON pipeline_executions(started_at DESC);
CREATE INDEX idx_pipeline_executions_content_hash ON pipeline_executions(content_hash); -- For cache lookup
CREATE INDEX idx_pipeline_executions_executed_by ON pipeline_executions(executed_by);

-- Table: pipeline_lineage (tracks data transformations - from dataframe_storage_implementation_plan.md)
CREATE TABLE pipeline_lineage (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    execution_id UUID NOT NULL REFERENCES pipeline_executions(id) ON DELETE CASCADE,
    source_node_id UUID NOT NULL,
    target_node_id UUID NOT NULL,
    source_data_snapshot TEXT, -- Optional: snapshot ID if materialized
    transformation_type TEXT, -- 'ingestion', 'operator', 'filter', etc.
    transformation_config JSONB, -- Parameters for the transformation

    CONSTRAINT pipeline_lineage_execution_nodes_unique UNIQUE (execution_id, source_node_id, target_node_id)
);

CREATE INDEX idx_pipeline_lineage_execution_id ON pipeline_lineage(execution_id);
CREATE INDEX idx_pipeline_lineage_source ON pipeline_lineage(source_node_id);
CREATE INDEX idx_pipeline_lineage_target ON pipeline_lineage(target_node_id);

-- Table: operator_registry (catalog of available operators)
-- ✅ Updated: Supports multiple versions per operator_id with version management
CREATE TABLE IF NOT EXISTS public.operator_registry (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    operator_id TEXT NOT NULL, -- e.g., 'shale_volume_larionov' (removed UNIQUE - supports multiple versions)
    operator_name TEXT NOT NULL,
    category TEXT NOT NULL, -- 'subsurface', 'signal', 'wells', 'mining', 'arrays', 'quality'
    description TEXT,
    version TEXT NOT NULL DEFAULT '1.0.0',
    input_schema JSONB NOT NULL, -- Required input SchemaDefinition
    output_schema JSONB NOT NULL, -- Expected output SchemaDefinition
    parameters_schema JSONB NOT NULL, -- JSON Schema for parameter validation
    implementation_type TEXT NOT NULL, -- 'native_rust', 'udf', 'wasm'
    implementation_path TEXT, -- Path to implementation
    is_builtin BOOLEAN DEFAULT true, -- Built-in vs custom operator
    created_by UUID, -- References users table (if exists)
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- Version management for operator updates
    is_latest BOOLEAN DEFAULT false, -- Indicates if this is the latest version (for auto-update strategy)
    deprecated_at TIMESTAMPTZ, -- Timestamp when this version was deprecated
    replacement_version TEXT, -- Version that replaces this deprecated version
    schema_compatible BOOLEAN DEFAULT true, -- Whether this version is schema-compatible with previous versions

    -- Constraints
    CONSTRAINT operator_registry_category_check
        CHECK (category IN ('subsurface', 'signal', 'wells', 'mining', 'arrays', 'quality')),
    CONSTRAINT operator_registry_operator_version_unique
        UNIQUE (operator_id, version) -- Composite unique key: one version per operator_id
);

CREATE INDEX IF NOT EXISTS idx_operator_registry_category ON public.operator_registry(category);
CREATE INDEX IF NOT EXISTS idx_operator_registry_operator_id ON public.operator_registry(operator_id);
CREATE INDEX IF NOT EXISTS idx_operator_registry_builtin ON public.operator_registry(is_builtin) WHERE is_builtin = true;
CREATE INDEX IF NOT EXISTS idx_operator_registry_latest ON public.operator_registry(operator_id, is_latest) WHERE is_latest = true; -- For finding latest version
CREATE INDEX IF NOT EXISTS idx_operator_registry_version ON public.operator_registry(operator_id, version); -- For version lookups
```

### **2.2 Database Access Layer** ✅ **COMPLETED**

```rust
// crates/dags/storage/src/pipeline_storage.rs

use tokio_postgres::Client;
use uuid::Uuid;
use crate::core::{DagDefinition, DagNode};

pub struct PipelineStorage {
    client: Client,
}

impl PipelineStorage {
    /// Save a DAG definition to database
    /// ✅ Implemented: Supports reference-based operator schemas with automatic lookup
    pub async fn save_pipeline(
        &mut self,
        dag: &DagDefinition,
        user_id: Option<Uuid>,
    ) -> Result<Uuid, PipelineStorageError> {
        let pipeline_id = dag.id;

        // Extract schemas before transaction (looks up from operator_registry if needed)
        let mut node_schemas = Vec::new();
        for node in &dag.nodes {
            let schemas_result = Self::extract_schemas(&node.node_type, &self.client).await?;
            node_schemas.push(schemas_result);
        }

        // Begin transaction
        let mut tx = self.client.transaction().await?;

        // Serialize DAG definition to JSONB
        let dag_json = Json(serde_json::to_value(dag)?);

        // Insert main pipeline record
        tx.execute(
            r#"
            INSERT INTO pipelines (id, project_id, name, description, version, dag_definition, created_by)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            ON CONFLICT (project_id, name, version) DO UPDATE
            SET dag_definition = EXCLUDED.dag_definition,
                updated_at = NOW()
            "#,
            &[
                &pipeline_id,
                &dag.project_id,
                &dag.name,
                &dag.metadata.description,
                &dag.version,
                &dag_json,
                &user_id,
            ],
        )
        .await?;

        // Delete existing nodes and edges (for updates)
        tx.execute(
            "DELETE FROM pipeline_nodes WHERE pipeline_id = $1",
            &[&pipeline_id],
        )
        .await?;
        tx.execute(
            "DELETE FROM pipeline_edges WHERE pipeline_id = $1",
            &[&pipeline_id],
        )
        .await?;

        // Insert nodes (use pre-extracted schemas)
        for (node, (input_schema, output_schema)) in dag.nodes.iter().zip(node_schemas.iter()) {
            let node_type_str = match &node.node_type {
                NodeType::Ingestion { .. } => "ingestion",
                NodeType::Operator { .. } => "operator",
                NodeType::Output { .. } => "output",
            };

            // ... node insertion logic with pre-extracted schemas ...

        tx.commit().await?;
        Ok(pipeline_id)
    }

    /// Load a DAG definition from database
    /// ✅ Implemented: Loads complete DAG from JSONB
    pub async fn load_pipeline(&self, pipeline_id: Uuid) -> Result<DagDefinition, PipelineStorageError> {
        let row = self
            .client
            .query_opt(
                "SELECT dag_definition FROM pipelines WHERE id = $1",
                &[&pipeline_id],
            )
            .await?;

        let row = row.ok_or_else(|| PipelineStorageError::NotFound(pipeline_id))?;

        let dag_json: Json<Value> = row.get("dag_definition");

        let dag: DagDefinition = serde_json::from_value(dag_json.0)
            .map_err(|e| {
                PipelineStorageError::Deserialization(format!("Failed to deserialize DAG: {}", e))
            })?;

        Ok(dag)
    }

    /// List all pipelines for a project
    pub async fn list_pipelines(
        &self,
        project_id: Uuid,
    ) -> Result<Vec<PipelineSummary>, PipelineStorageError> {
        let rows = sqlx::query!(
            r#"
            SELECT id, name, description, version, created_at, updated_at, tags
            FROM pipelines
            WHERE project_id = $1 AND is_active = true
            ORDER BY updated_at DESC
            "#,
            project_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(|r| PipelineSummary {
            id: r.id,
            name: r.name,
            description: r.description,
            version: r.version,
            created_at: r.created_at,
            updated_at: r.updated_at,
            tags: r.tags,
        }).collect())
    }

    /// Extract input and output schemas from a node type
    /// ✅ Implemented: For Operator nodes, if schemas are None (reference-based), looks up from operator_registry
    /// Supports VersionStrategy: Pinned, Latest, Compatible
    async fn extract_schemas(
        node_type: &NodeType,
        client: &Client,
    ) -> Result<(Option<SchemaDefinition>, Option<SchemaDefinition>), PipelineStorageError> {
        match node_type {
            NodeType::Ingestion { output_schema, .. } => {
                Ok((None, Some(output_schema.clone())))
            }
            NodeType::Operator {
                operator_id, operator_version, input_schema, output_schema, version_strategy, ..
            } => {
                // If schemas are stored inline (backward compatibility), use them
                // Otherwise, lookup from operator_registry
                match (input_schema, output_schema) {
                    (Some(in_schema), Some(out_schema)) => {
                        // Inline schemas (old format)
                        Ok((Some(in_schema.clone()), Some(out_schema.clone())))
                    }
                    _ => {
                        // Reference-based (new format) - lookup from registry
                        // Determine version based on VersionStrategy
                        let version_to_use = match version_strategy {
                            Some(VersionStrategy::Pinned { version }) => version.clone(),
                            Some(VersionStrategy::Latest) => {
                                let latest_op = OperatorRegistryLoader::load_latest_operator(operator_id, client).await?;
                                latest_op.version
                            }
                            Some(VersionStrategy::Compatible { fallback_version }) => {
                                match OperatorRegistryLoader::load_latest_operator(operator_id, client).await {
                                    Ok(latest_op) => latest_op.version,
                                    Err(_) => fallback_version.clone(),
                                }
                            }
                            None => operator_version.clone(), // Default: use pinned version
                        };

                        let operator = OperatorRegistryLoader::load_operator(operator_id, &version_to_use, client).await?;
                        Ok((Some(operator.input_schema), Some(operator.output_schema)))
                    }
                }
            }
            NodeType::Output { input_schema, .. } => {
                Ok((Some(input_schema.clone()), None))
            }
        }
    }
}
```

---

## 🔗 **Phase 3: Frontend-to-Backend Bridge**

### **3.1 Frontend DAG Builder Component**

```typescript
// src/lib/components/pipelines/dag-builder.svelte.ts

import { getPostgresWellsState } from "$lib/state/postgres/postgres-wells-state.svelte";
import { getPostgresCurveMetadataState } from "$lib/state/postgres/postgres-curve-metadata-state.svelte";
import { invoke } from "@tauri-apps/api/core";

export interface IngestionNodeConfig {
  sourceType: "well_logs" | "surface" | "well_markers" | "postgres_table";
  projectId: string;

  // Well logs specific
  wellIds?: string[];
  curveNames?: string[];
  depthRange?: { min: number; max: number };

  // Surface specific
  surfaceName?: string;

  // Postgres table specific
  tableName?: string;
  filters?: Record<string, any>;
}

export interface OperatorNodeConfig {
  operatorId: string;
  operatorVersion: string;
  parameters: Record<string, any>;
}

export interface OutputNodeConfig {
  outputType: "frontend" | "parquet" | "postgres_table" | "csv";
  storagePath?: string;
  tableName?: string;
}

export class DagBuilderState {
  nodes = $state<DagNode[]>([]);
  edges = $state<DagEdge[]>([]);
  selectedNodeId = $state<string | null>(null);

  // Helper methods to create nodes from frontend state
  createIngestionNodeFromSelections(config: IngestionNodeConfig): DagNode {
    const wellsState = getPostgresWellsState();
    const curvesState = getPostgresCurveMetadataState();

    // Build node from user selections
    return {
      id: crypto.randomUUID(),
      name: `Ingest ${config.sourceType}`,
      nodeType: {
        type: "ingestion",
        sourceType: config.sourceType,
        filters: {
          depthRange: config.depthRange
            ? [config.depthRange.min, config.depthRange.max]
            : null,
          curveFilters: {},
        },
        outputSchema: this.inferOutputSchema(config),
      },
      dependencies: [],
      metadata: {
        position: null,
        cacheEnabled: true,
        timeoutSeconds: null,
        tags: [],
        description: null,
      },
    };
  }

  private inferOutputSchema(config: IngestionNodeConfig): SchemaDefinition {
    // Infer schema based on selected data
    if (config.sourceType === "well_logs") {
      return {
        schemaId: "well_logs_standard",
        arrowSchemaJson: JSON.stringify({
          fields: [
            { name: "well_id", type: "utf8", nullable: false },
            { name: "depth", type: "float64", nullable: false },
            ...(config.curveNames?.map((name) => ({
              name,
              type: "float64",
              nullable: true,
            })) || []),
          ],
        }),
        requiredColumns: ["well_id", "depth"],
        optionalColumns: config.curveNames || [],
      };
    }
    // ... other source types
    throw new Error(`Unknown source type: ${config.sourceType}`);
  }

  async saveDag(projectId: string): Promise<string> {
    const dagDefinition = {
      id: crypto.randomUUID(),
      name: "My Pipeline",
      projectId,
      nodes: this.nodes,
      version: "1.0.0",
      metadata: {
        description: null,
        createdBy: "current_user_id", // Get from auth
        createdAt: new Date().toISOString(),
        updatedAt: new Date().toISOString(),
        tags: [],
      },
    };

    // Send to backend via Tauri command
    const pipelineId = await invoke<string>("save_pipeline", {
      dagDefinition: JSON.parse(JSON.stringify(dagDefinition)),
    });

    return pipelineId;
  }

  async executeDag(pipelineId: string): Promise<ExecutionResult> {
    return await invoke<ExecutionResult>("execute_pipeline", {
      pipelineId,
    });
  }
}
```

### **3.2 Tauri Commands**

```rust
// src-tauri/src/tauri_commands/pipeline_commands.rs

use crate::dags::core::DagDefinition;
use crate::dags::storage::PipelineStorage;
use crate::dags::executor::DagExecutor;

#[tauri::command]
pub async fn save_pipeline(
    dag_definition: serde_json::Value,
    user_id: String,
) -> Result<String, String> {
    // Deserialize DAG definition
    let dag: DagDefinition = serde_json::from_value(dag_definition)
        .map_err(|e| format!("Failed to parse DAG definition: {}", e))?;

    // Validate DAG structure
    validate_dag_structure(&dag)?;

    // Save to database
    let storage = PipelineStorage::new(pool).await?;
    let user_uuid = Uuid::parse_str(&user_id)
        .map_err(|e| format!("Invalid user ID: {}", e))?;

    let pipeline_id = storage.save_pipeline(&dag, user_uuid).await
        .map_err(|e| e.to_string())?;

    Ok(pipeline_id.to_string())
}

#[tauri::command]
pub async fn execute_pipeline(
    pipeline_id: String,
) -> Result<ExecutionResult, String> {
    let pipeline_uuid = Uuid::parse_str(&pipeline_id)
        .map_err(|e| format!("Invalid pipeline ID: {}", e))?;

    // Load DAG from database
    let storage = PipelineStorage::new(pool).await?;
    let dag = storage.load_pipeline(pipeline_uuid).await
        .map_err(|e| e.to_string())?;

    // Execute DAG
    let executor = DagExecutor::new(storage_manager, query_engine).await?;
    let result = executor.execute(&dag).await
        .map_err(|e| e.to_string())?;

    Ok(result)
}

#[tauri::command]
pub async fn get_available_operators(
    category: Option<String>,
) -> Result<Vec<OperatorInfo>, String> {
    let storage = PipelineStorage::new(pool).await?;
    let operators = storage.list_operators(category.as_deref()).await
        .map_err(|e| e.to_string())?;

    Ok(operators)
}

#[tauri::command]
pub async fn validate_node_connection(
    source_node_id: String,
    target_node_id: String,
    pipeline_id: String,
) -> Result<bool, String> {
    // Load nodes and check schema compatibility
    let storage = PipelineStorage::new(pool).await?;
    let source_node = storage.get_node_by_id(&source_node_id).await?;
    let target_node = storage.get_node_by_id(&target_node_id).await?;

    // Validate schema compatibility
    let compatible = match (&source_node.node_type, &target_node.node_type) {
        (NodeType::Ingestion { output_schema, .. },
         NodeType::Operator { input_schema, .. }) => {
            output_schema.validate_compatible(input_schema)?
        }
        (NodeType::Operator { output_schema, .. },
         NodeType::Operator { input_schema, .. }) => {
            output_schema.validate_compatible(input_schema)?
        }
        (NodeType::Operator { output_schema, .. },
         NodeType::Output { input_schema, .. }) => {
            output_schema.validate_compatible(input_schema)?
        }
        _ => false,
    };

    Ok(compatible)
}
```

---

## ⚙️ **Phase 4: Type-Safe Execution Engine**

### **4.1 DAG Executor with Type Safety**

```rust
// crates/dags/executor/src/executor.rs

use arrow::record_batch::RecordBatch;
use datafusion::prelude::*;
use petgraph::{Graph, algo::toposort, Direction};
use tokio::sync::mpsc;
use std::collections::HashMap;
use uuid::Uuid;
use tracing_opentelemetry::OpenTelemetrySpanExt;

pub struct DagExecutor {
    storage_manager: Arc<MudRockStorageManager>,
    query_engine: Arc<QueryEngine>,
    operator_registry: OperatorRegistry,
    execution_cache: Arc<MokaCache<String, RecordBatch>>,
}

impl DagExecutor {
    /// Execute a complete DAG with parallel execution and backpressure handling
    /// ✅ Uses tokio bounded channels for backpressure
    /// ✅ Uses OpenTelemetry for observability
    pub async fn execute(
        &self,
        dag: &DagDefinition,
    ) -> Result<ExecutionResult, DagExecutionError> {
        // Create root span for execution
        let execution_span = tracing::info_span!("dag_execution", dag_id = %dag.id, dag_name = dag.name.as_str());
        let _guard = execution_span.enter();

        // 1. Validate DAG structure
        self.validate_dag(dag)?;

        // 2. Build petgraph Graph
        let graph = self.build_petgraph(dag)?;

        // 3. Topological sort using petgraph
        let execution_order = toposort(&graph, None)
            .map_err(|cycle| DagExecutionError::CycleDetected(format!("Cycle detected: {:?}", cycle)))?;

        // 4. Create bounded channels for each node (backpressure handling)
        let mut node_senders: HashMap<Uuid, mpsc::Sender<RecordBatch>> = HashMap::new();
        let mut node_receivers: HashMap<Uuid, mpsc::Receiver<RecordBatch>> = HashMap::new();

        // Channel buffer size (limits memory usage)
        const CHANNEL_BUFFER: usize = 10;

        for node_id in dag.nodes.iter().map(|n| n.id) {
            let (tx, rx) = mpsc::channel::<RecordBatch>(CHANNEL_BUFFER);
            node_senders.insert(node_id, tx);
            node_receivers.insert(node_id, rx);
        }

        // 5. Spawn tasks for parallel execution (nodes without dependencies execute first)
        let mut tasks = Vec::new();
        let executor = Arc::new(self.clone()); // Or wrap self in Arc if needed

        for node_id in execution_order.iter().rev() { // Process in reverse order for dependency resolution
            let node = dag.nodes.iter()
                .find(|n| n.id == *node_id)
                .ok_or_else(|| DagExecutionError::NodeNotFound(*node_id))?;

            // Get dependency node IDs
            let deps: Vec<Uuid> = graph
                .neighbors_directed(*node_id, Direction::Incoming)
                .collect();

            let node_sender = node_senders.get(node_id).cloned()
                .ok_or_else(|| DagExecutionError::InternalError("Channel not found".to_string()))?;

            let dep_receivers: Vec<(&Uuid, mpsc::Receiver<RecordBatch>)> = deps.iter()
                .filter_map(|dep_id| {
                    node_receivers.remove(dep_id).map(|rx| (dep_id, rx))
                })
                .collect();

            let executor_clone = executor.clone();
            let node_clone = node.clone();

            // Spawn async task for node execution
            tasks.push(tokio::spawn(async move {
                let node_span = tracing::info_span!("execute_node", node_id = %node_clone.id, node_name = node_clone.name.as_str());
                let _node_guard = node_span.enter();

                // Collect input from dependencies (if any)
                let input_data = if dep_receivers.is_empty() {
                    None
                } else {
                    // Merge inputs from multiple dependencies
                    executor_clone.merge_dependencies(dep_receivers).await?
                };

                // Execute node
                let start_time = std::time::Instant::now();
                let output = executor_clone.execute_node(&node_clone, input_data).await?;
                let execution_time = start_time.elapsed().as_millis() as u64;

                // Record metrics
                node_span.record("execution_time_ms", execution_time);
                node_span.record("output_rows", output.num_rows());
                node_span.record("output_columns", output.num_columns());

                // Send output to channel (for dependent nodes)
                node_sender.send(output).await
                    .map_err(|e| DagExecutionError::InternalError(format!("Failed to send output: {}", e)))?;

                Ok(execution_time)
            }));
        }

        // 6. Await all tasks and collect results
        let results = futures::future::join_all(tasks).await;
        let mut total_time_ms = 0;
        let mut nodes_executed = 0;

        for result in results {
            match result {
                Ok(Ok(execution_time)) => {
                    total_time_ms += execution_time;
                    nodes_executed += 1;
                }
                Ok(Err(e)) => return Err(e),
                Err(e) => return Err(DagExecutionError::InternalError(format!("Task panic: {}", e))),
            }
        }

        // 7. Get final result from output node
        let output_node = dag.nodes.iter()
            .find(|n| matches!(n.node_type, NodeType::Output { .. }))
            .ok_or(DagExecutionError::NoOutputNode)?;

        // Final output should be in the output node's receiver
        let final_receiver = node_receivers.remove(&output_node.id)
            .ok_or_else(|| DagExecutionError::OutputNotGenerated)?;

        // Receive final batch
        let final_result = final_receiver.recv().await
            .ok_or_else(|| DagExecutionError::OutputNotGenerated)?;

        // Record execution metrics
        execution_span.record("total_time_ms", total_time_ms);
        execution_span.record("nodes_executed", nodes_executed);

        Ok(ExecutionResult {
            data: final_result,
            row_count: final_result.num_rows(),
            execution_stats: ExecutionStats {
                nodes_executed,
                total_time_ms,
            },
        })
    }

    /// Execute a single node with type safety
    async fn execute_node(
        &self,
        node: &DagNode,
        input_data: Option<RecordBatch>,
    ) -> Result<RecordBatch, DagExecutionError> {
        match &node.node_type {
            NodeType::Ingestion { source_type, filters, output_schema } => {
                // Validate input is None (ingestion nodes are sources)
                if input_data.is_some() {
                    return Err(DagExecutionError::InvalidInput(
                        "Ingestion nodes cannot have input data".to_string()
                    ));
                }

                // Execute ingestion
                let result = self.execute_ingestion(source_type, filters).await?;

                // Validate output schema matches expected
                output_schema.validate_schema(result.schema())?;

                Ok(result)
            }

            NodeType::Operator { operator_id, operator_version, parameters, input_schema, output_schema } => {
                // Validate input data
                let input = input_data.ok_or_else(|| DagExecutionError::MissingInput(
                    format!("Operator {} requires input data", operator_id)
                ))?;

                // Validate input schema matches operator's expected input
                input_schema.validate_schema(input.schema())?;

                // Execute operator
                let result = self.execute_operator(
                    operator_id,
                    operator_version,
                    parameters,
                    input,
                ).await?;

                // Validate output schema matches expected
                output_schema.validate_schema(result.schema())?;

                Ok(result)
            }

            NodeType::Output { output_type, input_schema } => {
                // Validate input data
                let input = input_data.ok_or_else(|| DagExecutionError::MissingInput(
                    "Output node requires input data".to_string()
                ))?;

                // Validate input schema
                input_schema.validate_schema(input.schema())?;

                // Execute output operation
                self.execute_output(output_type, input).await?;

                // Return input data (output nodes don't transform)
                Ok(input)
            }
        }
    }

    /// Execute ingestion node - fetch data from storage
    async fn execute_ingestion(
        &self,
        source_type: &IngestionSourceType,
        filters: &IngestionFilters,
    ) -> Result<RecordBatch, DagExecutionError> {
        match source_type {
            IngestionSourceType::WellLogs { project_id, well_ids, curve_names, depth_range } => {
                // Use OpenDAL to fetch Parquet files
                let mut batches = Vec::new();

                for well_id in well_ids {
                    // Build storage path using project-data-layout
                    let log_path = self.storage_manager
                        .get_well_log_path(project_id, well_id, "composite")?;

                    // Read Parquet file using OpenDAL
                    let parquet_data = self.storage_manager
                        .read_parquet_file(&log_path).await?;

                    // Convert to RecordBatch
                    let batch = self.parquet_to_record_batch(parquet_data)?;

                    // Apply filters
                    let filtered = self.apply_filters(batch, filters)?;

                    batches.push(filtered);
                }

                // Combine batches from multiple wells
                let combined = self.combine_batches(batches)?;

                // Project only requested curves
                let projected = self.project_curves(combined, curve_names)?;

                Ok(projected)
            }

            IngestionSourceType::PostgresTable { table_name, filters: table_filters } => {
                // Use UnifiedDatabaseService to fetch table data
                let unified_service = UnifiedDatabaseService::get_instance()?;
                let data = unified_service.get_table_data(table_name).await?;

                // Convert JSON to RecordBatch
                self.json_to_record_batch(data)
            }

            // ... other source types
            _ => Err(DagExecutionError::UnsupportedSourceType),
        }
    }

    /// Execute operator node - apply transformation
    async fn execute_operator(
        &self,
        operator_id: &str,
        operator_version: &str,
        parameters: &HashMap<String, serde_json::Value>,
        input: RecordBatch,
    ) -> Result<RecordBatch, DagExecutionError> {
        // Lookup operator in registry
        let operator = self.operator_registry
            .get_operator(operator_id, operator_version)
            .ok_or_else(|| DagExecutionError::OperatorNotFound(
                format!("{}:{}", operator_id, operator_version)
            ))?;

        // Execute operator with input data
        match operator.implementation_type.as_str() {
            "native_rust" => {
                self.execute_native_operator(operator, parameters, input).await
            }
            "udf" => {
                self.execute_udf_operator(operator, parameters, input).await
            }
            "wasm" => {
                self.execute_wasm_operator(operator, parameters, input).await
            }
            _ => Err(DagExecutionError::UnsupportedOperatorType),
        }
    }
}
```

### **4.2 Operator Registry**

**From MVP.md**: 100+ built-in operators across subsurface, signal, wells, mining, arrays, and quality categories.

**✅ Enhanced with `schemars` for JSON Schema generation**

```rust
// crates/dags/operators/src/registry.rs

use arrow::record_batch::RecordBatch;
use schemars::{JsonSchema, schema_for};
use std::collections::HashMap;

pub struct OperatorRegistry {
    operators: HashMap<String, Vec<OperatorDefinition>>,
    db_pool: Option<sqlx::PgPool>, // Optional: Load custom operators from database
}

impl OperatorRegistry {
    /// Initialize registry with built-in operators (from MVP.md operator toolbox)
    pub fn new_with_builtins() -> Self {
        let mut registry = Self {
            operators: HashMap::new(),
            db_pool: None,
        };

        // Register 100+ built-in operators (from MVP.md)
        // Categories: subsurface, signal, wells, mining, arrays, quality
        registry.register_builtin_operators();

        registry
    }

    /// Register all built-in operators (from MVP.md)
    fn register_builtin_operators(&mut self) {
        // Subsurface operators (from MVP.md Section 3.B)
        self.register(ShaleVolumeOperator::new(ShaleVolumeMethod::Larionov));
        self.register(ShaleVolumeOperator::new(ShaleVolumeMethod::Clavier));
        self.register(ShaleVolumeOperator::new(ShaleVolumeMethod::Steiber));
        self.register(PorosityOperator::new(PorosityMethod::NeutronDensity));
        self.register(PorosityOperator::new(PorosityMethod::Sonic));
        self.register(PorosityOperator::new(PorosityMethod::Density));
        self.register(SaturationOperator::new(SaturationMethod::Archie));
        self.register(SaturationOperator::new(SaturationMethod::Indonesian));
        self.register(SaturationOperator::new(SaturationMethod::WaxmanSmits));
        // ... more subsurface operators

        // Signal processing operators (from MVP.md)
        self.register(FilterOperator::new(FilterType::MovingAverage));
        self.register(FilterOperator::new(FilterType::Median));
        self.register(FilterOperator::new(FilterType::SavitzkyGolay));
        self.register(DespikeOperator::new());
        self.register(NormalizeOperator::new(NormalizeMethod::ZScore));
        self.register(NormalizeOperator::new(NormalizeMethod::MinMax));
        // ... more signal processing operators

        // Wells, mining, arrays, quality operators
        // ... (from MVP.md operator catalog)
    }

    /// Register a built-in operator
    pub fn register(&mut self, operator: OperatorDefinition) {
        self.operators
            .entry(operator.operator_id.clone())
            .or_insert_with(Vec::new)
            .push(operator);
    }

    /// Get operator by ID and version (from database or built-in)
    pub async fn get_operator(
        &self,
        operator_id: &str,
        version: &str,
    ) -> Result<Option<OperatorDefinition>, OperatorRegistryError> {
        // First check built-in operators
        if let Some(builtin) = self.operators
            .get(operator_id)?
            .iter()
            .find(|op| op.version == version)
            .cloned()
        {
            return Ok(Some(builtin));
        }

        // Then check database (custom operators from operator_registry table)
        if let Some(pool) = &self.db_pool {
            return self.load_operator_from_db(operator_id, version).await;
        }

        Ok(None)
    }

    /// List all operators for a category
    pub fn list_operators_by_category(
        &self,
        category: &str,
    ) -> Vec<&OperatorDefinition> {
        self.operators
            .values()
            .flatten()
            .filter(|op| op.category == category)
            .collect()
    }

    /// Load custom operator from database
    async fn load_operator_from_db(
        &self,
        operator_id: &str,
        version: &str,
    ) -> Result<Option<OperatorDefinition>, OperatorRegistryError> {
        // Load from operator_registry table
        todo!("Implement database operator loading")
    }
}

pub struct OperatorDefinition {
    pub operator_id: String,
    pub operator_name: String,
    pub category: String,
    pub version: String,
    pub input_schema: SchemaDefinition,
    pub output_schema: SchemaDefinition,
    pub parameters_schema: serde_json::Value, // JSON Schema (generated via schemars)
    pub implementation_type: String,
    pub executor: Box<dyn OperatorExecutor>,
}

impl OperatorDefinition {
    /// Generate JSON Schema for operator parameters using schemars
    /// This enables auto-generation of UI forms and type-safe validation
    pub fn generate_parameters_schema<T: JsonSchema>() -> serde_json::Value {
        schema_for!(T).schema.into()
    }
}

#[async_trait]
pub trait OperatorExecutor: Send + Sync {
    async fn execute(
        &self,
        input: RecordBatch,
        parameters: &HashMap<String, serde_json::Value>,
    ) -> Result<RecordBatch, OperatorError>;
}

// Example: Shale Volume Operator with JSON Schema generation
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ShaleVolumeParams {
    #[schemars(description = "Gamma ray curve column name")]
    gr_column: String,

    #[schemars(description = "Minimum gamma ray value for clean sand")]
    gr_min: f64,

    #[schemars(description = "Maximum gamma ray value for shale")]
    gr_max: f64,
}

pub struct ShaleVolumeOperator {
    pub method: ShaleVolumeMethod,
}

impl ShaleVolumeOperator {
    /// Generate JSON Schema for parameters (used by UI for form generation)
    pub fn parameters_schema() -> serde_json::Value {
        OperatorDefinition::generate_parameters_schema::<ShaleVolumeParams>()
    }
}

#[async_trait]
impl OperatorExecutor for ShaleVolumeOperator {
    async fn execute(
        &self,
        input: RecordBatch,
        parameters: &HashMap<String, serde_json::Value>,
    ) -> Result<RecordBatch, OperatorError> {
        // Extract GR column from input
        let gr_col = input.column_by_name("gr")
            .ok_or(OperatorError::MissingColumn("gr".to_string()))?;

        // Apply shale volume calculation
        let vsh = match self.method {
            ShaleVolumeMethod::Larionov => {
                calculate_larionov_vsh(gr_col, parameters)?
            }
            ShaleVolumeMethod::Clavier => {
                calculate_clavier_vsh(gr_col, parameters)?
            }
        };

        // Create new RecordBatch with added VSh column
        let mut new_columns = input.columns().to_vec();
        new_columns.push(Arc::new(vsh));

        let new_schema = input.schema().as_ref().try_with_column(
            Field::new("vsh", DataType::Float64, true)
        )?;

        Ok(RecordBatch::try_new(new_schema, new_columns)?)
    }
}
```

---

## 🔄 **Phase 5: Integration with Existing Storage Layer**

### **5.1 OpenDAL Integration for Ingestion Nodes**

```rust
// crates/dags/executor/src/ingestion/well_logs_ingestion.rs

use opendal_storage_adapter::EnhancedOpenDALStorageAdapter;
use project_data_layout::ProjectDataLayoutManager;

pub struct WellLogsIngestionExecutor {
    storage_adapter: Arc<EnhancedOpenDALStorageAdapter>,
    layout_manager: ProjectDataLayoutManager,
    query_engine: Arc<QueryEngine>,
}

impl WellLogsIngestionExecutor {
    /// Execute well logs ingestion node
    pub async fn execute(
        &self,
        project_id: &str,
        well_ids: &[String],
        curve_names: &[String],
        depth_range: Option<(f64, f64)>,
    ) -> Result<RecordBatch, IngestionError> {
        let mut all_batches = Vec::new();

        for well_id in well_ids {
            // Build storage path using project-data-layout
            let log_path = self.layout_manager
                .well_log_path(well_id, "composite");

            // Read Parquet file using OpenDAL (with caching)
            let parquet_bytes = self.storage_adapter
                .read(&log_path)
                .await
                .map_err(IngestionError::Storage)?;

            // Convert Parquet to RecordBatch using DuckDB
            let batch = self.parquet_to_record_batch(parquet_bytes)?;

            // Add well_id column
            let batch_with_well_id = self.add_well_id_column(batch, well_id)?;

            // Apply depth filter if specified
            let filtered = if let Some((min_depth, max_depth)) = depth_range {
                self.filter_depth_range(batch_with_well_id, min_depth, max_depth)?
            } else {
                batch_with_well_id
            };

            // Project only requested curves
            let projected = self.project_columns(filtered, curve_names)?;

            all_batches.push(projected);
        }

        // Combine batches from all wells
        Ok(self.combine_batches(all_batches)?)
    }

    fn parquet_to_record_batch(
        &self,
        parquet_bytes: bytes::Bytes,
    ) -> Result<RecordBatch, IngestionError> {
        // Use DuckDB or Arrow readers to convert Parquet to RecordBatch
        // This leverages existing parquet-log-query-engine capabilities
        todo!("Implement Parquet to RecordBatch conversion")
    }
}
```

### **5.2 DataFusion Session Context Setup**

```rust
// crates/dags/executor/src/executor.rs

use datafusion::prelude::*;

impl DagExecutor {
    /// Create DataFusion session context with OpenDAL integration
    async fn create_session_context(
        &self,
        project_id: &str,
    ) -> Result<SessionContext, DagExecutionError> {
        let ctx = SessionContext::new();

        // Register OpenDAL object store for S3/MinIO access
        let object_store = self.storage_adapter.create_object_store().await?;
        ctx.runtime_env().register_object_store(
            "s3",
            "project-bucket",
            Arc::new(object_store),
        );

        // Register project-specific Parquet files as tables
        // This allows ingestion nodes to reference data as tables
        self.register_project_tables(&ctx, project_id).await?;

        Ok(ctx)
    }

    async fn register_project_tables(
        &self,
        ctx: &SessionContext,
        project_id: &str,
    ) -> Result<(), DagExecutionError> {
        // List all wells for this project
        let wells = self.storage_manager.list_wells(project_id).await?;

        for well in wells {
            // Register each well's composite log file as a table
            let table_name = format!("well_{}", well.id);
            let s3_path = format!("s3://project-{}/wells/{}/logs_composite.parquet",
                                 project_id, well.id);

            ctx.sql(&format!(
                "CREATE EXTERNAL TABLE {} STORED AS PARQUET LOCATION '{}'",
                table_name, s3_path
            )).await?;
        }

        Ok(())
    }
}
```

---

## 📚 **Integration with Related Architecture Documents**

### **Review of Older Documentation**

The following documents were reviewed and the best ideas integrated:

#### **✅ Integrated from `data-extraction-flow.md`**:

- Frontend component structure (well-selector, curve-selector, depth-range-picker) ✓
- Integration with OpenDAL storage adapter ✓
- Query optimization approach ✓
- **NOTE**: Standalone `DataExtractionQuery` approach replaced with DAG node system

#### **✅ Integrated from `data-extraction.md`**:

- **Well-by-well wide format schema** (current implementation) ✓
- Per-well Parquet storage structure: `project-{id}/wells/{well-id}/logs_{log-type}.parquet` ✓
- Cross-well normalization on-demand approach ✓
- Query optimization strategies for well-by-well storage ✓
- **NOTE**: Mentions of `storage-resolver` and `s3-config` should be replaced with OpenDAL

#### **✅ Integrated from `dataframe_storage_implementation_plan.md`**:

- **Content-addressed caching** using `content_hash` ✓
- **Lineage tracking** concepts (tracking data transformations) ✓
- **Run metadata** (execution history, metrics, reproducibility) ✓
- JSON DAG structure with versioning ✓
- **NOTE**: Iceberg control tables concept replaced with PostgreSQL tables

#### **✅ Integrated from `rust_dag_plan.md`**:

- Overall DAG architecture analogies (factory assembly line) ✓
- Arrow RecordBatch as data transport format ✓
- Content-addressed caching strategy ✓
- WASM support for custom operators (future) ✓

#### **✅ Integrated from `datafusion_planning_sketch.md`**:

- **User vs Backend separation**: Users work in DAG space, backend handles logical/physical plans ✓
- Arrow RecordBatch streaming between nodes ✓
- Schema compatibility checking at connection time ✓
- Materialization/caching hints per node ✓

### **Documents to Update or Archive**

#### **⚠️ `data-extraction-flow.md`** - **UPDATE RECOMMENDED**

**Status**: Partially outdated  
**Action**: Update to reference DAG ingestion nodes instead of standalone extraction query

**Key Changes Needed**:

- Replace `DataExtractionQuery` with `IngestionNode` approach
- Update to use OpenDAL instead of storage-resolver
- Connect to DAG executor instead of standalone extraction engine

#### **⚠️ `data-extraction.md`** - **UPDATE RECOMMENDED**

**Status**: Partially outdated  
**Action**: Update references to use OpenDAL instead of legacy crates

**Key Changes Needed**:

- Replace `storage-resolver` references with `EnhancedOpenDALStorageAdapter`
- Replace `s3-config` references with OpenDAL configuration
- Update to show integration with DAG executor rather than standalone crate
- Keep: Well-by-well storage schema concepts (still valid)

#### **✅ `dataframe_storage_implementation_plan.md`** - **ARCHIVE OR UPDATE**

**Status**: Conceptually relevant but architecture mismatch  
**Action**: Archive as reference or update to PostgreSQL-based control tables

**Key Differences**:

- Uses Iceberg control tables → We use PostgreSQL tables
- Uses Substrait plans → Future optimization (not MVP)
- **Keep for reference**: Content-hash caching, lineage concepts, run metadata structure

#### **✅ `rust_dag_plan.md`** - **KEEP AS REFERENCE**

**Status**: High-level concepts still valid  
**Action**: Keep as high-level reference, detailed implementation in `dag-execution.md`

**Still Relevant**:

- Overall architecture analogies
- Arrow/Parquet data flow concepts
- Caching and lineage strategies

#### **✅ `datafusion_planning_sketch.md`** - **KEEP AS REFERENCE**

**Status**: Still relevant  
**Action**: Keep as reference for DataFusion integration details

**Still Relevant**:

- User vs backend separation
- Logical plan vs physical plan concepts
- Schema validation approaches

---

## ✅ **Phase 6: Step-by-Step Implementation Plan**

### **Week 1-2: Foundation & Type System**

#### **Step 1.1: Create Core Node Types** ✅ **COMPLETED**

```bash
# Create new crate structure
mkdir -p crates/dags/{core,executor,operators,storage}

# Implement core types
# crates/dags/core/src/lib.rs
# - NodeType enum (Ingestion, Operator, Output)
# - SchemaDefinition struct
# - DagDefinition struct
# - Schema validation logic
```

**Deliverables:**

- [x] ✅ Node type system with type-safe schemas
- [x] ✅ Schema validation functions
- [x] ✅ Basic DAG structure definitions
- [x] ✅ VersionStrategy enum for operator version management
- [x] ✅ Optional schema support for reference-based operator storage

#### **Step 1.2: Database Schema Migration** ✅ **COMPLETED**

```sql
-- Create migration file
-- db/init/02-pipeline-schema.sql
-- - pipelines table
-- - pipeline_nodes table
-- - pipeline_edges table
-- - pipeline_executions table
-- - operator_registry table (with versioning support)
```

**Deliverables:**

- [x] ✅ PostgreSQL tables for DAG storage
- [x] ✅ Indexes for performance
- [x] ✅ Foreign key constraints
- [x] ✅ **Operator versioning support** - Multiple versions per operator, `is_latest` flag
- [x] ✅ **Migration support** - `deprecated_at`, `replacement_version`, `schema_compatible` columns
- [x] ✅ **Composite unique constraint** - `(operator_id, version)` ensures version uniqueness

#### **Step 1.3: Rust Best Practices & Performance Optimizations** ✅ **COMPLETED**

**Updated `.cursor/rules/global.mdc`** with comprehensive Rust best practices covering:

- Memory & performance patterns (reduce allocations, use `Arc`, `bytes::Bytes`)
- Async patterns (`tokio`, `async_trait`, `tokio::sync::RwLock`)
- Error handling (`thiserror::Error` with `#[from]` attributes)
- Storage layer patterns (OpenDAL, caching with `moka`)
- Database patterns (`tokio_postgres`, `postgres_types::Json`)
- Arrow/DataFusion patterns (`Arc<Schema>`, `Arc<RecordBatch>`)
- Type safety (`uuid::Uuid`, `chrono::DateTime`)
- Concurrency (`Arc`, bounded channels)

**Performance Optimizations Applied:**

1. **`crates/dags/core/src/schema.rs`**:
   - ✅ Optimized `has_column()`: Changed from `contains(&column_name.to_string())` to `iter().any(|c| c == column_name)` (eliminates unnecessary `String` allocation)
   - ✅ Optimized `all_columns()`: Added capacity hint `HashSet::with_capacity()` before extending

2. **`crates/dags/validator/src/lib.rs`**:
   - ✅ Optimized `build_graph()`: Added capacity hints to `Graph::with_capacity()` and `HashMap::with_capacity()`
   - ✅ Improved return type: Changed from `Vec<(Uuid, NodeIndex)>` to `HashMap<Uuid, NodeIndex>` (more efficient lookup, avoids unnecessary conversion)
   - ✅ Fixed imports: Moved `HashMap` to top-level, removed unused `DagNode` import

3. **`crates/dags/storage/src/pipeline_storage.rs`**:
   - ✅ Added capacity hint: `Vec::with_capacity(node_count)` for `node_schemas`
   - ✅ Fixed unused `mut` warning on transaction

**Deliverables:**

- [x] ✅ Comprehensive Rust best practices documented in `global.mdc`
- [x] ✅ All Phase 1 crates optimized for memory efficiency
- [x] ✅ All crates compile successfully with no warnings or errors
- [x] ✅ Code follows patterns from `storage-layer.md` and `opendal-integration.md`

### **Week 3-4: Ingestion Nodes**

#### **Step 2.1: Well Logs Ingestion Node** ✅ **COMPLETED**

```rust
// crates/dags/executor/src/ingestion/well_logs.rs

pub struct WellLogsIngestionExecutor {
    storage_adapter: Arc<EnhancedOpenDALStorageAdapter>,
    layout_manager: ProjectDataLayoutManager,
}

impl WellLogsIngestionExecutor {
    pub async fn execute(
        &self,
        source_type: &IngestionSourceType,
        filters: &IngestionFilters,
    ) -> Result<RecordBatch> {
        // 1. Build storage paths for selected wells ✅
        // 2. Use OpenDAL to read Parquet files ✅
        // 3. Apply filters (depth range, curve selection) ✅
        // 4. Combine data from multiple wells ✅
        // 5. Return RecordBatch with expected schema ✅
    }
}
```

**Integration Points:**

- ✅ Uses `EnhancedOpenDALStorageAdapter` for storage access (with caching)
- ✅ Uses `ProjectDataLayoutManager` for path construction
- ✅ Uses DataFusion for Parquet reading (via temporary files)
- ✅ Integrates with `dag-core` types (`IngestionSourceType`, `IngestionFilters`)
- ⏳ Frontend: Integrates with `postgres-wells-state.svelte.ts` for well selections (pending)
- ⏳ Frontend: Integrates with `postgres-curve-metadata-state.svelte.ts` for curve selections (pending)

**Deliverables:**

- ✅ Well logs ingestion node implementation (`WellLogsIngestionExecutor`)
- ✅ OpenDAL integration for Parquet reading (`storage_adapter.download()`)
- ✅ DataFusion-based Parquet to RecordBatch conversion
- ✅ Depth range filtering (`filter_depth_range()`)
- ✅ Curve projection (`project_curves()`)
- ✅ Multi-well data combination (`combine_batches()`)
- ✅ Well ID column addition (`add_well_id_column()`)

#### **Step 2.2: Frontend Ingestion Node Builder** ✅ Priority

**Component Structure** (from `data-extraction-flow.md`):

```typescript
// src/lib/components/pipelines/nodes/ingestion-node-builder.svelte

// Main component that orchestrates sub-components
<script lang="ts">
  import WellSelector from './well-selector.svelte';
  import CurveSelector from './curve-selector.svelte';
  import DepthRangePicker from './depth-range-picker.svelte';
  import { getPostgresWellsState } from '$lib/state/postgres/postgres-wells-state.svelte';
  import { getPostgresCurveMetadataState } from '$lib/state/postgres/postgres-curve-metadata-state.svelte';

  const wellsState = getPostgresWellsState();
  const curvesState = getPostgresCurveMetadataState();

  let selectedWellIds = $state<string[]>([]);
  let selectedCurveNames = $state<string[]>([]);
  let depthRange = $state<{ min: number; max: number } | null>(null);

  // Infer output schema based on selections (real-time)
  let outputSchema = $derived.by(() => {
    if (selectedWellIds.length === 0 || selectedCurveNames.length === 0) {
      return null;
    }

    return {
      schemaId: 'well_logs_standard',
      requiredColumns: ['well_id', 'depth'],
      optionalColumns: selectedCurveNames,
      arrowSchemaJson: JSON.stringify({
        fields: [
          { name: 'well_id', type: 'utf8', nullable: false },
          { name: 'depth', type: 'float64', nullable: false },
          ...selectedCurveNames.map(name => ({
            name,
            type: 'float64',
            nullable: true,
          })),
        ],
      }),
    };
  });
</script>

<div class="ingestion-node-builder">
  <WellSelector
    wells={wellsState.wells}
    selected={selectedWellIds}
    on:select={(e) => selectedWellIds = e.detail}
  />

  <CurveSelector
    curves={curvesState.curveMetadata}
    selected={selectedCurveNames}
    on:select={(e) => selectedCurveNames = e.detail}
  />

  <DepthRangePicker
    value={depthRange}
    on:change={(e) => depthRange = e.detail}
  />

  {#if outputSchema}
    <SchemaPreview schema={outputSchema} />
  {/if}
</div>
```

**Sub-components** (from `data-extraction-flow.md`):

- `well-selector.svelte`: Multi-select well picker using AG-Grid or shadcn components
- `curve-selector.svelte`: Multi-select curve picker (filtered by available curves)
- `depth-range-picker.svelte`: Min/max depth inputs with validation

**Deliverables:**

- ✅ UI component for building ingestion nodes (`content-dag-ingestion-node-editor.svelte`)
- ✅ Integration with Postgres global states (wells, curves, projects)
- ✅ Real-time schema inference (as user selects wells/curves)
- ✅ Frontend components match structure from `data-extraction-flow.md`
- ✅ Well selector component (`well-selector.svelte`) with multi-select
- ✅ Curve selector component (`curve-selector.svelte`) with multi-select
- ✅ Depth range picker component (`depth-range-picker.svelte`)
- ✅ Schema preview component (`schema-preview.svelte`)
- ✅ Node editor sidebar (`node-editor-sidebar.svelte`) integrated into DAG editor
- ✅ Node creation functionality (`dag-node-creator.ts`) - creates DagNode from config
- ✅ Automatic node addition to graph with Dagre layout

### **Week 5-6: Operator Nodes**

#### **Step 3.1: Operator Registry & Built-in Operators** ✅ Priority

```rust
// crates/dags/operators/src/registry.rs

pub fn register_builtin_operators() -> OperatorRegistry {
    let mut registry = OperatorRegistry::new();

    // Subsurface operators
    registry.register(ShaleVolumeOperator::new(ShaleVolumeMethod::Larionov));
    registry.register(PorosityOperator::new(PorosityMethod::NeutronDensity));
    registry.register(SaturationOperator::new(SaturationMethod::Archie));

    // Signal processing operators
    registry.register(DespikeOperator::new());
    registry.register(FilterOperator::new(FilterType::MovingAverage));
    registry.register(NormalizeOperator::new(NormalizeMethod::ZScore));

    registry
}
```

**Deliverables:**

- ✅ Operator registry implementation
- ✅ 20+ built-in operators (focus on subsurface and signal processing)
- ✅ Schema definitions for each operator
- ✅ Operator execution traits

#### **Step 3.2: Frontend Operator Node Builder** ✅ Priority

```typescript
// src/lib/components/pipelines/nodes/operator-node-builder.svelte

// Component that:
// 1. Shows operator catalog (from operator_registry table)
// 2. Shows operator parameters form (from parameters_schema)
// 3. Validates operator can connect to upstream node (schema compatibility)
// 4. Generates OperatorNodeConfig
```

**Deliverables:**

- ✅ UI component for building operator nodes
- ✅ Operator catalog browser
- ✅ Real-time schema compatibility checking

### **Week 7-8: Execution Engine**

#### **Step 4.1: DAG Compiler & Validator** ✅ Priority

```rust
// crates/dags/executor/src/compiler.rs

pub struct DagCompiler;

impl DagCompiler {
    /// Validate DAG structure and type safety
    pub fn validate(&self, dag: &DagDefinition) -> Result<(), CompilationError> {
        // 1. Check for cycles (not a DAG)
        // 2. Validate all node connections are type-compatible
        // 3. Ensure there's exactly one output node
        // 4. Check all operator IDs exist in registry
        // 5. Validate parameter schemas match operators
    }

    /// Optimize DAG for execution
    pub fn optimize(&self, dag: &DagDefinition) -> DagDefinition {
        // 1. Fuse compatible operators (e.g., multiple filters)
        // 2. Push filters down to ingestion nodes
        // 3. Eliminate unnecessary projections
        // 4. Reorder nodes for better cache locality
    }
}
```

**Deliverables:**

- ✅ DAG validation logic
- ✅ Type safety checking
- ✅ DAG optimization passes

#### **Step 4.2: DAG Executor** ✅ Priority

```rust
// crates/dags/executor/src/executor.rs

impl DagExecutor {
    pub async fn execute(&self, dag: &DagDefinition) -> Result<ExecutionResult> {
        // 1. Topological sort nodes
        // 2. Execute nodes in order
        // 3. Pass data between nodes as RecordBatch
        // 4. Cache intermediate results
        // 5. Handle errors gracefully
    }
}
```

**Deliverables:**

- ✅ Complete DAG execution engine
- ✅ Node dependency resolution
- ✅ Caching layer for intermediate results
- ✅ Error handling and recovery

### **Week 9-10: Frontend Integration**

#### **Step 5.1: Visual DAG Editor** ✅ Priority

**✅ Using Cytoscape.js for production-ready graph visualization**

**Why Cytoscape.js over alternatives?** (See `dag-editor-library-comparison.md` for detailed analysis)

- ✅ **Library, not platform** - Node-RED is a complete execution engine (would conflict with your Rust backend)
- ✅ **Best Svelte integration** - `svelte-cytoscape` wrapper, no React required
- ✅ **Battle-tested** - Handles 1000+ nodes efficiently with WebGL rendering
- ✅ **Fits your architecture** - Pure visualization, executes DAGs in Rust backend (no dual engines)
- ✅ **Balanced features** - Good balance of performance, built-in layouts, and ease of use
- ✅ **Flexible** - Can add ELK.js for advanced layouts if needed

**Alternatives considered:**

- **Sigma.js**: Excellent performance for very large DAGs (5,000+ nodes) but requires more manual implementation work
- **React Flow**: Excellent UX but requires React micro-frontend (more complexity)
- **Node-RED**: Complete platform, not a library (would conflict with Rust execution engine)
- **Svelvet**: Too simple, limited features for complex DAGs

```typescript
// src/lib/components/pipelines/dag-editor.svelte

<script lang="ts">
  import { SvelteFlow, Background, Controls, MiniMap, Panel } from '@xyflow/svelte';
  import '@xyflow/svelte/dist/style.css';
  import dagre from '@dagrejs/dagre';
  import type { DagDefinition, DagNode } from '$lib/types/dag';
  import { invoke } from '@tauri-apps/api/core';
  import { dagDefinitionToSvelteFlow } from '$lib/services/dag-converter';

  let dag: DagDefinition = $state({...});
  let { nodes, edges } = dagDefinitionToSvelteFlow(dag);

  // Auto-layout using Dagre
  function applyLayout() {
    const dagreGraph = new dagre.graphlib.Graph();
    dagreGraph.setDefaultEdgeLabel(() => ({}));
    dagreGraph.setGraph({ rankdir: 'TB' }); // Top-to-bottom layout

    nodes.forEach((node) => {
      dagreGraph.setNode(node.id, { width: 200, height: 50 });
    });

    edges.forEach((edge) => {
      dagreGraph.setEdge(edge.source, edge.target);
    });

    dagre.layout(dagreGraph);

    nodes = nodes.map((node) => {
      const nodeWithPosition = dagreGraph.node(node.id);
      return {
        ...node,
        position: {
          x: nodeWithPosition.x - 100,
          y: nodeWithPosition.y - 25,
        },
      };
    });
  }

  function handleNodeClick(event: CustomEvent) {
    const nodeId = event.detail.node.id;
    // Open node configuration panel
  }

  function handleConnect(event: CustomEvent) {
    // Validate schema compatibility when user connects nodes
    const { source, target } = event.detail;
    validateConnection(source, target);
  }
</script>

<div class="dag-editor">
  <div class="editor-toolbar">
    <button on:click={addIngestionNode}>Add Ingestion</button>
    <button on:click={addOperatorNode}>Add Operator</button>
    <button on:click={addOutputNode}>Add Output</button>
    <button on:click={applyLayout}>Auto Layout</button>
    <button on:click={saveDag}>Save DAG</button>
  </div>

  <SvelteFlow
    nodes={nodes}
    edges={edges}
    onnodeclick={handleNodeClick}
    onconnect={handleConnect}
    isValidConnection={isValidConnection}
    class="graph-container"
  >
    <Background />
    <Controls />
    <MiniMap />
    <Panel position="top-right">
      <button on:click={applyLayout}>Layout</button>
    </Panel>
  </SvelteFlow>
</div>
```

**Package Installation**:

```bash
pnpm add @xyflow/svelte @dagrejs/dagre
```

**Why Svelte Flow**:

- ✅ Native Svelte integration (no wrapper needed)
- ✅ Modern API with TypeScript support
- ✅ Built-in Dagre layout integration
- ✅ Connection validation support (`isValidConnection`)
- ✅ Handles 1000+ nodes efficiently
- ✅ Active development and community support

**Deliverables:**

- ✅ Visual DAG editor component using Svelte Flow (`SF-pipeline-flow.svelte`)
- ✅ Node library sidebar
- ✅ Connection validation UI (red/green edges via `isValidConnection`)
- ✅ Save/load DAG functionality
- ✅ Drag-and-drop node creation

#### **Step 5.2: Execution Monitor** ✅ Priority

```typescript
// src/lib/components/pipelines/execution-monitor.svelte

// Component that:
// 1. Shows execution progress
// 2. Displays per-node execution status
// 3. Shows errors if execution fails
// 4. Displays final results
```

**Deliverables:**

- ✅ Real-time execution monitoring
- ✅ Progress indicators
- ✅ Error display
- ✅ Results viewer

---

## 🔒 **Type Safety & Validation Strategy**

### **Schema Compatibility Rules**

**From `datafusion_planning_sketch.md`**: Schema validation happens at multiple levels - user DAG construction, logical plan compilation, and physical plan execution. Users work in "DAG space" while backend handles plan internals.

```rust
impl SchemaDefinition {
    /// Check if this schema is compatible with another (for node connections)
    pub fn validate_compatible(
        &self,
        other: &SchemaDefinition,
    ) -> Result<(), SchemaValidationError> {
        // Rule 1: All required columns in 'other' must exist in 'self'
        for required_col in &other.required_columns {
            if !self.has_column(required_col) {
                return Err(SchemaValidationError::MissingColumn(required_col.clone()));
            }
        }

        // Rule 2: Column types must match for overlapping columns
        for col in &other.required_columns {
            if let Some(self_type) = self.get_column_type(col) {
                if let Some(other_type) = other.get_column_type(col) {
                    if !self_type.is_compatible_with(&other_type) {
                        return Err(SchemaValidationError::TypeMismatch(
                            col.clone(),
                            self_type,
                            other_type,
                        ));
                    }
                }
            }
        }

        Ok(())
    }

    /// Validate schema using actual Arrow schema (at execution time)
    pub fn validate_arrow_schema(
        &self,
        actual_schema: &Schema,
    ) -> Result<(), SchemaValidationError> {
        // Check required columns exist
        for required_col in &self.required_columns {
            actual_schema
                .field_with_name(required_col)
                .map_err(|_| SchemaValidationError::MissingColumn(required_col.clone()))?;
        }

        // Check column types match (from Arrow schema)
        // TODO: Implement type matching logic using Arrow DataType
        Ok(())
    }
}
```

### **Content-Addressed Caching** (from `dataframe_storage_implementation_plan.md`)

To enable caching of intermediate DAG results, compute a content hash:

```rust
// crates/dags/executor/src/cache/content_cache.rs

pub fn compute_content_hash(
    dag: &DagDefinition,
    node_id: &Uuid,
    input_snapshots: &[String], // Input data snapshot IDs
    parameters: &HashMap<String, serde_json::Value>,
) -> String {
    use sha2::{Sha256, Digest};

    let mut hasher = Sha256::new();

    // Hash node configuration
    hasher.update(serde_json::to_string(&dag.nodes.iter().find(|n| n.id == *node_id)).unwrap());

    // Hash input snapshots (ensures cache invalidation on data change)
    hasher.update(serde_json::to_string(input_snapshots).unwrap());

    // Hash parameters
    hasher.update(serde_json::to_string(parameters).unwrap());

    // Hash operator versions (ensures cache invalidation on operator update)
    if let NodeType::Operator { operator_version, .. } = &node.node_type {
        hasher.update(operator_version.as_bytes());
    }

    format!("{:x}", hasher.finalize())
}
```

This enables:

- **Subgraph caching**: Cache results of expensive subgraphs
- **Incremental recomputation**: Only recompute nodes when inputs change
- **Reproducibility**: Same inputs + config → same cache key

### **Frontend Type Checking**

```typescript
// src/lib/components/pipelines/type-checker.ts

export class DagTypeChecker {
  validateNodeConnection(
    sourceNode: DagNode,
    targetNode: DagNode,
  ): ValidationResult {
    // Call backend Tauri command to validate
    return invoke<ValidationResult>("validate_node_connection", {
      sourceNodeId: sourceNode.id,
      targetNodeId: targetNode.id,
    });
  }

  inferSchemaFromSelections(
    sourceType: IngestionSourceType,
    selections: IngestionSelections,
  ): SchemaDefinition {
    // Infer schema based on frontend selections
    // This provides immediate feedback without backend call
  }
}
```

---

## 📦 **Modular Architecture Benefits**

### **1. Type Safety at Multiple Levels**

- **Compile-time**: Rust type system ensures correct node structures
- **Database**: PostgreSQL constraints ensure data integrity
- **Runtime**: Schema validation before node execution
- **Frontend**: Real-time type checking in UI

### **2. Clear Separation of Concerns**

```
crates/dags/
├── core/              # Type definitions, schemas
├── storage/            # Database operations
├── executor/           # Execution engine
├── operators/          # Operator implementations
└── ingestion/          # Ingestion node implementations
```

### **3. Easy Extension Points**

- **New Ingestion Types**: Add to `IngestionSourceType` enum
- **New Operators**: Register in `operator_registry` table
- **New Output Types**: Add to `OutputType` enum
- **Custom Operators**: Users can create and register custom operators

---

## 🎯 **Example: Complete DAG Flow**

### **Frontend User Creates DAG**

**User Interaction Flow** (based on `data-extraction-flow.md` and current architecture):

```typescript
// Step 1: User creates Ingestion Node
// UI Components used (from data-extraction-flow.md):
//   - well-selector.svelte: Multi-select from PostgresWellsState
//   - curve-selector.svelte: Multi-select from PostgresCurveMetadataState
//   - depth-range-picker.svelte: Min/max depth inputs

// User selections:
//   - Wells: [Well A (id: "well-123"), Well B (id: "well-456")]
//   - Curves: [GR, RHOB] (from curve_metadata table)
//   - Depth: 1000-2000m
//
// Frontend infers output schema:
//   {
//     schemaId: "well_logs_standard",
//     requiredColumns: ["well_id", "depth"],
//     optionalColumns: ["gr", "rhob"]
//   }

// Step 2: Creates Operator Node (GR → VShale)
//   - Selects operator: "shale_volume_larionov" from operator_registry
//   - UI validates schema compatibility:
//     * Ingestion output: {well_id, depth, gr, rhob}
//     * Operator input: {well_id, depth, gr, ...}
//     * ✓ Compatible (all required columns present)
//   - User connects node to Ingestion Node
//   - UI validates connection: ✓ Type-safe

// Step 3: Creates Operator Node (Despike VShale)
//   - Selects operator: "despike"
//   - UI validates schema compatibility:
//     * Previous operator output: {well_id, depth, gr, rhob, vsh}
//     * Despike input: {well_id, depth, value (any numeric), ...}
//     * UI suggests: "vsh" column for despike
//     * ✓ Compatible
//   - Connects to VShale Operator Node ✓

// Step 4: Creates Output Node
//   - Output type: "frontend" (returns to UI)
//   - UI validates: compatible with Despike output ✓
//   - DAG is complete and valid
```

### **Backend Execution**

**Execution Flow** (with content-addressed caching from `dataframe_storage_implementation_plan.md`):

```
1. DAG saved to PostgreSQL (pipelines table)
2. User clicks "Execute" → Creates pipeline_executions row (status='pending')
3. Backend loads DAG from database
4. DAG Compiler validates:
   - ✓ No cycles (using petgraph)
   - ✓ All schema connections valid
   - ✓ All operators exist in operator_registry
   - ✓ All input data accessible (wells exist, curves available)

5. Content Hash Computation (for caching):
   - Compute content_hash = SHA256(dag_json + params + input_snapshots)
   - Check cache_index table for existing results
   - If cache hit → Return cached result immediately
   - If cache miss → Continue execution

6. DAG Executor (topological sort):
   - Node 1 (Ingestion):
     * Fetches Well A & B Parquet files from OpenDAL (project-123/wells/well-123/logs_composite.parquet)
     * Uses EnhancedOpenDALStorageAdapter with Moka caching
     * Applies depth filter: WHERE depth BETWEEN 1000.0 AND 2000.0
     * Projects curves: SELECT well_id, depth, gr, rhob
     * Returns: RecordBatch {well_id, depth, gr, rhob}
     * Content hash computed for cache key

   - Node 2 (VShale Operator):
     * Receives: RecordBatch {well_id, depth, gr, rhob}
     * Validates input schema matches operator's expected input ✓
     * Executes: shale_volume_larionov(gr, params)
     * Returns: RecordBatch {well_id, depth, gr, rhob, vsh}
     * Content hash computed for cache key

   - Node 3 (Despike):
     * Receives: RecordBatch {well_id, depth, gr, rhob, vsh}
     * Validates input schema ✓
     * Executes: despike(vsh, threshold=0.1)
     * Returns: RecordBatch {well_id, depth, gr, rhob, vsh}
     * Content hash computed for cache key

   - Node 4 (Output):
     * Receives: RecordBatch {well_id, depth, gr, rhob, vsh}
     * Validates input schema ✓
     * Returns data to frontend (Arrow RecordBatch)

7. Execution Completion:
   - Update pipeline_executions: status='completed', metrics, execution_time
   - Store lineage in pipeline_lineage table
   - Store cache entries in cache_index (if cache enabled)
   - Return results to frontend

8. Frontend Display:
   - Receives Arrow RecordBatch
   - Converts to JSON for AG-Grid (results-table.svelte)
   - Displays in editable table
   - Optionally charts in results-chart.svelte (Plotly/D3)
```

---

## 🚀 **Implementation Checklist**

### **Phase 1: Foundation (Week 1-2)** ✅ **COMPLETED & OPTIMIZED**

- [x] Create `crates/dags/core` with node type system ✅
- [x] Implement `SchemaDefinition` with validation ✅
- [x] Create PostgreSQL migration for pipeline tables ✅
- [x] Implement `PipelineStorage` for database operations ✅
- [x] **Operator versioning support** ✅ - Reference-based storage, VersionStrategy enum
- [x] **Performance optimizations** ✅ - Reduced allocations, added capacity hints, optimized data structures
- [x] **Rust best practices** ✅ - Updated `global.mdc` with comprehensive guidelines
- [x] Create basic Tauri commands (`save_pipeline`, `load_pipeline`) ✅

### **Phase 2: Ingestion Nodes (Week 3-4)** ✅ **BACKEND COMPLETE**

- [x] ✅ Implement `WellLogsIngestionExecutor` (`crates/dags/executor/src/ingestion/well_logs.rs`)
- [x] ✅ Integrate with OpenDAL storage adapter (`EnhancedOpenDALStorageAdapter`)
- [x] ✅ Parquet to RecordBatch conversion using DataFusion
- [x] ✅ Depth range filtering implementation
- [x] ✅ Curve projection (column selection)
- [x] ✅ Multi-well data combination
- [x] ✅ Well ID column addition
- [ ] Create frontend ingestion node builder component
- [ ] Connect to Postgres global states (wells, curves)
- [ ] Test end-to-end: Frontend selection → Backend fetch → RecordBatch

### **Phase 3: Operator Nodes (Week 5-6)** ✅ **ENHANCED WITH JSON SCHEMA**

- [ ] Create `OperatorRegistry` with built-in operators
- [ ] Implement 10+ core operators (subsurface + signal processing) with:
  - ✅ `schemars` derive for parameter structs
  - ✅ Auto-generated JSON Schema for UI forms
- [ ] Create operator execution traits
- [ ] Create frontend operator node builder (uses JSON Schema for form generation)
- [ ] Implement schema compatibility checking

### **Phase 4: Execution Engine (Week 7-8)** ✅ **ENHANCED WITH PRODUCTION PATTERNS**

- [ ] Implement DAG compiler and validator
- [ ] Implement DAG executor with:
  - ✅ petgraph for graph structure and topological sort
  - ✅ tokio bounded channels for backpressure and parallel execution
  - ✅ OpenTelemetry spans for observability
- [ ] Add caching layer for intermediate results
- [ ] Create execution history tracking with OpenTelemetry metrics
- [ ] Test complete DAG execution end-to-end

### **Phase 5: Frontend Integration (Week 9-10)** ✅ **ENHANCED WITH SVELTE FLOW**

- [ ] Create visual DAG editor component using:
  - ✅ Svelte Flow (`@xyflow/svelte`) for graph visualization
  - ✅ Dagre (`@dagrejs/dagre`) for automatic hierarchical layout
  - ✅ Visual schema validation (red/green edges via `isValidConnection`)
- [ ] Implement node library browser
- [ ] Add real-time type checking UI
- [ ] Create execution monitor component with OpenTelemetry metrics
- [ ] Add DAG save/load functionality

---

## 📊 **Success Metrics**

- ✅ **Type Safety**: Zero runtime schema errors (caught at compile/validation time)
- ✅ **Performance**: DAG execution completes in <5s for typical workflows
  - **Caching**: 10-100x improvement for repeated operations (OpenDAL Moka cache)
  - **Filter Pushdown**: Leverage Parquet statistics for depth filtering
- ✅ **Usability**: Users can build pipelines without coding
- ✅ **Extensibility**: New operators can be added without breaking existing DAGs
- ✅ **Reliability**: 99%+ successful DAG executions

---

## 📝 **Summary of Document Review**

### **✅ Integrated Concepts:**

1. **From `data-extraction-flow.md`**:
   - Frontend component structure (well-selector, curve-selector, depth-range-picker)
   - Query optimization approach
   - Results display components

2. **From `data-extraction.md`**:
   - Well-by-well wide-format storage (already implemented ✓)
   - Cross-well normalization on-demand
   - Query optimization strategies
   - Per-well Parquet file structure

3. **From `dataframe_storage_implementation_plan.md`**:
   - Content-addressed caching with `content_hash`
   - Enhanced execution metrics (rows_in, rows_out, bytes_scanned, etc.)
   - Lineage tracking table (`pipeline_lineage`)
   - Reproducibility concepts (input snapshots, engine versions)
   - Observability (OpenTelemetry trace IDs, logs URI)

4. **From `rust_dag_plan.md`**:
   - Overall architecture analogies
   - Arrow RecordBatch as data transport
   - Content-addressed caching strategy

5. **From `datafusion_planning_sketch.md`**:
   - User vs backend separation
   - Arrow RecordBatch streaming
   - Schema validation at multiple levels

### **⚠️ Documents Requiring Updates:**

#### **`data-extraction-flow.md`** - **ACTION: UPDATE**

**Current Status**: Describes standalone `DataExtractionQuery` approach  
**Required Changes**:

- Replace `DataExtractionQuery` concept with DAG `IngestionNode` approach
- Update storage layer references: `storage-resolver` → `EnhancedOpenDALStorageAdapter`
- Update to show integration with DAG executor
- **Keep**: Frontend component structure (well-selector, curve-selector, depth-range-picker) ✓

#### **`data-extraction.md`** - **ACTION: UPDATE**

**Current Status**: Contains outdated storage crate references  
**Required Changes**:

- Replace all `storage-resolver` references with `EnhancedOpenDALStorageAdapter`
- Replace all `s3-config` references with OpenDAL configuration
- Update to reference DAG executor instead of standalone `data-extraction` crate
- **Keep**: Well-by-well storage concepts, query optimization strategies ✓

### **✅ Documents to Keep as Reference:**

#### **`dataframe_storage_implementation_plan.md`** - **KEEP FOR REFERENCE**

**Status**: Conceptually valuable but uses different architecture  
**Why Keep**:

- Content-addressed caching concepts
- Lineage tracking patterns
- Run metadata structure
- **Note**: Uses Iceberg control tables (we use PostgreSQL), Substrait (future optimization)

#### **`rust_dag_plan.md`** - **KEEP AS HIGH-LEVEL REFERENCE**

**Status**: High-level concepts still valid  
**Why Keep**:

- Architecture analogies (factory assembly line)
- Arrow/Parquet data flow concepts
- Overall DAG vision

#### **`datafusion_planning_sketch.md`** - **KEEP AS REFERENCE**

**Status**: Still relevant for DataFusion integration  
**Why Keep**:

- User vs backend separation concepts
- Logical vs physical plan understanding
- Schema validation approaches

### **✅ Current Architecture Alignment:**

- **Storage**: ✅ OpenDAL with wide-format per-well Parquet files
- **Frontend**: ✅ Postgres global states (wells, curves) with Realtime
- **Query Engine**: ✅ DuckDB + DataFusion integration
- **Caching**: ✅ Moka cache layer in OpenDAL adapter

---

This modular architecture provides a solid foundation for type-safe DAG execution while maintaining clear separation of concerns and easy extensibility.

---

## 📋 **Architecture Alignment Notes**

### **Storage Layer Integration**

**Current Implementation** (confirmed from codebase):

- ✅ **Storage Format**: Wide-format Parquet files per well
  - Schema: `{well_name, depth, gr, rhob, nphi, ...}` (curves as columns)
  - Path: `project-{id}/wells/{well-id}/logs_{log-type}.parquet`
- ✅ **Storage Backend**: OpenDAL with EnhancedOpenDALStorageAdapter
  - **Replaced**: `storage-resolver` and `s3-config` crates
  - **Current**: `opendal-storage-adapter` with Moka caching
- ✅ **Path Management**: `ProjectDataLayoutManager` for centralized path construction

**DAG Integration**:

- Ingestion nodes use `EnhancedOpenDALStorageAdapter.read()` for Parquet access
- Leverages existing Moka cache layer (10-100x performance improvement)
- Uses `ProjectDataLayoutManager` to construct well log paths
- Cross-well queries normalize wide-format data on-demand

### **Query Engine Integration**

**Current Implementation**:

- ✅ **Parquet Query Service**: `opendal-parquet-query-service` with DuckDB
- ✅ **Query Engine**: `parquet-log-query-engine` with DataFusion
- ✅ **Analytics**: DuckDB on VPS (Port 8081)

**DAG Integration**:

- Ingestion nodes can use DuckDB for complex Parquet queries
- Operator nodes use DataFusion for transformations
- Final execution uses Arrow RecordBatch streaming (DataFusion-native)

### **Frontend State Integration**

**Current Implementation**:

- ✅ `PostgresWellsState`: Global wells state with Realtime updates
- ✅ `PostgresCurveMetadataState`: Global curve metadata state
- ✅ Both initialized in `+layout.svelte`

**DAG Integration**:

- Frontend ingestion node builder uses these states for selection UI
- Real-time updates to Postgres automatically refresh available options
- Schema inference uses selected wells/curves to determine output schema

---

## 🚀 **Future Enhancements: Advanced Operator Management Patterns**

### **Overview**

Based on battle-tested patterns from production DAG systems, here are valuable enhancements that can be added to our architecture as the system matures. Our current MVP design is simpler but **fully extensible** to support these patterns.

### **Enhancement 1: Operator Channels (Replaces `is_latest`)**

**Current Design**: `is_latest` flag - simple, one "latest" version per operator  
**Enhanced Design**: Multiple channels (`stable`, `beta`, `canary`, `lts-1.x`) per operator

**Value**: Allows gradual rollouts and environment-specific versions without updating individual DAGs.

```sql
-- Optional enhancement: operator_channels table
CREATE TABLE IF NOT EXISTS public.operator_channels (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    operator_id TEXT NOT NULL,
    channel_name TEXT NOT NULL, -- 'stable', 'beta', 'canary', 'lts-1.x'
    version TEXT NOT NULL,
    UNIQUE (operator_id, channel_name),
    FOREIGN KEY (operator_id, version) REFERENCES operator_registry(operator_id, version)
);

-- Update VersionStrategy enum to support channels
pub enum VersionStrategy {
    Pinned { version: String },
    Channel { channel: String },  // NEW: e.g., "stable", "beta"
    Latest,  // Maps to "stable" channel
    Compatible { fallback_version: String },
}
```

**Migration Path**:

- Phase 1: Keep `is_latest` for backward compatibility
- Phase 2: Add `operator_channels` table, treat `is_latest=true` as `channel='stable'`
- Phase 3: Migrate to channel-based system, deprecate `is_latest`

### **Enhancement 2: Operator Families**

**Value**: Group related operators (e.g., all VShale methods under `vshale` family, all porosity methods under `porosity`).

```sql
-- Optional enhancement: operator_families table
CREATE TABLE IF NOT EXISTS public.operator_families (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    slug TEXT NOT NULL UNIQUE, -- 'vshale', 'porosity', 'saturation'
    title TEXT NOT NULL,
    kind TEXT CHECK (kind IN ('scalar', 'aggregate', 'window', 'table', 'io', 'control')),
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Add family_id to operator_registry
ALTER TABLE operator_registry ADD COLUMN family_id UUID REFERENCES operator_families(id);
```

**Value**:

- Better organization and discovery
- Semantic grouping: `vshale@stable` instead of `shale_volume_larionov@stable`
- Easier to find related operators

### **Enhancement 3: DAG Revisions & Branches (Git-like Versioning)**

**Current Design**: Simple `version` field, single DAG definition per `(project_id, name, version)`  
**Enhanced Design**: Immutable revisions with parent pointers, branches for dev/prod

```sql
-- Optional enhancement: DAG revision tracking
CREATE TABLE IF NOT EXISTS public.pipeline_revisions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    pipeline_id UUID NOT NULL REFERENCES pipelines(id),
    parent_revision_id UUID REFERENCES pipeline_revisions(id), -- Git-like parent
    revision_message TEXT,
    snapshot_uri TEXT, -- s3://... optional Substrait/JSON snapshot
    created_at TIMESTAMPTZ DEFAULT NOW(),
    created_by UUID
);

CREATE TABLE IF NOT EXISTS public.pipeline_branches (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    pipeline_id UUID NOT NULL REFERENCES pipelines(id),
    branch_name TEXT NOT NULL, -- 'main', 'dev', 'feature-x'
    head_revision_id UUID REFERENCES pipeline_revisions(id),
    UNIQUE (pipeline_id, branch_name)
);
```

**Value**:

- Full version history with diffs
- Branch isolation (dev vs production)
- Audit trail of all changes
- Rollback capability

**Current Workaround**: Use `version` field (e.g., `1.0.0`, `1.1.0`) and `pipelines` table's `version` column.

### **Enhancement 4: Semantic Versioning (Semver)**

**Current Design**: Simple `version` TEXT field  
**Enhanced Design**: Explicit semver parsing and validation

```sql
-- Add semver helper columns
ALTER TABLE operator_registry ADD COLUMN IF NOT EXISTS
    semver_major INTEGER GENERATED ALWAYS AS (CAST(SPLIT_PART(version, '.', 1) AS INTEGER)) STORED,
    semver_minor INTEGER GENERATED ALWAYS AS (CAST(SPLIT_PART(version, '.', 2) AS INTEGER)) STORED,
    semver_patch INTEGER GENERATED ALWAYS AS (CAST(SPLIT_PART(version, '.', 3) AS INTEGER)) STORED;

-- Enable queries like: "Get all operators compatible with 1.x"
CREATE INDEX idx_operator_registry_semver ON operator_registry(operator_id, semver_major, semver_minor, semver_patch);
```

**Value**: Better compatibility checks and version range queries.

### **Enhancement 5: Artifact Storage in MinIO (Hybrid Pattern)**

**Current Design**: Large artifacts stored via OpenDAL (MinIO/S3 compatible)  
**Enhanced Pattern**: Explicit separation of control plane (Postgres) and artifacts (MinIO)

**Current Implementation**: ✅ **Already Supported** via OpenDAL

- `pipeline_executions.logs_uri` → MinIO path for logs
- `pipeline_executions` → Metrics and metadata in Postgres
- Large Parquet outputs → Stored via OpenDAL (MinIO/S3)

**Suggested Object Key Pattern**:

```
s3://bucket/dag/{pipeline_id}/{revision_id}/snapshot.json     # DAG snapshot (optional)
s3://bucket/dag_run/{execution_id}/logs/*                     # Execution logs
s3://bucket/schemas/{schema_id}.arrow                         # Arrow IPC schemas
s3://bucket/operators/{operator_id}/{version}/*               # Operator artifacts
```

**Action**: ✅ **No changes needed** - Our OpenDAL integration already supports this pattern.

### **Enhancement 6: Execution Hints (Node-level Optimization Metadata)**

**Value**: Advanced users can provide hints for optimization.

```sql
-- Optional: Add execution hints to pipeline_nodes
ALTER TABLE pipeline_nodes ADD COLUMN IF NOT EXISTS
    execution_hints JSONB DEFAULT '{}'::jsonb;  -- {resources, ordering_promises, cache_hint, branch_policy}

-- Example hints:
{
  "resources": {"cpu": "high", "memory": "8GB"},
  "ordering_promises": [{"col": "well_id"}, {"col": "depth", "dir": "ASC"}],
  "cache_hint": true,
  "cache_ttl_seconds": 3600
}
```

**Value**:

- Performance optimization hints for compiler
- Cache strategy per node
- Resource allocation hints
- Data ordering guarantees

### **Enhancement 7: Resolved Operators Tracking**

**Current Design**: `pipeline_executions.content_hash` for reproducibility  
**Enhanced Pattern**: Explicit `resolved_ops` array per run

```sql
-- Optional: Add resolved operators tracking
ALTER TABLE pipeline_executions ADD COLUMN IF NOT EXISTS
    resolved_operators JSONB;  -- Array of {node_id, operator_id, version, impl_digest}

-- Example:
[
  {"node_id": "node-1", "operator_id": "vshale", "version": "1.4.2", "impl_digest": "sha256:..."},
  {"node_id": "node-2", "operator_id": "despike", "version": "2.1.0", "impl_digest": "sha256:..."}
]
```

**Value**: Perfect reproducibility - exact operator versions used are recorded.

### **Comparison: Current vs Enhanced**

| Feature                | Current (MVP)         | Enhanced Pattern                     | When to Add                            |
| ---------------------- | --------------------- | ------------------------------------ | -------------------------------------- |
| **Operator Updates**   | `is_latest` flag      | Operator channels (`stable`, `beta`) | When needing multiple release channels |
| **Organization**       | Flat operator list    | Operator families + channels         | When operator catalog grows large      |
| **DAG Versioning**     | `version` string      | Git-like revisions + branches        | When needing full history/rollback     |
| **Artifact Storage**   | ✅ OpenDAL (MinIO/S3) | ✅ Explicit MinIO pattern            | ✅ Already supported                   |
| **Reproducibility**    | `content_hash`        | `resolved_operators` array           | When needing explicit version tracking |
| **Optimization Hints** | Basic metadata        | Rich execution hints                 | When optimizing performance            |

### **Recommended Approach**

1. **Keep Current MVP Design** ✅
   - Simpler, easier to understand and maintain
   - Sufficient for initial use cases
   - Fully functional for operator updates

2. **Add Enhancements Incrementally** 📈
   - **Phase 1 (Current)**: `is_latest` + VersionStrategy enum
   - **Phase 2 (Future)**: Add operator channels when needed
   - **Phase 3 (Future)**: Add DAG revisions if version history is critical
   - **Phase 4 (Future)**: Add execution hints for performance optimization

3. **Migration Paths Exist** ✅
   - All enhancements can be added without breaking existing DAGs
   - Backward compatibility maintained throughout

### **Key Takeaway**

The suggested architecture provides **excellent patterns for production-scale systems**, but our current MVP design is:

- ✅ **Simpler to implement and maintain**
- ✅ **Sufficient for initial use cases**
- ✅ **Fully extensible** to support these patterns when needed

**Decision**: **Keep current design**, document enhancements as optional future improvements. Add operator channels as first enhancement when multiple release channels are needed (e.g., `stable`, `beta`, `canary`).

---

## 🚀 **Immediate Next Steps (Action Items)**

Based on the architecture review, here are the concrete next steps to implement the enhanced DAG execution system:

### **Step 1: Update Dependencies** ✅ **COMPLETED**

#### **Backend (Rust)**

**Update `crates/dags/executor/Cargo.toml`** ✅ **DONE**:

- [x] ✅ Already has `petgraph`
- [x] ✅ Already has `tokio`
- [x] ✅ Added `opentelemetry`, `opentelemetry-otlp`, `tracing-opentelemetry`

**Phase 1 Optimizations** ✅ **COMPLETED**:

- [x] ✅ Optimized `crates/dags/core` for memory efficiency (eliminated unnecessary `String` allocations)
- [x] ✅ Optimized `crates/dags/validator` with capacity hints (`Graph::with_capacity`, `HashMap::with_capacity`)
- [x] ✅ Optimized `crates/dags/storage` with capacity hints (`Vec::with_capacity`)
- [x] ✅ Updated `.cursor/rules/global.mdc` with comprehensive Rust best practices
- [x] ✅ All crates compile successfully with no warnings or errors

**Create `crates/dags/operators/Cargo.toml`** (when creating operators crate):

```toml
[dependencies]
schemars = { version = "0.8", features = ["derive"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
arrow = { workspace = true }
# ... other deps
```

#### **Frontend (Svelte)**

**Install graph library**:

```bash
cd src
pnpm add @xyflow/svelte @dagrejs/dagre
```

### **Step 2: Implement Enhanced Executor** ✅ **PRIORITY**

**File**: `crates/dags/executor/src/executor.rs`

1. **Add helper methods**:

   ```rust
   fn build_petgraph(&self, dag: &DagDefinition) -> Result<Graph<Uuid, ()>, DagExecutionError>
   fn merge_dependencies(&self, deps: Vec<(&Uuid, mpsc::Receiver<RecordBatch>)>) -> Result<Option<RecordBatch>, DagExecutionError>
   ```

2. **Replace sequential execution** with parallel execution using bounded channels (code already provided in section 4.1)

3. **Add OpenTelemetry spans** to track execution metrics

### **Step 3: Create Operators Crate with schemars** ⚠️ **PENDING**

**File**: `crates/dags/operators/Cargo.toml` (create new crate)

1. Create `crates/dags/operators/` directory
2. Add `Cargo.toml` with `schemars` dependency
3. Create operator parameter structs with `#[derive(JsonSchema)]`
4. Implement `OperatorDefinition::generate_parameters_schema<T>()` method

**Example structure**:

```
crates/dags/operators/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── registry.rs
│   ├── operators/
│   │   ├── mod.rs
│   │   ├── shale_volume.rs  // With ShaleVolumeParams struct
│   │   ├── porosity.rs
│   │   └── ...
```

### **Step 4: Install and Integrate Svelte Flow** ✅ **COMPLETED**

**Files**: `package.json`, `src/lib/components/SF/SF-pipeline-flow.svelte`

1. ✅ Install packages:

   ```bash
   pnpm add @xyflow/svelte @dagrejs/dagre
   ```

2. ✅ Create basic graph editor component (`SF-pipeline-flow.svelte`)

3. ✅ Test with simple DAG visualization

### **Step 5: Create Tauri Commands** ✅ **NEXT PHASE**

**File**: `src-tauri/src/tauri_commands/pipeline_commands.rs`

1. `save_pipeline` - Save DAG to PostgreSQL
2. `load_pipeline` - Load DAG from PostgreSQL
3. `execute_pipeline` - Execute DAG (uses enhanced executor)
4. `list_pipelines` - List all pipelines for a project
5. `get_available_operators` - Get operator catalog from registry

### **Step 6: Wire Up Frontend → Backend** ⚠️ **AFTER STEPS 1-5**

1. Connect DAG editor to Tauri commands
2. Save DAG from frontend to backend
3. Execute DAG from frontend
4. Display execution results

---

## 📅 **Recommended Implementation Order**

**Week 1-2 (Foundation Setup)**:

1. ✅ Update `crates/dags/executor/Cargo.toml` (done)
2. ✅ Install Svelte Flow (`pnpm add @xyflow/svelte @dagrejs/dagre`)
3. ✅ Create basic graph editor component (`SF-pipeline-flow.svelte`)
4. ✅ **Phase 1 Optimizations**: Optimized core crates for memory efficiency, updated Rust best practices
5. ⚠️ Implement `build_petgraph()` and `merge_dependencies()` helpers

**Week 3-4 (Core Execution)**:

1. ⚠️ Implement enhanced executor with bounded channels
2. ⚠️ Add OpenTelemetry spans and metrics
3. ⚠️ Test parallel execution with simple DAGs
4. ⚠️ Create `crates/dags/operators` crate with `schemars`

**Week 5-6 (Integration)**:

1. ⚠️ Create Tauri commands (`save_pipeline`, `load_pipeline`, `execute_pipeline`)
2. ⚠️ Connect frontend graph editor to Tauri commands
3. ⚠️ End-to-end test: Build DAG → Save → Execute → Display results

---

## 🎯 **Immediate Action Items (This Week)**

1. **Install frontend dependencies**:

   ```bash
   cd src
   pnpm add @xyflow/svelte @dagrejs/dagre
   ```

2. **Review executor code** in `dag-execution.md` section 4.1 (enhanced version with bounded channels)

3. **Create operators crate structure**:

   ```bash
   mkdir -p crates/dags/operators/src/operators
   # Create Cargo.toml with schemars dependency
   ```

4. **Test petgraph integration**: Verify `toposort` works with your `DagDefinition` structure

**Next Step**: Start with **Step 1** (dependencies) and **Step 2** (enhanced executor implementation). The executor changes are the most critical for production readiness.

---

## 🎨 **Svelte Flow Integration Plan**

### **Overview**

We're using **Svelte Flow** (formerly React Flow for Svelte) as the visual DAG editor, with **Dagre** for automatic hierarchical layout. This provides a no-code interface for building and editing pipelines.

### **Storage Format Decision: JSONB in Postgres ✅**

**Decision**: Keep `dag_definition` as **JSONB** in Postgres.

**Rationale**:

- ✅ **Flexible**: Handles evolving DAG structure without schema migrations
- ✅ **Queryable**: Postgres JSONB supports indexing and queries (`@>`, `->`, `->>`)
- ✅ **Normalized tables**: `pipeline_nodes` and `pipeline_edges` provide efficient graph queries
- ✅ **WLIP Compatible**: JSONB can store PROV-O lineage metadata alongside DAG structure
- ✅ **Type-safe**: Rust `serde` handles serialization/deserialization with validation
- ❌ **Not needed**: External storage (MinIO) would add complexity without benefits for DAG metadata

**Hybrid Pattern** (already in place):

- **Control plane** (Postgres): DAG definitions, metadata, execution history
- **Data plane** (MinIO): Large Parquet outputs, execution logs (via `logs_uri`)

### **Type Conversion Architecture**

**Files Created**:

- `src/lib/services/dag-types.ts` - TypeScript types matching Rust `DagDefinition`
- `src/lib/services/dag-converter.ts` - Bidirectional conversion utilities

**Conversion Flow**:

```
Backend (Rust)                    Frontend (TypeScript)
─────────────────                 ─────────────────────
DagDefinition                     DagDefinition (TypeScript)
  ├─ id: Uuid                     ├─ id: string (UUID)
  ├─ nodes: Vec<DagNode>          ├─ nodes: DagNode[]
  │   ├─ id: Uuid                 │   ├─ id: string
  │   ├─ dependencies: Vec<Uuid>   │   ├─ dependencies: string[]
  │   └─ metadata.position        │   └─ metadata.position: [x, y]
  └─ ...                          └─ ...

         │                                    │
         │ JSONB in Postgres                  │
         │ (serde_json serialization)         │
         │                                    │
         ▼                                    ▼

Svelte Flow Format
──────────────────
{ nodes: SvelteFlowNode[], edges: SvelteFlowEdge[] }
  ├─ nodes: { id, position, data: { label, nodeType, nodeConfig } }
  └─ edges: { id, source, target, type: 'smoothstep' }
```

**Key Conversion Functions**:

```typescript
// dag-converter.ts

// Backend → Frontend (for visualization)
dagDefinitionToSvelteFlow(dag: DagDefinition): { nodes, edges }

// Frontend → Backend (for saving)
svelteFlowToDagDefinition(nodes, edges, baseDag): DagDefinition

// Layout (preserves manual positions, auto-layouts missing positions)
applyDagreLayout(nodes, edges, direction: 'TB' | 'LR'): SvelteFlowNode[]
```

### **Component Architecture**

```
content-dag-editor.svelte (Parent)
├─ Fetches pipeline via getPipelineById(selectedPipelineId)
├─ Converts: dagDefinitionToSvelteFlow(dag_definition)
├─ Manages save/load state
└─ Passes data as props to child

SF-pipeline-flow.svelte (Child - Pure Visualization)
├─ Receives: { nodes, edges } as props
├─ Uses Dagre for automatic layout (if positions missing)
├─ Handles user interactions (drag, connect, delete)
├─ Emits changes back to parent via callbacks
└─ Uses Svelte Flow: SvelteFlow, Background, Controls, MiniMap, Panel
```

**State Management**:

- `ContentDagPipelineState.selectedPipelineId` - Global state for selected pipeline
- Local state in `content-dag-editor.svelte` for:
  - `dagDefinition` (backend format)
  - `svelteFlowData` (frontend format)
  - `isDirty` (unsaved changes flag)

### **Dagre Layout Integration**

**Library**: `@dagrejs/dagre` (install via `pnpm add @dagrejs/dagre`)

**Layout Strategy**:

1. **Preserve manual positions**: If `node.metadata.position` exists, use it
2. **Auto-layout missing**: Apply Dagre to nodes without positions
3. **Direction**: Top-to-bottom (`TB`) by default, configurable to left-to-right (`LR`)
4. **Re-layout button**: Allow users to trigger re-layout while preserving manual adjustments

**Implementation** (in `SF-pipeline-flow.svelte`):

```typescript
import dagre from "@dagrejs/dagre";
import { Position } from "@xyflow/svelte";

function getLayoutedElements(nodes: Node[], edges: Edge[], direction = "TB") {
  const dagreGraph = new dagre.graphlib.Graph();
  dagreGraph.setDefaultEdgeLabel(() => ({}));
  dagreGraph.setGraph({ rankdir: direction });

  const nodeWidth = 200;
  const nodeHeight = 50;

  // Add nodes (only if position missing or user requests re-layout)
  nodes.forEach((node) => {
    dagreGraph.setNode(node.id, { width: nodeWidth, height: nodeHeight });
  });

  // Add edges
  edges.forEach((edge) => {
    dagreGraph.setEdge(edge.source, edge.target);
  });

  dagre.layout(dagreGraph);

  // Update positions
  return nodes.map((node) => {
    const nodeWithPosition = dagreGraph.node(node.id);
    return {
      ...node,
      position: {
        x: nodeWithPosition.x - nodeWidth / 2,
        y: nodeWithPosition.y - nodeHeight / 2,
      },
      sourcePosition: direction === "LR" ? Position.Right : Position.Bottom,
      targetPosition: direction === "LR" ? Position.Left : Position.Top,
    };
  });
}
```

### **WLIP Compatibility Considerations**

**Future-proofing for Well Lifecycle Integration Platform**:

1. **PROV-O Lineage**: `DagDefinition` structure already supports provenance:
   - Each `DagNode` can store `metadata` with lineage info
   - `pipeline_lineage` table tracks execution-time transformations
   - Can extend `NodeMetadata` with PROV-O fields (Entity, Activity, Agent)

2. **Immutable Audit**:
   - `pipeline_executions` table provides execution history
   - Can add `pipeline_revisions` table for Git-like versioning (future enhancement)

3. **Schema Evolution**:
   - JSONB allows adding fields without migrations
   - Versioned `DagDefinition` structure (`version` field)
   - Backward-compatible deserialization via `serde`

4. **Adapter SDK Integration**:
   - Operators can emit PROV-O events during execution
   - `pipeline_lineage` table stores transformation metadata
   - Future: Add `wlip-provenance-svc` integration layer

### **Implementation Steps**

#### **Step 1: Install Dependencies** ✅ **PRIORITY**

```bash
pnpm add @dagrejs/dagre
# @xyflow/svelte already installed ✅
```

#### **Step 2: Update Components** ⏳ **IN PROGRESS**

1. **Update `SF-pipeline-flow.svelte`**:
   - Accept `nodes` and `edges` as props (instead of hardcoded)
   - Add Dagre layout function
   - Handle node/edge changes and emit to parent
   - Add layout direction toggle (TB/LR)

2. **Update `content-dag-editor.svelte`**:
   - Fetch `dag_definition` from `getPipelineById(selectedPipelineId)`
   - Convert using `dagDefinitionToSvelteFlow()`
   - Pass converted data to `SF-pipeline-flow.svelte`
   - Handle save: convert back using `svelteFlowToDagDefinition()`
   - Call `updatePipelineDagDefinition()` to persist

3. **Add Save/Load Handlers**:
   - "Save" button: convert → validate → save to Postgres
   - "Reload" button: fetch latest from Postgres
   - Auto-save on node position changes (debounced)

#### **Step 3: Node Type Styling** ⏳ **NEXT**

- Custom node types for `ingestion`, `operator`, `output`
- Color coding: ingestion (blue), operator (green), output (orange)
- Node labels show: name, type, operator_id (if operator)
- Connection validation: highlight incompatible edges

#### **Step 4: User Interactions** ⏳ **FUTURE**

- **Add Node**: Drag from palette or context menu
- **Connect Nodes**: Click source handle → drag → click target handle
- **Delete Node**: Right-click → delete (with dependency check)
- **Edit Node**: Double-click to open configuration panel
- **Layout**: Button to trigger Dagre re-layout

### **Data Flow Summary**

```
User Action: Select Pipeline
    ↓
content-dag-editor.svelte: getPipelineById(selectedPipelineId)
    ↓
Postgres: SELECT dag_definition FROM pipelines WHERE id = ...
    ↓
content-dag-editor.svelte: dagDefinitionToSvelteFlow(dag_definition)
    ↓
SF-pipeline-flow.svelte: Render nodes/edges with Dagre layout
    ↓
User Action: Drag node, add edge, etc.
    ↓
SF-pipeline-flow.svelte: Emit onChange(nodes, edges)
    ↓
content-dag-editor.svelte: svelteFlowToDagDefinition(nodes, edges, baseDag)
    ↓
User Action: Click "Save"
    ↓
content-dag-editor.svelte: updatePipelineDagDefinition(pipelineId, dagDefinition)
    ↓
Postgres: UPDATE pipelines SET dag_definition = ... WHERE id = ...
```

### **Schema Compatibility**

**Current Schema** (`02-pipeline-schema.sql`):

- ✅ `pipelines.dag_definition` (JSONB) - Stores complete `DagDefinition`
- ✅ `pipeline_nodes` (normalized) - For efficient queries
- ✅ `pipeline_edges` (normalized) - For graph queries
- ✅ `NodeMetadata.position` - Stores `[x, y]` coordinates

**No Schema Changes Needed**: Current design supports Svelte Flow integration.

**Future Enhancements** (optional):

- Add `pipeline_revisions` table for Git-like versioning
- Add `pipeline_branches` table for dev/prod isolation
- Extend `NodeMetadata` with PROV-O fields (Entity, Activity, Agent)

### **Testing Strategy**

1. **Unit Tests** (`dag-converter.ts`):
   - Test `dagDefinitionToSvelteFlow()` with various DAG structures
   - Test `svelteFlowToDagDefinition()` round-trip conversion
   - Test position preservation

2. **Integration Tests**:
   - Load pipeline → convert → render → edit → save → reload
   - Verify positions persist after save/reload
   - Verify Dagre layout works for empty DAGs

3. **E2E Tests**:
   - Create pipeline → add nodes → connect → save → execute
   - Verify execution uses correct DAG structure

---

**Next Step**: Complete **Step 2** (Update Components) to wire up the conversion and visualization.
