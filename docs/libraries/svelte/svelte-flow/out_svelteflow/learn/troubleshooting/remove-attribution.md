# Remove Attribution

<img src="out_svelteflow/learn/troubleshooting/remove-attribution/index_media/a7db0f47f02f813c5a264458a8e522bf6320ca5a.svg" class="x:mt-[.3em]" />

If you’re considering removing the attribution, we’d first like to mention:

**If you’re using Svelte Flow at your organization and making money from it**, we rely on your support to keep Svelte Flow developed and maintained under an MIT License. Before you remove the attribution, <a href="https://svelteflow.dev/support-us" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]">see the ways you can support Svelte Flow to keep it running</a>.

**Are you using Svelte Flow for a personal project?** Great! Go ahead and remove the attribution. You can support us by reporting any bugs you find, sending us screenshots of your projects, and starring us on <a href="https://github.com/xyflow/xyflow" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]" rel="noreferrer" target="_blank">Github <img src="out_svelteflow/learn/troubleshooting/remove-attribution/index_media/926b7c4e31213c3a8240847d8251d75a921b6a21.svg" class="x:inline x:align-baseline x:shrink-0" /></a>. If you start making money using Svelte Flow or use it in an organization in the future, we would ask that you re-add the attribution or become a Github or Open Collective Sponsor.

Thank you for supporting us ✌🏻

- <a href="https://xyflow.com/about" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]" rel="noreferrer" target="_blank">the xyflow team <img src="out_svelteflow/learn/troubleshooting/remove-attribution/index_media/926b7c4e31213c3a8240847d8251d75a921b6a21.svg" class="x:inline x:align-baseline x:shrink-0" /></a>

To remove our attribution in the corner of your application you can pass `hideAttribution` through `proOptions` prop to the `<SvelteFlow />` component.

<img src="out_svelteflow/learn/troubleshooting/remove-attribution/index_media/0b7eab85abc3682cb59a01670459912dfc0cfbe8.svg" class="x:max-w-6 x:shrink-0" />App.svelte

<img src="out_svelteflow/learn/troubleshooting/remove-attribution/index_media/4ad055c8513380c64c1f5319faccf453f477bb38.svg" class="nextra-copy-icon" />

``` x:group
<script>
  import { SvelteFlow } from '@xyflow/svelte';
  import '@xyflow/svelte/dist/style.css';
</script>
 
<SvelteFlow
  proOptions={{ hideAttribution: true }}
/>
```
