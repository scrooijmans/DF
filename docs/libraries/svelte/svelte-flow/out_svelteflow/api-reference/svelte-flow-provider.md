# \<SvelteFlowProvider /\>

<a href="https://github.com/xyflow/xyflow/blob/main/packages/svelte/src/lib/components/SvelteFlowProvider/SvelteFlowProvider.svelte" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]" rel="noreferrer" target="_blank">Source on GitHub <img src="out_svelteflow/api-reference/svelte-flow-provider/index_media/926b7c4e31213c3a8240847d8251d75a921b6a21.svg" class="x:inline x:align-baseline x:shrink-0" /></a>

The `<SvelteFlowProvider />` component wraps its child nodes with a Svelte context that makes it possible to access a flow’s internal state outside of the <a href="https://svelteflow.dev/api-reference/svelte-flow" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]"><code class="nextra-code" dir="ltr">&lt;SvelteFlow /&gt;</code></a> component. Many of the hooks we provide rely on this component to work.

<img src="out_svelteflow/api-reference/svelte-flow-provider/index_media/0b7eab85abc3682cb59a01670459912dfc0cfbe8.svg" class="x:max-w-6 x:shrink-0" />App.svelte

<img src="out_svelteflow/api-reference/svelte-flow-provider/index_media/4ad055c8513380c64c1f5319faccf453f477bb38.svg" class="nextra-copy-icon" />

``` x:group
<script>
  import { SvelteFlow, SvelteFlowProvider } from '@xyflow/svelte';
  import '@xyflow/svelte/dist/style.css';
 
  import Sidebar from './Sidebar.svelte';
 
  /* ... */
</script>
 
<SvelteFlowProvider>
  <SvelteFlow bind:nodes bind:edges />
  <Sidebar />
</SvelteFlowProvider>
```

<img src="out_svelteflow/api-reference/svelte-flow-provider/index_media/0b7eab85abc3682cb59a01670459912dfc0cfbe8.svg" class="x:max-w-6 x:shrink-0" />Sidebar.svelte

<img src="out_svelteflow/api-reference/svelte-flow-provider/index_media/4ad055c8513380c64c1f5319faccf453f477bb38.svg" class="nextra-copy-icon" />

``` x:group
<script>
  import { SvelteFlow, SvelteFlowProvider } from '@xyflow/svelte'
 
  // This hook will only work if the component it's used in
  // is a child of <SvelteFlowProvider />
  const nodes = useNodes()
</script>
 
<aside>
  {#each nodes.current as node (node.id)}
    <div key={node.id}>
      Node {node.id} -
        x: {node.position.x.toFixed(2)},
        y: {node.position.y.toFixed(2)}
    </div>
  {/each}
</aside>
```

<img src="out_svelteflow/api-reference/svelte-flow-provider/index_media/a7db0f47f02f813c5a264458a8e522bf6320ca5a.svg" class="x:mt-[.3em]" />

The state provided by `<SvelteFlowProvider />` is first initialized with default values. Only after the `<SvelteFlow />` component initializes, will the state be replaced with correct values. However, you can expect this to happen before the first render.

## Notes<a href="https://svelteflow.dev/api-reference/svelte-flow-provider#notes" class="x:focus-visible:nextra-focus subheading-anchor" aria-label="Permalink for this section"></a>

- If you’re using a router and want your flow’s state to persist across routes, it’s vital that you place the `<SvelteFlowProvider />` component *outside* of your router.
- If you have multiple flows on the same page you will need to use a separate `<SvelteFlowProvider />`.
