# AG Grid JavaScript: Clipboard Operations - Cut and Paste Guide

This document explains how to use AG Grid's clipboard functionality to cut and paste values, removing existing values and overriding them with new data.

## Overview

AG Grid provides comprehensive clipboard support for:
- **Cut** (Ctrl+X / Cmd+X): Removes values from selected cells and copies them to clipboard
- **Paste** (Ctrl+V / Cmd+V): Overrides existing cell values with data from clipboard
- **Copy** (Ctrl+C / Cmd+C): Copies selected cells to clipboard without removing them

## Prerequisites

### Required Modules

Clipboard functionality requires the **Enterprise** version of AG Grid with the `ClipboardModule`:

```javascript
import { ClipboardModule } from 'ag-grid-enterprise';
import { ModuleRegistry } from 'ag-grid-community';

ModuleRegistry.registerModules([
    ClipboardModule,
    // ... other modules
]);
```

### Enable Cell Selection

For clipboard operations to work with ranges, you must enable cell selection:

```javascript
const gridOptions = {
    cellSelection: true,  // Required for clipboard operations
    // ... other options
};
```

## Cut Operations

### How Cut Works

When you cut cells (Ctrl+X / Cmd+X):
1. Selected cell values are **copied** to the clipboard
2. Selected cell values are **removed** (cleared) from the grid
3. The grid data is updated immediately

### Basic Cut Configuration

```javascript
const gridOptions = {
    cellSelection: true,
    defaultColDef: {
        editable: true  // Cells must be editable for cut to work
    },
    // Cut is enabled by default
};
```

### Disable Cut Operations

```javascript
const gridOptions = {
    suppressCutToClipboard: true,  // Disables cut functionality
    // ... other options
};
```

### Cut Events

#### `onCutStart`

Fired when a cut operation begins:

```javascript
const gridOptions = {
    onCutStart: (event) => {
        console.log('Cut started:', {
            cells: event.cells,  // Array of cells being cut
            ranges: event.ranges // Cell ranges being cut
        });
    }
};
```

**Event Properties:**
- `cells`: Array of cell objects being cut
- `ranges`: Array of cell range objects
- `api`: Grid API reference

#### `onCutEnd`

Fired when a cut operation completes:

```javascript
const gridOptions = {
    onCutEnd: (event) => {
        console.log('Cut completed:', {
            cells: event.cells,  // Cells that were cut
            ranges: event.ranges
        });
        
        // Values have been removed from the grid at this point
    }
};
```

**Event Properties:**
- `cells`: Array of cell objects that were cut
- `ranges`: Array of cell range objects
- `api`: Grid API reference

### Cut Example

```javascript
import { ClipboardModule } from 'ag-grid-enterprise';
import { ModuleRegistry, createGrid, GridOptions } from 'ag-grid-community';

ModuleRegistry.registerModules([ClipboardModule]);

const gridOptions = {
    columnDefs: [
        { field: 'athlete', editable: true },
        { field: 'age', editable: true },
        { field: 'country', editable: true }
    ],
    cellSelection: true,
    
    onCutStart: (event) => {
        console.log('Cutting cells:', event.cells.length);
    },
    
    onCutEnd: (event) => {
        console.log('Cut complete. Cells removed:', event.cells.length);
        // Grid data has been updated - values are now empty/null
    }
};

// User selects cells and presses Ctrl+X
// 1. onCutStart fires
// 2. Values copied to clipboard
// 3. Values removed from grid
// 4. onCutEnd fires
// 5. cellValueChanged events fire for each cleared cell
```

## Paste Operations

### How Paste Works

When you paste data (Ctrl+V / Cmd+V):
1. Data is read from the clipboard
2. Data is **overridden** into cells starting from the focused cell
3. Existing values are **replaced** with pasted values
4. The grid data is updated immediately

### Basic Paste Configuration

