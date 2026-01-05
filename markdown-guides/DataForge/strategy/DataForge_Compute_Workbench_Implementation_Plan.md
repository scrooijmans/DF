# DataForge Compute Workbench — Implementation Plan

*A principal-level implementation roadmap integrating architectural insights from DataForge's design documentation.*

This plan synthesizes the detailed architecture from [DFC-extensibility-contract-udfs.md](../design-concepts/DF-Compute/DFC-extensibility-contract-udfs.md), [DFC-model-view-sync.md](../design-concepts/DF-Compute/DFC-model-view-sync.md), [DFC-udf-type-safety.md](../design-concepts/DF-Compute/DFC-udf-type-safety.md), [DFC-Recomputations-trigger.md](../design-concepts/DF-Compute/DFC-Recomputations-trigger.md), and the core DataForge principles from [DF-unit-of-truth.md](../design-concepts/DF-unit-of-truth.md) and [DF-mutability.md](../design-concepts/DF-mutability.md).

---

## 0. Foundational Principles (The Invariants)

These principles are **non-negotiable** and derive from the MVP spec and architecture documents:

### Core Invariants

1. **Explicit Execution Only** — Tools run when user clicks "Execute", never implicitly (QGIS-style)
2. **Immutable Inputs** — UDF inputs are `Arc<CurveData>`, never modified
3. **Append-Only Outputs** — Derived artifacts are new curves with provenance, never mutations
4. **Pure Function Computation** — `execute(inputs, params) → output` is deterministic
5. **One Data Model, Many Views** — Views observe data through DataStore, never own it
6. **Curve as Unit of Truth** — Curve = Metadata (SQL) + Data (content-addressed Parquet blob)

### Red Flag Checks (Before Any Implementation)

- ❌ Any view writes data directly → Fix architecture
- ❌ Any background process changes results implicitly → Redesign
- ❌ Any UDF mutates input curves → Forbid at API level
- ❌ Silent type coercion without user consent → Reject

### Shopping List

- [x] "Golden path" demo: Load GR curve → Run VShale Linear → View result with provenance ✅ *Implemented in DataForge-Compute*
- [ ] Written invariants posted in team docs
- [ ] Vocabulary document: Artifact, Run, Curve, UDF, Provider

---

## 1. Core Domain Model (Unit of Truth)

### 1.1 The Curve Contract

A DataForge **Curve** (the unit of truth) comprises:

```
Curve = Metadata (curves table) + Data (Parquet blob by SHA-256 hash)
```

**Metadata (SQL):**
- `id` — UUID (stable identity, never changes)
- `mnemonic` — Channel name (GR, RHOB, NPHI)
- `well_id` — Parent well reference
- `native_parquet_hash` — SHA-256 of current data blob
- `version` — Monotonic version counter
- Statistics, units, depth range, provenance

**Data (Blob Store):**
- Parquet file: `[depth, value]` columns
- Content-addressed by SHA-256
- Immutable once written

### 1.2 Compute-Specific Entities

```rust
// Execution record (provenance)
struct ExecutionRecord {
    id: Uuid,
    udf_id: String,                    // e.g., "petro:vshale_linear"
    udf_version: String,               // UDF semantic version
    inputs: Vec<InputReference>,       // (curve_id, version, parquet_hash)
    parameters: serde_json::Value,     // Serialized parameter values
    output_curve_id: Uuid,             // Resulting curve
    output_parquet_hash: String,       // Content hash of output
    started_at: DateTime<Utc>,
    completed_at: DateTime<Utc>,
    compute_app_version: String,       // For reproducibility
    status: ExecutionStatus,           // Succeeded, Failed
}

// Input reference for provenance tracking
struct InputReference {
    curve_id: Uuid,
    version: u64,
    parquet_hash: String,              // Content-addressed reference
}
```

### Shopping List

- [x] Artifact ID scheme: UUID for identity, SHA-256 for content addressing ✅ *types.rs: CurveData, InputReference*
- [x] `curves` table integration (read existing, write derived) ✅ *commands.rs: list_curves, get_curve_data*
- [x] `execution_records` table schema ✅ *types.rs: ExecutionRecord, data_loader.rs schema*
- [x] Clear labeling: raw vs derived artifacts (UI affordances + metadata flag) ✅ *is_derived flag, source_execution_id*
- [x] Lineage edges: `inputs → execution_record → outputs` ✅ *InputReference tracking in ExecutionRecord*

---

## 2. UDF Framework (Provider-Based Extensibility)

### 2.1 Architecture Overview

Adopting the provider-based model from [DFC-extensibility-contract-udfs.md](../design-concepts/DF-Compute/DFC-extensibility-contract-udfs.md):

