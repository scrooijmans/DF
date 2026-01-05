# Eraser

This example shows how to create an eraser tool that allows you to delete nodes and edges by wiping them out. It’s made up of two parts:

1.  The `Eraser` component that handles the erasing logic and rendering of the eraser trail.
2.  The custom `ErasableNode` and `ErasableEdge` that reacts to the `toBeDeleted` flag.

Determining if the trail intersects with a node is fairly straight forward - however detecting intersections between the trail and an edge is a bit more complex: We sample points along the edge through the <a href="https://developer.mozilla.org/en-US/docs/Web/API/SVGGeometryElement/getPointAtLength" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]" rel="noreferrer" target="_blank"><code class="nextra-code" dir="ltr">getPointAtLength</code></a> method of the SVG path element, construct a polyline that we can then use to detect intersections with the eraser trail. This is a trade-off between performance and accuracy - you can play around with the `sampleDistance` variable to see the effect it has on the eraser trail.

App.svelte

ErasableEdge.svelte

ErasableNode.svelte

Eraser.svelte

xy-theme.css

index.css

utils.ts

<img src="out_svelteflow/examples/whiteboard/eraser/index_media/a8615f5af2f285e6489e8b474f5d8c4d43e8c2bd.svg" class="size-5 stroke-2" />

<img src="out_svelteflow/examples/whiteboard/eraser/index_media/602cbd22f5011ed5cb261c84522d5ea8f82ecff9.svg" class="size-6" />

<img src="out_svelteflow/examples/whiteboard/eraser/index_media/7074fca34cc45baec265a1de8808fab16b667ede.svg" class="size-5 stroke-2" />

<img src="out_svelteflow/examples/whiteboard/eraser/index_media/824817a2c7b3c6650883ed0f84fc74ac655539f5.svg" class="size-4 fill-slate-700 stroke-slate-700" />

``` x:group
<script lang="ts">
  import { SvelteFlow, Background, Controls, Panel } from '@xyflow/svelte';
 
  import ErasableNode, { type ErasableNodeType } from './ErasableNode.svelte';
  import ErasableEdge, { type ErasableEdgeType } from './ErasableEdge.svelte';
  import Eraser from './Eraser.svelte';
 
  import '@xyflow/svelte/dist/style.css';
 
  const initialNodes: ErasableNodeType[] = [
    {
      id: '1',
      type: 'erasable-node',
      position: { x: 0, y: 0 },
      data: { label: 'Hello' },
    },
    {
      id: '2',
      type: 'erasable-node',
      position: { x: 300, y: 0 },
      data: { label: 'World' },
    },
  ];
 
  const initialEdges: ErasableEdgeType[] = [
    {
      id: '1->2',
      type: 'erasable-edge',
      source: '1',
      target: '2',
      data: {},
    },
  ];
 
  const nodeTypes = {
    'erasable-node': ErasableNode,
  };
 
  const edgeTypes = {
    'erasable-edge': ErasableEdge,
  };
 
  let nodes = $state.raw(initialNodes);
  let edges = $state.raw(initialEdges);
 
  let isEraserActive = $state(true);
</script>
 
<SvelteFlow
  bind:nodes
  bind:edges
  {nodeTypes}
  {edgeTypes}
  fitView
  defaultEdgeOptions={{ type: 'erasable-edge' }}
>
  <Controls />
  <Background />
 
  {#if isEraserActive}
    <Eraser />
  {/if}
 
  <Panel position="top-left" class="controls">
    <div class="xy-theme__button-group">
      <button
        class={['xy-theme__button', isEraserActive && 'active']}
        onclick={() => {
          isEraserActive = true;
        }}
      >
        Eraser Mode
      </button>
      <button
        class={['xy-theme__button', !isEraserActive && 'active']}
        onclick={() => {
          isEraserActive = false;
        }}
      >
        Selection Mode
      </button>
    </div>
  </Panel>
</SvelteFlow>
 
<style>
  :global .controls {
    z-index: 1001;
  }
</style>
```
