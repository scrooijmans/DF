# Chart Modifier Activation/Deactivation: End-to-End Process

## Overview

This document describes the complete end-to-end process of how chart modifiers (interaction tools) are activated and deactivated in the MudRock charting system. The system uses a centralized modifier registry, reactive state management, and a visual interactions sidebar to provide users with intuitive control over chart interactions.

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│              Chart Modifier Registry                         │
│  chart-modifier-registry.ts                                  │
│  - Defines available modifiers per chart type                │
│  - Specifies mutual exclusivity relationships                │
│  - Maps modifiers to icons and descriptions                  │
└────────────────────┬────────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────────┐
│              Chart State Classes                             │
│  Chart2DBase (base class)                                    │
│  - Manages modifier instances                                │
│  - Provides enableModifier() / getModifier() methods        │
│  - Handles mutual exclusivity logic                          │
└────────────────────┬────────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────────┐
│              Interactions Sidebar Components                 │
│  chart-editor-interactions-sidebar-*.svelte                  │
│  - Displays modifier icons                                   │
│  - Handles user clicks                                       │
│  - Shows active/inactive state                               │
└─────────────────────────────────────────────────────────────┘
```

## Complete Flow

### Step 1: Modifier Registry Configuration

**Location**: `src/lib/modifiers/chart-modifier-registry.ts`

**Purpose**: Centralized configuration defining which modifiers are available for each chart type and their relationships.

**Key Data Structures**:

```typescript
export interface ModifierConfig {
  key: ModifierKey;
  name: string;
  icon: string; // Lucide icon name
  description: string;
  enabledByDefault: boolean;
  mutuallyExclusiveWith?: ModifierKey[]; // Modifiers disabled when this is enabled
  isIndependent?: boolean; // Can be enabled alongside other modifiers
}

export type ModifierKey =
  | "pan"
  | "dataSelection"
  | "crosshair"
  | "rubberBandZoom"
  | "mouseWheelZoom"
  | "zoomExtents";
```

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
      mutuallyExclusiveWith: ["dataSelection"], // Disables dataSelection when enabled
      isIndependent: false, // Part of mutually exclusive group
    },
    {
      key: "crosshair",
      name: "Crosshair",
      icon: "Crosshair",
      description: "Show crosshair cursor with tooltips",
      enabledByDefault: false,
      isIndependent: true, // Can be enabled alongside any other modifier
    },
  ],
}
```

**What Happens**:

1. Registry defines all available modifiers for each chart type
2. Specifies which modifiers are mutually exclusive (e.g., pan vs dataSelection)
3. Marks independent modifiers (e.g., crosshair) that can coexist with others
4. Maps each modifier to a Lucide icon for display in the sidebar

---

### Step 2: Chart Initialization - Modifier Creation

**Location**: `src/lib/state/postgres/chart-states/xy-chart-state.svelte.ts` (and other chart state classes)

**Code Flow**:

```typescript
async initializeChart(container: HTMLDivElement, chart: Chart): Promise<void> {
  // ... create SciChart surface, axes, series ...

  // Get available modifiers from registry
  const availableModifiers = chartModifierRegistry.getAvailableModifiers(
    chart.chart_type,
  );

  // Create and add each modifier
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
        // Subscribe to selection events
        if (modifier.selectionChanged) {
          modifier.selectionChanged.subscribe((args: any) => {
            console.log("Selected data points:", args.selectedDataPoints);
          });
        }
        break;

      case "crosshair":
        modifier = new CursorModifier({
          crosshairStroke: "#FF6600",
          crosshairStrokeThickness: 1,
          showXLine: true,
          showYLine: true,
          showTooltip: true,
        });
        modifier.isEnabled = modifierConfig.enabledByDefault;
        break;
    }

    if (modifier) {
      // Add modifier to SciChart surface
      this.sciChartSurface.chartModifiers.add(modifier);
      // Store in map for later toggling
      this.modifiers.set(modifierConfig.key, modifier);
    }
  }
}
```

