# Polars Integration Plan (`polars.md`)

This document explains **how Polars fits into our stack** alongside **DataFusion, Arrow/Parquet, GeoArrow, Iceberg, MinIO/S3**, and the **petgraph** orchestrator.

**Positioning:**

- **Data lake engine**: **DataFusion** (queries over Parquet/Iceberg via `object_store`).
- **Geospatial**: **GeoArrow/geoarrow-rs**.
- **In-memory dataframe toolbox**: **Polars** (optional, focused on post-processing, reshaping, stats, exports).
- **Orchestration**: **petgraph**.

Polars is **not a replacement** for DataFusion here; it is a **complement** for fast, ergonomic, in-memory transformations on result sets returned by DataFusion (Arrow `RecordBatch`es).

---

## When to use Polars vs DataFusion

| Situation                                                                               | Use            |
| --------------------------------------------------------------------------------------- | -------------- |
| Scan/Filter/Join across many Parquet files (S3/MinIO)                                   | **DataFusion** |
| SQL queries for users / pushdown needed                                                 | **DataFusion** |
| Vectorized UDFs (e.g., VShale)                                                          | **DataFusion** |
| Geospatial predicates (point-in-polygon, etc.)                                          | **GeoArrow**   |
| **Post-query transforms**: reshaping, window ops, feature eng., quick stats, CSV export | **Polars**     |

**Rule:** Standardize on **Arrow** between components. DataFusion → Arrow → Polars → (optional) Arrow/Parquet back.

---

## Cargo Dependencies

```toml
# In the crate that does in-memory post-processing
[dependencies]
polars = { version = "0.43", features = ["lazy", "ipc", "parquet", "dtype-full"] }
# If writing CSV for user export
polars-io = { version = "0.43", features = ["csv", "ipc", "parquet"] }
# Arrow & DataFusion are in other crates; Arrow is the interchange
arrow = "51"
datafusion = { version = "41", features = ["parquet"] }
```

_(Use current compatible versions in your workspace.)_

---

## Interop Patterns

### A) DataFusion ➜ Arrow ➜ Polars (recommended)

1. Run DF query and collect Arrow batches:

```rust
let batches: Vec<arrow::record_batch::RecordBatch> = df.collect().await?;
```

2. Convert to Polars:

- **Option 1: IPC round-trip (robust across versions)**

```rust
use arrow::ipc::writer::FileWriter;

let mut buf = Vec::new();
{
  let schema = batches[0].schema();
  let mut w = FileWriter::try_new(&mut buf, &schema)?;
  for b in &batches { w.write(b)?; }
  w.finish()?;
}

let cursor = std::io::Cursor::new(buf);
let pdf = polars::io::ipc::read_ipc(cursor)?; // -> Polars DataFrame
```

- **Option 2: Parquet round-trip** (good when sizes are large and you want a persisted artifact)

```rust
// Write batches to Parquet (in-memory or temp file) then read with Polars
```

> Why IPC? It keeps data Arrow-native and avoids type edge cases. Direct zero-copy Arrow <-> Polars is improving but IPC is reliable today.

### B) Polars ➜ Arrow/DataFusion (if needed)

- Write Polars DF to Arrow IPC or Parquet and then register as a DataFusion MemTable or read as Parquet.

```rust
// Polars -> IPC (to bytes), then Arrow FileReader -> RecordBatches -> MemTable in DF
```

---

## Common Post-Processing with Polars

### 1) Column transforms & expressions

```rust
use polars::prelude::*;

let mut df = pdf;

// Example: normalize `value`, build a flag, and compute rolling mean
df = df
  .lazy()
  .with_columns([
      ((col("value") - col("value").mean()) / col("value").std()).alias("value_z"),
      (when(col("value").gt(lit(150.0))).then(lit(1)).otherwise(lit(0))).alias("gr_flag"),
  ])
  .groupby([col("well_id"), col("curve")])
  .agg([
      col("value").rolling_mean(RollingOptionsFixedWindow {
          window_size: 25,
          min_periods: 1,
          ..Default::default()
      }).alias("value_rm25")
  ])
  .collect()?;
```

### 2) Pivot/reshape

