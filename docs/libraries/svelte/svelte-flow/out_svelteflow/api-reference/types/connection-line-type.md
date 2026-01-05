# ConnectionLineType

With the `connectionLineType` prop on your <a href="https://svelteflow.dev/api-reference/svelte-flow#connection-connectionLineType" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]"><code class="nextra-code" dir="ltr">&lt;SvelteFlow /&gt;</code></a> component, you can configure the type of the connection line. Svelte Flow comes with built-in support for the following types: ‘default’ (bezier), ‘straight’, ‘step’, ‘smoothstep’ and ‘simplebezier’.

``` x:group
export enum ConnectionLineType {
  Bezier = 'default',
  Straight = 'straight',
  Step = 'step',
  SmoothStep = 'smoothstep',
  SimpleBezier = 'simplebezier',
}
```
