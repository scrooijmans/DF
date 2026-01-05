# Svelte 5 Runes Best Practices: Dynamic Data Loading

This guide outlines the best practices for using Svelte 5 runes when working with dynamically loaded data from backend operations, based on real-world implementation experience.

## 🚨 Critical Anti-Patterns to Avoid

### ❌ DON'T: Mutate State Inside `$effect`

```typescript
// ❌ WRONG - Causes infinite loops
$effect(() => {
  if (backendData) {
    processedData = backendData.map((item) => transform(item)); // Mutating state!
  }
});
```

**Problem**: This creates an infinite loop because:

1. `$effect` runs when `backendData` changes
2. `processedData` is mutated inside the effect
3. This triggers the effect to run again
4. Loop continues indefinitely

### ❌ DON'T: Use `$effect` for Data Transformation

```typescript
// ❌ WRONG - Use $derived instead
$effect(() => {
  if (apiResponse) {
    gridData = apiResponse.items.map((item) => ({
      id: item.id,
      name: item.name,
      processed: true,
    }));
  }
});
```

### ❌ DON'T: Sync Props with Global State in `$effect`

```typescript
// ❌ WRONG - Causes UI freezing and infinite re-renders
$effect(() => {
  selectedFiles = globalState.selectedFiles; // Mutating props!
  onFilesChange?.(selectedFiles); // Side effect!
});
```

**Problem**: This pattern causes UI freezing because:

1. `$effect` runs when `globalState.selectedFiles` changes
2. `selectedFiles` prop is mutated, triggering parent re-render
3. Parent re-render causes child to re-render
4. Child re-render triggers `$effect` again
5. **UI freezes** due to infinite re-render cycle

**Symptoms**:
- UI becomes unresponsive after user interaction
- Console shows continuous logging
- Browser tab may become unresponsive
- Selection states don't update properly

## ✅ Correct Patterns

### 1. Use `$derived` for Data Transformation

```typescript
// ✅ CORRECT - Pure computation with $derived
const processedData = $derived(() => {
  if (!backendData) return [];

  return backendData.map((item) => ({
    id: item.id,
    name: item.name,
    processed: true,
  }));
});
```

**Benefits**:

- Pure computation (no side effects)
- Automatically reactive
- No infinite loops
- Better performance

### 2. Use `$derived` for Complex Data Structures

```typescript
// ✅ CORRECT - Complex DataFrame transformation
const gridData = $derived(() => {
  if (
    !lasProcessingResult?.curve_data ||
    lasProcessingResult.curve_data.length === 0
  )
    return [];

  // Get the maximum number of data points across all curves
  const maxDataPoints = Math.max(
    ...lasProcessingResult.curve_data.map((curve) => curve.values.length),
  );

  // Create rows where each row represents a depth/data point
  const rows = [];
  for (let i = 0; i < maxDataPoints; i++) {
    const row: any = { id: i };

    // Add each curve as a column
    lasProcessingResult.curve_data.forEach((curve) => {
      row[curve.mnemonic] =
        curve.values[i] !== undefined ? curve.values[i] : null;
    });

    rows.push(row);
  }

  return rows;
});
```

### 3. Use `$derived` for Column Definitions

```typescript
// ✅ CORRECT - Dynamic column generation
const columnDefs = $derived(() => {
  if (!lasProcessingResult?.curve_data) return [];

  return lasProcessingResult.curve_data.map((curve) => ({
    field: curve.mnemonic,
    headerName: curve.mnemonic,
    width: 120,
    sortable: true,
    filter: true,
    resizable: true,
    cellRenderer: (params: any) => {
      if (params.value === null || params.value === undefined) {
        return '<span style="color: #999;">-</span>';
      }
      return typeof params.value === "number"
        ? params.value.toFixed(3)
        : params.value;
    },
    tooltipField: curve.mnemonic,
    headerTooltip: `${curve.mnemonic} (${curve.unit}) - ${curve.description}`,
  }));
});
```

### 4. Use `$derived` for Global State Access

```typescript
// ✅ CORRECT - Access global state reactively without props
const globalState = getGlobalProjectObjects();

// Derive selection state from global state
const isSelected = $derived(globalState.selectedFiles.some(selectedFile => selectedFile.id === file.id));

// Get selected files from global state
let selectedDataFiles = $derived(globalState.selectedFiles);
```

**Benefits**:
- No props needed - direct global state access
- Automatic reactivity when global state changes
- No circular dependencies
- No UI freezing

### 5. Separate Components by Responsibility

```typescript
// ✅ CORRECT - Dedicated component for selected files display
// selected-files.svelte
<script lang="ts">
  import { getGlobalProjectObjects } from './project-objects.svelte.ts';
  
  const globalState = getGlobalProjectObjects();
  
  function handleFileDeselection(fileId: string) {
    globalState.toggleFileSelection(fileId);
  }
</script>

{#if globalState.selectedFiles.length > 0}
  <div class="selected-files">
    {#each globalState.selectedFiles as file (file.id)}
      <div class="file-item">
        <span>{file.name}</span>
        <button onclick={() => handleFileDeselection(file.id)}>×</button>
      </div>
    {/each}
  </div>
{/if}
```

### 6. Use `$effect` Only for Side Effects

```typescript
// ✅ CORRECT - Use $effect for updating external systems
$effect(() => {
  gridOptions.columnDefs = columnDefs();
});

// ✅ CORRECT - Use $effect for API calls
$effect(() => {
  if (selectedTable) {
    loadTableData(selectedTable);
  }
});
```

### 7. Use `$inspect` for Debugging