```rust
let pivoted = pdf
  .lazy()
  .groupby([col("well_id"), col("depth")])
  .pivot(col("curve"), PivotAgg::First)
  .agg([col("value")])
  .collect()?;
```

### 3) Window functions

```rust
let out = pdf
  .lazy()
  .with_columns([
    col("value").over([col("well_id")]).rank(RankOptions::default()).alias("rank_in_well"),
  ])
  .collect()?;
```

### 4) Quick stats / profiling

```rust
println!("{:?}", pdf.describe(None)?);
```

### 5) Export to CSV/Parquet for users

```rust
use polars::prelude::*;
use std::fs::File;

let mut file = File::create("out.csv")?;
CsvWriter::new(&mut file).finish(&pdf)?;

let mut pf = File::create("out.parquet")?;
ParquetWriter::new(&mut pf).finish(&pdf)?;
```

---

## Integration in Our Architecture

### Petgraph node examples

- `DfQuery` → outputs Arrow batches
- `ToPolars` → converts batches to Polars DF (optional node)
- `PolarsTransform` → runs a list of Polars expressions / transforms
- `ExportCsv` / `ExportParquet` → user downloads
- Back to DF (optional): `PolarsToArrow` → Arrow batches

### Suggested module boundaries

```
parquet-query-engine/   # DataFusion
polars-toolbox/         # Polars transforms & exports (optional crate)
storage-resolvers/      # URIs & object_store/OpenDAL
domain-queries/         # high-level flows
app/                    # Tauri frontend
```

`polars-toolbox` exposes reusable functions:

```rust
pub fn batches_to_polars(batches: &[RecordBatch]) -> anyhow::Result<polars::prelude::DataFrame> { /* IPC */ }
pub fn polars_to_parquet(df: &DataFrame) -> anyhow::Result<Vec<u8>> { /* write to bytes */ }
pub fn basic_stats(df: &DataFrame) -> anyhow::Result<DataFrame> { /* describe-like */ }
pub fn pivot_curves_long_to_wide(df: &DataFrame) -> anyhow::Result<DataFrame> { /* example */ }
```

---

## Performance & Memory Tips

- Prefer **lazy** Polars pipelines for complex transforms (query optimizer + predicate pushdown inside Polars).
- Avoid row-wise closures (`apply` with per-row lambdas) in hot paths; use Polars expressions instead.
- For very large results, keep heavy compute in **DataFusion**; use Polars on **downstream subsets** or aggregates.
- IPC vs Parquet interchange:
  - **IPC** is fastest in-memory; **Parquet** is best if you want a persisted artifact anyway.
- In Tauri, write exported CSV/Parquet to app’s data dir and return a download link to the UI.

---

## Geospatial Note

Polars doesn’t natively operate on GeoArrow geometry arrays. Keep geo ops in **geoarrow-rs** and pass attributes/geometries back to Polars only when needed for table ops or exports. Use Arrow IPC/Parquet as the bridge.

---

## Testing

- Add **golden tests** for Polars transforms on small fixtures.
- Cross-check a few transforms by comparing: `DataFusion → Arrow → Polars` vs a pure `DataFusion` equivalent to catch semantic drift.
- Validate dtypes and null handling after conversions (especially for timestamps/decimals).

---

## Example: End-to-end snippet

```rust
// 1) Run DataFusion query (logs sliced and vshale computed), collect Arrow batches.
let batches = df.collect().await?;

// 2) Convert to Polars
let pdf = batches_to_polars(&batches)?;

// 3) Polars post-processing (e.g., pivot to wide format per depth)
let wide = pdf
  .lazy()
  .groupby([col("well_id"), col("depth")])
  .pivot(col("curve"), PivotAgg::First)
  .agg([col("vsh")])
  .collect()?;

// 4) Export for user
let bytes = polars_to_parquet(&wide)?;
// upload via object_store/OpenDAL or save locally for download
```

---

## TL;DR

- Keep **DataFusion** as the lake query engine; use **Polars** for ergonomic in-memory transforms, stats, reshaping, and exports.
- Use **Arrow IPC** (or Parquet) as the interchange format.
- Keep geospatial in **GeoArrow**, and integrate via Arrow buffers.
- Model the optional Polars steps as nodes in your petgraph DAG for clarity, caching, and observability.
