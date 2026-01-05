Description: $derived • Svelte documentation

Title: $derived • Docs • Svelte

Skip to main content

### On this page

Derived state is declared with the `$derived` rune:

```
<script>
let count = $state(0);
let doubled = $derived(count * 2);
</script>

<button onclick={() => count++}>
{doubled}
</button>

<p>{count} doubled is {doubled}</p>
```

The expression inside `$derived(...)` should be free of side-effects. Svelte will disallow state changes (e.g. `count++`) inside derived expressions.

As with `$state`, you can mark class fields as `$derived`.

> Code in Svelte components is only executed once at creation. Without the `$derived` rune, `doubled` would maintain its original value even when `count` changes.

## $derived.by

Sometimes you need to create complex derivations that don’t fit inside a short expression. In these cases, you can use `$derived.by` which accepts a function as its argument.

```
<script>
let numbers = $state([1, 2, 3]);
let total = $derived.by(() => {
let total = 0;
for (const n of numbers) {
total += n;
}
return total;
});
</script>

<button onclick={() => numbers.push(numbers.length + 1)}>
{numbers.join(' + ')} = {total}
</button>
```

In essence, `$derived(expression)` is equivalent to `$derived.by(() => expression)`.

## Understanding dependencies

Anything read synchronously inside the `$derived` expression (or `$derived.by` function body) is considered a _dependency_ of the derived state. When the state changes, the derived will be marked as _dirty_ and recalculated when it is next read.

To exempt a piece of state from being treated as a dependency, use `untrack`.

## Overriding derived values

Derived expressions are recalculated when their dependencies change, but you can temporarily override their values by reassigning them (unless they are declared with `const`). This can be useful for things like _optimistic UI_, where a value is derived from the ‘source of truth’ (such as data from your server) but you’d like to show immediate feedback to the user:

```
<script>
let { post, like } = $props();

let likes = $derived(post.likes);

async function onclick() {
// increment the `likes` count immediately...
likes += 1;

// and tell the server, which will eventually update `post`
try {
await like();
} catch {
// failed! roll back the change
likes -= 1;
}
}
</script>

<button {onclick}>🧡 {likes}</button>
```

> Prior to Svelte 5.25, deriveds were read-only.

## Deriveds and reactivity

Unlike `$state`, which converts objects and arrays to deeply reactive proxies, `$derived` values are left as-is. For example, in a case like this...

```
let items = $state([...]);

let index = $state(0);
let selected = $derived(items[index]);
```

...you can change (or `bind:` to) properties of `selected` and it will affect the underlying `items` array. If `items` was _not_ deeply reactive, mutating `selected` would have no effect.

## Destructuring

If you use destructuring with a `$derived` declaration, the resulting variables will all be reactive — this...

```
let { let a: numbera, let b: numberb, let c: numberc } = function $derived<{
a: number;
b: number;
c: number;
}>(expression: {
a: number;
b: number;
c: number;
}): {
a: number;
b: number;
c: number;
}
namespace $derivedDeclares derived state, i.e. one that depends on other state variables.
The expression inside $derived(...) should be free of side-effects.
Example:
let double = $derived(count * 2);https://svelte.dev/docs/svelte/$derived
@paramexpression The derived state expression$derived(function stuff(): {
a: number;
b: number;
c: number;
}stuff());
```

...is roughly equivalent to this:

```
let let _stuff: {
a: number;
b: number;
c: number;
}_stuff = function $derived<{
a: number;
b: number;
c: number;
}>(expression: {
a: number;
b: number;
c: number;
}): {
a: number;
b: number;
c: number;
}
namespace $derivedDeclares derived state, i.e. one that depends on other state variables.
The expression inside $derived(...) should be free of side-effects.
Example:
let double = $derived(count * 2);https://svelte.dev/docs/svelte/$derived
@paramexpression The derived state expression$derived(function stuff(): {
a: number;
b: number;
c: number;
}stuff());
let let a: numbera = function $derived<number>(expression: number): number
namespace $derivedDeclares derived state, i.e. one that depends on other state variables.
The expression inside $derived(...) should be free of side-effects.
Example:
let double = $derived(count * 2);https://svelte.dev/docs/svelte/$derived
@paramexpression The derived state expression$derived(let _stuff: {
a: number;
b: number;
c: number;
}_stuff.a: numbera);
let let b: numberb = function $derived<number>(expression: number): number
namespace $derivedDeclares derived state, i.e. one that depends on other state variables.
The expression inside $derived(...) should be free of side-effects.
Example:
let double = $derived(count * 2);https://svelte.dev/docs/svelte/$derived
@paramexpression The derived state expression$derived(let _stuff: {
a: number;
b: number;
c: number;
}_stuff.b: numberb);
let let c: numberc = function $derived<number>(expression: number): number
namespace $derivedDeclares derived state, i.e. one that depends on other state variables.
The expression inside $derived(...) should be free of side-effects.
Example:
let double = $derived(count * 2);https://svelte.dev/docs/svelte/$derived
@paramexpression The derived state expression$derived(let _stuff: {
a: number;
b: number;
c: number;
}_stuff.c: numberc);
```

## Update propagation

Svelte uses something called _push-pull reactivity_ — when state is updated, everything that depends on the state (whether directly or indirectly) is immediately notified of the change (the ‘push’), but derived values are not re-evaluated until they are actually read (the ‘pull’).

If the new value of a derived is referentially identical to its previous value, downstream updates will be skipped. In other words, Svelte will only update the text inside the button when `large` changes, not when `count` changes, even though `large` depends on `count`:

```
<script>
let count = $state(0);
let large = $derived(count > 10);
</script>

<button onclick={() => count++}>
{large}
</button>
```

Edit this page on GitHub llms.txt

previous next

$state $effect
