# Chart Interactions Sidebar

## Overview

The chart interactions sidebar is a vertical toolbar located on the left side of the chart editor that provides interactive tools for manipulating charts. It allows users to enable/disable SciChart modifiers (zoom, pan, data selection) by clicking icons, with only one modifier active at a time for mutually exclusive interactions.

## Architecture

### Layout Structure

```
chart-editor.svelte
├── Main Content Area (flex-1)
│   ├── Left Sidebar (48px, flex-shrink-0) - Interactions
│   │   └── chart-editor-interactions-sidebar.svelte
│   │       └── [Chart-type-specific interactions component]
│   ├── Center - Chart Visualization (flex-1)
│   └── (No right sidebar)
```

### Component Structure

**Main Router Component**: `chart-editor-interactions-sidebar.svelte`

- Dynamically renders chart-type-specific interaction components based on `selectedChart.chart_type`
- Passes `chart` prop to chart-type-specific components

**Chart-Type-Specific Components**:

- `chart-editor-interactions-sidebar-xy.svelte` - For `xy`, `line`, `scatter`, `geothermal_gradient` charts
- `chart-editor-interactions-sidebar-well-correlation.svelte` - For `well_correlation` charts
- `chart-editor-interactions-sidebar-map.svelte` - For `map` charts

## Modifier Registry System

### Centralized Configuration

All chart modifiers are configured in `chart-modifier-registry.ts`, which defines:

- **Available Modifiers**: Which modifiers are available for each chart type
- **Modifier Configuration**: Icon, description, default enabled state
- **Mutual Exclusivity**: Which modifiers should be disabled when another is enabled

**Example Configuration**:

```typescript
{
  chartType: "xy",
  availableModifiers: [
    {
      key: "pan",
      name: "Pan",
      icon: "Move",
      description: "Pan the chart by dragging",
      enabledByDefault: false,
      mutuallyExclusiveWith: ["dataSelection"],
    },
    {
      key: "dataSelection",
      name: "Data Selection",
      icon: "MousePointer2",
      description: "Select data points on the chart",
      enabledByDefault: false,
      mutuallyExclusiveWith: ["pan"],
    },
  ],
}
```

### Base Class for Chart States

All 2D chart states extend `Chart2DBase`, which provides:

- **Modifier Management**: Methods to enable/disable modifiers
- **Modifier Storage**: Map of modifier instances for toggling
- **Registry Integration**: Access to modifier registry

**Key Methods**:

```typescript
class Chart2DBase {
  getModifier(key: string): any;
  enableModifier(key: string): void;
  getEnabledModifierKey(): string | null;
}
```

## Available Modifiers

### Pan Modifier (`ZoomPanModifier`)

- **Icon**: `Move` (from `@lucide/svelte`)
- **Description**: Pan the chart by dragging
- **Default State**: Disabled
- **Mutually Exclusive With**: Data Selection

**Usage**: Click the Pan icon to enable panning. Click again to disable.

### Data Selection Modifier (`DataPointSelectionModifier`)

- **Icon**: `MousePointer2` (from `@lucide/svelte`)
- **Description**: Select data points on the chart
- **Default State**: Disabled
- **Mutually Exclusive With**: Pan

**Usage**: Click the Data Selection icon to enable point selection. Selected points are highlighted with white fill/stroke via `DataPointSelectionPaletteProvider`.

**Visual Feedback**: When enabled, clicking data points will highlight them. The palette provider changes the point marker fill/stroke to white when selected.

### Zoom Modifiers (Always Enabled)

These modifiers are always enabled and don't appear in the interactions sidebar:

- **Rubber Band Zoom** (`RubberBandXyZoomModifier`) - Right-click drag to zoom
- **Mouse Wheel Zoom** (`MouseWheelZoomModifier`) - Scroll to zoom
- **Zoom Extents** (`ZoomExtentsModifier`) - Double-click to fit all data

## Interaction Flow

### Enabling a Modifier

1. User clicks an icon in the interactions sidebar
2. Component calls `chartState.enableModifier(modifierKey)`
3. Base class disables all other modifiers
4. Base class enables the requested modifier
5. Icon updates to show active state (variant changes from 'ghost' to 'default')

### Disabling a Modifier

1. User clicks the same icon again (currently active)
2. Component disables all modifiers
3. Icon updates to show inactive state

### Mutual Exclusivity

