https://www.ag-grid.com/javascript-data-grid/cell-data-types/
Cell Data Types

javascript logoJavaScript
Working with values of different data types is made easy by using cell data types.

This allows different grid features to work without any additional configuration, including Rendering, Editing, Filtering, Sorting, Row Grouping and Import & Export (CSV Export, Excel Export, Clipboard).

Enable Cell Data Types
Copy Link
There are a number of pre-defined cell data types: 'text', 'number', 'boolean', 'date', 'dateString', 'dateTime', 'dateTimeString' and 'object'.

These are enabled by default, with the data type being inferred from the row data if possible (see Inferring Data Types).

Specific cell data types can also be defined by setting the cellDataType property on the column definition.

const gridOptions = {
    columnDefs: [
        {
            field: 'athlete',
            // enables cell data type `text`
            cellDataType: 'text'
        }
    ],

    // other grid options ...
}
The following example demonstrates the pre-defined cell data types (most of which are inferred from the row data):

The Athlete column has a 'text' data type.
The Age column has a 'number' data type.
The Gold column has a 'boolean' data type.
The Date column has a 'date' data type (cell values are Date objects).
The DateTime column has a 'dateTime' data type (cell values are Date objects). This is explicitly set to cellDataType: 'dateTime' as Date objects are inferred to be 'date' data type.
The Date (String) column has a 'dateString' data type (cell values are strings representing dates).
The DateTime (String) column has a 'dateTimeString' data type (cell values are strings representing dates).
The Country column has an 'object' data type. This also Overrides the Pre-Defined Cell Data Type Definition so that the value parser and formatter work with the object structure.

 Code
Copy Link
New Tab
CodeSandbox
Plunker
Inferring Data Types
Copy Link
By default, the grid will infer cell data types the first time that row data is passed into the grid.

For inference to work for a column, it must contain non-null values and have the field property set. The resolved column definition (including the default column definition and column types) must also not have the Value Getter, Value Parser or reference data properties set, or be using Sparklines. If these conditions are not met, no cell data type will be set (it will need to be defined directly on the column if desired).

Because 'dateTime' corresponds to cell values that are Date objects, there is no easy way to tell them apart from regular 'date' columns. If you wish to enable 'dateTime''s higher precision fields, please explicitly specify cellDataType: 'dateTime' in the corresponding columnDefs entry.

Data type inference can be disabled by setting cellDataType = false on an individual column, or for all columns on the Default Column Definition.

Note that where inference is possible, but it does not match any of the pre-defined cell data types, it will default to object.

Inferring cell data types only works for the Client-Side Row Model. For other row models, you will need to define cell data types for each column.

Pre-Defined Cell Data Types
Copy Link
Each of the pre-defined cell data types work by setting specific column definition properties with default values/callbacks. This enables the different grid features to work correctly for that data type.

The column definition properties that are set based on the cell data type will override any in the Default Column Definition, but will be overridden by any Column Type properties as well as properties set directly on individual column definitions. Note that for filterParams, only nested properties on the default column definition will be overridden (rather than the entire object).

If you wish to override one of the properties set below for all types, you can do so by creating a Column Type, and assigning the column type to the Default Column Definition.

All the cell data types set the following (unless specified):

A Value Parser to convert from string to the relevant data type.
A Value Formatter to convert from the relevant data type to string (except for 'text').
A Key Creator which uses the Value Formatter to allow Row Grouping to work (except for 'number' and 'text').
Note that when using cell data types, the Value Formatter will not run for values in group columns (as they have already been formatted), or for aggregated values where the data type can differ. To apply custom formatting in these cases, cell data types will need to be disabled for the underlying columns.

Text
Copy Link
The 'text' cell data type is used for string values. As most grid functionality works directly with string values, the 'text' cell data type does not set any properties outside the ones specified above for all data types.

Number
Copy Link
The 'number' cell data type is used for number values.

The following properties are set:

The Number Cell Editor is used for editing.
When the Set Filter is Disabled by Default, the Number Filter is used.
When the Set Filter is used, filterParams.comparator is set to Sort the Filter List.
To show only a certain number of decimal places, you can Override the Pre-Defined Cell Data Type Definition and provide your own Value Formatter. It is also possible to control the number of decimal places allowed during editing, by providing a precision to the Number Cell Editor.

Boolean
Copy Link
The 'boolean' cell data type is used for boolean values.

