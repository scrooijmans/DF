# AG Grid JavaScript: Delete and Insert Rows

This document explains how to delete and insert rows in AG Grid using the Transaction API, which provides efficient batch operations for adding, removing, and updating rows.

## Overview

AG Grid uses a **Transaction API** (`applyTransaction`) to efficiently add, remove, and update rows. This approach is preferred over replacing the entire dataset because it:
- Only updates affected rows
- Preserves selection, expansion, and scroll position
- Provides better performance for large datasets
- Returns detailed results about what was changed

## Prerequisites

### Required Configuration: getRowId

For transactions to work correctly, especially for `remove` and `update` operations, you must configure `getRowId` to uniquely identify rows:

```javascript
const gridOptions = {
    // Required: Provide unique row IDs
    getRowId: (params) => {
        // Use a unique field from your data
        return params.data.id;  // or params.data.employeeId, etc.
    },
    
    // ... other options
};
```

**Why it's needed:**
- Without `getRowId`, AG Grid uses object reference equality
- With `getRowId`, AG Grid can match rows by ID, making transactions reliable
- Essential for `remove` and `update` operations

## Delete Rows

### Basic Delete Operation

```javascript
// Delete a single row
function deleteRow(rowData) {
    const transaction = {
        remove: [rowData]
    };
    
    const result = gridApi.applyTransaction(transaction);
    console.log('Deleted rows:', result.remove);
}
```

### Delete Multiple Rows

```javascript
// Delete multiple rows
function deleteRows(rowsToDelete) {
    const transaction = {
        remove: rowsToDelete  // Array of row data objects
    };
    
    const result = gridApi.applyTransaction(transaction);
    console.log('Deleted', result.remove.length, 'rows');
}
```

### Delete Selected Rows

```javascript
// Delete all selected rows
function deleteSelectedRows() {
    const selectedRows = gridApi.getSelectedRows();
    
    if (selectedRows.length === 0) {
        alert('No rows selected');
        return;
    }
    
    const transaction = {
        remove: selectedRows
    };
    
    const result = gridApi.applyTransaction(transaction);
    console.log('Deleted', result.remove.length, 'selected rows');
}
```

### Delete Rows by Condition

```javascript
// Delete rows matching a condition
function deleteRowsByCondition() {
    const rowsToDelete = [];
    
    gridApi.forEachNode((node) => {
        if (node.data.country === 'USA') {
            rowsToDelete.push(node.data);
        }
    });
    
    if (rowsToDelete.length === 0) {
        console.log('No rows match condition');
        return;
    }
    
    const transaction = {
        remove: rowsToDelete
    };
    
    const result = gridApi.applyTransaction(transaction);
    console.log('Deleted', result.remove.length, 'rows');
}
```

### Delete Using Row ID Only

When using `getRowId`, you only need the ID field for deletion:

```javascript
const gridOptions = {
    getRowId: (params) => params.data.employeeId
};

// Delete using only the ID
function deleteById(employeeId) {
    const transaction = {
        remove: [{ employeeId: employeeId }]  // Only ID needed
    };
    
    gridApi.applyTransaction(transaction);
}
```

## Insert Rows

### Basic Insert Operation

```javascript
// Add a single new row
function addRow() {
    const newRow = {
        id: generateId(),
        make: 'Tesla',
        model: 'Model 3',
        price: 50000
    };
    
    const transaction = {
        add: [newRow]
    };
    
    const result = gridApi.applyTransaction(transaction);
    console.log('Added row:', result.add);
}
```

### Insert Multiple Rows

```javascript
// Add multiple rows
function addRows() {
    const newRows = [
        { id: 1, make: 'Tesla', model: 'Model 3', price: 50000 },
        { id: 2, make: 'Ford', model: 'Mustang', price: 45000 },
        { id: 3, make: 'BMW', model: 'M3', price: 65000 }
    ];
    
    const transaction = {
        add: newRows
    };
    
    const result = gridApi.applyTransaction(transaction);
    console.log('Added', result.add.length, 'rows');
}
```

### Insert at Specific Position

Use `addIndex` to insert rows at a specific position:

```javascript
// Insert at the beginning (index 0)
function addRowAtStart() {
    const newRow = {
        id: generateId(),
        make: 'Tesla',
        model: 'Model 3',
        price: 50000
    };
    
    const transaction = {
        add: [newRow],
        addIndex: 0  // Insert at the beginning
    };
    
    gridApi.applyTransaction(transaction);
}

// Insert at a specific index
function addRowAtIndex(index) {
    const newRow = {
        id: generateId(),
        make: 'Tesla',
        model: 'Model 3',
        price: 50000
    };
    
    const transaction = {
        add: [newRow],
        addIndex: index  // Insert at specific position
    };
    
    gridApi.applyTransaction(transaction);
}

// Insert after selected row
function addRowAfterSelected() {
    const selectedNodes = gridApi.getSelectedNodes();
    
    if (selectedNodes.length === 0) {
        alert('No row selected');
        return;
    }
    
    const selectedNode = selectedNodes[0];
    const rowIndex = selectedNode.rowIndex;
    
    const newRow = {
        id: generateId(),
        make: 'Tesla',
        model: 'Model 3',
        price: 50000
    };
    
    const transaction = {
        add: [newRow],
        addIndex: rowIndex + 1  // Insert after selected row
    };
    
    gridApi.applyTransaction(transaction);
}
```

### Insert at End (Default)

If `addIndex` is not specified, rows are added at the end:

```javascript
// Add at end (default behavior)
function addRowAtEnd() {
    const newRow = {
        id: generateId(),
        make: 'Tesla',
        model: 'Model 3',
        price: 50000
    };
    
    const transaction = {
        add: [newRow]
        // No addIndex = adds at end
    };
    
    gridApi.applyTransaction(transaction);
}
```

## Combined Operations

### Delete and Insert in One Transaction

You can combine multiple operations in a single transaction:

```javascript
// Delete some rows and add new ones in one operation
function replaceRows() {
    const rowsToDelete = gridApi.getSelectedRows();
    
    const newRows = [
        { id: 101, make: 'Tesla', model: 'Model 3', price: 50000 },
        { id: 102, make: 'Ford', model: 'Mustang', price: 45000 }
    ];
    
    const transaction = {
        remove: rowsToDelete,
        add: newRows
    };
    
    const result = gridApi.applyTransaction(transaction);
    console.log('Removed:', result.remove.length);
    console.log('Added:', result.add.length);
}
```

### Add, Update, and Remove in One Transaction

```javascript
// Perform all operations in one transaction
function complexTransaction() {
    const transaction = {
        add: [
            { id: 101, make: 'Tesla', model: 'Model 3', price: 50000 }
        ],
        update: [
            { id: 2, price: 35000 }  // Update existing row
        ],
        remove: [
            { id: 5 }  // Remove row by ID
        ]
    };
    
    const result = gridApi.applyTransaction(transaction);
    console.log('Transaction complete:', result);
}
```

## Complete Example

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

ModuleRegistry.registerModules([
    RowSelectionModule,
    ClientSideRowModelModule,
    ...(process.env.NODE_ENV !== "production" ? [ValidationModule] : []),
]);

let gridApi: GridApi;
let rowIdCounter = 100;

const gridOptions = {
    columnDefs: [
        { field: 'make' },
        { field: 'model' },
        { field: 'price', type: 'numberColumn' }
    ],
    
    defaultColDef: {
        flex: 1,
        minWidth: 100
    },
    
    // Required for transactions
    getRowId: (params) => {
        return params.data.id;
    },
    
    rowSelection: {
        mode: 'multiRow'
    },
    
    rowData: [
        { id: 1, make: 'Toyota', model: 'Celica', price: 35000 },
        { id: 2, make: 'Ford', model: 'Mondeo', price: 32000 },
        { id: 3, make: 'Porsche', model: 'Boxster', price: 72000 }
    ]
};

// Initialize grid
const gridDiv = document.querySelector('#myGrid');
gridApi = createGrid(gridDiv, gridOptions);

// ===== DELETE OPERATIONS =====

function deleteSelectedRows() {
    const selectedRows = gridApi.getSelectedRows();
    
    if (selectedRows.length === 0) {
        alert('No rows selected');
        return;
    }
    
    const transaction = {
        remove: selectedRows
    };
    
    const result = gridApi.applyTransaction(transaction);
    console.log('Deleted', result.remove.length, 'rows');
}

function deleteRowById(rowId) {
    const transaction = {
        remove: [{ id: rowId }]
    };
    
    gridApi.applyTransaction(transaction);
}

function deleteAllRows() {
    const allRows = [];
    gridApi.forEachNode((node) => {
        allRows.push(node.data);
    });
    
    const transaction = {
        remove: allRows
    };
    
    gridApi.applyTransaction(transaction);
}

// ===== INSERT OPERATIONS =====

function addNewRow() {
    const newRow = {
        id: rowIdCounter++,
        make: 'Tesla',
        model: 'Model 3',
        price: 50000
    };
    
    const transaction = {
        add: [newRow]
    };
    
    const result = gridApi.applyTransaction(transaction);
    console.log('Added row:', result.add[0].data);
}