```javascript
const gridOptions = {
    cellSelection: true,
    defaultColDef: {
        editable: true  // Cells must be editable for paste to work
    },
    // Paste is enabled by default in Enterprise version
};
```

### Disable Paste Operations

```javascript
const gridOptions = {
    suppressClipboardPaste: true,  // Disables paste functionality
    // ... other options
};
```

### Paste Events

#### `onPasteStart`

Fired when a paste operation begins:

```javascript
const gridOptions = {
    onPasteStart: (event) => {
        console.log('Paste started:', {
            data: event.data,  // 2D array of data being pasted
            ranges: event.ranges // Target ranges
        });
    }
};
```

**Event Properties:**
- `data`: 2D array of strings representing the data to be pasted
- `ranges`: Array of cell range objects where data will be pasted
- `api`: Grid API reference

#### `onPasteEnd`

Fired when a paste operation completes:

```javascript
const gridOptions = {
    onPasteEnd: (event) => {
        console.log('Paste completed:', {
            data: event.data,  // Data that was pasted
            ranges: event.ranges
        });
        
        // Values have been updated in the grid at this point
    }
};
```

**Event Properties:**
- `data`: 2D array of strings that were pasted
- `ranges`: Array of cell range objects where data was pasted
- `api`: Grid API reference

### Paste Behavior

#### Single Cell Paste

When pasting into a single cell:
- Only that cell's value is overridden
- The cell must be editable

```javascript
// User focuses on cell at row 0, column 'athlete'
// Pastes "New Value"
// Result: Only that cell is updated
```

#### Range Paste

When pasting a range of data:
- Data is pasted starting from the focused cell
- Data flows right and down from the starting position
- Existing values are overridden

```javascript
// Clipboard contains:
// "Value1"  "Value2"
// "Value3"  "Value4"

// User focuses on cell at row 0, column 'athlete'
// Pastes the data
// Result:
// Row 0: athlete="Value1", age="Value2"
// Row 1: athlete="Value3", age="Value4"
```

#### Paste Beyond Grid Boundaries

By default, paste operations are limited to existing rows. To allow pasting that creates new rows, use `processDataFromClipboard`:

```javascript
function processDataFromClipboard(params) {
    const data = [...params.data];
    const focusedCell = params.api.getFocusedCell();
    const focusedIndex = focusedCell.rowIndex;
    const lastIndex = params.api.getDisplayedRowCount() - 1;
    
    // Check if paste would exceed grid
    if (focusedIndex + data.length - 1 > lastIndex) {
        const numRowsToAdd = (focusedIndex + data.length - 1) - lastIndex;
        
        // Create new rows
        const rowsToAdd = [];
        for (let i = 0; i < numRowsToAdd; i++) {
            const row = data[data.length - numRowsToAdd + i];
            const rowObject = {};
            let currentColumn = focusedCell.column;
            
            row.forEach((item) => {
                if (currentColumn) {
                    rowObject[currentColumn.colDef.field] = item;
                    currentColumn = params.api.getDisplayedColAfter(currentColumn);
                }
            });
            
            rowsToAdd.push(rowObject);
        }
        
        // Add new rows to grid
        params.api.applyTransaction({ add: rowsToAdd });
    }
    
    return data;
}

const gridOptions = {
    processDataFromClipboard: processDataFromClipboard
};
```

### Paste Example

```javascript
const gridOptions = {
    columnDefs: [
        { field: 'athlete', editable: true },
        { field: 'age', editable: true },
        { field: 'country', editable: true }
    ],
    cellSelection: true,
    
    onPasteStart: (event) => {
        console.log('Pasting data:', event.data);
        // event.data is a 2D array: [["Value1", "Value2"], ["Value3", "Value4"]]
    },
    
    onPasteEnd: (event) => {
        console.log('Paste complete. Data overridden:', event.data);
        // Grid data has been updated with new values
    },
    
    onCellValueChanged: (event) => {
        console.log('Cell value changed:', {
            field: event.colDef.field,
            oldValue: event.oldValue,
            newValue: event.newValue
        });
        // This fires for each cell that was overridden
    }
};

// User focuses on cell at row 0, column 'athlete'
// Pastes "New Athlete" from clipboard
// 1. onPasteStart fires
// 2. Cell value is overridden
// 3. onCellValueChanged fires
// 4. onPasteEnd fires
```