The following properties are set:

The Checkbox Cell Renderer is used for rendering, which displays a checkbox. Set cellRendererParams.disabled=true for the checkbox to be read only.
The Checkbox Cell Editor is used for editing (similar to the renderer).
suppressKeyboardEvent is set to enable the ␣ Space key to toggle the renderer value.
When the Set Filter is Disabled by Default, the Text Filter is used.
When the Text Filter is used, filterParams is set to display a single dropdown with 'True'/'False' (or equivalents with Localisation).
When the Set Filter is used, filterParams.valueFormatter is set to show 'True'/'False' (or equivalents with Localisation).
Date
Copy Link
The 'date' cell data type is used for date values that are represented as Date objects.

The default Value Parser and Value Formatter use the ISO string format 'YYYY-MM-DD'. If you wish to use a different date format, then you can Override the Pre-Defined Cell Data Type Definition.

Please note that the 'date' cell data type compares full Date objects, including the time portion. As a result, if a date includes a time other than midnight (00:00:00.000), filtering or editing might behave unexpectedly. For consistent results with built-in filters, it’s best to normalize all time values to the same value. If keeping the time component is important, consider using the 'dateTime' cell data type instead or defining a custom comparator, as explained in the Date Filter Comparator.

The following properties are set:

The Date Cell Editor is used for editing.
When the Set Filter is Disabled by Default, the Date Filter is used.
When the Set Filter is used, the Set Filter Tree List is enabled, and the Values are Formatted by setting filterParams.treeListFormatter to convert the months to names and filterParams.valueFormatter to format the Floating Filter values using the Value Formatter.
Date as String
Copy Link
The 'dateString' cell data type is used for date values that are represented as string values.

This data type uses the ISO string format 'YYYY-MM-DD'. If you wish to use a different date format, then you can Override the Pre-Defined Cell Data Type Definition.

The following properties are set:

The Date as String Cell Editor is used for editing.
When the Set Filter is Disabled by Default, the Date Filter is used.
When the Date Filter is used, filterParams.comparator is set to parse the string date values.
When the Set Filter is used, the Set Filter Tree List is enabled, with filterParams.treeListPathGetter set to convert the string date values into paths, and the Values are Formatted by setting filterParams.treeListFormatter to convert the months to names and filterParams.valueFormatter to format the Floating Filter values using the Value Formatter.
DateTime
Copy Link
The 'dateTime' cell data type is used for date and time values that are represented as Date objects. Unlike the 'date' cell data type which only shows the date portion, 'dateTime' displays both date and time components.

This data type uses the ISO string format 'YYYY-MM-DDThh:mm:ssZ'. If you wish to use a different format, you can Override the Pre-Defined Cell Data Type Definition.

The following properties are set:

The Date Cell Editor is used for editing.
When the Set Filter is Disabled by Default, the Date Filter is used.
When the Set Filter is used, the Set Filter Tree List is enabled, and the Values are Formatted by setting filterParams.treeListFormatter to convert the months to names and filterParams.valueFormatter to format the Floating Filter values using the Value Formatter.
DateTime as String
Copy Link
The 'dateTimeString' cell data type is used for date and time values that are represented as string values. Unlike the 'dateString' cell data type which only shows the date portion, 'dateTimeString' displays both date and time components.

This data type uses the ISO string format 'YYYY-MM-DDThh:mm:ssZ'. If you wish to use a different format, you can Override the Pre-Defined Cell Data Type Definition.

The following properties are set:

The Date as String Cell Editor is used for editing.
When the Set Filter is Disabled by Default, the Date Filter is used.
When the Date Filter is used, filterParams.comparator is set to parse the string date values.
When the Set Filter is used, the Set Filter Tree List is enabled, with filterParams.treeListPathGetter set to convert the string date values into paths, and the Values are Formatted by setting filterParams.treeListFormatter to convert the months to names and filterParams.valueFormatter to format the Floating Filter values using the Value Formatter.
Object
Copy Link
The 'object' cell data type is used for values that are complex objects (e.g. none of the above data types).

If you have different types of complex object, you will want to Provide Custom Cell Data Types.

For objects to work properly, you must provide a Value Formatter, and a Value Parser if editing is enabled. This is because their behaviour needs to change based on the object structure. Generally these should be provided on the data type definition, but they can be provided directly on the column if necessary.