function addRowAtStart() {
    const newRow = {
        id: rowIdCounter++,
        make: 'BMW',
        model: 'M3',
        price: 65000
    };
    
    const transaction = {
        add: [newRow],
        addIndex: 0  // Insert at beginning
    };
    
    gridApi.applyTransaction(transaction);
}

function addRowAfterSelected() {
    const selectedNodes = gridApi.getSelectedNodes();
    
    if (selectedNodes.length === 0) {
        alert('No row selected');
        return;
    }
    
    const selectedNode = selectedNodes[0];
    const newRow = {
        id: rowIdCounter++,
        make: 'Audi',
        model: 'A4',
        price: 40000
    };
    
    const transaction = {
        add: [newRow],
        addIndex: selectedNode.rowIndex + 1
    };
    
    gridApi.applyTransaction(transaction);
}

function addMultipleRows() {
    const newRows = [
        { id: rowIdCounter++, make: 'Mercedes', model: 'C-Class', price: 45000 },
        { id: rowIdCounter++, make: 'Lexus', model: 'IS', price: 42000 },
        { id: rowIdCounter++, make: 'Acura', model: 'TLX', price: 38000 }
    ];
    
    const transaction = {
        add: newRows
    };
    
    const result = gridApi.applyTransaction(transaction);
    console.log('Added', result.add.length, 'rows');
}

// ===== COMBINED OPERATIONS =====

function replaceSelectedWithNew() {
    const selectedRows = gridApi.getSelectedRows();
    
    if (selectedRows.length === 0) {
        alert('No rows selected');
        return;
    }
    
    const newRows = selectedRows.map((row, index) => ({
        id: rowIdCounter++,
        make: `New Make ${index + 1}`,
        model: `New Model ${index + 1}`,
        price: 30000 + (index * 5000)
    }));
    
    const transaction = {
        remove: selectedRows,
        add: newRows
    };
    
    const result = gridApi.applyTransaction(transaction);
    console.log('Replaced', result.remove.length, 'rows with', result.add.length, 'new rows');
}

// Attach to buttons
document.querySelector('#btn-delete-selected').addEventListener('click', deleteSelectedRows);
document.querySelector('#btn-add-row').addEventListener('click', addNewRow);
document.querySelector('#btn-add-at-start').addEventListener('click', addRowAtStart);
document.querySelector('#btn-add-after-selected').addEventListener('click', addRowAfterSelected);
document.querySelector('#btn-add-multiple').addEventListener('click', addMultipleRows);
document.querySelector('#btn-replace-selected').addEventListener('click', replaceSelectedWithNew);
```

## Transaction Response

The `applyTransaction` method returns a result object:

```javascript
const result = gridApi.applyTransaction(transaction);

// Result structure:
{
    add: [
        { data: {...}, node: RowNode, ... },  // Added rows
        ...
    ],
    remove: [
        { data: {...}, node: RowNode, ... },  // Removed rows
        ...
    ],
    update: [
        { data: {...}, node: RowNode, ... }   // Updated rows
        ...
    ]
}
```

### Using Transaction Results

```javascript
function handleTransaction(transaction) {
    const result = gridApi.applyTransaction(transaction);
    
    if (result.add && result.add.length > 0) {
        console.log('Added rows:', result.add.length);
        result.add.forEach((rowNode) => {
            console.log('  -', rowNode.data);
        });
    }
    
    if (result.remove && result.remove.length > 0) {
        console.log('Removed rows:', result.remove.length);
        result.remove.forEach((rowNode) => {
            console.log('  -', rowNode.data);
        });
    }
    
    if (result.update && result.update.length > 0) {
        console.log('Updated rows:', result.update.length);
        result.update.forEach((rowNode) => {
            console.log('  -', rowNode.data);
        });
    }
    
    // Update UI based on results
    updateRowCount();
    clearSelectionIfNeeded(result);
}
```

## Row Identification

### Using getRowId (Recommended)

```javascript
const gridOptions = {
    getRowId: (params) => {
        // Use a unique identifier from your data
        return params.data.id;
        // or: return params.data.employeeId;
        // or: return `${params.data.category}-${params.data.index}`;
    }
};
```

### Without getRowId (Object Reference)

If you don't provide `getRowId`, AG Grid uses object reference equality:

```javascript
// Works, but less reliable
function deleteRow() {
    // Must pass the exact same object reference
    const rowToDelete = gridApi.getSelectedRows()[0];
    
    const transaction = {
        remove: [rowToDelete]  // Must be same object reference
    };
    
    gridApi.applyTransaction(transaction);
}
```

**Limitation:** This only works if you pass the exact same object that's in the grid. Cloned objects won't match.

## Best Practices

### 1. Always Configure getRowId

```javascript
const gridOptions = {
    getRowId: (params) => params.data.id  // Required for reliable transactions
};
```

### 2. Use Transactions for Batch Operations

```javascript
// Good: Single transaction
const transaction = {
    remove: rowsToDelete,
    add: newRows
};
gridApi.applyTransaction(transaction);

