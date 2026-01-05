# Multi-Select Component Issues and Solutions

## Problem Summary

The `DataMultiSelectFilter` component was not responding to user clicks in the DAG Editor (`content-dag-ingestion-node-editor-items.svelte`), particularly for surfaces, markers, and other data types. However, it worked correctly in `content-data.svelte`.

## Root Causes

### Issue 1: New Array References on Every Render

**Problem**: In components like `content-dag-ingestion-node-editor-items.svelte`, the `selectedValues` prop was computed inline:

```svelte
selectedValues={markersState.selectedMarkers.map(
  (m) => `${m.well_name}_${m.formation_name}_${m.depth}`,
)}
```

This creates a **new array reference on every render**, even when the underlying data hasn't changed. The `DataMultiSelectFilter` component's effect tracking system couldn't distinguish between actual changes and render-induced reference changes, causing it to reset selections.

**Solution**: Use `$derived` to memoize computed values:

```svelte
// Derived selected values for markers (to prevent new array on every render)
let selectedMarkerValues = $derived(
  markersState.selectedMarkers.map(
    (m) => `${m.well_name}_${m.formation_name}_${m.depth}`,
  ),
);

// Then use the derived value:
selectedValues={selectedMarkerValues}
```

**Why it works**: `$derived` only recalculates when its dependencies actually change, preserving array reference equality when the data is unchanged.

### Issue 2: Reactive Loop and Prop Synchronization

**Problem**: The component had difficulty distinguishing between:

1. **User-initiated changes**: When a user clicks an option in the MultiSelect dropdown
2. **External prop updates**: When parent components update `selectedValues` from state changes

The original implementation tried to sync props in both directions, causing:

- Infinite loops (props updating → effect fires → props update → effect fires again)
- User selections being reset by prop sync effects
- Callback not firing or firing multiple times

**Solution**: Implement a two-effect system with previous value tracking:

```svelte
// Track the previous prop value to detect external changes
let previousPropValue = $state<string[]>([...selectedValues]);

// Helper to normalize arrays for comparison
const normalize = (arr: string[]) => [...arr].sort().join('|');

// Effect 1: Sync local state with prop changes (external updates only)
$effect(() => {
  const propNormalized = normalize(selectedValues);
  const prevPropNormalized = normalize(previousPropValue);
  const localNormalized = normalize(localSelectedValues);

  // If prop changed externally, sync it to local state
  if (propNormalized !== prevPropNormalized) {
    localSelectedValues = selectedValues;
    previousPropValue = [...selectedValues];
  }
});

// Effect 2: Watch for user-initiated changes
$effect(() => {
  const localNormalized = normalize(localSelectedValues);
  const propNormalized = normalize(selectedValues);
  const prevPropNormalized = normalize(previousPropValue);

  // If local changed but prop hasn't, it's a user-initiated change
  if (localNormalized !== propNormalized && propNormalized === prevPropNormalized) {
    // Update bindable prop
    selectedValues = localSelectedValues;

    // Update previous prop value to prevent re-triggering
    previousPropValue = [...localSelectedValues];

    // Call callback for user-initiated changes
    if (onSelectionChange) {
      onSelectionChange(localSelectedValues);
    }
  }
});
```

**Why it works**:

- By tracking `previousPropValue`, we can detect when props change externally vs. when local state changes due to user interaction
- Normalizing arrays (sorting and joining) ensures we compare content, not references
- The conditions ensure effects only fire for their specific scenarios:
  - Effect 1: Only when prop changes externally
  - Effect 2: Only when local changes without prop change (user-initiated)

## Implementation Details

### Component Structure

`DataMultiSelectFilter` is a standalone component with a dual-list interface (available options on left, selected options on right) and provides:

1. **Bidirectional binding**: `selectedValues` is `$bindable`, allowing parent components to control selection
2. **Callback notification**: `onSelectionChange` fires only for user-initiated changes
3. **State synchronization**: Automatically syncs when parent updates `selectedValues`
4. **Dual filtering**: Separate filter inputs for available and selected options
5. **Expanded by default**: Shows both lists side-by-side with scrollable content

### Normalization Function

The `normalize` function is critical for reliable comparisons:

```typescript
const normalize = (arr: string[]) => [...arr].sort().join("|");
```

Benefits:

- Handles different array ordering (selections might be in different order but same items)
- Works with array references (compares content, not identity)
- Fast string comparison after normalization

## Best Practices

### For Parent Components

1. **Use `$derived` for computed selected values**:

   ```svelte
   let selectedMarkerValues = $derived(
     markersState.selectedMarkers.map(m => computeValue(m))
   );
   ```

2. **Pass direct state references when possible**:

   ```svelte
   selectedValues={wellsState.selectedWells}  // ✅ Direct reference
   ```

3. **Avoid inline computations**:
   ```svelte
   selectedValues={state.items.map(x => x.value)}  // ❌ Creates new array each render
   ```

### For the Component Itself

1. **Always normalize arrays before comparison** to handle reference equality issues
2. **Track previous values** to distinguish change sources
3. **Use separate effects** for different update scenarios (user vs. external)
4. **Update tracking variables atomically** to prevent race conditions

## Testing Checklist

When working with `DataMultiSelectFilter`, verify:

- [ ] Clicking options updates selection immediately
- [ ] Selection persists after re-renders
- [ ] Parent updates to `selectedValues` sync correctly
- [ ] Callback fires only once per user action
- [ ] No infinite loops in console
- [ ] Works with all data types (wells, markers, trajectories, surfaces, tables)

## Related Files

- `src/lib/components/pages/home/content-main/content-data/data-multi-select-filter/data-multi-select-filter.svelte` - Main component with dual-list interface
- `src/lib/components/pages/home/content-main/content-dag-editor/content-dag-ingestion-node-editor/content-dag-ingestion-node-editor-items/content-dag-ingestion-node-editor-items.svelte` - Example usage
