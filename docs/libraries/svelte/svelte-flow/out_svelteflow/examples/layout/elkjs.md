# ELK.js Layout

This example demonstrates how to use <a href="https://github.com/kieler/elkjs" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]" rel="noreferrer" target="_blank">ELK.js <img src="out_svelteflow/examples/layout/elkjs/index_media/926b7c4e31213c3a8240847d8251d75a921b6a21.svg" class="x:inline x:align-baseline x:shrink-0" /></a> for automatic graph layout in Svelte Flow. ELK.js is a powerful layout engine that can automatically arrange nodes and edges in various ways, such as hierarchical, force-directed, or circular layouts.

App.svelte

Flow.svelte

xy-theme.css

index.css

nodes-and-edges.ts

<img src="out_svelteflow/examples/layout/elkjs/index_media/a8615f5af2f285e6489e8b474f5d8c4d43e8c2bd.svg" class="size-5 stroke-2" />

<img src="out_svelteflow/examples/layout/elkjs/index_media/602cbd22f5011ed5cb261c84522d5ea8f82ecff9.svg" class="size-6" />

<img src="out_svelteflow/examples/layout/elkjs/index_media/7074fca34cc45baec265a1de8808fab16b667ede.svg" class="size-5 stroke-2" />

<img src="out_svelteflow/examples/layout/elkjs/index_media/824817a2c7b3c6650883ed0f84fc74ac655539f5.svg" class="size-4 fill-slate-700 stroke-slate-700" />

``` x:group
<script lang="ts">
  import { SvelteFlowProvider } from '@xyflow/svelte';
  import Flow from './Flow.svelte';
</script>
 
<SvelteFlowProvider>
  <Flow />
</SvelteFlowProvider>
```
