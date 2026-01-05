# 🦀 Modular Query Flow for Petrophysical Logs (Rust + DataFusion)

This document outlines a **simple modular flow** for querying Parquet log files dynamically,
where each file may contain different sets of curves (e.g., some wells have GR + RES, others only GR).
The goal is to support flexible, user-driven queries without hard-coded assumptions.

---

## 1. Storage Layer: Dynamic File Resolution

Define **StorageResolvers** for different data types (logs, tops, trajectories, etc.).
Each resolver knows how to translate a **logical name** (like "wellA") into one or more URIs
(local files, MinIO/S3, etc.).

```rust
pub trait StorageResolver {
    fn resolve_selection(&self, sel: FileSelection) -> anyhow::Result<Vec<Url>>;
}

pub struct FileSelection {
    pub names: Vec<String>,
    pub prefixes: Vec<String>,
    pub explicit_uris: Vec<Url>,
}
```

Examples:

- `LogsResolver` for petrophysical logs
- `TopsResolver` for well tops
- `TrajResolver` for trajectories

---

## 2. Catalog Layer: Field Mapping

Different datasets may label columns differently (`WELL`, `well_name`, etc.).
A **FieldMap** provides a semantic mapping:

```rust
pub struct FieldMap {
    pub well: String,
    pub curve: String,
    pub depth: String,
    pub value: String,
}
```

Each dataset (logs, tops, etc.) defines its own FieldMap.

---

## 3. Engine Context: Registering Tables

Use DataFusion’s **listing tables** to treat many Parquet files as **one logical table**.

```rust
pub struct EngineContext {
    pub ctx: SessionContext,
}

impl EngineContext {
    pub fn new() -> Self {
        Self { ctx: SessionContext::new() }
    }

    pub async fn register_listing_table(
        &self,
        uris: &[Url],
        temp_name: &str,
    ) -> datafusion::error::Result<()> {
        let format = Arc::new(ParquetFormat::default());
        let opts = ListingOptions::new(format);
        let mut cfg = ListingTableConfig::new_with_multi_paths(uris.to_vec());
        let state = self.ctx.state();
        let cfg = cfg.with_listing_options(opts).infer(&state).await?;
        let table = ListingTable::try_new(cfg)?;
        self.ctx.register_table(temp_name, Arc::new(table))?;
        Ok(())
    }
}
```

---

## 4. Query Criteria & Predicate Builders

Define **query criteria** with flexible filters:

```rust
pub enum CurveFilter {
    AnyCurveIn(Vec<String>),
    CurveValueRange { curve: String, min: f64, max: f64 },
    CurveValueGte { curve: String, min: f64 },
    CurveValueLte { curve: String, max: f64 },
}

pub struct QueryCriteria {
    pub wells: Vec<String>,
    pub depth_min: Option<f64>,
    pub depth_max: Option<f64>,
    pub curve_filters: Vec<CurveFilter>,
}
```

Predicate builder turns this into a DataFusion `Expr`.

---

## 5. Building & Running a Query

```rust
pub async fn run_depth_slice<R: StorageResolver>(
    resolver: &R,
    dataset: &Dataset,
    criteria: &QueryCriteria,
) -> anyhow::Result<Vec<RecordBatch>> {
    // 1) Resolve URIs
    let uris = resolver.resolve_selection(FileSelection {
        names: criteria.wells.clone(),
        prefixes: vec![],
        explicit_uris: vec![],
    })?;

    // 2) Register a temp table
    let engine = EngineContext::new();
    let temp = "logs_tmp";
    engine.register_listing_table(&uris, temp).await?;

    // 3) Build predicates
    let fm = &dataset.field_map;
    let mut pred = lit(true);

    if !criteria.wells.is_empty() {
        let wells = criteria.wells.iter().cloned().map(lit).collect::<Vec<_>>();
        pred = pred.and(col(&fm.well).in_list(wells, false));
    }
    if let Some(min) = criteria.depth_min {
        pred = pred.and(col(&fm.depth).gt_eq(lit(min)));
    }
    if let Some(max) = criteria.depth_max {
        pred = pred.and(col(&fm.depth).lt_eq(lit(max)));
    }

    let mut curve_pred = lit(false);
    for cf in &criteria.curve_filters {
        let e = match cf {
            CurveFilter::AnyCurveIn(vs) =>
                col(&fm.curve).in_list(vs.iter().cloned().map(lit).collect(), false),
            CurveFilter::CurveValueRange { curve, min, max } =>
                col(&fm.curve).eq(lit(curve.clone()))
                  .and(col(&fm.value).gt_eq(lit(*min)))
                  .and(col(&fm.value).lt_eq(lit(*max))),
            CurveFilter::CurveValueGte { curve, min } =>
                col(&fm.curve).eq(lit(curve.clone()))
                  .and(col(&fm.value).gt_eq(lit(*min))),
            CurveFilter::CurveValueLte { curve, max } =>
                col(&fm.curve).eq(lit(curve.clone()))
                  .and(col(&fm.value).lt_eq(lit(*max))),
        };
        curve_pred = curve_pred.or(e);
    }
    if !criteria.curve_filters.is_empty() {
        pred = pred.and(curve_pred);
    }

    // 4) Run query
    let df = engine.ctx.table(temp).await?
        .filter(pred)?
        .select(vec![
            col(&fm.well).alias("well"),
            col(&fm.curve).alias("curve"),
            col(&fm.depth).alias("depth"),
            col(&fm.value).alias("value"),
        ])?
        .sort(vec![
            col(&fm.well).sort(true, true),
            col(&fm.curve).sort(true, true),
            col(&fm.depth).sort(true, true),
        ])?;

    let batches = df.collect().await?;
    Ok(batches)
}
```

