Column Options Reference

javascript logoJavaScript
Configuration for columns ColDef<TData, TValue> and column groups ColGroupDef<TData>.

Columns
field
Copy Link
ColDefField
The field of the row object to get the cell's data from. Deep references into a row object is supported via dot notation, i.e 'address.firstLine'.
Accessing Row Data Values
colId
Copy Link
string
The unique ID to give the column. This is optional. If missing, the ID will default to the field. If both field and colId are missing, a unique ID will be generated. This ID is used to identify the column in the API for sorting, filtering etc.
type
Copy Link
string | string[]
A comma separated string or array of strings containing ColumnType keys which can be used as a template for a column. This helps to reduce duplication of properties when you have a lot of common column properties.
Column Types
cellDataType
Copy Link
boolean | string
default: true
The data type of the cell values for this column. Can either infer the data type from the row data (true - the default behaviour), define a specific data type (string), or have no data type (false). If setting a specific data type (string value), this can either be one of the pre-defined data types 'text', 'number', 'boolean', 'date', 'dateString' or 'object', or a custom data type that has been defined in the dataTypeDefinitions grid option. Data type inference only works for the Client-Side Row Model, and requires non-null data. It will also not work if the valueGetter, valueParser or refData properties are defined, or if this column is a sparkline.
Cell Data Types
valueGetter
Copy Link

string | ValueGetterFunc
Function or expression. Gets the value from your data for display.
Value Getters
valueFormatter
Copy Link

string | ValueFormatterFunc
A function or expression to format a value, should return a string.
Value Formatters
refData
Copy Link
{ [key: string]: string }
Provided a reference data map to be used to map column values to their respective value from the map.
Using the 'refData' Property
keyCreator
Copy Link

Function
Function to return a string key for a value. This string is used for grouping, Set filtering, and searching within cell editor dropdowns. When filtering and searching the string is exposed to the user, so make sure to return a human-readable value.
equals
Copy Link

Function
Custom comparator for values, used by renderer to know if values have changed. Cells whose values have not changed don't get refreshed. By default the grid uses === which should work for most use cases.
Change Detection Comparing Values
toolPanelClass
Copy Link

ToolPanelClass
CSS class to use for the tool panel cell. Can be a string, array of strings, or function.
ColumnsToolPanelModule
suppressColumnsToolPanel
Copy Link
boolean
default: false
Set to true if you do not want this column or group to appear in the Columns Tool Panel.
ColumnsToolPanelModule
columnGroupShow
Copy Link

ColumnGroupShowType
Whether to only show the column when the group is open / closed. If not set the column is always displayed as part of the group.
icons
Copy Link
{ [key: string]: ((...args: any[]) => any) | string }
Initial
Icons to use inside the column instead of the grid's default icons. Leave undefined to use defaults.
Custom Icons
suppressNavigable
Copy Link

boolean | SuppressNavigableCallback
default: false
Set to true if this column is not navigable (i.e. cannot be tabbed into), otherwise false. Can also be a callback function to have different rows navigable.
suppressKeyboardEvent
Copy Link

Function
default: false
Allows the user to suppress certain keyboard events in the grid cell.
Suppress Keyboard Events
suppressPaste
Copy Link

boolean | SuppressPasteCallback
Pasting is on by default as long as cells are editable (non-editable cells cannot be modified, even with a paste operation). Set to true turn paste operations off.
suppressFillHandle
Copy Link
boolean
Set to true to prevent the fillHandle from being rendered in any cell that belongs to this column
Suppressing the Fill Handle
contextMenuItems
Copy Link

(DefaultMenuItem | MenuItemDef)[] | GetContextMenuItems
Customise the list of menu items available in the context menu.
Configuring the Context Menu
ContextMenuModule
context
Copy Link
any
Context property that can be used to associate arbitrary application data with this column definition.
Columns: Accessibility
cellAriaRole
Copy Link
string
default: 'gridcell'
Used for screen reader announcements - the role property of the cells that belong to this column.
Columns: Aggregation
(Enterprise only) See Aggregation

aggFunc
Copy Link