When a modifier is enabled, all modifiers listed in `mutuallyExclusiveWith` are automatically disabled:

```typescript
// When pan is enabled, dataSelection is disabled
if (modifierConfig.key === "pan") {
  // Disable dataSelection
  const dataSelectionModifier = this.modifiers.get("dataSelection");
  if (dataSelectionModifier) {
    dataSelectionModifier.isEnabled = false;
  }
}
```

## Implementation Details

### Chart State Integration

Chart states initialize modifiers using the registry:

```typescript
// In initializeChart()
const availableModifiers = chartModifierRegistry.getAvailableModifiers(
  chart.chart_type,
);

for (const modifierConfig of availableModifiers) {
  let modifier: any = null;

  switch (modifierConfig.key) {
    case "pan":
      modifier = new ZoomPanModifier({ enableZoom: true });
      modifier.isEnabled = modifierConfig.enabledByDefault;
      break;
    case "dataSelection":
      modifier = new DataPointSelectionModifier();
      modifier.isEnabled = modifierConfig.enabledByDefault;
      break;
    // ... other modifiers
  }

  if (modifier) {
    this.sciChartSurface.chartModifiers.add(modifier);
    this.modifiers.set(modifierConfig.key, modifier);
  }
}
```

### Sidebar Component

The sidebar component uses the registry to determine which icons to show:

```svelte
<script lang="ts">
  const toggleableModifiers = $derived(
    chartModifierRegistry.getToggleableModifiers(chart.chart_type),
  );

  const enabledModifierKey = $derived(
    chartState?.getEnabledModifierKey() || null,
  );

  function handleModifierClick(modifierKey: ModifierKey) {
    if (enabledModifierKey === modifierKey) {
      // Disable all modifiers
      for (const mod of toggleableModifiers) {
        const modifier = chartState.getModifier(mod.key);
        if (modifier) modifier.isEnabled = false;
      }
    } else {
      // Enable the clicked modifier
      chartState.enableModifier(modifierKey);
    }
  }
</script>

{#each toggleableModifiers as modifier}
  {@const isActive = enabledModifierKey === modifier.key}
  <button
    class={buttonVariants({ variant: isActive ? 'default' : 'ghost' })}
    onclick={() => handleModifierClick(modifier.key)}
  >
    <IconComponent class="size-4" />
  </button>
{/each}
```

## Data Point Selection

### Setup

When the data selection modifier is available, series are created with `DataPointSelectionPaletteProvider`:

```typescript
const lineSeries = new FastLineRenderableSeries(this.wasmContext, {
  dataSeries: dataSeries,
  stroke: series.stroke || themeColors.lineSeriesColor,
  strokeThickness: series.strokeThickness || 2,
});

// Add palette provider for visual feedback
const dataSelectionModifier = this.modifiers.get("dataSelection");
if (dataSelectionModifier) {
  lineSeries.paletteProvider = new DataPointSelectionPaletteProvider({
    fill: "white",
    stroke: "white",
  });
}
```

### Selection Behavior

- **Click**: Select a single data point (replaces previous selection)
- **Ctrl+Click**: Add point to selection (multi-select)
- **Shift+Click**: Invert selection (deselect if selected, select if not)
- **Drag Rectangle**: Select all points within rectangle

### Visual Feedback

Selected points are highlighted with white fill and stroke, making them clearly visible against the chart background.

## Type Safety

The modifier registry uses TypeScript types to ensure:

- **Valid Modifier Keys**: Only registered modifier keys can be used
- **Chart Type Safety**: Modifiers are only available for appropriate chart types
- **Icon Name Validation**: Icon names must match Lucide icon exports

## Benefits

1. **Centralized Configuration**: All modifier settings in one place
2. **Type Safety**: Compile-time validation of modifier configurations
3. **Consistent UI**: Same interaction pattern across all chart types
4. **Easy Extension**: Add new modifiers by updating the registry
5. **Mutual Exclusivity**: Prevents conflicting interactions
6. **Visual Feedback**: Clear indication of active modifier

## Future Enhancements

1. **Keyboard Shortcuts**: Add keyboard shortcuts for modifier toggling
2. **Modifier Presets**: Save and restore modifier configurations
3. **Custom Modifiers**: Allow users to create custom modifier combinations
4. **Modifier Tooltips**: Enhanced tooltips with usage instructions
5. **Selection Callbacks**: Handle data point selection events (e.g., show details, export)