---

## 6. Example Usage

Request:

- Wells: `["A","B"]`
- Depth: `1000-2000`
- Curves: GR anywhere, RES between 5–200

```rust
let criteria = QueryCriteria {
    wells: vec!["A".into(), "B".into()],
    depth_min: Some(1000.0),
    depth_max: Some(2000.0),
    curve_filters: vec![
        CurveFilter::AnyCurveIn(vec!["GR".into()]),
        CurveFilter::CurveValueRange { curve: "RES".into(), min: 5.0, max: 200.0 },
    ],
};

let result_batches = run_depth_slice(&logs_resolver, &logs_dataset, &criteria).await?;
```

---

## ✅ Summary

- **StorageResolver** abstracts file resolution (local/S3/MinIO).
- **FieldMap** abstracts schema differences.
- **EngineContext** mounts multiple Parquet files into a single logical table.
- **QueryCriteria + Predicate builder** let you flexibly filter by wells, curves, depth ranges, or curve-specific value ranges.
- **DataFusion** executes efficiently with predicate & projection pushdown.

This modular flow scales to:

- More datasets (tops, trajectories)
- More flexible query shapes
- Reporting missing data per well/curve
- Adding UDFs for custom computations

---

## 📋 Progress Tracking & Next Steps

### ✅ **Completed Components**

#### **1. Storage Layer - COMPLETED** ✅

- **Storage Resolver Crate**: `crates/storage/storage-resolver/`
- **Well Path Management**: Standardized path construction (`project-{id}/wells/{well-id}/logs_{log-type}.parquet`)
- **WellStorageManager**: High-level storage operations with well-centric paths
- **MinIO/S3 Integration**: Working resolvers for S3-compatible storage
- **Path Standardization**: All LAS upload and Parquet fetch operations use consistent paths

#### **2. Schema Management - COMPLETED** ✅

- **Project Data Layout**: `crates/storage/project-data-layout/` for centralized schema management
- **Modular Architecture**: Separate files for different schema types
- **Centralized Registry**: Type-safe schema management
- **LAS Integration**: `las-to-parquet` crate uses centralized schemas
- **Rich Metadata**: Field descriptions, units, and data types

#### **3. Data Processing - COMPLETED** ✅

- **LAS to Parquet Conversion**: Working with standardized schemas
- **Tauri Integration**: LAS upload and Parquet fetch commands
- **Storage Integration**: Files stored using well-centric paths
- **Compilation**: All crates compile successfully

### ✅ **Recently Completed Components**

#### **4. Query Engine - COMPLETED** ✅

- **Parquet Log Query Engine**: `crates/query-engine/parquet-log-query-engine/`
- **DataFusion Integration**: Fully working with S3/MinIO support
- **Multi-File Querying**: Support for querying multiple Parquet files with different schemas
- **Unified Schema Queries**: Join wells with different curve sets, handling missing columns
- **Export Functionality**: Save query results to Parquet files
- **Compilation**: All DataFusion API compatibility issues resolved

#### **5. Arrow Statistics - COMPLETED** ✅

