Context Menu

javascript logoJavaScript
Enterprise
The user can bring up the context menu by right clicking on a cell. By default, the context menu shows Clipboard, CSV Export, Excel Export and Integrated Charts menu items (if the relevant Modules are loaded).

Code
Copy Link
New Tab
CodeSandbox
Plunker
Configuring the Context Menu
Copy Link
You can customise the context menu in one of two ways:

Set colDef.contextMenuItems. This can either be a list of menu items, or a callback which is passed the list of default menu items.
Set the grid option getContextMenuItems(). This callback will be passed the list of default menu items as well as the column.
Note that colDef.contextMenuItems will take priority over getContextMenuItems().

The menu item list should be a list with each item either a) a string or b) a MenuItemDef description. Use 'string' to pick from built-in menu items (listed below) and use MenuItemDef descriptions for your own menu items.

contextMenuItems
Copy Link

(DefaultMenuItem | MenuItemDef)[] | GetContextMenuItems
Customise the list of menu items available in the context menu.
ContextMenuModule
getContextMenuItems
Copy Link

GetContextMenuItems
For customising the context menu.
ContextMenuModule
You can also provide your own Custom Menu Item Components.

If you want to access your underlying data item, you can access that through the rowNode as node.data.

Note: if you set checked=true, then the icon will be ignored - these options are mutually exclusive.

If you want to turn off the context menu completely, set the grid property suppressContextMenu=true.

Built In Menu Items
Copy Link
The following is a list of all the default built in menu items with the rules about when they are shown.

autoSizeAll: Auto-size all columns. Not shown by default.
expandAll: When set, it's only shown if grouping by at least one column. Not shown by default.
contractAll: Collapse all groups. When set, it's only shown if grouping by at least one column. Not shown by default.
copy: Copy selected value to clipboard. Shown by default.
copyWithHeaders: Copy selected value to clipboard with headers. Shown by default.
copyWithGroupHeaders: Copy selected value to clipboard with headers and header groups. Shown by default.
cut: Cut the selected value to clipboard. Not shown by default.
paste: Paste the clipboard value into the selected cell (see note below). Shown by default.
resetColumns: Reset all columns. Not shown by default.
export: Export sub menu (containing csvExport and excelExport). Shown by default.
csvExport: Export to CSV using all default export values. Shown by default.
excelExport: Export to Excel (.xlsx) using all default export values. Shown by default.
chartRange: Chart a range of selected cells. Only shown if charting is enabled. Configured via chartToolPanelsDef.
pivotChart: Chart all grouped and pivoted data from the grid. Only shown if charting is enabled and in Pivot Mode. Configured via chartToolPanelsDef.
pinRowSubMenu: Row pinning sub menu (containing pinTop, pinBottom and unpinRow). Shown when Row Pinning is enabled.
pinTop: Pin a row to the top of the grid. Shown when Row Pinning is enabled.
pinBottom: Pin a row to the botom of the grid. Shown when Row Pinning is enabled.
unpinRow: Unpin a row from the top or bottom of the grid. Shown for pinned rows when Row Pinning is enabled.
Menu items also require the relevant Module to be loaded in order to be displayed.

The 'paste' operation in the context menu uses the Clipboard API. This means that if your app is running inside an iframe, or if the Clipboard API is blocked, only blank data will be pasted into the cell.

When the Clipboard API is not available, browser security restrictions prevent JavaScript from reading clipboard data without the user explicitly performing a paste action (e.g. ^ Ctrl+V or selecting Paste from the browser menu). These restrictions exist to prevent malicious websites from stealing clipboard data.

This paste option will be disabled if suppressClipboardApi={true} or if the target cell is not editable.

Default Context Menu
Copy Link
One drawback of using the AG Grid context menu is that you may want to show the browser's context menu when debugging, for example in order to access your browser's dev tools. If you want the grid to do nothing (and hence allow the browser to display its context menu) then hold down the ^ Ctrl key while clicking for the context menu. If you always want the grid's context menu, even when ^ Ctrl is pressed, then set allowContextMenuWithControlKey=true.

Hiding the Context Menu
Copy Link
Hide the context menu with the grid API hidePopupMenu(), which will hide either the context menu or the Column Menu, whichever is showing.

Context Menu Example
Copy Link
Below shows a configured context menu in action demonstrating a customised menu with a mix of custom items. You should notice the following:

A mix of built in items and custom items are used.
The first item uses the contents of the cell to display its value. Clicking on it logs the cell data to the developer console.
The Country column uses a Promise to return the menu items asynchronously.
Country and Person are sub menus. The country sub menu contains icons.
The top menu item has CSS classes applied to it.
The 'Always Disabled' menu item has a tooltip.

Console
Clear
Console logs from the example shown here...
Code
Copy Link
New Tab
CodeSandbox
Plunker
Popup Parent
Copy Link
Under most scenarios, the menu will fit inside the grid. However if the grid is small and / or the menu is very large, then the menu will not fit inside the grid and it will be clipped.

This will lead to a bad user experience which is demonstrated in the following example:

Open the context menu or the column menu in the grid
Notice the menu will not be fully visible (i.e. clipped)

Code
Copy Link
New Tab
CodeSandbox
Plunker
The solution is to set the popupParent element:

popupParent
Copy Link
HTMLElement | null
DOM element to use as the popup parent for grid popups (context menu, column menu etc).
Each mechanism allows you to set the popup parent to any HTML DOM element. The element must:

Exist in the DOM.
Cover the same area as the grid (or simply be a parent of the grid), so that when the popup is positioned, it can be positioned over the grid.
Most of the time, you will simply set the popup parent to the document body.

The example below is identical to the previous example except it sets the popup parent to the document body.

Code
Copy Link
New Tab
CodeSandbox
Plunker
Context Menu API / Events
Copy Link
The gridApi has the following methods that can be used to interact with the context menu:

showContextMenu
Copy Link

Function
Displays the AG Grid context menu
ContextMenuModule
hidePopupMenu
Copy Link

Function
Hides any visible Context Menu or Column Menu.

The following context menu event is emitted by the grid:

contextMenuVisibleChanged
Copy Link

ContextMenuVisibleChangedEvent
The context menu visibility has changed (opened or closed).
