Getting Values

javascript logoJavaScript
Values are mapped into Cells using either field or valueGetter from the Column Definition.

data.js
main.js
index.html
Language:

JavaScript
let gridApi;

const gridOptions = {
  // define grid columns
  columnDefs: [
    { headerName: "Name (field)", field: "name" },
    // Using dot notation to access nested property
    { headerName: "Country (field & dot notation)", field: "person.country" },
    // Show default header name
    {
      headerName: "Total Medals (valueGetter)",
      valueGetter: (p) =>
        p.data.medals.gold + p.data.medals.silver + p.data.medals.bronze,
    },
  ],
  defaultColDef: {
    flex: 1,
  },
  rowData: getData(),
};

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
Field
Copy Link
The Column Definition field property maps values from the row data object to the Column's Cells.

The field supports dot notation (e.g. medals.gold) to access properties of complex objects.

const gridOptions = {
    rowData: [
        {
            athlete: 'Michael Phelps',
            medals: {
                gold: 8, silver: 1, bronze: 0
            }
        }
    ],

    columnDefs: [
        // simple field attribute
        { field: 'athlete' },
        // using dot notation, a Header Name is usally needed
        { field: 'medals.gold', headerName: 'Gold' },
    ],

    // other grid options ...
}
Value Getter
Copy Link
A Value Getter is a function that gets called for each row to return the Cell Value for a Column. Typically column cell values are loaded using a field, and then a valueGetter is used when retrieving the value requires custom logic. Columns with Value Getters usually have manually provided Header Names as the grid cannot derive Header Names from Value Getters like it does with Fields.

columnDefs: [
    // achieves the same as using 'athlete' for the field
    { headerName: 'Athlete', valueGetter: p => p.data.athlete },
    // using valueGetter to combine 3 values into 1
    { headerName: 'Total Medals', valueGetter: p => p.data.bronze + p.data.silver + p.data.gold }
],
valueGetter
Copy Link

string | ValueGetterFunc
Function or expression. Gets the value from your data for display.
All valueGetters must be pure functions. That means, given the same state of your data, it should consistently return the same result. This is important as the grid will only call your valueGetter once during a redraw, even though the value may be used multiple times. For example, the value will be used to display the cell value, however it can additionally be used to provide values to an aggregation function when grouping.

The example below demonstrates valueGetter. The following can be noted from the demo:

Columns A and B are simple columns using field

Value Getters are used in all subsequent columns as follows:

Column 'ID #' prints the row number, taken from the Row Node.
Column 'A+B' adds A and B.
Column 'A * 1000' multiplies A by 1000.
Column 'B * 137' multiplies B by 137.
Column 'Random' doesn't take any value from the data, rather it returns a random value.
Column 'Chain' takes the value 'A+B' and works on it further, thus chaining value getters.
Column 'Const' returns back the same value for each column.
styles.css
main.js
index.html
Language:

JavaScript
function hashValueGetter(params) {
  return params.node ? Number(params.node.id) : null;
}

function abValueGetter(params) {
  return params.data.a + params.data.b;
}

function a1000ValueGetter(params) {
  return params.data.a * 1000;
}
function b137ValueGetter(params) {
  return params.data.b * 137;
}
function randomValueGetter() {
  return Math.floor(window.agRandom() * 1000);
}
function chainValueGetter(params) {
  return params.getValue("a&b") * 1000;
}
function constValueGetter() {
  return 99999;
}
let gridApi;
const gridOptions = {
  columnDefs: [
    {
      headerName: "ID #",
      maxWidth: 100,
      valueGetter: hashValueGetter,
    },
    { field: "a" },
    { field: "b" },
    {
      headerName: "A + B",
      colId: "a&b",
      valueGetter: abValueGetter,
    },
    {
      headerName: "A * 1000",
      minWidth: 95,
      valueGetter: a1000ValueGetter,
    },
    {
      headerName: "B * 137",
      minWidth: 90,
      valueGetter: b137ValueGetter,
    },
    {
      headerName: "Random",
      minWidth: 90,
      valueGetter: randomValueGetter,
    },
    {
      headerName: "Chain",
      valueGetter: chainValueGetter,
    },
    {
      headerName: "Const",
      minWidth: 85,
      valueGetter: constValueGetter,
    },
  ],
  defaultColDef: {
    flex: 1,
    minWidth: 75,
    // cellClass: 'number-cell'
  },
  rowData: createRowData(),
};

function createRowData() {
  const rowData = [];
  for (let i = 0; i < 100; i++) {
    rowData.push({
      a: Math.floor(i % 4),
      b: Math.floor(i % 7),
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
Header Value Getters
Copy Link
See the Column Header Value Getters for an example of using headerValueGetter.