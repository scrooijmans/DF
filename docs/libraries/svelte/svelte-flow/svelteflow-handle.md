# The Handle component - Svelte Flow

[Source on GitHub](https://github.com/xyflow/xyflow/blob/main/packages/svelte/src/lib/components/Handle/Handle.svelte) 

The `<Handle />` component is used in your [custom nodes](https://svelteflow.dev/learn/customization/custom-nodes) to define connection points.

```
<script lang="ts">
  import { Handle, Position, type NodeProps } from '@xyflow/svelte';

  let { data } : NodeProps = $props();
</script>

<div>
  {data.label}
</div>

<Handle type="target" position={Position.Left} />
<Handle type="source" position={Position.Right} />
```

## Props[](#props)

The type for props of `<Handle />` component is exported as `HandleProps`. Additionally, it extends the props of `<div />`.

- Name: type
  - Type: HandleTypeType of the handle.
  - Default: "source"
- Name: position
  - Type: PositionThe position of the handle relative to the node. In a horizontal flow source handles aretypically Position.Right and in a vertical flow they are typically Position.Top.
  - Default: Position.Top
- Name: isConnectable
  - Type: booleanShould you be able to connect to/from this handle.
  - Default: true
- Name: isConnectableStart
  - Type: booleanDictates whether a connection can start from this handle.
  - Default: true
- Name: isConnectableEnd
  - Type: booleanDictates whether a connection can end on this handle.
  - Default: true
- Name: isValidConnection
  - Type: IsValidConnectionCalled when a connection is dragged to this handle. You can use this callback to perform somecustom validation logic based on the connection target and source, for example. Where possible,we recommend you move this logic to the isValidConnection prop on the main ReactFlowcomponent for performance reasons.
  - Default:
- Name: onconnect
  - Type: (connections: Connection[]) => void
  - Default:
- Name: ondisconnect
  - Type: (connections: Connection[]) => void
  - Default:
- Name: ...props
  - Type: HTMLAttributes<HTMLDivElement>
  - Default:

## Examples[](#examples)

### Custom handle with validation[](#custom-handle-with-validation)

You can create your own custom handles by wrapping the `<Handle />` component. This example shows a custom handle that only allows connections when the connection source matches a given id.

```
<script lang="ts">
	import { Handle, type HandleProps  } from '@xyflow/svelte';

	let { source, ...rest } : HandleProps = $props();

	function isValidConnection(connection) {
		return connection.source === source;
	}
</script>

<Handle
	type="target"
	{isValidConnection}
	{...rest}
/>
```

### Style handles when connecting[](#style-handles-when-connecting)

The handle receives the additional class names `connecting` when the connection line is above the handle and `valid` if the connection is valid. You can find an example which uses these classes [here](https://svelteflow.dev/examples/interaction/validation).

### Multiple handles[](#multiple-handles)

If you need multiple source or target handles you can achieve this by creating a custom node. Normally you just use the id of a node for the `source` or `target` of an edge. If you have multiple source or target handles you need to pass an id to these handles. These ids can be used by an edge with the `sourceHandle` and `targetHandle` options, so that you can connect a specific handle. If you have a node with `id: 'node-1'` and a handle with `id: 'handle-1'` you can connect an edge to this handle by defining it with `source: 'node-1'` and `sourceHandle: 'hadnle-1'`.

### Dynamic handles[](#dynamic-handles)

If you are programmatically changing the position or number of handles in your custom node, you need to update the node internals with the [`useUpdateNodeInternals`](https://svelteflow.dev/api-reference/hooks/use-update-node-internals) hook.

You can find an example of how to implement a custom node with multiple handles in the [custom node guide](https://svelteflow.dev/learn/customization/custom-nodes) or in the [custom node example](https://svelteflow.dev/examples/nodes/custom-node).

### Custom handle styles[](#custom-handle-styles)

Since the handle is a div, you can use CSS to style it or pass a style prop to customize a Handle. You can see this in the [Add Node On Edge Drop](https://svelteflow.dev/examples/nodes/add-node-on-edge-drop) and [Simple Floating Edges](https://svelteflow.dev/examples/edges/simple-floating-edges) examples.

Last updated on

May 14, 2025

[<EdgeReconnectAnchor />](https://svelteflow.dev/api-reference/components/edge-reconnect-anchor '<EdgeReconnectAnchor />')
[<MiniMap />](https://svelteflow.dev/api-reference/components/mini-map '<MiniMap />')
