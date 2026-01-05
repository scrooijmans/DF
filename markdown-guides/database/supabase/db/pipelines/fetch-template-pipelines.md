# Fetch Template Pipelines - End-to-End Flow

## Overview

This document describes the complete end-to-end flow for fetching and displaying template pipelines. Template pipelines are pre-configured pipelines stored in PostgreSQL that users can browse and use as starting points for their own workflows.

## Architecture: Pure SvelteKit (No Rust Backend)

**Important**: This flow uses **pure SvelteKit** with direct Supabase PostgREST API calls. **No Rust backend functions** are involved, and **no storage layer/OpenDAL functionality** is used. This is a simple metadata fetch operation from PostgreSQL.

## Complete Data Flow

```
PostgreSQL (pipelines table)
    ↓
Supabase PostgREST API (auto-generated REST endpoint)
    ↓
Supabase JavaScript Client (via Tauri HTTP plugin)
    ↓
pipeline-service.ts::listTemplatePipelines()
    ↓
template-pipelines.svelte (component state)
    ↓
template-pipelines-item.svelte (renders each template card)
```

## Step-by-Step Flow

### 1. Database Query (PostgreSQL)

**Table**: `public.pipelines`

**Query Logic**:

```sql
SELECT
    id, name, description, project_id, version, tags,
    created_at, updated_at, is_template
FROM pipelines
WHERE is_template = true
  AND is_active = true
ORDER BY updated_at DESC;
```

**Filtering**:

- `is_template = true`: Only template pipelines
- `is_active = true`: Only active templates (soft-deleted templates excluded)

**Index Used**: `idx_pipelines_is_template` (partial index on `is_template = true`)

### 2. Supabase PostgREST API

**Endpoint**: `GET /rest/v1/pipelines`

**Query Parameters**:

- `select=id,name,description,project_id,version,tags,created_at,updated_at,is_template`
- `is_template=eq.true`
- `is_active=eq.true`
- `order=updated_at.desc`

**Full URL Example**:

```
GET /rest/v1/pipelines?select=id,name,description,project_id,version,tags,created_at,updated_at,is_template&is_template=eq.true&is_active=eq.true&order=updated_at.desc
```

**Response Format**:

```json
[
  {
    "id": "11111111-1111-1111-1111-111111111111",
    "name": "Basic Shale Volume Analysis",
    "description": "Template pipeline for calculating shale volume from gamma ray logs",
    "project_id": "00000000-0000-0000-0000-000000000000",
    "version": "1.0.0",
    "tags": ["shale", "volume", "template"],
    "created_at": "2025-01-20T00:00:00Z",
    "updated_at": "2025-01-20T00:00:00Z",
    "is_template": true
  },
  ...
]
```

### 3. Supabase JavaScript Client

**Location**: `src/lib/supabase.ts`

**Client Configuration**:

- Uses Tauri HTTP plugin for network requests
- Authenticated via Supabase Auth (JWT tokens)
- Connects to Supabase PostgREST API

**No Rust Backend**: The Supabase client runs entirely in the browser/Electron context, making HTTP requests directly to Supabase's REST API.

### 4. Service Layer: `pipeline-service.ts`

**Location**: `src/lib/services/pipeline-service.ts`

**Function**: `listTemplatePipelines()`

```typescript
export async function listTemplatePipelines(): Promise<PipelineRow[]> {
  const { data, error } = await supabase
    .from("pipelines")
    .select(
      `id, name, description, project_id, version, tags, created_at, updated_at, is_template`,
    )
    .eq("is_template", true)
    .eq("is_active", true)
    .order("updated_at", { ascending: false });

  if (error) throw new Error(error.message);
  return (data as PipelineRow[]) ?? [];
}
```

**What Happens**:

1. **Supabase Query Builder**: Uses Supabase's query builder to construct the PostgREST request
2. **Type Safety**: Returns `PipelineRow[]` with TypeScript types
3. **Error Handling**: Throws errors if the request fails
4. **No Data Transformation**: Returns raw database rows (no processing needed)

**No Storage Layer**: This function only fetches metadata from PostgreSQL. No file storage operations, no Parquet reading, no OpenDAL operations.

### 5. Component: `template-pipelines.svelte`

**Location**: `src/lib/components/pages/projects/template-pipelines/template-pipelines.svelte`

**Responsibilities**:

- Fetches template pipelines on component mount
- Manages loading, error, and empty states
- Renders responsive grid layout

**State Management**:

```svelte
<script lang="ts">
  import { listTemplatePipelines } from '$lib/services/pipeline-service';
  import type { PipelineRow } from '$lib/services/pipeline-service';

  let templatePipelines = $state<PipelineRow[]>([]);
  let isLoading = $state(true);
  let error = $state<string | null>(null);

  $effect(() => {
    loadTemplatePipelines();
  });

  async function loadTemplatePipelines() {
    try {
      isLoading = true;
      error = null;
      templatePipelines = await listTemplatePipelines();
    } catch (e: any) {
      error = e?.message ?? 'Failed to load template pipelines';
      console.error('Failed to load template pipelines:', e);
    } finally {
      isLoading = false;
    }
  }
</script>
```

**Reactivity**:

- `$effect()` runs once on component mount
- Automatically calls `loadTemplatePipelines()` when component initializes
- Updates `templatePipelines` state when fetch completes

**UI States**:

- **Loading**: Shows "Loading templates..." message
- **Error**: Shows error message if fetch fails
- **Empty**: Shows "No template pipelines available" if array is empty
- **Success**: Renders grid of template pipeline cards

### 6. Component: `template-pipelines-item.svelte`

**Location**: `src/lib/components/pages/projects/template-pipelines/template-pipelines-item/template-pipelines-item.svelte`

**Responsibilities**:

- Displays individual template pipeline card
- Shows pipeline metadata (name, description, updated date)
- Provides visual thumbnail

**Props**:

```typescript
interface Props {
  pipeline: PipelineRow;
}
```

**Display Elements**:

- **Thumbnail**: Gradient background with first letter of pipeline name
- **Name**: Truncated pipeline name (full name on hover via `title` attribute)
- **Description**: Two-line truncated description (`line-clamp-2`)
- **Metadata**: Last updated date with relative time formatting

**Styling**:

- Uses `STYLE_CONSTANTS` for consistent spacing, fonts, and colors
- Responsive card layout with hover effects
- Grid layout automatically adjusts based on viewport size

## Database Schema

### Pipelines Table Structure

```sql
CREATE TABLE public.pipelines (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    project_id UUID NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    version TEXT NOT NULL DEFAULT '1.0.0',
    dag_definition JSONB NOT NULL,
    created_by UUID,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    tags TEXT[] DEFAULT '{}',
    is_active BOOLEAN DEFAULT true,
    is_public BOOLEAN DEFAULT false,
    is_template BOOLEAN DEFAULT false NOT NULL, -- Template flag

    CONSTRAINT pipelines_project_name_version_unique UNIQUE (project_id, name, version)
);

-- Index for efficient template queries
CREATE INDEX idx_pipelines_is_template
ON public.pipelines(is_template)
WHERE is_template = true;
```

### Template Pipeline Characteristics

- **`is_template = true`**: Marks pipeline as a template
- **`project_id = '00000000-0000-0000-0000-000000000000'`**: Special reserved project ID for templates
- **`is_active = true`**: Only active templates are shown
- **`dag_definition`**: Contains JSONB with `node_ids[]` array referencing nodes in `nodes` table

## Technology Stack

### ✅ **What We're Using**

1. **SvelteKit**: Frontend framework
2. **Supabase JavaScript Client**: Direct PostgREST API calls
3. **PostgreSQL**: Database storage (via Supabase)
4. **TypeScript**: Type safety

### ❌ **What We're NOT Using**

1. **Rust Backend Functions**: No Tauri commands involved
2. **Storage Layer**: No `mudrock-storage-manager` or `opendal-storage-adapter`
3. **OpenDAL**: No file storage operations
4. **Parquet Processing**: No data file reading/writing
5. **MinIO/S3**: No object storage access

## Why Pure SvelteKit?

### **Simple Metadata Fetch**

Template pipeline fetching is a **simple metadata operation**:

- Only reads from PostgreSQL `pipelines` table
- No file I/O operations needed
- No data processing required
- No complex transformations

### **Performance Benefits**

- **Direct API Calls**: No intermediate Rust layer
- **Fast Response**: PostgREST is optimized for simple queries
- **Efficient Indexing**: Partial index on `is_template = true` ensures fast queries
- **Client-Side Caching**: Can be cached in browser if needed

### **Simplicity**

- **Fewer Dependencies**: No need for Rust backend coordination
- **Easier Debugging**: Direct HTTP requests visible in network tab
- **Type Safety**: TypeScript types ensure correct data structures
- **Maintainability**: Simple, straightforward code path

## Comparison: Template Fetch vs. Pipeline Execution

### **Template Fetch (This Flow)**

- ✅ **Pure SvelteKit**: Direct Supabase API calls
- ✅ **Metadata Only**: Reads from PostgreSQL
- ✅ **No Storage Layer**: No file operations
- ✅ **Fast**: Simple query, returns immediately

