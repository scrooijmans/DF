# MarkerType

Svelte Flow comes with two built-in markers: `MarkerType.Arrow` and `MarkerType.ArrowClosed`. You can use these by setting the `markerStart`/ `markerEnd` <a href="https://svelteflow.dev/api-reference/types/edge" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]">Edge</a> option.

``` x:group
export enum MarkerType {
  Arrow = 'arrow',
  ArrowClosed = 'arrowclosed',
}
```
