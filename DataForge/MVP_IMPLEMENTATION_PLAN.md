# DataForge MVP Implementation Plan

**Version**: 2.3
**Created**: 2024-12-14
**Updated**: 2024-12-18
**Status**: 🟢 VALIDATION READY - Core MVP Complete, Ready for User Testing

---

## Table of Contents

1. [Executive Summary](#executive-summary)
   - [What We're Building](#what-were-building)
   - [Key Architectural Decisions](#key-architectural-decisions)
   - [What We're NOT Building](#what-were-not-building-deferred)
   - [Strategic Context: Composable Architecture](#strategic-context-composable-architecture)
   - [Positioning: Resilience, Not Replacement](#positioning-resilience-not-replacement)
2. [Core Architecture](#core-architecture)
3. [Technology Stack](#technology-stack)
4. [Data Model](#data-model)
5. [Deployment Options](#deployment-options)
6. [Implementation Phases](#implementation-phases)
7. [Reusable Crates](#reusable-crates)
8. [Progress Tracking](#progress-tracking)
9. [Summary](#summary)
   - [Validation Phase](#validation-phase-current)
   - [Compute MVP (Planned Follow-on)](#compute-mvp-planned-follow-on)

---

## Executive Summary

### What We're Building

**DataForge is the local engineering workspace** — where truth lives, and other tools connect.

It's an **offline-first, local-first** application that provides **resilience** when centralized systems, connectivity, or licenses are unavailable. Engineers can:

1. **Ingest LAS files** → Parse → Convert to Parquet → Store with provenance
2. **Browse and inspect** well log data (read-only, passive visualization)
3. **Work offline** with full functionality — no network required
4. **Sync to server** when online (Git-like, not Figma-like)
5. **Share data** with team members in private/enterprise environments
6. **Export data** in standard formats (LAS, CSV, Parquet)

> **Mental Model**: DataForge is not a database, not a platform — it's **the workspace**. Other tools (compute clients, visualization apps, export utilities) read from it and write back to it. DataForge keeps truth stable; other tools are free to experiment.

> **Note**: A separate **Compute MVP** (planned) will demonstrate how computation clients can safely read from DataForge, run explicit calculations, and write derived results back. See [Compute MVP Spec](markdown-guides/DataForge/strategy/DataForge_Compute_MVP.md) and [Platform Strategy](markdown-guides/DataForge/strategy/DataForge_Platform_Strategy.md).

### Key Architectural Decisions

| Decision       | Choice                    | Rationale                                                      |
| -------------- | ------------------------- | -------------------------------------------------------------- |
| **Sync Model** | Git-like (pull-based)     | Enterprise users don't need real-time Figma-like collaboration |
| **CRDT**       | None for MVP              | Overkill for Git-like sync; simple version vectors suffice     |
| **Large Data** | Content-addressed Parquet | Immutable blobs referenced by SHA-256 hash                     |
| **Local DB**   | SQLite (Tauri native)     | No IndexedDB needed; Tauri has filesystem access               |
| **Server DB**  | SQLite for MVP            | Can swap to PostgreSQL later; simpler deployment               |
| **Real-time**  | REST API (no WebSocket)   | Git-like sync doesn't need push notifications                  |

### What We're NOT Building (Deferred)

- ❌ Real-time collaborative editing (Figma-like)
- ❌ Yjs/CRDT for document sync
- ❌ WebSocket push notifications
- ❌ Pipelines, nodes, notebooks
- ❌ Complex execution tracking
- ❌ Parametric computation / derived curves (separate app per Platform Strategy)
- ❌ Interactive curve editing / splicing tools
- ❌ Computed unified views with interpolation

### Strategic Context: Composable Architecture

DataForge follows the same architectural principles as modern composable data stacks (e.g., modern GIS) — adapted for subsurface constraints.

#### Analogy: Modern GIS Stack → DataForge Stack

| Modern GIS Stack | DataForge Stack | Purpose |
|------------------|-----------------|---------|
| QGIS (client) | DataForge Desktop | Interface layer — see and interact with data |
| PostGIS | Local SQLite | Source of truth — metadata, permissions, transactions |
| GeoParquet / Zarr | Parquet (content-addressed) | Modern storage formats — query what you need |
| Sedona / DuckDB | DuckDB (embedded) | Analytical layer — fast queries, aggregations |
| Multiple viewers | Future compute & viz clients | Presentation — choose the tool for the audience |

**Key insight**: You choose where complexity lives. DataForge provides a stable foundation; computation and visualization clients can be swapped, upgraded, or purpose-built without affecting the system of record.

#### System of Record vs System of Computation

Per the [DataForge Platform Strategy](markdown-guides/DataForge/strategy/DataForge_Platform_Strategy.md):

| DataForge (This MVP) | Compute MVP (Planned Follow-on) |
|---------------------|--------------------------------|
| System of Record | System of Computation |
| Store, version, sync | Calculate, derive, model |
| Passive visualization | Active, explicit computation |
| "This IS the data" | "This is derived from data" |
| Workspace — truth lives here | Client — experiments happen here |

**DataForge MVP includes passive visualization** (read-only display of stored curves) but defers active computation (derived curves, parametric models) to separate client applications.

### Positioning: Resilience, Not Replacement

**Important**: DataForge does **not** replace Petrel, OSDU, or cloud platforms.

Instead:

> DataForge supports uninterrupted engineering work when centralized systems, connectivity, or licenses are unavailable or impractical.

This positioning emphasizes:
- **Resilience** — work continues when corporate systems are down
- **Sovereignty** — data stays under your control
- **Portability** — no vendor lock-in, open formats throughout

---

## Core Architecture

### High-Level Architecture

```
┌─────────────────────────────────────────────────────────────────────┐
│                    DataForge MVP Architecture                        │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  ┌────────────────────────────────────────────────────────────────┐ │
│  │  CLIENT (Tauri Desktop App) - SYSTEM OF RECORD                │ │
│  │  ┌──────────────┐  ┌──────────────┐  ┌──────────────────────┐ │ │
│  │  │ SvelteKit UI │  │ Rust Backend │  │ Local Storage        │ │ │
│  │  │ - Data Grid  │  │ - LAS Parser │  │ - SQLite (metadata)  │ │ │
│  │  │ - Inspector  │  │ - Parquet    │  │ - Parquet (curves)   │ │ │
│  │  │ - Upload     │  │ - DuckDB     │  │ - Blob Store         │ │ │
│  │  └──────────────┘  └──────────────┘  └──────────────────────┘ │ │
│  └────────────────────────────────────────────────────────────────┘ │
│                              │                                       │
│                              │ REST API (when online)                │
│                              │ Git-like sync                         │
│                              ▼                                       │
│  ┌────────────────────────────────────────────────────────────────┐ │
│  │  SERVER (Self-Hosted / Private Cloud)                          │ │
│  │  ┌──────────────┐  ┌──────────────┐  ┌──────────────────────┐ │ │
│  │  │ REST API     │  │ SQLite DB    │  │ Blob Storage         │ │ │
│  │  │ - Auth       │  │ - Users      │  │ - S3/MinIO           │ │ │
│  │  │ - Sync       │  │ - Projects   │  │ - Local Filesystem   │ │ │
│  │  │ - Upload     │  │ - Wells      │  │ - Parquet files      │ │ │
│  │  └──────────────┘  └──────────────┘  └──────────────────────┘ │ │
│  └────────────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────────┘
```

### Data Flow: LAS Upload to Local Parquet and Query

```
┌─────────────────────────────────────────────────────────────────────┐
│  1. User Selects LAS File                                           │
└──────────────────────┬──────────────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────────────┐
│  2. Rust LAS Parser (las-parser crate)                              │
│  - Parse LAS 1.2, 2.0, 3.0                                          │
│  - Extract well metadata                                            │
│  - Detect curve types (GR, RHOB, NPHI, etc.)                       │
│  - Handle wrapped/unwrapped data                                    │
└──────────────────────┬──────────────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────────────┐
│  3. Unit Conversion (unit-conversions crate)                        │
│  - Convert to standard units                                        │
│  - GR → GAPI, RHOB → g/cm³, DT → µs/ft                            │
└──────────────────────┬──────────────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────────────┐
│  4. Parquet Generation (Arrow + Parquet)                            │
│  - Create Arrow RecordBatch                                         │
│  - Write compressed Parquet file                                    │
│  - Compute SHA-256 hash                                             │
└──────────────────────┬──────────────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────────────┐
│  5. Content-Addressed Storage                                       │
│  - Store: blobs/{hash[0:2]}/{hash[2:4]}/{hash}.parquet             │
│  - Deduplication: Skip if hash exists                               │
│  - Immutable: Never modify, only add                                │
└──────────────────────┬──────────────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────────────┐
│  6. SQLite Metadata                                                 │
│  - Insert well record                                               │
│  - Insert curve records with blob_hash reference                    │
│  - Store statistics (min, max, row_count)                           │
└──────────────────────┬──────────────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────────────┐
│  7. DuckDB Query                                                    │
│  - SELECT DEPT, GR, RHOB FROM read_parquet('...')                  │
│  - WHERE DEPT BETWEEN 1000 AND 2000                                 │
│  - Efficient columnar queries                                       │
└─────────────────────────────────────────────────────────────────────┘
```

### Git-Like Sync Flow

```
┌─────────────────────────────────────────────────────────────────────┐
│  User Works Offline                                                  │
│  - Upload LAS files                                                  │
│  - Create wells, curves                                              │
│  - All changes saved to local SQLite + Parquet                      │
└──────────────────────┬──────────────────────────────────────────────┘
                       │
                       │ User clicks "Sync" (or automatic when online)
                       ▼
┌─────────────────────────────────────────────────────────────────────┐
│  1. Client: Get Local Changes                                       │
│  SELECT * FROM wells WHERE version > last_sync_version              │
│  SELECT * FROM curves WHERE version > last_sync_version             │
└──────────────────────┬──────────────────────────────────────────────┘
                       │
                       │ POST /api/sync/push
                       ▼
┌─────────────────────────────────────────────────────────────────────┐
│  2. Server: Receive Changes                                         │
│  - Apply changes to server SQLite                                   │
│  - Upload Parquet blobs to storage                                  │
│  - Return server's version                                          │
└──────────────────────┬──────────────────────────────────────────────┘
                       │
                       │ GET /api/sync/pull?since=<version>
                       ▼
┌─────────────────────────────────────────────────────────────────────┐
│  3. Client: Receive Server Changes                                  │
│  - Apply server changes to local SQLite                             │
│  - Download missing Parquet blobs                                   │
│  - Update last_sync_version                                         │
└─────────────────────────────────────────────────────────────────────┘
```

---

## Technology Stack

### Client (Tauri Desktop App)

| Component        | Technology        | Purpose                            |
| ---------------- | ----------------- | ---------------------------------- |
| **Framework**    | Tauri 2.0         | Desktop app wrapper, Rust backend  |
| **Frontend**     | SvelteKit 5       | UI framework                       |
| **Styling**      | Tailwind CSS 4    | Utility-first CSS                  |
| **Data Grid**    | AG Grid           | Enterprise data grid               |
| **Local DB**     | SQLite (rusqlite) | Metadata storage                   |
| **Query Engine** | DuckDB            | Parquet file queries               |
| **State**        | Svelte 5 runes    | Reactive state management          |

### Server (Self-Hosted)

| Component        | Technology                | Purpose                 |
| ---------------- | ------------------------- | ----------------------- |
| **API**          | Rust (Axum)               | REST API server         |
| **Database**     | SQLite (MVP) → PostgreSQL | Metadata storage        |
| **Blob Storage** | S3/MinIO/Filesystem       | Parquet file storage    |
| **Auth**         | JWT tokens                | Authentication          |
| **Deployment**   | Docker Compose            | Container orchestration |

### Shared (Rust Crates)

| Crate              | Purpose                      | Status      |
| ------------------ | ---------------------------- | ----------- |
| `las-parser`       | LAS file parsing             | ✅ Existing |
| `las-types`        | LAS data structures          | ✅ Existing |
| `unit-conversions` | Petrophysics unit conversion | ✅ Existing |
| `mudrock-core`     | Shared business logic        | 🆕 New      |
| `mudrock-sync`     | Sync protocol                | 🆕 New      |
| `mudrock-storage`  | Blob storage abstraction     | 🆕 New      |

---

## Data Model

### SQLite Schema (Client & Server)

> **Note**: The full schema is defined in `crates/dataforge-core/src/db.rs`. Below is a summary of key tables.

```sql
-- ============================================================
-- AUTHENTICATION (ColaNode-style Account-Workspace-Member)
-- ============================================================

CREATE TABLE accounts (
    id TEXT PRIMARY KEY,
    email TEXT UNIQUE NOT NULL,
    password_hash TEXT,
    name TEXT NOT NULL,
    status INTEGER NOT NULL DEFAULT 0,  -- 0=unverified, 1=verified, 2=suspended
    created_at TEXT, updated_at TEXT
);

CREATE TABLE workspaces (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    owner_account_id TEXT NOT NULL REFERENCES accounts(id),
    created_at TEXT, updated_at TEXT
);

CREATE TABLE workspace_members (
    id TEXT PRIMARY KEY,
    workspace_id TEXT NOT NULL REFERENCES workspaces(id),
    account_id TEXT NOT NULL REFERENCES accounts(id),
    role TEXT NOT NULL DEFAULT 'member',  -- 'owner', 'admin', 'member', 'viewer'
    UNIQUE(workspace_id, account_id)
);

-- ============================================================
-- WELLS (with Canonical Depth Grid)
-- ============================================================

CREATE TABLE wells (
    id TEXT PRIMARY KEY,
    workspace_id TEXT NOT NULL REFERENCES workspaces(id),
    name TEXT NOT NULL,
    uwi TEXT,  -- Unique Well Identifier
    -- Depth grid configuration (canonical sampling)
    depth_unit TEXT NOT NULL DEFAULT 'ft',
    depth_step REAL NOT NULL DEFAULT 0.5,
    depth_origin REAL NOT NULL DEFAULT 0.0,
    min_depth REAL, max_depth REAL,
    version INTEGER NOT NULL DEFAULT 1,
    deleted_at TEXT
);

-- ============================================================
-- LOG RUNS (one per LAS upload, preserves provenance)
-- ============================================================

CREATE TABLE log_runs (
    id TEXT PRIMARY KEY,
    well_id TEXT NOT NULL REFERENCES wells(id),
    source_filename TEXT NOT NULL,
    source_file_hash TEXT,
    log_type_id TEXT REFERENCES log_types(id),
    acquisition_type_id TEXT REFERENCES acquisition_types(id),
    original_top_depth REAL, original_bottom_depth REAL,
    original_step REAL, original_depth_unit TEXT,
    las_version TEXT, imported_by TEXT, imported_at TEXT
);

-- ============================================================
-- CURVES (Dual Native/Gridded Storage with Quality Tracking)
-- ============================================================

CREATE TABLE curves (
    id TEXT PRIMARY KEY,
    log_run_id TEXT NOT NULL REFERENCES log_runs(id),
    well_id TEXT NOT NULL REFERENCES wells(id),
    mnemonic TEXT NOT NULL,  -- Original from LAS (GR, RHOB, etc.)
    property_id TEXT REFERENCES curve_properties(id),
    unit TEXT,
    unit_id TEXT REFERENCES units(id),
    -- Native storage (original sampling)
    native_top_depth REAL, native_bottom_depth REAL,
    native_step REAL, native_sample_count INTEGER,
    native_parquet_hash TEXT,  -- SHA-256 content address
    -- Gridded storage (resampled to well grid)
    gridded_top_depth REAL, gridded_bottom_depth REAL,
    gridded_sample_count INTEGER,
    gridded_parquet_hash TEXT,
    resample_method TEXT,
    -- Statistics
    min_value REAL, max_value REAL, mean_value REAL, null_count INTEGER,
    -- Quality fields (OSDU-inspired)
    null_value REAL, quality_flag TEXT DEFAULT 'raw',
    acquisition_date TEXT, service_company TEXT,
    UNIQUE(log_run_id, mnemonic)
);

-- ============================================================
-- OSDU-INSPIRED UNIT SERVICE
-- ============================================================

CREATE TABLE measurement_types (
    id TEXT PRIMARY KEY,  -- 'length', 'pressure', 'resistivity'
    name TEXT NOT NULL UNIQUE,
    base_unit_id TEXT
);

CREATE TABLE units (
    id TEXT PRIMARY KEY,
    measurement_type_id TEXT NOT NULL REFERENCES measurement_types(id),
    symbol TEXT NOT NULL,  -- 'ft', 'm', 'psi'
    name TEXT NOT NULL,    -- 'feet', 'meters'
    to_base_factor REAL NOT NULL DEFAULT 1.0,
    to_base_offset REAL NOT NULL DEFAULT 0.0,
    is_base INTEGER NOT NULL DEFAULT 0,
    UNIQUE(measurement_type_id, symbol)
);

-- ============================================================
-- PWLS-STYLE CURVE PROPERTY DICTIONARY
-- ============================================================

CREATE TABLE curve_properties (
    id TEXT PRIMARY KEY,  -- 'gamma_ray', 'bulk_density'
    canonical_name TEXT NOT NULL UNIQUE,
    property_class TEXT,  -- 'Natural Radioactivity', 'Density'
    measurement_type_id TEXT REFERENCES measurement_types(id),
    typical_unit TEXT,
    display_color TEXT, log_scale INTEGER DEFAULT 0,
    min_valid_value REAL, max_valid_value REAL
);

CREATE TABLE curve_mnemonics (
    mnemonic TEXT PRIMARY KEY,  -- 'GR', 'SGR', 'RHOB'
    property_id TEXT REFERENCES curve_properties(id),
    priority INTEGER NOT NULL DEFAULT 0,
    vendor TEXT, notes TEXT
);

-- ============================================================
-- SYNC (ColaNode-style Git-like)
-- ============================================================

CREATE TABLE sync_state (
    workspace_id TEXT PRIMARY KEY REFERENCES workspaces(id),
    server_url TEXT NOT NULL,
    last_sync_version INTEGER NOT NULL DEFAULT 0,
    sync_status TEXT NOT NULL DEFAULT 'idle',
    conflict_strategy TEXT NOT NULL DEFAULT 'manual'
);

CREATE TABLE sync_queue (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    workspace_id TEXT NOT NULL,
    entity_type TEXT NOT NULL, entity_id TEXT NOT NULL,
    action TEXT NOT NULL,  -- 'create', 'update', 'delete'
    version INTEGER, payload TEXT, blob_hashes TEXT,
    priority INTEGER DEFAULT 0, attempts INTEGER DEFAULT 0,
    synced_at TEXT,
    UNIQUE(workspace_id, entity_type, entity_id)
);

CREATE TABLE sync_conflicts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    workspace_id TEXT NOT NULL,
    entity_type TEXT NOT NULL, entity_id TEXT NOT NULL,
    local_version INTEGER, local_data TEXT,
    remote_version INTEGER, remote_data TEXT,
    resolution TEXT  -- 'pending', 'local', 'remote', 'merged'
);
```

### Parquet File Structure

```
Local Storage:
{app_data}/
├── blobs/
│   ├── a3/
│   │   └── f2/
│   │       └── a3f2b8c9d1e4f5...parquet
│   └── 7b/
│       └── 1c/
│           └── 7b1c3d4e5f6a7...parquet
├── sqlite/
│   └── mudrock.db
└── cache/
    └── (temporary files)

Server Storage (S3/MinIO):
s3://mudrock-data/
├── blobs/
│   ├── a3/f2/a3f2b8c9d1e4f5...parquet
│   └── 7b/1c/7b1c3d4e5f6a7...parquet
└── (same structure as local)
```

### Parquet File Contents

```
┌─────────────────────────────────────────────────────────────────┐
│ Parquet File: a3f2b8c9d1e4f5...parquet                          │
├─────────────────────────────────────────────────────────────────┤
│ Metadata (Footer):                                              │
│   - well_id: "uuid-here"                                        │
│   - source_file: "well_a.las"                                   │
│   - las_version: "2.0"                                          │
│   - created_at: "2024-12-14T10:30:00Z"                          │
│   - created_by: "user-uuid"                                     │
├─────────────────────────────────────────────────────────────────┤
│ Schema:                                                         │
│   - DEPT: Float64 (depth index, always first)                   │
│   - GR: Float64 (nullable)                                      │
│   - RHOB: Float64 (nullable)                                    │
│   - NPHI: Float64 (nullable)                                    │
│   - RT: Float64 (nullable)                                      │
├─────────────────────────────────────────────────────────────────┤
│ Row Groups (optimized for depth range queries):                 │
│   - RowGroup 0: DEPT 0-1000                                     │
│   - RowGroup 1: DEPT 1000-2000                                  │
│   - RowGroup 2: DEPT 2000-3000                                  │
└─────────────────────────────────────────────────────────────────┘
```

---

## Deployment Options

### Comparison: ColaNode vs MudRock Deployment

| Aspect                | ColaNode                       | MudRock MVP             |
| --------------------- | ------------------------------ | ----------------------- |
| **Architecture**      | Full Yjs/CRDT real-time        | Git-like REST sync      |
| **Server Components** | PostgreSQL + Redis + WebSocket | SQLite + REST API       |
| **Complexity**        | High (many moving parts)       | Low (simple stack)      |
| **Real-time**         | Yes (WebSocket + Yjs)          | No (pull-based sync)    |
| **Offline**           | Yes (SQLite + IndexedDB)       | Yes (SQLite + Parquet)  |
| **Deployment**        | Docker Compose or K8s          | Single binary or Docker |

### Option 1: Air-Gapped / Fully Offline (Enterprise)

**Use Case**: Oil & gas companies with strict security requirements, no internet access.

```
┌─────────────────────────────────────────────────────────────────┐
│  Enterprise Data Center (Air-Gapped)                            │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  ┌────────────────────────────────────────────────────────────┐ │
│  │  Internal Server (Single VM)                               │ │
│  │  ┌──────────────┐  ┌──────────────┐  ┌──────────────────┐ │ │
│  │  │ MudRock API  │  │ SQLite DB    │  │ NFS/Local Disk   │ │ │
│  │  │ (Rust binary)│  │              │  │ (Parquet blobs)  │ │ │
│  │  └──────────────┘  └──────────────┘  └──────────────────┘ │ │
│  └────────────────────────────────────────────────────────────┘ │
│                              │                                   │
│                              │ LAN only                          │
│                              ▼                                   │
│  ┌────────────────────────────────────────────────────────────┐ │
│  │  User Workstations                                         │ │
│  │  ┌──────────────┐  ┌──────────────┐  ┌──────────────────┐ │ │
│  │  │ MudRock App  │  │ MudRock App  │  │ MudRock App      │ │ │
│  │  │ (Tauri)      │  │ (Tauri)      │  │ (Tauri)          │ │ │
│  │  └──────────────┘  └──────────────┘  └──────────────────┘ │ │
│  └────────────────────────────────────────────────────────────┘ │
│                                                                  │
│  Sync: Manual USB transfer or internal network only             │
└─────────────────────────────────────────────────────────────────┘
```

**Deployment Steps**:

1. Install single Rust binary on internal server
2. Configure SQLite path and blob storage path
3. No Docker required (optional)
4. No external dependencies

**Pros**:

- ✅ Maximum security (no internet)
- ✅ Simple deployment (single binary)
- ✅ Full data sovereignty
- ✅ Works in classified environments

**Cons**:

- ❌ Manual sync between sites
- ❌ No remote access

### Option 2: Private Cloud (Self-Hosted VPS)

**Use Case**: Companies wanting cloud convenience with data control.

```
┌─────────────────────────────────────────────────────────────────┐
│  Hetzner / AWS / Azure VPS                                      │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  ┌────────────────────────────────────────────────────────────┐ │
│  │  Docker Compose Stack                                      │ │
│  │  ┌──────────────┐  ┌──────────────┐  ┌──────────────────┐ │ │
│  │  │ Traefik      │  │ MudRock API  │  │ MinIO            │ │ │
│  │  │ (Reverse     │  │ (Rust)       │  │ (S3-compatible)  │ │ │
│  │  │  Proxy + SSL)│  │              │  │                  │ │ │
│  │  └──────────────┘  └──────────────┘  └──────────────────┘ │ │
│  │                                                            │ │
│  │  ┌──────────────┐  ┌──────────────────────────────────┐   │ │
│  │  │ SQLite       │  │ Volumes                          │   │ │
│  │  │ (mounted vol)│  │ - /data/sqlite                   │   │ │
│  │  └──────────────┘  │ - /data/blobs                    │   │ │
│  │                    └──────────────────────────────────┘   │ │
│  └────────────────────────────────────────────────────────────┘ │
│                              │                                   │
│                              │ HTTPS (VPN optional)              │
│                              ▼                                   │
│  ┌────────────────────────────────────────────────────────────┐ │
│  │  Remote Users                                              │ │
│  │  ┌──────────────┐  ┌──────────────┐  ┌──────────────────┐ │ │
│  │  │ MudRock App  │  │ MudRock App  │  │ MudRock App      │ │ │
│  │  │ (Home)       │  │ (Office)     │  │ (Field)          │ │ │
│  │  └──────────────┘  └──────────────┘  └──────────────────┘ │ │
│  └────────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
```

**docker-compose.yml**:

```yaml
version: '3.8'

services:
  traefik:
    image: traefik:v2.10
    command:
      - '--api.insecure=true'
      - '--providers.docker=true'
      - '--entrypoints.web.address=:80'
      - '--entrypoints.websecure.address=:443'
      - '--certificatesresolvers.letsencrypt.acme.tlschallenge=true'
      - '--certificatesresolvers.letsencrypt.acme.email=admin@company.com'
      - '--certificatesresolvers.letsencrypt.acme.storage=/letsencrypt/acme.json'
    ports:
      - '80:80'
      - '443:443'
    volumes:
      - '/var/run/docker.sock:/var/run/docker.sock:ro'
      - './letsencrypt:/letsencrypt'

  mudrock-api:
    image: mudrock/api:latest
    environment:
      - DATABASE_PATH=/data/sqlite/mudrock.db
      - BLOB_STORAGE_PATH=/data/blobs
      - JWT_SECRET=${JWT_SECRET}
    volumes:
      - sqlite_data:/data/sqlite
      - blob_data:/data/blobs
    labels:
      - 'traefik.enable=true'
      - 'traefik.http.routers.mudrock.rule=Host(`mudrock.company.com`)'
      - 'traefik.http.routers.mudrock.entrypoints=websecure'
      - 'traefik.http.routers.mudrock.tls.certresolver=letsencrypt'

  minio:
    image: minio/minio:latest
    command: server /data --console-address ":9001"
    environment:
      - MINIO_ROOT_USER=${MINIO_USER}
      - MINIO_ROOT_PASSWORD=${MINIO_PASSWORD}
    volumes:
      - minio_data:/data
    labels:
      - 'traefik.enable=true'
      - 'traefik.http.routers.minio.rule=Host(`storage.company.com`)'

volumes:
  sqlite_data:
  blob_data:
  minio_data:
```

**Pros**:

- ✅ Full data control (self-hosted)
- ✅ Remote access with SSL
- ✅ Easy backup (volume snapshots)
- ✅ Scalable (upgrade VM as needed)

**Cons**:

- ⚠️ Requires VPS management
- ⚠️ Monthly hosting cost (~€15-50/month)

### Option 3: Hybrid (Office Server + Cloud Backup)

**Use Case**: Enterprise with office server, cloud backup for disaster recovery.

```
┌─────────────────────────────────────────────────────────────────┐
│  Office Network                                                  │
├─────────────────────────────────────────────────────────────────┤
│  ┌────────────────────────────────────────────────────────────┐ │
│  │  Primary Server (On-Premise)                               │ │
│  │  ┌──────────────┐  ┌──────────────┐  ┌──────────────────┐ │ │
│  │  │ MudRock API  │  │ SQLite       │  │ NAS Storage      │ │ │
│  │  └──────────────┘  └──────────────┘  └──────────────────┘ │ │
│  └────────────────────────────────────────────────────────────┘ │
│                              │                                   │
│                              │ Nightly backup (encrypted)        │
│                              ▼                                   │
│  ┌────────────────────────────────────────────────────────────┐ │
│  │  Cloud Backup (S3/Azure Blob)                              │ │
│  │  - Encrypted at rest                                        │ │
│  │  - Only blobs + SQLite dump                                │ │
│  │  - Disaster recovery only                                   │ │
│  └────────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
```

**Pros**:

- ✅ Primary data stays on-premise
- ✅ Cloud backup for disaster recovery
- ✅ Low cloud costs (storage only)

### Option 4: Multi-Region Enterprise

**Use Case**: Large enterprise with multiple global offices.

```
┌─────────────────────────────────────────────────────────────────┐
│  Houston Office              │  Aberdeen Office                  │
│  ┌─────────────────┐         │  ┌─────────────────┐             │
│  │ MudRock Server  │         │  │ MudRock Server  │             │
│  │ (Primary)       │◄───────►│  │ (Replica)       │             │
│  └─────────────────┘  Sync   │  └─────────────────┘             │
│                              │                                   │
└──────────────────────────────┴───────────────────────────────────┘
                               │
                               ▼
                    ┌─────────────────────┐
                    │  Central S3 Bucket  │
                    │  (Blob Storage)     │
                    └─────────────────────┘
```

**Sync Strategy**:

- Each office has local server
- Blob storage shared (S3 with replication)
- Metadata synced via REST API between offices
- Conflict resolution: last-write-wins with version vectors

### Comparison with Current Hetzner/Dokploy Setup

| Aspect            | Current (Dokploy)          | Recommended MVP                 |
| ----------------- | -------------------------- | ------------------------------- |
| **Platform**      | Dokploy (Heroku-like)      | Docker Compose or Single Binary |
| **Database**      | PostgreSQL (Supabase)      | SQLite (simpler)                |
| **Complexity**    | High (many services)       | Low (minimal services)          |
| **Cost**          | ~€15/month (VPS)           | ~€5-15/month (smaller VPS)      |
| **Real-time**     | Supabase Realtime (broken) | None needed (Git-like)          |
| **Offline-First** | PowerSync                  | Native SQLite + REST sync       |

**Recommendation**: Start with Option 2 (Private Cloud) for MVP, with ability to deploy as Option 1 (Air-Gapped) for strict enterprise customers.

---

## Implementation Phases

### Phase 1: Core Data Layer (Week 1-2)

- [ ] Set up SQLite schema (client)
- [ ] Implement content-addressed blob store
- [ ] Port LAS parser integration
- [ ] Parquet read/write with DuckDB
- [ ] Basic Tauri commands for data access

### Phase 2: Desktop UI (Week 3-4)

- [ ] SvelteKit project structure
- [ ] LAS file upload flow
- [ ] Curve mapping UI
- [ ] Well list view
- [ ] Basic chart visualization (SciChart)

### Phase 3: Server & Sync (Week 5-6)

- [ ] Rust Axum REST API
- [ ] Authentication (JWT)
- [ ] Sync protocol implementation
- [ ] Blob upload/download
- [ ] Docker deployment

### Phase 4: Polish & Enterprise (Week 7-8)

- [ ] Team/project permissions
- [ ] Offline indicator UI
- [ ] Sync conflict resolution
- [ ] Backup/restore
- [ ] Documentation

---

## Reusable Crates

### From Existing MudRock Implementation

| Crate              | Path                                         | Purpose             | Reuse     |
| ------------------ | -------------------------------------------- | ------------------- | --------- |
| `las-parser`       | `crates/search/document_loading/las-parser/` | LAS file parsing    | ✅ Direct |
| `las-types`        | `crates/search/document_loading/las-types/`  | LAS data structures | ✅ Direct |
| `unit-conversions` | `crates/utils/unit-conversions/`             | Unit conversion     | ✅ Direct |

### Recommended New/External Crates

| Crate              | Purpose             | Why                     |
| ------------------ | ------------------- | ----------------------- |
| **Database**       |                     |                         |
| `rusqlite`         | SQLite bindings     | Mature, well-tested     |
| `sqlx`             | Async SQL           | If async needed         |
| **Parquet/Arrow**  |                     |                         |
| `arrow`            | Arrow arrays        | Core data format        |
| `parquet`          | Parquet files       | Industry standard       |
| `duckdb`           | Query engine        | Fast analytical queries |
| **Serialization**  |                     |                         |
| `serde`            | Serialization       | Standard                |
| `serde_json`       | JSON                | Standard                |
| **Web/API**        |                     |                         |
| `axum`             | Web framework       | Modern, ergonomic       |
| `tower`            | Middleware          | Composable              |
| `reqwest`          | HTTP client         | For sync                |
| **Auth**           |                     |                         |
| `jsonwebtoken`     | JWT                 | Simple auth             |
| `argon2`           | Password hashing    | Secure                  |
| **Storage**        |                     |                         |
| `opendal`          | Storage abstraction | S3/local/etc.           |
| `sha2`             | Hashing             | Content-addressing      |
| **Async**          |                     |                         |
| `tokio`            | Async runtime       | Standard                |
| **Error Handling** |                     |                         |
| `thiserror`        | Error types         | Ergonomic               |
| `anyhow`           | Error handling      | Application errors      |
| **Logging**        |                     |                         |
| `tracing`          | Logging/tracing     | Modern, structured      |
| **Testing**        |                     |                         |
| `insta`            | Snapshot testing    | Easy assertions         |
| `tempfile`         | Temp directories    | Test isolation          |

### Cargo.toml (Core Dependencies)

```toml
[dependencies]
# Database
rusqlite = { version = "0.31", features = ["bundled"] }

# Parquet/Arrow
arrow = "52"
parquet = "52"
duckdb = { version = "0.10", features = ["bundled"] }

# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# Web (server)
axum = "0.7"
tower = "0.4"
tower-http = { version = "0.5", features = ["cors", "trace"] }

# HTTP client (sync)
reqwest = { version = "0.11", features = ["json"] }

# Auth
jsonwebtoken = "9"
argon2 = "0.5"

# Storage
opendal = { version = "0.45", features = ["services-s3", "services-fs"] }
sha2 = "0.10"

# Async
tokio = { version = "1", features = ["full"] }

# Error handling
thiserror = "1.0"
anyhow = "1.0"

# Logging
tracing = "0.1"
tracing-subscriber = "0.3"

# UUID
uuid = { version = "1", features = ["v4", "serde"] }

# Time
chrono = { version = "0.4", features = ["serde"] }
```

---

## Progress Tracking

### Phase 1: Core Data Layer

| Task                         | Status  | Notes                                              |
| ---------------------------- | ------- | -------------------------------------------------- |
| SQLite schema design         | ✅ Done | 30+ tables including OSDU-inspired unit service    |
| Content-addressed blob store | ✅ Done | SHA-256 hashing, hierarchical storage              |
| LAS parser integration       | ✅ Done | Wraps las-parser crate, full metadata extraction   |
| Parquet generation           | ✅ Done | Dual native/gridded storage with Zstd compression  |
| DuckDB queries               | ✅ Done | Integrated for analytical queries                  |
| Tauri commands               | ✅ Done | 30+ commands for auth, ingest, sync, inspector     |
| Well depth grid              | ✅ Done | Configurable resampling with unit conversion       |
| Unit service (OSDU)          | ✅ Done | 15 measurement types, 40+ units with conversions   |
| Curve property dictionary    | ✅ Done | PWLS-style mnemonic mappings with quality tracking |

### Phase 2: Desktop UI

| Task                 | Status      | Notes                                         |
| -------------------- | ----------- | --------------------------------------------- |
| SvelteKit setup      | ✅ Done     | Svelte 5 with runes, Tailwind CSS 4           |
| LAS upload flow      | ✅ Done     | Multi-step wizard with file validation        |
| Curve mapping        | ✅ Done     | Auto-detection with manual override           |
| Well list            | ✅ Done     | Basic list with workspace filtering           |
| AG Grid integration  | ✅ Done     | Inspector with DBeaver-style SQL browser      |
| Auth UI              | ✅ Done     | Login, register, workspace selection          |
| Settings UI          | ✅ Done     | General, members, sync configuration          |
| Curve data viewer    | ✅ Done     | Multi-curve display with DuckDB/Parquet queries |
| Well summary page    | 🟡 Deferred | Home dashboard exists; detail page deferred   |

> **Note**: Per Platform Strategy, DataForge provides **passive visualization** (read-only display of stored data). Interactive charts with computation, derived curves, and parametric modeling belong to the separate Parametric Compute application.

### Phase 3: Server & Sync

| Task                    | Status      | Notes                                        |
| ----------------------- | ----------- | -------------------------------------------- |
| Sync architecture       | ✅ Done     | ColaNode-style Git-like pull model           |
| Sync queue (client)     | ✅ Done     | Offline change tracking with retry logic     |
| Conflict detection      | ✅ Done     | Version-based with manual resolution queue   |
| Conflict resolution UI  | ✅ Done     | Dialog for reviewing and resolving conflicts |
| Axum API server         | ✅ Done     | Production server with SQLite persistence    |
| Server authentication   | ✅ Done     | JWT-based auth middleware with skip option   |
| Blob sync server        | ✅ Done     | OpenDAL S3/MinIO integration with presigned URLs |
| Push/Pull endpoints     | ✅ Done     | Workspace-scoped sync with conflict detection |
| Docker deployment       | ✅ Done     | docker-compose.yml + docker-compose.prod.yml |

### Phase 4: Polish

| Task                    | Status      | Notes                                  |
| ----------------------- | ----------- | -------------------------------------- |
| Team permissions        | ✅ Done     | Owner/admin/member/viewer roles        |
| Offline UI              | ✅ Done     | Status indicator, queue visibility     |
| Conflict resolution     | ✅ Done     | Manual, LastWriteWins, Local/Remote    |
| Workspace members       | ✅ Done     | Add/remove/update member roles         |
| CSV/Excel ingestion     | 🟡 Deferred | UI exists, backend deferred until user demand |
| Data export             | 🟡 Deferred | Deferred until user demand during validation  |
| Backup/restore          | 🟡 Deferred | Deferred post-validation                      |
| Documentation           | 🟡 Deferred | API docs deferred; demo script ready          |

> **Deferred to Parametric App**: Unified views with computation, derived curves, curve splicing, environmental corrections. These involve "silent recompute" and belong to the System of Computation, not the System of Record.

---

## Current Architecture Status

### What's Implemented (Full MVP Stack)

The current implementation includes both **client and server** for multi-user collaboration:

```
┌─────────────────────────────────────────────────────────────────────┐
│  CLIENT (Tauri Desktop App) - SYSTEM OF RECORD                      │
├─────────────────────────────────────────────────────────────────────┤
│  ┌──────────────────┐  ┌──────────────────┐  ┌──────────────────┐  │
│  │ SvelteKit UI     │  │ Rust Backend     │  │ Local Storage    │  │
│  │ ✅ Auth/Login    │  │ ✅ LAS Parser    │  │ ✅ SQLite        │  │
│  │ ✅ Ingest Wizard │  │ ✅ Parquet Gen   │  │ ✅ Blob Store    │  │
│  │ ✅ Inspector     │  │ ✅ Unit Service  │  │ ✅ Sync Queue    │  │
│  │ ✅ Settings      │  │ ✅ Resampling    │  │                  │  │
│  │ ✅ Curve Viewer  │  │ ✅ DuckDB        │  │                  │  │
│  └──────────────────┘  └──────────────────┘  └──────────────────┘  │
└─────────────────────────────────────────────────────────────────────┘
                              │
                              │ REST API (Git-like sync)
                              ▼
┌─────────────────────────────────────────────────────────────────────┐
│  SERVER (Self-Hosted) - IMPLEMENTED                                  │
├─────────────────────────────────────────────────────────────────────┤
│  ✅ REST API (Axum)        - Production binary: dataforge-sync-server│
│  ✅ JWT Authentication     - Middleware with skip option for dev    │
│  ✅ Sync push/pull         - Workspace-scoped with conflict detection│
│  ✅ Blob storage (S3/MinIO)- OpenDAL with presigned URLs            │
│  ✅ Docker deployment      - docker-compose.yml + prod config       │
└─────────────────────────────────────────────────────────────────────┘
```

### Server Implementation Details

**Binary**: `cargo run --bin dataforge-sync-server`

**Endpoints**:
- `GET /api/health` - Health check
- `POST /api/sync/push` - Push changes with conflict detection
- `POST /api/sync/pull` - Pull changes with pagination
- `POST /api/blobs/urls` - Get presigned download URLs
- `POST /api/blobs/upload-urls` - Get presigned upload URLs
- `POST /api/blobs/:hash/uploaded` - Confirm blob upload

**Environment Variables**:
- `SYNC_SERVER_PORT` - Server port (default: 3000)
- `SYNC_SERVER_DB_PATH` - SQLite path (default: ./sync.db)
- `SYNC_AUTH_SECRET` - JWT signing secret
- `SYNC_AUTH_SKIP` - Skip auth for development
- `STORAGE_BACKEND` - "s3", "minio", or "filesystem"
- `STORAGE_BUCKET`, `STORAGE_ENDPOINT`, `STORAGE_ACCESS_KEY`, `STORAGE_SECRET_KEY`

**Docker**: See `docker/docker-compose.yml` for local dev with MinIO

### Key Architectural Achievements

1. **OSDU-Inspired Unit Service** - Centralized unit management with conversion factors
2. **Dual-Storage Model** - Native (original) + Gridded (resampled) Parquet files
3. **PWLS-Style Curve Dictionary** - Mnemonic to canonical property mappings
4. **ColaNode-Style Sync** - Git-like pull-based sync with conflict resolution
5. **Content-Addressed Storage** - SHA-256 based deduplication

---

## Summary

### Key Differentiators from Current MudRock

1. **Simpler Stack**: No Yjs, no CRDT, no WebSocket, no Supabase
2. **Git-like Sync**: Pull-based, not real-time push
3. **Content-Addressed Storage**: Parquet files by hash, immutable
4. **SQLite Everywhere**: Same schema client and server
5. **Single Binary Server**: Can run without Docker
6. **OSDU-Aligned**: Unit service and curve property patterns from industry standard

### Why This Architecture?

1. **Enterprise-Friendly**: Can deploy air-gapped, self-hosted, or hybrid
2. **Offline-First**: Full functionality without network
3. **Data Sovereignty**: All data under customer control
4. **Simple Operations**: Fewer moving parts = fewer failures
5. **Scalable Later**: Can add PostgreSQL, real-time, etc. when needed

### Validation Phase (Current)

DataForge is now **ready for user validation testing**. The core MVP is complete.

#### Validation Narrative

> "DataForge is the local engineering workspace that provides resilience when centralized systems, connectivity, or licenses are unavailable. It's where truth lives — and other tools connect."

#### What to Validate

| Question | Why It Matters |
|----------|----------------|
| Does "workspace" resonate as a mental model? | Positioning clarity |
| Does resilience (vs replacement) feel valuable? | Value proposition |
| Do people trust local-first for production data? | Adoption barrier |
| Would they deploy this internally? | Enterprise readiness |
| Who would own it? IT? Engineering? | Buyer identification |
| Do they ask about computation/derived data? | Signals need for Compute MVP |

#### What NOT to Test Yet

- Feature completeness (we have enough)
- UI perfection (polish comes later)
- Performance tuning (premature optimization)
- Compute/derived curves (wait for user pull)

#### Demo Scenario

One clean story for validation:

```
1. Engineer opens DataForge (offline-capable)
2. Ingests LAS file → auto-detects curves
3. Inspects curves in Curve Viewer (multi-curve display)
4. Reviews data coverage chart (gap detection)
5. Works completely offline (no server needed)
6. When ready, syncs to internal server
7. Another user pulls and sees the same data
```

#### Key Validation Questions to Ask Users

**Resilience-focused questions:**
- "What happens when Petrel / DecisionSpace licenses run out or are unavailable?"
- "What do engineers do when they're offline — on a rig, traveling, or remote?"
- "How do you currently share LAS files between team members?"

**Trust-focused questions:**
- "Would you trust this for production data, or only personal use?"
- "What would make you confident deploying this internally?"

**Architecture-focused questions:**
- "If you could run calculations on this data, what would you compute?"
- "Would you want derived results stored separately from raw data?"

#### Deferred Features (Build Only If Users Ask)

| Feature | Rationale |
|---------|-----------|
| CSV/Excel import | Table stakes, doesn't validate unique value |
| Data export | Build when someone says "I need this" |
| Well detail page | Home dashboard sufficient for demo |
| API documentation | Internal use doesn't need public docs yet |
| **Compute MVP** | Build only if users ask "can I run calculations?" |

### Post-Validation Next Steps

Only after validation confirms direction:

1. **If resilience resonates**: Double down on air-gapped deployment story
2. **If sync is the draw**: Invest in multi-user collaboration features
3. **If computation is requested**: Build Compute MVP (see below)
4. **If export is demanded**: Add LAS/CSV/Parquet export

#### Compute MVP (Planned Follow-on)

If users ask about derived curves, calculations, or "what can I do with this data?", the answer is the **Compute MVP** — a separate reference client that:

- Reads from DataForge (read-only)
- Runs 2-3 simple, explicit computations (moving average, scaling, resampling)
- Writes derived results back to DataForge as immutable artifacts
- Proves the boundary between System of Record and System of Computation

**Build time**: 2-4 weeks after validation confirms demand.

**Purpose**: Reference client, not a second product. Demonstrates that DataForge is extensible without being monolithic.

See [Compute MVP Spec](markdown-guides/DataForge/strategy/DataForge_Compute_MVP.md) for full details.

> **Clarification**: "Visualization" in DataForge means **passive display** of stored data (like a file browser shows files). Interactive charts with derived curves, parametric models, and computed views are planned for separate client applications per the Platform Strategy.

---

**Last Updated**: 2024-12-18
**Authors**: Claude (AI Assistant)
**Status**: 🟢 VALIDATION READY - Core MVP Complete