```
┌─────────────────────────────────────────────────────────┐
│                    UDF Registry                          │
│  - Indexes UDFs by ID: {provider_id}:{udf_id}           │
│  - Provides lookup, validation, execution               │
└────────────────────────┬────────────────────────────────┘
                         │
         ┌───────────────┼───────────────┐
         ▼               ▼               ▼
┌─────────────┐  ┌─────────────┐  ┌─────────────┐
│    core     │  │    petro    │  │   plugin    │
│  Provider   │  │  Provider   │  │  Provider   │
├─────────────┤  ├─────────────┤  ├─────────────┤
│ MovingAvg   │  │ VShale      │  │ CustomUDF1  │
│ Resample    │  │ Porosity    │  │ CustomUDF2  │
│ Scale       │  │ Saturation  │  └─────────────┘
└─────────────┘  └─────────────┘

┌─────────────────────────────────────────────────────────┐
│                  Execution Engine                        │
│  - Multi-stage validation                                │
│  - Sandboxed ExecutionContext                           │
│  - Exception boundary (catches panics)                   │
│  - Provenance recording                                  │
└─────────────────────────────────────────────────────────┘
```

### 2.2 Core Traits

```rust
// crates/dataforge-compute/src/udf/provider.rs

/// Groups related UDFs together (QGIS QgsProcessingProvider-inspired)
pub trait UdfProvider: Send + Sync {
    fn id(&self) -> &str;                           // e.g., "petro"
    fn name(&self) -> &str;                         // "Petrophysics"
    fn version(&self) -> &str;
    fn load_udfs(&self) -> Vec<Box<dyn Udf>>;
    fn is_available(&self) -> Result<(), ProviderError>;
}

// crates/dataforge-compute/src/udf/mod.rs

/// Core UDF trait (QGIS QgsProcessingAlgorithm-inspired)
pub trait Udf: Send + Sync {
    fn id(&self) -> &str;                           // e.g., "vshale_linear"
    fn metadata(&self) -> UdfMetadata;
    fn parameter_definitions(&self) -> Vec<Box<dyn ParameterDefinition>>;
    fn output_schema(&self) -> OutputSchema;

    // Blender-style poll function
    fn can_execute(&self, context: &AvailabilityContext) -> bool { true }
    fn unavailability_reason(&self, context: &AvailabilityContext) -> Option<String> { None }

    // QGIS-style lifecycle
    fn prepare(&self, context: &mut ExecutionContext) -> Result<bool, UdfError> { Ok(true) }
    fn execute(&self, context: &ExecutionContext) -> Result<UdfOutput, UdfError>;
    fn postprocess(&self, output: &mut UdfOutput, context: &ExecutionContext) -> Result<(), UdfError> { Ok(()) }

    // Custom validation beyond type checking
    fn check_parameters(&self, context: &ExecutionContext) -> Result<(), Vec<ValidationError>> { Ok(()) }
}
```

### 2.3 Execution Lifecycle

From [DFC-extensibility-contract-udfs.md](../design-concepts/DF-Compute/DFC-extensibility-contract-udfs.md):

```
User Clicks "Execute"
       │
       ▼
┌─────────────────────────────────────────────────────────┐
│ STAGE 1: Frontend Validation (ToolPanel.svelte)         │
│  → validateParameters() at GUI level                    │
│  → invoke('execute_udf', { udf_id, params, inputs })   │
└────────────────────────┬────────────────────────────────┘
                         ▼
┌─────────────────────────────────────────────────────────┐
│ STAGE 2: Command Layer (commands.rs)                    │
│  → Validate UDF exists in registry                      │
│  → Parse parameters to typed values                     │
│  → Load input curves from DataForge blobs               │
│  → Create ExecutionContext (sandboxed)                  │
└────────────────────────┬────────────────────────────────┘
                         ▼
┌─────────────────────────────────────────────────────────┐
│ STAGE 3: Multi-Stage Parameter Validation               │
│  → Type validation (ParameterDefinition.check_value)    │
│  → Constraint validation (ranges, enums)                │
│  → Custom validation (udf.check_parameters)             │
│  → Cross-parameter validation                           │
└────────────────────────┬────────────────────────────────┘
                         ▼
┌─────────────────────────────────────────────────────────┐
│ STAGE 4: Preparation (udf.prepare)                      │
│  → Validate inputs exist and accessible                 │
│  → Returns true/false (false aborts)                    │
└────────────────────────┬────────────────────────────────┘
                         ▼
┌─────────────────────────────────────────────────────────┐
│ STAGE 5: Execution (udf.execute) — PURE FUNCTION        │
│  → Inputs are Arc<CurveData> (immutable)               │
│  → Progress via context.set_progress()                  │
│  → Cancellation via context.is_cancelled()              │
│  → Returns UdfOutput (new data)                         │
│  → Exception boundary catches panics                    │
└────────────────────────┬────────────────────────────────┘
                         ▼
┌─────────────────────────────────────────────────────────┐
│ STAGE 6: Output Validation                              │
│  → Validate output matches output_schema                │
│  → Required outputs present                             │
└────────────────────────┬────────────────────────────────┘
                         ▼
┌─────────────────────────────────────────────────────────┐
│ STAGE 7: Postprocess (udf.postprocess)                  │
│  → Apply default styling/metadata                       │
│  → Compute derived statistics                           │
└────────────────────────┬────────────────────────────────┘
                         ▼
┌─────────────────────────────────────────────────────────┐
│ STAGE 8: Provenance Recording                           │
│  → Create ExecutionRecord                               │
│  → Store in execution_records table                     │
└────────────────────────┬────────────────────────────────┘
                         ▼
┌─────────────────────────────────────────────────────────┐
│ STAGE 9: Result Return + Event Emission                 │
│  → Write output blob (new Parquet)                      │
│  → Register output curve in DataForge                   │
│  → Emit DataChangeEvent { Curve, curve_id, Added }     │
└─────────────────────────────────────────────────────────┘
```