### **Pipeline Execution (Future)**

- 🔄 **Rust Backend**: Tauri commands for execution
- 🔄 **Storage Layer**: OpenDAL for reading/writing Parquet files
- 🔄 **Data Processing**: DataFusion for query execution
- 🔄 **Complex**: Multiple components involved

## Error Handling

### Service Layer Errors

```typescript
try {
  templatePipelines = await listTemplatePipelines();
} catch (e: any) {
  error = e?.message ?? "Failed to load template pipelines";
  console.error("Failed to load template pipelines:", e);
}
```

**Common Error Scenarios**:

- **Network Errors**: Connection timeout, DNS failure
- **Authentication Errors**: Invalid JWT token, expired session
- **Database Errors**: RLS policy violations, connection issues
- **Query Errors**: Invalid SQL syntax (unlikely with Supabase client)

### Component Error Display

```svelte
{:else if error}
  <div class="{STYLE_CONSTANTS.FONT_SIZE.DEFAULT} text-red-600">
    {error}
  </div>
{/if}
```

## Performance Considerations

### **Query Optimization**

- **Partial Index**: `idx_pipelines_is_template` only indexes rows where `is_template = true`
- **Selective Columns**: Only fetches needed columns (not full `dag_definition`)
- **Ordering**: Orders by `updated_at DESC` for most recent templates first

### **Caching Opportunities**

**Future Enhancement**: Could add client-side caching:

```typescript
// Future: Add caching layer
const templateCache = new Map<
  string,
  { data: PipelineRow[]; timestamp: number }
>();
const CACHE_TTL = 5 * 60 * 1000; // 5 minutes

export async function listTemplatePipelines(): Promise<PipelineRow[]> {
  const cacheKey = "template-pipelines";
  const cached = templateCache.get(cacheKey);

  if (cached && Date.now() - cached.timestamp < CACHE_TTL) {
    return cached.data;
  }

  const data = await fetchFromSupabase();
  templateCache.set(cacheKey, { data, timestamp: Date.now() });
  return data;
}
```

## Integration Points

### Projects Page

**Location**: `src/lib/components/pages/projects/projects-page.svelte`

```svelte
<script lang="ts">
  import SidebarProjects from './sidebar-projects/sidebar-projects.svelte';
  import TemplatePipelines from './template-pipelines/template-pipelines.svelte';
</script>

<div class="h-full flex">
  <SidebarProjects />
  <div class="flex-1 min-h-0 flex flex-col">
    <TemplatePipelines />
  </div>
</div>
```

**Layout**:

- **Left Sidebar**: Project list (`SidebarProjects`)
- **Right Content**: Template pipelines grid (`TemplatePipelines`)

## Future Enhancements

### 🚀 **Template Pipeline Duplication**

**Future Flow** (will involve Rust backend):

```
User clicks "Duplicate Template"
    ↓
template-pipelines-item.svelte::handleDuplicate()
    ↓
Tauri Command: duplicate_template_pipeline()
    ↓
Rust Backend:
  - Copy pipeline row with new project_id
  - Copy referenced nodes
  - Create pipeline_node_references
    ↓
Return new pipeline ID
    ↓
Navigate to DAG editor with new pipeline
```

### 🚀 **Template Pipeline Preview**

- Show DAG visualization before duplication
- Display node count and workflow description
- Preview node configurations

### 🚀 **Template Categories & Filtering**

- Filter templates by tags
- Search templates by name/description
- Group by UDF categories

## Summary

The template pipeline fetch flow is a **pure SvelteKit implementation** that:

- ✅ **Fetches metadata** from PostgreSQL via Supabase PostgREST
- ✅ **No Rust backend** functions involved
- ✅ **No storage layer** operations (no OpenDAL, no file I/O)
- ✅ **Simple and fast** - direct API calls with efficient indexing
- ✅ **Type-safe** - TypeScript ensures correct data structures
- ✅ **Reactive** - Svelte 5 runes provide automatic UI updates

**Key Design Decisions**:

- ✅ **Pure Frontend**: Simple metadata fetch doesn't need Rust backend
- ✅ **Direct API Calls**: Supabase PostgREST provides efficient access
- ✅ **Type Safety**: TypeScript types match database schema
- ✅ **Performance**: Partial index ensures fast template queries
- ✅ **Simplicity**: Straightforward code path, easy to maintain

This architecture keeps template fetching simple and performant while reserving Rust backend and storage layer operations for more complex tasks like pipeline execution and data processing.
