# Usage with TypeScript

Svelte Flow is written in TypeScript because we value the additional safety barrier it provides. We export all the types you need for correctly typing data structures and functions you pass to the Svelte Flow component. We also provide a way to extend the types of nodes and edges.

## Basic Usage<a href="https://svelteflow.dev/learn/advanced/typescript#basic-usage" class="x:focus-visible:nextra-focus subheading-anchor" aria-label="Permalink for this section"></a>

Let’s start with the essential types needed for a basic implementation. While TypeScript can infer some types automatically, we’ll define them explicitly for clarity.

``` x:group
<script lang="ts">
  import {
    SvelteFlow,
    Controls,
    Background,
    BackgroundVariant,
    type Node,
    type Edge,
    type FitViewOptions,
    type DefaultEdgeOptions,
  } from '@xyflow/svelte';
 
  import '@xyflow/svelte/dist/style.css';
 
  let nodes = $state.raw<Node[]>([
    {
      id: '1',
      type: 'input',
      data: { label: 'Node 1' },
      position: { x: 5, y: 5 },
    },
    {
      id: '2',
      type: 'default',
      data: { label: 'Node 2' },
      position: { x: 5, y: 100 },
    },
  ]);
 
  let edges = $state.raw<Edge[]>([
    { id: 'e1-2', source: '1', target: '2' }
  ]);
 
  const fitViewOptions: FitViewOptions = {
    padding: 0.2,
  };
 
  const defaultEdgeOptions: DefaultEdgeOptions = {
    animated: true,
  };
</script>
 
<SvelteFlow
  bind:nodes
  bind:edges
  fitView
  {fitViewOptions}
  {defaultEdgeOptions}
>
  <Controls />
  <Background variant={BackgroundVariant.Dots} />
</SvelteFlow>
```

### Custom Nodes<a href="https://svelteflow.dev/learn/advanced/typescript#custom-nodes" class="x:focus-visible:nextra-focus subheading-anchor" aria-label="Permalink for this section"></a>

When working with <a href="https://svelteflow.dev/learn/customization/custom-nodes" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]">custom nodes</a>, you can extend the base `Node` type to include your custom data. There are two main approaches:

1.  For **multiple custom nodes**, specify a custom `Node` type as a generic to `NodeProps`:

``` x:group
<script module>
  export type NumberNodeType = Node<{ number: number }, 'number'>;
</script>
 
<script lang="ts">
  import { Handle, Position, type NodeProps, type Node } from '@xyflow/svelte';
 
  let { id, data }: NodeProps<NumberNodeType> = $props();
</script>
 
<div class="custom">
  <div>A special number: {data.number}</div>
  <Handle type="source" position={Position.Right} />
</div>
```

⚠️ When defining node data separately, you must use `type` (interfaces won’t work):

``` x:group
type NumberNodeData = { number: number };
type NumberNodeType = Node<NumberNodeData, 'number'>;
```

2.  For **a single custom node** that renders different content based on the node type, use a union type:

``` x:group
<script module>
  export type NumberNodeType = Node<{ number: number }, 'number'>;
  export type TextNodeType = Node<{ text: string }, 'text'>;
 
  export type NodeType = NumberNodeType | TextNodeType;
</script>
 
<script lang="ts">
  import { Handle, Position, type NodeProps } from '@xyflow/svelte';
 
  let { data }: NodeProps<NodeType> = $props();
</script>
 
<div class="custom">
  {#if data.type === 'number'}
    <div>A special number: {data.number}</div>
  {:else}
    <div>A special text: {data.text}</div>
  {/if}
  <Handle type="source" position={Position.Right} />
</div>
```

### Custom Edges<a href="https://svelteflow.dev/learn/advanced/typescript#custom-edges" class="x:focus-visible:nextra-focus subheading-anchor" aria-label="Permalink for this section"></a>

Similar to custom nodes, you can extend the base `Edge` type for <a href="https://svelteflow.dev/learn/customization/custom-edges" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]">custom edges</a>:

``` x:group
<script module>
  export type EdgeType = Edge<{ value: number }, 'custom'>;
</script>
 
<script lang="ts">
  import { getStraightPath, BaseEdge, type EdgeProps } from '@xyflow/svelte';
 
  let { id, sourceX, sourceY, targetX, targetY }: EdgeProps<EdgeType> = $props();
 
  let [edgePath] = $derived(getStraightPath({ sourceX, sourceY, targetX, targetY }));
</script>
 
<BaseEdge {id} path={edgePath} />
```

## Advanced Usage<a href="https://svelteflow.dev/learn/advanced/typescript#advanced-usage" class="x:focus-visible:nextra-focus subheading-anchor" aria-label="Permalink for this section"></a>

