# Observability Plan: `tracing` + OpenTelemetry (`opentelemetry.md`)

This document explains **what `tracing` and OpenTelemetry (OTel) are**, why they’re useful in our stack, and a practical plan to instrument our **petgraph DAG**, **DataFusion queries**, and **storage I/O**. It also clarifies that OTel is for **observability** (performance, errors, behavior), **not** version control.

---

## What are `tracing` and OpenTelemetry?

- **`tracing`** (Rust crates: `tracing`, `tracing-subscriber`) provides **structured, contextual logs** and **spans** in Rust programs.
  - A **span** = a timed, named operation (e.g., “DfScanLogs”). Spans can be nested and tagged with attributes (key/value pairs).
  - **Events** (logs) occur inside spans and inherit context (request id, run id, node id).
- **OpenTelemetry (OTel)** is a **standard** for **traces, metrics, and logs**. It defines data models, APIs, and exporters to send telemetry to backends (Jaeger, Tempo, OTEL Collector, Prometheus, etc.).

### What is it useful for?
- **Performance**: measure durations, I/O sizes, cache hits/misses, CPU time per node, etc.
- **Failures**: where did the DAG fail? which node? with what inputs?
- **Capacity planning**: which queries are hottest? which S3 prefixes are read the most?
- **Reproducibility metadata**: attach **code version** and **UDF version** to spans for auditing. *(This is not version control itself; it just records version info as metadata).*

**Not version control**: keep using **Git** for source and **Iceberg snapshots** for dataset versions. OTel captures **runtime telemetry**, not source/data change history.

---

## Where to instrument in our architecture

1. **Petgraph executor (app-level DAG)**
   - A top-level **Run** span (e.g., `run_id`) for each user execution.
   - One **Node** span per DAG node (geo, DF, UDF, storage).
   - Attributes: `node.kind`, `node.name`, `node.version`, `inputs.hash`, `params`, `cache.hit`.
2. **DataFusion**
   - Wrap `DataFrame.collect()` with a span.
   - Record: logical plan fingerprint, bytes scanned (if available), rows out, duration.
3. **Storage I/O** (`object_store` / OpenDAL)
   - Spans around `get/put/list` with attributes: `bucket`, `path/prefix`, `bytes`, `retries`.
4. **UDFs**
   - Span per UDF eval, tag `udf.name`, `udf.version`, `params`, `rows`.
5. **Geo ops** (GeoArrow)
   - Span for spatial predicates: `op=point_in_polygon`, `candidate_points`, `matches`, `index=rtree|grid`.

---

## Minimal Rust setup

### Dependencies
```toml
[dependencies]
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter", "fmt", "json"] }
tracing-opentelemetry = "0.23"
opentelemetry = { version = "0.23", features = ["rt-tokio"] }
opentelemetry-otlp = { version = "0.16", features = ["grpc-tonic"] }
# metrics (optional new API)
opentelemetry-metrics = "0.23"
```

### Init (dev: pretty logs; prod: OTLP exporter)
```rust
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use tracing_opentelemetry::OpenTelemetryLayer;
use opentelemetry::{sdk::{Resource, trace as sdktrace}, KeyValue};
use opentelemetry_otlp::WithExportConfig;

pub fn init_tracing(service_name: &str) -> anyhow::Result<()> {
    // OTel tracer
    let resource = Resource::new(vec![KeyValue::new("service.name", service_name)]);
    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_trace_config(sdktrace::config().with_resource(resource))
        .with_exporter(opentelemetry_otlp::new_exporter().tonic()) // OTLP/gRPC to Collector
        .install_batch(opentelemetry::runtime::Tokio)?;

    let otel_layer = OpenTelemetryLayer::new(tracer);

    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| "info,our_crate=debug".into());

    tracing_subscriber::registry()
        .with(env_filter)
        .with(tracing_subscriber::fmt::layer().compact()) // dev pretty logs
        .with(otel_layer) // export traces to OTel
        .init();

    Ok(())
}

// Call at startup:
init_tracing("tauri-backend")?;
```

### Spans around DAG nodes
```rust
use tracing::{info_span, Instrument};

async fn execute_node(node: &Node, inputs: &Artifact) -> anyhow::Result<Artifact> {
    let span = info_span!(
        "dag.node",
        node.name = %node.name,
        node.kind = ?node.kind,
        node.version = %node.version,
        cache_enabled = true,
    );
    async move {
        // cache check...
        let out = run_node_impl(node, inputs).await?;
        // record sizes, hashes
        Ok(out)
    }.instrument(span).await
}
```

