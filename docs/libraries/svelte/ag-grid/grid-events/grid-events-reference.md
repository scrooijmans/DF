Grid Events Reference

javascript logoJavaScript
To listen to events see Grid Events.

Accessories
toolPanelVisibleChanged
Copy Link

ToolPanelVisibleChangedEvent
The tool panel visibility has changed. Fires twice if switching between panels - once with the old panel and once with the new panel.
Tool Panel Events
toolPanelSizeChanged
Copy Link

ToolPanelSizeChangedEvent
The tool panel size has been changed.
Tool Panel Events
columnMenuVisibleChanged
Copy Link

ColumnMenuVisibleChangedEvent
The column menu visibility has changed. Fires twice if switching between tabs - once with the old tab and once with the new tab.
Column Menu API / Events
contextMenuVisibleChanged
Copy Link

ContextMenuVisibleChangedEvent
The context menu visibility has changed (opened or closed).
Context Menu API / Events
Clipboard
See Clipboard for more information.

cutStart
Copy Link

CutStartEvent
Cut operation has started.
Clipboard Events
cutEnd
Copy Link

CutEndEvent
Cut operation has ended.
Clipboard Events
pasteStart
Copy Link

PasteStartEvent
Paste operation has started.
Clipboard Events
pasteEnd
Copy Link

PasteEndEvent
Paste operation has ended.
Clipboard Events
Columns
columnVisible
Copy Link

ColumnVisibleEvent
A column, or group of columns, was hidden / shown.
columnPinned
Copy Link

ColumnPinnedEvent
A column, or group of columns, was pinned / unpinned.
columnResized
Copy Link

ColumnResizedEvent
A column was resized.
columnMoved
Copy Link

ColumnMovedEvent
A column was moved.
columnValueChanged
Copy Link

ColumnValueChangedEvent
A value column was added or removed.
columnPivotModeChanged
Copy Link

ColumnPivotModeChangedEvent
The pivot mode flag was changed.
columnPivotChanged
Copy Link

ColumnPivotChangedEvent
A pivot column was added, removed or order changed.
columnGroupOpened
Copy Link

ColumnGroupOpenedEvent
A column group was opened / closed.
newColumnsLoaded
Copy Link

NewColumnsLoadedEvent
User set new columns.
gridColumnsChanged
Copy Link

GridColumnsChangedEvent
The list of grid columns changed.
displayedColumnsChanged
Copy Link

DisplayedColumnsChangedEvent
The list of displayed columns changed. This can result from columns open / close, column move, pivot, group, etc.
virtualColumnsChanged
Copy Link

VirtualColumnsChangedEvent
The list of rendered columns changed (only columns in the visible scrolled viewport are rendered by default).
columnHeaderMouseOver
Copy Link

ColumnHeaderMouseOverEvent
A mouse cursor is initially moved over a column header.
columnHeaderMouseLeave
Copy Link

ColumnHeaderMouseLeaveEvent
A mouse cursor is moved out of a column header.
columnHeaderClicked
Copy Link

ColumnHeaderClickedEvent
A click is performed on a column header.
columnHeaderContextMenu
Copy Link

ColumnHeaderContextMenuEvent
A context menu action, such as right-click or context menu key press, is performed on a column header.
pivotMaxColumnsExceeded
Copy Link

PivotMaxColumnsExceededEvent
Exceeded the pivotMaxGeneratedColumns limit when generating columns.
columnsReset
Copy Link

ColumnsResetEvent
Columns have been reset to their default state as reflected by the colDefs.
Components
See Components for more information.

componentStateChanged
Copy Link

ComponentStateChangedEvent
Only used by Angular, React and VueJS AG Grid components (not used if doing plain JavaScript). If the grid receives changes due to bound properties, this event fires after the grid has finished processing the change.
Editing
See Cell Editing for more information.

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
undoStarted
Copy Link

UndoStartedEvent
Undo operation has started.
Undo / Redo Events
undoEnded
Copy Link

UndoEndedEvent
Undo operation has ended.
Undo / Redo Events
redoStarted
Copy Link

RedoStartedEvent
Redo operation has started.
Undo / Redo Events
redoEnded
Copy Link

