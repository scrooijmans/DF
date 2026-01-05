# AG Grid JavaScript: macOS Finder-Like Row Selection

This document explains how to implement macOS Finder-like row selection in AG Grid, where users can click to select single rows and use Cmd+Click (or Ctrl+Click) to select multiple non-contiguous rows, similar to selecting files in macOS Finder.

## Overview

AG Grid supports Finder-like selection behavior where:
- **Single Click**: Selects a row (or toggles selection in multi-row mode)
- **Cmd+Click** (macOS) / **Ctrl+Click** (Windows/Linux): Adds/removes rows from selection without clearing existing selections
- **Shift+Click**: Selects a range of rows (optional, can be configured)

## Basic Configuration

### Enable Finder-Like Selection

```javascript
import { RowSelectionModule } from 'ag-grid-community';
import { ModuleRegistry, createGrid, GridOptions } from 'ag-grid-community';

ModuleRegistry.registerModules([RowSelectionModule]);

const gridOptions = {
    columnDefs: [
        { field: 'athlete' },
        { field: 'age' },
        { field: 'country' }
    ],
    
    // Enable multi-row selection
    rowSelection: {
        mode: 'multiRow',
        // Enable click selection (Finder-like behavior)
        enableClickSelection: true,
        // Hide checkboxes for cleaner UI (optional)
        checkboxes: false,
        headerCheckbox: false
    }
};
```

### Key Properties

- **`mode: 'multiRow'`**: Enables selecting multiple rows
- **`enableClickSelection: true`**: Enables clicking rows to select them (Finder-like behavior)
- **`checkboxes: false`**: Hides checkboxes for a cleaner, Finder-like appearance (optional)

## Selection Behavior

### Single Click Behavior

With `enableClickSelection: true`:

```javascript
const gridOptions = {
    rowSelection: {
        mode: 'multiRow',
        enableClickSelection: true
    }
};
```

**Behavior:**
- **First click on unselected row**: Selects that row and **clears previous selections**
- **Click on selected row**: Deselects that row (in multi-row mode)
- **Cmd+Click / Ctrl+Click on unselected row**: Adds row to selection **without clearing** other selections
- **Cmd+Click / Ctrl+Click on selected row**: Removes row from selection **without affecting** other selections

### Example: Finder-Like Selection Flow

```
Initial state: No rows selected

1. User clicks Row 1
   → Row 1 selected, others deselected

2. User Cmd+Clicks Row 3
   → Row 1 and Row 3 selected

3. User Cmd+Clicks Row 5
   → Row 1, Row 3, and Row 5 selected

4. User Cmd+Clicks Row 3 (already selected)
   → Row 1 and Row 5 selected (Row 3 deselected)

5. User clicks Row 2 (without Cmd)
   → Row 2 selected, Row 1 and Row 5 deselected
```

## Complete Example

### Basic Finder-Like Selection

```javascript
import {
    ClientSideRowModelModule,
    GridApi,
    GridOptions,
    ModuleRegistry,
    RowSelectionModule,
    ValidationModule,
    createGrid,
} from 'ag-grid-community';
import {
    ColumnMenuModule,
    ColumnsToolPanelModule,
    ContextMenuModule,
} from 'ag-grid-enterprise';

ModuleRegistry.registerModules([
    RowSelectionModule,
    ClientSideRowModelModule,
    ColumnsToolPanelModule,
    ColumnMenuModule,
    ContextMenuModule,
    ...(process.env.NODE_ENV !== "production" ? [ValidationModule] : []),
]);

let gridApi: GridApi;

const gridOptions = {
    columnDefs: [
        { field: 'athlete', minWidth: 150 },
        { field: 'age', maxWidth: 90 },
        { field: 'country', minWidth: 150 },
        { field: 'year', maxWidth: 90 },
        { field: 'date', minWidth: 150 },
        { field: 'sport', minWidth: 150 },
        { field: 'gold' },
        { field: 'silver' },
        { field: 'bronze' },
        { field: 'total' }
    ],
    
    defaultColDef: {
        flex: 1,
        minWidth: 100
    },
    
    // Finder-like selection configuration
    rowSelection: {
        mode: 'multiRow',
        enableClickSelection: true,  // Enable click selection
        checkboxes: false,           // Hide checkboxes (optional)
        headerCheckbox: false        // Hide header checkbox (optional)
    },
    
    // Optional: Handle selection changes
    onSelectionChanged: (event) => {
        const selectedRows = event.api.getSelectedRows();
        console.log('Selected rows:', selectedRows.length);
    }
};

const gridDiv = document.querySelector('#myGrid');
gridApi = createGrid(gridDiv, gridOptions);

// Load data
fetch('https://www.ag-grid.com/example-assets/olympic-winners.json')
    .then(response => response.json())
    .then(data => gridApi.setGridOption('rowData', data));
```