### Spans around DataFusion collect
```rust
use tracing::info_span;
use datafusion::prelude::*;

async fn run_df(df: DataFrame, label: &str) -> datafusion::error::Result<Vec<arrow::record_batch::RecordBatch>> {
    let span = info_span!("datafusion.collect", query = %label);
    let _g = span.enter();
    let out = df.collect().await?;
    // Optionally: record row/column counts as events or metrics
    Ok(out)
}
```

### Storage I/O instrumentation
```rust
use tracing::{info_span, info};

async fn get_bytes(store: &Arc<dyn object_store::ObjectStore>, path: &object_store::path::Path) -> object_store::Result<bytes::Bytes> {
    let span = info_span!("object_store.get", path = %path);
    let _g = span.enter();
    let resp = store.get(path).await?;
    let bytes = resp.bytes().await?;
    info!(len = bytes.len(), "downloaded");
    Ok(bytes)
}
```

---

## Metrics (counters & histograms)

Add OTel **metrics** to capture system health over time.

Examples:
- `dag.node.duration_ms` (histogram)
- `datafusion.bytes_scanned` (counter)
- `object_store.bytes_in` / `bytes_out` (counter)
- `udf.rows_processed` (counter)
- `cache.hits` / `cache.misses` (counter)

Sketch (API evolving; choose the stable metrics crate version that matches your OTel/TS exporter):
```rust
use opentelemetry::{global, metrics::Meter};

let meter = global::meter("our-app");
let duration_hist = meter.f64_histogram("dag.node.duration_ms").init();

let start = std::time::Instant::now();
// ... do work ...
let ms = start.elapsed().as_secs_f64() * 1000.0;
duration_hist.record(ms, &[]);
```

---

## Propagation & correlation

- Generate a **run_id** (UUID) per user execution; attach to the **root span**.
- Pass **trace context** across async tasks and threads (tracing + Tokio preserves it).
- Include **business keys**: `project_id`, `user_id` (hash/anonymize if needed), `polygon_id`, `wells_count`.
- Record **code/data versions** as attributes:
  - `git.commit`, `udf.vshale.version`, `iceberg.snapshot_id`.
  - These help **audit** runs but are **not** version control.

---

## Backends to receive telemetry

- **OpenTelemetry Collector** (recommended): central receiver; forwards to:
  - **Grafana Tempo** or **Jaeger** for traces,
  - **Prometheus** or **OTel metrics** backends for metrics,
  - **Loki/Elastic** for logs (if needed).
- Local dev: set `OTEL_EXPORTER_OTLP_ENDPOINT=http://localhost:4317` (gRPC).

---

## Privacy & PII

- Don’t log raw well names or sensitive metadata unless necessary. Prefer **IDs**.
- Use **attributes** to tag context; avoid dumping full payloads into logs.
- Consider sampling (`parentbased_traceidratio`) if traffic grows.

---

## What observability is **not**

- Not **source control** → use **Git**.
- Not **data versioning** → use **Iceberg snapshots** and **catalogs** (Nessie/Hive/REST).  
- Observability **records runtime behavior** (latency, errors, sizes), not history of code/data changes.

---

## Quick checklist

- [ ] Initialize `tracing` + OTel exporter at app startup.
- [ ] Create a **root run span** with `run_id`, `project_id`, `user_id`.
- [ ] Emit **one span per DAG node**; attach inputs/params and cache status.
- [ ] Wrap **DataFusion `.collect()`** with spans; record rows/bytes.
- [ ] Instrument **object_store** I/O (get/put/list) with spans + byte counts.
- [ ] Add **metrics**: node durations, bytes in/out, cache hits, errors.
- [ ] Export to **OTel Collector**; view traces in Tempo/Jaeger.
- [ ] Document PII policy; anonymize attributes where needed.

---

## TL;DR

Use `tracing` for structured spans/events and OpenTelemetry for **exporting traces/metrics**. Instrument the **petgraph executor**, **DataFusion queries**, **storage I/O**, **UDFs**, and **geo ops**. This gives you clear performance visibility and failure diagnostics. OTel **augments** (does not replace) Git and Iceberg for versioning.
