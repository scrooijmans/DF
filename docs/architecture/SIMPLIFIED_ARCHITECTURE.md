# MudRock Simplified Architecture

## Overview

This document defines the simplified 4-layer storage architecture for MudRock, based on architectural decisions made for an offline-first, enterprise-grade geoscience data platform.

## Key Decisions

### Technology Choices

| Component | Current | Recommended | Reason |
|-----------|---------|-------------|--------|
| CRDT Library | Automerge | **Yjs** | Faster, smaller (30KB vs 300KB), better ecosystem |
| Collaboration Server | Supabase Realtime (broken) | **Hocuspocus** | Yjs-native, reliable, PostgreSQL persistence |
| Database | Supabase PostgreSQL | **Keep** | Self-hosted, RLS, relationships |
| Object Storage | S3/MinIO | **Keep** | Parquet files for large arrays |
| Offline Sync | PowerSync | **Keep** | SQLite sync for large data |
| Local Cache | - | **SQLite + Local Parquet** | Offline operation via Tauri |

### What We're Removing (Deferred)

- Pipeline DAGs
- Node configs
- Notebook cells
- Executions/audit trail
- All `*_ops`, `*_snapshots`, `*_actors` CRDT tables

### What We're Keeping (Simplified)

- Projects, teams, users (PostgreSQL)
- Wells, curves metadata (PostgreSQL)
- Curve data as Parquet files
- Computed results as Parquet files (same as curve data)

---

## 4-Layer Storage Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                        CLIENT (Tauri)                          │
├─────────────────────────────────────────────────────────────────┤
│  Layer 4: Local Cache                                           │
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐ │
│  │ SQLite (Tauri)  │  │ Local Parquet   │  │ Yjs IndexedDB   │ │
│  │ Metadata cache  │  │ Curve data      │  │ Chart docs      │ │
│  └─────────────────┘  └─────────────────┘  └─────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
                              │
                              │ Sync when online
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                         SERVER                                  │
├─────────────────────────────────────────────────────────────────┤
│  Layer 1: PostgreSQL          │  Layer 2: Hocuspocus            │
│  ┌─────────────────────────┐  │  ┌─────────────────────────┐   │
│  │ projects, teams, users  │  │  │ Yjs collaboration        │   │
│  │ wells, curves metadata  │  │  │ Chart documents          │   │
│  │ chart metadata (FK)     │  │  │ Real-time sync           │   │
│  │ selections              │  │  │ Persisted to PostgreSQL  │   │
│  └─────────────────────────┘  │  └─────────────────────────┘   │
├───────────────────────────────┴─────────────────────────────────┤
│  Layer 3: Object Storage (S3/MinIO)                             │
│  ┌─────────────────────────────────────────────────────────────┐│
│  │ Parquet files: well curves, computed results                ││
│  │ Path: s3://{bucket}/{project_id}/wells/{well_id}/curves/    ││
│  └─────────────────────────────────────────────────────────────┘│
└─────────────────────────────────────────────────────────────────┘
```

---

## Layer 1: PostgreSQL (Relational Metadata)

### Simplified Schema

**Tables to KEEP:**

```sql
-- Core entities
projects          -- Project metadata, ownership
teams             -- Team organization
team_members      -- Team membership
project_permissions -- Access control

-- Domain data (metadata only)
wells             -- Well location, name, project FK
curves            -- Curve metadata, parquet file path
curve_metadata    -- Curve type definitions (GR, RHOB, etc.)

-- Visualization
charts            -- Chart metadata + yjs_doc_id reference
chart_types       -- Chart type definitions
chart_series      -- Which curves are on which charts

-- User workflow
selections        -- User-defined data selections
selection_items   -- Items in a selection

-- System
spatial_ref_sys   -- PostGIS reference (if needed)
```

**Tables to REMOVE:**

```sql
-- CRDT operation tables (replaced by Yjs)
chart_actors, chart_ops, chart_snapshots
pipeline_actors, pipeline_ops, pipeline_snapshots
node_ops, node_snapshots
project_ops, project_snapshots

-- Deferred features
pipelines, pipeline_node_references, pipeline_lineage
nodes, chart_node_assignments
notebooks
node_executions, pipeline_executions, chart_executions

