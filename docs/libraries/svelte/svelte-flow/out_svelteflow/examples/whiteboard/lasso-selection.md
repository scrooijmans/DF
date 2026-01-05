# Lasso Selection

This example demonstrates how to implement a lasso selection tool that allows users to select multiple nodes by drawing a freehand selection area. The component features:

- **Lasso Drawing**: Draw freehand selection areas using pointer events
- **Partial/Full Selection**: Toggle between selecting nodes partially or fully enclosed by the lasso
- **Visual Feedback**: Real-time visual feedback while drawing the selection area
- **Canvas Rendering**: Uses HTML5 Canvas for smooth drawing performance

App.svelte

Lasso.svelte

xy-theme.css

index.css

utils.ts

<img src="out_svelteflow/examples/whiteboard/lasso-selection/index_media/a8615f5af2f285e6489e8b474f5d8c4d43e8c2bd.svg" class="size-5 stroke-2" />

<img src="out_svelteflow/examples/whiteboard/lasso-selection/index_media/602cbd22f5011ed5cb261c84522d5ea8f82ecff9.svg" class="size-6" />

<img src="out_svelteflow/examples/whiteboard/lasso-selection/index_media/7074fca34cc45baec265a1de8808fab16b667ede.svg" class="size-5 stroke-2" />

<img src="out_svelteflow/examples/whiteboard/lasso-selection/index_media/824817a2c7b3c6650883ed0f84fc74ac655539f5.svg" class="size-4 fill-slate-700 stroke-slate-700" />

``` x:group
<script lang="ts">
  import { SvelteFlow, Background, Controls, MiniMap, Panel } from '@xyflow/svelte';
  import type { Edge } from '@xyflow/svelte';
 
  import Lasso from './Lasso.svelte';
 
  import '@xyflow/svelte/dist/style.css';
 
  const initialNodes = [
    {
      id: '1',
      position: { x: 0, y: 0 },
      data: { label: 'Hello' },
    },
    {
      id: '2',
      position: { x: 300, y: 0 },
      data: { label: 'World' },
    },
  ];
 
  const initialEdges: Edge[] = [];
 
  let nodes = $state.raw(initialNodes);
  let edges = $state.raw(initialEdges);
 
  let isLassoActive = $state(true);
  let partial = $state(false);
</script>
 
<SvelteFlow bind:nodes bind:edges fitView>
  <Background />
  <MiniMap />
  <Controls />
 
  {#if isLassoActive}
    <Lasso {partial} />
  {/if}
 
  <Panel position="top-left">
    <div style="display: flex; align-items: center; gap: 1em;">
      <div class="xy-theme__button-group">
        <button
          class={['xy-theme__button', isLassoActive && 'active']}
          onclick={() => () => (isLassoActive = true)}
        >
          Lasso Mode
        </button>
        <button
          class={['xy-theme__button', !isLassoActive && 'active']}
          onclick={() => () => (isLassoActive = false)}
        >
          Selection Mode
        </button>
      </div>
      <label>
        <input
          type="checkbox"
          checked={partial}
          onchange={() => (partial = !partial)}
          class="xy-theme__checkbox"
        />
        Partial selection
      </label>
    </div>
  </Panel>
</SvelteFlow>
```
