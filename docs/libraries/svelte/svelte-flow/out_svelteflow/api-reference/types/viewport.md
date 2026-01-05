# Viewport

Internally, Svelte Flow maintains a coordinate system that is independent of the rest of the page. The `Viewport` type tells you where in that system your flow is currently being display at and how zoomed in or out it is.

## Fields<a href="https://svelteflow.dev/api-reference/types/viewport#fields" class="x:focus-visible:nextra-focus subheading-anchor" aria-label="Permalink for this section"></a>

| Name | Type | Default |
|----|----|----|
| <a href="https://svelteflow.dev/api-reference/types/viewport#x" class="x:absolute x:top-0 x:right-0 x:text-lg x:font-black x:before:content-[&quot;#&quot;] x:hover:text-black x:dark:hover:text-white x:px-3 x:py-[min(1%,12px)] x:group-hover:opacity-100! x:lg:top-1/2 x:lg:right-full x:lg:-translate-y-1/2"></a>`x` | `number` |  |
| <a href="https://svelteflow.dev/api-reference/types/viewport#y" class="x:absolute x:top-0 x:right-0 x:text-lg x:font-black x:before:content-[&quot;#&quot;] x:hover:text-black x:dark:hover:text-white x:px-3 x:py-[min(1%,12px)] x:group-hover:opacity-100! x:lg:top-1/2 x:lg:right-full x:lg:-translate-y-1/2"></a>`y` | `number` |  |
| <a href="https://svelteflow.dev/api-reference/types/viewport#zoom" class="x:absolute x:top-0 x:right-0 x:text-lg x:font-black x:before:content-[&quot;#&quot;] x:hover:text-black x:dark:hover:text-white x:px-3 x:py-[min(1%,12px)] x:group-hover:opacity-100! x:lg:top-1/2 x:lg:right-full x:lg:-translate-y-1/2"></a>`zoom` | `number` |  |

## Notes<a href="https://svelteflow.dev/api-reference/types/viewport#notes" class="x:focus-visible:nextra-focus subheading-anchor" aria-label="Permalink for this section"></a>

- A `Transform` has the same properties as the viewport, but they represent different things. Make sure you don’t get them muddled up or things will start to look weird!
