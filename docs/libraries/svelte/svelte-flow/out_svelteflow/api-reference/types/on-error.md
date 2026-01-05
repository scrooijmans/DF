# OnError

<a href="https://github.com/xyflow/xyflow/blob/main/packages/system/src/types/general.ts#L88" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]" rel="noreferrer" target="_blank">Source on GitHub <img src="out_svelteflow/api-reference/types/on-error/index_media/926b7c4e31213c3a8240847d8251d75a921b6a21.svg" class="x:inline x:align-baseline x:shrink-0" /></a>

The `OnError` type is a callback function that is called when an error occurs in SvelteFlow. It receives the error ID and a message describing the error.

``` x:group
type OnError = (id: string, message: string) => void;
```

**Parameters:**

| Name | Type | Default |
|----|----|----|
| <a href="https://svelteflow.dev/api-reference/types/on-error#id" class="x:absolute x:top-0 x:right-0 x:text-lg x:font-black x:before:content-[&quot;#&quot;] x:hover:text-black x:dark:hover:text-white x:px-3 x:py-[min(1%,12px)] x:group-hover:opacity-100! x:lg:top-1/2 x:lg:right-full x:lg:-translate-y-1/2"></a>`id` | `string` |  |
| <a href="https://svelteflow.dev/api-reference/types/on-error#message" class="x:absolute x:top-0 x:right-0 x:text-lg x:font-black x:before:content-[&quot;#&quot;] x:hover:text-black x:dark:hover:text-white x:px-3 x:py-[min(1%,12px)] x:group-hover:opacity-100! x:lg:top-1/2 x:lg:right-full x:lg:-translate-y-1/2"></a>`message` | `string` |  |

**Returns:**

<a href="https://svelteflow.dev/api-reference/types/on-error#returns" class="x:absolute x:top-0 x:right-0 x:text-lg x:font-black x:before:content-[&quot;#&quot;] x:hover:text-black x:dark:hover:text-white x:px-3 x:py-[min(1%,12px)]"></a>`void`
