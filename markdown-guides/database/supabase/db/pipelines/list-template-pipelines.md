# List Template Pipelines - End-to-End Flow

## Overview

This document describes the complete end-to-end flow for fetching and displaying template pipelines in the projects page. Template pipelines are pre-configured pipelines that users can use as starting points for their own workflows.

## Flow

```
Database (pipelines table with is_template=true)
    ↓
pipeline-service.ts::listTemplatePipelines()
    ↓
template-pipelines.svelte (loads on mount)
    ↓
template-pipelines-item.svelte (renders each template)
```

## Database Schema

### Pipelines Table

The `pipelines` table includes an `is_template` boolean column:

```sql
ALTER TABLE public.pipelines 
ADD COLUMN IF NOT EXISTS is_template BOOLEAN DEFAULT false NOT NULL;
```

**Template Pipeline Characteristics:**
- `is_template = true`: Marks pipeline as a template
- `project_id = '00000000-0000-0000-0000-000000000000'`: Special project ID for templates
- `is_active = true`: Only active templates are shown
- Contains `dag_definition` with `node_ids[]` referencing nodes in the `nodes` table

### Template Nodes

Template pipelines reference nodes from the `nodes` table. These nodes are operator nodes based on UDFs:

- **Shale Volume Nodes**: `linear_vsh`, `larionov_old_vsh`
- **Porosity Nodes**: `density_porosity_sandstone`, `density_porosity_limestone`
- **Saturation Nodes**: `archie_sw`, `simandoux_sw`

## Service Layer

### `pipeline-service.ts`

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

**Query**: Fetches all pipelines where `is_template = true` and `is_active = true`, ordered by most recently updated.

## Frontend Components

### `template-pipelines.svelte`

**Location**: `src/lib/components/pages/projects/template-pipelines/template-pipelines.svelte`

**Responsibilities**:
- Fetches template pipelines on component mount using `$effect`
- Displays loading, error, and empty states
- Renders responsive grid layout using `template-pipelines-item` components

**State Management**:
- `templatePipelines`: Array of `PipelineRow` objects
- `isLoading`: Loading state
- `error`: Error message if fetch fails

**Code Flow**:

```svelte
<script lang="ts">
  import { listTemplatePipelines } from '$lib/services/pipeline-service';
  
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
    } finally {
      isLoading = false;
    }
  }
</script>
```

### `template-pipelines-item.svelte`

**Location**: `src/lib/components/pages/projects/template-pipelines/template-pipelines-item/template-pipelines-item.svelte`

**Responsibilities**:
- Displays individual template pipeline card
- Shows pipeline name, description, and last updated date
- Provides visual thumbnail with first letter of pipeline name

**Props**:
- `pipeline`: `PipelineRow` object containing pipeline metadata

**Display**:
- **Thumbnail**: Gradient background with first letter of pipeline name
- **Name**: Truncated pipeline name with full name on hover
- **Description**: Two-line truncated description
- **Metadata**: Last updated date with relative time formatting

## Template Pipeline Examples

### 1. Basic Shale Volume Analysis
- **Nodes**: Linear Shale Volume, Larionov Old Rocks Vsh
- **Description**: Template pipeline for calculating shale volume from gamma ray logs
- **Tags**: `['shale', 'volume', 'template']`

### 2. Porosity Analysis (Sandstone)
- **Nodes**: Density Porosity (Sandstone)
- **Description**: Template pipeline for calculating porosity from density logs in sandstone formations
- **Tags**: `['porosity', 'sandstone', 'template']`

### 3. Porosity Analysis (Limestone)
- **Nodes**: Density Porosity (Limestone)
- **Description**: Template pipeline for calculating porosity from density logs in limestone formations
- **Tags**: `['porosity', 'limestone', 'template']`

### 4. Water Saturation Analysis (Archie)
- **Nodes**: Archie Water Saturation
- **Description**: Template pipeline for calculating water saturation using Archie equation
- **Tags**: `['saturation', 'water', 'archie', 'template']`

### 5. Comprehensive Petrophysical Analysis
- **Nodes**: Linear Shale Volume, Density Porosity (Sandstone), Archie Water Saturation
- **Description**: Complete template pipeline combining shale volume, porosity, and water saturation calculations
- **Tags**: `['comprehensive', 'petrophysics', 'template']`

## SQL Migrations

### Migration 1: Add `is_template` Column

**File**: `db/migrations/003-add-is-template-to-pipelines.sql`

```sql
ALTER TABLE public.pipelines 
ADD COLUMN IF NOT EXISTS is_template BOOLEAN DEFAULT false NOT NULL;

CREATE INDEX IF NOT EXISTS idx_pipelines_is_template 
ON public.pipelines(is_template) 
WHERE is_template = true;
```

### Migration 2: Create Template Nodes and Pipelines

**File**: `db/migrations/004-create-template-pipelines.sql`

- Creates 6 template nodes based on UDFs
- Creates 5 template pipelines with appropriate node references
- Creates `pipeline_node_references` entries for positioning

## Integration Points

### Projects Page

**Location**: `src/lib/components/pages/projects/projects-page.svelte`

The projects page integrates the template pipelines component:

```svelte
<script lang="ts">
  import TemplatePipelines from './template-pipelines/template-pipelines.svelte';
</script>

<div class="h-full flex">
  <SidebarProjects />
  <div class="flex-1 min-h-0 flex flex-col">
    <TemplatePipelines />
  </div>
</div>
```

## Future Enhancements

### 🚀 **Template Pipeline Duplication**
- Allow users to duplicate template pipelines into their own projects
- Copy nodes and create new pipeline with user's project_id

### 🚀 **Template Pipeline Preview**
- Show pipeline DAG visualization before duplication
- Display node count and workflow description

### 🚀 **Template Categories**
- Organize templates by category (shale volume, porosity, saturation)
- Filter templates by tags

### 🚀 **Template Search**
- Search templates by name, description, or tags
- Filter by UDF categories

## Summary

The template pipeline system provides users with pre-configured starting points for common petrophysical workflows. By leveraging the existing pipeline and node architecture, templates can be easily created, maintained, and displayed in a consistent grid layout.

**Key Design Decisions**:
- ✅ **Database Flag**: `is_template` column clearly marks template pipelines
- ✅ **Special Project ID**: Templates use a reserved project ID for isolation
- ✅ **Node Reuse**: Templates reference shared nodes from the `nodes` table
- ✅ **Responsive Grid**: Template cards automatically rearrange based on viewport size
- ✅ **Type Safety**: TypeScript types ensure correct data structures throughout

