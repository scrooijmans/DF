# AG Grid: Editable Cells and Row Selection/Highlighting

## A) Editable Grid Cells

### Overview

AG Grid allows you to make cells editable, enabling users to modify data directly within the grid. Cells can be made editable per column, with support for custom editors, validation, and various editing triggers.

### Basic Configuration

#### Enable Editing for a Column

Set the `editable` property to `true` in the column definition:

```javascript
const gridOptions = {
    columnDefs: [
        {
            field: 'athlete',
            editable: true  // Makes this column editable
        },
        {
            field: 'age',
            editable: true
        }
    ]
};
```

#### Enable Editing for All Columns

Set `editable: true` in `defaultColDef`:

```javascript
const gridOptions = {
    columnDefs: [
        { field: "athlete" },
        { field: "age" },
        { field: "country" }
    ],
    defaultColDef: {
        editable: true,  // All columns are now editable
    }
};
```

### Conditional Editing

Use a callback function to conditionally enable editing based on row data:

```javascript
const gridOptions = {
    columnDefs: [
        {
            field: 'athlete',
            // Only editable for rows where year == 2012
            editable: (params) => params.data.year == 2012
        }
    ]
};
```

### Editing Triggers

Editing starts when any of the following occurs:

1. **Edit Keys**: Press `Enter` or `F2`
2. **Backspace**: Press `Backspace` (Windows) or enable with `enableCellEditingOnBackspace=true`
3. **Printable Characters**: Type any letter, number, or symbol
4. **Double Click**: Double-click the cell
5. **Single Click**: Set `singleClickEdit: true` in column definition
6. **Programmatic**: Call `gridApi.startEditingCell()`

```javascript
// Enable single-click editing for a column
{
    field: 'athlete',
    editable: true,
    singleClickEdit: true
}

// Prevent click editing (only keyboard/programmatic)
{
    field: 'athlete',
    editable: true,
    suppressClickEdit: true
}
```

### Cell Editor Types

AG Grid provides several built-in cell editors:

#### Text Editor (Default)
```javascript
{
    field: 'athlete',
    editable: true,
    // Uses default text editor automatically
}
```

#### Number Editor
```javascript
{
    field: 'age',
    editable: true,
    cellEditor: 'agNumberCellEditor',
    cellEditorParams: {
        min: 0,
        max: 100,
        precision: 0
    }
}
```

#### Select/Dropdown Editor
```javascript
{
    field: 'country',
    editable: true,
    cellEditor: 'agSelectCellEditor',
    cellEditorParams: {
        values: ['USA', 'UK', 'Canada', 'Australia']
    }
}
```

#### Popup Editor
```javascript
{
    field: 'description',
    editable: true,
    cellEditor: 'agTextCellEditor',
    cellEditorPopup: true,  // Opens in popup
    cellEditorPopupPosition: 'over'  // or 'under'
}
```

### Custom Cell Editors

#### Class-Based Custom Editor

```javascript
class NumericCellEditor {
    eInput;
    cancelBeforeStart = false;

    init(params) {
        this.eInput = document.createElement('input');
        this.eInput.type = 'number';
        this.eInput.className = 'numeric-editor';
        this.eInput.value = params.value || '';
        this.eInput.style.width = '100%';
        this.eInput.style.height = '100%';

        // Cancel if invalid key pressed
        if (params.eventKey && (params.eventKey < '0' || params.eventKey > '9')) {
            this.cancelBeforeStart = true;
            return;
        }

        // Set initial value from key press
        if (params.eventKey) {
            this.eInput.value = params.eventKey;
        }

        // Handle keyboard events
        this.eInput.addEventListener('keydown', (event) => {
            if (event.key === 'Enter') {
                params.stopEditing();
            } else if (event.key === 'Escape') {
                params.stopEditing(true); // Cancel
            }
        });
    }

    getGui() {
        return this.eInput;
    }

    afterGuiAttached() {
        this.eInput.focus();
        this.eInput.select();
    }

    getValue() {
        return parseInt(this.eInput.value) || 0;
    }

    isCancelBeforeStart() {
        return this.cancelBeforeStart;
    }

    isCancelAfterEnd() {
        // Validate: no negative numbers
        return this.getValue() < 0;
    }

    isPopup() {
        return false;
    }
}

// Usage
{
    field: 'age',
    editable: true,
    cellEditor: NumericCellEditor,
    cellEditorParams: {
        min: 0,
        max: 100
    }
}
```

### Value Parsers and Setters

#### Value Parser
Transform the value before it's saved:

```javascript
{
    field: 'price',
    editable: true,
    valueParser: (params) => {
        // Remove currency symbols and parse
        return parseFloat(params.newValue.replace(/[^0-9.]/g, ''));
    }
}
```

#### Value Setter
Custom logic for saving the value:

```javascript
{
    field: 'fullName',
    editable: true,
    valueSetter: (params) => {
        // Custom save logic
        params.data.firstName = params.newValue.split(' ')[0];
        params.data.lastName = params.newValue.split(' ')[1];
        return true; // Return true to accept the change
    }
}
```

### Editing API

#### Start Editing Programmatically

```javascript
gridApi.startEditingCell({
    rowIndex: 2,
    colKey: 'athlete',
    key: 'Enter'  // Optional: simulate key press
});
```

#### Stop Editing

```javascript
gridApi.stopEditing(false);  // false = don't cancel, true = cancel
```

#### Get Currently Editing Cells

```javascript
const editingCells = gridApi.getEditingCells();
console.log('Editing:', editingCells);
```

### Editing Events

```javascript
const gridOptions = {
    // Fired when editing starts
    onCellEditingStarted: (event) => {
        console.log('Started editing:', event.column.getColId());
    },

    // Fired when editing stops
    onCellEditingStopped: (event) => {
        console.log('Stopped editing');
        console.log('Value changed:', event.valueChanged);
        console.log('Old value:', event.oldValue);
        console.log('New value:', event.newValue);
    },

    // Fired when cell value changes
    onCellValueChanged: (event) => {
        console.log('Value changed from', event.oldValue, 'to', event.newValue);
        console.log('Row data:', event.data);

        // Save to backend
        fetch('/api/update', {
            method: 'POST',
            body: JSON.stringify(event.data)
        });
    }
};
```

### Undo/Redo Support

Enable undo/redo for cell edits:

```javascript
const gridOptions = {
    defaultColDef: {
        editable: true,
        enableCellChangeFlash: true  // Flash cells when changed
    },
    
    // Enable undo/redo
    undoRedoCellEditing: true,
    
    // Limit undo/redo steps
    undoRedoCellEditingLimit: 5
};

// Use undo/redo programmatically
gridApi.undoCellEditing();
gridApi.redoCellEditing();
const undoSize = gridApi.getCurrentUndoSize();
const redoSize = gridApi.getCurrentRedoSize();
```

---

## B) Highlightable Rows with Selection Color

### Overview

AG Grid supports row selection with customizable highlighting. Selected rows can change color to provide visual feedback, and you can configure selection modes, styling, and behavior.

### Basic Row Selection Configuration

#### Enable Row Selection

```javascript
const gridOptions = {
    columnDefs: [
        { field: 'athlete' },
        { field: 'age' },
        { field: 'country' }
    ],
    
    // Enable multi-row selection
    rowSelection: {
        mode: 'multiRow'  // Options: 'singleRow', 'multiRow'
    }
};
```

#### Single Row Selection

```javascript
rowSelection: {
    mode: 'singleRow'
}
```

#### Multi Row Selection

```javascript
rowSelection: {
    mode: 'multiRow'
}
```

### Styling Selected Rows

#### Using CSS Variables (Recommended)

```css
.ag-theme-quartz {
    /* Selected row background color (semi-transparent) */
    --ag-selected-row-background-color: rgb(0, 255, 0, 0.1);
    
    /* Keep alternating row colors visible */
    --ag-odd-row-background-color: #f5f5f5;
}
```

#### Using Theme Parameters (TypeScript/JavaScript)

```typescript
import { themeQuartz } from 'ag-grid-community';

const myTheme = themeQuartz.withParams({
    // Bright green, 10% opacity
    selectedRowBackgroundColor: "rgba(0, 255, 0, 0.1)",
    
    // Alternating row colors visible through selection
    oddRowBackgroundColor: "#8881"
});

const gridOptions = {
    theme: myTheme,
    rowSelection: {
        mode: 'multiRow'
    }
};
```

#### Custom CSS Class

```css
.ag-theme-quartz .ag-row-selected {
    background-color: rgba(33, 150, 243, 0.2) !important;
}

.ag-theme-quartz .ag-row-selected:hover {
    background-color: rgba(33, 150, 243, 0.3) !important;
}
```

### Selection API

#### Select/Deselect Rows Programmatically

```javascript
// Select all rows
gridApi.selectAll();

// Select only filtered rows
gridApi.selectAllFiltered();

// Deselect all rows
gridApi.deselectAll();

// Get selected rows (data objects)
const selectedRows = gridApi.getSelectedRows();
console.log('Selected data:', selectedRows);

// Get selected nodes (RowNode objects)
const selectedNodes = gridApi.getSelectedNodes();
console.log('Selected nodes:', selectedNodes);
```

#### Select Specific Rows

```javascript
// Get a specific row node
const rowNode = gridApi.getRowNode('rowId');

// Select it (true = selected, false = don't clear others)
rowNode.setSelected(true, false);

// Or use the API
gridApi.setNodesSelected({
    nodes: [rowNode],
    newValue: true,
    source: 'api'
});
```