### 2.4 Core State Protection

From [DFC-extensibility-contract-udfs.md](../design-concepts/DF-Compute/DFC-extensibility-contract-udfs.md):

1. **Input Immutability** — Inputs are `Arc<CurveData>`, cannot be modified
2. **Context Isolation** — Each execution gets fresh, isolated context
3. **Error Isolation** — Exception boundary catches panics, UDF errors don't crash system
4. **DataForge Protection** — UDFs have read-only access to CurveStore

### Shopping List

- [x] `UdfProvider` trait implementation ✅ *compute/providers/mod.rs*
- [x] `Udf` trait implementation ✅ *compute/providers/mod.rs: Udf trait*
- [x] `UdfRegistry` with provider/UDF registration ✅ *compute/registry.rs*
- [x] `ExecutionEngine` with 9-stage lifecycle ✅ *compute/engine.rs*
- [x] `ExecutionContext` with sandboxed access ✅ *compute/context.rs*
- [x] Exception boundary (panic catching) ✅ *engine.rs: catch_unwind in execute*
- [x] Parameter definitions: `CurveParameter`, `NumericParameter`, `BooleanParameter`, `EnumParameter` ✅ *compute/parameters.rs*

---

## 3. Type Safety System (Multi-Stage Validation)

### 3.1 Type Hierarchy

From [DFC-udf-type-safety.md](../design-concepts/DF-Compute/DFC-udf-type-safety.md):

```
CurveDataType (Base)
├── NumericCurve
│   ├── Float64Curve      // f64 (most common)
│   ├── Float32Curve      // f32 (memory efficient)
│   └── IntegerCurve      // i64 (discrete data)
├── CategoricalCurve
│   ├── StringCurve       // String categories
│   └── EnumCurve         // Predefined enum values
├── BooleanCurve          // true/false flags
└── NullableCurve<T>      // Any type with null handling
```

### 3.2 Validation Stages

```
┌─────────────────────────────────────────────────────────┐
│ Stage 1: GUI Validation (Immediate Feedback)            │
│  - Widget-level type checking                           │
│  - Range validation for numeric inputs                  │
│  - Dropdown filtering by compatible types               │
│  - Real-time error indicators                           │
└────────────────────────┬────────────────────────────────┘
                         ▼
┌─────────────────────────────────────────────────────────┐
│ Stage 2: Parameter Definition Validation (Pre-Execute)  │
│  - All parameters validated against definitions         │
│  - Type constraints checked                             │
│  - Required parameters verified                         │
└────────────────────────┬────────────────────────────────┘
                         ▼
┌─────────────────────────────────────────────────────────┐
│ Stage 3: Parameter Evaluation (Type-Safe Retrieval)     │
│  - parameter_as_curve() validates and returns curve     │
│  - parameter_as_float() validates and returns f64       │
│  - Coercion applied only if allowed                     │
└────────────────────────┬────────────────────────────────┘
                         ▼
┌─────────────────────────────────────────────────────────┐
│ Stage 4: UDF-Level Validation (Custom Checks)           │
│  - check_parameters() for algorithm-specific rules      │
│  - Cross-parameter validation                           │
│  - Domain-specific constraints                          │
└────────────────────────┬────────────────────────────────┘
                         ▼
┌─────────────────────────────────────────────────────────┐
│ Stage 5: Execution Validation (Runtime Safety)          │
│  - SafeDownCast-style type assertions                   │
│  - Defense in depth                                     │
└─────────────────────────────────────────────────────────┘
```

### 3.3 Coercion Rules

**Principle: No silent conversions**

| From → To | Rule |
|-----------|------|
| Float32 → Float64 | Always Allow (widening) |
| Integer → Float64 | Always Allow (widening) |
| Float64 → Float32 | Allow With Warning (precision loss) |
| Float64 → Integer | Allow With Warning (truncation) |
| Numeric ↔ String | Requires Consent |
| Any ↔ Boolean | Requires Consent |

### Shopping List

