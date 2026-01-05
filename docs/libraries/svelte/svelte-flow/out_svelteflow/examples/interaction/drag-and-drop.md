# Drag and Drop

A drag and drop user interface is very common for node-based workflow editors. The drag and drop behavior outside of the Svelte Flow pane is not built in but can be implemented with the native <a href="https://developer.mozilla.org/en-US/docs/Web/API/HTML_Drag_and_Drop_API" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]" rel="noreferrer" target="_blank">HTML Drag and Drop API <img src="out_svelteflow/examples/interaction/drag-and-drop/index_media/926b7c4e31213c3a8240847d8251d75a921b6a21.svg" class="x:inline x:align-baseline x:shrink-0" /></a> (used in this example) or a third party library like <a href="https://github.com/sveltejs/svelte-draggable" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]" rel="noreferrer" target="_blank">svelte-draggable <img src="out_svelteflow/examples/interaction/drag-and-drop/index_media/926b7c4e31213c3a8240847d8251d75a921b6a21.svg" class="x:inline x:align-baseline x:shrink-0" /></a>.

App.svelte

DnDProvider.svelte

Flow.svelte

Sidebar.svelte

xy-theme.css

index.css

<img src="out_svelteflow/examples/interaction/drag-and-drop/index_media/a8615f5af2f285e6489e8b474f5d8c4d43e8c2bd.svg" class="size-5 stroke-2" />

<img src="out_svelteflow/examples/interaction/drag-and-drop/index_media/602cbd22f5011ed5cb261c84522d5ea8f82ecff9.svg" class="size-6" />

<img src="out_svelteflow/examples/interaction/drag-and-drop/index_media/7074fca34cc45baec265a1de8808fab16b667ede.svg" class="size-5 stroke-2" />

<img src="out_svelteflow/examples/interaction/drag-and-drop/index_media/824817a2c7b3c6650883ed0f84fc74ac655539f5.svg" class="size-4 fill-slate-700 stroke-slate-700" />

``` x:group
<script>
  import { SvelteFlowProvider } from '@xyflow/svelte';
 
  import Flow from './Flow.svelte';
  import DnDProvider from './DnDProvider.svelte';
</script>
 
<SvelteFlowProvider>
  <DnDProvider>
    <Flow />
  </DnDProvider>
</SvelteFlowProvider>
```