-- Can remove
audit_log         -- Optional, add back if needed
operator_registry -- Deferred until pipelines return
polygons          -- Keep if used, remove if not
project_automerge_indexes -- Replace with yjs_doc_id
```

### Updated Charts Table

```sql
CREATE TABLE charts (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  project_id UUID NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
  name TEXT NOT NULL,
  description TEXT,
  chart_type TEXT NOT NULL,

  -- Yjs document reference (replaces automerge_url)
  yjs_doc_id TEXT NOT NULL UNIQUE,

  -- Denormalized config for quick queries (Yjs is source of truth)
  chart_config JSONB DEFAULT '{}',
  data_source_config JSONB DEFAULT '{}',

  -- Metadata
  created_by UUID NOT NULL REFERENCES auth.users(id),
  created_at TIMESTAMPTZ DEFAULT NOW(),
  updated_at TIMESTAMPTZ DEFAULT NOW(),
  tags TEXT[],
  is_active BOOLEAN DEFAULT true
);

-- Index for fast lookup
CREATE INDEX idx_charts_yjs_doc_id ON charts(yjs_doc_id);
CREATE INDEX idx_charts_project_id ON charts(project_id);
```

### Updated Curves Table

```sql
CREATE TABLE curves (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  well_id INTEGER NOT NULL REFERENCES wells(id) ON DELETE CASCADE,

  -- Curve identification
  curve_id TEXT NOT NULL,
  curve_name TEXT NOT NULL,
  curve_metadata_id UUID REFERENCES curve_metadata(id),

  -- Parquet storage reference
  parquet_file_path TEXT NOT NULL,  -- s3://bucket/project/well/curves/curve.parquet
  parquet_column_name TEXT,         -- Column name in parquet file

  -- Statistics (denormalized from parquet for queries)
  row_count INTEGER,
  min_value DOUBLE PRECISION,
  max_value DOUBLE PRECISION,

  -- Metadata
  created_at TIMESTAMPTZ DEFAULT NOW(),
  updated_at TIMESTAMPTZ DEFAULT NOW(),
  created_by UUID REFERENCES auth.users(id),

  -- Computed curves reference their source
  is_computed BOOLEAN DEFAULT false,
  source_curve_ids UUID[],  -- For computed curves, track lineage

  UNIQUE(well_id, curve_id)
);
```

---

## Layer 2: Yjs + Hocuspocus (Collaborative Documents)

### What Goes in Yjs

Charts are the primary collaborative document. Each chart has:

```typescript
// Yjs document structure for a chart
interface YjsChartDocument {
  // Chart configuration
  config: Y.Map<{
    chartType: string;
    title: string;
    description: string;

    // Axis configuration
    axes: {
      x: AxisConfig;
      y: AxisConfig[];
    };

    // Series configuration
    series: SeriesConfig[];

    // Display settings
    theme: string;
    gridEnabled: boolean;
    legendPosition: string;
  }>;

  // Data source bindings (which curves to display)
  dataSources: Y.Array<{
    curveId: string;
    seriesIndex: number;
    transform?: TransformConfig;
  }>;

  // Annotations (collaborative)
  annotations: Y.Array<{
    id: string;
    type: 'marker' | 'line' | 'region';
    position: Position;
    text?: string;
    style?: Style;
  }>;

  // Awareness (cursor positions, selections)
  // Handled automatically by Yjs awareness protocol
}
```

### Hocuspocus Server Setup

```typescript
// server/hocuspocus.ts
import { Server } from '@hocuspocus/server';
import { Database } from '@hocuspocus/extension-database';
import { Logger } from '@hocuspocus/extension-logger';
import pg from 'pg';

const pool = new pg.Pool({
  connectionString: process.env.DATABASE_URL,
});

const server = Server.configure({
  port: 3030,

  extensions: [
    new Logger(),
    new Database({
      // Load document from PostgreSQL
      fetch: async ({ documentName }) => {
        const result = await pool.query(
          'SELECT yjs_state FROM yjs_documents WHERE doc_id = $1',
          [documentName]
        );
        return result.rows[0]?.yjs_state || null;
      },

      // Save document to PostgreSQL
      store: async ({ documentName, state }) => {
        await pool.query(
          `INSERT INTO yjs_documents (doc_id, yjs_state, updated_at)
           VALUES ($1, $2, NOW())
           ON CONFLICT (doc_id) DO UPDATE SET
             yjs_state = $2,
             updated_at = NOW()`,
          [documentName, state]
        );
      },
    }),
  ],

  // Authentication
  async onAuthenticate({ token }) {
    // Verify JWT from Supabase
    const user = await verifySupabaseToken(token);
    if (!user) throw new Error('Unauthorized');
    return { user };
  },

  // Authorization
  async onConnect({ documentName, context }) {
    // Check if user has access to this chart's project
    const chartId = documentName.replace('chart:', '');
    const hasAccess = await checkChartAccess(context.user.id, chartId);
    if (!hasAccess) throw new Error('Forbidden');
  },
});

