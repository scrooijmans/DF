# ConnectionMode

<a href="https://github.com/xyflow/xyflow/blob/main/packages/system/src/types/general.ts#L68" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]" rel="noreferrer" target="_blank">Source on GitHub <img src="out_svelteflow/api-reference/types/connection-mode/index_media/926b7c4e31213c3a8240847d8251d75a921b6a21.svg" class="x:inline x:align-baseline x:shrink-0" /></a>

The `ConnectionMode` enum defines the available strategies for connecting nodes. Use `Strict` to only allow connections to valid handles, or `Loose` to allow more flexible connections.

``` x:group
enum ConnectionMode {
  Strict = 'strict',
  Loose = 'loose',
}
```
