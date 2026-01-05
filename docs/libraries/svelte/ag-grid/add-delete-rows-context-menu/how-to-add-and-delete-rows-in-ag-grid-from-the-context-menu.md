Title: How To Add and Delete Rows in AG Grid from the Context Menu

Description: Add Excel-style row operations to AG Grid. Right-click to insert/delete rows based on cell selection. Includes live demo & source code for React, Angular, Vue and JS.

### Quick Links

- Why Use AG Grid?
- Tutorials
- Friends & Collaborators
- Sample Applications

### Recent Posts

- What's New in AG Grid 34.2
- What's New in AG Charts 12.2
- What's New in AG Grid's Figma Design System
- What's New in AG Charts 12.1
- What's New in AG Grid 34.1
- How To Add and Delete Rows in AG Grid from the Context Menu
- What's New in AG Grid 34
- What's New in AG Charts 12
- AG Grid Showcase: Sample Apps, Demos, Examples & Extensions
- No user left behind: let’s talk accessibility

- James Swinton-Bland

28 July 2025   |   How To

This blog demonstrates how to add and remove rows in AG Grid, similar to the functionality found in Google Sheets or Excel, using custom context menu items and the Transaction API.

You will learn how to:

1.  Implement custom context menu items for adding & removing rows.
2.  Calculate how many rows to add/delete, and where to add them, by accessing the currently selected cells.
3.  Use the Transaction API to add/remove X number of rows.
4.  Use the Grid API to automatically enable editing on the new rows

0:00

/0:14

1×

💡

If you want to define the data before adding a new row, refer to our blog on Adding New Rows Using a Pinned Row.

## Example & Source Code

In the example below, selecting one or more cells and then opening the context menu provides options to insert or delete the same number of rows above or below the current selected range:

**Source Code:** React | Angular | Vue | JavaScript

## TL;DR

A quick summary of this how-to

## Configure Grid Options

```tsx
<AgGridReact<IOlympicData>
  onGridReady={onGridReady} // Fetch data & set API ref (not shown here)
  getContextMenuItems={getContextMenuItems} // Custom context menu provider
  editType="fullRow" // Enable full row editing mode
  cellSelection // Enable cell selection feature
  rowNumbers // Display row numbers for each row
/>
```

### Get Selection Bounds

```ts
/**
* Extracts information about the currently selected cell range or clicked row

* If a cell range is selected, returns normalised start/end indices and a row count.
* If no range is selected, returns the index of the rowNode the user clicked on
*/
const getCellSelectionBounds = useCallback(
  (params: GetContextMenuItemsParams): ICellSelectionBounds => {
    // Get cell ranges from the grid API
    const cellRanges = gridRef.current?.api.getCellRanges();

    // Fallback to clicked row if no cell range is selected
    if (!cellRanges || !cellRanges[0]?.startRow || !cellRanges[0]?.endRow) {
      const rowIndex = params.node?.rowIndex || 0;
      return { startIndex: rowIndex, endIndex: rowIndex, rowCount: 1 };
    }

    // Extract row indices from the first cell range
    const cellRangeStartRowIndex = cellRanges[0].startRow.rowIndex;
    const cellRangeEndRowIndex = cellRanges[0].endRow.rowIndex;

    // Calculate total rows in selection (inclusive)
    const rowCount =
      Math.abs(cellRangeEndRowIndex - cellRangeStartRowIndex) + 1;

    // Normalize indices since selection can be made in either direction
    const startIndex = Math.min(cellRangeStartRowIndex, cellRangeEndRowIndex);
    const endIndex = Math.max(cellRangeStartRowIndex, cellRangeEndRowIndex);

    // Return normalized selection bounds
    return { startIndex, endIndex, rowCount };
  },
  [],
);
```

### Implement Custom Context Menu