- [x] `CurveDataType` enum with compatibility checking ✅ *compute/types.rs: CurveDataType enum*
- [x] `ParameterDefinition` trait implementations ✅ *compute/parameters.rs: ParameterDefinition trait*
- [x] Type-safe accessor methods: `parameter_as_curve()`, `parameter_as_float()`, etc. ✅ *context.rs: get_curve, get_float, get_bool*
- [x] `ValidationError` enum with rich error messages ✅ *compute/error.rs: ValidationError struct*
- [x] Frontend validation integration (real-time feedback) ✅ *ParameterForm.svelte: real-time validation*

---

## 4. Multi-View Synchronization Architecture

### 4.1 One Data Model, Many Views

From [DFC-model-view-sync.md](../design-concepts/DF-Compute/DFC-model-view-sync.md):

```
┌─────────────────────────────────────────────────────────┐
│                    UI LAYER (Svelte)                     │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐ │
│  │ Map View │  │ Scatter  │  │   Well   │  │  Curve   │ │
│  │          │  │   Plot   │  │  Chart   │  │  Editor  │ │
│  ├──────────┤  ├──────────┤  ├──────────┤  ├──────────┤ │
│  │ Accepts: │  │ Accepts: │  │ Accepts: │  │ Accepts: │ │
│  │ Well     │  │ Curve    │  │ Curve    │  │ Curve    │ │
│  │ Selection│  │ Selection│  │ Well     │  │ Selection│ │
│  └────┬─────┘  └────┬─────┘  └────┬─────┘  └────┬─────┘ │
└───────┼─────────────┼─────────────┼─────────────┼───────┘
        └─────────────┴──────┬──────┴─────────────┘
                             ▼
┌─────────────────────────────────────────────────────────┐
│                     VIEW MANAGER                         │
│  - Manages view lifecycle                                │
│  - Routes notifications to relevant views               │
│  - Filters by view capabilities                         │
└────────────────────────┬────────────────────────────────┘
                         ▼  Events / Notifications
┌─────────────────────────────────────────────────────────┐
│                    BRIDGE LAYER                          │
│  ┌─────────────────────────────────────────────────────┐│
│  │              DATA-VIEW BRIDGE                        ││
│  │  - Subscribes to DataStore events                   ││
│  │  - Translates to view-specific notifications        ││
│  └─────────────────────────────────────────────────────┘│
│  ┌─────────────────────────────────────────────────────┐│
│  │           EXTERNAL DATA BRIDGE                       ││
│  │  - Polls DataForge database for changes             ││
│  │  - Converts external changes to internal events     ││
│  └─────────────────────────────────────────────────────┘│
└────────────────────────┬────────────────────────────────┘
                         ▼
┌─────────────────────────────────────────────────────────┐
│                  DATA STORE (Rust)                       │
│  - Single source of truth                                │
│  - Emits DataChangeEvent on all changes                 │
│  - Read-only access to DataForge database               │
│  ┌────────────┐  ┌────────────┐  ┌────────────────────┐ │
│  │ CurveStore │  │  WellStore │  │ ExecutionResultStore│ │
│  └────────────┘  └────────────┘  └────────────────────┘ │
└─────────────────────────────────────────────────────────┘
```

### 4.2 Event-Driven Updates

**Key Principle: Views never own data**

```typescript
// Event Types
enum DataChangeType {
  Curve = 'curve',
  Well = 'well',
  ToolResult = 'tool_result',
  Selection = 'selection',
}

interface DataChangeEvent {
  dataType: DataChangeType;
  itemId: string;
  changeKind: 'added' | 'updated' | 'removed' | 'refreshed';
  context?: Record<string, unknown>;
}

// View capabilities declaration
interface ViewCapabilities {
  acceptedDataTypes: DataChangeType[];
  needsWellPositions: boolean;
  needsCurveData: boolean;
  respondsToSelection: boolean;
}
```

### 4.3 Selection Synchronization

Selection is shared across all views:

```typescript
const selectionStore = createSelectionStore();

function onWellClicked(wellId: string) {
  selectionStore.setActiveWell(wellId);
  selectionStore.setSelectedWells([wellId]);
  // ViewManager automatically notifies all views
}
```

### Shopping List

- [x] `DataStore` (Rust) with event emission ✅ *stores/dataStore.ts: central data management*
- [x] `DataViewBridge` (Rust) for data → view translation ✅ *stores/events.ts: DataEventBus*
- [ ] `ExternalDataBridge` (Rust) for DataForge polling — *Not needed for MVP (read-only)*
- [x] `ViewManager` (TypeScript) for view registration/routing ✅ *+page.svelte: tabbed view management*
- [x] `ViewCapabilities` declarations per view type ✅ *events.ts: ViewCapabilities interface*
- [x] `SelectionStore` (TypeScript) with sync to backend ✅ *stores/selection.ts*
- [x] Event taxonomy: `ArtifactAdded`, `ArtifactUpdated`, `RunStateChanged` ✅ *events.ts: DataChangeType enum*

