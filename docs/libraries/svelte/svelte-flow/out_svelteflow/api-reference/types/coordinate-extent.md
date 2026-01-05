# CoordinateExtent

A coordinate extent represents two points in a coordinate system: one in the top left corner and one in the bottom right corner. It is used to represent the bounds of nodes in the flow or the bounds of the viewport.

``` x:group
export type CoordinateExtent = [[number, number], [number, number]];
```

## Notes<a href="https://svelteflow.dev/api-reference/types/coordinate-extent#notes" class="x:focus-visible:nextra-focus subheading-anchor" aria-label="Permalink for this section"></a>

- Props (like nodeExtent or translateExtent) that expect a `CoordinateExtent` usually default to `[[-∞, -∞], [+∞, +∞]]` to represent an unbounded extent.
