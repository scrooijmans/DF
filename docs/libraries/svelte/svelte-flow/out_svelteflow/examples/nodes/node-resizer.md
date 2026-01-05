# Node Resizer

The <a href="https://svelteflow.dev/api-reference/components/node-resizer" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]"><code class="nextra-code" dir="ltr">&lt;NodeResizer /&gt;</code></a> component can be used to add a resize UI for a custom node. The `svelteflow` package also exports a `<NodeResizeControl />` component for implementing a custom resizing UI as shown in this example.

App.svelte

CustomResizerNode.svelte

ResizableNode.svelte

ResizableNodeSelected.svelte

xy-theme.css

index.css

<img src="out_svelteflow/examples/nodes/node-resizer/index_media/a8615f5af2f285e6489e8b474f5d8c4d43e8c2bd.svg" class="size-5 stroke-2" />

<img src="out_svelteflow/examples/nodes/node-resizer/index_media/602cbd22f5011ed5cb261c84522d5ea8f82ecff9.svg" class="size-6" />

<img src="out_svelteflow/examples/nodes/node-resizer/index_media/7074fca34cc45baec265a1de8808fab16b667ede.svg" class="size-5 stroke-2" />

<img src="out_svelteflow/examples/nodes/node-resizer/index_media/824817a2c7b3c6650883ed0f84fc74ac655539f5.svg" class="size-4 fill-slate-700 stroke-slate-700" />

``` x:group
<script lang="ts">
  import { SvelteFlow, Background, type Node, type Edge } from '@xyflow/svelte';
 
  import ResizableNode from './ResizableNode.svelte';
  import CustomResizerNode from './CustomResizerNode.svelte';
  import ResizableNodeSelected from './ResizableNodeSelected.svelte';
 
  import '@xyflow/svelte/dist/style.css';
 
  // const nodeStyle =
  //   'background: #fff; border: 1px solid black; border-radius: 15px; font-size: 12px;';
 
  const initialEdges: Edge[] = [];
  const initialNodes: Node[] = [
    {
      id: '1',
      type: 'ResizableNode',
      data: { label: 'NodeResizer' },
      position: { x: 0, y: 50 },
    },
    {
      id: '2',
      type: 'ResizableNodeSelected',
      data: { label: 'NodeResizer when selected' },
      position: { x: -100, y: 150 },
    },
    {
      id: '3',
      type: 'CustomResizerNode',
      data: { label: 'Custom Resize Icon' },
      position: { x: 150, y: 150 },
      style: ' height: 100px;',
    },
  ];
 
  let nodes = $state.raw<Node[]>(initialNodes);
  let edges = $state.raw<Edge[]>(initialEdges);
 
  const nodeTypes = {
    ResizableNode,
    CustomResizerNode,
    ResizableNodeSelected,
  };
</script>
 
<SvelteFlow bind:nodes {nodeTypes} bind:edges fitView>
  <Background />
</SvelteFlow>
```
