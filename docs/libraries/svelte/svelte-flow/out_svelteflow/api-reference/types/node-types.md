# NodeTypes

<a href="https://github.com/xyflow/xyflow/blob/main/packages/svelte/src/lib/types/nodes.ts#L33" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]" rel="noreferrer" target="_blank">Source on GitHub <img src="out_svelteflow/api-reference/types/node-types/index_media/926b7c4e31213c3a8240847d8251d75a921b6a21.svg" class="x:inline x:align-baseline x:shrink-0" /></a>

The `NodeTypes` type is a record that maps node type identifiers to their corresponding Svelte components. This allows you to define custom node types and their implementations.

``` x:group
type NodeTypes = Record<string, SvelteComponent>;
```
