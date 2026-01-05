# Plan: Using `object_store` in Our Architecture

This document explains **how** to use the Rust `object_store` crate as the primary data-plane abstraction in our stack (MinIO/S3 + Arrow/Parquet + DataFusion + Iceberg/GeoParquet), why it fits well, and how it coexists with OpenDAL and aws-sdk-s3.

---

## Why `object_store`?

- **Native with DataFusion**: DataFusion uses `object_store` internally. Registering an S3/MinIO store on the DF runtime enables direct Parquet scans with predicate/projection pushdown.
- **Portability across backends**: single API for S3/MinIO, Azure Blob, GCS, local FS, HTTP.
- **Parquet-friendly I/O**: streaming + ranged GET (critical for Parquet footers and selective row-group reads).
- **Ecosystem alignment**: widely adopted in Rust data engines (DataFusion/Ballista; many lakehouse libs).

We’ll use it as our **default storage layer** for reads/writes, especially for DataFusion-powered queries.

---

## Where it fits in our system

```
Tauri UI
   │
   ├─► StorageResolver (URIs, catalog) ──► object_store (read/list/write)
   │
   ├─► DataFusion SessionContext ──► register object_store (S3/MinIO)
   │          └─► register_parquet / ListingTable over s3:// paths
   │
   ├─► GeoArrow ops (geoarrow-rs) ──► read GeoParquet via object_store (bytes/streams)
   │
   └─► Writers/Exports ──► ArrowWriter → Parquet → object_store.put()
```

- **Reads**: DataFusion pulls Parquet directly using `object_store` (pushdown). GeoArrow utilities can read GeoParquet by fetching bytes via `object_store`.
- **Writes**: We write Parquet results back through `object_store` (streaming uploads).

---

## Compatibility Snapshot

| Component                  | `object_store` fit | Notes                                                                                    |
| -------------------------- | ------------------ | ---------------------------------------------------------------------------------------- |
| **DataFusion**             | ✅ Native          | Register S3/MinIO store on runtime for seamless `s3://` reads.                           |
| **Arrow / Parquet**        | ✅ Indirect        | We stream bytes for readers/writers; Arrow is in-memory only.                            |
| **Iceberg (Rust)**         | ✅ Common          | Many Iceberg bindings integrate or can be layered on top; catalog configured separately. |
| **GeoArrow / geoarrow-rs** | ✅ Agnostic        | Read GeoParquet from bytes/streams fetched via `object_store`.                           |
| **DuckDB**                 | ◻️ Separate        | DuckDB has its own httpfs/S3 config; not via `object_store`.                             |
| **aws-sdk-s3**             | ◻️ Control-plane   | Keep for presign, lifecycle, IAM-ish ops not covered by data-plane I/O.                  |
| **OpenDAL**                | ◻️ Optional        | Can be used in app-layer for extra layers (retry/cache/metrics).                         |

---

## Minimal Integration: DataFusion + MinIO via `object_store`

```rust
use std::sync::Arc;
use datafusion::prelude::*;
use datafusion::datasource::object_store::ObjectStoreUrl;
use object_store::aws::AmazonS3Builder;

#[tokio::main]
async fn main() -> datafusion::error::Result<()> {
    // 1) DataFusion context
    let ctx = SessionContext::new();

    // 2) Register MinIO (S3-compatible)
    let s3 = AmazonS3Builder::new()
        .with_endpoint("http://localhost:9000")
        .with_allow_http(true)
        .with_region("us-east-1")
        .with_access_key_id("minioadmin")
        .with_secret_access_key("minioadmin")
        .build()
        .expect("s3 store");

    ctx.runtime_env().register_object_store(
        ObjectStoreUrl::parse("s3://")?,
        Arc::new(s3)
    );

    // 3) Query Parquet directly from MinIO
    ctx.register_parquet("logs", "s3://mybucket/projects/123/logs/", ParquetReadOptions::default()).await?;

    let df = ctx.sql(r#"
        SELECT well_id, depth, value
        FROM logs
        WHERE curve = 'GR' AND depth BETWEEN 1000 AND 2000
    "#).await?;

    let batches = df.collect().await?;
    println!("{}", arrow::util::pretty::pretty_format_batches(&batches).unwrap());
    Ok(())
}
```