// Avoid: Multiple separate operations
rowsToDelete.forEach(row => {
    gridApi.applyTransaction({ remove: [row] });
});
```

### 3. Handle Transaction Results

```javascript
const result = gridApi.applyTransaction(transaction);

if (result.add.length > 0) {
    // Handle added rows
}

if (result.remove.length > 0) {
    // Handle removed rows
}
```

### 4. Validate Before Deleting

```javascript
function deleteSelectedRows() {
    const selectedRows = gridApi.getSelectedRows();
    
    if (selectedRows.length === 0) {
        alert('No rows selected');
        return;
    }
    
    // Confirm deletion
    if (!confirm(`Delete ${selectedRows.length} row(s)?`)) {
        return;
    }
    
    const transaction = {
        remove: selectedRows
    };
    
    gridApi.applyTransaction(transaction);
}
```

### 5. Generate Unique IDs for New Rows

```javascript
let idCounter = 1;

function addNewRow() {
    const newRow = {
        id: idCounter++,  // Ensure unique ID
        make: 'Tesla',
        model: 'Model 3',
        price: 50000
    };
    
    const transaction = {
        add: [newRow]
    };
    
    gridApi.applyTransaction(transaction);
}
```

## Common Patterns

### Pattern 1: Delete and Add Replacement

```javascript
function replaceRow(oldRow, newRowData) {
    const transaction = {
        remove: [oldRow],
        add: [newRowData]
    };
    
    gridApi.applyTransaction(transaction);
}
```

### Pattern 2: Duplicate Selected Row

```javascript
function duplicateSelectedRow() {
    const selectedRows = gridApi.getSelectedRows();
    
    if (selectedRows.length === 0) {
        return;
    }
    
    const duplicatedRows = selectedRows.map(row => ({
        ...row,
        id: generateNewId(),  // New unique ID
        // Optionally modify other fields
    }));
    
    const transaction = {
        add: duplicatedRows
    };
    
    gridApi.applyTransaction(transaction);
}
```

### Pattern 3: Move Row Up/Down

```javascript
function moveRowUp(rowData) {
    const allRows = [];
    let currentIndex = -1;
    
    gridApi.forEachNode((node, index) => {
        allRows.push(node.data);
        if (node.data.id === rowData.id) {
            currentIndex = index;
        }
    });
    
    if (currentIndex <= 0) {
        return;  // Already at top
    }
    
    // Remove from current position
    const transaction = {
        remove: [rowData],
        add: [rowData],
        addIndex: currentIndex - 1  // Insert one position up
    };
    
    gridApi.applyTransaction(transaction);
}
```

### Pattern 4: Bulk Delete by Filter

```javascript
function deleteFilteredRows() {
    const rowsToDelete = [];
    
    gridApi.forEachNodeAfterFilter((node) => {
        if (node.data.price < 40000) {
            rowsToDelete.push(node.data);
        }
    });
    
    if (rowsToDelete.length === 0) {
        return;
    }
    
    const transaction = {
        remove: rowsToDelete
    };
    
    gridApi.applyTransaction(transaction);
}
```

## Troubleshooting

### Issue: Rows not being deleted

**Solution:**
- Ensure `getRowId` is configured
- Verify the row data object matches (same ID)
- Check that the row exists in the grid

### Issue: Rows added in wrong position

**Solution:**
- Use `addIndex` to specify position
- Remember: `addIndex` is 0-based
- Without `addIndex`, rows are added at the end

### Issue: Transaction not working

**Solution:**
- Verify `getRowId` is configured correctly
- Check that row IDs are unique
- Ensure transaction object structure is correct
- Check browser console for errors

### Issue: Performance issues with many rows

**Solution:**
- Batch operations in single transaction
- Avoid individual transactions in loops
- Use `applyTransactionAsync` for very large operations

## References

- [AG Grid Transaction Updates](https://www.ag-grid.com/javascript-data-grid/data-update-transactions/)
- [AG Grid Data Updates](https://www.ag-grid.com/javascript-data-grid/data-update/)
- [AG Grid getRowId](https://www.ag-grid.com/javascript-data-grid/row-ids/)