```typescript
// ✅ CORRECT - Debug reactive state changes
$inspect(processedData, "Processed data");
$inspect(columnDefs, "Column definitions");
$inspect(backendData, "Backend data");
$inspect(globalState.selectedFiles, "Selected files");
```

**Benefits**:

- Automatic logging when values change
- Deep reactivity tracking
- No performance impact in production
- Helps identify infinite loops
- Track UI freezing issues

## 🚫 Preventing UI Freezing

### The Problem

UI freezing occurs when components create circular dependencies through props and effects:

```typescript
// ❌ PROBLEMATIC PATTERN
// Parent component
let selectedFiles = $state([]);

// Child component
$effect(() => {
  selectedFiles = globalState.selectedFiles; // Mutates parent prop!
  onFilesChange?.(selectedFiles); // Triggers parent re-render!
});
```

### The Solution

**Eliminate props entirely** and use global state directly:

```typescript
// ✅ SOLUTION - No props, direct global state access
// Child component
const globalState = getGlobalProjectObjects();
const isSelected = $derived(globalState.selectedFiles.some(f => f.id === file.id));

// Parent component
const globalState = getGlobalProjectObjects();
const selectedDataFiles = $derived(globalState.selectedFiles);
```

### Component Architecture

```
❌ WRONG - Props-based (causes freezing)
Parent → Props → Child → $effect → Mutate Props → Parent Re-render → Loop

✅ CORRECT - Global state-based (no freezing)
Parent → Global State ← Child
       ↓
   $derived() ← $derived()
```

### Key Principles

1. **No props for reactive data** - Use global state instead
2. **No $effect for data sync** - Use $derived for reactive access
3. **Separate concerns** - Each component handles its own UI logic
4. **Direct global state access** - No intermediate prop passing

## 🏗️ Complete Component Pattern

Here's a complete example showing the proper pattern for dynamic data loading:

```typescript
<script lang="ts">
  import { onMount } from 'svelte';

  // Props from parent
  let { apiEndpoint, transformFunction }: {
    apiEndpoint: string;
    transformFunction: (data: any) => any;
  } = $props();

  // Loading states
  let isLoading = $state(false);
  let error = $state<string | null>(null);

  // Backend data (loaded from API)
  let backendData = $state<any[]>([]);

  // Derived data transformations
  const processedData = $derived(() => {
    if (!backendData.length) return [];
    return transformFunction(backendData);
  });

  const columnDefs = $derived(() => {
    if (!processedData().length) return [];
    return generateColumns(processedData()[0]);
  });

  // Grid options
  let gridOptions = $state({
    columnDefs: [],
    // ... other options
  });

  // Update grid when columns change
  $effect(() => {
    gridOptions.columnDefs = columnDefs();
  });

  // Debug reactive changes
  $inspect(backendData, 'Backend data');
  $inspect(processedData, 'Processed data');

  // Load data from backend
  async function loadData() {
    try {
      isLoading = true;
      error = null;

      const response = await fetch(apiEndpoint);
      const data = await response.json();

      backendData = data; // This triggers all derived computations

    } catch (err) {
      error = err instanceof Error ? err.message : 'Failed to load data';
    } finally {
      isLoading = false;
    }
  }

  // Load data on mount
  onMount(() => {
    loadData();
  });
</script>

{#if isLoading}
  <div>Loading...</div>
{:else if error}
  <div>Error: {error}</div>
{:else if processedData().length === 0}
  <div>No data available</div>
{:else}
  <div class="grid-container">
    <AgGrid {gridOptions} rowData={processedData()} />
  </div>
{/if}
```

## 🔄 Data Flow Pattern

```
Backend API → $state(backendData) → $derived(processedData) → UI
                    ↓
              $derived(columnDefs) → $effect(gridOptions) → AG-Grid
```

## 📋 Key Principles

### 1. **Separation of Concerns**

- `$state`: Store raw backend data
- `$derived`: Transform data for UI
- `$effect`: Handle side effects (API calls, external updates)

### 2. **Pure Functions**

- Keep `$derived` functions pure (no side effects)
- Use `$effect` only for side effects

### 3. **Reactive Chain**

- Backend data → Derived transformations → UI updates
- Each step is reactive and automatic

### 4. **Performance**

- `$derived` only recalculates when dependencies change
- No unnecessary re-renders
- Efficient memory usage

## 🐛 Common Debugging Techniques

### 1. Use `$inspect` to Track Changes

```typescript
$inspect(backendData, "Backend data");
$inspect(processedData, "Processed data");
```

### 2. Check for Infinite Loops

Look for console logs that repeat continuously - this indicates an infinite loop.

### 3. Verify Dependencies

Make sure `$derived` functions only depend on reactive state, not on external variables.

### 4. Test State Mutations

Never mutate state inside `$derived` or `$effect` unless it's a side effect in `$effect`.

## 🎯 Summary

- **Use `$derived`** for data transformation and computation
- **Use `$effect`** only for side effects (API calls, external updates)
- **Use `$inspect`** for debugging reactive state
- **Never mutate state** inside `$derived`
- **Keep functions pure** in `$derived`
- **Separate concerns** between data loading, transformation, and UI updates
- **Avoid props for reactive data** - Use global state with `$derived` instead
- **Prevent UI freezing** by eliminating circular dependencies between props and effects

## 🚨 UI Freezing Prevention Checklist

- ✅ No `$effect` that mutates props
- ✅ No props for reactive data that changes frequently
- ✅ Use global state with `$derived` for reactive access
- ✅ Separate components by responsibility
- ✅ Use `$inspect` to debug reactivity issues
- ✅ Test selection states and user interactions

This pattern ensures efficient, maintainable, and bug-free reactive components when working with dynamic backend data, while preventing the common UI freezing issues that occur with circular prop dependencies.