## Clipboard Data Processing

### Process Data Before Paste

Use `processDataFromClipboard` to transform or validate data before it's pasted:

```javascript
function processDataFromClipboard(params) {
    const data = params.data;  // 2D array of strings
    
    // Example: Block paste if data contains "RED"
    for (let i = 0; i < data.length; i++) {
        for (let j = 0; j < data[i].length; j++) {
            if (data[i][j] && data[i][j].includes('RED')) {
                // Cancel the paste
                return null;
            }
        }
    }
    
    // Example: Replace paste with custom data
    if (data[0][0] === 'SPECIAL') {
        return [
            ['Custom 1', 'Custom 2'],
            ['Custom 3', 'Custom 4']
        ];
    }
    
    // Return data as-is (or modified)
    return data;
}

const gridOptions = {
    processDataFromClipboard: processDataFromClipboard
};
```

**Return Values:**
- `string[][]`: Modified data to paste
- `null`: Cancel the paste operation

### Process Individual Cells After Paste

Use `processCellFromClipboard` to transform individual cell values:

```javascript
function processCellFromClipboard(params) {
    // params.value is the raw string from clipboard
    // params.column is the column definition
    // params.node is the row node
    // params.data is the row data
    
    // Example: Validate numeric columns
    if (params.column.colDef.field === 'age') {
        const numValue = parseInt(params.value);
        if (isNaN(numValue)) {
            return null;  // Block invalid paste
        }
        return numValue.toString();
    }
    
    // Example: Transform text
    if (params.column.colDef.field === 'athlete') {
        return params.value.toUpperCase();
    }
    
    // Return value as-is
    return params.value;
}

const gridOptions = {
    processCellFromClipboard: processCellFromClipboard
};
```

**Return Values:**
- `string`: Processed value to insert
- `null`: Block this cell from being updated

### Process Data Before Copy/Cut

Use `processCellForClipboard` to transform data before it's copied:

```javascript
function processCellForClipboard(params) {
    // Format data for external applications (e.g., Excel)
    if (params.value instanceof Date) {
        return params.value.toISOString();
    }
    
    // Add prefix to copied data
    return 'C-' + params.value;
}

const gridOptions = {
    processCellForClipboard: processCellForClipboard
};
```

## Complete Cut and Paste Example

