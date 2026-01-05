# \<ControlButton /\>

<a href="https://github.com/xyflow/xyflow/blob/main/packages/svelte/src/lib/plugins/Controls/ControlButton.svelte" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]" rel="noreferrer" target="_blank">Source on GitHub <img src="out_svelteflow/api-reference/components/control-button/index_media/926b7c4e31213c3a8240847d8251d75a921b6a21.svg" class="x:inline x:align-baseline x:shrink-0" /></a>

The `<ControlButton />` component is used to render custom/ additional buttons for the `<Controls />` component.

``` x:group
<script lang="ts">
  import { Controls, ControlButton } from '@xyflow/svelte';
</script>
 
<Controls>
  <ControlButton onclick={() => console.log('⚡️')}>
    ⚡️
  </ControlButton>
</Controls>
```

## Props<a href="https://svelteflow.dev/api-reference/components/control-button#props" class="x:focus-visible:nextra-focus subheading-anchor" aria-label="Permalink for this section"></a>

The type for props of `<ControlButton />` component is exported as `ControlButtonProps`. Additionally, it extends the props of `<button />`.

| Name | Type | Default |
|----|----|----|
| <a href="https://svelteflow.dev/api-reference/components/control-button#bgcolor" class="x:absolute x:top-0 x:right-0 x:text-lg x:font-black x:before:content-[&quot;#&quot;] x:hover:text-black x:dark:hover:text-white x:px-3 x:py-[min(1%,12px)] x:group-hover:opacity-100! x:lg:top-1/2 x:lg:right-full x:lg:-translate-y-1/2"></a>`bgColor` | `string` |  |
| <a href="https://svelteflow.dev/api-reference/components/control-button#bgcolorhover" class="x:absolute x:top-0 x:right-0 x:text-lg x:font-black x:before:content-[&quot;#&quot;] x:hover:text-black x:dark:hover:text-white x:px-3 x:py-[min(1%,12px)] x:group-hover:opacity-100! x:lg:top-1/2 x:lg:right-full x:lg:-translate-y-1/2"></a>`bgColorHover` | `string` |  |
| <a href="https://svelteflow.dev/api-reference/components/control-button#colorhover" class="x:absolute x:top-0 x:right-0 x:text-lg x:font-black x:before:content-[&quot;#&quot;] x:hover:text-black x:dark:hover:text-white x:px-3 x:py-[min(1%,12px)] x:group-hover:opacity-100! x:lg:top-1/2 x:lg:right-full x:lg:-translate-y-1/2"></a>`colorHover` | `string` |  |
| <a href="https://svelteflow.dev/api-reference/components/control-button#bordercolor" class="x:absolute x:top-0 x:right-0 x:text-lg x:font-black x:before:content-[&quot;#&quot;] x:hover:text-black x:dark:hover:text-white x:px-3 x:py-[min(1%,12px)] x:group-hover:opacity-100! x:lg:top-1/2 x:lg:right-full x:lg:-translate-y-1/2"></a>`borderColor` | `string` |  |
| <a href="https://svelteflow.dev/api-reference/components/control-button#props" class="x:absolute x:top-0 x:right-0 x:text-lg x:font-black x:before:content-[&quot;#&quot;] x:hover:text-black x:dark:hover:text-white x:px-3 x:py-[min(1%,12px)] x:group-hover:opacity-100! x:lg:top-1/2 x:lg:right-full x:lg:-translate-y-1/2"></a>`...props` | `HTMLButtonAttributes` |  |
