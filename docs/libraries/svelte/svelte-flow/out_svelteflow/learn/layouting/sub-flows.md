# Sub Flows

A sub flow is a flowgraph contained within a node. These nested flows can operate independently or connect with nodes outside their parent node, making them perfect for organizing and grouping related nodes. This guide will show you how to implement sub flows and explain the available options for child nodes.

### Defining Child Nodes<a href="https://svelteflow.dev/learn/layouting/sub-flows#defining-child-nodes" class="x:focus-visible:nextra-focus subheading-anchor" aria-label="Permalink for this section"></a>

To define a child node, use the `parentId` option (see all available options in the <a href="https://svelteflow.dev/api-reference/types/node" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]">node options section</a>). Child nodes are positioned relative to their parent, with `{ x: 0, y: 0 }` representing the top-left corner of the parent node.

``` x:group
let nodes = $state.raw([
  {
    id: 'A',
    data: { label: 'parent' },
    position: { x: 0, y: 0 },
  },
  {
    id: 'B',
    data: { label: 'child' },
    position: { x: 10, y: 10 },
    parentId: 'A',
  },
]);
```

<img src="out_svelteflow/learn/layouting/sub-flows/index_media/a7db0f47f02f813c5a264458a8e522bf6320ca5a.svg" class="x:mt-[.3em]" />

**Order of Nodes:** Parents have to appear before their child nodes in the `nodes` array!

#### `extent: 'parent'`<a href="https://svelteflow.dev/learn/layouting/sub-flows#extent-parent" class="x:focus-visible:nextra-focus subheading-anchor" aria-label="Permalink for this section"></a>

When a parent node has defined dimensions, you can restrict child node movement to stay within the parent’s boundaries by setting `extent: 'parent'`. This prevents child nodes from being dragged outside their parent’s area.

<img src="out_svelteflow/learn/layouting/sub-flows/index_media/602cbd22f5011ed5cb261c84522d5ea8f82ecff9.svg" class="size-6" />

<img src="out_svelteflow/learn/layouting/sub-flows/index_media/7074fca34cc45baec265a1de8808fab16b667ede.svg" class="size-5 stroke-2" />

<img src="out_svelteflow/learn/layouting/sub-flows/index_media/824817a2c7b3c6650883ed0f84fc74ac655539f5.svg" class="size-4 fill-slate-700 stroke-slate-700" />

### Child Node Behavior<a href="https://svelteflow.dev/learn/layouting/sub-flows#child-node-behavior" class="x:focus-visible:nextra-focus subheading-anchor" aria-label="Permalink for this section"></a>

Child nodes maintain their relative position when the parent node is moved. While the `parentId` option establishes the parent-child relationship, child nodes can still be positioned outside their parent (unless `extent: 'parent'` is set). However, they will always move with their parent when it’s dragged.

In our examples, we use the built-in `group` node type for parent nodes, which provides a transparent background with a border. You can also use <a href="https://svelteflow.dev/learn/layouting/sub-flows#any-node-can-be-a-parent-node" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]">any custom node type</a> as a parent.

Let’s explore more complex scenarios by adding additional nodes and edges. You can create connections both within a group and between sub flows and outer nodes:

<img src="out_svelteflow/learn/layouting/sub-flows/index_media/602cbd22f5011ed5cb261c84522d5ea8f82ecff9.svg" class="size-6" />

<img src="out_svelteflow/learn/layouting/sub-flows/index_media/7074fca34cc45baec265a1de8808fab16b667ede.svg" class="size-5 stroke-2" />

<img src="out_svelteflow/learn/layouting/sub-flows/index_media/824817a2c7b3c6650883ed0f84fc74ac655539f5.svg" class="size-4 fill-slate-700 stroke-slate-700" />

### Using a Custom Parent Node<a href="https://svelteflow.dev/learn/layouting/sub-flows#using-a-custom-parent-node" class="x:focus-visible:nextra-focus subheading-anchor" aria-label="Permalink for this section"></a>

To demonstrate the flexibility of parent nodes, let’s modify our example by removing the label from node B and adding child nodes. This example shows how you can use any node type as a parent. We’ll also set `draggable: false` on the child nodes to make them static.

<img src="out_svelteflow/learn/layouting/sub-flows/index_media/602cbd22f5011ed5cb261c84522d5ea8f82ecff9.svg" class="size-6" />

<img src="out_svelteflow/learn/layouting/sub-flows/index_media/7074fca34cc45baec265a1de8808fab16b667ede.svg" class="size-5 stroke-2" />

<img src="out_svelteflow/learn/layouting/sub-flows/index_media/824817a2c7b3c6650883ed0f84fc74ac655539f5.svg" class="size-4 fill-slate-700 stroke-slate-700" />
