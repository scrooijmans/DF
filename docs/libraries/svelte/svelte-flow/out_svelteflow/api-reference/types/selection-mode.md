# SelectionMode

<a href="https://github.com/xyflow/xyflow/blob/main/packages/system/src/types/general.ts#L223" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]" rel="noreferrer" target="_blank">Source on GitHub <img src="out_svelteflow/api-reference/types/selection-mode/index_media/926b7c4e31213c3a8240847d8251d75a921b6a21.svg" class="x:inline x:align-baseline x:shrink-0" /></a>

The `SelectionMode` enum determines how node selection works. Use `Full` to require the entire node to be within the selection area, or `Partial` to allow overlap.

``` x:group
enum SelectionMode {
  Partial = 'partial',
  Full = 'full',
}
```
