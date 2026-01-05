# AG Grid JavaScript: Cell Editing API - Complete Guide

This document explains the complete API and architecture for cell editing in AG Grid JavaScript, including the distinction between individual cell editing and full row editing, and the complete lifecycle from clicking a cell to submitting the new value.

## Overview

AG Grid provides two primary editing modes:

1. **Cell Editing** (default): Edit one cell at a time
2. **Full Row Editing**: Edit all editable cells in a row simultaneously

Both modes share similar APIs but have different behaviors and event sequences.

## Architecture

### Editing Modes

```
Grid Configuration
    ├── Cell Editing (default)
    │   ├── Single cell editable at a time
    │   ├── Events: cellEditingStarted, cellEditingStopped, cellValueChanged
    │   └── API: startEditingCell(), stopEditing()
    │
    └── Full Row Editing (editType: 'fullRow')
        ├── All editable cells in row editable simultaneously
        ├── Events: rowEditingStarted, rowEditingStopped, rowValueChanged
        └── API: startEditingCell() (affects entire row)
```

## Enabling Editing

### 1. **Cell-Level Editing Configuration**

#### Enable Editing for Specific Columns

```javascript
const gridOptions = {
	columnDefs: [
		{
			field: 'athlete',
			editable: true // This column is editable
		},
		{
			field: 'age',
			editable: false // This column is NOT editable
		}
	]
};
```

#### Enable Editing for All Columns (Default)

```javascript
const gridOptions = {
	columnDefs: [{ field: 'athlete' }, { field: 'age' }, { field: 'country' }],
	defaultColDef: {
		editable: true // All columns editable by default
	}
};
```

#### Conditional Editing (Per Row)

```javascript
const gridOptions = {
	columnDefs: [
		{
			field: 'athlete',
			// Enable editing only for rows where year == 2012
			editable: (params) => params.data.year == 2012
		}
	]
};
```

### 2. **Full Row Editing Configuration**

```javascript
const gridOptions = {
	columnDefs: [
		{ field: 'make', editable: true },
		{ field: 'model', editable: true },
		{ field: 'price', editable: true },
		{ field: 'readOnly', editable: false } // Excluded from row editing
	],
	defaultColDef: {
		editable: true
	},
	editType: 'fullRow' // Enable full row editing mode
};
```

**Key Differences:**

- **Cell Editing**: Only one cell is editable at a time
- **Full Row Editing**: All editable cells in the row become editable simultaneously
- **Read-only columns**: Columns with `editable: false` are excluded from row editing

## Complete Cell Editing Lifecycle

### Phase 1: Cell Click Detection

#### User Interactions That Trigger Editing

1. **Mouse Double-Click** (default)

   ```javascript
   // Default behavior: double-click to start editing
   // No configuration needed
   ```

2. **Mouse Single-Click** (optional)

   ```javascript
   const gridOptions = {
   	singleClickEdit: true // Enable single-click editing
   };
   ```

3. **Disable Click Editing**

   ```javascript
   const gridOptions = {
   	suppressClickEdit: true // Disable both single and double-click editing
   	// Use custom cell renderer with button to start editing programmatically
   };
   ```

4. **Keyboard Triggers**
   - **Enter** or **F2**: Start editing
   - **Backspace**: Start editing and clear cell (Windows default, Mac requires `enableCellEditingOnBackspace: true`)
   - **Printable characters** (a-z, A-Z, 0-9, etc.): Start editing with that character

5. **Programmatic Start**
   ```javascript
   gridApi.startEditingCell({
   	rowIndex: 0,
   	colKey: 'athlete'
   });
   ```

### Phase 2: Editing Start

#### Event: `cellEditingStarted`

Fired when editing begins for a cell.

```javascript
const gridOptions = {
	onCellEditingStarted: (event) => {
		console.log('Editing started for cell:', {
			rowIndex: event.rowIndex,
			column: event.column.getId(),
			value: event.value,
			data: event.data
		});
	}
};
```

**Event Properties:**

- `rowIndex`: Index of the row being edited
- `column`: Column definition object
- `value`: Current cell value
- `data`: Row data object
- `node`: Row node object
- `api`: Grid API
- `columnApi`: Column API

#### API: `startEditingCell()`

Programmatically start editing a specific cell.

```javascript
// Basic usage
gridApi.startEditingCell({
	rowIndex: 0,
	colKey: 'athlete'
});

// With optional parameters
gridApi.startEditingCell({
	rowIndex: 0,
	colKey: 'athlete',
	rowPinned: 'top', // For pinned rows: 'top', 'bottom', or undefined
	key: 'Enter' // Simulate key press
});
```

