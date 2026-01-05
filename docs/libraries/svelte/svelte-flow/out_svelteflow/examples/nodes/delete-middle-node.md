# Delete Middle Node

This example shows you how to recover deleted edges when you remove a node from the middle of a chain. In other words, if we have three nodes connected in sequence - `a->b->c` - and we deleted the middle node `b`, this example shows you how to end up with the graph `a->c`.

To achieve this, we need to make use of a few bits:

- The <a href="https://svelteflow.dev/api-reference/svelte-flow#onbeforedelete" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]"><code class="nextra-code" dir="ltr">onbeforedelete</code></a> handler lets us intercept and modify the deletion process before nodes are removed.
- <a href="https://svelteflow.dev/api-reference/utils/get-connected-edges" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]"><code class="nextra-code" dir="ltr">getConnectedEdges</code></a> gives us all the edges connected to a node, either as source or target.
- <a href="https://svelteflow.dev/api-reference/utils/get-incomers" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]"><code class="nextra-code" dir="ltr">getIncomers</code></a> and <a href="https://svelteflow.dev/api-reference/utils/get-outgoers" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]"><code class="nextra-code" dir="ltr">getOutgoers</code></a> give us the nodes connected to a node as source or target.

All together, this allows us to take all the nodes connected to the deleted node, and reconnect them to any nodes the deleted node was connected to.

App.svelte

xy-theme.css

index.css

nodes-and-edges.ts

<img src="out_svelteflow/examples/nodes/delete-middle-node/index_media/a8615f5af2f285e6489e8b474f5d8c4d43e8c2bd.svg" class="size-5 stroke-2" />

<img src="out_svelteflow/examples/nodes/delete-middle-node/index_media/602cbd22f5011ed5cb261c84522d5ea8f82ecff9.svg" class="size-6" />

<img src="out_svelteflow/examples/nodes/delete-middle-node/index_media/7074fca34cc45baec265a1de8808fab16b667ede.svg" class="size-5 stroke-2" />

<img src="out_svelteflow/examples/nodes/delete-middle-node/index_media/824817a2c7b3c6650883ed0f84fc74ac655539f5.svg" class="size-4 fill-slate-700 stroke-slate-700" />

``` x:group
<script lang="ts">
  import {
    SvelteFlow,
    Background,
    type Node,
    type Edge,
    type OnBeforeDelete,
    getIncomers,
    getOutgoers,
    getConnectedEdges,
  } from '@xyflow/svelte';
 
  import '@xyflow/svelte/dist/style.css';
 
  import { initialNodes, initialEdges } from './nodes-and-edges';
 
  let nodes = $state.raw<Node[]>(initialNodes);
  let edges = $state.raw<Edge[]>(initialEdges);
 
  const onbeforedelete: OnBeforeDelete = async ({ nodes: deletedNodes, edges: _edges }) => {
    let remainingNodes = [...nodes]; 
    
    edges = deletedNodes.reduce((acc, node) => {
      const incomers = getIncomers(node, remainingNodes, acc);
      const outgoers = getOutgoers(node, remainingNodes, acc);
      const connectedEdges = getConnectedEdges([node], acc);
 
      const remainingEdges = acc.filter((edge) => !connectedEdges.includes(edge));
 
      const createdEdges = incomers.flatMap(({ id: source }) =>
        outgoers.map(({ id: target }) => ({
          id: `${source}->${target}`,
          source,
          target,
        })),
      );
 
      remainingNodes = remainingNodes.filter((rn) => rn.id !== node.id);
 
      return [...remainingEdges, ...createdEdges];
    }, edges);
 
    nodes = remainingNodes;
 
    return true;
  };
</script>
 
<SvelteFlow bind:nodes bind:edges {onbeforedelete} fitView>
  <Background />
</SvelteFlow>
```

Although this example is less than 20 lines of code there’s quite a lot to digest. Let’s break some of it down:

- Our `onbeforedelete` handler is called with an object containing `nodes` and `edges` arrays representing what will be deleted. The `nodes` array contains every node that will be deleted. If you select an individual node and press the delete key, it will contain just that node, but if you make a selection *all* the nodes in that selection will be included.

- We create a new array of edges - `remainingEdges` - that contains all the edges in the flow that have nothing to do with the node(s) we’re about to delete.