---

## 5. Explicit Execution Model

### 5.1 What Triggers Computation

From [DFC-Recomputations-trigger.md](../design-concepts/DF-Compute/DFC-Recomputations-trigger.md):

**✅ Triggers Recomputation:**
1. "Execute" Button Click — User explicitly runs a UDF
2. Pipeline "Run" Command — User executes entire pipeline
3. API Call: `execute_udf(udf_id, params, inputs)` — Programmatic execution
4. Batch Processing Request — User submits batch of UDFs

**❌ Does NOT Trigger Recomputation:**
- Parameter changes (just marks UI as "modified")
- Input curve selection changes (UI only)
- UDF selection changes (UI only)
- Source data changes in DataForge (Compute is read-only)
- Pipeline topology changes (validation only)

### 5.2 UI State vs Execution State

```typescript
// UI State (changes don't trigger computation)
const toolUIState = {
  selectedUdf: writable<Udf | null>(null),
  currentParams: writable<Record<string, unknown>>({}),
  selectedInputs: writable<CurveSelection[]>([]),
  validationErrors: writable<ValidationError[]>([]),
  isModified: writable(false),
  isExecuting: writable(false),
};

// Execution State (changes after computation completes)
const executionState = {
  lastResult: writable<ExecutionResult | null>(null),
  executionHistory: writable<ExecutionRecord[]>([]),
  executionStatus: writable<'idle' | 'running' | 'completed' | 'failed'>('idle'),
};
```

### 5.3 Parameter Change Flow

```
Parameter Change
       │
       ▼
┌─────────────────────────────────────────────────────────┐
│ Update UI State                                          │
│  - toolUIState.currentParams.set(...)                   │
│  - toolUIState.isModified.set(true)                     │
│  - Validate parameters (frontend)                        │
│  - Update validation indicators                          │
│                                                          │
│ ❌ NO COMPUTATION                                        │
└─────────────────────────────────────────────────────────┘
       │
       ▼
┌─────────────────────────────────────────────────────────┐
│ UI Re-renders                                            │
│  - Parameter input reflects new value                   │
│  - "Modified" indicator shown                           │
│  - Execute button enabled/disabled based on validation  │
└─────────────────────────────────────────────────────────┘
```

### Shopping List

- [x] Clear "Execute" button workflow ✅ *ParameterForm.svelte: Execute button*
- [ ] "Modified" indicator in UI — *Nice to have, not MVP critical*
- [x] Separation of UI state and execution state ✅ *stores/compute.ts: separate stores*
- [x] No implicit recomputation on any user interaction ✅ *Only executes on button click*
- [x] Progress reporting during execution ✅ *context.rs: ProgressState, commands.rs: get_execution_progress*
- [x] Cancellation support ✅ *context.rs: CancellationToken, commands.rs: cancel_execution*

---

## 6. Compute Contract With DataForge (Read/Write APIs)

### 6.1 Read Contract

Compute is a **read-only client** of DataForge:

```rust
// Read-only database connection
let db = Connection::open_with_flags(
    &db_path,
    rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY,
)?;

// Read operations
trait DataForgeReader {
    /// List curves for a well
    fn list_curves(&self, well_id: Uuid) -> Result<Vec<CurveMetadata>>;

    /// Get curve metadata
    fn get_curve_metadata(&self, curve_id: Uuid) -> Result<CurveMetadata>;

    /// Load curve data from blob store
    fn load_curve_data(&self, parquet_hash: &str) -> Result<Arc<CurveData>>;
}
```

### 6.2 Write Contract (Transactional)

Writing derived artifacts back to DataForge:

```rust
trait DataForgeWriter {
    /// Register an execution run and its output
    fn register_execution(
        &self,
        request: RegisterExecutionRequest,
    ) -> Result<RegisterExecutionResponse>;
}

struct RegisterExecutionRequest {
    // UDF identification
    udf_id: String,
    udf_version: String,

    // Input provenance
    inputs: Vec<InputReference>,

    // Parameters (serialized)
    parameters: serde_json::Value,

    // Compute environment
    compute_app_version: String,

    // Output
    output_mnemonic: String,
    output_parquet: Vec<u8>,           // Parquet bytes
    output_metadata: CurveMetadata,    // Units, description, etc.
}

struct RegisterExecutionResponse {
    execution_id: Uuid,
    output_curve_id: Uuid,
    output_parquet_hash: String,
}
```

### 6.3 Atomic Write Strategy

```
1. Write output Parquet to temp location
2. fsync() to ensure durability
3. Compute SHA-256 hash
4. Rename to final path (atomic on POSIX)
5. Insert curve record in transaction
6. Insert execution_record in same transaction
7. Commit
```

### Shopping List