**Parameters:**

- `rowIndex` (number, required): Zero-based row index
- `colKey` (string, required): Column field or ID
- `rowPinned` (string, optional): 'top', 'bottom', or undefined
- `key` (string, optional): Key to simulate ('Enter', 'F2', etc.)

### Phase 3: Editing in Progress

#### Cell Editor Components

AG Grid provides several built-in editors:

1. **Text Editor** (default for strings)

   ```javascript
   {
       field: 'name',
       cellEditor: 'agTextCellEditor'
   }
   ```

2. **Number Editor** (default for numbers)

   ```javascript
   {
       field: 'age',
       cellEditor: 'agNumberCellEditor'
   }
   ```

3. **Select Editor**

   ```javascript
   {
       field: 'country',
       cellEditor: 'agSelectCellEditor',
       cellEditorParams: {
           values: ['USA', 'UK', 'Canada']
       }
   }
   ```

4. **Date Editor**

   ```javascript
   {
       field: 'date',
       cellEditor: 'agDateCellEditor',
       cellEditorParams: {
           min: '2020-01-01',
           max: '2025-12-31'
       }
   }
   ```

5. **Custom Editor**
   ```javascript
   {
       field: 'custom',
       cellEditor: MyCustomEditor,
       cellEditorParams: {
           // Custom parameters
       }
   }
   ```

#### Popup Editors

Editors can appear in popups instead of inline:

```javascript
{
    field: 'year',
    cellEditor: YearCellEditor,
    cellEditorPopup: true,  // Show as popup
    cellEditorPopupPosition: 'over'  // or 'under'
}
```

### Phase 4: Editing Stop Triggers

Editing stops when any of the following occur:

#### 1. **Editor Callback: `stopEditing()`**

The cell editor calls `params.stopEditing()` to commit changes:

```javascript
class MyCellEditor {
	init(params) {
		this.params = params;
		// ... setup editor
	}

	getValue() {
		return this.newValue;
	}

	// Called when user clicks "Save" button
	onSave() {
		this.params.stopEditing(); // Commit changes
	}

	// Called when user clicks "Cancel" button
	onCancel() {
		this.params.stopEditing(true); // Cancel changes (discard)
	}
}
```

#### 2. **Focus Change**

Moving focus to another cell stops editing and commits changes.

#### 3. **Enter Key**

Pressing Enter stops editing and commits changes.

**Excel-style behavior** (move to next row):

```javascript
const gridOptions = {
	enterNavigatesVertically: true,
	enterNavigatesVerticallyAfterEdit: true
};
```

#### 4. **Escape Key**

Pressing Escape stops editing and **discards** changes.

#### 5. **Tab Key**

Pressing Tab stops editing, commits changes, and moves to next/previous cell.

**Suppress auto-start on Tab:**

```javascript
const gridOptions = {
	suppressStartEditOnTab: true // Tab navigates but doesn't start editing
};
```

#### 6. **Popup Editor Closed**

For popup editors, clicking outside closes the popup and stops editing.

#### 7. **Grid Loses Focus**

```javascript
const gridOptions = {
	stopEditingWhenCellsLoseFocus: true // Stop editing when grid loses focus
};
```

#### 8. **API Call: `stopEditing()`**

```javascript
gridApi.stopEditing(); // Stop all editing, commit changes
```

### Phase 5: Editing Stop Event

#### Event: `cellEditingStopped`

Fired when editing stops for a cell.

```javascript
const gridOptions = {
	onCellEditingStopped: (event) => {
		console.log('Editing stopped for cell:', {
			rowIndex: event.rowIndex,
			column: event.column.getId(),
			value: event.value,
			oldValue: event.oldValue
		});
	}
};
```

**Event Properties:**

- `rowIndex`: Index of the row
- `column`: Column definition
- `value`: New cell value (if committed)
- `oldValue`: Previous cell value
- `data`: Row data object
- `node`: Row node object

### Phase 6: Value Submission

#### Event: `cellValueChanged`

Fired when a cell's value is successfully changed.

```javascript
const gridOptions = {
	onCellValueChanged: (event) => {
		console.log('Cell value changed:', {
			field: event.colDef.field,
			oldValue: event.oldValue,
			newValue: event.newValue,
			data: event.data,
			rowIndex: event.rowIndex
		});

		// Example: Trigger filtering after edit
		event.api.onFilterChanged();

		// Example: Save to backend
		saveToBackend(event.data);
	}
};
```