```ts
/**
 * Builds custom context menu items based on cell selection:
 * Adds options for Adding rows above & below current selection
 * Adds option for deleting currently selected rows
 */
const getContextMenuItems = useCallback(
  (params: GetContextMenuItemsParams): (DefaultMenuItem | MenuItemDef)[] => {
    // Get selection bounds (either from cell range or clicked row)
    const { startIndex, endIndex, rowCount } = getCellSelectionBounds(params);

    // Create pluralized label for menu items
    const rowLabel = `${rowCount} Row${rowCount !== 1 ? "s" : ""}`;

    // Build context menu with row manipulation options
    return [
      {
        name: `Insert ${rowLabel} Above`,
        action: () => addRows(rowCount, startIndex),
        icon: '<span class="ag-icon ag-icon-plus"></span>',
      },
      {
        name: `Insert ${rowLabel} Below`,
        action: () => addRows(rowCount, endIndex + 1),
        icon: '<span class="ag-icon ag-icon-plus"></span>',
      },
      "separator",
      {
        name: `Delete ${rowLabel}`,
        action: () => deleteRows(startIndex, endIndex),
        icon: '<span class="ag-icon ag-icon-minus"></span>',
      },
      // Include default menu items (copy, paste, export, etc.)
      ...(params.defaultItems ?? []),
    ];
  },
  [addRows, deleteRows, getCellSelectionBounds],
);
```

### Add Rows

```ts
// Clear selection and focus on the first new row for immediate editing
const startEditingCell = useCallback(
  (insertIndex: number, firstColumn: string) => {
    gridApi.current?.clearCellSelection();
    gridApi.current?.setFocusedCell(insertIndex, firstColumn);
    gridApi.current?.startEditingCell({
      rowIndex: insertIndex,
      colKey: firstColumn,
    });
  },
  [],
);

/**
 * Adds X number of empty rows to the grid, either above or below the
 * currently selected cell range, based on the number of selected rows.
 */
const addRows = useCallback(
  (rowCount: number, startIndex?: number, endIndex?: number) => {
    // Create empty row objects for insertion
    const newRows = Array.from({ length: rowCount }, () => ({}));

    // Determine insertion point
    const insertIndex = startIndex || endIndex || 0;

    // Insert rows at the specified index
    const result = gridApi.current?.applyTransaction({
      add: newRows,
      addIndex: insertIndex,
    });

    // If rows are added, focus on and start editing first new cell
    if (result && result?.add?.length > 0) {
      // Wait for next frame to ensure grid has processed the transaction
      requestAnimationFrame(() => {
        startEditingCell(insertIndex, columnDefs[0].field || "");
      });
    }
  },
  [columnDefs, startEditingCell],
);
```

### Delete Rows

```ts
/**
 * Deletes rows from the grid within the specified range
 */
const deleteRows = useCallback((startIndex: number, endIndex: number) => {
  // Collect row data within the specified range
  const rowDataToRemove = [];
  for (let i = startIndex; i <= endIndex; i++) {
    const node = gridApi.current?.getDisplayedRowAtIndex(i);
    if (node?.data) {
      rowDataToRemove.push(node.data);
    }
  }

  // Skip removal if no valid rows found
  if (rowDataToRemove.length === 0) return;

  // Remove collected rows from the grid
  gridApi.current?.applyTransaction({ remove: rowDataToRemove });

  // Clear selection after deletion
  gridApi.current?.clearCellSelection();
}, []);
```

## Configuring Grid Options

Several grid options are required to implement this solution, including:

- `onGridReady` - A callback function that stores a reference to the Grid API once the grid has loaded.
- `getContextMenuItems` - A callback function that returns the context menu items to be displayed.
- `editType="fullRow"` - Makes the entire row editable whenever an edit is triggered, either by the user or the API.
- `cellSelection` - Allows the user to select a range of cells.
- `rowNumbers` - Optional, but provides a useful visual anchor when adding or removing rows.

