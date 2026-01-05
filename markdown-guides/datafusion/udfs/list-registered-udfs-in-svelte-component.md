# End-to-End Flow: Displaying Registered UDFs in Svelte Components

This document outlines the complete flow, from UDF registration in the Rust backend to their dynamic display in the Svelte frontend's DAG editor for pipeline node configuration.

## 1. UDF Registration in Rust Backend (`udf-core` crate)

- **Location**: `crates/query-engine/udf-core/src/lib.rs` (and potentially other modules within `udf-core`).
- **Mechanism**: Petrophysical and other domain-specific UDFs are defined and registered within the `udf-core` crate. This crate maintains a central registry of all available UDFs, including their names, descriptions, input types, output types, categories, tags, and examples.
- **Data Structures**: The `udf-core` crate defines the canonical Rust structs for `UdfInfo` and `UdfCategoryInfo` which represent the metadata for each UDF and its categorization.

## 2. Exposing UDFs via Tauri Commands (`src-tauri/src/udf_commands.rs`)

- **Location**: `src-tauri/src/udf_commands.rs`.
- **Purpose**: This module acts as the bridge between the Rust backend's UDF registry and the Svelte frontend. It defines Tauri commands that can be invoked from the frontend.
- **Commands**:
  - `get_all_udfs()`: This command queries the `udf-core` registry and returns a structured list of all registered UDFs, grouped by their categories. The output is a `Vec<UdfCategoryInfo>`, where each `UdfCategoryInfo` contains a `Vec<UdfInfo>`.
  - Other commands like `get_udfs_by_category`, `get_udf_by_name`, and `search_udfs` provide more granular access to the UDF registry.
- **Serialization**: The Rust `UdfInfo` and `UdfCategoryInfo` structs are marked with `#[derive(Serialize, Deserialize)]` to allow them to be seamlessly converted to and from JSON when communicated between Rust and Svelte via Tauri's `invoke` mechanism.

## 3. Frontend UDF Service (`src/lib/services/udf-service.ts`)

- **Location**: `src/lib/services/udf-service.ts`.
- **Purpose**: This Svelte service is responsible for interacting with the Tauri backend to fetch UDF data and manage its state on the frontend.
- **Initialization**:
  - The `initialize()` method is called to fetch all UDFs from the backend.
  - It invokes the Tauri command `get_all_udfs` (e.g., `await invoke<UdfCategoryInfo[]>("get_all_udfs");`).
  - The fetched `UdfCategoryInfo` array is stored in `this.categories`.
  - A flattened list of all `UdfInfo` objects is extracted from `this.categories` and stored in `this.udfs`.
- **Data Transformation**:
  - The `getUdfsAsSqlFunctions()` method transforms the raw `UdfInfo` objects into a `SqlFunction` interface. This interface is defined within `udf-service.ts` and provides properties like `name`, `description`, `syntax`, `example`, `category`, and `returnType`, which are directly consumable by the UI components.
  - This transformation ensures that the UDFs conform to the expected format for display alongside other SQL operators.

## 4. SQL Operators Integration (`src/lib/services/sql-operators.ts`)

- **Location**: `src/lib/services/sql-operators.ts`.
- **Purpose**: This module centralizes the definition of all available SQL functions and operators, including both static (hardcoded) and dynamic (fetched UDFs) ones.
- **Dynamic UDF Loading**:
  - The `getDynamicUdfFunctions()` asynchronous function calls `udfService.getUdfsAsSqlFunctions()` to retrieve the dynamically registered UDFs in the `SqlFunction` format.
- **Consolidated Function List**:
  - The `getAllFunctionsWithUdfs()` asynchronous function combines the static `SQL_FUNCTIONS`, `PETROPHYSICAL_FUNCTIONS`, and the `dynamicUdfs` into a single comprehensive list of `SqlFunction` objects. This is the list that the DAG editor components will consume.

## 5. DAG Editor Components (`.svelte` files)

- **`src/lib/components/pages/home/content-main/content-editor/content-editor.svelte` (Main DAG Editor)**:
  - This is the top-level component for the pipeline DAG editor interface.
  - It calls `getAllFunctionsWithUdfs()` on `onMount` to populate its `allFunctions` state, which is then passed down to node configuration components.
- **`src/lib/components/pages/home/content-main/node-editor-sidebar/node-editor-sidebar.svelte` (Node Configuration Sidebar)**:
  - This component manages node configuration for operator nodes in the DAG.
  - On `onMount`, it calls `getAllFunctionsWithUdfs()` to load the complete list of functions into its `allFunctions` state.
  - It displays available operators/UDFs for selection when configuring operator nodes.
  - **Implementation**: Uses `$effect()` to create `filteredFunctions` from `allFunctions` based on search query.
  - **Scrollable Container**: Fixed height showing functions with Shadcn ScrollArea.
  - **Search Functionality**: Real-time filtering by name, description, syntax, and category.
  - **Interactive**: Each function can be selected to configure an operator node.
  - **Clean UI**: Proper spacing, hover effects, and responsive design.

