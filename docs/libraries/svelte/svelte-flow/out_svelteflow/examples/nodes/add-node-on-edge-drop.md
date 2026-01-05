# Add Node On Edge Drop

You can create a new node when you drop the connection line on the pane by using the `onConnectStart` and `onConnectEnd` handlers.

App.svelte

Flow.svelte

xy-theme.css

index.css

<img src="out_svelteflow/examples/nodes/add-node-on-edge-drop/index_media/a8615f5af2f285e6489e8b474f5d8c4d43e8c2bd.svg" class="size-5 stroke-2" />

<img src="out_svelteflow/examples/nodes/add-node-on-edge-drop/index_media/602cbd22f5011ed5cb261c84522d5ea8f82ecff9.svg" class="size-6" />

<img src="out_svelteflow/examples/nodes/add-node-on-edge-drop/index_media/7074fca34cc45baec265a1de8808fab16b667ede.svg" class="size-5 stroke-2" />

<img src="out_svelteflow/examples/nodes/add-node-on-edge-drop/index_media/824817a2c7b3c6650883ed0f84fc74ac655539f5.svg" class="size-4 fill-slate-700 stroke-slate-700" />

``` x:group
<script lang="ts">
  import { SvelteFlowProvider } from '@xyflow/svelte';
  import Flow from './Flow.svelte';
</script>
 
<!-- You need the SvelteFlowProvider so you can useSvelteFlow  -->
<SvelteFlowProvider>
  <Flow />
</SvelteFlowProvider>
```
