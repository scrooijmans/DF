Cell Editing

javascript logoJavaScript
Enable Editing
Copy Link
To enable Cell Editing for a Column use the editable property on the Column Definition.

editable
Copy Link

boolean | EditableCallback
default: false
Set to true if this column is editable, otherwise false. Can also be a function to have different rows editable.
const gridOptions = {
columnDefs: [
{
field: 'athlete',
// enables editing
editable: true
}
],

    // other grid options ...

}
By default, the grid uses Cell Data Types to provide different editors based on the type of each column. For example, string columns will use a text input, number columns will use a numeric input.

The example below shows editing enabled on all columns by setting editable=true on the defaultColDef.

Code
Copy Link
New Tab
CodeSandbox
Plunker
Conditional Editing
Copy Link
To dynamically determine which cells are editable, a callback function can be supplied to the editable property on the Column Definition:

const gridOptions = {
columnDefs: [
{
field: 'athlete',
// conditionally enables editing for data for 2012
editable: (params) => params.data.year == 2012
}
],

    // other grid options ...

}
In the snippet above, Athlete cells will be editable on rows where the Year is 2012.

This is demonstrated in the following example, note that:

An editable callback is added to the Athlete and Age columns to control which cells are editable based on the selected Year.
A custom editableColumn Column Type is used to avoid duplication of the callback for Athlete and Age.
Buttons are provided to change the Year used by the editable callback function to control which cells are editable.
A blue Cell Style has been added to highlight editable cells using the same logic as the editable callback.

Code
Copy Link
New Tab
CodeSandbox
Plunker
Editing Events
Copy Link
Cell editing results in the following events.

cellValueChanged
Copy Link

CellValueChangedEvent
Cell value has changed. This occurs after the following scenarios:
Editing. Will not fire if any of the following are true:
new value is the same as old value; readOnlyEdit = true; editing was cancelled (e.g. Escape key was pressed); or new value is of the wrong cell data type for the column.
Cut.
Paste.
Cell clear (pressing Delete key).
Fill handle.
Copy range down.
Undo and redo.
Editing Events
cellEditRequest
Copy Link

CellEditRequestEvent
Value has changed after editing. Only fires when readOnlyEdit=true.
Read Only Edit
rowValueChanged
Copy Link

RowValueChangedEvent
A cell's value within a row has changed. This event corresponds to Full Row Editing only.
Full Row Editing
cellEditingStarted
Copy Link

CellEditingStartedEvent
Editing a cell has started.
Editing Events
cellEditingStopped
Copy Link

CellEditingStoppedEvent
Editing a cell has stopped.
Editing Events
rowEditingStarted
Copy Link

RowEditingStartedEvent
Editing a row has started (when row editing is enabled). When row editing, this event will be fired once and cellEditingStarted will be fired for each individual cell. Only fires when doing Full Row Editing.
Full Row Editing
rowEditingStopped
Copy Link

RowEditingStoppedEvent
Editing a row has stopped (when row editing is enabled). When row editing, this event will be fired once and cellEditingStopped will be fired for each individual cell. Only fires when doing Full Row Editing.
Full Row Editing
Cell Editing