```javascript
import {
    ClipboardModule,
    CellSelectionModule
} from 'ag-grid-enterprise';
import {
    ModuleRegistry,
    createGrid,
    GridOptions,
    GridApi,
    CellValueChangedEvent,
    CutStartEvent,
    CutEndEvent,
    PasteStartEvent,
    PasteEndEvent,
    ProcessDataFromClipboardParams,
    ProcessCellForExportParams
} from 'ag-grid-community';

ModuleRegistry.registerModules([
    ClipboardModule,
    CellSelectionModule
]);

let gridApi: GridApi;

const gridOptions = {
    columnDefs: [
        { field: 'athlete', editable: true },
        { field: 'age', editable: true, valueParser: (params) => parseInt(params.newValue) },
        { field: 'country', editable: true }
    ],
    
    cellSelection: true,
    defaultColDef: {
        editable: true
    },
    
    // Cut event handlers
    onCutStart: (event: CutStartEvent) => {
        console.log('Cutting', event.cells.length, 'cells');
    },
    
    onCutEnd: (event: CutEndEvent) => {
        console.log('Cut complete. Values removed from grid');
        // Values are now empty/null in the grid
    },
    
    // Paste event handlers
    onPasteStart: (event: PasteStartEvent) => {
        console.log('Pasting data:', event.data);
    },
    
    onPasteEnd: (event: PasteEndEvent) => {
        console.log('Paste complete. Values overridden in grid');
    },
    
    // Cell value changed (fires for both cut and paste)
    onCellValueChanged: (event: CellValueChangedEvent) => {
        console.log('Cell value changed:', {
            field: event.colDef.field,
            oldValue: event.oldValue,
            newValue: event.newValue,
            source: 'clipboard'  // Indicates clipboard operation
        });
    },
    
    // Process data before paste
    processDataFromClipboard: (params: ProcessDataFromClipboardParams) => {
        // Remove empty last row (Excel quirk)
        const data = [...params.data];
        const lastRow = data[data.length - 1];
        if (lastRow && lastRow.length === 1 && lastRow[0] === '') {
            data.pop();
        }
        
        // Validate or transform data
        return data;
    },
    
    // Process individual cells after paste
    processCellFromClipboard: (params: ProcessCellForExportParams) => {
        // Validate numeric columns
        if (params.column.colDef.field === 'age') {
            const num = parseInt(params.value);
            return isNaN(num) ? null : num.toString();
        }
        
        return params.value;
    }
};

const gridDiv = document.querySelector('#myGrid');
gridApi = createGrid(gridDiv, gridOptions);

// Load data
fetch('data.json')
    .then(response => response.json())
    .then(data => gridApi.setGridOption('rowData', data));
```

## Clipboard Configuration Options

### Grid Options

```javascript
const gridOptions = {
    // Enable/disable clipboard operations
    suppressCutToClipboard: false,      // Disable cut
    suppressClipboardPaste: false,      // Disable paste
    
    // Clipboard formatting
    clipboardDelimiter: '\t',           // Delimiter for clipboard data (default: tab)
    copyHeadersToClipboard: false,      // Include headers when copying
    copyGroupHeadersToClipboard: false, // Include group headers when copying
    
    // Excel compatibility
    suppressLastEmptyLineOnPaste: false, // Remove Excel's extra empty line
    
    // Custom clipboard API
    suppressClipboardApi: false,        // Use fallback instead of Clipboard API
    sendToClipboard: (params) => {       // Custom clipboard handler
        // params.data is 2D array of strings
        // Implement custom clipboard logic
    }
};
```

### Column-Level Paste Control

```javascript
const columnDefs = [
    {
        field: 'athlete',
        editable: true,
        suppressPaste: false  // Allow paste (default)
    },
    {
        field: 'readOnly',
        editable: false,
        suppressPaste: true  // Block paste for this column
    },
    {
        field: 'conditional',
        editable: true,
        suppressPaste: (params) => {
            // Conditionally block paste
            return params.node.data.status === 'locked';
        }
    }
];
```

## Event Sequence: Complete Flow

### Cut Operation Flow

```
1. User selects cells
   ↓
2. User presses Ctrl+X / Cmd+X
   ↓
3. onCutStart event fires
   ↓
4. processCellForClipboard called for each cell
   ↓
5. Data copied to clipboard
   ↓
6. Cell values cleared (set to null/empty)
   ↓
7. cellValueChanged events fire for each cleared cell
   ↓
8. onCutEnd event fires
```

### Paste Operation Flow

```
1. User focuses on target cell
   ↓
2. User presses Ctrl+V / Cmd+V
   ↓
3. Data read from clipboard
   ↓
4. onPasteStart event fires
   ↓
5. processDataFromClipboard called (can modify/cancel)
   ↓
6. For each cell to be pasted:
   a. processCellFromClipboard called (can modify/block)
   b. Cell value overridden
   c. cellValueChanged event fires
   ↓
7. onPasteEnd event fires
```

## Advanced Use Cases

### Read-Only Edit Mode with Clipboard

When using `readOnlyEdit: true`, clipboard operations fire `cellEditRequest` instead of directly updating data:

```javascript
const gridOptions = {
    readOnlyEdit: true,
    cellSelection: true,
    
    onCellEditRequest: (event) => {
        // Handle clipboard updates in immutable store
        const { data, colDef, newValue } = event;
        
        // Update immutable data store
        const updatedData = {
            ...data,
            [colDef.field]: newValue
        };
        
        // Update grid with new data
        gridApi.setGridOption('rowData', updatedRowData);
    }
};
```

### Custom Clipboard Format

```javascript
function sendToClipboard(params) {
    // Custom clipboard format (e.g., JSON)
    const jsonData = JSON.stringify(params.data);
    
    // Use custom clipboard API
    navigator.clipboard.writeText(jsonData).then(() => {
        console.log('Data copied in custom format');
    });
}

const gridOptions = {
    sendToClipboard: sendToClipboard
};
```

### Paste Validation with User Feedback

```javascript
function processDataFromClipboard(params) {
    const data = params.data;
    const errors = [];
    
    // Validate data
    for (let i = 0; i < data.length; i++) {
        for (let j = 0; j < data[i].length; j++) {
            const value = data[i][j];
            const column = params.api.getDisplayedColAfter(
                params.api.getFocusedCell().column, j
            );
            
            // Example: Validate age column
            if (column.colDef.field === 'age') {
                const age = parseInt(value);
                if (isNaN(age) || age < 0 || age > 150) {
                    errors.push(`Invalid age at row ${i + 1}: ${value}`);
                }
            }
        }
    }
    
    if (errors.length > 0) {
        // Show error to user
        alert('Paste validation failed:\n' + errors.join('\n'));
        return null;  // Cancel paste
    }
    
    return data;  // Allow paste
}
```

## Best Practices

1. **Always enable cell selection** for clipboard operations:
   ```javascript
   cellSelection: true
   ```

2. **Make cells editable** for clipboard to work:
   ```javascript
   defaultColDef: { editable: true }
   ```

3. **Validate pasted data** using `processDataFromClipboard` or `processCellFromClipboard`

4. **Handle empty values** - Excel often adds empty rows/columns:
   ```javascript
   function processDataFromClipboard(params) {
       const data = params.data.filter(row => 
           row.some(cell => cell && cell.trim() !== '')
       );
       return data;
   }
   ```

5. **Use events for logging/auditing**:
   ```javascript
   onPasteEnd: (event) => {
       logPasteOperation(event.data, event.ranges);
   }
   ```

6. **Consider read-only edit mode** for immutable data stores:
   ```javascript
   readOnlyEdit: true,
   onCellEditRequest: handleClipboardUpdate
   ```

7. **Handle edge cases**:
   - Paste beyond grid boundaries
   - Paste into non-editable cells
   - Paste invalid data types
   - Large paste operations

## Common Issues and Solutions

### Issue: Paste doesn't work

**Solution:**
- Ensure `cellSelection: true` is set
- Ensure cells are `editable: true`
- Check that `suppressClipboardPaste: false` (default)
- Verify `ClipboardModule` is registered

### Issue: Cut doesn't remove values

**Solution:**
- Ensure cells are `editable: true`
- Check that `suppressCutToClipboard: false` (default)
- Verify `ClipboardModule` is registered

### Issue: Paste overrides wrong cells

**Solution:**
- Check focused cell position before paste
- Verify `processDataFromClipboard` isn't modifying data incorrectly
- Ensure column mapping is correct

### Issue: Excel compatibility issues

**Solution:**
- Use `suppressLastEmptyLineOnPaste: true` to handle Excel's extra line
- Use `clipboardDelimiter: '\t'` (default) for tab-separated values
- Process data in `processDataFromClipboard` to handle Excel formatting

## References

- [AG Grid Clipboard Documentation](https://www.ag-grid.com/javascript-data-grid/clipboard/)
- [AG Grid Cell Selection](https://www.ag-grid.com/javascript-data-grid/cell-selection/)
- [AG Grid Events](https://www.ag-grid.com/javascript-data-grid/grid-events/)

