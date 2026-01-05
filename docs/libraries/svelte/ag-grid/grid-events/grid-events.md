Grid Overview

javascript logoJavaScript
This section provides key information for configuring and interacting with a grid.

Grid Options
Copy Link
The gridOptions object is used to configure the grid. The example below shows the different types of items available on gridOptions.

const gridOptions = {
// PROPERTIES
columnDefs: myColDefs,
pagination: true,

    // EVENTS
    onRowClicked: event => console.log('A row was clicked'),

    // CALLBACKS
    getRowHeight: (params) => 25

}

// Pass gridOptions to createGrid
const api = createGrid(gridDiv, gridOptions)
Updating Grid Options
Copy Link
It is a common requirement to update a grid option after the grid has been created. For example, you may want to change rowHeight via an application toggle.

Properties can be updated by calling either api.setGridOption or api.updateGridOptions. In this example all the rows will be redrawn with the new height.

// update the rowHeight
api.setGridOption('rowHeight', 50);
Initial Grid Options
Copy Link
A small number of Grid Options do not support updating their value. Their value is only read during the initial setup of the grid. These options are marked as Initial on the Options Reference. For these properties the grid must be destroyed and re-created for the new value to take effect.

Not all Grid Options support updates. These are marked as Initial.

For a full list of options see: Options Reference.

Global Grid Options
Copy Link
Global Grid Options can be used to share configuration across all grids in an application. Global grid options are provided by passing the global options to provideGlobalGridOptions. Each grid will inherit the global options with local options taking precedence if both define the same property.

import { provideGlobalGridOptions } from 'ag-grid-community';

// provide localeText to all grids via global options
provideGlobalGridOptions({
localeText: userLocaleText,
});
The provideGlobalGridOptions function takes an optional second parameter (deep / shallow) to control the behaviour when object configurations exists on both global and local grid options. With shallow an object property on the local grid options will completely replace the global object property. With deep the global object properties are merged with the local object properties. Default is shallow.

Grid Events
Copy Link
As a user interacts with the grid events will be fired to enable your application to respond to specific actions.

Register callbacks for events through the GridOptions interface. The name of the callback is constructed by prefixing the event name with on. For example, the callback for the cellClicked event is gridOptions.onCellClicked.

const gridOptions = {
// Add event handlers
onCellClicked: (event: CellClickedEvent) => console.log('Cell was clicked'),
}
Listening to Events
Copy Link
In addition to adding event listeners directly via the gridOptions object, it is possible to register for events, similar to registering for events on native DOM elements. This means there are two ways to listen for events: either use the onXXX() method on the gridOptions (where XXX is replaced with the event name), or register a listener for the event. The latter option allows you to add multiple handlers for the same event. The following example demonstrates the two options:

// create handler function
function myRowClickedHandler(event) {
console.log('The row was clicked');
}

// option 1: use the gridOptions
gridOptions.onRowClicked = myRowClickedHandler;

// option 2: register the handler
api.addEventListener('rowClicked', myRowClickedHandler);
TypeScript users can take advantage of the events' interfaces. Construct the interface name by suffixing the event name with Event. For example, the cellClicked event uses the interface CellClickedEvent. All events support TypeScript Generics.

For a full list of events see: Grid Events.

Grid API
Copy Link
You can access the grid api by storing a reference to the api as returned from createGrid.

// create the grid
const api = createGrid(div, gridOptions);

// Call an api method
const cell = api.getFocusedCell();
API within Events and Callbacks
Copy Link
Alternatively all Grid callbacks and events include the api as part of their arguments.

const gridOptions: GridOptions = {
onGridReady: (event: GridReadyEvent) {
// use api from event
event.api.ensureIndexVisible(10);
}
}
For a full list of api methods see: Grid API.

Grid State
Copy Link
As a user interacts with the grid they may change state such as filtering, sorting and column order. This state is independent of the configuration and to provide save and restore capabilities the grid enables applications to save / restore this state.

For a full list of the state properties see: Grid State.

Grid Lifecycle
Copy Link
When working with AG Grid it is a common requirement to perform actions when the grid is first initialised, when data is first rendered and when the grid is about to be destroyed.

For full details about how to interact with the grid at these key moments see: Grid Lifecycle.
