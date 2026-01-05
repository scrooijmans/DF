# The SvelteFlow component - Svelte Flow

[Source on Github](https://github.com/xyflow/xyflow/blob/main/packages/svelte/src/lib/container/SvelteFlow/SvelteFlow.svelte) 

The `<SvelteFlow />` component is the heart of your Svelte Flow application.

```
<script>
  import { SvelteFlow } from '@xyflow/svelte';
  import '@xyflow/svelte/dist/style.css';

  let nodes = $state.raw([
    { id: 'a', data: { label: 'node a' }, position: { x: 0, y: 0 } },
    { id: 'b', data: { label: 'node b' }, position: { x: 0, y: 100 } }
  ]);

  let edges = $state.raw([
    { id: 'e1-2', source: 'a', target: 'b' }
  ]);
</script>

<SvelteFlow
  bind:nodes
  bind:edges
  fitView
/>
```

This component takes a lot of different props, most of which are optional. We’ve tried to document them in groups that make sense to help you find your way.

## Common props[](#common-props)

These are the props you will most commonly use when working with Svelte Flow.

- Name: width
  - Type: numberSets a fixed width for the flow
  - Default:
- Name: height
  - Type: numberSets a fixed height for the flow
  - Default:
- Name: nodes
  - Type: Node[]An array of nodes to render in a flow.
  - Default:
- Name: edges
  - Type: Edge[]An array of edges to render in a flow.
  - Default:
- Name: nodeTypes
  - Type: NodeTypesCustom node types to be available in a flow.Svelte Flow matches a node’s type to a component in the nodeTypes object.
  - Default:
- Name: edgeTypes
  - Type: EdgeTypesCustom edge types to be available in a flow.Svelte Flow matches an edge’s type to a component in the edgeTypes object.
  - Default:
- Name: colorMode
  - Type: ColorModeControls color scheme used for styling the flow
  - Default: 'system'
- Name: colorModeSSR
  - Type: Omit<ColorMode, "system">Fallback color mode for SSR if colorMode is set to ‘system’
  - Default:
- Name: proOptions
  - Type: ProOptionsBy default, we render a small attribution in the corner of your flows that links back to the project.You are free to remove this attribution but we ask that you take a quick look at ourhttps://svelteflow.dev/learn/troubleshooting/remove-attribution  removing attribution guidebefore doing so.
  - Default:
- Name: ...props
  - Type: HTMLAttributes<HTMLDivElement>
  - Default:

## Viewport props[](#viewport-props)

- Name: viewport
  - Type: ViewportCustom viewport to be used instead of internal one
  - Default:
- Name: initialViewport
  - Type: ViewportSets the initial position and zoom of the viewport.If a default viewport is provided but fitView is enabled, the default viewport will be ignored.
  - Default: { zoom: 1, position: { x: 0, y: 0 } }
- Name: fitView
  - Type: booleanIf set, initial viewport will show all nodes & edges
  - Default:
- Name: fitViewOptions
  - Type: FitViewOptions<Node>Options to be used in combination with fitView
  - Default:
- Name: minZoom
  - Type: numberMinimum zoom level
  - Default: 0.5
- Name: maxZoom
  - Type: numberMaximum zoom level
  - Default: 2
- Name: snapGrid
  - Type: SnapGridGrid all nodes will snap to
  - Default:
- Name: onlyRenderVisibleElements
  - Type: booleanYou can enable this optimisation to instruct Svelte Flow to only render nodes and edges that would be visible in the viewport.This might improve performance when you have a large number of nodes and edges but also adds an overhead.
  - Default: false
- Name: translateExtent
  - Type: CoordinateExtentBy default the viewport extends infinitely. You can use this prop to set a boundary.The first pair of coordinates is the top left boundary and the second pair is the bottom right.
  - Default: [[-∞, -∞], [+∞, +∞]]
- Name: preventScrolling
  - Type: booleanDisabling this prop will allow the user to scroll the page even when their pointer is over the flow.
  - Default: true
- Name: attributionPosition
  - Type: PanelPositionSet position of the attribution
  - Default: 'bottom-right'

## Node props[](#node-props)

- Name: nodeOrigin
  - Type: NodeOriginDefines nodes relative position to its coordinates
  - Default: [0, 0]
- Name: nodesDraggable
  - Type: booleanControls if all nodes should be draggable
  - Default: true
- Name: nodesConnectable
  - Type: booleanControls if all nodes should be connectable to each other
  - Default: true
- Name: nodesFocusable
  - Type: booleanWhen true, focus between nodes can be cycled with the Tab key and selected with the Enterkey. This option can be overridden by individual nodes by setting their focusable prop.
  - Default: true
- Name: nodeDragThreshold
  - Type: numberWith a threshold greater than zero you can control the distinction between node drag and click events.If threshold equals 1, you need to drag the node 1 pixel before a drag event is fired.
  - Default: 1
- Name: nodeClickDistance
  - Type: numberDistance that the mouse can move between mousedown/up that will trigger a click
  - Default: 0
- Name: nodeExtent
  - Type: CoordinateExtentBy default the nodes can be placed anywhere. You can use this prop to set a boundary.The first pair of coordinates is the top left boundary and the second pair is the bottom right.
  - Default: [[-∞, -∞], [+∞, +∞]]
- Name: elevateNodesOnSelect
  - Type: booleanEnabling this option will raise the z-index of nodes when they are selected.
  - Default: true

## Edge props[](#edge-props)

- Name: edgesFocusable
  - Type: booleanWhen true, focus between edges can be cycled with the Tab key and selected with the Enterkey. This option can be overridden by individual edges by setting their focusable prop.
  - Default: true
- Name: elevateEdgesOnSelect
  - Type: booleanEnabling this option will raise the z-index of edges when they are selected,or when the connected nodes are selected.
  - Default: true
- Name: defaultMarkerColor
  - Type: stringColor of edge markers
  - Default:
- Name: defaultEdgeOptions
  - Type: DefaultEdgeOptionsDefaults to be applied to all new edges that are added to the flow.Properties on a new edge will override these defaults if they exist.
  - Default:

## Event handlers[](#event-handlers)

### General Events[](#general-events)

- Name: oninit
  - Type: () => voidThis handler gets called when the flow is finished initializing
  - Default:
- Name: onflowerror
  - Type: OnErrorOcassionally something may happen that causes Svelte Flow to throw an error.Instead of exploding your application, we log a message to the console and then call this event handler.You might use it for additional logging or to show a message to the user.
  - Default:
- Name: ondelete
  - Type: OnDelete<Node, Edge>This handler gets called when the user deletes nodes or edges.
  - Default:
- Name: onbeforedelete
  - Type: OnBeforeDelete<Node, Edge>This handler gets called before the user deletes nodes or edges and provides a way to abort the deletion by returning false.
  - Default:

### Node Events[](#node-events)

- Name: onnodeclick
  - Type: NodeEventWithPointer<MouseEvent | TouchEvent, Node>This event handler is called when a user clicks on a node.
  - Default:
- Name: onnodedragstart
  - Type: NodeTargetEventWithPointer<MouseEvent | TouchEvent, Node>This event handler is called when a user starts to drag a node.
  - Default:
- Name: onnodedrag
  - Type: NodeTargetEventWithPointer<MouseEvent | TouchEvent, Node>This event handler is called when a user drags a node.
  - Default:
- Name: onnodedragstop
  - Type: NodeTargetEventWithPointer<MouseEvent | TouchEvent, Node>This event handler is called when a user stops dragging a node.
  - Default:
- Name: onnodepointerenter
  - Type: NodeEventWithPointer<PointerEvent, Node>This event handler is called when the pointer of a user enters a node.
  - Default:
- Name: onnodepointermove
  - Type: NodeEventWithPointer<PointerEvent, Node>This event handler is called when the pointer of a user moves over a node.
  - Default:
- Name: onnodepointerleave
  - Type: NodeEventWithPointer<PointerEvent, Node>This event handler is called when the pointer of a user leaves a node.
  - Default:
- Name: onnodecontextmenu
  - Type: NodeEventWithPointer<MouseEvent, Node>This event handler is called when a user right-clicks on a node.
  - Default:

### Edge Events[](#edge-events)

- Name: onedgeclick
  - Type: ({ edge, event }: { edge: Edge; event: MouseEvent; }) => voidThis event handler is called when a user clicks an edge.
  - Default:
- Name: onedgecontextmenu
  - Type: ({ edge, event }: { edge: Edge; event: MouseEvent; }) => voidThis event handler is called when a user right-clicks an edge.
  - Default:
- Name: onedgepointerenter
  - Type: ({ edge, event }: { edge: Edge; event: PointerEvent; }) => voidThis event handler is called when the pointer of a user enters an edge.
  - Default:
- Name: onedgepointerleave
  - Type: ({ edge, event }: { edge: Edge; event: PointerEvent; }) => voidThis event handler is called when the pointer of a user enters an edge.
  - Default:
- Name: onreconnect
  - Type: OnReconnect<Edge>This event gets fired when after an edge was reconnected
  - Default:
- Name: onreconnectstart
  - Type: OnReconnectStart<Edge>This event gets fired when a user starts to reconnect an edge
  - Default:
- Name: onreconnectend
  - Type: OnReconnectEnd<Edge>This event gets fired when a user stops reconnecting an edge
  - Default:
- Name: onbeforereconnect
  - Type: OnBeforeReconnect<Edge>This handler gets called when an edge is reconnected. You can use it to modify the edge before the update is applied.
  - Default:

### Selection Events[](#selection-events)

- Name: onselectionchanged
  - Type: unknown
  - Default:
- Name: onselectionclick
  - Type: NodesEventWithPointer<MouseEvent, Node>This event handler is called when a user clicks the selection box.
  - Default:
- Name: onselectioncontextmenu
  - Type: NodesEventWithPointer<MouseEvent, Node>This event handler is called when a user right-clicks the selection box.
  - Default:
- Name: onselectiondragstart
  - Type: OnSelectionDrag<Node>This event handler gets called when a user starts to drag a selection box.
  - Default:
- Name: onselectiondrag
  - Type: OnSelectionDrag<Node>This event handler gets called when a user drags a selection box.
  - Default:
- Name: onselectiondragstop
  - Type: OnSelectionDrag<Node>This event handler gets called when a user stops dragging a selection box.
  - Default:
- Name: onselectionstart
  - Type: (event: PointerEvent) => voidThis event handler gets called when the user starts to drag a selection box
  - Default:
- Name: onselectionend
  - Type: (event: PointerEvent) => voidThis event handler gets called when the user finishes dragging a selection box
  - Default:

### Pane Events[](#pane-events)

- Name: onpaneclick
  - Type: ({ event }: { event: MouseEvent; }) => voidThis event handler is called when a user clicks the pane.
  - Default:
- Name: onpanecontextmenu
  - Type: ({ event }: { event: MouseEvent; }) => voidThis event handler is called when a user right-clicks the pane.
  - Default:
- Name: onmovestart
  - Type: OnMoveThis event handler is called when the user begins to pan or zoom the viewport
  - Default:
- Name: onmove
  - Type: OnMoveThis event handler is called when the user pans or zooms the viewport
  - Default:
- Name: onmoveend
  - Type: OnMoveThis event handler is called when the user stops panning or zooming the viewport
  - Default:

### Connection Events[](#connection-events)

- Name: onconnect
  - Type: OnConnectThis event gets fired when a connection successfully completes and an edge is created.
  - Default:
- Name: onconnectstart
  - Type: OnConnectStartWhen a user starts to drag a connection line, this event gets fired.
  - Default:
- Name: onbeforeconnect
  - Type: OnBeforeConnect<Edge>This handler gets called when a new edge is created. You can use it to modify the newly created edge.
  - Default:
- Name: onconnectend
  - Type: OnConnectEndWhen a user stops dragging a connection line, this event gets fired.
  - Default:
- Name: isValidConnection
  - Type: IsValidConnection
  - Default:
- Name: clickConnect
  - Type: booleanToggles ability to make connections via clicking the handles
  - Default:
- Name: onclickconnectstart
  - Type: OnConnectStartA connection is started by clicking on a handle
  - Default:
- Name: onclickconnectend
  - Type: OnConnectEndA connection is finished by clicking on a handle
  - Default:

## Connection line props[](#connection-line-props)

- Name: connectionRadius
  - Type: numberThe radius around a handle where you drop a connection line to create a new edge.
  - Default: 20
- Name: connectionLineComponent
  - Type: Component<{}, {}, string>Provide a custom snippet to be used insted of the default connection line
  - Default:
- Name: connectionLineType
  - Type: ConnectionLineTypeChoose from the built-in edge types to be used for connections
  - Default: 'default' | ConnectionLineType.Bezier
- Name: connectionLineStyle
  - Type: stringStyles to be applied to the connection line
  - Default:
- Name: connectionLineContainerStyle
  - Type: stringStyles to be applied to the container of the connection line
  - Default:

## Interaction props[](#interaction-props)

- Name: elementsSelectable
  - Type: booleanControls if all elements should (nodes & edges) be selectable
  - Default: true
- Name: autoPanOnConnect
  - Type: booleanYou can enable this prop to automatically pan the viewport while making a new connection.
  - Default: true
- Name: autoPanOnNodeDrag
  - Type: booleanYou can enable this prop to automatically pan the viewport while dragging a node.
  - Default: true
- Name: selectNodesOnDrag
  - Type: booleanControls if nodes should be automatically selected when being dragged
  - Default:
- Name: panOnDrag
  - Type: boolean | number[]Enableing this prop allows users to pan the viewport by clicking and dragging.You can also set this prop to an array of numbers to limit which mouse buttons can activate panning.
  - Default: true
- Name: selectionOnDrag
  - Type: booleanSelect multiple elements with a selection box, without pressing down selectionKey.
  - Default: false
- Name: selectionMode
  - Type: SelectionModeWhen set to “partial”, when the user creates a selection box by click and draggingnodes that are only partially in the box are still selected.
  - Default: 'full'
- Name: panOnScroll
  - Type: booleanControls if the viewport should pan by scrolling inside the containerCan be limited to a specific direction with panOnScrollMode
  - Default: false
- Name: panOnScrollMode
  - Type: PanOnScrollModeThis prop is used to limit the direction of panning when panOnScroll is enabled.The “free” option allows panning in any direction.
  - Default: "free"
- Name: zoomOnScroll
  - Type: booleanControls if the viewport should zoom by scrolling inside the container.
  - Default: true
- Name: zoomOnPinch
  - Type: booleanControls if the viewport should zoom by pinching on a touch screen
  - Default: true
- Name: zoomOnDoubleClick
  - Type: booleanControls if the viewport should zoom by double clicking somewhere on the flow
  - Default: true
- Name: connectionMode
  - Type: ConnectionMode‘strict’ connection mode will only allow you to connect source handles to target handles.‘loose’ connection mode will allow you to connect handles of any type to one another.
  - Default: 'strict'
- Name: paneClickDistance
  - Type: numberDistance that the mouse can move between mousedown/up that will trigger a click
  - Default: 0

## Keyboard props[](#keyboard-props)

- Name: deleteKey
  - Type: KeyDefinition | KeyDefinition[] | nullPressing down this key deletes all selected nodes & edges.
  - Default: 'Backspace'
- Name: selectionKey
  - Type: KeyDefinition | KeyDefinition[] | nullPressing down this key you can select multiple elements with a selection box.
  - Default: 'Shift'
- Name: multiSelectionKey
  - Type: KeyDefinition | KeyDefinition[] | nullPressing down this key you can select multiple elements by clicking.
  - Default: 'Meta' for macOS, "Ctrl" for other systems
- Name: zoomActivationKey
  - Type: KeyDefinition | KeyDefinition[] | nullIf a key is set, you can zoom the viewport while that key is held down even if panOnScroll is set to false.By setting this prop to null you can disable this functionality.
  - Default: 'Meta' for macOS, "Ctrl" for other systems
- Name: panActivationKey
  - Type: KeyDefinition | KeyDefinition[] | nullIf a key is set, you can pan the viewport while that key is held down even if panOnScroll is set to false.By setting this prop to null you can disable this functionality.
  - Default: 'Space'
- Name: disableKeyboardA11y
  - Type: booleanYou can use this prop to disable keyboard accessibility features such as selecting nodes ormoving selected nodes with the arrow keys.
  - Default: false

## Style props[](#style-props)

Applying certain classes to elements rendered inside the canvas will change how interactions are handled. These props let you configure those class names if you need to.

- Name: noPanClass
  - Type: stringIf an element in the canvas does not stop mouse events from propagating, clicking and draggingthat element will pan the viewport. Adding the "nopan" class prevents this behavior and thisprop allows you to change the name of that class.
  - Default: "nopan"
- Name: noDragClass
  - Type: stringIf a node is draggable, clicking and dragging that node will move it around the canvas. Addingthe "nodrag" class prevents this behavior and this prop allows you to change the name of thatclass.
  - Default: "nodrag"
- Name: noWheelClass
  - Type: stringTypically, scrolling the mouse wheel when the mouse is over the canvas will zoom the viewport.Adding the "nowheel" class to an element n the canvas will prevent this behavior and this propallows you to change the name of that class.
  - Default: "nowheel"

## Notes[](#notes)

- The props of this component get exported as `SvelteFlowProps`