```ts
// Ref to API
const gridApi = useRef<GridApi>(null);

// Set API
const onGridReady = useCallback(async (e: GridReadyEvent) => {
// Set API
gridApi.current = e.api;
}, []);

<AgGridReact<IOlympicData>
onGridReady={onGridReady} //Set API ref
getContextMenuItems={getContextMenuItems} // Custom context menu provider
editType="fullRow" // Enable full row editing mode
cellSelection // Enable cell selection feature
rowNumbers // Display row numbers for each row
/>
```

### Determining the Selection Bounds

To display an intuitive context menu, we first need to know the start & end index of the selected rows, as well as the number of rows selected.

The function below:

1.  Calls the `getCellRanges()` API to get the start and end row indexes of the currently selected cell range.
2.  If no range is selected, it returns the index of the clicked row, with a default `rowCount` of 1.
3.  If a range is selected, it uses `Math.abs`, `Math.min` and `Math.max` to calculate the `rowCount`, `startIndex` and `endIndex` respectively.

```ts
/**
* Extracts information about the currently selected cell range or clicked row

* If a cell range is selected, returns normalised start/end indices and a row count.
* If no range is selected, returns the index of the rowNode the user clicked on
*/
const getCellSelectionBounds = useCallback(
  (params: GetContextMenuItemsParams): ICellSelectionBounds => {
    // Get cell ranges from the grid API
    const cellRanges = gridRef.current?.api.getCellRanges();

    // Fallback to clicked row if no cell range is selected
    if (!cellRanges || !cellRanges[0]?.startRow || !cellRanges[0]?.endRow) {
      const rowIndex = params.node?.rowIndex || 0;
      return { startIndex: rowIndex, endIndex: rowIndex, rowCount: 1 };
    }

    // Extract row indices from the first cell range
    const cellRangeStartRowIndex = cellRanges[0].startRow.rowIndex;
    const cellRangeEndRowIndex = cellRanges[0].endRow.rowIndex;

    // Calculate total rows in selection (inclusive)
    const rowCount =
      Math.abs(cellRangeEndRowIndex - cellRangeStartRowIndex) + 1;

    // Normalize indices since selection can be made in either direction
    const startIndex = Math.min(cellRangeStartRowIndex, cellRangeEndRowIndex);
    const endIndex = Math.max(cellRangeStartRowIndex, cellRangeEndRowIndex);

    // Return normalized selection bounds
    return { startIndex, endIndex, rowCount };
  },
  [],
);
```

## Implementing a Custom Context Menu

Next, we need to implement the `getContextMenuItems` callback to provide the additional context menu options by using the result of the `cellSelectionBounds()` function to display three additional context menu items that handle the insert above, insert below, and delete actions:

```ts
/**
 * Builds custom context menu items based on cell selection:
 * Adds options for Adding rows above & below current selection
 * Adds option for deleting currently selected rows
 */
const getContextMenuItems = useCallback(
  (params: GetContextMenuItemsParams): (DefaultMenuItem | MenuItemDef)[] => {
    // Get selection bounds (either from cell range or clicked row)
    const { startIndex, endIndex, rowCount } = getCellSelectionBounds(params);

    // Create pluralized label for menu items
    const rowLabel = `${rowCount} Row${rowCount !== 1 ? "s" : ""}`;

    // Build context menu with row manipulation options
    return [
      {
        name: `Insert ${rowLabel} Above`,
        action: () => addRows(rowCount, startIndex),
        icon: '<span class="ag-icon ag-icon-plus"></span>',
      },
      {
        name: `Insert ${rowLabel} Below`,
        action: () => addRows(rowCount, endIndex + 1),
        icon: '<span class="ag-icon ag-icon-plus"></span>',
      },
      "separator",
      {
        name: `Delete ${rowLabel}`,
        action: () => deleteRows(startIndex, endIndex),
        icon: '<span class="ag-icon ag-icon-minus"></span>',
      },
      // Include default menu items (copy, paste, export, etc.)
      ...(params.defaultItems ?? []),
    ];
  },
  [addRows, deleteRows, getCellSelectionBounds],
);
```

## Adding Rows

Once we have information about the cells that have been selected, we can use the Transaction API to add the new rows at the correct index.

