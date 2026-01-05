# Project Nessie Integration Plan (`nessie_plan.md`)

This document explains **how to integrate Project Nessie** with our **Iceberg + MinIO/S3 + DataFusion** stack, how it changes workflows (branches/tags, time travel), and shows practical wiring for dev and prod.

---

## What is Project Nessie? (Git for your data lake)

**Project Nessie** is a **versioned catalog** for lakehouse tables (Iceberg/Delta), offering:

- **Branches & tags** (Git-like) across many tables at once.
- **Atomic commits** of table changes (new snapshots, schema/partition evolution).
- **Isolated workspaces** per user/project; promote changes via merge/fast-forward to `main`.
- **Time travel**: read the lake **as-of** any branch & commit id.

**Why we want it:** it gives us **safe multi-user development**, **repeatable experiments**, and **simple promotion** of results without copying data.

---

## High-Level Architecture

```
Tauri App (Rust)
   │
   ├─► Petgraph Executor (DAG)
   │     ├─ DF nodes (DataFusion over Iceberg)
   │     ├─ Geo nodes (GeoArrow)
   │     └─ Commit nodes (Iceberg write -> Nessie commit)
   │
   ├─► Iceberg tables (logs, markers, wells, polygons)
   │     └─ stored in MinIO/S3
   │
   └─► Nessie Server (Catalog)
         └─ REST Catalog endpoint used by Iceberg clients
```

- **Nessie** = the **catalog**: tracks table metadata & versions per branch.
- **MinIO/S3** = object storage for actual Parquet/metadata files.
- **DataFusion** = reads/writes Iceberg tables against a **chosen Nessie ref** (branch/tag).

---

## Deployment (dev & prod)

### Local dev (Docker Compose)

```yaml
version: "3.9"
services:
  minio:
    image: minio/minio:latest
    command: server /data --console-address ":9001"
    environment:
      MINIO_ROOT_USER: minioadmin
      MINIO_ROOT_PASSWORD: minioadmin
    ports: ["9000:9000", "9001:9001"]
    volumes: ["./.data/minio:/data"]

  nessie:
    image: ghcr.io/projectnessie/nessie:latest
    environment:
      QUARKUS_HTTP_PORT: 19120
      QUARKUS_PROFILE: prod
    ports: ["19120:19120"]

  # Optional: Iceberg REST Catalog proxy (if not using Nessie's REST directly)
```

Nessie default REST endpoint: `http://localhost:19120/api/v2`.

### Production

- Run Nessie behind TLS and auth (OIDC/JWT).
- Store **Nessie catalog database** on a managed RDBMS (e.g., Postgres) for durability.
- Configure backup/snapshot policies for the catalog DB.

---

## Iceberg + Nessie Wiring

Nessie implements an **Iceberg REST Catalog**. Iceberg clients talk to Nessie using the REST protocol and specify a **reference** (branch/tag).

### Catalog Configuration (environment)

```
ICEBERG_CATALOG__URI=http://localhost:19120/api/v2
ICEBERG_CATALOG__WAREHOUSE=s3://mybucket/projects/
ICEBERG_CATALOG__AUTH_TYPE=NONE   # dev; use OIDC in prod
```

### Selecting a Branch/Tag

- **Ref per session** (recommended): choose the branch for the DataFusion session doing reads/writes.
- You can model **one branch per project** or **per user feature-branch**.

---

## Branching Strategy

- `main` — stable data consumed by end users.
- `proj-{id}` — long-lived branch per project/workspace (optional).
- `user-{name}` or `feat-{ticket}` — short-lived branches for experiments.

**Workflow:**

1. Create branch from `main`: `feat-vshale-params`.
2. Run petrophysical pipeline (petgraph): write results to Iceberg tables **on that branch**.
3. Validate outputs; if good, **merge** or **fast-forward** `feat-vshale-params` → `main`.

This enables **isolated changes** without affecting other users.

---

## Petgraph Nodes that touch Nessie

Add explicit nodes to your DAG to declare intent:

- `NessieCreateBranch { from: "main", to: "feat-xyz" }`
- `DfWriteIceberg { table: "logs", mode: "append|overwrite", ref: "feat-xyz" }`
- `NessieMerge { from: "feat-xyz", to: "main" }`
- `NessieTagSnapshot { table: "markers", tag: "qc-2025-10-15" }`

Each node wraps the appropriate Iceberg client calls (against the chosen ref) and records **commit metadata** (author, message, params) for auditability.

---

## Reading/Writing on a Specific Nessie Ref

Exact Rust APIs depend on your Iceberg client. Conceptually:

```rust
// Pseudo-code – choose Nessie ref for this session
let catalog = IcebergRestCatalog::new("http://localhost:19120/api/v2")
    .with_warehouse("s3://mybucket/projects/")
    .with_reference("feat-vshale-params"); // branch

// Read table "logs" at ref
let table = catalog.load_table("logs")?;

// DataFusion session reads via table location (manifest paths → S3)
let ctx = SessionContext::new();
// register as Iceberg table or listing table with paths from table metadata
// ... run queries ...

// Write: construct Append/Overwrite operation tied to ref
// table.new_append().add_files(...).commit_with_metadata(commit_msg, props);
```

**Key idea:** all **reads/writes** are resolved according to the active **Nessie reference**. You can have many concurrent sessions pointing to different branches without conflicts.

---

## Promotion & Isolation

- **Isolation**: a branch is a **consistent view**. Running queries and writing results on `feat-xyz` never affects `main` until you merge.
- **Promotion**: when validated, a **merge** operation advances `main` to include the snapshots added on `feat-xyz`.
- **Rollback/Time Travel**: query `main` as-of a snapshot id if needed for forensic analysis.

---

## Access Control

- Enable Nessie auth (OIDC/JWT).
- Map users/groups to branch permissions (create, read, merge).
- Optionally enforce write policies (e.g., only CI merges to `main`).

---

## Observability

- Record **commit metadata** (author, message, parameters, git SHA of UDF code) with Iceberg operations.
- Emit **tracing spans** around create/merge/commit with attributes: `nessie.ref`, `table`, `snapshot_id`.
- Include **run_id** from our petgraph executor for cross-correlation.

---

## Retention & Maintenance

- **Snapshot retention**: keep last N snapshots or days, per table & branch.
- **Compaction**: periodically rewrite manifests / compact small files.
- **Branch cleanup**: auto-delete stale branches after merge + retention window.

---

## Testing Strategy

- Docker-compose (MinIO + Nessie).
- Seed with small Iceberg tables (logs/markers/wells/polygons).
- CI tests:
  - `CreateBranch` → run pipeline → `Merge` → validate table snapshot advanced.
  - Concurrency test: two branches writing to the same table → ensure isolation.
  - Time-travel reads to verify determinism.

---

## Example User Flow (end-to-end)

1. User selects **Project A** in Tauri; app creates/chooses a branch `proj-A-alice`.
2. User draws polygon, picks curves and markers → petgraph pipeline runs.
3. DataFusion reads/writes Iceberg tables **on `proj-A-alice`**.
4. User reviews outputs; clicks **Promote** → app triggers `NessieMerge { from: "proj-A-alice", to: "main" }`.
5. Downstream consumers switch to latest `main` snapshot automatically.

---

## When to add Spark (optional)

For large compactions or heavy ETL, a Spark job can be run **against Nessie** (same branch semantics). This is optional; keep DataFusion for interactive/app workflows.

---

## Security Notes

- Store Nessie tokens securely (Tauri keychain).
- Use least-privilege IAM on MinIO/S3: Nessie service account writes metadata & data; clients read/write as permitted.
- Audit merges (record who merged what, when).

---

## TL;DR

- **Nessie** gives us **Git-like version control** over our Iceberg lake: branches, tags, merges, time travel.
- Integrate it as the **catalog**; point DataFusion sessions at the **appropriate ref** per pipeline run.
- Model create/merge/commit as DAG nodes; record commit metadata for observability.
- This yields safe multi-user development, easy promotion, and reproducible results without copying data.