## Advanced Configuration Options

### enableClickSelection Values

The `enableClickSelection` property accepts different values for fine-grained control:

```javascript
const gridOptions = {
    rowSelection: {
        mode: 'multiRow',
        
        // Option 1: Enable both selection and deselection
        enableClickSelection: true,
        // - Click: Selects row (clears others)
        // - Cmd+Click: Toggles selection (adds/removes)
        
        // Option 2: Only enable selection (no deselection by clicking)
        enableClickSelection: 'enableSelection',
        // - Click: Selects row (clears others)
        // - Cmd+Click: Adds to selection
        // - Cannot deselect by clicking (use Cmd+Click on selected row)
        
        // Option 3: Only enable deselection
        enableClickSelection: 'enableDeselection',
        // - Click: No effect
        // - Cmd+Click: Toggles selection (adds/removes)
        
        // Option 4: Disable click selection
        enableClickSelection: false,
        // - Click: No selection effect
        // - Use checkboxes or API only
    }
};
```

### With Checkboxes (Hybrid Approach)

You can enable both checkboxes and click selection:

```javascript
const gridOptions = {
    rowSelection: {
        mode: 'multiRow',
        enableClickSelection: true,
        checkboxes: true,        // Show checkboxes
        headerCheckbox: true     // Show header checkbox
    }
};
```

**Behavior:**
- Users can select via checkboxes OR clicking rows
- Cmd+Click still works for multi-selection
- Header checkbox selects/deselects all

### Touch Device Support

For touch devices (tablets, phones), enable selection without modifier keys:

```javascript
const gridOptions = {
    rowSelection: {
        mode: 'multiRow',
        enableClickSelection: true,
        enableSelectionWithoutKeys: true,  // Enable touch selection
        checkboxes: false,
        headerCheckbox: false
    }
};
```

**Behavior:**
- **Single tap**: Selects row (clears others)
- **Tap another row**: Adds to selection (no modifier needed)
- Works on touch devices without keyboard modifiers

## Selection Events

### onSelectionChanged

Fired whenever the selection changes:

```javascript
const gridOptions = {
    rowSelection: {
        mode: 'multiRow',
        enableClickSelection: true
    },
    
    onSelectionChanged: (event) => {
        const selectedRows = event.api.getSelectedRows();
        const selectedNodes = event.api.getSelectedNodes();
        
        console.log('Selection changed:', {
            count: selectedRows.length,
            rows: selectedRows,
            nodes: selectedNodes
        });
        
        // Update UI based on selection
        updateSelectionUI(selectedRows);
    }
};
```

### onRowClicked

Fired when a row is clicked (before selection changes):

```javascript
const gridOptions = {
    onRowClicked: (event) => {
        console.log('Row clicked:', {
            data: event.data,
            node: event.node,
            isSelected: event.node.isSelected(),
            event: event.event  // Original click event
        });
        
        // Check if Cmd/Ctrl was pressed
        const isModifierPressed = event.event.metaKey || event.event.ctrlKey;
        console.log('Modifier pressed:', isModifierPressed);
    }
};
```

## Selection API

### Get Selected Rows