RedoEndedEvent
Redo operation has ended.
Undo / Redo Events
cellSelectionDeleteStart
Copy Link

CellSelectionDeleteStartEvent
Cell selection delete operation (cell clear) has started.
Delete Cell Selection
cellSelectionDeleteEnd
Copy Link

CellSelectionDeleteEndEvent
Cell selection delete operation (cell clear) has ended.
Delete Cell Selection
Filtering
See Filtering for more information.

filterOpened
Copy Link

FilterOpenedEvent
Filter has been opened.
Filter Events
filterChanged
Copy Link

FilterChangedEvent
Filter has been modified and applied.
Filter Events
filterModified
Copy Link

FilterModifiedEvent
Filter was modified but not applied (when using enableFilterHandlers = false). Used when filters have 'Apply' buttons.
Filter Events
filterUiChanged
Copy Link

FilterUiChangedEvent
Filter UI was modified (when using enableFilterHandlers = true).
Filter Events
floatingFilterUiChanged
Copy Link

FloatingFilterUiChangedEvent
Floating filter UI modified (when using enableFilterHandlers = true).
Filter Events
advancedFilterBuilderVisibleChanged
Copy Link

AdvancedFilterBuilderVisibleChangedEvent
Advanced Filter Builder visibility has changed (opened or closed).
Advanced Filter
Find
See Find for more information.

findChanged
Copy Link

FindChangedEvent
Find details have changed (e.g. Find search value, active match, or updates to grid cells).
Find
Integrated Charts
See Integrated Charts Events for more information.

chartCreated
Copy Link

ChartCreatedEvent
A chart has been created.
Chart Created
chartRangeSelectionChanged
Copy Link

ChartRangeSelectionChangedEvent
The data range for the chart has been changed.
Chart Range Selection Changed
chartOptionsChanged
Copy Link

ChartOptionsChangedEvent
Formatting changes have been made by users through the Customize Panel.
Chart Options Changed
chartDestroyed
Copy Link

ChartDestroyedEvent
A chart has been destroyed.
Chart Destroyed
Keyboard Navigation
See Keyboard Navigation for more information.

cellKeyDown
Copy Link

CellKeyDownEvent | FullWidthCellKeyDownEvent
DOM event keyDown happened on a cell.
Keyboard Events
Miscellaneous
gridReady
Copy Link

GridReadyEvent
The grid has initialised and is ready for most api calls, but may not be fully rendered yet
Grid Ready
gridPreDestroyed
Copy Link

GridPreDestroyedEvent
Invoked immediately before the grid is destroyed. This is useful for cleanup logic that needs to run before the grid is torn down.
Grid Pre-Destroyed
firstDataRendered
Copy Link

FirstDataRenderedEvent
Fired the first time data is rendered into the grid. Use this event if you want to auto resize columns based on their contents
First Data Rendered
gridSizeChanged
Copy Link

GridSizeChangedEvent
The size of the grid div has changed. In other words, the grid was resized.
Grid Layout
modelUpdated
Copy Link

ModelUpdatedEvent
Displayed rows have changed. Triggered after sort, filter or tree expand / collapse events.
virtualRowRemoved
Copy Link

VirtualRowRemovedEvent
A row was removed from the DOM, for any reason. Use to clean up resources (if any) used by the row.
viewportChanged
Copy Link

ViewportChangedEvent
Which rows are rendered in the DOM has changed.
bodyScroll
Copy Link

BodyScrollEvent
The body was scrolled horizontally or vertically.
bodyScrollEnd
Copy Link

BodyScrollEndEvent
Main body of the grid has stopped scrolling, either horizontally or vertically.
dragStarted
Copy Link

DragStartedEvent
When dragging starts. This could be any action that uses the grid's Drag and Drop service, e.g. Column Moving, Column Resizing, Range Selection, Fill Handle, etc.
dragStopped
Copy Link

DragStoppedEvent
When dragging stops. This could be any action that uses the grid's Drag and Drop service, e.g. Column Moving, Column Resizing, Range Selection, Fill Handle, etc.
stateUpdated
Copy Link

StateUpdatedEvent
Grid state has been updated.
Grid State
Pagination
See Row Pagination for more information.

paginationChanged
Copy Link

