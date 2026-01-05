# Create Pipeline - End-to-End Flow (Frontend → Supabase)

## Overview

This describes how a pipeline is created from the UI using the Supabase JavaScript client (via the Tauri HTTP plugin). It mirrors the Project creation flow: metadata is stored in Postgres, with a minimal `dag_definition` stub so the pipeline can be edited later in the DAG Editor.

## Flow

```
User → new-pipeline.svelte → pipeline-service.ts → Supabase (PostgREST) → PostgreSQL
```

## Frontend

### `new-pipeline.svelte`

- Reads `currentSelectedProjectId` from `PostgresProjectsState`.
- Validates name and that a project is selected.
- Calls `createPipelineJS({ projectId, name, description })`.

```svelte
<script lang="ts">
  import { getPostgresProjectsState } from "$lib/state/postgres/postgres-projects-state.svelte";
  import { createPipelineJS } from "$lib/services/pipeline-service";
  const projectsState = getPostgresProjectsState();
  async function handleSubmit(){
    const row = await createPipelineJS({
      projectId: projectsState.currentSelectedProjectId!,
      name: name.trim(),
      description,
    });
  }
</script>
```

### `pipeline-service.ts`

- Inserts a pipeline row and supplies a minimal `dag_definition` to satisfy NOT NULL.
- Exposes `listPipelinesByProject(projectId)` for the DAG Editor list.

```ts
export async function createPipelineJS(params: {
  projectId: string;
  name: string;
  description?: string | null;
}) {
  const dagDefinition = {
    id: crypto.randomUUID?.() ?? `${Date.now()}-${Math.random()}`,
    name: params.name,
    project_id: params.projectId,
    nodes: [],
    edges: [],
    version: "1.0.0",
    metadata: {
      description: params.description ?? null,
      created_by: "",
      created_at: new Date().toISOString(),
      updated_at: new Date().toISOString(),
      tags: [],
    },
  };
  const { data } = await supabase
    .from("pipelines")
    .insert({
      project_id: params.projectId,
      name: params.name,
      description: params.description ?? null,
      version: "1.0.0",
      tags: [],
      dag_definition: dagDefinition,
    })
    .select(
      `id, name, description, project_id, version, tags, created_at, updated_at`,
    )
    .single();
  return data;
}
```

## API Layer

- Supabase client: `src/lib/supabase.ts` (uses Tauri HTTP plugin).
- Endpoints:
  - `POST /rest/v1/pipelines` (insert)
  - `GET /rest/v1/pipelines?project_id=eq.<uuid>` (list)

Example insert body:

```json
{
  "project_id": "<uuid>",
  "name": "my-pipeline",
  "description": "optional",
  "version": "1.0.0",
  "tags": [],
  "dag_definition": {
    "id": "<uuid>",
    "name": "my-pipeline",
    "project_id": "<uuid>",
    "nodes": [],
    "edges": [],
    "version": "1.0.0",
    "metadata": {
      "description": "optional",
      "created_by": "",
      "created_at": "2025-10-30T10:00:00Z",
      "updated_at": "2025-10-30T10:00:00Z",
      "tags": []
    }
  }
}
```

## Database & Realtime

- Table: `public.pipelines` (includes `dag_definition JSONB NOT NULL`).
- Realtime: Enabled on `public.pipelines`; the DAG Editor subscribes and reloads on INSERT/UPDATE/DELETE.
- RLS: permissive SELECT for anon/auth configured for Realtime in `enable_realtime_all_public_tables.sql` (tighten for production).

## DAG Editor Integration

- `content-dag-editor-pipelines.svelte` lists pipelines for the selected project via `listPipelinesByProject()` and updates on Realtime events.
- Future: load/update `dag_definition` when editing nodes/edges.