- **Arrow Statistics Crate**: `crates/core/arrow-statistics/`
- **On-the-Fly Statistics**: Min/max, mean, std dev, variance computation using DataFusion
- **Frontend-Ready Output**: JSON serialization for filter ranges
- **Real Data Testing**: Tested with actual Parquet files from multi-file queries
- **S3/MinIO Integration**: Works with remote storage for statistics computation

### 📋 **Next Priority Steps**

#### **Phase 1: Storage Resolver Integration** (Immediate - 1-2 days)

1. **Integrate Storage Resolver with Query Engine**
   - Update `parquet-log-query-engine` to use `WellStorageManager` instead of hardcoded paths
   - Replace current S3 configuration with `s3-config` crate
   - Implement dynamic file resolution for query operations
   - Test end-to-end query flow with real data

2. **Implement Field Mapping System**
   - Create `FieldMap` struct for schema abstraction across different datasets
   - Add support for different column naming conventions (well_name vs WELL, etc.)
   - Integrate with existing query engine for flexible column mapping

3. **Complete Engine Context Implementation**
   - Finish `EngineContext` with listing table registration
   - Add support for multiple Parquet files as single logical table
   - Test with real data from multiple wells

#### **Phase 2: Query Criteria & Predicate System** (Next - 2-3 days)

1. **Implement Query Criteria**
   - `QueryCriteria` struct with flexible filters
   - `CurveFilter` enum for curve-specific filtering
   - Depth range and well filtering

2. **Build Predicate Builder**
   - Convert criteria to DataFusion expressions
   - Support complex filtering logic
   - Optimize predicate pushdown

3. **Create Query Builder**
   - High-level query construction API
   - Integration with Tauri commands
   - Error handling and validation

#### **Phase 3: Frontend Integration** (Following - 3-5 days)

1. **Update Frontend Components**
   - Modify existing Parquet query components
   - Add support for new query criteria
   - Implement well-centric data browsing

2. **Add Query Interface**
   - Well selection interface
   - Depth range selection
   - Curve filtering options
   - Results visualization

#### **Phase 4: Advanced Features** (Future - 1-2 weeks)

1. **Additional Dataset Support**
   - Tops resolver and schemas
   - Trajectory resolver and schemas
   - Cross-dataset queries

2. **Performance Optimization**
   - Query result caching
   - Parallel query execution
   - Memory management

3. **Advanced Query Features**
   - Custom UDFs for petrophysical calculations
   - Aggregation queries
   - Statistical analysis

### 🎯 **Immediate Next Step Recommendation**

**Start with Phase 1, Step 1: Integrate Storage Resolver with Query Engine**

This is the logical next step because:

- We have working storage resolver, query engine, and statistics components
- Integration will enable end-to-end testing with real data
- It builds directly on our completed work
- It unblocks the remaining query system development
- It creates a complete data pipeline from storage → query → statistics

**Specific Action**: Update `parquet-log-query-engine` to use `WellStorageManager` and `s3-config` for file resolution and S3 configuration, replacing the current hardcoded approaches.

**Expected Outcome**: A complete, working system where:

1. User selects wells through Tauri UI
2. System resolves well paths using `WellStorageManager`
3. Query engine fetches and processes Parquet files
4. Statistics are computed for frontend filter ranges
5. Results are exported back to storage or displayed in UI

### 📊 **Current Architecture Status**

```
✅ Storage Layer (WellStorageManager + Paths)
✅ Schema Management (Parquet Schemas + Registry)
✅ Data Processing (LAS to Parquet + Tauri)
✅ Query Engine (DataFusion + Multi-file + Export)
✅ Statistics Engine (Arrow Statistics + Frontend JSON)
🚧 Storage Integration (Needs WellStorageManager integration)
❌ Query Criteria (Not Started)
❌ Frontend Integration (Needs Update)
```

**Overall Progress: ~75% Complete**

### 🚀 **Path to DAG Architecture**

Our current progress puts us in an excellent position to move toward the DAG architecture outlined in `rust_dag_plan.md`:

**Current State**: We have the foundational components (storage, schemas, query engine, statistics)

**Next Phase**: Complete the integration layer (storage resolver + query engine)

**Future Phases**:

- Add DAG orchestration with `petgraph`
- Implement node-based processing (each query step becomes a DAG node)
- Add caching and lineage tracking
- Introduce WASM support for custom functions
