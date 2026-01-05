## DAG Editor: Listing Operators (End-to-End Flow)

This guide documents how the frontend and backend work together to display available operators in the DAG Editor's Operator mode.

### Overview

- Frontend (Svelte): `content-dag-operator-node-editor.svelte` invokes a Tauri command to fetch operators, then renders them via `content-dag-operator-node-editor-items.svelte` and `content-dag-operator-node-editor-item.svelte`.
- Backend (Rust): Tauri command `get_available_operators` calls the `OperatorRegistry` (in `crates/dags/operators`) to return a list of operator definitions.

### Backend

1. Operator registry defines built-in operators and their metadata (id, name, category, version, parameters schema, executor type).

```rust
// crates/dags/operators/src/registry.rs (abbreviated)
pub struct OperatorDefinition {
    pub operator_id: String,
    pub operator_name: String,
    pub category: String,
    pub description: String,
    pub version: String,
    pub input_schema: Option<SchemaDefinition>,
    pub output_schema: Option<SchemaDefinition>,
    pub parameters_schema: serde_json::Value,
    pub implementation_type: String,
}

pub struct OperatorRegistry { /* ... */ }

impl OperatorRegistry {
    pub fn new() -> Self { /* registers built-ins */ }
    pub fn list_operators(&self, category: Option<&str>) -> Vec<OperatorDefinition> { /* ... */ }
}
```

2. Tauri command exposes the registry to the UI and supports an optional category filter.

```rust
// src-tauri/src/tauri_commands/pipeline_commands.rs (abbreviated)
#[tauri::command]
pub async fn get_available_operators(category: Option<String>) -> Result<Vec<OperatorDefinitionDto>, String> {
    let registry = OperatorRegistry::new();
    let ops = registry
        .list_operators(category.as_deref())
        .into_iter()
        .map(OperatorDefinitionDto::from)
        .collect();
    Ok(ops)
}
```

3. Command is registered in the app's invoke handler.

```rust
// src-tauri/src/main.rs (abbreviated)
.invoke_handler(tauri::generate_handler![
    // ... other commands ...
    save_pipeline,
    load_pipeline,
    execute_pipeline,
    list_pipelines,
    get_available_operators,
])
```

### Frontend

1. Fetch operators and display loading/error states.

```svelte
<!-- src/lib/components/.../content-dag-operator-node-editor.svelte -->
<script lang="ts">
  import ContentDagOperatorNodeEditorItems from "./content-dag-operator-node-editor-items/content-dag-operator-node-editor-items.svelte";
  import { invoke } from "@tauri-apps/api/core";

  type OperatorInfo = {
    operator_id: string;
    operator_name: string;
    category: string;
    version: string;
    implementation_type: string;
    parameters_schema: any;
  };

  let isLoading = $state(true);
  let error: string | null = $state(null);
  let operators: OperatorInfo[] | null = $state(null);
  let selectedCategory: string | null = $state(null);

  async function loadOperators() {
    try {
      isLoading = true;
      error = null;
      operators = await invoke<OperatorInfo[]>("get_available_operators", { category: selectedCategory });
    } catch (e: any) {
      error = e?.toString?.() ?? "Failed to load operators";
    } finally {
      isLoading = false;
    }
  }

  $effect(() => {
    loadOperators();
  });
</script>

{#if error}
  <div class="text-sm text-red-600">{error}</div>
{:else if isLoading}
  <div class="text-sm text-gray-600">Loading operators...</div>
{:else if operators && operators.length}
  <ContentDagOperatorNodeEditorItems {operators} />
{:else}
  <div class="text-sm text-gray-600">No operators available.</div>
{/if}
```

2. Group by category and render items.

```svelte
<!-- content-dag-operator-node-editor-items.svelte (abbreviated) -->
<script lang="ts">
  import Item from "../content-dag-operator-node-editor-item/content-dag-operator-node-editor-item.svelte";
  let { operators } = $props<{ operators: Array<{ operator_id: string; operator_name: string; category: string; version: string; implementation_type: string; parameters_schema: any; }> }>();

  let grouped = $state(new Map<string, typeof operators>());
  $effect(() => {
    const map = new Map<string, typeof operators>();
    if (operators) for (const op of operators) (map.get(op.category) ?? map.set(op.category, []).get(op.category)!).push(op);
    grouped = map;
  });
</script>

{#each Array.from(grouped.entries()) as [category, items]}
  <h3 class="text-lg font-semibold">{category}</h3>
  <div class="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
    {#each items as op}
      <Item {op} />
    {/each}
  </div>
{/each}
```

3. Operator item card (concise metadata).

```svelte
<!-- content-dag-operator-node-editor-item.svelte (abbreviated) -->
<script lang="ts">
  let { op } = $props<{ op: { operator_id: string; operator_name: string; category: string; version: string; implementation_type: string; parameters_schema: any; } }>();
</script>

<div class="border rounded-lg p-4 bg-white">
  <div class="flex items-center justify-between mb-1">
    <h4 class="font-semibold">{op.operator_name}</h4>
    <span class="text-xs text-gray-500">v{op.version}</span>
  </div>
  <div class="text-xs text-gray-600 mb-2">{op.operator_id} • {op.implementation_type}</div>
  <div class="text-xs text-gray-500">Params: {Object.keys(op.parameters_schema?.properties ?? {}).length}</div>
</div>
```

### Communication Contract

- Command name: `get_available_operators`
- Request: `{ category?: string | null }`
- Response: `OperatorInfo[]`
  - `operator_id: string`
  - `operator_name: string`
  - `category: string`
  - `version: string`
  - `implementation_type: string` (e.g. "udf", "native_rust", "wasm")
  - `parameters_schema: JSON` (JSON Schema for operator parameters)

### Related

- Creating pipelines: see New Pipeline UI wiring and storage commands in the DAG editor section.
- Listing pipelines: use `list_pipelines(project_id)` and reactively update on project changes; realtime updates can mirror the wells pattern.
