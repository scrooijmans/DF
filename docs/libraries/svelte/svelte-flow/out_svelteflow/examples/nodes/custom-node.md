# Custom Nodes

Creating custom nodes is as easy as building a regular Svelte component and passing it to `nodeTypes`. Since they’re standard Svelte components, you can display any content and implement any functionality you need. Plus, you’ll have access to a range of props that allow you to extend and customize the default node behavior. For more details, check out our <a href="https://svelteflow.dev/learn/customization/custom-nodes" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]">Custom Nodes guide</a>.

App.svelte

ColorSelectorNode.svelte

xy-theme.css

index.css

<img src="out_svelteflow/examples/nodes/custom-node/index_media/a8615f5af2f285e6489e8b474f5d8c4d43e8c2bd.svg" class="size-5 stroke-2" />

<img src="out_svelteflow/examples/nodes/custom-node/index_media/602cbd22f5011ed5cb261c84522d5ea8f82ecff9.svg" class="size-6" />

<img src="out_svelteflow/examples/nodes/custom-node/index_media/7074fca34cc45baec265a1de8808fab16b667ede.svg" class="size-5 stroke-2" />

<img src="out_svelteflow/examples/nodes/custom-node/index_media/824817a2c7b3c6650883ed0f84fc74ac655539f5.svg" class="size-4 fill-slate-700 stroke-slate-700" />

``` x:group
<script module>
  class BgColor {
    current = $state('#F7F9FB');
  }
  export const bgColor = new BgColor();
</script>
 
<script lang="ts">
  import {
    SvelteFlow,
    Controls,
    MiniMap,
    Position,
    type Node,
    type Edge,
    Background,
  } from '@xyflow/svelte';
 
  import ColorSelectorNode from './ColorSelectorNode.svelte';
 
  import '@xyflow/svelte/dist/style.css';
 
  const nodeTypes = {
    selectorNode: ColorSelectorNode,
  };
 
  const initialNodes: Node[] = [
    {
      id: '1',
      type: 'input',
      data: { label: 'An input node' },
      position: { x: 0, y: 50 },
      sourcePosition: Position.Right,
    },
    {
      id: '2',
      type: 'selectorNode',
      data: {},
      position: { x: 300, y: 50 },
    },
    {
      id: '3',
      type: 'output',
      data: { label: 'Output A' },
      position: { x: 650, y: 25 },
      targetPosition: Position.Left,
    },
    {
      id: '4',
      type: 'output',
      data: { label: 'Output B' },
      position: { x: 650, y: 100 },
      targetPosition: Position.Left,
    },
  ];
 
  const initialEdges: Edge[] = [
    {
      id: 'e1-2',
      source: '1',
      target: '2',
      animated: true,
    },
    {
      id: 'e2a-3',
      source: '2',
      target: '3',
      animated: true,
    },
    {
      id: 'e2b-4',
      source: '2',
      target: '4',
      animated: true,
    },
  ];
 
  let nodes = $state.raw<Node[]>(initialNodes);
  let edges = $state.raw<Edge[]>(initialEdges);
 
  $inspect(bgColor.current);
</script>
 
<SvelteFlow bind:nodes bind:edges {nodeTypes} fitView>
  <Background bgColor={bgColor.current} />
  <Controls />
  <MiniMap />
</SvelteFlow>
```