string | IAggFunc | null
Name of function to use for aggregation. In-built options are: sum, min, max, count, avg, first, last. Also accepts a custom aggregation name or an aggregation function.
Enabling Aggregation
RowGroupingModule
+3
initialAggFunc
Copy Link

string | IAggFunc
Initial
Same as aggFunc, except only applied when creating a new column. Not applied when updating column definitions.
RowGroupingModule
+3
enableValue
Copy Link
boolean
default: false
Set to true if you want to be able to aggregate by this column via the GUI. This will not block the API or properties being used to achieve aggregation.
Configuring via the UI
RowGroupingModule
+3
allowedAggFuncs
Copy Link
string[]
Aggregation functions allowed on this column e.g. ['sum', 'avg']. If missing, all installed functions are allowed. This will only restrict what the GUI allows a user to select, it does not impact when you set a function via the API.
Allowed Functions
RowGroupingModule
+3
defaultAggFunc
Copy Link
string
default: 'sum'
The name of the aggregation function to use for this column when it is enabled via the GUI. Note that this does not immediately apply the aggregation function like aggFunc
Default Function
RowGroupingModule
+3
Columns: Display
hide
Copy Link
boolean | null
default: false
Set to true for this column to be hidden.
initialHide
Copy Link
boolean
Initial
Same as hide, except only applied when creating a new column. Not applied when updating column definitions.
lockVisible
Copy Link
boolean
default: false
Set to true to block making column visible / hidden via the UI (API will still work).
lockPosition
Copy Link
boolean | 'left' | 'right'
Lock a column to position to 'left' or'right' to always have this column displayed in that position. true is treated as 'left'
suppressMovable
Copy Link
boolean
default: false
Set to true if you do not want this column to be movable via dragging.
useValueFormatterForExport
Copy Link
boolean
default: true
By default, values are formatted using the column's valueFormatter when exporting data from the grid. This applies to CSV and Excel export, as well as clipboard operations and the fill handle. Set to false to prevent values from being formatted for these operations. Regardless of this option, if custom handling is provided for the export operation, the value formatter will not be used.
Using Value Formatters with Other Grid Features
Columns: Editing
See Cell Editing for more information.

editable
Copy Link

boolean | EditableCallback
default: false
Set to true if this column is editable, otherwise false. Can also be a function to have different rows editable.
valueSetter
Copy Link

string | ValueSetterFunc
Function or expression. Custom function to modify your data based off the new value for saving. Return true if the data changed.
Saving Values
valueParser
Copy Link

string | ValueParserFunc
Function or expression. Parses the value for saving.
Parsing Values
cellEditor
Copy Link
any
Provide your own cell editor component for this column's cells.
Cell Editors
cellEditorParams
Copy Link
any
Params to be passed to the cellEditor component.
cellEditorSelector
Copy Link

CellEditorSelectorFunc
Callback to select which cell editor to be used for a given row within the same column.
Many Editors One Column
cellEditorPopup
Copy Link
boolean
Set to true, to have the cell editor appear in a popup.
cellEditorPopupPosition
Copy Link
'over' | 'under'
default: 'over'
Set the position for the popup cell editor. Possible values are
over Popup will be positioned over the cell
under Popup will be positioned below the cell leaving the cell value visible.
singleClickEdit
Copy Link
boolean
default: false
Set to true to have cells under this column enter edit mode after single click.
useValueParserForImport
Copy Link
boolean
default: true
By default, values are parsed using the column's valueParser when importing data to the grid. This applies to clipboard operations and the fill handle. Set to false to prevent values from being parsed for these operations. Regardless of this option, if custom handling is provided for the import operation, the value parser will not be used.
Using Value Parsers with Other Grid Features
Columns: Events
onCellValueChanged
Copy Link

NewValueParams
Callback for after the value of a cell has changed, either due to editing or the application calling api.setValue().
onCellClicked
Copy Link

CellClickedEvent
Callback called when a cell is clicked.
onCellDoubleClicked
Copy Link

CellDoubleClickedEvent
Callback called when a cell is double clicked.
onCellContextMenu
Copy Link

CellContextMenuEvent
Callback called when a cell is right clicked.
Columns: Filter
See Filtering for more information.

