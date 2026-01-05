# useSvelteFlow

This example demonstrates how to use the `useSvelteFlow` store and actions in your Svelte components. Learn how to access and manipulate the flow state, handle events, and implement custom functionality using Svelte Flow’s built-in utilities.

App.svelte

ControlButtons.svelte

Flow.svelte

xy-theme.css

index.css

<img src="out_svelteflow/examples/misc/use-svelte-flow/index_media/a8615f5af2f285e6489e8b474f5d8c4d43e8c2bd.svg" class="size-5 stroke-2" />

<img src="out_svelteflow/examples/misc/use-svelte-flow/index_media/602cbd22f5011ed5cb261c84522d5ea8f82ecff9.svg" class="size-6" />

<img src="out_svelteflow/examples/misc/use-svelte-flow/index_media/7074fca34cc45baec265a1de8808fab16b667ede.svg" class="size-5 stroke-2" />

<img src="out_svelteflow/examples/misc/use-svelte-flow/index_media/824817a2c7b3c6650883ed0f84fc74ac655539f5.svg" class="size-4 fill-slate-700 stroke-slate-700" />

``` x:group
<script lang="ts">
  import { SvelteFlowProvider } from '@xyflow/svelte';
 
  import Flow from './Flow.svelte';
  import ControlButtons from './ControlButtons.svelte';
</script>
 
<SvelteFlowProvider>
  <Flow />
  <ControlButtons />
</SvelteFlowProvider>
```