---

## StorageResolver with `object_store`

We keep resolvers isolated from engine code. Resolvers turn **logical selections** into **URIs** and can read/write via `object_store`.

```rust
use std::sync::Arc;
use object_store::{ObjectStore, path::Path};
use bytes::Bytes;

#[async_trait::async_trait]
pub trait Blob {
    async fn get(&self, path: &str) -> anyhow::Result<Bytes>;
    async fn put(&self, path: &str, data: Bytes) -> anyhow::Result<()>;
    async fn list(&self, prefix: &str) -> anyhow::Result<Vec<String>>;
}

// object_store-backed impl
pub struct ObjStoreBlob {
    base: String,
    store: Arc<dyn ObjectStore>,
}

#[async_trait::async_trait]
impl Blob for ObjStoreBlob {
    async fn get(&self, path: &str) -> anyhow::Result<Bytes> {
        let p = Path::from(format!("{}{}", self.base, path));
        let resp = self.store.get(&p).await?;
        Ok(resp.bytes().await?)
    }
    async fn put(&self, path: &str, data: Bytes) -> anyhow::Result<()> {
        let p = Path::from(format!("{}{}", self.base, path));
        self.store.put(&p, data).await?;
        Ok(())
    }
    async fn list(&self, prefix: &str) -> anyhow::Result<Vec<String>> {
        let p = Path::from(format!("{}{}", self.base, prefix));
        let mut out = vec![];
        let mut stream = self.store.list(Some(&p)).await?;
        use futures::StreamExt;
        while let Some(item) = stream.next().await.transpose()? {
            out.push(item.location.to_string());
        }
        Ok(out)
    }
}
```

This keeps the **backend pluggable** and engine-agnostic.

---

## Writing Results: Arrow → Parquet → `object_store`

```rust
use parquet::arrow::ArrowWriter;
use parquet::file::properties::WriterProperties;
use bytes::Bytes;

// Convert Vec<RecordBatch> to Parquet bytes in-memory, then upload
fn batches_to_parquet_bytes(batches: &[arrow::record_batch::RecordBatch]) -> anyhow::Result<Vec<u8>> {
    let schema = batches[0].schema();
    let mut buf = Vec::new();
    {
        let props = WriterProperties::builder().build();
        let mut writer = ArrowWriter::try_new(&mut buf, schema, Some(props))?;
        for b in batches {
            writer.write(b)?;
        }
        writer.close()?;
    }
    Ok(buf)
}

// usage with Blob impl
async fn write_result<B: Blob>(blob: &B, path: &str, batches: &[arrow::record_batch::RecordBatch]) -> anyhow::Result<()> {
    let bytes = batches_to_parquet_bytes(batches)?;
    blob.put(path, Bytes::from(bytes)).await
}
```

---

## Error Handling, Retries, and Timeouts

- Wrap `object_store` calls with `backoff` or implement retries in your executor.
- Use **bounded channels** between nodes to prevent memory blowups.
- Validate `HEAD` (existence) before long operations when helpful.

```rust
use backoff::{ExponentialBackoff, future::retry};
let res = retry(ExponentialBackoff::default(), || async {
    let resp = store.get(&path).await?;
    Ok::<_, object_store::Error>(resp)
}).await;
```

---

## Security & Credentials

- For MinIO: endpoint, access key, secret key; set `allow_http(true)` for local dev.
- For AWS S3: rely on AWS env/provider chain (IAM role, env vars).
- Avoid hard-coding secrets; pass through Tauri secure storage or env.

---

## Testing & Local Dev

- Use `file://` backed store in unit/integration tests for quick runs.
- Spin up MinIO in CI (Docker) for end-to-end tests.
- Provide fixtures (small Parquet/GeoParquet) and test the full scan → filter → write loop.