PaginationChangedEvent
Triggered every time the paging state changes. Some of the most common scenarios for this event to be triggered are:

The page size changes
The current shown page is changed
New data is loaded onto the grid
Row Drag and Drop
See Row Dragging for more information.

rowDragEnter
Copy Link

RowDragEnterEvent
A drag has started, or dragging was already started and the mouse has re-entered the grid having previously left the grid.
Row Drag Events
rowDragMove
Copy Link

RowDragMoveEvent
The mouse has moved while dragging.
Row Drag Events
rowDragLeave
Copy Link

RowDragLeaveEvent
The mouse has left the grid while dragging.
Row Drag Events
rowDragEnd
Copy Link

RowDragEndEvent
The drag has finished over the grid.
Row Drag Events
rowDragCancel
Copy Link

RowDragCancelEvent
The drag has been cancelled over the grid.
Row Drag Events
Row Grouping
See Row Grouping for more information.

columnRowGroupChanged
Copy Link

ColumnRowGroupChangedEvent
A row group column was added, removed or reordered.
rowGroupOpened
Copy Link

RowGroupOpenedEvent
A row group was opened or closed.
expandOrCollapseAll
Copy Link

ExpandOrCollapseAllEvent
Fired when calling either of the API methods expandAll() or collapseAll().
Row Pinning
See Row Pinning for more information.

pinnedRowDataChanged
Copy Link

PinnedRowDataChangedEvent
The client has set new pinned row data into the grid.
pinnedRowsChanged
Copy Link

PinnedRowsChangedEvent
A row has been pinned to top or bottom, or unpinned.
RowModel: Client-Side
rowDataUpdated
Copy Link

RowDataUpdatedEvent
Client-Side Row Model only. The client has updated data for the grid by either a) setting new Row Data or b) Applying a Row Transaction.
Row Data Updated
asyncTransactionsFlushed
Copy Link

AsyncTransactionsFlushedEvent
Async transactions have been applied. Contains a list of all transaction results.
Flush Async Transactions
RowModel: Server-Side
See Server-Side Row Model for more information.

storeRefreshed
Copy Link

StoreRefreshedEvent
A server side store has finished refreshing.
SSRM Refresh
Selection
headerFocused
Copy Link

HeaderFocusedEvent
Header is focused.
cellClicked
Copy Link

CellClickedEvent
Cell is clicked.
cellDoubleClicked
Copy Link

CellDoubleClickedEvent
Cell is double clicked.
cellFocused
Copy Link

CellFocusedEvent
Cell is focused.
cellMouseOver
Copy Link

CellMouseOverEvent
Mouse entered cell.
cellMouseOut
Copy Link

CellMouseOutEvent
Mouse left cell.
cellMouseDown
Copy Link

CellMouseDownEvent
Mouse down on cell.
rowClicked
Copy Link

RowClickedEvent
Row is clicked.
rowDoubleClicked
Copy Link

RowDoubleClickedEvent
Row is double clicked.
rowSelected
Copy Link

RowSelectedEvent
Row is selected or deselected. The event contains the node in question, so call the node's isSelected() method to see if it was just selected or deselected.
Selection Events
selectionChanged
Copy Link

SelectionChangedEvent
Row selection is changed. Use the selectedNodes field to get the list of selected nodes at the time of the event. When using the SSRM, selectedNodes will be null when selecting all nodes. Instead, refer to the serverSideState field.
Selection Events
cellContextMenu
Copy Link

CellContextMenuEvent
Cell is right clicked.
cellSelectionChanged
Copy Link

CellSelectionChangedEvent
A change to cell selection has occurred.
Cell Selection Changed Event
fillStart
Copy Link

FillStartEvent
Fill operation has started.
Fill Range Event
fillEnd
Copy Link

FillEndEvent
Fill operation has ended.
Fill Range Event
Sorting
See Row Sorting for more information.

sortChanged
Copy Link

SortChangedEvent
Sort has changed. The grid also listens for this and updates the model.
Tooltips
See Tooltip Component for more information.

tooltipShow
Copy Link

TooltipShowEvent
A tooltip has been displayed
tooltipHide
Copy Link

TooltipHideEvent
A tooltip was hidden
