# Position

While <a href="https://svelteflow.dev/api-reference/types/panel-position" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]"><code class="nextra-code" dir="ltr">PanelPosition</code></a> can be used to place a component in the corners of a container, the `Position` enum is less precise and used primarily in relation to edges and handles.

``` x:group
export enum Position {
  Left = 'left',
  Top = 'top',
  Right = 'right',
  Bottom = 'bottom',
}
```