In complex applications, you’ll likely have multiple custom nodes and edges with different data structures. When using built-in functions and hooks, you’ll need to properly <a href="https://www.typescriptlang.org/docs/handbook/2/narrowing.html" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]" rel="noreferrer" target="_blank">narrow down <img src="out_svelteflow/learn/advanced/typescript/index_media/926b7c4e31213c3a8240847d8251d75a921b6a21.svg" class="x:inline x:align-baseline x:shrink-0" /></a> the types to prevent runtime errors.

### `Node` and `Edge` Type Unions<a href="https://svelteflow.dev/learn/advanced/typescript#node-and-edge-type-unions" class="x:focus-visible:nextra-focus subheading-anchor" aria-label="Permalink for this section"></a>

Many functions, callbacks, and hooks (including the SvelteFlow component) expect `NodeType` or `EdgeType` generics. These are unions of all your custom node and edge types. As long as you’ve properly typed your data objects, you can use their exported types.

<img src="out_svelteflow/learn/advanced/typescript/index_media/a7db0f47f02f813c5a264458a8e522bf6320ca5a.svg" class="x:mt-[.3em]" />

If you’re using any built-in nodes (‘input’, ‘output’, ‘default’) or edges (‘straight’, ‘step’, ‘smoothstep’, ‘bezier’), include the `BuiltInNode` and `BuiltInEdge` types from `@xyflow/svelte` in your union type.

``` x:group
<script module>
  import type { BuiltInNode, BuiltInEdge } from '@xyflow/svelte';
 
  // Custom nodes
  import type { NumberNodeType } from './NumberNode.svelte';
  import type { TextNodeType } from './TextNode.svelte';
 
  // Custom edge
  import type { CustomEdgeType } from './CustomEdge.svelte';
 
  export type NodeType = BuiltInNode | NumberNodeType | TextNodeType;
  export type EdgeType = BuiltInEdge | CustomEdgeType;
</script>
 
<script lang="ts">
  import { SvelteFlow, type NodeTypes, type EdgeTypes } from '@xyflow/svelte';
  import NumberNode from './NumberNode.svelte';
  import TextNode from './TextNode.svelte';
  import CustomEdge from './CustomEdge.svelte';
 
  const nodeTypes: NodeTypes = {
    number: NumberNode,
    text: TextNode,
  };
 
  const edgeTypes: EdgeTypes = {
    custom: CustomEdge,
  };
 
  let nodes = $state.raw<NodeType[]>([]);
  let edges = $state.raw<EdgeType[]>([]);
</script>
 
<SvelteFlow bind:nodes bind:edges {nodeTypes} {edgeTypes} fitView>
  <!-- ... -->
</SvelteFlow>
```

### Hooks<a href="https://svelteflow.dev/learn/advanced/typescript#hooks" class="x:focus-visible:nextra-focus subheading-anchor" aria-label="Permalink for this section"></a>

You can use these type unions to properly type the return values of hooks:

``` x:group
<script lang="ts">
  import { useSvelteFlow, useNodeConnections, useNodesData } from '@xyflow/svelte';
  import type { NodeType, EdgeType } from './types';
 
  // Nodes and edges are now correctly typed
  const { getNodes, getEdges } = useSvelteFlow<NodeType, EdgeType>();
 
  const connections = useNodeConnections({
    handleType: 'target',
  });
 
  const nodesData = useNodesData<NodeType>(connections.current.map(c => c.source));
 
  $effect(() => {
    nodesData.current.forEach(({ type, data }) => {
      if (type === 'number') {
        // Type-safe access to number property
        console.log(data.number);
      }
    });
  });
</script>
```

### Type Guards<a href="https://svelteflow.dev/learn/advanced/typescript#type-guards" class="x:focus-visible:nextra-focus subheading-anchor" aria-label="Permalink for this section"></a>

TypeScript provides several ways to implement <a href="https://www.typescriptlang.org/docs/handbook/2/narrowing.html#typeof-type-guards" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]" rel="noreferrer" target="_blank">type guards <img src="out_svelteflow/learn/advanced/typescript/index_media/926b7c4e31213c3a8240847d8251d75a921b6a21.svg" class="x:inline x:align-baseline x:shrink-0" /></a>. One common approach is to create type guard functions like `isNumberNode` or `isTextNode` to filter specific nodes from a list:

``` x:group
function isNumberNode(node: NodeType): node is NumberNodeType {
  return node.type === 'number';
}
 
// numberNodes is now correctly typed as NumberNodeType[]
let numberNodes = $derived(nodes.filter(isNumberNode));
```