```javascript
// Get selected row data
const selectedRows = gridApi.getSelectedRows();
console.log('Selected rows:', selectedRows);

// Get selected nodes (for accessing node properties)
const selectedNodes = gridApi.getSelectedNodes();
selectedNodes.forEach(node => {
    console.log('Node:', {
        data: node.data,
        isSelected: node.isSelected(),
        id: node.id
    });
});
```

### Programmatic Selection

```javascript
// Select specific rows
gridApi.forEachNode((node) => {
    if (node.data.country === 'USA') {
        node.setSelected(true);  // Add to selection
    }
});

// Select exclusively (clear others)
gridApi.forEachNode((node) => {
    if (node.data.country === 'USA') {
        node.setSelected(true, true);  // Select and clear others
    }
});

// Deselect all
gridApi.deselectAll();

// Select all
gridApi.selectAll();
```

### Set Multiple Nodes Selected

```javascript
// Collect nodes to select
const nodesToSelect = [];
gridApi.forEachNode((node) => {
    if (node.data.year === 2012) {
        nodesToSelect.push(node);
    }
});

// Select all at once
gridApi.setNodesSelected({
    nodes: nodesToSelect,
    newValue: true  // true = select, false = deselect
});
```

## Complete Finder-Like Example

### Full Implementation

```javascript
import {
    ClientSideRowModelModule,
    GridApi,
    GridOptions,
    ModuleRegistry,
    RowSelectionModule,
    ValidationModule,
    createGrid,
} from 'ag-grid-community';
import {
    ColumnMenuModule,
    ColumnsToolPanelModule,
    ContextMenuModule,
} from 'ag-grid-enterprise';

ModuleRegistry.registerModules([
    RowSelectionModule,
    ClientSideRowModelModule,
    ColumnsToolPanelModule,
    ColumnMenuModule,
    ContextMenuModule,
    ...(process.env.NODE_ENV !== "production" ? [ValidationModule] : []),
]);

let gridApi: GridApi;

const gridOptions = {
    columnDefs: [
        { field: 'athlete', minWidth: 150 },
        { field: 'age', maxWidth: 90 },
        { field: 'country', minWidth: 150 },
        { field: 'year', maxWidth: 90 },
        { field: 'date', minWidth: 150 },
        { field: 'sport', minWidth: 150 },
        { field: 'gold' },
        { field: 'silver' },
        { field: 'bronze' },
        { field: 'total' }
    ],
    
    defaultColDef: {
        flex: 1,
        minWidth: 100
    },
    
    // Finder-like selection
    rowSelection: {
        mode: 'multiRow',
        enableClickSelection: true,
        checkboxes: false,           // Clean UI without checkboxes
        headerCheckbox: false
    },
    
    // Selection event handlers
    onSelectionChanged: (event) => {
        const selectedRows = event.api.getSelectedRows();
        updateSelectionCount(selectedRows.length);
        updateActionButtons(selectedRows.length > 0);
    },
    
    onRowClicked: (event) => {
        const isModifierPressed = event.event.metaKey || event.event.ctrlKey;
        console.log('Row clicked:', {
            athlete: event.data.athlete,
            wasSelected: event.node.isSelected(),
            modifierPressed: isModifierPressed
        });
    }
};

// Helper functions
function updateSelectionCount(count) {
    const countElement = document.querySelector('#selection-count');
    if (countElement) {
        countElement.textContent = `${count} row(s) selected`;
    }
}

function updateActionButtons(hasSelection) {
    const buttons = document.querySelectorAll('.action-button');
    buttons.forEach(button => {
        button.disabled = !hasSelection;
    });
}

// Initialize grid
const gridDiv = document.querySelector('#myGrid');
gridApi = createGrid(gridDiv, gridOptions);

// Load data
fetch('https://www.ag-grid.com/example-assets/olympic-winners.json')
    .then(response => response.json())
    .then(data => gridApi.setGridOption('rowData', data));

// Action buttons
document.querySelector('#delete-selected').addEventListener('click', () => {
    const selectedRows = gridApi.getSelectedRows();
    console.log('Deleting:', selectedRows);
    // Implement delete logic
});

document.querySelector('#export-selected').addEventListener('click', () => {
    const selectedRows = gridApi.getSelectedRows();
    console.log('Exporting:', selectedRows);
    // Implement export logic
});
```

