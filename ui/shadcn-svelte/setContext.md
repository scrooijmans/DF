Title: Context API / setContext and getContext • Svelte Tutorial

Skip to main content

Basic Svelte Introduction

- Welcome to Svelte
- Your first component
- Dynamic attributes
- Styling
- Nested components
- HTML tags

Reactivity

- State
- Deep state
- Derived state
- Inspecting state
- Effects
- Universal reactivity

Props

- Declaring props
- Default values
- Spread props

Logic

- If blocks
- Else blocks
- Else-if blocks
- Each blocks
- Keyed each blocks
- Await blocks

Events

- DOM events
- Inline handlers
- Capturing
- Component events
- Spreading events

Bindings

- Text inputs
- Numeric inputs
- Checkbox inputs
- Select bindings
- Group inputs
- Select multiple
- Textarea inputs

Classes and styles

- The class attribute
- The style directive
- Component styles

Actions

- The use directive
- Adding parameters

Transitions

- The transition directive
- Adding parameters
- In and out
- Custom CSS transitions
- Custom JS transitions
- Transition events
- Global transitions
- Key blocks

Advanced Svelte Advanced reactivity

- Raw state
- Reactive classes
- Getters and setters
- Reactive built-ins
- Stores

Reusing content

- Snippets and render tags
- Passing snippets to components
- Implicit snippet props

Motion

- Tweened values
- Springs

Advanced bindings

- Contenteditable bindings
- Each block bindings
- Media elements
- Dimensions
- This
- Component bindings
- Binding to component instances

Advanced transitions

- Deferred transitions
- Animations

Context API

- setContext and getContext

Special elements

- <svelte:window>
- <svelte:window> bindings
- <svelte:document>
- <svelte:body>
- <svelte:head>
- <svelte:element>
- <svelte:boundary>

<script module>

*   Sharing code
*   Exports

Next steps

*   Congratulations!

Basic SvelteKit Introduction

*   What is SvelteKit?

Routing

*   Pages
*   Layouts
*   Route parameters

Loading data

*   Page data
*   Layout data

Headers and cookies

*   Setting headers
*   Reading and writing cookies

Shared modules

*   The $lib alias

Forms

*   The <form> element
*   Named form actions
*   Validation
*   Progressive enhancement
*   Customizing use:enhance

API routes

*   GET handlers
*   POST handlers
*   Other handlers

$app/state

*   page
*   navigating
*   updated

Errors and redirects

*   Basics
*   Error pages
*   Fallback errors
*   Redirects

Advanced SvelteKit Hooks

*   handle
*   The RequestEvent object
*   handleFetch
*   handleError

Page options

*   Basics
*   ssr
*   csr
*   prerender
*   trailingSlash

Link options

*   Preloading
*   Reloading the page

Advanced routing

*   Optional parameters
*   Rest parameters
*   Param matchers
*   Route groups
*   Breaking out of layouts

Advanced loading

*   Universal load functions
*   Using both load functions
*   Using parent data
*   Invalidation
*   Custom dependencies
*   invalidateAll

Environment variables

*   $env/static/private
*   $env/dynamic/private
*   $env/static/public
*   $env/dynamic/public

Conclusion

*   Next steps

Advanced Svelte Context API setContext and getContext

solve

The context API provides a mechanism for components to ‘talk’ to each other without passing around data and functions as props, or dispatching lots of events. It’s an advanced feature, but a useful one. In this exercise, we’re going to recreate Schotter by George Nees — one of the pioneers of generative art — using the context API.

Inside `Canvas.svelte`, there’s an `addItem` function that adds an item to the canvas. We can make it available to components inside `<Canvas>`, like `<Square>`, with `setContext`:

Canvas

```
import { setContext } from 'svelte';
import { SvelteSet } from 'svelte/reactivity';

let { width = 100, height = 100, children } = $props();

let canvas;
let items = new SvelteSet();

setContext('canvas', { addItem });

function addItem(fn) {
$effect(() => {
items.add(fn);
return () => items.delete(fn);
});
}
```

Inside child components, we can now get the context with, well, `getContext`:

Square

```
import { getContext } from 'svelte';

let { x, y, size, rotate } = $props();

getContext('canvas').addItem(draw);
```

So far, so... boring. Let’s add some randomness to the grid:

App

```
<div class="container">
<Canvas width={800} height={1200}>
{#each Array(12) as _, c}
{#each Array(22) as _, r}
<Square
x={180 + c * 40 + jitter(r * 2)}
y={180 + r * 40 + jitter(r * 2)}
size={40}
rotate={jitter(r * 0.05)}
/>
{/each}
{/each}
</Canvas>
</div>
```

`setContext` and `getContext` must be called during component initialisation, so that the context can be correctly bound. The key — `'canvas'` in this case — can be anything you like, including non-strings, which is useful for controlling who can access the context.

> Your context object can include anything, including reactive state. This allows you to pass values that change over time to child components:
> 
> ```
> // in a parent component
> import { setContext } from 'svelte';
> 
> let context = $state({...});
> setContext('my-context', context);
> ```
> 
> ```
> // in a child component
> import { getContext } from 'svelte';
> 
> const context = getContext('my-context');
> ```

Edit this page on GitHub

previous next

Animations <svelte:window>

*   src
*   App.svelte
*   Canvas.svelte
*   Square.svelte

1

2

3

4

5

6

7

8

9

10

11

12

13

14

15

16

17

18

19

20

21

22

23

24

25

26

27

28

29

30

31

32

33

34

35

36

37

38

39

40

41

<script>

import Canvas from './Canvas.svelte';

import Square from './Square.svelte';

// we use a seeded random number generator to get consistent jitter

let seed = 1;

function random() {

seed \*= 16807;

seed %= 2147483647;

return (seed - 1) / 2147483646;

}

function jitter(amount) {

return amount \* (random() - 0.5);

}

</script>

<div class="container">

<Canvas width={800} height={1200}>

{#each Array(12) as \_, c}

{#each Array(22) as \_, r}

<Square

x={180 + c \* 40}

y={180 + r \* 40}

size={40}

/>

{/each}

{/each}

</Canvas>

</div>

<style>

.container {

height: 100%;

aspect-ratio: 2 / 3;

margin: 0 auto;

background: rgb(224, 219, 213);

filter: drop-shadow(0.5em 0.5em 1em rgba(0, 0, 0, 0.1));

}

</style>

show text show editor
