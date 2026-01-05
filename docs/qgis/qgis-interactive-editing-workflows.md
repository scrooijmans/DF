# QGIS Interactive Editing Workflows: Geometry and Attribute Editing

## Overview

QGIS enables users to create and modify geospatial data through a comprehensive set of interactive editing tools. The editing system is built around an **editing buffer** architecture that keeps changes in memory until explicitly committed, allowing for undo/redo operations and safe experimentation with edits.

## Interactive Editing Tools

### Geometry Editing Tools

QGIS provides two main toolbars for geometry editing:

1. **Digitizing Toolbar**: Core tools for creating and modifying vector features
   - Add point, line, and polygon features
   - Move, copy, rotate, and scale features
   - Delete features
   - Vertex manipulation tools

2. **Advanced Digitizing Toolbar**: Advanced tools for complex editing tasks
   - Reshape features
   - Split features
   - Merge features
   - Add/delete rings and parts
   - Simplify features
   - Topological editing

#### Vertex Editor Panel

When using vertex tools, QGIS automatically opens the **Vertex Editor Panel**, which provides:
- A table listing all vertices with their x, y, z (if applicable), and m coordinates
- Interactive selection: selecting a row highlights the corresponding vertex on the map canvas
- Direct coordinate editing: changing values in the table updates vertex positions in real-time
- Multi-vertex selection and deletion
- Exclusive editing mode: when a feature is being edited, other features are temporarily disabled

### Attribute Editing Tools

Attribute editing is available through:

1. **Attribute Table**: 
   - Direct cell editing (double-click to modify)
   - Field Calculator for complex calculations
   - Multi-edit mode for updating multiple features simultaneously
   - Quick field calculation bar

2. **Feature Form**: 
   - Modal dialog for editing individual feature attributes
   - Customizable form layouts
   - Support for related features

## Edit Sessions: How They Work

### Starting an Edit Session

To begin editing, users must activate **edit mode** for a layer by:
- Clicking the "Toggle Editing" button (available from layer context menu, attribute table, or digitizing toolbar)
- Using the Layer menu → Toggle Editing
- Programmatically via `layer.startEditing()`

When edit mode is activated:
- Additional tool buttons become active
- Vertex markers appear on features (unless "Show markers only for selected features" is enabled)
- The layer's editing buffer is initialized
- The layer enters a state where changes are tracked but not yet persisted

### The Editing Buffer

**Key Concept**: All changes made during an edit session are stored in an **in-memory editing buffer**, not written to the data source until explicitly committed.

```python
# Programmatic example
layer.startEditing()  # Initialize edit buffer
# ... make changes ...
layer.commitChanges()  # Write to data provider
# OR
layer.rollBack()  # Discard all changes
```

The editing buffer architecture provides several benefits:
- **Safety**: Changes can be discarded without affecting the original data
- **Undo/Redo**: Operations can be reversed before committing
- **Performance**: Multiple edits can be batched into a single write operation
- **User Control**: Users decide when to persist changes

### Transaction Modes

QGIS supports different transaction modes for managing edits:

1. **Local Edit Buffer** (default):
   - Edits are buffered locally
   - Sent to provider when toggling edit mode or clicking "Save Edits"

2. **Automatic Transaction Groups**:
   - For supported datasources (PostgreSQL, GeoPackage)
   - Edits sent directly to server-side transaction
   - All tables from same database synchronized
   - Committed when toggling edit mode

3. **Buffered Transaction Groups**:
   - All editable layers synchronized
   - Edits saved in local buffer
   - Saving executes within single transaction per provider

## Commit and Rollback Mechanisms

### Committing Changes

Changes are committed to the data source when:
- User clicks "Save Edits" button (keeps layer in edit mode)
- User toggles edit mode off (prompts to save or discard)
- Programmatically via `layer.commitChanges()`
- Using context manager: `with edit(layer):` (automatically commits on success)

```python
# Using context manager (recommended)
with edit(layer):
    # Make changes
    layer.addFeature(feature)
    layer.changeAttributeValue(fid, field_index, new_value)
    # Automatically commits on success
    # Automatically rolls back on exception
```

### Rolling Back Changes

Changes can be discarded by:
- Clicking "Rollback" button
- Choosing "Discard" when prompted on exit
- Programmatically via `layer.rollBack()`
- Exception handling in context manager

When rolling back:
- All changes in the editing buffer are discarded
- Layer returns to state before edit session started
- No changes are written to the data provider

### Error Handling

If commit fails (e.g., disk full, invalid attribute values):
- QGIS preserves the in-memory state
- User can correct issues and retry
- `commitError()` method returns the last error message
- Context manager raises `QgsEditError` exception on commit failure

## View Updates in Response to Changes

### Real-Time Canvas Updates

QGIS uses a **signal-based architecture** to keep views synchronized with edits:

#### Key Signals Emitted During Editing

1. **Geometry Changes**:
   - `geometryChanged`: Emitted when geometry is modified in edit buffer
   - Triggers canvas repaint to show updated geometry

2. **Attribute Changes**:
   - `attributeValueChanged(fid, field, value)`: Emitted when attribute value changes
   - Updates attribute table displays
   - May trigger label/symbology updates if dependent on attributes

3. **Feature Lifecycle**:
   - `featureAdded`: New feature added to layer
   - `featureDeleted`: Feature removed from layer
   - `featuresDeleted`: Multiple features removed

4. **Edit Commands**:
   - `editCommandStarted`: New undo/redo command begins
   - `editCommandEnded`: Command successfully completed
   - `editCommandDestroyed`: Command discarded

5. **Commit/Rollback Events**:
   - `beforeCommitChanges`: Before changes are written
   - `afterCommitChanges`: After successful commit
   - `beforeRollBack`: Before rollback
   - `afterRollBack`: After rollback

### Canvas Refresh Mechanisms

The map canvas updates through several mechanisms:

1. **Automatic Refresh**:
   - Canvas automatically repaints when layer signals are emitted
   - Changes visible immediately during editing
   - No manual refresh needed for most operations

2. **Manual Refresh**:
   ```python
   canvas.refresh()  # Quick repaint
   canvas.refreshAllLayers()  # Reload all layers and refresh
   ```

3. **Extent Updates**:
   - `layer.recalculateExtents()`: Requests canvas to update extents
   - Automatically called when features are added/deleted
   - Ensures zoom-to-layer operations show all features

4. **Freeze/Thaw**:
   ```python
   canvas.freeze(True)  # Prevent updates during batch operations
   # ... make multiple changes ...
   canvas.freeze(False)  # Resume updates
   ```

### Attribute Table Synchronization

The attribute table stays synchronized through:

1. **Visible Features List**:
   - `visibleReloaded` signal: Emitted when visible features change
   - Updates when map extent changes
   - Filters table to show only visible features (if enabled)

2. **Cache Updates**:
   - `QgsVectorLayerCache` re-emits layer signals
   - Ensures cached values are updated
   - Prevents stale data in attribute table

3. **Filter Model Updates**:
   - `generateListOfVisibleFeatures()`: Updates visible feature list
   - Called automatically on extent changes
   - Keeps table in sync with map view

### Undo/Redo Panel Updates

The Undo/Redo panel:
- Shows history of operations for each edited layer
- Updates in real-time as operations are performed
- Selecting an operation reverts features to that state
- Synchronized with edit command signals

## Undo/Redo System

### Edit Commands

For undo/redo to work correctly, operations must be wrapped in **edit commands**:

```python
layer.beginEditCommand("Add feature")
# ... make changes ...
layer.endEditCommand()  # Pushes command onto undo stack
```

If an error occurs:
```python
layer.destroyEditCommand()  # Removes command and discards changes
```

### Keyboard Shortcuts

- **Ctrl+Z**: Undo last operation
- **Ctrl+Shift+Z**: Redo last undone operation

### Undo/Redo Panel

A dockable widget showing:
- Complete history of operations
- Ability to jump to any previous state
- Visual indication of current position in history

## Special Cases

### GRASS Vector Layers

GRASS layers have unique behavior:
- Changes are **immediately written** to the vector map (not buffered)
- Undo/redo still available through QGIS
- Discard changes option reverts entire session
- Specialized "GRASS Edit" renderer applied during editing
- Topological editing with automatic symbology

### Mesh Layers

Mesh editing includes:
- Frame editing with commit/rollback
- Vertex coordinate transformation
- Face splitting based on geometry
- Reindexing faces and vertices

## Best Practices

1. **Use Context Manager**: Prefer `with edit(layer):` for automatic error handling
2. **Wrap Operations**: Use `beginEditCommand()`/`endEditCommand()` for undo/redo
3. **Check Edit Mode**: Verify `layer.isEditable()` before making changes
4. **Handle Errors**: Check return values of `commitChanges()` and handle `QgsEditError`
5. **Freeze Canvas**: Use `canvas.freeze()` for batch operations to improve performance
6. **Signal Connections**: Connect to layer signals for custom update logic

## Summary

QGIS provides a robust, user-friendly editing system that:

- **Separates editing from persistence**: Changes live in memory until committed
- **Enables safe experimentation**: Rollback discards unwanted changes
- **Supports undo/redo**: Edit commands track operations for reversal
- **Updates views automatically**: Signal-based architecture keeps UI synchronized
- **Handles errors gracefully**: Failed commits preserve state for correction
- **Supports multiple transaction modes**: Flexible persistence strategies

This architecture ensures data integrity while providing a responsive, interactive editing experience that updates all views (map canvas, attribute tables, panels) in real-time as users create and modify geospatial data.