**Event Properties:**

- `colDef`: Column definition
- `field`: Field name
- `oldValue`: Previous value
- `newValue`: New value
- `data`: Updated row data
- `node`: Row node
- `rowIndex`: Row index
- `api`: Grid API

**Important Notes:**

- Only fires if the new value is **different** from the old value
- Does **not** fire if editing was cancelled (Escape pressed)
- Does **not** fire if `readOnlyEdit: true` (use `onCellEditRequest` instead)
- Fires **after** `cellEditingStopped`

#### Value Processing Pipeline

```
User Input
    ↓
Cell Editor.getValue()
    ↓
valueParser (if defined)
    ↓
valueSetter (if defined)
    ↓
Data Updated
    ↓
cellValueChanged Event
```

**Value Parser:**

```javascript
{
    field: 'price',
    valueParser: (params) => {
        // Transform value before saving
        return parseFloat(params.newValue);
    }
}
```

**Value Setter:**

```javascript
{
    field: 'total',
    valueSetter: (params) => {
        // Custom logic for setting value
        params.data.total = params.newValue;
        params.data.calculated = true;
        return true;  // Return true to accept, false to reject
    }
}
```

## Full Row Editing Lifecycle

### Configuration

```javascript
const gridOptions = {
	columnDefs: [
		{ field: 'make', editable: true },
		{ field: 'model', editable: true },
		{ field: 'price', editable: true },
		{ field: 'readOnly', editable: false } // Excluded
	],
	defaultColDef: {
		editable: true
	},
	editType: 'fullRow' // Enable full row editing
};
```

### Row Editing Events

#### 1. **`rowEditingStarted`**

Fired when row editing begins.

```javascript
const gridOptions = {
	onRowEditingStarted: (event) => {
		console.log('Row editing started:', {
			rowIndex: event.rowIndex,
			data: event.data,
			node: event.node
		});
	}
};
```

#### 2. **`rowEditingStopped`**

Fired when row editing ends.

```javascript
const gridOptions = {
	onRowEditingStopped: (event) => {
		console.log('Row editing stopped:', {
			rowIndex: event.rowIndex,
			data: event.data
		});
	}
};
```

#### 3. **`rowValueChanged`**

Fired when any cell value in the row changes (after commit).

```javascript
const gridOptions = {
	onRowValueChanged: (event) => {
		console.log('Row value changed:', {
			rowIndex: event.rowIndex,
			data: event.data, // Complete updated row data
			node: event.node
		});
	}
};
```

**Note:** In full row editing mode:

- `cellValueChanged` still fires for individual cells
- `rowValueChanged` fires when the row editing session ends
- All editable cells in the row are editable simultaneously

### Row Editing Validation

```javascript
const gridOptions = {
	editType: 'fullRow',
	getFullRowEditValidationErrors: (params) => {
		const errors = [];

		// Validate price
		if (params.data.price < 0) {
			errors.push({
				colId: 'price',
				message: 'Price cannot be negative'
			});
		}

		// Validate model
		if (!params.data.model || params.data.model.length < 2) {
			errors.push({
				colId: 'model',
				message: 'Model must be at least 2 characters'
			});
		}

		return errors;
	}
};
```

## Complete API Reference

### Grid API Methods

#### `startEditingCell(params)`

Start editing a specific cell.

```javascript
gridApi.startEditingCell({
	rowIndex: 0, // Required: zero-based row index
	colKey: 'athlete', // Required: column field or ID
	rowPinned: undefined, // Optional: 'top', 'bottom', or undefined
	key: 'Enter' // Optional: simulate key press
});
```

#### `stopEditing(cancel = false)`

Stop all editing.

```javascript
gridApi.stopEditing(); // Commit changes
gridApi.stopEditing(true); // Cancel changes (discard)
```

#### `getEditingCells()`

Get all currently editing cells.

```javascript
const editingCells = gridApi.getEditingCells();

editingCells.forEach((cell) => {
	console.log('Editing cell:', {
		rowIndex: cell.rowIndex,
		columnId: cell.column.getId(),
		floating: cell.floating // 'top', 'bottom', or undefined
	});
});
```

#### `tabToNextCell()` / `tabToPreviousCell()`

Navigate to next/previous editable cell.