- [x] `RegisterExecution` API shape ✅ *output_writer.rs: RegisteredOutput, commit_execution*
- [ ] Atomic write implementation (temp → fsync → rename → commit) — *Prepared in output_writer.rs, not wired for production*
- [x] Error model (typed error codes) ✅ *compute/error.rs: UdfError enum*
- [x] Policy: outputs are always new artifacts (never overwrite) ✅ *Append-only by design*

---

## 7. MVP View Set

### 7.1 Required Views

| View | Purpose | Accepts |
|------|---------|---------|
| **Artifact Inspector** | Show curve metadata, statistics | Curve |
| **Coverage View** | Curve coverage intervals vs depth | Curve |
| **Simple Plot View** | 1-2 curves vs depth (minimal) | Curve, Selection |

### 7.2 Provenance Panel

For any derived artifact:
- Parent artifacts (input curves)
- UDF + parameters used
- Execution timestamp
- Compute app version

### Shopping List

- [x] Artifact Inspector component ✅ *components/ArtifactInspector.svelte*
- [x] Coverage View component ✅ *components/CoverageView.svelte*
- [x] Simple Plot View component ✅ *components/SimplePlot.svelte*
- [x] Provenance Panel component ✅ *components/ProvenancePanel.svelte*
- [x] Shared depth axis semantics (direction, units, datum) ✅ *SimplePlot uses depth as Y-axis*
- [x] Cross-view selection highlighting ✅ *selection.ts: shared selection state*

---

## 8. MVP UDF Set (Boring on Purpose)

### 8.1 Core Provider UDFs

```rust
// core_provider.rs

pub struct CoreProvider;

impl UdfProvider for CoreProvider {
    fn id(&self) -> &str { "core" }
    fn name(&self) -> &str { "Core Processing" }

    fn load_udfs(&self) -> Vec<Box<dyn Udf>> {
        vec![
            Box::new(MovingAverageUdf::new()),
            Box::new(LinearScaleUdf::new()),
            Box::new(DepthResampleUdf::new()),
        ]
    }
}
```

### 8.2 Petrophysics Provider UDFs

```rust
// petro_provider.rs

pub struct PetrophysicsProvider;

impl UdfProvider for PetrophysicsProvider {
    fn id(&self) -> &str { "petro" }
    fn name(&self) -> &str { "Petrophysics" }

    fn load_udfs(&self) -> Vec<Box<dyn Udf>> {
        vec![
            Box::new(VShaleLinearUdf::new()),
            // Future: VShaleClavier, PorosityDensity, etc.
        ]
    }
}
```

### 8.3 First UDF: VShale Linear

Complete implementation following the UDF contract:

```rust
pub struct VShaleLinearUdf;

impl Udf for VShaleLinearUdf {
    fn id(&self) -> &str { "vshale_linear" }

    fn metadata(&self) -> UdfMetadata {
        UdfMetadata {
            id: "vshale_linear".to_string(),
            name: "V-Shale (Linear)".to_string(),
            description: "Vsh = (GR - GRclean) / (GRshale - GRclean)".to_string(),
            category: "petrophysics".to_string(),
            version: "1.0.0".to_string(),
            tags: vec!["vshale", "gamma-ray", "shale-volume"],
        }
    }

    fn parameter_definitions(&self) -> Vec<Box<dyn ParameterDefinition>> {
        vec![
            Box::new(CurveParameter {
                name: "gr".to_string(),
                description: "Gamma Ray log (API units)".to_string(),
                required: true,
                allowed_types: vec![CurveDataType::Float64, CurveDataType::Float32],
                min_length: Some(10),
                allow_nulls: true,
            }),
            Box::new(NumericParameter {
                name: "gr_clean".to_string(),
                description: "GR value for clean sand (API)".to_string(),
                required: true,
                default: Some(20.0),
                min_value: Some(0.0),
                max_value: Some(500.0),
            }),
            Box::new(NumericParameter {
                name: "gr_shale".to_string(),
                description: "GR value for 100% shale (API)".to_string(),
                required: true,
                default: Some(120.0),
                min_value: Some(0.0),
                max_value: Some(500.0),
            }),
            Box::new(BooleanParameter {
                name: "clamp_output".to_string(),
                description: "Clamp output to 0-1 range".to_string(),
                default: Some(true),
            }),
        ]
    }

    fn output_schema(&self) -> OutputSchema {
        OutputSchema {
            outputs: vec![OutputDefinition {
                name: "vshale".to_string(),
                description: "Volume of shale (0-1 fractional)".to_string(),
                data_type: CurveDataType::Float64,
            }],
        }
    }

    fn check_parameters(&self, context: &ExecutionContext) -> Result<(), Vec<ValidationError>> {
        let gr_clean = context.parameter_as_float("gr_clean")?;
        let gr_shale = context.parameter_as_float("gr_shale")?;

        if gr_shale <= gr_clean {
            return Err(vec![ValidationError::Custom {
                parameter: "gr_shale".to_string(),
                message: format!(
                    "GR Shale ({:.1}) must be greater than GR Clean ({:.1})",
                    gr_shale, gr_clean
                ),
            }]);
        }

        Ok(())
    }

    fn execute(&self, context: &ExecutionContext) -> Result<UdfOutput, UdfError> {
        let gr_curve = context.parameter_as_curve("gr")?;
        let gr_clean = context.parameter_as_float("gr_clean")?;
        let gr_shale = context.parameter_as_float("gr_shale")?;
        let clamp = context.parameter_as_bool("clamp_output").unwrap_or(true);

        let gr_data = gr_curve.as_f64_slice()?;
        let depths = gr_curve.depths();

        let vshale: Vec<Option<f64>> = gr_data.iter().enumerate()
            .map(|(i, gr_opt)| {
                if i % 1000 == 0 {
                    context.set_progress((i as f64 / gr_data.len() as f64) * 100.0);
                    if context.is_cancelled() {
                        return None; // Will be handled in error path
                    }
                }

                gr_opt.map(|gr| {
                    let vsh = (gr - gr_clean) / (gr_shale - gr_clean);
                    if clamp { vsh.clamp(0.0, 1.0) } else { vsh }
                })
            })
            .collect();

        Ok(UdfOutput::new()
            .with_curve("vshale", CurveData::new(depths.to_vec(), vshale)))
    }
}
```