The following properties are set:

cellEditorParams.useFormatter = true so that the cell editor uses the Value Formatter.
A comparator is defined to allow Custom Sorting using the Value Formatter.
When the Text Filter is used, a Filter Value Getter is used to convert the value with the Value Formatter.
When the Set Filter is used, filterParams.valueFormatter is set to format the values using the Value Formatter.
Pre-Defined Cell Data Type Example
Copy Link
The Enable Cell Data Types Example above demonstrates each of the different pre-defined cell data types with AG Grid Community.

The example below shows the same data types in AG Grid Enterprise:

Row grouping is enabled allowing each of the fields to be grouped on.
Import/Export features are enabled allowing the following:
Clipboard (copy/paste)
Fill handle
CSV/Excel export
main.js
index.html
Language:

JavaScript
let gridApi;

const gridOptions = {
  columnDefs: [
    { field: "athlete" },
    { field: "age", minWidth: 100 },
    { field: "hasGold", minWidth: 100, headerName: "Gold" },
    {
      field: "hasSilver",
      minWidth: 100,
      headerName: "Silver",
      cellRendererParams: { disabled: true },
    },
    { field: "dateObject", headerName: "Date" },
    { field: "date", headerName: "Date (String)" },
    {
      field: "dateTime",
      headerName: "DateTime",
      cellDataType: "dateTime",
      minWidth: 250,
    },
    { field: "dateTimeString", headerName: "DateTime (String)", minWidth: 250 },
    { field: "countryObject", headerName: "Country" },
  ],
  defaultColDef: {
    flex: 1,
    minWidth: 180,
    filter: true,
    floatingFilter: true,
    editable: true,
    enableRowGroup: true,
  },
  dataTypeDefinitions: {
    object: {
      baseDataType: "object",
      extendsDataType: "object",
      valueParser: (params) => ({ name: params.newValue }),
      valueFormatter: (params) =>
        params.value == null ? "" : params.value.name,
    },
  },
  rowGroupPanelShow: "always",
  cellSelection: {
    handle: { mode: "fill" },
  },
};

// setup the grid after the page has finished loading
document.addEventListener("DOMContentLoaded", () => {
  const gridDiv = document.querySelector("#myGrid");
  gridApi = agGrid.createGrid(gridDiv, gridOptions);

  fetch("https://www.ag-grid.com/example-assets/olympic-winners.json")
    .then((response) => response.json())
    .then((data) =>
      gridApi.setGridOption(
        "rowData",
        data.map((rowData) => {
          const dateParts = rowData.date.split("/");
          const [year, month, day] = dateParts
            .reverse()
            .map((e) => parseInt(e, 10));
          const [h, m, s] = [
            Math.floor(window.agRandom() * 24),
            Math.floor(window.agRandom() * 60),
            Math.floor(window.agRandom() * 60),
          ];
          const paddedDateTimeStrings = [month, day, h, m, s].map((e) =>
            e.toString().padStart(2, "0"),
          );
          const dateString = `${year}-${paddedDateTimeStrings[0]}-${paddedDateTimeStrings[1]}`;
          const dateTimeString = `${year}-${paddedDateTimeStrings[0]}-${paddedDateTimeStrings[1]}T${paddedDateTimeStrings.slice(2).join(":")}`;
          return {
            ...rowData,
            date: dateString,
            dateObject: new Date(year, month - 1, day),
            dateTimeString,
            dateTime: new Date(year, month - 1, day, h, m, s),
            countryObject: {
              name: rowData.country,
            },
            hasGold: rowData.gold > 0,
            hasSilver: rowData.silver > 0,
          };
        }),
      ),
    );
});
 Preview
Copy Link
New Tab
CodeSandbox
Plunker
Providing Custom Cell Data Types
Copy Link
Custom cell data types can be added by setting the grid option dataTypeDefinitions.

dataTypeDefinitions
Copy Link

{ [cellDataType: string]: DataTypeDefinition; }
An object map of cell data types to their definitions. Cell data types can either override/update the pre-defined data types ('text', 'number', 'boolean', 'date', 'dateString', 'dateTime', 'dateTimeString' or 'object'), or can be custom data types.
const gridOptions = {
    dataTypeDefinitions: {
        percentage: {
            extendsDataType: 'number',
            baseDataType: 'number',
            valueFormatter: params => params.value == null
                ? ''
                : `${Math.round(params.value * 100)}%`,
        }
    },

    // other grid options ...
}
Each custom data type definition must have a baseDataType of one of the Pre-Defined Cell Data Types, which represents the data type of the underlying cell values.