```javascript
gridApi.tabToNextCell(); // Move to next editable cell
gridApi.tabToPreviousCell(); // Move to previous editable cell
```

### Column Definition Properties

#### Editing Configuration

```javascript
{
    field: 'athlete',

    // Enable/disable editing
    editable: true,  // or function: (params) => boolean

    // Single-click editing (column-specific)
    singleClickEdit: true,

    // Cell editor
    cellEditor: 'agTextCellEditor',  // or custom component
    cellEditorParams: {
        // Editor-specific parameters
    },

    // Popup editor
    cellEditorPopup: true,
    cellEditorPopupPosition: 'over',  // or 'under'

    // Value processing
    valueParser: (params) => {
        // Transform value before saving
        return params.newValue;
    },

    valueSetter: (params) => {
        // Custom setter logic
        params.data[params.colDef.field] = params.newValue;
        return true;  // Return true to accept
    }
}
```

### Grid Options for Editing

```javascript
const gridOptions = {
	// Editing mode
	editType: undefined, // or 'fullRow' for row editing

	// Click behavior
	singleClickEdit: false, // Single-click to edit (global)
	suppressClickEdit: false, // Disable click editing

	// Keyboard behavior
	enableCellEditingOnBackspace: false, // Enable on Mac
	suppressStartEditOnTab: false, // Don't start editing on Tab
	enterNavigatesVertically: false, // Excel-style Enter behavior
	enterNavigatesVerticallyAfterEdit: false,

	// Focus behavior
	stopEditingWhenCellsLoseFocus: false, // Stop on focus loss

	// Read-only edit mode
	readOnlyEdit: false, // Grid doesn't update data, fires cellEditRequest

	// Validation
	invalidEditValueMode: 'block', // Block invalid edits

	// Undo/Redo
	undoRedoCellEditing: false, // Enable undo/redo
	undoRedoCellEditingLimit: 10 // Undo stack size
};
```

## Event Sequence: Complete Flow

### Cell Editing Flow

```
1. User clicks cell (double-click by default)
   ↓
2. onCellClicked (if registered)
   ↓
3. cellEditingStarted event
   ↓
4. Cell editor initialized
   ↓
5. User edits value
   ↓
6. User presses Enter/Tab/Escape or clicks away
   ↓
7. cellEditingStopped event
   ↓
8. Value processed (valueParser → valueSetter)
   ↓
9. Data updated
   ↓
10. cellValueChanged event
```

### Full Row Editing Flow

```
1. User clicks cell in row (double-click by default)
   ↓
2. onCellClicked (if registered)
   ↓
3. rowEditingStarted event
   ↓
4. cellEditingStarted event (for each editable cell)
   ↓
5. All editable cells in row become editable
   ↓
6. User edits multiple cells
   ↓
7. User presses Enter/Tab/Escape or clicks away
   ↓
8. Validation (getFullRowEditValidationErrors)
   ↓
9. cellEditingStopped event (for each cell)
   ↓
10. rowEditingStopped event
   ↓
11. Values processed and data updated
   ↓
12. cellValueChanged event (for each changed cell)
   ↓
13. rowValueChanged event
```

## Read-Only Edit Mode

When `readOnlyEdit: true`, the grid doesn't update data automatically. Instead, it fires `cellEditRequest` events:

```javascript
const gridOptions = {
	readOnlyEdit: true,
	onCellEditRequest: (event) => {
		const { data, colDef, newValue } = event;

		// Update immutable store
		const updatedData = {
			...data,
			[colDef.field]: newValue
		};

		// Update grid with new data
		gridApi.setGridOption('rowData', updatedRowData);
	}
};
```

**Event Properties:**

- `data`: Row data object
- `colDef`: Column definition
- `field`: Field name
- `newValue`: New value to be set
- `oldValue`: Previous value
- `rowIndex`: Row index
- `node`: Row node

## Example: Complete Cell Editing Setup