### Shopping List

- [x] MovingAverageUdf implementation ✅ *providers/core.rs: MovingAverageUdf*
- [x] LinearScaleUdf implementation ✅ *providers/core.rs: LinearScaleUdf*
- [x] DepthResampleUdf implementation ✅ *providers/core.rs: DepthResampleUdf*
- [x] VShaleLinearUdf implementation ✅ *providers/vshale.rs: VShaleLinearUdf*
- [ ] Golden dataset for regression tests — *Future: test fixtures*
- [ ] Snapshot tests (hash/row count/statistics) — *Future: CI integration*

---

## 9. Data Integrity and Concurrency

### 9.1 Integrity Rules

- **Raw artifacts are immutable** — Never modified after import
- **Derived artifacts are immutable** — Append-only with provenance
- **Only metadata may be edited** — And edits are versioned

### 9.2 Concurrency Model

- **Single-writer principle** for workspace DB
- **UI can read concurrently**
- **Executions serialize writes** (commit at end)

### 9.3 Failure Handling

- **Execution failure** → No artifact created, no side effects
- **Crash recovery** → Clean up temp outputs on next launch
- **Dangling runs** → Detect and mark as failed on restart

### Shopping List

- [x] Workspace locking strategy (file lock / SQLite busy timeout) ✅ *Read-only mode avoids write conflicts*
- [ ] Crash recovery: cleanup temp outputs — *Post-MVP hardening*
- [ ] Validation of incomplete runs on startup — *Post-MVP hardening*
- [x] Transaction isolation for atomic commits ✅ *Single execution per command*

---

## 10. Testing Strategy

### 10.1 Test Types

| Type | Purpose | Example |
|------|---------|---------|
| **Contract Tests** | Compute ↔ DataForge APIs | Read curve, register execution |
| **UDF Tests** | Golden inputs → deterministic outputs | VShale with known GR curve |
| **State Propagation** | Run → artifact appears → views update | Execute → verify event flow |
| **Failure Tests** | UDF failure → no artifact created | Invalid params → clean abort |
| **Integration** | End-to-end golden path | Load → execute → view provenance |

### 10.2 Golden Dataset

```
test_fixtures/
├── well_A/
│   ├── GR.parquet          # Test gamma ray curve
│   ├── RHOB.parquet        # Test density curve
│   └── metadata.json       # Curve metadata
└── expected_outputs/
    ├── vshale_linear.parquet
    └── moving_average_5.parquet
```

### Shopping List

- [ ] Golden dataset fixtures in repo — *Future: CI/CD*
- [ ] Snapshot tests for UDF outputs — *Future: CI/CD*
- [ ] End-to-end test: load → execute → view → verify provenance — *Future: CI/CD*
- [x] Structured logging + per-run logs ✅ *tauri-plugin-log with Info level*

---

## 11. Implementation Milestones

### Milestone A: Foundation (Contract + Domain Model) ✅ COMPLETE

**Goal:** Read artifacts from DataForge, register a Run and output artifact

**Deliverables:**
- [x] DataForge read APIs (list curves, load data) ✅ *commands.rs*
- [x] Execution record schema ✅ *types.rs: ExecutionRecord*
- [x] Atomic commit strategy ✅ *output_writer.rs*
- [x] Basic Tauri command: `execute_udf` ✅ *commands.rs*

**Exit Criteria:** ✅ Can load a curve and create a dummy derived artifact with provenance

### Milestone B: UDF Framework + First UDF ✅ COMPLETE

**Goal:** One UDF with full validation, execution, provenance, artifact output

