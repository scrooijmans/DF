# JupyterLab DockPanel Drag-and-Drop Reordering

This document explains how JupyterLab's DockPanel implements drag-and-drop reordering of widgets, including how drag data is represented and how new layout nodes are created during a drop.

## Overview

JupyterLab's DockPanel uses Lumino's drag-and-drop system to enable users to reorder and reorganize widgets by dragging tabs or widgets between different areas of the dock panel. The system handles drag data representation, drop target detection, and dynamic creation of layout nodes (split panels and tab areas) during the drop operation.

## Drag Data Representation

### MimeData Structure

During drag operations, widget information is stored in a `MimeData` object attached to the drag event:

```typescript
Drag.Event {
  mimeData: MimeData  // The mime data associated with the event
  proposedAction: DropAction  // The preferred drop action
  dropAction: DropAction  // The actual drop action taken
  // ... other event properties
}
```

### MimeData API

The `MimeData` class provides methods for storing and retrieving typed data:

```typescript
MimeData {
  setData(mime: string, data: any): void
  getData(mime: string): any
  hasData(mime: string): boolean
  deleteData(mime: string): void
  mimeTypes(): string[]
  clone(): MimeData
}
```

### Storing Widget References

When a drag operation starts, the widget reference is stored in the mime data using a specific MIME type:

```typescript
// During drag start
mimeData.setData('application/x-lumino-widget', widget);
```

**Key Points:**
- The widget object itself is stored (not just an identifier)
- This allows the drop handler to directly access the widget instance
- Multiple MIME types can be stored in the same `MimeData` object
- The MIME type `'application/x-lumino-widget'` is the standard for Lumino widgets

### Retrieving Widget References

During drop, the widget is extracted from the mime data:

```typescript
// During drop
const widget = event.mimeData.getData('application/x-lumino-widget');
if (widget) {
  // Process the widget
}
```

## Drop Event Handling

### Drag Over Events

During `'lm-dragover'` events, the drop target must indicate it can accept the drop:

```typescript
// In dragover handler
event.dropAction = 'move';  // or 'copy', 'link', 'none'
```

**Important:** If `dropAction` is not set to a supported action during `'lm-dragover'`, the drop event will not occur.

### Drop Events

When a drop occurs, the handler:

1. **Extracts the widget** from `mimeData`
2. **Determines the drop location** and insert mode
3. **Creates or modifies layout nodes** as needed

```typescript
// Drop handler
function handleDrop(event: Drag.Event) {
  const widget = event.mimeData.getData('application/x-lumino-widget');
  if (!widget) return;
  
  // Determine drop location and mode
  const insertMode = determineInsertMode(event);
  const refWidget = findReferenceWidget(event);
  
  // Add widget to new location
  dockPanel.addWidget(widget, {
    mode: insertMode,
    ref: refWidget
  });
}
```

## Insert Modes

### DockPanel.InsertMode

DockPanel uses `InsertMode` to specify how widgets should be inserted:

```typescript
DockPanel.InsertMode: {
  BEFORE,        // Insert before a reference widget
  AFTER,         // Insert after a reference widget
  SPLIT_TOP,     // Split area and place above
  SPLIT_BOTTOM,  // Split area and place below
  SPLIT_LEFT,    // Split area and place to the left
  SPLIT_RIGHT    // Split area and place to the right
}
```

### IAddOptions

When adding a widget, options specify the insertion behavior:

```typescript
DockPanel.IAddOptions {
  mode: DockPanel.InsertMode  // How to insert the widget
  ref?: string                 // Reference widget ID (for BEFORE/AFTER)
}
```

## Layout Node Creation Process

### Determining Drop Location

The drop handler analyzes the drop location to determine the appropriate action:

1. **Over a tab area**: Add widget to that tab area
2. **Over a split handle**: Create a new split area
3. **Over an edge**: Create a split area in that direction
4. **Over center of panel**: Add as a new tab

### Creating Tab Areas

When dropping over a tab area or center of a panel:

```typescript
// Widget is added to existing TabPanel
dockPanel.addWidget(widget, {
  mode: DockPanel.InsertMode.AFTER,
  ref: existingWidget
});

// Or creates new TabPanel if none exists
dockPanel.addWidget(widget);
```

**Process:**
- If a `TabPanel` exists at the location, the widget is added to it
- If no `TabPanel` exists, one is created automatically
- The widget becomes a new tab in the tab area

### Creating Split Areas

When dropping over an edge or split handle:

```typescript
// Create split area
dockPanel.addWidget(widget, {
  mode: DockPanel.InsertMode.SPLIT_RIGHT,
  ref: targetWidget
});
```

**Process:**
1. The target area is identified
2. A new `SplitPanel` is created
3. The dragged widget and target area become children of the split panel
4. The split panel is oriented based on the insert mode:
   - `SPLIT_TOP` / `SPLIT_BOTTOM`: Vertical split
   - `SPLIT_LEFT` / `SPLIT_RIGHT`: Horizontal split

### Layout Structure

The layout maintains a tree structure of areas:

```typescript
DockLayout.ISplitAreaConfig {
  size: number
  ratio: number
  // Contains child areas (split or tab)
}

DockLayout.ITabAreaConfig {
  tabs: DockPanel.ITab[]
  current: number
  // Contains widgets in a tabbed interface
}
```

**Hierarchy:**
- Root level: Split areas or tab areas
- Split areas: Contain other split areas or tab areas
- Tab areas: Contain widgets displayed as tabs