```javascript
import { createGrid, GridApi, GridOptions } from 'ag-grid-community';

let gridApi: GridApi;

const gridOptions: GridOptions = {
    columnDefs: [
        {
            field: 'athlete',
            editable: true,
            singleClickEdit: true  // Single-click to edit this column
        },
        {
            field: 'age',
            editable: true,
            cellEditor: 'agNumberCellEditor',
            valueParser: (params) => parseInt(params.newValue)
        },
        {
            field: 'country',
            editable: true,
            cellEditor: 'agSelectCellEditor',
            cellEditorParams: {
                values: ['USA', 'UK', 'Canada', 'Australia']
            }
        },
        {
            field: 'year',
            editable: (params) => params.data.year >= 2010  // Conditional
        }
    ],

    defaultColDef: {
        editable: false  // Most columns not editable by default
    },

    // Cell editing mode (default)
    // editType: undefined,

    // Event handlers
    onCellClicked: (event) => {
        console.log('Cell clicked:', event.colDef.field);
    },

    onCellEditingStarted: (event) => {
        console.log('Editing started:', event.colDef.field);
    },

    onCellEditingStopped: (event) => {
        console.log('Editing stopped:', event.colDef.field);
    },

    onCellValueChanged: (event) => {
        console.log('Value changed:', {
            field: event.colDef.field,
            oldValue: event.oldValue,
            newValue: event.newValue
        });

        // Save to backend
        saveToBackend(event.data);
    }
};

// Initialize grid
const gridDiv = document.querySelector('#myGrid');
gridApi = createGrid(gridDiv, gridOptions);

// Programmatic editing
function editCell(rowIndex, colKey) {
    gridApi.startEditingCell({
        rowIndex: rowIndex,
        colKey: colKey
    });
}

// Stop editing
function stopEditing() {
    gridApi.stopEditing();
}

// Get editing cells
function getEditingCells() {
    const cells = gridApi.getEditingCells();
    console.log('Currently editing:', cells);
    return cells;
}
```

## Example: Full Row Editing Setup

```javascript
const gridOptions: GridOptions = {
    columnDefs: [
        { field: 'make', editable: true },
        { field: 'model', editable: true },
        { field: 'price', editable: true },
        { field: 'readOnly', editable: false }
    ],

    defaultColDef: {
        editable: true
    },

    editType: 'fullRow',  // Enable full row editing

    // Row editing events
    onRowEditingStarted: (event) => {
        console.log('Row editing started');
    },

    onRowEditingStopped: (event) => {
        console.log('Row editing stopped');
    },

    // Individual cell events still fire
    onCellValueChanged: (event) => {
        console.log('Cell changed:', event.colDef.field);
    },

    // Row-level event
    onRowValueChanged: (event) => {
        console.log('Row value changed:', event.data);
        saveToBackend(event.data);
    },

    // Row validation
    getFullRowEditValidationErrors: (params) => {
        const errors = [];
        if (params.data.price < 0) {
            errors.push({
                colId: 'price',
                message: 'Price must be positive'
            });
        }
        return errors;
    }
};
```

## Key Differences: Cell vs Row Editing

| Aspect             | Cell Editing                  | Full Row Editing                           |
| ------------------ | ----------------------------- | ------------------------------------------ |
| **Configuration**  | `editable: true` (per column) | `editType: 'fullRow'`                      |
| **Editable Cells** | One at a time                 | All editable cells in row                  |
| **Start Event**    | `cellEditingStarted`          | `rowEditingStarted` + `cellEditingStarted` |
| **Stop Event**     | `cellEditingStopped`          | `cellEditingStopped` + `rowEditingStopped` |
| **Value Changed**  | `cellValueChanged`            | `cellValueChanged` + `rowValueChanged`     |
| **Validation**     | Per-cell (valueSetter)        | Per-row (`getFullRowEditValidationErrors`) |
| **Navigation**     | Tab moves to next cell        | Tab moves to next row                      |
| **Use Case**       | Excel-like editing            | Form-like editing                          |

## Best Practices

1. **Use Cell Editing for**: Large datasets, Excel-like workflows, quick edits
2. **Use Row Editing for**: Form-like interfaces, complex validation, related fields
3. **Always validate**: Use `valueParser` and `valueSetter` for data transformation
4. **Handle errors**: Use `invalidEditValueMode: 'block'` to prevent invalid data
5. **Save efficiently**: Batch saves in `rowValueChanged` for row editing
6. **Provide feedback**: Use events to show loading states during saves
7. **Consider UX**: Use `singleClickEdit` sparingly to avoid accidental edits

## References

- [AG Grid Cell Editing Documentation](https://www.ag-grid.com/javascript-data-grid/cell-editing/)
- [AG Grid Full Row Editing](https://www.ag-grid.com/javascript-data-grid/cell-editing-full-row/)
- [AG Grid Editing API](https://www.ag-grid.com/javascript-data-grid/cell-editing-start-stop/)
- [AG Grid Events](https://www.ag-grid.com/javascript-data-grid/grid-events/)
