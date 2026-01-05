Column Filters

javascript logoJavaScript
Column Filters are filters that are applied to the data at the column level. Many Column Filters can be active at once (e.g. filters set on different columns) and the grid will display rows that pass every column's filter.

Column Filters are accessed in the grid UI either through the Column Menu or the Tool Panel.

Column Filter Types
Copy Link
You can use the Provided Filters that come with the grid, or you can build your own Filter Components if you want to customise the filter experience to your application.

There are four main Provided Filters, plus the Multi Filter. These are as follows:

Text Filter - A filter for string comparisons.
Number Filter - A filter for number comparisons.
Date Filter - A filter for date comparisons.
Set Filter (e) - A filter influenced by how filters work in Microsoft Excel. This is an AG Grid Enterprise feature.
Multi Filter (e) - Allows for two filters to be used together (either one of the above Provided Filters or a Custom Filter Component). This is an AG Grid Enterprise feature.
Example: Provided Filters
Copy Link
The example below demonstrates the four Provided Filters and the Multi Filter:

Column Athlete has a Text Filter.
Column Age has a Number Filter.
Column Date has a Date Filter.
Column Country has a Set Filter.
Column Sport has a Multi Filter (consisting of a Text Filter and a Set Filter).
main.js
index.html
Language:

JavaScript
const filterParams = {
comparator: (filterLocalDateAtMidnight, cellValue) => {
const dateAsString = cellValue;
if (dateAsString == null) return -1;
const dateParts = dateAsString.split("/");
const cellDate = new Date(
Number(dateParts[2]),
Number(dateParts[1]) - 1,
Number(dateParts[0]),
);

    if (filterLocalDateAtMidnight.getTime() === cellDate.getTime()) {
      return 0;
    }

    if (cellDate < filterLocalDateAtMidnight) {
      return -1;
    }

    if (cellDate > filterLocalDateAtMidnight) {
      return 1;
    }
    return 0;

},
};

const columnDefs = [
{ field: "athlete" },
{ field: "age", filter: "agNumberColumnFilter", maxWidth: 100 },
{
field: "date",
filter: "agDateColumnFilter",
filterParams: filterParams,
},
{ field: "country", filter: "agSetColumnFilter" },
{ field: "sport", filter: "agMultiColumnFilter" },
{ field: "gold", filter: "agNumberColumnFilter" },
{ field: "silver", filter: "agNumberColumnFilter" },
{ field: "bronze", filter: "agNumberColumnFilter" },
{ field: "total", filter: false },
];

let gridApi;

const gridOptions = {
columnDefs: columnDefs,
defaultColDef: {
flex: 1,
minWidth: 150,
filter: "agTextColumnFilter",
suppressHeaderMenuButton: true,
suppressHeaderContextMenu: true,
},
};

// setup the grid after the page has finished loading
document.addEventListener("DOMContentLoaded", function () {
const gridDiv = document.querySelector("#myGrid");
gridApi = agGrid.createGrid(gridDiv, gridOptions);

fetch("https://www.ag-grid.com/example-assets/olympic-winners.json")
.then((response) => response.json())
.then((data) => gridApi.setGridOption("rowData", data));
});
Preview
Copy Link
New Tab
CodeSandbox
Plunker
Relation to Quick Filter and External Filter
Copy Link
Column Filters work independently of Quick Filter and External Filter. If Quick Filter and / or an external filter are applied along with a Column Filter, each filter type is considered and the row will only show if it passes all three types.

Column Filters are tied to a specific column. Quick Filter and external filter are not tied to any particular column. This section of the documentation talks about Column Filters only. For Quick Filter and external filter, click the links above to learn more.

Next Up
Copy Link
Continue to the next section to learn about Text Filters.
