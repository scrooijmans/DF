# Rectangle

In this example, you can draw rectangles on a whiteboard. Click and drag to create a rectangle, and it will be added to the flow.

App.svelte

RectangleNode.svelte

RectangleTool.svelte

xy-theme.css

index.css

<img src="out_svelteflow/examples/whiteboard/rectangle/index_media/a8615f5af2f285e6489e8b474f5d8c4d43e8c2bd.svg" class="size-5 stroke-2" />

<img src="out_svelteflow/examples/whiteboard/rectangle/index_media/602cbd22f5011ed5cb261c84522d5ea8f82ecff9.svg" class="size-6" />

<img src="out_svelteflow/examples/whiteboard/rectangle/index_media/7074fca34cc45baec265a1de8808fab16b667ede.svg" class="size-5 stroke-2" />

<img src="out_svelteflow/examples/whiteboard/rectangle/index_media/824817a2c7b3c6650883ed0f84fc74ac655539f5.svg" class="size-4 fill-slate-700 stroke-slate-700" />

``` x:group
<script lang="ts">
  import {
    SvelteFlow,
    Background,
    Controls,
    Panel,
    addEdge,
    type Connection,
  } from '@xyflow/svelte';
 
  import RectangleNode, { type RectangleNodeType } from './RectangleNode.svelte';
  import RectangleTool from './RectangleTool.svelte';
 
  import '@xyflow/svelte/dist/style.css';
 
  const initialNodes: RectangleNodeType[] = [
    {
      id: '1',
      type: 'rectangle',
      position: { x: 250, y: 5 },
      data: { color: '#ff7000' },
      width: 150,
      height: 100,
    },
  ];
  const initialEdges: any[] = [];
 
  const nodeTypes = {
    rectangle: RectangleNode,
  };
 
  let nodes = $state.raw(initialNodes);
  let edges = $state.raw(initialEdges);
  let isRectangleActive = $state(true);
 
  function onConnect(params: Connection) {
    edges = addEdge(params, edges);
  }
</script>
 
<SvelteFlow bind:nodes bind:edges {nodeTypes} {onConnect} fitView>
  <Controls />
  <Background />
 
  <Panel position="top-left">
    <div class="xy-theme__button-group">
      <button
        class={['xy-theme__button', isRectangleActive && 'active']}
        onclick={() => {
          isRectangleActive = true;
        }}
      >
        Rectangle Mode
      </button>
      <button
        class={['xy-theme__button', !isRectangleActive && 'active']}
        onclick={() => {
          isRectangleActive = false;
        }}
      >
        Selection Mode
      </button>
    </div>
  </Panel>
  {#if isRectangleActive}
    <RectangleTool />
  {/if}
</SvelteFlow>
```
