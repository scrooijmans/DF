Start/Stop Cell Editing

javascript logoJavaScript
This page discusses the different ways in which Cell Editing can be started and stopped.

Start Editing
Copy Link
Assuming editable=true or editable has a callback that returns true for the Column Definition, editing will start upon any of the following:

Edit Key Pressed: One of the following is pressed: ↵ Enter, F2.
Backspace: The default editor will start and clear the contents of the cell if Backspace is pressed on Windows. To mimic this behaviour on MacOS, use the enableCellEditingOnBackspace=true grid option.
Printable Key Pressed: Any of the following characters are pressed: abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890!"£$%^&amp;\*()\_+-=[];\'#,./\|<>?:@~{}
The default editor places this character into the edit field so that the user experience is they are typing into the cell.
Mouse Double Click: If the mouse is double-clicked. There is a grid property singleClickEdit that will allow single-click to start editing instead of double-click. Another property suppressClickEdit will prevent both single-click and double-click from starting the edit; use this if you only want to have your own way of starting editing, such as clicking a button in your custom cell renderer.
api.startEditingCell(params): If you call startEditingCell(params) on the grid API
Stop Editing
Copy Link
The grid will stop editing when any of the following happen:

Callback stopEditing: The callback stopEditing (from the params above) gets called by the editor. This is how your cell editor informs the grid to stop editing.
Other Cell Focus: If focus in the grid goes to another cell, the editing will stop.
Enter Key Down: If the grid receives an ↵ Enter key press event on the cell. If you do not want to stop editing when ↵ Enter is pressed, then listen for the event and stop propagation so the grid does not act on the event.
Escape Key Down: Similar to ↵ Enter, if ⎋ Esc key is pressed, editing will stop. Unlike ↵ Enter, the ⎋ Esc action will discard changes rather than taking the new value.
Tab Key Down: Editing will stop, accepting changes, and editing will move to the next cell, or the previous cell if ⇧ Shift is also pressed.
Popup Editor Closed: If using popup editor, the popup is configured to close if you click outside the editor. Closing the popup triggers the grid to stop editing.
gridApi.stopEditing(): If you call stopEditing() on the grid API.
Start / Stop Events
Copy Link
The following events are fired when editing is started and stopped.

cellEditingStarted
Copy Link

CellEditingStartedEvent
Editing a cell has started.
cellEditingStopped
Copy Link

CellEditingStoppedEvent
Editing a cell has stopped.
Tab Navigation
Copy Link
While editing, if you hit ⇥ Tab, the editing will stop for the current cell and start on the next cell. If you hold down ⇧ Shift+⇥ Tab, the same will happen except the previous cell will start editing rather than the next. This is in line with editing data in Excel.

The next and previous cells can also be navigated using the API functions api.tabToNextCell() and api.tabToPreviousCell(). Both of these methods will return true if the navigation was successful, otherwise false.

tabToNextCell
Copy Link

Function
Navigates the grid focus to the next cell, as if tabbing.
tabToPreviousCell
Copy Link

Function
Navigates the grid focus to the previous cell, as if shift-tabbing.
Editing API
Copy Link
The grid has the following API methods for editing:

startEditingCell
Copy Link

Function
Start editing the provided cell. If another cell is editing, the editing will be stopped in that other cell.
TextEditorModule
+7
stopEditing
Copy Link

Function
If a cell is editing, it stops the editing. Pass true if you want to cancel the editing (i.e. don't accept changes).
TextEditorModule
+7
getEditingCells
Copy Link

Function
If the grid is editing, returns back details of the editing cell(s).
TextEditorModule
+7
If the grid is editing, getEditingCells() returns back details of the editing cell(s). The result is an array of objects. If only one cell is editing (the default) then the array will have one entry. If multiple cells are editing (e.g. Full Row Edit) then the array contains all editing cells.

Below is a code example of using the editing API methods.

// start editing country cell on first row
api.startEditingCell({
rowIndex: 0,
colKey: 'country'
});

// stop editing
api.stopEditing();

// print details of editing cell
const cellDefs = api.getEditingCells();

cellDefs.forEach(cellDef => {
console.log(cellDef.rowIndex);
console.log(cellDef.column.getId());
console.log(cellDef.floating);
});
The example below illustrates different parts of the editing API. Each button starts editing the 'Last Name' column of the first row with the following differences:

edit(): Normal editing start.
edit(Backspace): Edit as if delete button was pressed (clears contents first).
edit('T'): Edit as if 'T' was pressed (places 'T' into cell).
edit(top): Edits top pinned row.
edit(bottom): Edits bottom pinned row.
The example also demonstrates the following buttons for edit navigation:

stop(): Stops editing.
next(): Edits the next cell.
previous(): Edits the previous cell.
Finally, the example also demonstrates querying which cell is editing:

which(): If the grid is editing, prints to the console which cell is in edit mode.
Events are logged in the developer console.

Console
Clear
Console logs from the example shown here...
Code
Copy Link
New Tab
CodeSandbox
Plunker
Enter Key Navigation
Copy Link
By default pressing ↵ Enter will start editing on a cell, or stop editing on an editing cell. It will not navigate to the cell below.

To allow consistency with Excel the grid has the following properties:

enterNavigatesVertically: Set to true to have ↵ Enter key move focus to the cell below if not editing. The default is ↵ Enter key starts editing the currently focused cell.
enterNavigatesVerticallyAfterEdit: Set to true to have ↵ Enter key move focus to the cell below after ↵ Enter is pressed while editing. The default is editing will stop and focus will remain on the editing cell.
The example below demonstrates the focus moving down when ↵ Enter is pressed.

Code
Copy Link
New Tab
CodeSandbox
Plunker
Single-Click Editing
Copy Link
The default is for the grid to enter editing when you Double-Click on a cell. To change the default so that a single-click starts editing, set the property gridOptions.singleClickEdit = true. This is useful when you want a cell to enter edit mode as soon as you click on it, similar to the experience you get when inside Excel.

It is also possible to define single-click editing on a per-column basis using colDef.singleClickEdit = true.

The grid below has singleClickEdit = true so that editing will start on a cell when you single-click on it.

Code
Copy Link
New Tab
CodeSandbox
Plunker
No-Click Editing
Copy Link
It is possible to configure the grid so neither Single-Click or Double-Click starts editing. To do this set the property suppressClickEdit=true. This is useful when you want to start the editing in another way, such as including a button in your cell renderer.

The grid below has suppressClickEdit = true so that clicking doesn't started editing. The grid configures a Cell Renderer with a button to start editing.

Code
Copy Link
New Tab
CodeSandbox
Plunker
Stop Editing When Grid Loses Focus
Copy Link
By default, the grid will not stop editing the currently editing cell when the cell loses focus, unless another cell is clicked on. This means clicking on the grid header, or another part of your application, will not stop editing. This can be bad if, for example, you have a save button, and you need the grid to stop editing before you execute your save function (e.g. you want to make sure the edit is saved into the grid's state).

If you want the grid to stop editing when focus leaves the cell or the grid, set the grid property stopEditingWhenCellsLoseFocus = true.

The example below shows the editing with stopEditingWhenCellsLoseFocus = true. Notice the following:

Double-click to start editing 'Age', then click outside the grid (on the 'Dummy Save' button, or the dummy text field) and the grid will stop editing.
Double-click to start editing 'Year', a custom popup editor appears, you can click anywhere on the popup editor, but once you click outside the editor, the popup closes.
