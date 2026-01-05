# Chart Settings Menubar

## Overview

The chart settings menubar is a horizontal toolbar located at the top of the chart editor that provides quick access to chart settings and information. It follows a pattern similar to professional financial charting applications, with chart-type-specific icons and dialogs.

## Architecture

### Layout Structure

```
chart-editor.svelte
├── Top Menubar (flex-shrink-0)
│   └── chart-editor-settings-menubar.svelte
│       └── [Chart-type-specific menubar component]
├── Main Content Area (flex-1)
│   ├── Left Sidebar (48px) - Interactions
│   ├── Center - Chart Visualization
│   └── (No right sidebar - removed for cleaner layout)
```

### Component Structure

**Main Router Component**: `chart-editor-settings-menubar.svelte`

- Dynamically renders chart-type-specific menubar components based on `selectedChart.chart_type`
- Includes common icons (Settings, Info) that open dialogs
- Passes `chart` prop to chart-type-specific components

**Chart-Type-Specific Components**:

- `chart-editor-settings-menubar-xy.svelte` - For `xy`, `line`, `scatter`, `geothermal_gradient` charts
- `chart-editor-settings-menubar-well-correlation.svelte` - For `well_correlation` charts
- `chart-editor-settings-menubar-map.svelte` - For `map` charts

## Common Icons

All chart types include these common icons:

1. **Settings Icon** (`Settings` from `@lucide/svelte`)
   - Opens `ChartSettingsEditor` in a dialog
   - Provides access to all chart settings (title, grid, axes, series, opacity, etc.)

2. **Info Icon** (`Info` from `@lucide/svelte`)
   - Opens `ChartsEditorInfoSection` in a dialog
   - Displays chart metadata, configuration, and data source information

## Chart-Type-Specific Icons

Each chart type can define additional quick-access icons for frequently used settings:

### XY-Based Charts (`xy`, `line`, `scatter`, `geothermal_gradient`)

Placeholder icons for future quick-access settings:

- **Grid** (`LayoutGrid`) - Grid settings
- **Axis** (`MoveHorizontal`) - Axis settings
- **Palette** (`Paintbrush`) - Series colors
- **Opacity** (`Layers`) - Opacity settings
- **Line Style** (`Minus`) - Line style settings

**Note**: These are currently placeholder icons (disabled). Future implementation will connect them to specific settings dialogs or quick-access panels.

## Dialog Integration

### Settings Dialog

The Settings icon opens `ChartSettingsEditor` in a dialog:

```svelte
<Dialog.Root bind:open={settingsDialogOpen}>
  <Dialog.Trigger>
    <Settings class="size-4" />
  </Dialog.Trigger>
  <Dialog.Content class="max-w-2xl max-h-[90vh]">
    <ChartSettingsEditor />
  </Dialog.Content>
</Dialog.Root>
```

**Features**:
- Full access to all chart settings
- Tabbed interface (Info | Settings)
- Scrollable content for long settings lists
- Responsive dialog sizing

### Info Dialog

The Info icon opens `ChartsEditorInfoSection` in a dialog:

```svelte
<Dialog.Root bind:open={infoDialogOpen}>
  <Dialog.Trigger>
    <Info class="size-4" />
  </Dialog.Trigger>
  <Dialog.Content class="max-w-lg max-h-[80vh]">
    <ChartsEditorInfoSection chart={chart} />
  </Dialog.Content>
</Dialog.Root>
```

**Features**:
- Chart metadata display
- Configuration JSON preview
- Data source information
- Read-only information display

## Type-Safe Component Routing

The menubar uses type-safe component routing based on `chart.chart_type`:

```svelte
function getChartTypeMenubarComponent(chart: Chart | null) {
  if (!chart) return null;
  
  switch (chart.chart_type) {
    case 'xy':
    case 'line':
    case 'scatter':
    case 'geothermal_gradient':
      return ChartEditorSettingsMenubarXy;
    case 'well_correlation':
      return ChartEditorSettingsMenubarWellCorrelation;
    case 'map':
      return ChartEditorSettingsMenubarMap;
    default:
      return null;
  }
}
```

## Benefits

1. **Cleaner Layout**: Removes congestion from the main chart area
2. **Professional Appearance**: Similar to financial charting applications
3. **Quick Access**: Common settings accessible via icons
4. **Type Safety**: Chart-type-specific icons ensure correct functionality
5. **Extensibility**: Easy to add new chart types and icons
6. **Dialog-Based Settings**: Settings don't take up permanent screen space

## Future Enhancements

1. **Quick-Access Panels**: Connect placeholder icons to specific settings
2. **Keyboard Shortcuts**: Add keyboard shortcuts for common actions
3. **Tooltips**: Enhanced tooltips with keyboard shortcut hints
4. **Icon Badges**: Show notification badges for unsaved changes
5. **Contextual Icons**: Show/hide icons based on chart state