## Complete Drag-and-Drop Flow

### 1. Drag Start

```typescript
// User initiates drag on a tab or widget
function handleDragStart(event: Drag.Event) {
  // Store widget reference in mime data
  event.mimeData.setData('application/x-lumino-widget', widget);
  
  // Set proposed action
  event.proposedAction = 'move';
}
```

### 2. Drag Over

```typescript
// As user drags over potential drop targets
function handleDragOver(event: Drag.Event) {
  // Check if this is a valid drop target
  if (canAcceptDrop(event)) {
    // Indicate acceptance
    event.dropAction = 'move';
    
    // Show visual feedback (overlay)
    showDropOverlay(event);
  } else {
    event.dropAction = 'none';
  }
}
```

### 3. Drop

```typescript
// User releases mouse button
function handleDrop(event: Drag.Event) {
  // Extract widget from mime data
  const widget = event.mimeData.getData('application/x-lumino-widget');
  if (!widget) return;
  
  // Determine drop location
  const dropInfo = calculateDropLocation(event);
  
  // Remove widget from old location (if needed)
  if (widget.parent) {
    widget.parent = null;  // Automatically removes from layout
  }
  
  // Add widget to new location
  dockPanel.addWidget(widget, {
    mode: dropInfo.mode,
    ref: dropInfo.refWidget
  });
  
  // Hide overlay
  hideDropOverlay();
  
  // Set final drop action
  event.dropAction = 'move';
}
```

### 4. Layout Update

The layout system automatically:
- Creates new split panels or tab panels as needed
- Updates the layout tree structure
- Repositions widgets in the DOM
- Updates split panel sizes and ratios
- Maintains layout consistency

## Tab Movement

### Within Same Tab Bar

When dragging a tab within the same tab bar:

```typescript
// Tab is reordered within its TabPanel
tabPanel.insertWidget(newIndex, widget);
```

### Between Tab Bars

When dragging a tab to a different tab bar:

```typescript
// Widget is removed from old TabPanel
widget.parent = null;

// Widget is added to new TabPanel
newTabPanel.addWidget(widget);
```

### Tab Constraints

The `tabsConstrained` property controls tab movement:

```typescript
dockPanel.tabsConstrained: boolean
```

- `true`: Tabs can only be moved within their source dock panel
- `false`: Tabs can be moved to other dock panels

## Programmatic Widget Movement

Widgets can also be moved programmatically:

```typescript
// Move widget to new location
dockPanel.addWidget(widget, {
  mode: DockPanel.InsertMode.SPLIT_RIGHT,
  ref: referenceWidget
});

// If widget is already in panel, it will be moved
// No need to manually remove it first
```

**Note:** If a widget is already in the dock panel, calling `addWidget()` with different options will move it to the new location automatically.

## Layout Node Lifecycle

### Creation

Layout nodes are created on-demand during drop operations:

1. **TabPanel**: Created when first widget is added to a new tab area
2. **SplitPanel**: Created when a widget is dropped to split an existing area
3. **Layout Items**: Created automatically by the layout system

### Removal

Layout nodes are removed when they become empty:

1. **TabPanel**: Removed when last widget is removed
2. **SplitPanel**: Removed when it has fewer than 2 children
3. **Layout Items**: Removed automatically by the layout system

### Persistence

Layout structure is preserved in layout configurations:

```typescript
const config = dockPanel.saveLayout();
// Config includes all split panels, tab panels, and their relationships
```

## Best Practices

### Drag Data

1. **Use Standard MIME Types**: Use `'application/x-lumino-widget'` for widget references
2. **Store Complete Objects**: Store widget instances, not just IDs
3. **Check for Data**: Always verify data exists before using it
4. **Handle Multiple Types**: Support multiple MIME types if needed

### Drop Handling

1. **Set dropAction Early**: Set `dropAction` during `'lm-dragover'` events
2. **Validate Drop Targets**: Check if drop is valid before accepting
3. **Provide Visual Feedback**: Show overlays or highlights during drag
4. **Handle Edge Cases**: Handle drops outside valid areas gracefully

### Layout Creation

1. **Let System Create Nodes**: Don't manually create TabPanels or SplitPanels
2. **Use Insert Modes**: Use appropriate insert modes for desired behavior
3. **Preserve Layout**: Save layout before making major changes
4. **Handle Empty Areas**: Clean up empty layout nodes when appropriate

### Performance

1. **Minimize DOM Updates**: Batch layout updates when possible
2. **Debounce Overlay Updates**: Update overlay position with throttling
3. **Cache Drop Calculations**: Cache drop location calculations during drag
4. **Optimize Layout Updates**: Use layout update batching for multiple moves

## Summary

JupyterLab's DockPanel drag-and-drop reordering system provides:

1. **Drag Data Representation**: Widget references stored in `MimeData` using standard MIME types
2. **Drop Event Handling**: Comprehensive event system for drag over and drop operations
3. **Insert Modes**: Flexible insertion modes (BEFORE, AFTER, SPLIT_*) for precise placement
4. **Layout Node Creation**: Automatic creation of TabPanels and SplitPanels as needed
5. **Layout Structure**: Hierarchical tree of split areas and tab areas
6. **Tab Movement**: Support for moving tabs within and between tab bars
7. **Programmatic Control**: APIs for programmatic widget movement

The system enables intuitive drag-and-drop reordering while maintaining layout consistency and providing a flexible foundation for complex widget arrangements.