filter
Copy Link
any
Filter component to use for this column.
Set to true to use the default filter.
Set to the name of a Provided Filter or set to a IFilterComp.
Column Filters
filterParams
Copy Link
any
Params to be passed to the filter component specified in filter.
Filter Parameters
filterValueGetter
Copy Link

string | ValueGetterFunc
Function or expression. Gets the value for filtering purposes.
getQuickFilterText
Copy Link

Function
A function to tell the grid what Quick Filter text to use for this column if you don't want to use the default (which is calling toString on the value).
Overriding the Quick Filter Value
QuickFilterModule
floatingFilter
Copy Link
boolean
default: false
Whether to display a floating filter for this column.
Floating Filter
floatingFilterComponent
Copy Link
any
The custom component to be used for rendering the floating filter. If none is specified the default AG Grid is used.
Floating Filter Component
floatingFilterComponentParams
Copy Link
any
Params to be passed to floatingFilterComponent.
Floating Filter Parameters
suppressFiltersToolPanel
Copy Link
boolean
default: false
Set to true if you do not want this column (filter) or group (filter group) to appear in the Filters Tool Panel.
ColumnsToolPanelModule
dateComponent
Copy Link
any
Custom date selection component to be used in Date Filters and Date Floating Filters for this column.
Custom Selection Component
dateComponentParams
Copy Link
any
The parameters to be passed to the dateComponent.
Custom Selection Component
Columns: Find
getFindText
Copy Link

GetFindTextFunc
When using Find with custom cell renderers, this allows providing a custom value to search within. E.g. if the cell renderer is displaying text that is different from the cell formatted value. Returning null means Find will not search within the cell.
Using Find with Cell Components
FindModule
Columns: Header
See Column Headers for more information.

headerName
Copy Link
string
The name to render in the column header. If not specified and field is specified, the field name will be used as the header name.
headerValueGetter
Copy Link

string | HeaderValueGetterFunc
Function or expression. Gets the value for display in the header.
headerTooltip
Copy Link
string
Tooltip for the column header
TooltipModule
headerStyle
Copy Link

HeaderStyle | HeaderStyleFunc
An object of CSS values / or function returning an object of CSS values for a particular header.
headerClass
Copy Link

HeaderClass
CSS class to use for the header cell. Can be a string, array of strings, or function.
headerComponent
Copy Link
any
The custom header group component to be used for rendering the component header. If none specified the default AG Grid is used.
Header Component
headerComponentParams
Copy Link
any
The parameters to be passed to the headerComponent.
wrapHeaderText
Copy Link
boolean
If enabled then column header names that are too long for the column width will wrap onto the next line. Default false
autoHeaderHeight
Copy Link
boolean
default: false
If enabled then the column header row will automatically adjust height to accommodate the size of the header cell. This can be useful when using your own headerComponent or long header names in conjunction with wrapHeaderText.
Auto Header Height
menuTabs
Copy Link

ColumnMenuTab[]
Set to an array containing zero, one or many of the following options: 'filterMenuTab' | 'generalMenuTab' | 'columnsMenuTab'. This is used to figure out which menu tabs are present and in which order the tabs are shown.
Legacy Tabbed Column Menu
columnChooserParams
Copy Link

ColumnChooserParams
Params used to change the behaviour and appearance of the Column Chooser/Columns Menu tab.
Customising the Column Chooser
ColumnMenuModule
mainMenuItems
Copy Link

(DefaultMenuItem | MenuItemDef)[] | GetMainMenuItems
Customise the list of menu items available in the column menu.
Customising the Menu Items
ColumnMenuModule
suppressHeaderMenuButton
Copy Link
boolean
default: false
Set to true if no menu button should be shown for this column header.
Customising the Column Menu
suppressHeaderFilterButton
Copy Link
boolean
default: false
Set to true to not display the filter button in the column header. Doesn't apply when columnMenu = 'legacy'.
Customising the Column Menu
suppressHeaderContextMenu
Copy Link
boolean
default: false
Set to true to not display the column menu when the column header is right-clicked. Doesn't apply when columnMenu = 'legacy'.
Customising the Column Menu
suppressHeaderKeyboardEvent
Copy Link