---

## Coexistence with OpenDAL and aws-sdk-s3

- Keep **aws-sdk-s3** for **presigned URLs**, bucket policies, lifecycle rules—**control-plane** features.
- You can use **OpenDAL** in app-layer modules (if you want retry/cache/metrics layers) while DataFusion continues to use `object_store` under the hood.
- If you prefer a single facade in your code, define the `Blob` trait (above) and implement it twice (object_store-backed and OpenDAL-backed). Choose at runtime via config.

---

## Iceberg & Catalogs

- **Iceberg data files** (Parquet/GeoParquet) are read via `object_store`.
- Configure an **Iceberg catalog** (REST/Hive/Nessie) separately; many Rust bindings rely on `object_store` for the underlying file I/O.

Recommended layout under a project:

```
s3://mybucket/projects/{project_id}/
  ├─ logs/          # Iceberg table (Parquet data + metadata dir)
  ├─ markers/       # Iceberg table
  ├─ wells/         # Iceberg table (GeoParquet)
  └─ polygons/      # Iceberg table (GeoParquet)
```

---

## Observability

- Wrap I/O in `tracing` spans to time range reads, lists, and uploads.
- Tag spans with: bucket, prefix, byte counts, retry count.

```rust
let span = tracing::info_span!("obj.get", path=%path);
let _g = span.enter();
// perform store.get(...)
```

---

## Implementation Plan

### **Phase 1: Core Integration (Week 1-2)**

1. **Create `object-store-adapter` crate**:
   - Centralized storage abstraction using `object_store`
   - Integration with existing `s3-config` for credentials
   - Blob trait for backend-agnostic operations
   - Support for S3/MinIO, local files, and memory stores

2. **Update `parquet-log-query-engine`**:
   - Replace manual S3 configuration with `object_store` registration
   - Enable direct `s3://` path queries in DataFusion
   - Maintain existing API compatibility
   - Add streaming capabilities for large datasets

3. **Update `storage-manager`**:
   - Use `object_store` for all read/write operations
   - Keep existing `WellStorageManager` interface
   - Add multipart upload support for large files
   - Integrate with `project-data-layout` for path management

### **Phase 2: Enhanced Features (Week 3-4)**

1. **Add advanced capabilities**:
   - Throttling and rate limiting
   - Retry logic with exponential backoff
   - Streaming Parquet reads for memory efficiency
   - Conditional operations for data integrity

2. **Integrate with Iceberg**:
   - Use `object_store` for Iceberg data file operations
   - Maintain catalog separation
   - Support for time travel queries

### **Key Modules Used**

- **`aws/`** - S3/MinIO support (primary backend)
- **`path/`** - Path handling and validation
- **`multipart/`** - Large file uploads and streaming
- **`buffered/`** - High-performance I/O operations
- **`throttle/`** - API rate limiting and backpressure
- **`client/`** - HTTP client configuration and retries
- **`integration/`** - Testing and validation utilities

### **Benefits for MudRock MVP**

1. **Immediate Performance Gains**:
   - Direct Parquet scans without intermediate downloads
   - Predicate pushdown for faster queries
   - Streaming I/O for large datasets

2. **Simplified Architecture**:
   - Single storage abstraction across all components
   - Cleaner configuration management
   - Better error handling and retries

3. **Future Scalability**:
   - Easy backend switching (S3 → Azure → GCS)
   - Better integration with data lake features
   - Enhanced observability and monitoring

## TL;DR

- Adopt `object_store` as the **primary data-plane** layer.
- Register MinIO/S3 with DataFusion's runtime for **direct scans**.
- Keep aws-sdk-s3 for control-plane; optionally use OpenDAL in app modules that benefit from its layers.
- Use a small `Blob` facade to stay backend-agnostic and testable.
- This setup maximizes compatibility with Arrow/DataFusion today and keeps us flexible for future backends.