**What Happens**:

1. Chart state queries registry for available modifiers
2. Creates SciChart modifier instances (ZoomPanModifier, DataPointSelectionModifier, CursorModifier, etc.)
3. Sets initial enabled state from registry configuration
4. Adds modifiers to SciChart surface's `chartModifiers` collection
5. Stores modifier instances in a `Map<string, any>` for later access

**Key Point**: All modifiers are created during initialization, but most are disabled by default. Only modifiers with `enabledByDefault: true` are active initially.

---

### Step 3: User Clicks Modifier Icon in Sidebar

**Location**: `src/lib/components/pages/home/charts/chart-editor/chart-editor-interactions-sidebar/chart-editor-interactions-sidebar-xy.svelte`

**Code Flow**:

```svelte
<script lang="ts">
  const chartsState = getPostgresChartsState();
  const xyPlotState = $derived(chartsState.xyPlotState);
  const chartState = $derived(
    xyPlotState && currentChartId === chart.id ? xyPlotState : null,
  );

  // Get modifiers from registry
  const mutuallyExclusiveModifiers = $derived(
    chartModifierRegistry.getMutuallyExclusiveModifiers(chart.chart_type),
  );
  const independentModifiers = $derived(
    chartModifierRegistry.getIndependentModifiers(chart.chart_type),
  );

  // Track enabled state for visual feedback
  const enabledMutuallyExclusiveKey = $derived(
    chartState?.getEnabledModifierKey() || null,
  );
  const enabledIndependentKeys = $derived(
    chartState?.getEnabledIndependentModifierKeys() || [],
  );

  // Handle mutually exclusive modifier click
  function handleMutuallyExclusiveClick(modifierKey: ModifierKey) {
    if (!chartState) return;

    // If clicking the same modifier, disable it
    if (enabledMutuallyExclusiveKey === modifierKey) {
      chartState.disableModifier(modifierKey);
    } else {
      // Enable clicked modifier (automatically disables others)
      chartState.enableModifier(modifierKey);
    }
  }

  // Handle independent modifier click (toggle)
  function handleIndependentClick(modifierKey: ModifierKey) {
    if (!chartState) return;

    const isEnabled = enabledIndependentKeys.includes(modifierKey);
    if (isEnabled) {
      chartState.disableModifier(modifierKey);
    } else {
      chartState.enableIndependentModifier(modifierKey);
    }
  }
</script>

<!-- Mutually Exclusive Modifiers Section -->
<div class="flex flex-col gap-1">
  {#each mutuallyExclusiveModifiers as modifier}
    {@const isActive = enabledMutuallyExclusiveKey === modifier.key}
    <button
      class={buttonVariants({ variant: isActive ? 'default' : 'ghost' })}
      onclick={() => handleMutuallyExclusiveClick(modifier.key)}
    >
      <IconComponent class="size-4" />
    </button>
  {/each}
</div>

<!-- Independent Modifiers Section -->
<div class="flex flex-col gap-1 border-t pt-2">
  {#each independentModifiers as modifier}
    {@const isActive = enabledIndependentKeys.includes(modifier.key)}
    <button
      class={buttonVariants({ variant: isActive ? 'default' : 'ghost' })}
      onclick={() => handleIndependentClick(modifier.key)}
    >
      <IconComponent class="size-4" />
    </button>
  {/each}
</div>
```

**What Happens**:

1. Sidebar component gets current chart state from global state
2. Queries registry for mutually exclusive and independent modifiers
3. Displays two separate sections of icons
4. User clicks an icon
5. Handler function determines if modifier is mutually exclusive or independent
6. Calls appropriate method on chart state to enable/disable modifier

---

### Step 4: Chart State Processes Modifier Toggle

**Location**: `src/lib/state/postgres/chart-states/chart-2D-base.svelte.ts`

**Code Flow**:

```typescript
/**
 * Enable a mutually exclusive modifier (disables others in the group)
 */
enableModifier(key: string): void {
  if (!this.modifierRegistry) return;

  const modifierConfig = this.modifierRegistry.getModifierConfig(
    this.chart?.chart_type || "",
    key,
  );

  if (!modifierConfig) return;

  // If this modifier has mutually exclusive relationships, disable those first
  if (modifierConfig.mutuallyExclusiveWith) {
    for (const exclusiveKey of modifierConfig.mutuallyExclusiveWith) {
      const exclusiveModifier = this.modifiers.get(exclusiveKey);
      if (exclusiveModifier && typeof exclusiveModifier.isEnabled !== "undefined") {
        exclusiveModifier.isEnabled = false;
      }
    }
  }

  // Also disable all other mutually exclusive modifiers
  const allMutuallyExclusive = this.modifierRegistry.getMutuallyExclusiveModifiers(
    this.chart?.chart_type || "",
  );
  for (const modConfig of allMutuallyExclusive) {
    if (modConfig.key !== key) {
      const mod = this.modifiers.get(modConfig.key);
      if (mod && typeof mod.isEnabled !== "undefined") {
        mod.isEnabled = false;
      }
    }
  }

  // Enable the requested modifier
  const modifier = this.modifiers.get(key);
  if (modifier && typeof modifier.isEnabled !== "undefined") {
    modifier.isEnabled = true;
    console.log(`[Chart2DBase] Enabled modifier: ${key}`);
  }
}

/**
 * Enable an independent modifier (doesn't affect others)
 */
enableIndependentModifier(key: string): void {
  const modifier = this.modifiers.get(key);
  if (modifier && typeof modifier.isEnabled !== "undefined") {
    modifier.isEnabled = true;
    console.log(`[Chart2DBase] Enabled independent modifier: ${key}`);
  }
}

/**
 * Disable a modifier
 */
disableModifier(key: string): void {
  const modifier = this.modifiers.get(key);
  if (modifier && typeof modifier.isEnabled !== "undefined") {
    modifier.isEnabled = false;
    console.log(`[Chart2DBase] Disabled modifier: ${key}`);
  }
}

/**
 * Get currently enabled mutually exclusive modifier key
 */
getEnabledModifierKey(): string | null {
  const mutuallyExclusive = this.modifierRegistry?.getMutuallyExclusiveModifiers(
    this.chart?.chart_type || "",
  ) || [];

  for (const modConfig of mutuallyExclusive) {
    const modifier = this.modifiers.get(modConfig.key);
    if (modifier && typeof modifier.isEnabled !== "undefined" && modifier.isEnabled) {
      return modConfig.key;
    }
  }
  return null;
}

/**
 * Get currently enabled independent modifier keys
 */
getEnabledIndependentModifierKeys(): string[] {
  const independent = this.modifierRegistry?.getIndependentModifiers(
    this.chart?.chart_type || "",
  ) || [];

  const enabled: string[] = [];
  for (const modConfig of independent) {
    const modifier = this.modifiers.get(modConfig.key);
    if (modifier && typeof modifier.isEnabled !== "undefined" && modifier.isEnabled) {
      enabled.push(modConfig.key);
    }
  }
  return enabled;
}
```

**What Happens**:

1. **For Mutually Exclusive Modifiers**:
   - Checks registry for modifiers that should be disabled
   - Disables all other mutually exclusive modifiers
   - Enables the requested modifier
   - Only one mutually exclusive modifier can be active at a time

2. **For Independent Modifiers**:
   - Simply toggles the modifier's `isEnabled` property
   - Does not affect other modifiers
   - Multiple independent modifiers can be active simultaneously

3. **State Updates**:
   - Modifier's `isEnabled` property is set directly
   - SciChart automatically reflects the change in chart behavior
   - Reactive state in sidebar component updates to show new active state

---

### Step 5: SciChart Reflects Modifier State

**Location**: SciChart library (internal)

