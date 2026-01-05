Text Formatting

javascript logoJavaScript
Use a Value Formatter to provide text formatting of values.

In the example below:

Columns A and B display the value of the field property
Columns £A and £B use a currencyFormatter to display the value as a currency
Columns(A) and (B) use a bracketsFormatter to display the value inside brackets
styles.css
main.js
index.html
Language:

JavaScript
let gridApi;

const gridOptions = {
  columnDefs: [
    { headerName: "A", field: "a" },
    { headerName: "B", field: "b" },
    { headerName: "£A", field: "a", valueFormatter: currencyFormatter },
    { headerName: "£B", field: "b", valueFormatter: currencyFormatter },
    { headerName: "(A)", field: "a", valueFormatter: bracketsFormatter },
    { headerName: "(B)", field: "b", valueFormatter: bracketsFormatter },
  ],
  defaultColDef: {
    flex: 1,
    cellClass: "number-cell",
  },
  rowData: createRowData(),
};

function bracketsFormatter(params) {
  return "(" + params.value + ")";
}

function currencyFormatter(params) {
  return "£" + formatNumber(params.value);
}

function formatNumber(number) {
  return Math.floor(number).toLocaleString();
}

function createRowData() {
  const rowData = [];

  for (let i = 0; i < 100; i++) {
    rowData.push({
      a: Math.floor(((i + 2) * 173456) % 10000),
      b: Math.floor(((i + 7) * 373456) % 10000),
    });
  }

  return rowData;
}

// setup the grid after the page has finished loading
document.addEventListener("DOMContentLoaded", function () {
  const gridDiv = document.querySelector("#myGrid");
  gridApi = agGrid.createGrid(gridDiv, gridOptions);
});
 Preview
Copy Link
New Tab
CodeSandbox
Plunker
const gridOptions = {
    columnDefs: [
        // simple currency formatter
        { field: 'price', valueFormatter: params => '$' + params.value },
        // simple UPPER CASE formatter
        { field: 'code', valueFormatter: params => params.value.toUpperCase() }
    ],

    // other grid options ...
}
Value Formatter Definition
Copy Link
Below shows the column definition properties for value formatters.

valueFormatter
Copy Link

string | ValueFormatterFunc
A function or expression to format a value, should return a string.
Please note the Value Formatter params won't always have data and node supplied, e.g. the params supplied to the Value Formatter in the Set Filter.

As a result favour formatter implementations that rely upon the 'value' argument instead, as this will lead to better reuse of your Value Formatters.

If using Cell Data Types, value formatters are set by default to handle the display of each of the different data types.

If you want more than text formatting, e.g. you need Buttons in the Cell, then use a Cell Component.

Formatting for Export
Copy Link
By default, the grid uses the value formatter when performing other grid operations that need values in string format.

This behaviour can be prevented by setting the column definition property useValueFormatterForExport = false (note this does not apply to rendering).

useValueFormatterForExport
Copy Link
boolean
default: true
By default, values are formatted using the column's valueFormatter when exporting data from the grid. This applies to CSV and Excel export, as well as clipboard operations and the fill handle. Set to false to prevent values from being formatted for these operations. Regardless of this option, if custom handling is provided for the export operation, the value formatter will not be used.
Using the value formatter for export applies to the following features:

Copy/Cut
Fill Handle
Copy Range Down
CSV Export
Excel Export
Using a value formatter for export is normally used in conjunction with Using a Value Parser for Import, where a Value Parser is defined that does the reverse of the value formatter.

The following example demonstrates the default behaviour using the value formatter for export with each of the supported features mentioned above.

main.js
index.html
Language:

JavaScript
let gridApi;

const gridOptions = {
  columnDefs: [
    {
      headerName: "£A",
      field: "a",
      valueFormatter: currencyFormatter,
      valueParser: currencyParser,
    },
    {
      headerName: "£B",
      field: "b",
      valueFormatter: currencyFormatter,
      valueParser: currencyParser,
    },
  ],
  defaultColDef: {
    cellDataType: false,
    editable: true,
  },
  rowData: createRowData(),
  cellSelection: {
    handle: {
      mode: "fill",
    },
  },
};

function currencyFormatter(params) {
  return params.value == null ? "" : "£" + params.value;
}

function currencyParser(params) {
  let value = params.newValue;
  if (value == null || value === "") {
    return null;
  }
  value = String(value);

  if (value.startsWith("£")) {
    value = value.slice(1);
  }
  return parseFloat(value);
}

function createRowData() {
  const rowData = [];

  for (let i = 0; i < 100; i++) {
    rowData.push({
      a: Math.floor(((i + 2) * 173456) % 10000),
      b: Math.floor(((i + 7) * 373456) % 10000),
    });
  }

  return rowData;
}

// setup the grid after the page has finished loading
document.addEventListener("DOMContentLoaded", function () {
  const gridDiv = document.querySelector("#myGrid");
  gridApi = agGrid.createGrid(gridDiv, gridOptions);
});
 Preview
Copy Link
New Tab
CodeSandbox
Plunker
Note that if any of the following conditions are true, then useValueFormatterForExport is ignored for that feature and the value will be either the original value or that set in the custom handler:

If processCellForClipboard is provided when using copy/cut.
If fillOperation is provided when using fill handle.
If processCellForClipboard is provided when using copy range down.
If processCellCallback is provided when using CSV export.
If processCellCallback or Excel Data Types are provided when using Excel export.
If the underlying value is a number when using Excel export. To export formatted number values to Excel, please use the Excel Data Type feature.