#### Check Selection State

```javascript
// Check if a row is selected
const isSelected = rowNode.isSelected();

// Check if row is selectable
const isSelectable = rowNode.selectable;
```

### Selection Events

```javascript
const gridOptions = {
    // Fired when a row's selection changes
    onRowSelected: (event) => {
        console.log('Row selection changed');
        console.log('Is selected:', event.node.isSelected());
        console.log('Row data:', event.data);
    },

    // Fired when any selection changes
    onSelectionChanged: (event) => {
        const selectedRows = event.api.getSelectedRows();
        console.log('Selection changed, count:', selectedRows.length);
        console.log('Selected rows:', selectedRows);
    },

    // Handle row click to toggle selection
    onRowClicked: (event) => {
        console.log('Row clicked:', event.data);
        // Toggle selection
        event.node.setSelected(!event.node.isSelected());
    }
};
```

### Conditional Row Selection

Make rows conditionally selectable:

```javascript
const gridOptions = {
    rowSelection: {
        mode: 'multiRow'
    },
    
    // Check if row is selectable
    isRowSelectable: (rowNode) => {
        // Only allow selection of rows where age > 18
        return rowNode.data.age > 18;
    }
};
```

### Pre-select Rows on Load

```javascript
const gridOptions = {
    rowSelection: {
        mode: 'multiRow'
    },
    
    onFirstDataRendered: (params) => {
        // Select rows at specific indices
        params.api.forEachNode((node) => {
            if (node.rowIndex >= 2 && node.rowIndex <= 6) {
                node.setSelected(true);
            }
        });
    }
};
```

### Checkbox Selection Column

Add a checkbox column for selection:

```javascript
const gridOptions = {
    columnDefs: [
        {
            checkboxSelection: true,  // Adds checkbox
            headerCheckboxSelection: true,  // Adds header checkbox
            width: 50
        },
        { field: 'athlete' },
        { field: 'age' }
    ],
    
    rowSelection: {
        mode: 'multiRow'
    }
};
```

### Advanced Selection Styling

#### Multiple Selection Colors

For overlapping selections (Enterprise feature):

```css
.ag-theme-quartz {
    --ag-range-selection-background-color: rgb(255, 0, 128, 0.1);
    --ag-range-selection-background-color-2: rgb(255, 0, 128, 0.19);
    --ag-range-selection-background-color-3: rgb(255, 0, 128, 0.27);
    --ag-range-selection-background-color-4: rgb(255, 0, 128, 0.34);
}
```

### Complete Example

```javascript
import { createGrid, themeQuartz } from 'ag-grid-community';

// Create custom theme with selection colors
const myTheme = themeQuartz.withParams({
    selectedRowBackgroundColor: "rgba(33, 150, 243, 0.2)",
    oddRowBackgroundColor: "#f5f5f5"
});

const gridOptions = {
    theme: myTheme,
    
    columnDefs: [
        {
            checkboxSelection: true,
            headerCheckboxSelection: true,
            width: 50
        },
        { field: 'athlete', editable: true },
        { field: 'age', editable: true },
        { field: 'country', editable: true }
    ],
    
    defaultColDef: {
        editable: true,
        filter: true
    },
    
    rowSelection: {
        mode: 'multiRow'
    },
    
    // Selection events
    onRowSelected: (event) => {
        console.log('Row selected:', event.node.isSelected());
    },
    
    onSelectionChanged: (event) => {
        const selected = event.api.getSelectedRows();
        console.log('Selected rows:', selected.length);
    },
    
    // Editing events
    onCellValueChanged: (event) => {
        console.log('Cell edited:', event.oldValue, '->', event.newValue);
    }
};

// Create grid
const gridDiv = document.querySelector('#myGrid');
const gridApi = createGrid(gridDiv, gridOptions);

// Load data
fetch('https://www.ag-grid.com/example-assets/olympic-winners.json')
    .then(response => response.json())
    .then(data => gridApi.setGridOption('rowData', data));
```

### Key Points

**Editable Cells:**
- Set `editable: true` in column definition or `defaultColDef`
- Use callbacks for conditional editing
- Multiple built-in editors (text, number, select)
- Support for custom editors
- Value parsers and setters for data transformation
- Undo/redo support available

**Row Selection:**
- Configure via `rowSelection.mode`
- Style with CSS variables or theme parameters
- Use semi-transparent colors to maintain row striping
- Programmatic selection via API
- Event handlers for selection changes
- Checkbox column for visual selection

## References

- [AG Grid Cell Editing Documentation](https://www.ag-grid.com/javascript-data-grid/cell-editing)
- [AG Grid Row Selection Documentation](https://www.ag-grid.com/javascript-data-grid/row-selection)
- [AG Grid Theming Documentation](https://www.ag-grid.com/javascript-data-grid/theming)