**What Happens**:

1. When `modifier.isEnabled = true`:
   - SciChart activates the modifier's behavior
   - Modifier starts responding to mouse/keyboard events
   - Visual feedback appears (crosshair, selection highlights, etc.)

2. When `modifier.isEnabled = false`:
   - SciChart deactivates the modifier's behavior
   - Modifier stops responding to events
   - Visual feedback disappears

**Key Point**: SciChart modifiers have built-in `isEnabled` property that controls their active state. We simply toggle this property, and SciChart handles the rest.

---

### Step 6: Sidebar Updates Visual State

**Location**: `chart-editor-interactions-sidebar-xy.svelte` (reactive updates)

**What Happens**:

1. Chart state methods update modifier `isEnabled` properties
2. Sidebar component's `$derived` values automatically recompute:
   - `enabledMutuallyExclusiveKey` - checks which mutually exclusive modifier is enabled
   - `enabledIndependentKeys` - checks which independent modifiers are enabled
3. Button `variant` prop updates based on active state:
   - `variant: 'default'` for active modifiers (highlighted)
   - `variant: 'ghost'` for inactive modifiers (subtle)
4. UI re-renders to show new active/inactive states

**Reactive Flow**:

```
Chart State: modifier.isEnabled = true
    ↓
Sidebar: enabledMutuallyExclusiveKey = $derived(chartState.getEnabledModifierKey())
    ↓
Button: variant = isActive ? 'default' : 'ghost'
    ↓
UI: Button highlights to show active state
```

---

## Modifier Types and Discrimination

The modifier system distinguishes between two types of modifiers based on their interaction behavior:

### Mutually Exclusive Modifiers

These modifiers cannot be active simultaneously. Enabling one automatically disables others in the group. They are used for interaction modes that conflict with each other.

**Examples**:

- **Pan** (`ZoomPanModifier`) - Allows dragging to pan the chart
- **Data Selection** (`DataPointSelectionModifier`) - Allows clicking to select data points

**Configuration**:

```typescript
{
  key: "pan",
  name: "Pan",
  icon: "Move",
  description: "Pan the chart by dragging",
  enabledByDefault: false,
  mutuallyExclusiveWith: ["dataSelection"], // Disables dataSelection when enabled
  isIndependent: false, // Part of mutually exclusive group
}
```

**Behavior**:

- Only one can be active at a time
- Clicking an active modifier disables it
- Clicking an inactive modifier enables it and disables the currently active one
- Each modifier has a unique background color when enabled (e.g., Pan = blue, Data Selection = green)

**Why Mutually Exclusive?**:

- Pan and Data Selection both respond to mouse clicks/drags, but with different behaviors
- Having both active simultaneously would cause conflicts (e.g., trying to pan while selecting points)
- Users need to choose one interaction mode at a time

### Independent Modifiers

These modifiers can be active alongside other modifiers (including mutually exclusive ones). They provide additional functionality that doesn't conflict with interaction modes.

**Examples**:

- **Crosshair** (`CursorModifier`) - Shows crosshair cursor with tooltips
- **Series Selection** (`SeriesSelectionModifier`) - Select entire series by clicking on them
- **Zoom Extents** (`ZoomExtentsModifier`) - Button to reset zoom (always available)

**Configuration**:

```typescript
{
  key: "crosshair",
  name: "Crosshair",
  icon: "Crosshair",
  description: "Show crosshair cursor with tooltips",
  enabledByDefault: false,
  isIndependent: true, // Can be enabled alongside other modifiers
  // No mutuallyExclusiveWith - doesn't conflict with others
}
```

**Behavior**:

- Can be toggled independently (click to enable, click again to disable)
- Multiple independent modifiers can be active simultaneously
- Can be active alongside one mutually exclusive modifier
- Each modifier has a unique background color when enabled (e.g., Crosshair = orange)

**Why Independent?**:

- **Crosshair**: Provides visual feedback (cursor lines and tooltips) that doesn't interfere with panning or selection. It's a passive display feature that enhances the user experience without changing interaction behavior.
- **Series Selection**: Allows users to select entire series by clicking on them, which is useful for highlighting or analyzing specific data series. This doesn't conflict with panning or data point selection, as it operates at a different level (series vs. individual points).

Users can have multiple independent modifiers enabled simultaneously, and they can be active alongside one mutually exclusive modifier.

### Visual Discrimination

The interactions sidebar displays modifiers in two separate sections:

1. **Top Section**: Mutually exclusive modifiers (Pan, Data Selection)
   - Only one icon highlighted at a time
   - Each has a unique color when active (blue for Pan, green for Data Selection)

2. **Bottom Section**: Independent modifiers (Crosshair, Series Selection)
   - Separated by a border
   - Can have multiple icons highlighted simultaneously
   - Each has a unique color when active (orange for Crosshair, yellow for Series Selection)

**Color Coding**:

All modifiers (both mutually exclusive and independent) display unique background colors when enabled:

- **Pan**: Blue background (`!bg-blue-500`) with white text
- **Data Selection**: Green background (`!bg-green-500`) with white text
- **Crosshair**: Orange background (`!bg-orange-500`) with white text
- **Series Selection**: Yellow background (`!bg-yellow-500`) with white text
- **Rubber Band Zoom**: Purple background (`!bg-purple-500`) with white text
- **Mouse Wheel Zoom**: Pink background (`!bg-pink-500`) with white text
- **Zoom Extents**: Cyan background (`!bg-cyan-500`) with white text

**Visual Implementation**:

- All modifier buttons use `variant: 'ghost'` as the base style
- When enabled, the button's background color changes to the modifier's unique color
- White text (`text-white`) is applied for better contrast on colored backgrounds
- The `!` prefix in Tailwind classes ensures the background color overrides default button styles

This visual distinction helps users understand which modifiers can be used together and which are mutually exclusive. The consistent color coding across both sections makes it easy to identify active modifiers at a glance.

---

## Key Files Reference

| File                                          | Purpose                                                  |
| --------------------------------------------- | -------------------------------------------------------- |
| `chart-modifier-registry.ts`                  | Centralized modifier configuration                       |
| `chart-2D-base.svelte.ts`                     | Base class with modifier management methods              |
| `xy-chart-state.svelte.ts`                    | XY chart state (creates modifiers during initialization) |
| `chart-editor-interactions-sidebar-xy.svelte` | Sidebar UI for XY-based charts                           |
| `chart-editor-interactions-sidebar.svelte`    | Router component for chart-type-specific sidebars        |

---

## Benefits

### ✅ **Centralized Configuration**

- Single source of truth for modifier availability
- Easy to add new modifiers or chart types
- Consistent behavior across all charts

### ✅ **Type Safety**

- TypeScript ensures correct modifier keys
- Compile-time validation of modifier relationships
- IDE autocomplete for modifier names

### ✅ **Reactive UI**

- Automatic visual feedback when modifiers are toggled
- No manual state synchronization needed
- Consistent user experience

### ✅ **Flexible Architecture**

- Supports both mutually exclusive and independent modifiers
- Easy to extend with new modifier types
- Clear separation of concerns

---

## Example: Enabling Crosshair on XY Chart

1. **User clicks crosshair icon** in interactions sidebar
2. **Sidebar handler** calls `chartState.enableIndependentModifier('crosshair')`
3. **Chart state** sets `crosshairModifier.isEnabled = true`
4. **SciChart** activates crosshair, showing cursor lines and tooltips
5. **Sidebar** updates to show crosshair icon as active (highlighted)
6. **User can still use pan or data selection** (crosshair is independent)

---

## Summary

The modifier activation system provides a clean, reactive, and type-safe way to manage chart interactions. By centralizing configuration in the registry and using reactive state management, we ensure consistent behavior across all chart types while maintaining flexibility for future extensions.
