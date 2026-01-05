# PanelPosition

This type is mostly used to help position things on top of the flow viewport. For example both the <a href="https://svelteflow.dev/api-reference/components/mini-map" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]"><code class="nextra-code" dir="ltr">&lt;MiniMap /&gt;</code></a> and <a href="https://svelteflow.dev/api-reference/components/controls" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]"><code class="nextra-code" dir="ltr">&lt;Controls /&gt;</code></a> components take a `position` prop of this type.

``` x:group
export type PanelPosition =
  | 'top-left'
  | 'top-center'
  | 'top-right'
  | 'bottom-left'
  | 'bottom-center'
  | 'bottom-right'
  | 'center-left'
  | 'center-right';
```

## Fields<a href="https://svelteflow.dev/api-reference/types/panel-position#fields" class="x:focus-visible:nextra-focus subheading-anchor" aria-label="Permalink for this section"></a>
