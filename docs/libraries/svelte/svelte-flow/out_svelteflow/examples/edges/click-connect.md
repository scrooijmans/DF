# Click Connect

You can create a new connection by clicking on a handle one by one. You can control this functionality via the <a href="https://svelteflow.dev/api-reference/svelte-flow#clickconnect" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]"><code class="nextra-code" dir="ltr">clickConnect</code></a> prop on the `<SvelteFlow />` component.

AnnotationNode.svelte

App.svelte

xy-theme.css

index.css

nodes-and-edges.ts

<img src="out_svelteflow/examples/edges/click-connect/index_media/a8615f5af2f285e6489e8b474f5d8c4d43e8c2bd.svg" class="size-5 stroke-2" />

<img src="out_svelteflow/examples/edges/click-connect/index_media/602cbd22f5011ed5cb261c84522d5ea8f82ecff9.svg" class="size-6" />

<img src="out_svelteflow/examples/edges/click-connect/index_media/7074fca34cc45baec265a1de8808fab16b667ede.svg" class="size-5 stroke-2" />

<img src="out_svelteflow/examples/edges/click-connect/index_media/824817a2c7b3c6650883ed0f84fc74ac655539f5.svg" class="size-4 fill-slate-700 stroke-slate-700" />

``` x:group
<script module>
  import type { Node, NodeProps } from '@xyflow/svelte';
 
  export type AnnotationNode = Node<{
    label: string;
    arrowStyle?: string;
  }>;
</script>
 
<script lang="ts">
  let { data }: NodeProps<AnnotationNode> = $props();
</script>
 
<div class="annotation-content">
  <div>{data.label}</div>
</div>
{#if data.arrowStyle}
  <div class="annotation-arrow" style={data.arrowStyle}>⤹</div>
{/if}
```
