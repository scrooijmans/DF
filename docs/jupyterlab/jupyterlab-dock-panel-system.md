# JupyterLab Dock Panel System Implementation

This document explains how JupyterLab implements its dock panel system, including widget management, layout state persistence, and restoration mechanisms.

## Overview

JupyterLab's dock panel system is built on top of Lumino widgets (formerly PhosphorJS), providing a flexible docking interface where widgets can be added, moved, resized, and organized into a hierarchical layout structure. The system supports two modes: `multiple-document` (default) and `single-document`.

## Core Components

### DockPanelSvg

The main dock panel class (`DockPanelSvg`) extends Lumino's `DockPanel` and provides the primary interface for managing docked widgets. It's located in `@lumino/widgets` and wrapped by JupyterLab's UI components.

## Adding Widgets

### Basic Widget Addition

Widgets are added to the dock panel using the `addWidget()` method:

```typescript
dockPanel.addWidget(widget: Widget, options?: IAddOptions): void
```

**Parameters:**

- `widget`: The widget instance to add
- `options`: Optional configuration object (`IAddOptions`) that specifies:
  - Where to add the widget (reference widget, mode, etc.)
  - Whether to activate it immediately

**Behavior:**

- In `single-document` mode, options are ignored and the widget is always added as a tab in the hidden tab bar
- In `multiple-document` mode, options control placement and docking behavior
- If a widget is already in the panel, it will be moved to the new position

### Widget Activation and Selection

After adding, widgets can be activated or selected:

```typescript
// Activate a widget (selects and focuses it)
dockPanel.activateWidget(widget: Widget): void

// Select a widget (makes it current in its tab area)
dockPanel.selectWidget(widget: Widget): void
```

## Moving Widgets

### Drag and Drop

Widgets can be moved through drag-and-drop interactions:

1. **Tab Dragging**: Users can drag tabs to reposition widgets within the same tab bar or to different tab bars
2. **Docking Zones**: When dragging, the dock panel shows drop zones indicating where the widget can be docked:
   - Left/Right/Top/Bottom edges of existing panels
   - Center of existing panels (to create tabs)
   - Outside the panel (to create new floating panels, if allowed)

### Programmatic Movement

Widgets can be moved programmatically by:

- Re-adding them with different options
- Using `insertWidget()` on panels to change order
- The dock panel automatically handles widget movement when they're re-added

### Tab Constraints

The `tabsConstrained` property controls whether tabs can be dragged outside their source dock panel:

```typescript
dockPanel.tabsConstrained: boolean
```

When `true`, tabs are constrained to their original dock panel. When `false`, tabs can be dragged to other dock panels.

## Resizing Widgets

### Split Panel System

The dock panel uses Lumino's `SplitPanel` internally to manage resizable regions. When widgets are docked side-by-side or stacked, they're organized in split panels.

### Relative Sizing

Split panels manage widget sizes using relative sizing:

```typescript
splitPanel.setRelativeSizes(sizes: number[], update?: boolean): void
```

**Parameters:**

- `sizes`: Array of relative size values for each widget
- `update`: Whether to update the layout immediately (default: `true`)

**Behavior:**

- Sizes are relative proportions (e.g., `[1, 2, 1]` means the middle widget is twice as large as the outer ones)
- Extra values are ignored; too few values may result in undefined layout
- DOM geometry updates asynchronously

### Orientation

Split panels can be oriented horizontally or vertically:

```typescript
splitPanel.orientation: Orientation  // 'horizontal' | 'vertical'
```

### Spacing and Alignment

Additional layout properties:

```typescript
splitPanel.spacing: number        // Inter-element spacing
splitPanel.alignment: Alignment   // Content alignment in layout direction
```

## Layout State Persistence

### Saving Layout State

The dock panel can save its current layout configuration:

```typescript
const config: ILayoutConfig = dockPanel.saveLayout();
```

**Returns:** An `ILayoutConfig` object representing the current layout state, including:

- Widget positions and relationships
- Split panel configurations
- Tab bar states
- Relative sizes

**Important Notes:**

- The returned config can be provided to `restoreLayout()` to restore the layout
- Changing the dock panel mode is destructive - save layout before changing modes

### Restoring Layout State

Saved layouts can be restored:

```typescript
dockPanel.restoreLayout(config: ILayoutConfig): void
```

**Behavior:**

- Widgets currently in the layout but not in the config will be unparented (removed)
- The dock panel automatically reverts to `multiple-document` mode when restoring
- Widgets referenced in the config must exist and be available

## Application-Level Layout Persistence

### LayoutRestorer

JupyterLab provides a `LayoutRestorer` class that manages application-wide layout state persistence:

```typescript
interface ILayoutRestorer {
  restored: Promise<void>;
  add(widget: Widget, name: string): void;
  restore<T extends Widget>(
    tracker: WidgetTracker<T>,
    options: IRestorer.IOptions<T>
  ): Promise<any>;
}
```

### Widget Registration

Widgets must be registered with the layout restorer to be tracked:

```typescript
layoutRestorer.add(widget: Widget, name: string): void
```

**Parameters:**

- `widget`: The widget to track
- `name`: Unique identifier for the widget

### Widget Restoration

Widgets are restored through their trackers:

```typescript
await layoutRestorer.restore(
    tracker: WidgetTracker<Widget>,
    options: IRestorer.IOptions<Widget>
): Promise<any>
```

**Parameters:**

- `tracker`: The widget tracker managing the widget type
- `options`: Restoration options including:
  - Command to create new widgets
  - Naming function
  - State restoration callbacks

### State Storage

The `LayoutRestorer` provides methods for saving and fetching layout state:

```typescript
// Save current layout state
await layoutRestorer.save(layout: ILayout): Promise<void>

// Fetch saved layout state
const layout: ILayout = await layoutRestorer.fetch()
```

**Important:** `fetch()` relies on all widget restoration being complete, ensuring the returned layout reflects the fully restored state.

### Restoration Promise

The `restored` promise resolves when the layout restorer is ready:

```typescript
await layoutRestorer.restored; // Promise<void>
```

This is useful for extensions that need to wait for layout restoration to complete before performing actions.

## LabShell Integration

### Shell Layout Management

The `LabShell` (application shell) coordinates layout restoration:

```typescript
await labShell.restoreLayout(
    mode: Mode,
    layoutRestorer: LayoutRestorer,
    configuration?: { [m: string]: IUserLayout }
): Promise<void>
```

**Parameters:**

- `mode`: The dock panel mode (`'multiple-document'` or `'single-document'`)
- `layoutRestorer`: The layout restorer instance
- `configuration`: Optional user layout configurations for each mode

**Important:** This should only be called once during application initialization.

### User Layout

The shell maintains user-customized layouts:

```typescript
labShell.userLayout: {
    "multiple-document": IUserLayout;
    "single-document": IUserLayout;
}
```

### Layout Saving

The shell can save its dehydrated state:

```typescript
const layout: ILayout = labShell.saveLayout();
```

### Restoration Status

The shell provides a promise that resolves when state is first restored:

```typescript
const layout: ILayout = await labShell.restored;
```

## Layout Structure

### ILayoutConfig

The layout configuration object (`ILayoutConfig`) contains:

- Hierarchical structure of panels and widgets
- Split panel configurations (orientation, sizes)
- Tab bar states
- Widget references and positions

### ILayout

The application layout object (`ILayout`) includes:

- Main area layout
- Side panel layouts
- Widget states
- Mode information

## Iterating Over Widgets

### User Widgets

Iterate over user widgets (excluding tab bars):

```typescript
for (const widget of dockPanel.widgets()) {
  // Process widget
}
```

### Selected Widgets

Iterate over currently selected widgets (current tab in each tab bar):

```typescript
for (const widget of dockPanel.selectedWidgets()) {
  // Process selected widget
}
```

### Tab Bars

Iterate over tab bars (excluding user widgets):

```typescript
for (const tabBar of dockPanel.tabBars()) {
  // Process tab bar
}
```

## Mode Management

### Dock Panel Modes

The dock panel supports two modes:

```typescript
dockPanel.mode: Mode  // 'multiple-document' | 'single-document'
```

**Multiple Document Mode:**

- Widgets can be docked in multiple panels
- Supports complex layouts with splits and tabs
- Default mode

**Single Document Mode:**

- All widgets are in a single hidden tab bar
- Only one widget visible at a time
- Simpler, notebook-like interface

**Important:** Changing mode is destructive - save layout before changing modes.

## Best Practices

1. **Always save layout before mode changes**: Mode changes destroy the current layout
2. **Register widgets with LayoutRestorer**: Required for state persistence
3. **Wait for restoration**: Use `restored` promises before accessing layout-dependent features
4. **Handle missing widgets**: When restoring, widgets may not exist - handle gracefully
5. **Use unique names**: Widget names in `LayoutRestorer.add()` must be unique
6. **Track widget lifecycle**: Ensure widgets are properly disposed when removed

## Summary

JupyterLab's dock panel system provides:

1. **Widget Management**: Add, move, and organize widgets through drag-and-drop or programmatic APIs
2. **Resizing**: Automatic split panel management with relative sizing
3. **State Persistence**: Save and restore layout configurations at both panel and application levels
4. **Mode Support**: Multiple-document and single-document modes for different workflows
5. **Integration**: Seamless integration with JupyterLab's shell and extension system

The system is built on Lumino widgets and provides a robust foundation for building flexible, stateful user interfaces in JupyterLab extensions.

