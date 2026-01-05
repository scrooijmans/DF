# Drag Handle

You can restrict dragging to a specific part of node, by specifying a class that will act as a <a href="https://svelteflow.dev/api-reference/types/node" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]"><code class="nextra-code" dir="ltr">dragHandle</code></a>. To prevent specific elements within a drag handle from triggering a drag, you can use the <a href="https://svelteflow.dev/api-reference/types/node-props#notes" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]"><code class="nextra-code" dir="ltr">nodrag</code></a> class name on those elements.

App.svelte

DragHandleNode.svelte

xy-theme.css

index.css

<img src="out_svelteflow/examples/nodes/drag-handle/index_media/a8615f5af2f285e6489e8b474f5d8c4d43e8c2bd.svg" class="size-5 stroke-2" />

<img src="out_svelteflow/examples/nodes/drag-handle/index_media/602cbd22f5011ed5cb261c84522d5ea8f82ecff9.svg" class="size-6" />

<img src="out_svelteflow/examples/nodes/drag-handle/index_media/7074fca34cc45baec265a1de8808fab16b667ede.svg" class="size-5 stroke-2" />

<img src="out_svelteflow/examples/nodes/drag-handle/index_media/824817a2c7b3c6650883ed0f84fc74ac655539f5.svg" class="size-4 fill-slate-700 stroke-slate-700" />

``` x:group
<script lang="ts">
  import {
    SvelteFlow,
    Controls,
    Background,
    type Node,
    type Edge,
  } from '@xyflow/svelte';
 
  import DragHandleNode from './DragHandleNode.svelte';
 
  import '@xyflow/svelte/dist/style.css';
 
  const nodeTypes = {
    dragHandleNode: DragHandleNode,
  };
 
  let nodes = $state.raw<Node[]>([
    {
      id: '2',
      type: 'dragHandleNode',
      // Specify the custom class acting as a drag handle
      dragHandle: '.custom-drag-handle',
      style: 'border: 1px solid #ddd;',
      position: { x: 200, y: 200 },
      data: {
        label: 'Drag Handle',
      },
    },
  ]);
 
  let edges = $state.raw<Edge[]>([]);
</script>
 
<SvelteFlow bind:nodes bind:edges {nodeTypes} fitView>
  <Controls />
  <Background />
</SvelteFlow>
```