Data type definitions support inheritance via the extendsDataType property. Each custom cell data type must either extend one of the pre-defined types, or another custom type. Any non-overridden properties are inherited from the parent definition. To prevent inheriting properties from the parent definition, suppressDefaultProperties = true can be set on the definition.

Column Types can be set via the columnTypes property to allow other column definition properties to be set for the data type. By default, these will replace any column types against the parent definition. To allow these to be appended to the parent definition column types, appendColumnTypes = true can be set.

To allow Inferring Cell Data Types to work for custom types, the dataTypeMatcher property can be set. This returns true if the value is of the correct type. Note that the data type matchers will be called in the order they are provided in dataTypeDefinitions (for custom only), and then the pre-defined data type matchers will be called.

The following example demonstrates providing custom cell data types:

The Country column contains complex objects and has a cell data type of 'country'.
The Sport column contains a different type of complex object and has a cell data type of 'sport'.
The dataTypeMatcher callback is defined for both cell data types to allow inferring the type.
main.js
index.html
Language:

JavaScript
let gridApi;

const gridOptions = {
  columnDefs: [
    { field: "athlete" },
    { field: "countryObject", headerName: "Country" },
    { field: "sportObject", headerName: "Sport" },
  ],
  defaultColDef: {
    filter: true,
    floatingFilter: true,
    editable: true,
  },
  dataTypeDefinitions: {
    country: {
      baseDataType: "object",
      extendsDataType: "object",
      valueParser: (params) =>
        params.newValue == null || params.newValue === ""
          ? null
          : { code: params.newValue },
      valueFormatter: (params) =>
        params.value == null ? "" : params.value.code,
      dataTypeMatcher: (value) => value && !!value.code,
    },
    sport: {
      baseDataType: "object",
      extendsDataType: "object",
      valueParser: (params) =>
        params.newValue == null || params.newValue === ""
          ? null
          : { name: params.newValue },
      valueFormatter: (params) =>
        params.value == null ? "" : params.value.name,
      dataTypeMatcher: (value) => value && !!value.name,
    },
  },
  cellSelection: { handle: { mode: "fill" } },
};

// setup the grid after the page has finished loading
document.addEventListener("DOMContentLoaded", () => {
  const gridDiv = document.querySelector("#myGrid");
  gridApi = agGrid.createGrid(gridDiv, gridOptions);

  fetch("https://www.ag-grid.com/example-assets/olympic-winners.json")
    .then((response) => response.json())
    .then((data) =>
      gridApi.setGridOption(
        "rowData",
        data.map((rowData) => {
          return {
            ...rowData,
            countryObject: {
              code: rowData.country,
            },
            sportObject: {
              name: rowData.sport,
            },
          };
        }),
      ),
    );
});
 Preview
Copy Link
New Tab
CodeSandbox
Plunker
Overriding the Pre-Defined Cell Data Type Definitions
Copy Link
The default properties for the Pre-Defined Cell Data Types can be overridden.

For example, this is required if a different date format is desired.

This works in the same way as when Providing Custom Cell Data Types.

const gridOptions = {
    dataTypeDefinitions: {
        // override `date` to handle custom date format `dd/mm/yyyy`
        date: {
            baseDataType: 'date',
            extendsDataType: 'date',
            valueParser: params => {
                if (params.newValue == null) {
                    return null;
                }
                // convert from `dd/mm/yyyy`
                const dateParts = params.newValue.split('/');
                return dateParts.length === 3 ? new Date(
                    parseInt(dateParts[2]),
                    parseInt(dateParts[1]) - 1,
                    parseInt(dateParts[0])
                ) : null;
            },
            valueFormatter: params => {
                // convert to `dd/mm/yyyy`
                const date = params.value;
                return date == null
                    ? ''
                    : `${date.getDate()}/${date.getMonth() + 1}/${date.getFullYear()}`;
            },
        }
    },

    // other grid options ...
}
The following example demonstrates overriding pre-defined cell data types:

The Date column is of type 'dateString' which has been overridden to use a different date format (dd/mm/yyyy).
The data type definition for 'dateString' provides a dateParser and dateFormatter as it is a Date as String Data Type Definition.
The DateTimeWithSpace column overrides a built-in 'dateTimeString' type with custom parsing/formatting logic to support dd/MM/yyyy HH:mm:ss format instead of the default yyyy-MM-ddTHH:mm:ss.
main.js
index.html
Language:

JavaScript
let gridApi;

const dateTimeRegex = /(\d{2})\/(\d{2})\/(\d{4}).{1,2}(\d{2}):(\d{2}):(\d{2})/;
const pad = (n) => (n < 10 ? `0${n}` : n);
const rand = (min, max) => Math.floor((max + min) * window.agRandom() - min);

const gridOptions = {
  columnDefs: [
    { field: "athlete" },
    { field: "age" },
    { field: "date" },
    {
      field: "dateTimeWithSpace",
      cellDataType: "dateTimeString",
      filterParams: { includeTime: true },
      cellEditorParams: { includeTime: true },
    },
  ],
  defaultColDef: {
    filter: true,
    floatingFilter: true,
    editable: true,
  },
  dataTypeDefinitions: {
    dateString: {
      baseDataType: "dateString",
      extendsDataType: "dateString",
      valueParser: (params) =>
        params.newValue != null && params.newValue.match("\\d{2}/\\d{2}/\\d{4}")
          ? params.newValue
          : null,
      valueFormatter: (params) => (params.value == null ? "" : params.value),
      dataTypeMatcher: (value) =>
        typeof value === "string" && !!value.match("\\d{2}/\\d{2}/\\d{4}"),
      dateParser: (value) => {
        if (value == null || value === "") {
          return undefined;
        }
        const dateParts = value.split("/");
        return dateParts.length === 3
          ? new Date(
              parseInt(dateParts[2]),
              parseInt(dateParts[1]) - 1,
              parseInt(dateParts[0]),
            )
          : undefined;
      },
      dateFormatter: (value) => {
        if (value == null) {
          return undefined;
        }
        const date = String(value.getDate());
        const month = String(value.getMonth() + 1);
        return `${date.length === 1 ? "0" + date : date}/${month.length === 1 ? "0" + month : month}/${value.getFullYear()}`;
      },
    },
    dateTimeString: {
      baseDataType: "dateTimeString",
      extendsDataType: "dateTimeString",
      valueParser: (params) => {
        if (params.newValue != null && params.newValue.match(dateTimeRegex)) {
          return params.newValue;
        } else {
          return null;
        }
      },
      dateParser: (value) => {
        if (value == null) {
          return;
        }
        let [_, dd, MM, yyyy, HH, mm, ss] = (
          value.match(dateTimeRegex) || Array(7).fill("0")
        ).map((e) => e || "0");
        return new Date(
          parseInt(yyyy),
          parseInt(MM) - 1,
          parseInt(dd),
          parseInt(HH),
          parseInt(mm),
          parseInt(ss),
        );
      },
      dateFormatter: (value) => {
        // convert to `HH:mm:ss dd/MM/yyyy`
        return value == null
          ? ""
          : `${pad(value.getDate())}/${pad(value.getMonth() + 1)}/${value.getFullYear()}` +
              " " +
              `${pad(value.getHours())}:${pad(value.getMinutes())}:${pad(value.getSeconds())}`;
      },
    },
  },
};

// setup the grid after the page has finished loading
document.addEventListener("DOMContentLoaded", () => {
  const gridDiv = document.querySelector("#myGrid");
  gridApi = agGrid.createGrid(gridDiv, gridOptions);

  fetch("https://www.ag-grid.com/example-assets/olympic-winners.json")
    .then((response) => response.json())
    .then((data) =>
      gridApi.setGridOption(
        "rowData",
        data.map((d) => ({
          ...d,
          dateTimeWithSpace: `${d.date} ${pad(rand(0, 23))}:${pad(rand(0, 59))}:${pad(rand(0, 59))}`,
        })),
      ),
    );
});
 Preview
Copy Link
New Tab
CodeSandbox
Plunker
Date and DateTime as String Data Type Definition
Copy Link
If overriding 'dateString' or 'dateTimeString'' with a different date format, then a couple of extra properties need to be set to handle conversion between Date objects and the desired string format.

dateParser
Copy Link

Function
Converts a date in string format to a Date.
dateFormatter
Copy Link

Function
Converts a date in Date format to a string.