After the transaction has been applied, the grid automatically performs a model update. `requestAnimationFrame()` is used to wait for this to complete before calling `startEditingCell()` to focus on and start editing the first cell in the first new row.

```ts
// Clear selection and focus on the first new row for immediate editing
const startEditingCell = useCallback(
  (insertIndex: number, firstColumn: string) => {
    gridApi.current?.clearCellSelection();
    gridApi.current?.setFocusedCell(insertIndex, firstColumn);
    gridApi.current?.startEditingCell({
      rowIndex: insertIndex,
      colKey: firstColumn,
    });
  },
  [],
);

/**
 * Adds X number of empty rows to the grid, either above or below the
 * currently selected cell range, based on the number of selected rows.
 */
const addRows = useCallback(
  (rowCount: number, startIndex?: number, endIndex?: number) => {
    // Create empty row objects for insertion
    const newRows = Array.from({ length: rowCount }, () => ({}));

    // Determine insertion point
    const insertIndex = startIndex || endIndex || 0;

    // Insert rows at the specified index
    const result = gridApi.current?.applyTransaction({
      add: newRows,
      addIndex: insertIndex,
    });

    // If rows are added, focus on and start editing the first new cell
    if (result && result?.add?.length > 0) {
      // Wait for next frame to ensure grid has processed the transaction
      requestAnimationFrame(() => {
        startEditingCell(insertIndex, columnDefs[0].field || "");
      });
    }
  },
  [columnDefs, startEditingCell],
);
```

## Deleting Rows

To remove the selected rows, we simply loop through the existing row data to find the selected rows and use the Transaction API to remove those from the grid.

```ts
// Deletes rows from the grid within the specified range
const deleteRows = useCallback((startIndex: number, endIndex: number) => {
  // Collect row data within the specified range
  const rowDataToRemove = [];
  for (let i = startIndex; i <= endIndex; i++) {
    const node = gridApi.current?.getDisplayedRowAtIndex(i);
    if (node?.data) {
      rowDataToRemove.push(node.data);
    }
  }

  // Skip removal if no valid rows found
  if (rowDataToRemove.length === 0) return;

  // Remove collected rows from the grid
  gridApi.current?.applyTransaction({ remove: rowDataToRemove });

  // Clear selection after deletion
  gridApi.current?.clearCellSelection();
}, []);
```

## Things to Consider

The Transaction API automatically performs sorting/filtering actions whenever an add or delete transaction is applied. This means that empty rows will be subject to the conditions applied by the user and may not display at the defined `insertIndex`.

### Options

To workaround this, you can:

- Implement custom sorting & filtering functions that ignore the new rows during the first update after the transaction is applied by adding flags to these rows.
- Use an additional UI element to capture the data for these new rows before applying the transaction.
- Disable the add rows functionality when sorting/filtering is applied.

## Conclusion

This blog demonstrates how to easily implement Excel-like row manipulation in AG Grid using custom context menu items and the Transaction API. Users can select cells and insert/delete corresponding numbers of rows above or below the selection.

Learn more about the features used in this blog on our docs:

- Transaction API
- Custom Context Menu
- Get Selected Cell Ranges API
- Start Cell Editing API

## Next Steps

New to AG Grid? Get started in minutes, for free:

Considering AG Grid Enterprise? Request a free two-week trial licence to test your application in production and get direct access to our support team.

Happy coding!

Read more posts about...

- Versions
- How toʼs
- Tutorials
- Testing

- Javascript
- React
- Angular
- Vue

Join the AG Grid Newsletter

- End-to-End Testing for AG Grid in React with Cypress
- Unit testing AG Grid React Tables with React Testing Library and Vitest
- Writing E2E Tests for AG Grid React Tables with Playwright

Releases

AG Grid 34.1 introduces tree data support for master/detail, testing IDs, improved integrated charts formatters, and accessibility improvements.

Releases

AG Grid Enterprise 34 introduces a new filters tool panel, cell editor validation, bulk & batch cell editing, and support for tree data drag & drop.
