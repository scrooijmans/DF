# 🦀 Modular Workspace Layout for Petrophysical Queries (Rust + DataFusion)

This document outlines how to structure a Rust workspace into multiple crates for querying Parquet files dynamically.
We separate **storage resolution** (finding URIs for files) from **query execution** (DataFusion/Arrow), with an optional layer for **domain-specific queries**.

## ✅ **Current Implementation Status**

**COMPLETED**: We have successfully implemented most of the components described in this document:

- ✅ **Storage Resolvers**: `crates/storage/storage-resolver/` with `WellStorageManager`
- ✅ **Query Engine**: `crates/query-engine/parquet-log-query-engine/` with DataFusion
- ✅ **S3 Configuration**: `crates/storage/s3-config/` for centralized S3/MinIO config
- ✅ **Statistics Engine**: `crates/core/arrow-statistics/` for on-the-fly statistics
- ✅ **Schema Management**: `crates/storage/project-data-layout/` for centralized schemas

**NEXT STEP**: Integrate the storage resolver with the query engine to complete the end-to-end pipeline.

---

## 📂 Workspace Layout

```
your-workspace/
├─ Cargo.toml                  # [workspace] with members
├─ storage-resolvers/          # crate: storage-resolvers
│  ├─ Cargo.toml
│  └─ src/
│     ├─ lib.rs
│     ├─ traits.rs
│     ├─ types.rs
│     ├─ registry.rs
│     └─ resolvers/
│        ├─ logs.rs
│        ├─ tops.rs
│        └─ trajectories.rs
├─ parquet-query-engine/       # crate: parquet-query-engine (DataFusion)
│  ├─ Cargo.toml
│  └─ src/
│     ├─ lib.rs
│     ├─ engine/
│     │  ├─ mod.rs
│     │  ├─ context.rs
│     │  └─ listing.rs
│     ├─ catalog/
│     │  ├─ mod.rs
│     │  └─ field_map.rs
│     ├─ query/
│     │  ├─ mod.rs
│     │  ├─ criteria.rs
│     │  ├─ predicates.rs
│     │  └─ builder.rs
│     ├─ exec.rs
│     └─ types.rs
├─ domain-queries/             # optional crate: domain-queries (high-level templates)
│  ├─ Cargo.toml
│  └─ src/
│     ├─ lib.rs
│     └─ depth_slice.rs
└─ app/                        # your Tauri or CLI that uses the crates
   ├─ Cargo.toml
   └─ src/main.rs
```

---

## 🗂️ Top-level `Cargo.toml`

```toml
[workspace]
members = [
  "storage-resolvers",
  "parquet-query-engine",
  "domain-queries",   # optional; comment out if not used yet
  "app"
]
resolver = "2"
```

---

## 📦 Crate: `storage-resolvers`

Purpose: **map logical selections to URIs** (local FS, MinIO/S3, etc).  
This crate does not depend on DataFusion.

### `Cargo.toml`

```toml
[package]
name = "storage-resolvers"
version = "0.1.0"
edition = "2021"

[dependencies]
anyhow = "1"
url = "2"

# Optional, behind features
object_store = { version = "0.9", optional = true }  # S3/MinIO etc.
opendal = { version = "0.46", optional = true }      # unified backends

[features]
s3 = ["object_store"]
opendal = ["dep:opendal"]
```

### `src/traits.rs`

```rust
use crate::types::{FileSelection};
use url::Url;

#[async_trait::async_trait]
pub trait StorageResolver: Send + Sync {
    /// Resolve a selection (names/prefixes/explicit URIs) to concrete file URLs.
    async fn resolve_selection(&self, sel: FileSelection) -> anyhow::Result<Vec<Url>>;
}
```

### `src/types.rs`

```rust
use url::Url;

#[derive(Clone, Debug, Default)]
pub struct FileSelection {
    pub names: Vec<String>,        // e.g. well logical names
    pub prefixes: Vec<String>,     // dir/prefix filters
    pub explicit_uris: Vec<Url>,   // fully specified
}

#[derive(Clone, Debug)]
pub enum DatasetKind {
    Logs,
    Tops,
    Trajectories,
}

#[derive(Clone, Debug)]
pub struct DatasetDescriptor {
    pub kind: DatasetKind,
    pub name: String,            // logical dataset name
    pub base_uri: String,        // e.g. "s3://bucket/petro/"
}
```

### `src/registry.rs`

```rust
use std::collections::HashMap;
use std::sync::Arc;
use crate::traits::StorageResolver;

pub struct ResolverRegistry {
    inner: HashMap<String, Arc<dyn StorageResolver>>,
}

impl ResolverRegistry {
    pub fn new() -> Self { Self { inner: HashMap::new() } }
    pub fn register(&mut self, key: &str, resolver: Arc<dyn StorageResolver>) {
        self.inner.insert(key.to_string(), resolver);
    }
    pub fn get(&self, key: &str) -> Option<Arc<dyn StorageResolver>> {
        self.inner.get(key).cloned()
    }
}
```

### `src/resolvers/logs.rs` (example)

```rust
use std::sync::Arc;
use url::Url;
use crate::traits::StorageResolver;
use crate::types::FileSelection;

pub struct LogsResolver {
    pub base_uri: String, // e.g. "s3://mybucket/logs/"
}

#[async_trait::async_trait]
impl StorageResolver for LogsResolver {
    async fn resolve_selection(&self, sel: FileSelection) -> anyhow::Result<Vec<Url>> {
        let mut out = vec![];
        for name in sel.names {
            // Example mapping rule:
            let uri = format!("{}{}.parquet", self.base_uri, name);
            out.push(Url::parse(&uri)?);
        }
        for uri in sel.explicit_uris {
            out.push(uri);
        }
        Ok(out)
    }
}
```