server.listen();
```

### Yjs Documents Table

```sql
-- Store Yjs document state in PostgreSQL
CREATE TABLE yjs_documents (
  doc_id TEXT PRIMARY KEY,
  yjs_state BYTEA NOT NULL,
  created_at TIMESTAMPTZ DEFAULT NOW(),
  updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_yjs_documents_updated ON yjs_documents(updated_at);
```

### Client-Side Yjs Integration

```typescript
// src/lib/yjs/chart-provider.ts
import * as Y from 'yjs';
import { HocuspocusProvider } from '@hocuspocus/provider';
import { IndexeddbPersistence } from 'y-indexeddb';

export function createChartProvider(chartId: string, authToken: string) {
  const doc = new Y.Doc();

  // Local persistence (offline support)
  const indexeddbProvider = new IndexeddbPersistence(`chart:${chartId}`, doc);

  // Remote sync
  const hocuspocusProvider = new HocuspocusProvider({
    url: 'ws://localhost:3030',
    name: `chart:${chartId}`,
    document: doc,
    token: authToken,

    onConnect: () => console.log('Connected to Hocuspocus'),
    onDisconnect: () => console.log('Disconnected from Hocuspocus'),
    onSynced: ({ state }) => console.log('Synced:', state),
  });

  return {
    doc,
    provider: hocuspocusProvider,
    indexeddb: indexeddbProvider,

    // Cleanup
    destroy: () => {
      hocuspocusProvider.destroy();
      indexeddbProvider.destroy();
    },
  };
}
```

---

## Layer 3: Parquet in S3/MinIO (Large Array Data)

### Storage Layout

```
s3://mudrock-data/
├── {project_id}/
│   └── wells/
│       └── {well_id}/
│           └── curves/
│               ├── raw/
│               │   ├── GR.parquet
│               │   ├── RHOB.parquet
│               │   └── NPHI.parquet
│               └── computed/
│                   ├── VSH_computed_{timestamp}.parquet
│                   └── PHI_computed_{timestamp}.parquet
```

### Parquet File Format

Each curve is stored as a single-column parquet file with an index column:

```
┌─────────────┬────────────────┐
│ depth_index │ value          │
├─────────────┼────────────────┤
│ 1000.0      │ 45.2           │
│ 1000.5      │ 46.1           │
│ 1001.0      │ 44.8           │
│ ...         │ ...            │
└─────────────┴────────────────┘
```

Metadata in parquet file footer:
- `curve_id`: UUID reference
- `well_id`: Well reference
- `unit`: Physical unit (e.g., "gAPI", "g/cm3")
- `created_at`: Timestamp
- `created_by`: User ID
- `source_curve_ids`: For computed curves

### DuckDB Query Engine

```rust
// src-tauri/src/parquet_query.rs
use duckdb::{Connection, Result};

pub fn query_curve_data(
    parquet_path: &str,
    depth_min: Option<f64>,
    depth_max: Option<f64>,
) -> Result<Vec<(f64, f64)>> {
    let conn = Connection::open_in_memory()?;

    let query = format!(
        "SELECT depth_index, value FROM read_parquet('{}')
         WHERE depth_index >= {} AND depth_index <= {}
         ORDER BY depth_index",
        parquet_path,
        depth_min.unwrap_or(f64::MIN),
        depth_max.unwrap_or(f64::MAX)
    );

    let mut stmt = conn.prepare(&query)?;
    let rows = stmt.query_map([], |row| {
        Ok((row.get::<_, f64>(0)?, row.get::<_, f64>(1)?))
    })?;

    rows.collect()
}
```

---

## Layer 4: Local Cache (SQLite + Local Parquet)

### SQLite Schema for Offline

```sql
-- Local SQLite for offline metadata cache
CREATE TABLE local_projects (
  id TEXT PRIMARY KEY,
  name TEXT NOT NULL,
  owner_id TEXT NOT NULL,
  synced_at TEXT,
  json_data TEXT  -- Full project data as JSON
);

CREATE TABLE local_wells (
  id INTEGER PRIMARY KEY,
  project_id TEXT NOT NULL,
  name TEXT NOT NULL,
  synced_at TEXT
);

CREATE TABLE local_curves (
  id TEXT PRIMARY KEY,
  well_id INTEGER NOT NULL,
  curve_id TEXT NOT NULL,
  curve_name TEXT NOT NULL,
  local_parquet_path TEXT,  -- Path to local parquet file
  synced_at TEXT,
  needs_upload BOOLEAN DEFAULT false  -- For computed curves created offline
);

CREATE TABLE local_charts (
  id TEXT PRIMARY KEY,
  project_id TEXT NOT NULL,
  yjs_doc_id TEXT NOT NULL,
  synced_at TEXT
);

-- Sync queue for offline changes
CREATE TABLE sync_queue (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  entity_type TEXT NOT NULL,  -- 'curve', 'chart', etc.
  entity_id TEXT NOT NULL,
  action TEXT NOT NULL,       -- 'create', 'update', 'delete'
  payload TEXT,               -- JSON data
  created_at TEXT DEFAULT CURRENT_TIMESTAMP,
  attempts INTEGER DEFAULT 0
);
```

### Local File Structure

```
{app_data}/
├── cache/
│   └── {project_id}/
│       └── wells/
│           └── {well_id}/
│               └── curves/
│                   ├── GR.parquet      # Downloaded from S3
│                   └── VSH_local.parquet # Created offline
├── sqlite/
│   └── mudrock.db
└── yjs/
    └── (IndexedDB managed by browser)
```

---

## Data Flow Examples

### 1. User Opens a Chart (Online)

```
1. Client requests chart metadata from PostgreSQL
2. Client connects to Hocuspocus with chart's yjs_doc_id
3. Yjs syncs chart config from server (or loads from IndexedDB if cached)
4. Client reads curve IDs from chart config
5. Client queries PostgreSQL for curve metadata (parquet paths)
6. Client fetches parquet files from S3 (or uses local cache)
7. DuckDB loads parquet data for visualization
8. SciChart renders the data
```

### 2. User Edits Chart Configuration (Online)

```
1. User changes axis scale or adds annotation
2. Yjs updates local document
3. Hocuspocus broadcasts change to other users
4. IndexedDB saves local copy
5. Hocuspocus persists to PostgreSQL (debounced)
```

### 3. User Creates Computed Curve (Offline)

```
1. User runs UDF on local parquet data
2. Result saved as local parquet file
3. SQLite records new curve with needs_upload=true
4. Sync queue entry created
5. When online:
   a. Upload parquet to S3
   b. Create curve record in PostgreSQL
   c. Clear needs_upload flag
```

### 4. Collaborator Sees Real-time Changes

```
1. User A edits chart annotation
2. Yjs broadcasts through Hocuspocus
3. User B's Yjs client receives update
4. Svelte reactivity updates UI
5. Both users see the same annotation
```

---

## Migration Plan

### Phase 1: Simplify PostgreSQL Schema

1. Create migration to drop unused tables
2. Update `database.types.ts`
3. Remove related TypeScript code

### Phase 2: Replace Automerge with Yjs

1. Install Yjs packages:
   ```bash
   npm install yjs @hocuspocus/provider y-indexeddb
   ```
2. Create Yjs document providers
3. Migrate chart state management
4. Remove Automerge dependencies

### Phase 3: Deploy Hocuspocus Server

1. Set up Hocuspocus on existing VPS
2. Configure PostgreSQL persistence
3. Add authentication middleware
4. Update client to use Hocuspocus

### Phase 4: Remove Automerge Code

1. Remove Automerge packages
2. Delete Automerge-related state files
3. Remove `automerge_url` columns
4. Clean up unused CRDT tables

---

## Package Changes

### Remove

```json
{
  "@automerge/automerge": "remove",
  "@automerge/automerge-repo": "remove",
  "@automerge/automerge-repo-network-websocket": "remove",
  "@automerge/automerge-repo-network-broadcastchannel": "remove",
  "@automerge/automerge-repo-storage-indexeddb": "remove",
  "@automerge/automerge-repo-svelte-store": "remove"
}
```

### Add

```json
{
  "yjs": "^13.6.0",
  "@hocuspocus/provider": "^2.0.0",
  "y-indexeddb": "^9.0.0",
  "y-protocols": "^1.0.0"
}
```

### Server (new package)

```json
{
  "@hocuspocus/server": "^2.0.0",
  "@hocuspocus/extension-database": "^2.0.0",
  "@hocuspocus/extension-logger": "^2.0.0"
}
```

---

## Summary

This simplified architecture:

1. **Reduces complexity** by removing pipelines, nodes, notebooks, and CRDT operation tables
2. **Improves performance** by switching from Automerge to Yjs
3. **Maintains offline-first** with SQLite + local Parquet + Yjs IndexedDB
4. **Enables collaboration** through Hocuspocus WebSocket sync
5. **Scales for enterprise** with PostgreSQL + S3 for authoritative data
6. **Keeps existing infrastructure** (Hetzner VPS, self-hosted Supabase components)

The result is a focused MVP that handles:
- Well data (metadata in PostgreSQL, arrays in Parquet)
- Collaborative charts (Yjs + Hocuspocus)
- Offline computation (local DuckDB + Parquet)
- Team sharing (PostgreSQL relationships + RLS)