Function
Suppress the grid taking action for the relevant keyboard event when a header is focused.
Suppress Keyboard Events
suppressFloatingFilterButton
Copy Link
boolean
If true, the button in the floating filter that opens the parent filter in a popup will not be displayed. Only applies if floatingFilter = true.
Floating Filters
Columns: Integrated Charts
(Enterprise only) See Integrated Charts

chartDataType
Copy Link
'category' | 'series' | 'time' | 'excluded'
Defines the chart data type that should be used for a column.
IntegratedChartsModule
Columns: Pinned
See Column Pinning for more information.

pinned
Copy Link
boolean | 'left' | 'right' | null
Pin a column to one side: right or left. A value of true is converted to 'left'.
initialPinned
Copy Link
boolean | 'left' | 'right'
Initial
Same as pinned, except only applied when creating a new column. Not applied when updating column definitions.
lockPinned
Copy Link
boolean
default: false
Set to true to block the user pinning the column, the column can only be pinned via definitions or API.
Columns: Pivoting
(Enterprise only) See Pivoting

pivot
Copy Link
boolean | null
Set to true to pivot by this column.
Enabling Pivoting
PivotModule
initialPivot
Copy Link
boolean
Initial
Same as pivot, except only applied when creating a new column. Not applied when updating column definitions.
PivotModule
pivotIndex
Copy Link
number | null
Set this in columns you want to pivot by. If only pivoting by one column, set this to any number (e.g. 0). If pivoting by multiple columns, set this to where you want this column to be in the order of pivots (e.g. 0 for first, 1 for second, and so on).
PivotModule
initialPivotIndex
Copy Link
number
Initial
Same as pivotIndex, except only applied when creating a new column. Not applied when updating column definitions.
PivotModule
enablePivot
Copy Link
boolean
default: false
Set to true if you want to be able to pivot by this column via the GUI. This will not block the API or properties being used to achieve pivot.
Configuring via the UI
PivotModule
pivotComparator
Copy Link

Function
Initial
Only for CSRM, see SSRM Pivoting. Comparator to use when ordering the pivot columns, when this column is used to pivot on. The values will always be strings, as the pivot service uses strings as keys for the pivot groups.
Ordering Pivot Result Groups
PivotModule
Columns: Rendering and Styling
cellStyle
Copy Link

CellStyle | CellStyleFunc
An object of CSS values / or function returning an object of CSS values for a particular cell.
Cell Style
CellStyleModule
cellClass
Copy Link

string | string[] | CellClassFunc
Class to use for the cell. Can be string, array of strings, or function that returns a string or array of strings.
Cell Class
CellStyleModule
cellClassRules
Copy Link

CellClassRules
Rules which can be applied to include certain CSS classes.
Cell Class Rules
CellStyleModule
cellRenderer
Copy Link
any
Provide your own cell Renderer component for this column's cells.
Cell Renderer
cellRendererParams
Copy Link
any
Params to be passed to the cellRenderer component.
Cell Renderer Params
cellRendererSelector
Copy Link

CellRendererSelectorFunc
Callback to select which cell renderer to be used for a given row within the same column.
Many Renderers One Column
loadingCellRenderer
Copy Link
any
Provide your own cell loading Renderer component for this column's cells when using SSRM or when a cell renderer is deferred.
SSRM Cell Loading
loadingCellRendererParams
Copy Link
any
Params to be passed to the loadingCellRenderer component.
SSRM Cell Loading
loadingCellRendererSelector
Copy Link

ILoadingCellRendererSelectorFunc
Callback to select which loading renderer to be used.
SSRM Cell Loading
autoHeight
Copy Link
boolean
default: false
Set to true to have the grid calculate the height of a row based on contents of this column.
Auto Row Height
RowAutoHeightModule
wrapText
Copy Link
boolean
default: false
Set to true to have the text wrap inside the cell - typically used with autoHeight.
Text Wrapping and Displaying Multi-Line Text
enableCellChangeFlash
Copy Link
boolean
default: false
Set to true to flash a cell when it's refreshed.
HighlightChangesModule
Columns: Row Dragging
See Row Dragging for more information.

rowDrag
Copy Link