## 6. UI Rendering and Interaction

- The DAG editor node configuration components successfully display all 46 registered UDFs (and other SQL functions) in a scrollable, searchable interface.
- **Function List Features**:
  - **Expandable/Collapsible**: Toggle button with proper arrow rotation (▶ when collapsed, ▼ when expanded)
  - **Searchable**: Real-time filtering as user types in the search input
  - **Scrollable**: Fixed height container showing ~5 functions at a time
  - **Interactive**: Click any function to insert its syntax into the SQL cell
- **Visual Design**: Clean interface using Shadcn Svelte components with proper spacing, hover effects, and responsive design.
- **User Experience**: Intuitive navigation through 46+ functions with instant search results and smooth scrolling.

## 7. Current Implementation Status

✅ **Completed**:

- UDF registration in Rust backend (`udf-core` crate)
- Tauri command exposure (`udf_commands.rs`)
- Frontend UDF service (`udf-service.ts`) with proper initialization
- SQL operators integration (`sql-operators.ts`) with dynamic UDF loading
- DAG editor components (`content-editor.svelte`)
- Node configuration sidebar (`node-editor-sidebar.svelte`)
- **Scrollable function list implementation** with fixed height (showing ~5 functions)
- **Search/filter functionality** for browsing through 46+ functions
- **Proper arrow rotation** for expand/collapse states
- **Clean UI** with removed search icons and improved styling
- **Complete UDF data flow** from backend to frontend display

✅ **Fully Functional**:

- All 46 registered UDFs are properly displayed in the DAG editor for node configuration
- Functions are searchable and filterable in real-time
- Scrollable container shows approximately 5 functions at a time
- Arrow properly rotates when expanding/collapsing the function list
- Clean search input without interfering icons
- Functions can be clicked to insert into SQL cells

## 8. Implementation Details

### Function Loading Process:

1. **Backend Registration**: UDFs are registered in `udf-core` crate with metadata
2. **Tauri Bridge**: `udf_commands.rs` exposes `get_all_udfs()` command
3. **Frontend Service**: `udf-service.ts` calls Tauri command and transforms data
4. **SQL Integration**: `sql-operators.ts` combines static and dynamic functions
5. **Component Rendering**: DAG editor node configuration displays functions in scrollable list

### UI Features:

- **Fixed Height**: Function list container has `h-40` (160px) height
- **Scrollable**: Uses Shadcn Svelte `ScrollArea` component
- **Searchable**: Real-time filtering based on name, description, syntax, category
- **Expandable**: Toggle button with proper arrow rotation (▶/▼)
- **Interactive**: Click functions to insert into SQL cell textarea

## 9. Technical Details

### Data Flow Summary:

```
Rust Backend (udf-core)
    ↓ (UDF registration with metadata)
Tauri Commands (udf_commands.rs)
    ↓ (get_all_udfs() command)
Frontend UDF Service (udf-service.ts)
    ↓ (initialize() → getUdfsAsSqlFunctions())
SQL Operators (sql-operators.ts)
    ↓ (getAllFunctionsWithUdfs())
DAG Editor (content-editor.svelte)
    ↓ (loadUdfFunctions() → allFunctions)
Node Editor Sidebar (node-editor-sidebar.svelte)
    ↓ (allFunctions prop)
Node Configuration Panel
    ↓ (filteredFunctions with $effect)
Scrollable Function List (Shadcn ScrollArea)

## Using the same discovery for DAG Operator Palette

You can reuse this UDF discovery pipeline to populate the DAG operator library:

- Add a frontend adapter that maps `UdfInfo` (or `SqlFunction`) to `OperatorListItem` entries for the DAG editor sidebar.
- Call Tauri `get_available_operators` to also include non-UDF built-ins. Merge both lists in the UI.
- When a user drops an operator node, use the operator’s JSON Schema (for UDFs or native operators) to render the parameter form.

See `markdown-guides/to-do/dag-execution.md` for operator registry and executor details.
```

### Key Interfaces:

- `UdfInfo`: Raw UDF metadata from backend
- `SqlFunction`: Transformed UDF data for UI consumption
- `UdfCategoryInfo`: Categorized UDF groupings

### Performance Considerations:

- **Single Load**: UDFs are loaded once on component mount via `udf-service.initialize()`
- **Caching**: Functions are cached in the UDF service singleton instance
- **No Repeated Calls**: No repeated backend calls for function data after initial load
- **Efficient Filtering**: Real-time search uses `$effect()` for reactive filtering
- **Scrollable Rendering**: Fixed height container with Shadcn ScrollArea for smooth scrolling
- **Memory Efficient**: Only visible functions are rendered in the DOM
- **Responsive**: Search and filtering happen instantly without API calls