- We create another array of edges by *flatMapping* over the array of `incomers`. These are nodes that were connected to the deleted node as a source. For each of these nodes, we create a new edge that connects to each node in the array of `outgoers`. These are nodes that were connected to the deleted node as a target.

- Finally, we return `true` from the callback to allow the deletion to proceed with our modified edge array.

<img src="out_svelteflow/examples/nodes/delete-middle-node/index_media/a7db0f47f02f813c5a264458a8e522bf6320ca5a.svg" class="x:mt-[.3em]" />

For brevity, we’re using object destructuring while at the same time renaming the variable bound (e.g. `({ id: source }) => ...)` destructures the `id` property of the object and binds it to a new variable called `source`) but you don’t need to do this

## Quick Reference<a href="https://svelteflow.dev/examples/nodes/delete-middle-node#quick-reference" class="x:focus-visible:nextra-focus subheading-anchor" aria-label="Permalink for this section"></a>

<a href="https://svelteflow.dev/api-reference/utils/getConnectedEdges" class="x:group x:focus-visible:nextra-focus nextra-card x:flex x:flex-col x:justify-start x:overflow-hidden x:rounded-lg x:border x:border-gray-200 x:text-current x:no-underline x:dark:shadow-none x:hover:shadow-gray-100 x:dark:hover:shadow-none x:shadow-gray-100 x:active:shadow-sm x:active:shadow-gray-200 x:transition-all x:duration-200 x:hover:border-gray-300 x:bg-transparent x:shadow-sm x:dark:border-neutral-800 x:hover:bg-slate-50 x:hover:shadow-md x:dark:hover:border-neutral-700 x:dark:hover:bg-neutral-900">getConnectedEdges</a><a href="https://svelteflow.dev/api-reference/utils/getIncomers" class="x:group x:focus-visible:nextra-focus nextra-card x:flex x:flex-col x:justify-start x:overflow-hidden x:rounded-lg x:border x:border-gray-200 x:text-current x:no-underline x:dark:shadow-none x:hover:shadow-gray-100 x:dark:hover:shadow-none x:shadow-gray-100 x:active:shadow-sm x:active:shadow-gray-200 x:transition-all x:duration-200 x:hover:border-gray-300 x:bg-transparent x:shadow-sm x:dark:border-neutral-800 x:hover:bg-slate-50 x:hover:shadow-md x:dark:hover:border-neutral-700 x:dark:hover:bg-neutral-900">getIncomers</a><a href="https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/flatMap" class="x:group x:focus-visible:nextra-focus nextra-card x:flex x:flex-col x:justify-start x:overflow-hidden x:rounded-lg x:border x:border-gray-200 x:text-current x:no-underline x:dark:shadow-none x:hover:shadow-gray-100 x:dark:hover:shadow-none x:shadow-gray-100 x:active:shadow-sm x:active:shadow-gray-200 x:transition-all x:duration-200 x:hover:border-gray-300 x:bg-transparent x:shadow-sm x:dark:border-neutral-800 x:hover:bg-slate-50 x:hover:shadow-md x:dark:hover:border-neutral-700 x:dark:hover:bg-neutral-900"><img src="out_svelteflow/examples/nodes/delete-middle-node/index_media/760a8bf40a0172350304e85d3f9c4ca26ac4091d.svg" />Array.prototype.flatMap</a><a href="https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Destructuring_assignment#assigning_to_new_variable_names" class="x:group x:focus-visible:nextra-focus nextra-card x:flex x:flex-col x:justify-start x:overflow-hidden x:rounded-lg x:border x:border-gray-200 x:text-current x:no-underline x:dark:shadow-none x:hover:shadow-gray-100 x:dark:hover:shadow-none x:shadow-gray-100 x:active:shadow-sm x:active:shadow-gray-200 x:transition-all x:duration-200 x:hover:border-gray-300 x:bg-transparent x:shadow-sm x:dark:border-neutral-800 x:hover:bg-slate-50 x:hover:shadow-md x:dark:hover:border-neutral-700 x:dark:hover:bg-neutral-900"><img src="out_svelteflow/examples/nodes/delete-middle-node/index_media/760a8bf40a0172350304e85d3f9c4ca26ac4091d.svg" />Destructuring assignment</a>