## Selection Modes Comparison

### Single Row Mode

```javascript
rowSelection: {
    mode: 'singleRow',
    enableClickSelection: true
}
```

**Behavior:**
- Only one row can be selected at a time
- Click: Selects row (deselects previous)
- Cmd+Click: Same as click (only one row selectable)

### Multi Row Mode (Finder-Like)

```javascript
rowSelection: {
    mode: 'multiRow',
    enableClickSelection: true
}
```

**Behavior:**
- Multiple rows can be selected
- Click: Selects row (clears others)
- Cmd+Click: Adds/removes from selection

## Styling Selected Rows

### Custom CSS

```css
/* Selected row styling */
.ag-row-selected {
    background-color: #e3f2fd !important;
}

/* Hover effect */
.ag-row:hover {
    background-color: #f5f5f5;
}

/* Selected row hover */
.ag-row-selected:hover {
    background-color: #bbdefb !important;
}
```

### Row Class Rules

```javascript
const gridOptions = {
    getRowClass: (params) => {
        if (params.node.isSelected()) {
            return 'row-selected';
        }
        return '';
    }
};
```

## Common Patterns

### Select All Button

```javascript
function selectAll() {
    gridApi.selectAll();
}

function deselectAll() {
    gridApi.deselectAll();
}
```

### Select Filtered Rows

```javascript
function selectFiltered() {
    gridApi.selectAll({ mode: 'filtered' });
}
```

### Select by Condition

```javascript
function selectUSA() {
    gridApi.forEachNode((node) => {
        if (node.data.country === 'USA') {
            node.setSelected(true);
        }
    });
}
```

### Get Selection Summary

```javascript
function getSelectionSummary() {
    const selectedRows = gridApi.getSelectedRows();
    
    return {
        count: selectedRows.length,
        countries: [...new Set(selectedRows.map(r => r.country))],
        totalGold: selectedRows.reduce((sum, r) => sum + (r.gold || 0), 0)
    };
}
```

## Best Practices

1. **Use `enableClickSelection: true`** for Finder-like behavior
2. **Hide checkboxes** (`checkboxes: false`) for cleaner UI if not needed
3. **Handle `onSelectionChanged`** to update UI based on selection
4. **Use `getSelectedRows()`** to get row data (not nodes)
5. **Use `getSelectedNodes()`** when you need node properties
6. **Consider touch devices** with `enableSelectionWithoutKeys: true`
7. **Provide visual feedback** with CSS for selected rows
8. **Handle empty selection** in action buttons/handlers

## Troubleshooting

### Issue: Click doesn't select rows

**Solution:**
- Ensure `enableClickSelection: true` is set
- Check that `mode: 'multiRow'` or `mode: 'singleRow'` is set
- Verify `RowSelectionModule` is registered

### Issue: Cmd+Click doesn't work

**Solution:**
- Ensure `mode: 'multiRow'` (not `singleRow`)
- Check that `enableClickSelection: true` is set
- Verify browser supports metaKey/ctrlKey detection

### Issue: Selection clears on every click

**Solution:**
- Use Cmd+Click / Ctrl+Click to add to selection
- Regular click clears others (this is expected behavior)
- Consider `enableSelectionWithoutKeys: true` for touch devices

### Issue: Can't deselect rows

**Solution:**
- Use Cmd+Click / Ctrl+Click on selected row to deselect
- Or set `enableClickSelection: 'enableDeselection'`
- Or use checkboxes for deselection

## References

- [AG Grid Row Selection Documentation](https://www.ag-grid.com/javascript-data-grid/row-selection-multi-row/)
- [AG Grid Selection API](https://www.ag-grid.com/javascript-data-grid/row-selection-api-reference/)
- [AG Grid Events](https://www.ag-grid.com/javascript-data-grid/grid-events/)

