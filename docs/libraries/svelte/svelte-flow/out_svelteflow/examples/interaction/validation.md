# Validation

Custom nodes need to have at least one <a href="https://svelteflow.dev/api-reference/components/handle" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]"><code class="nextra-code" dir="ltr">Handle</code> component</a> to be connectable. You can pass a validation function <a href="https://svelteflow.dev/api-reference/svelte-flow#isvalidconnection-1" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]"><code class="nextra-code" dir="ltr">isValidConnection</code></a> to the SvelteFlow component in order to check if a new connection is valid and should be added.

App.svelte

xy-theme.css

index.css

<img src="out_svelteflow/examples/interaction/validation/index_media/a8615f5af2f285e6489e8b474f5d8c4d43e8c2bd.svg" class="size-5 stroke-2" />

<img src="out_svelteflow/examples/interaction/validation/index_media/602cbd22f5011ed5cb261c84522d5ea8f82ecff9.svg" class="size-6" />

<img src="out_svelteflow/examples/interaction/validation/index_media/7074fca34cc45baec265a1de8808fab16b667ede.svg" class="size-5 stroke-2" />

<img src="out_svelteflow/examples/interaction/validation/index_media/824817a2c7b3c6650883ed0f84fc74ac655539f5.svg" class="size-4 fill-slate-700 stroke-slate-700" />

``` x:group
<script lang="ts">
  import {
    SvelteFlow,
    Controls,
    Background,
    BackgroundVariant,
    type IsValidConnection,
    Position,
  } from '@xyflow/svelte';
 
  import '@xyflow/svelte/dist/style.css';
 
  const nodeDefaults = {
    sourcePosition: Position.Right,
    targetPosition: Position.Left,
  };
 
  let nodes = $state.raw([
    {
      id: '0',
      position: { x: 0, y: 150 },
      data: { label: 'only connectable with B' },
      ...nodeDefaults,
    },
    {
      id: 'A',
      position: { x: 250, y: 0 },
      data: { label: 'A' },
      ...nodeDefaults,
    },
    {
      id: 'B',
      position: { x: 250, y: 150 },
      data: { label: 'B' },
      ...nodeDefaults,
    },
    {
      id: 'C',
      position: { x: 250, y: 300 },
      data: { label: 'C' },
      ...nodeDefaults,
    },
  ]);
 
  let edges = $state.raw([]);
 
  const isValidConnection: IsValidConnection = (connection) =>
    connection.target === 'B';
</script>
 
<main>
  <SvelteFlow
    bind:nodes
    bind:edges
    fitView
    minZoom={0.1}
    maxZoom={2.5}
    {isValidConnection}
  >
    <Controls />
    <Background variant={BackgroundVariant.Dots} />
  </SvelteFlow>
</main>
 
<style>
  main {
    height: 100vh;
  }
 
  :global(.svelte-flow .svelte-flow__node .svelte-flow__handle) {
    width: 8px;
    height: 8px;
  }
 
  :global(.svelte-flow .svelte-flow__handle.connectingto) {
    background: #ff6060;
  }
 
  :global(.svelte-flow .svelte-flow__handle.valid) {
    background: #55dd99;
  }
</style>
```