**Deliverables:**
- [x] `UdfProvider` and `Udf` traits ✅ *providers/mod.rs*
- [x] `UdfRegistry` with registration ✅ *registry.rs*
- [x] `ExecutionEngine` with multi-stage validation ✅ *engine.rs*
- [x] Parameter definitions ✅ *parameters.rs*
- [x] VShaleLinearUdf implementation ✅ *providers/vshale.rs*
- [x] Simple parameter UI generation ✅ *ParameterForm.svelte*

**Exit Criteria:** ✅ Can run VShale on a GR curve, view result with provenance

### Milestone C: Multi-View Workbench ✅ COMPLETE

**Goal:** Same artifact visible in multiple views consistently, event-driven updates

**Deliverables:**
- [x] `DataStore` with event emission ✅ *stores/dataStore.ts*
- [x] `DataViewBridge` ✅ *stores/events.ts*
- [x] `ViewManager` with capability routing ✅ *+page.svelte tabbed views*
- [x] Artifact Inspector view ✅ *ArtifactInspector.svelte*
- [x] Coverage View ✅ *CoverageView.svelte*
- [x] Minimal Plot View ✅ *SimplePlot.svelte*
- [x] Selection synchronization ✅ *selection.ts*

**Exit Criteria:** ✅ Executing a UDF updates all relevant views automatically

### Milestone D: UDF Toolbox + Hardening ✅ COMPLETE (core items)

**Goal:** 2-3 additional UDFs, robust failure handling, demo-ready

**Deliverables:**
- [x] MovingAverageUdf ✅ *providers/core.rs*
- [x] LinearScaleUdf ✅ *providers/core.rs*
- [x] DepthResampleUdf ✅ *providers/core.rs*
- [x] Cancellation support ✅ *context.rs: CancellationToken*
- [x] Progress reporting ✅ *context.rs: ProgressState*
- [ ] Crash recovery — *Post-MVP hardening*
- [x] Provenance panel ✅ *ProvenancePanel.svelte integrated into +page.svelte*
- [ ] Demo script + QA checklist — *Documentation task*

**Exit Criteria:** ✅ Complete golden path demo: load → select curves → configure → execute → view results with full provenance

---

## 12. Design Review Checklist (Before Each Milestone)

- [x] Does this introduce silent recompute? ✅ *No — explicit Execute button only*
- [x] Does any view start owning data? ✅ *No — views observe through stores*
- [x] Can a user explain provenance of every derived artifact? ✅ *ExecutionRecord tracks inputs/params*
- [x] Are outputs append-only with clear lineage? ✅ *Immutable by design*
- [x] Can we add a new UDF without changing existing UDFs? ✅ *Provider pattern*
- [x] Are run records backwards compatible? ✅ *JSON serialization*
- [x] Are inputs immutable (`Arc<CurveData>`)? ✅ *Arc<CurveData> in context*
- [x] Is there an exception boundary around UDF execution? ✅ *engine.rs: catch_unwind*

---

## 13. Extensibility Notes (Design Now, Implement Later)

### 13.1 Plugin System (Future)

UDFs shipped as:
- Built-in Rust UDFs (MVP)
- Dynamic plugins via manifest (post-MVP)

Plugin manifest format prepared:
```toml
[plugin]
id = "mycompany_petro"
version = "1.0.0"
dataforge_compute_version = ">=0.5.0"

[provider]
id = "mycompany"
name = "MyCompany Tools"

[[udfs]]
id = "custom_vshale"
name = "Custom V-Shale"
category = "petrophysics"
```

### 13.2 Python UDF Bridge (Future)

For domain experts who prefer Python:
- PyO3 bridge to Rust
- Python UDFs follow same contract
- Same sandboxing and provenance

### Shopping List (Post-MVP)

- [ ] Plugin manifest format
- [ ] Plugin discovery and loading
- [ ] UDF versioning strategy
- [ ] Backward compatibility policy
- [ ] Python UDF bridge design

---

## References

- [DFC-extensibility-contract-udfs.md](../design-concepts/DF-Compute/DFC-extensibility-contract-udfs.md) — UDF provider architecture
- [DFC-model-view-sync.md](../design-concepts/DF-Compute/DFC-model-view-sync.md) — Multi-view synchronization
- [DFC-udf-type-safety.md](../design-concepts/DF-Compute/DFC-udf-type-safety.md) — Type safety and validation
- [DFC-Recomputations-trigger.md](../design-concepts/DF-Compute/DFC-Recomputations-trigger.md) — Explicit execution model
- [DF-unit-of-truth.md](../design-concepts/DF-unit-of-truth.md) — Curve as unit of truth
- [DF-mutability.md](../design-concepts/DF-mutability.md) — Immutable data, mutable metadata
- [DataForge_Compute_MVP.md](./DataForge_Compute_MVP.md) — MVP scope and principles

---

*End of plan.*