boolean | RowDragCallback
default: false
boolean or Function. Set to true (or return true from function) to allow row dragging.
RowDragModule
rowDragText
Copy Link

Function
A callback that should return a string to be displayed by the rowDragComp while dragging a row. If this callback is not set, the rowDragText callback in the gridOptions will be used and if there is no callback in the gridOptions the current cell value will be used.
RowDragModule
dndSource
Copy Link

boolean | DndSourceCallback
default: false
boolean or Function. Set to true (or return true from function) to allow dragging for native drag and drop.
DragAndDropModule
dndSourceOnRowDrag
Copy Link

Function
Function to allow custom drag functionality for native drag and drop.
DragAndDropModule
Columns: Row Grouping
(Enterprise only) See Row Grouping

rowGroup
Copy Link
boolean | null
default: false
Set to true to row group by this column.
RowGroupingModule
initialRowGroup
Copy Link
boolean
Initial
Same as rowGroup, except only applied when creating a new column. Not applied when updating column definitions.
RowGroupingModule
rowGroupIndex
Copy Link
number | null
Set this in columns you want to group by. If only grouping by one column, set this to any number (e.g. 0). If grouping by multiple columns, set this to where you want this column to be in the group (e.g. 0 for first, 1 for second, and so on).
RowGroupingModule
initialRowGroupIndex
Copy Link
number
Initial
Same as rowGroupIndex, except only applied when creating a new column. Not applied when updating column definitions.
RowGroupingModule
enableRowGroup
Copy Link
boolean
default: false
Set to true if you want to be able to row group by this column via the GUI. This will not block the API or properties being used to achieve row grouping.
RowGroupingModule
showRowGroup
Copy Link
string | boolean
Initial
Set to true to have the grid place the values for the group into the cell, or put the name of a grouped column to just show that group.
Custom Group Columns
RowGroupingModule
rowGroupingHierarchy
Copy Link

(GroupHierarchyParts | string | ColDef)[]
rowGroupingHierarchy
Grouping by Dates and Times
RowGroupingModule
+1
Columns: Sort
See Row Sorting for more information.

sortable
Copy Link
boolean
default: true
Set to false to disable sorting which is enabled by default.
sort
Copy Link

SortDirection
If sorting by default, set it here. Set to asc or desc.
initialSort
Copy Link

SortDirection
Initial
Same as sort, except only applied when creating a new column. Not applied when updating column definitions.
sortIndex
Copy Link
number | null
If sorting more than one column by default, specifies order in which the sorting should be applied.
initialSortIndex
Copy Link
number
Initial
Same as sortIndex, except only applied when creating a new column. Not applied when updating column definitions.
sortingOrder
Copy Link

SortDirection[]
Array defining the order in which sorting occurs (if sorting is enabled). An array with any of the following in any order ['asc','desc',null]
comparator
Copy Link

Function
Override the default sorting order by providing a custom sort comparator.
valueA, valueB are the values to compare.
nodeA, nodeB are the corresponding RowNodes. Useful if additional details are required by the sort.
isDescending - true if sort direction is desc. Not to be used for inverting the return value as the grid already applies asc or desc ordering.
Return:
0 valueA is the same as valueB

> 0 Sort valueA after valueB
> < 0 Sort valueA before valueB
> unSortIcon
> Copy Link
> boolean
> default: false
> Set to true if you want the unsorted icon to be shown when no sort is applied to this column.
> Columns: Spanning
> See Column / Row Spanning

colSpan
Copy Link

Function
By default, each cell will take up the width of one column. You can change this behaviour to allow cells to span multiple columns.
spanRows
Copy Link

boolean | ((params: SpanRowsParams) => boolean)
Set to true to automatically merge cells in this column with equal values. Provide a callback to specify custom merging logic.
Row Spanning
CellSpanModule
Columns: Tooltips
tooltipField
Copy Link

ColDefField
The field of the tooltip to apply to the cell.
TooltipModule
tooltipValueGetter
Copy Link

