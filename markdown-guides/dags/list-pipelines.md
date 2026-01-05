# List & Select Pipelines - End-to-End Flow (Sidebar in DAG Editor)

## Overview

This describes how pipelines are listed and selected in the DAG Editor. The sidebar renders pipelines for the currently selected project (from `PostgresProjectsState`), supports realtime updates, and writes the selection to a global `ContentDagPipelineState`.

## Flow

```
Project selection (PostgresProjectsState)
    ↓
content-dag-editor-pipelines.svelte (loads via Supabase service)
    ↓ (Realtime refresh on INSERT/UPDATE/DELETE)
content-dag-editor-pipeline-item.svelte (click → global state)
    ↓
ContentDagPipelineState.selectedPipelineId (context)
    ↓
content-dag-pipeline.svelte (renders selection)
```

## Components & State

### 1) Global State: `ContentDagPipelineState`

File: `content-dag-editor-pipeline.svelte.ts`

- Holds `selectedPipelineId: string | null`.
- `setSelectedPipelineId(id)` updates selection and logs for debug.
- Accessors: `setContentDagPipelineState()` and `getContentDagPipelineState()`.
- Initialized in `src/routes/home/+layout.svelte` via `setContentDagPipelineState()`.

### 2) List & Realtime: `content-dag-editor-pipelines.svelte`

- Loads pipelines for `PostgresProjectsState.currentSelectedProjectId` using `listPipelinesByProject(projectId)` from `pipeline-service.ts`.
- Subscribes to Supabase Realtime on `public.pipelines` and re-loads when any change occurs.
- Renders items with `<PipelineItem pipeline={p} />`.

Key behaviors:

- Scoped to the selected project (auto-select project happens in `PostgresProjectsState`).
- Graceful empty and error states.
- Realtime: `.channel('pipelines-changes').on('postgres_changes', ...)` → `loadPipelines()`.

### 3) Selectable Item: `content-dag-editor-pipeline-item.svelte`

- Receives `pipeline: PipelineRow` prop.
- On click, calls `pipelineState.setSelectedPipelineId(pipeline.id)`.
- Highlights the selected item by comparing `pipelineState.selectedPipelineId`.

### 4) Selected Pipeline Display: `content-dag-pipeline.svelte`

- Reads the global state (`getContentDagPipelineState()`), shows the `selectedPipelineId`.
- Intended as the header for the right-hand editor workspace.

### 5) DAG Editor Layout: `content-dag-editor.svelte`

- Two-column layout:
  - Left: Sidebar (`ContentDagEditorPipelines`) → ~25% width
  - Right: Workspace (~75%) → shows `ContentDagPipeline`, mode selector, and node editors

## Service Layer

File: `src/lib/services/pipeline-service.ts`

- `listPipelinesByProject(projectId)` → Supabase select scoped by `project_id`.
- Reused by the list component.

## Realtime & Security

- Realtime enabled for `public.pipelines` via `enable_realtime_all_public_tables.sql`.
- Policies: permissive SELECT for anon/auth to allow Realtime broadcast (tighten for production).

## UX Notes

- Selection persists in memory via context; can be extended to persist in local storage if needed.
- Next step: Add an “Open” action to load the full `dag_definition` of the selected pipeline into the editor state.