---

## 📦 Crate: `parquet-query-engine`

Purpose: **DataFusion orchestration**.  
It knows how to run queries against Parquet files, but not how URIs are discovered.

### `Cargo.toml`

```toml
[package]
name = "parquet-query-engine"
version = "0.1.0"
edition = "2021"

[dependencies]
anyhow = "1"
datafusion = { version = "41", features = ["parquet"] }
arrow = "51"
parquet = "51"
url = "2"
tokio = { version = "1", features = ["rt-multi-thread", "macros"] }
```

### `src/engine/context.rs`

```rust
use datafusion::prelude::SessionContext;

pub struct EngineContext {
    pub ctx: SessionContext,
}

impl EngineContext {
    pub fn new() -> Self {
        Self { ctx: SessionContext::new() }
    }
}
```

### `src/engine/listing.rs`

```rust
use std::sync::Arc;
use url::Url;
use datafusion::prelude::*;
use datafusion::datasource::listing::{ListingTable, ListingTableConfig, ListingOptions};
use datafusion::datasource::file_format::parquet::ParquetFormat;

pub async fn register_listing_table(
    ctx: &SessionContext,
    uris: &[Url],
    temp_name: &str,
) -> datafusion::error::Result<()> {
    let format = Arc::new(ParquetFormat::default());
    let opts = ListingOptions::new(format);
    let mut cfg = ListingTableConfig::new_with_multi_paths(uris.to_vec());
    let state = ctx.state();
    let cfg = cfg.with_listing_options(opts).infer(&state).await?;
    let table = ListingTable::try_new(cfg)?;
    ctx.register_table(temp_name, Arc::new(table))?;
    Ok(())
}
```

### `src/catalog/field_map.rs`

```rust
#[derive(Clone, Debug)]
pub struct FieldMap {
    pub well: String,
    pub curve: String,
    pub depth: String,
    pub value: String,
}

impl Default for FieldMap {
    fn default() -> Self {
        Self {
            well: "well".into(),
            curve: "curve".into(),
            depth: "depth".into(),
            value: "value".into(),
        }
    }
}
```

### `src/query/criteria.rs`

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

### `src/query/predicates.rs`

```rust
use datafusion::prelude::*;
use crate::catalog::field_map::FieldMap;
use super::criteria::{QueryCriteria, CurveFilter};

pub fn build_predicate(fm: &FieldMap, c: &QueryCriteria) -> Expr {
    let mut pred = lit(true);

    if !c.wells.is_empty() {
        let wells = c.wells.iter().cloned().map(lit).collect::<Vec<_>>();
        pred = pred.and(col(&fm.well).in_list(wells, false));
    }
    if let Some(min) = c.depth_min {
        pred = pred.and(col(&fm.depth).gt_eq(lit(min)));
    }
    if let Some(max) = c.depth_max {
        pred = pred.and(col(&fm.depth).lt_eq(lit(max)));
    }

    if !c.curve_filters.is_empty() {
        let mut curve_pred = lit(false);
        for cf in &c.curve_filters {
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
        pred = pred.and(curve_pred);
    }
    pred
}
```

### `src/exec.rs`

```rust
use datafusion::arrow::record_batch::RecordBatch;
use datafusion::prelude::*;
use crate::catalog::field_map::FieldMap;
use crate::engine::{context::EngineContext, listing::register_listing_table};
use crate::query::{criteria::QueryCriteria, predicates::build_predicate};
use url::Url;

pub async fn run_query(
    uris: &[Url],
    fm: &FieldMap,
    criteria: &QueryCriteria,
) -> anyhow::Result<Vec<RecordBatch>> {
    let engine = EngineContext::new();
    let temp = "tmp_logs";
    register_listing_table(&engine.ctx, uris, temp).await?;

    let pred = build_predicate(fm, criteria);
    let df = engine.ctx
        .table(temp).await?
        .filter(pred)?
        .select(vec![col(&fm.well), col(&fm.curve), col(&fm.depth), col(&fm.value)])?
        .sort(vec![
            col(&fm.well).sort(true, true),
            col(&fm.curve).sort(true, true),
            col(&fm.depth).sort(true, true),
        ])?;

    Ok(df.collect().await?)
}
```

---

## 📦 Crate: `domain-queries` (optional)

Purpose: reusable **templates** (depth slice, curve stats, joins…), built _on top_ of `parquet-query-engine` and consuming `storage-resolvers`.

### `src/depth_slice.rs`

```rust
use storage_resolvers::{traits::StorageResolver, types::FileSelection};
use parquet_query_engine::{exec::run_query, catalog::field_map::FieldMap, query::criteria::*};
use datafusion::arrow::record_batch::RecordBatch;

pub async fn depth_slice<R: StorageResolver>(
    resolver: &R,
    field_map: &FieldMap,
    wells: Vec<String>,
    curve_filters: Vec<CurveFilter>,
    depth_min: Option<f64>,
    depth_max: Option<f64>,
) -> anyhow::Result<Vec<RecordBatch>> {
    let uris = resolver.resolve_selection(FileSelection { names: wells.clone(), ..Default::default() }).await?;
    let crit = QueryCriteria { wells, depth_min, depth_max, curve_filters };
    run_query(&uris, field_map, &crit).await
}
```

---

## ✅ Summary

- **`storage-resolvers`**: logical → physical file mapping (handles MinIO/S3, FS).
- **`parquet-query-engine`**: orchestrates DataFusion queries on Parquet.
- **`domain-queries`**: optional higher-level query templates.
- **`app`**: Tauri or CLI frontend consuming the crates.

This separation keeps your system **modular** but still lean for incremental development.