Function
Callback that should return the string to use for a tooltip, tooltipField takes precedence if set. If using a custom tooltipComponent you may return any custom value to be passed to your tooltip component.
Tooltip Component
TooltipModule
tooltipComponent
Copy Link
any
Provide your own tooltip component for the column.
Tooltip Component
TooltipModule
tooltipComponentParams
Copy Link
any
The params used to configure tooltipComponent.
TooltipModule
Columns: Width
See Column Sizing for more information.

width
Copy Link
number
Initial width in pixels for the cell. If no width or flex properties set, cell width will default to 200 pixels.
initialWidth
Copy Link
number
Initial
Same as width, except only applied when creating a new column. Not applied when updating column definitions.
minWidth
Copy Link
number
Minimum width in pixels for the cell.
maxWidth
Copy Link
number
Maximum width in pixels for the cell.
flex
Copy Link
number | null
Equivalent to flex-grow in CSS. When flex is set on one or more columns, any width value is ignored and instead the remaining free space in the grid is divided among flex columns in proportion to their flex value, so a column with flex: 2 will be twice the size as one with flex: 1.
Column Flex
initialFlex
Copy Link
number
Initial
Same as flex, except only applied when creating a new column. Not applied when updating column definitions.
resizable
Copy Link
boolean
default: true
Set to false to disable resizing which is enabled by default.
suppressSizeToFit
Copy Link
boolean
default: false
Set to true if you want this column's width to be fixed during 'size to fit' operations.
suppressAutoSize
Copy Link
boolean
default: false
Set to true if you do not want this column to be auto-resizable by double clicking it's edge.
Groups
For column groups, the property children is mandatory. When the grid sees children it knows it's a column group.

See Column Groups for more information.

children required
Copy Link
(ColDef | ColGroupDef)[]
A list containing a mix of columns and column groups.
groupId
Copy Link
string
The unique ID to give the column. This is optional. If missing, a unique ID will be generated. This ID is used to identify the column group in the API.
marryChildren
Copy Link
boolean
default: false
Set to true to keep columns in this group beside each other in the grid. Moving the columns outside of the group (and hence breaking the group) is not allowed.
openByDefault
Copy Link
boolean
default: false
Set to true if this group should be opened by default.
columnGroupShow
Copy Link

ColumnGroupShowType
Whether to only show the column when the group is open / closed. If not set the column is always displayed as part of the group.
toolPanelClass
Copy Link

ToolPanelClass
CSS class to use for the tool panel cell. Can be a string, array of strings, or function.
ColumnsToolPanelModule
suppressColumnsToolPanel
Copy Link
boolean
default: false
Set to true if you do not want this column or group to appear in the Columns Tool Panel.
ColumnsToolPanelModule
suppressFiltersToolPanel
Copy Link
boolean
default: false
Set to true if you do not want this column (filter) or group (filter group) to appear in the Filters Tool Panel.
ColumnsToolPanelModule
tooltipComponent
Copy Link
any
Provide your own tooltip component for the column group.
Tooltip Component
TooltipModule
tooltipComponentParams
Copy Link
any
The params used to configure tooltipComponent.
TooltipModule
context
Copy Link
any
Context property that can be used to associate arbitrary application data with this column definition.
Groups: Header
See Column Headers for more information.

headerName
Copy Link
string
The name to render in the column header. If not specified and field is specified, the field name will be used as the header name.
headerClass
Copy Link

HeaderClass
CSS class to use for the header cell. Can be a string, array of strings, or function.
headerTooltip
Copy Link
string
Tooltip for the column header
TooltipModule
autoHeaderHeight
Copy Link
boolean
default: false
If enabled then the column header row will automatically adjust height to accommodate the size of the header cell. This can be useful when using your own headerComponent or long header names in conjunction with wrapHeaderText.
Auto Header Height
headerGroupComponent
Copy Link
any
The custom header group component to be used for rendering the component header. If none specified the default AG Grid is used.
Header Group Component
headerGroupComponentParams
Copy Link
any
The params used to configure the headerGroupComponent.
suppressSpanHeaderHeight
Copy Link
boolean
default: false
Set to true if you don't want the column header for this column to span the whole height of the header container.
suppressStickyLabel
Copy Link
boolean
default: false
If true the label of the Column Group will not scroll alongside the grid to always remain visible.
