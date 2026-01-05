Description: svelte/reactivity • Svelte documentation

Title: svelte/reactivity • Svelte Docs

Skip to main content

### On this page

Svelte provides reactive versions of various built-ins like `Map`, `Set` and `URL` that can be used just like their native counterparts, as well as a handful of additional utilities for handling reactivity.

```
import {
class MediaQueryCreates a media query and provides a current property that reflects whether or not it matches.
Use it carefully — during server-side rendering, there is no way to know what the correct value should be, potentially causing content to change upon hydration.
If you can use the media query in CSS to achieve the same effect, do that.
&#x3C;script>
import { MediaQuery } from 'svelte/reactivity';

const large = new MediaQuery('min-width: 800px');
&#x3C;/script>

&#x3C;h1>{large.current ? 'large screen' : 'small screen'}&#x3C;/h1>@extendsReactiveValue<boolean> *@since5.7.0MediaQuery,
class SvelteDateA reactive version of the built-in Date object.
Reading the date (whether with methods like date.getTime() or date.toString(), or via things like Intl.DateTimeFormat)
in an effect or derived
will cause it to be re-evaluated when the value of the date changes.
&#x3C;script>
import { SvelteDate } from 'svelte/reactivity';

const date = new SvelteDate();

const formatter = new Intl.DateTimeFormat(undefined, {
hour: 'numeric',
minute: 'numeric',
second: 'numeric'
});

$effect(() => {
const interval = setInterval(() => {
date.setTime(Date.now());
}, 1000);

return () => {
clearInterval(interval);
};
});
&#x3C;/script>

&#x3C;p>The time is {formatter.format(date)}&#x3C;/p>SvelteDate,
class SvelteMap<K, V>A reactive version of the built-in Map object.
Reading contents of the map (by iterating, or by reading map.size or calling map.get(...) or map.has(...) as in the tic-tac-toe example below) in an effect or derived
will cause it to be re-evaluated as necessary when the map is updated.
Note that values in a reactive map are not made deeply reactive.
&#x3C;script>
import { SvelteMap } from 'svelte/reactivity';
import { result } from './game.js';

let board = new SvelteMap();
let player = $state('x');
let winner = $derived(result(board));

function reset() {
player = 'x';
board.clear();
}
&#x3C;/script>

&#x3C;div class="board">
{#each Array(9), i}
&#x3C;button
disabled={board.has(i) || winner}
onclick={() => {
board.set(i, player);
player = player === 'x' ? 'o' : 'x';
}}
>{board.get(i)}&#x3C;/button>
{/each}
&#x3C;/div>

{#if winner}
&#x3C;p>{winner} wins!&#x3C;/p>
&#x3C;button onclick={reset}>reset&#x3C;/button>
{:else}
&#x3C;p>{player} is next&#x3C;/p>
{/if}SvelteMap,
class SvelteSet<T>A reactive version of the built-in Set object.
Reading contents of the set (by iterating, or by reading set.size or calling set.has(...) as in the example below) in an effect or derived
will cause it to be re-evaluated as necessary when the set is updated.
Note that values in a reactive set are not made deeply reactive.
&#x3C;script>
import { SvelteSet } from 'svelte/reactivity';
let monkeys = new SvelteSet();

function toggle(monkey) {
if (monkeys.has(monkey)) {
monkeys.delete(monkey);
} else {
monkeys.add(monkey);
}
}
&#x3C;/script>

{#each ['🙈', '🙉', '🙊'] as monkey}
&#x3C;button onclick={() => toggle(monkey)}>{monkey}&#x3C;/button>
{/each}

&#x3C;button onclick={() => monkeys.clear()}>clear&#x3C;/button>

{#if monkeys.has('🙈')}&#x3C;p>see no evil&#x3C;/p>{/if}
{#if monkeys.has('🙉')}&#x3C;p>hear no evil&#x3C;/p>{/if}
{#if monkeys.has('🙊')}&#x3C;p>speak no evil&#x3C;/p>{/if}SvelteSet,
class SvelteURLA reactive version of the built-in URL object.
Reading properties of the URL (such as url.href or url.pathname) in an effect or derived
will cause it to be re-evaluated as necessary when the URL changes.
The searchParams property is an instance of SvelteURLSearchParams.
Example:
&#x3C;script>
import { SvelteURL } from 'svelte/reactivity';

const url = new SvelteURL('https://example.com/path');
&#x3C;/script>

&#x3C;!-- changes to these... -->
&#x3C;input bind:value={url.protocol} />
&#x3C;input bind:value={url.hostname} />
&#x3C;input bind:value={url.pathname} />

&#x3C;hr />

&#x3C;!-- will update `href` and vice versa -->
&#x3C;input bind:value={url.href} size="65" />SvelteURL,
class SvelteURLSearchParamsA reactive version of the built-in URLSearchParams object.
Reading its contents (by iterating, or by calling params.get(...) or params.getAll(...) as in the example below) in an effect or derived
will cause it to be re-evaluated as necessary when the params are updated.
&#x3C;script>
import { SvelteURLSearchParams } from 'svelte/reactivity';

const params = new SvelteURLSearchParams('message=hello');

let key = $state('key');
let value = $state('value');
&#x3C;/script>

&#x3C;input bind:value={key} />
&#x3C;input bind:value={value} />
&#x3C;button onclick={() => params.append(key, value)}>append&#x3C;/button>

&#x3C;p>?{params.toString()}&#x3C;/p>

{#each params as [key, value]}
&#x3C;p>{key}: {value}&#x3C;/p>
{/each}SvelteURLSearchParams,
function createSubscriber(start: (update: () => void) => (() => void) | void): () => voidReturns a subscribe function that integrates external event-based systems with Svelte’s reactivity.
It’s particularly useful for integrating with web APIs like MediaQuery, IntersectionObserver, or WebSocket.
If subscribe is called inside an effect (including indirectly, for example inside a getter),
the start callback will be called with an update function. Whenever update is called, the effect re-runs.
If start returns a cleanup function, it will be called when the effect is destroyed.
If subscribe is called in multiple effects, start will only be called once as long as the effects
are active, and the returned teardown function will only be called when all effects are destroyed.
It’s best understood with an example. Here’s an implementation of MediaQuery:
import { createSubscriber } from 'svelte/reactivity';
import { on } from 'svelte/events';

export class MediaQuery {
#query;
#subscribe;

constructor(query) {
this.#query = window.matchMedia(`(${query})`);

this.#subscribe = createSubscriber((update) => {
// when the `change` event occurs, re-run any effects that read `this.current`
const off = on(this.#query, 'change', update);

// stop listening when all the effects are destroyed
return () => off();
});
}

get current() {
// This makes the getter reactive, if read in an effect
this.#subscribe();

// Return the current state of the query, whether or not we're in an effect
return this.#query.matches;
}
}@since5.7.0createSubscriber
} from 'svelte/reactivity';
```

## MediaQuery

> Available since 5.7.0

Creates a media query and provides a `current` property that reflects whether or not it matches.

Use it carefully — during server-side rendering, there is no way to know what the correct value should be, potentially causing content to change upon hydration. If you can use the media query in CSS to achieve the same effect, do that.

```
<script>
import { MediaQuery } from 'svelte/reactivity';

const large = new MediaQuery('min-width: 800px');
</script>

<h1>{large.current ? 'large screen' : 'small screen'}</h1>
```

```
class MediaQuery extends ReactiveValue<boolean> {…}
```

```
constructor(query: string, fallback?: boolean | undefined);
```

- `query` A media query string
- `fallback` Fallback value for the server

## SvelteDate

A reactive version of the built-in `Date` object. Reading the date (whether with methods like `date.getTime()` or `date.toString()`, or via things like `Intl.DateTimeFormat`) in an effect or derived will cause it to be re-evaluated when the value of the date changes.

```
<script>
import { SvelteDate } from 'svelte/reactivity';

const date = new SvelteDate();

const formatter = new Intl.DateTimeFormat(undefined, {
hour: 'numeric',
minute: 'numeric',
second: 'numeric'
});

$effect(() => {
const interval = setInterval(() => {
date.setTime(Date.now());
}, 1000);

return () => {
clearInterval(interval);
};
});
</script>

<p>The time is {formatter.format(date)}</p>
```

```
class SvelteDate extends Date {…}
```

```
constructor(...params: any[]);
```

## SvelteMap

A reactive version of the built-in `Map` object. Reading contents of the map (by iterating, or by reading `map.size` or calling `map.get(...)` or `map.has(...)` as in the tic-tac-toe example below) in an effect or derived will cause it to be re-evaluated as necessary when the map is updated.

Note that values in a reactive map are _not_ made deeply reactive.

```
<script>
import { SvelteMap } from 'svelte/reactivity';
import { result } from './game.js';

let board = new SvelteMap();
let player = $state('x');
let winner = $derived(result(board));

function reset() {
player = 'x';
board.clear();
}
</script>

<div class="board">
{#each Array(9), i}
<button
disabled={board.has(i) || winner}
onclick={() => {
board.set(i, player);
player = player === 'x' ? 'o' : 'x';
}}
>{board.get(i)}</button>
{/each}
</div>

{#if winner}
<p>{winner} wins!</p>
<button onclick={reset}>reset</button>
{:else}
<p>{player} is next</p>
{/if}
```

```
class SvelteMap<K, V> extends Map<K, V> {…}
```

```
constructor(value?: Iterable<readonly [K, V]> | null | undefined);
```

```
set(key: K, value: V): this;
```

## SvelteSet

A reactive version of the built-in `Set` object. Reading contents of the set (by iterating, or by reading `set.size` or calling `set.has(...)` as in the example below) in an effect or derived will cause it to be re-evaluated as necessary when the set is updated.

Note that values in a reactive set are _not_ made deeply reactive.

```
<script>
import { SvelteSet } from 'svelte/reactivity';
let monkeys = new SvelteSet();

function toggle(monkey) {
if (monkeys.has(monkey)) {
monkeys.delete(monkey);
} else {
monkeys.add(monkey);
}
}
</script>

{#each ['🙈', '🙉', '🙊'] as monkey}
<button onclick={() => toggle(monkey)}>{monkey}</button>
{/each}

<button onclick={() => monkeys.clear()}>clear</button>

{#if monkeys.has('🙈')}<p>see no evil</p>{/if}
{#if monkeys.has('🙉')}<p>hear no evil</p>{/if}
{#if monkeys.has('🙊')}<p>speak no evil</p>{/if}
```

```
class SvelteSet<T> extends Set<T> {…}
```

```
constructor(value?: Iterable<T> | null | undefined);
```

```
add(value: T): this;
```

## SvelteURL

A reactive version of the built-in `URL` object. Reading properties of the URL (such as `url.href` or `url.pathname`) in an effect or derived will cause it to be re-evaluated as necessary when the URL changes.

The `searchParams` property is an instance of SvelteURLSearchParams.

Example:

```
<script>
import { SvelteURL } from 'svelte/reactivity';

const url = new SvelteURL('https://example.com/path');
</script>

<!-- changes to these... -->
<input bind:value={url.protocol} />
<input bind:value={url.hostname} />
<input bind:value={url.pathname} />

<hr />

<!-- will update `href` and vice versa -->
<input bind:value={url.href} size="65" />
```

```
class SvelteURL extends URL {…}
```

```
get searchParams(): SvelteURLSearchParams;
```

## SvelteURLSearchParams

A reactive version of the built-in `URLSearchParams` object. Reading its contents (by iterating, or by calling `params.get(...)` or `params.getAll(...)` as in the example below) in an effect or derived will cause it to be re-evaluated as necessary when the params are updated.

```
<script>
import { SvelteURLSearchParams } from 'svelte/reactivity';

const params = new SvelteURLSearchParams('message=hello');

let key = $state('key');
let value = $state('value');
</script>

<input bind:value={key} />
<input bind:value={value} />
<button onclick={() => params.append(key, value)}>append</button>

<p>?{params.toString()}</p>

{#each params as [key, value]}
<p>{key}: {value}</p>
{/each}
```

```
class SvelteURLSearchParams extends URLSearchParams {…}
```

```
[REPLACE](params:URLSearchParams): void;
```

## createSubscriber

> Available since 5.7.0

Returns a `subscribe` function that integrates external event-based systems with Svelte’s reactivity. It’s particularly useful for integrating with web APIs like `MediaQuery`, `IntersectionObserver`, or `WebSocket`.

If `subscribe` is called inside an effect (including indirectly, for example inside a getter), the `start` callback will be called with an `update` function. Whenever `update` is called, the effect re-runs.

If `start` returns a cleanup function, it will be called when the effect is destroyed.

If `subscribe` is called in multiple effects, `start` will only be called once as long as the effects are active, and the returned teardown function will only be called when all effects are destroyed.

It’s best understood with an example. Here’s an implementation of `MediaQuery`:

```
import { function createSubscriber(start: (update: () => void) => (() => void) | void): () => voidReturns a subscribe function that integrates external event-based systems with Svelte’s reactivity.
It’s particularly useful for integrating with web APIs like MediaQuery, IntersectionObserver, or WebSocket.
If subscribe is called inside an effect (including indirectly, for example inside a getter),
the start callback will be called with an update function. Whenever update is called, the effect re-runs.
If start returns a cleanup function, it will be called when the effect is destroyed.
If subscribe is called in multiple effects, start will only be called once as long as the effects
are active, and the returned teardown function will only be called when all effects are destroyed.
It’s best understood with an example. Here’s an implementation of MediaQuery:
import { createSubscriber } from 'svelte/reactivity';
import { on } from 'svelte/events';

export class MediaQuery {
#query;
#subscribe;

constructor(query) {
this.#query = window.matchMedia(`(${query})`);

this.#subscribe = createSubscriber((update) => {
// when the `change` event occurs, re-run any effects that read `this.current`
const off = on(this.#query, 'change', update);

// stop listening when all the effects are destroyed
return () => off();
});
}

get current() {
// This makes the getter reactive, if read in an effect
this.#subscribe();

// Return the current state of the query, whether or not we're in an effect
return this.#query.matches;
}
}@since5.7.0createSubscriber } from 'svelte/reactivity';
import { function on<Type extends keyof WindowEventMap>(window: Window, type: Type, handler: (this: Window, event: WindowEventMap[Type]) => any, options?: AddEventListenerOptions | undefined): () => void (+4 overloads)Attaches an event handler to the window and returns a function that removes the handler. Using this
rather than addEventListener will preserve the correct order relative to handlers added declaratively
(with attributes like onclick), which use event delegation for performance reasons
on } from 'svelte/events';

export class class MediaQueryMediaQuery {
#query;
#subscribe;

constructor(query: anyquery) {
this.#query = var window: Window & typeof globalThisMDN Reference
window.function matchMedia(query: string): MediaQueryListMDN Reference
matchMedia(`(${query: anyquery})`);

this.#subscribe = function createSubscriber(start: (update: () => void) => (() => void) | void): () => voidReturns a subscribe function that integrates external event-based systems with Svelte’s reactivity.
It’s particularly useful for integrating with web APIs like MediaQuery, IntersectionObserver, or WebSocket.
If subscribe is called inside an effect (including indirectly, for example inside a getter),
the start callback will be called with an update function. Whenever update is called, the effect re-runs.
If start returns a cleanup function, it will be called when the effect is destroyed.
If subscribe is called in multiple effects, start will only be called once as long as the effects
are active, and the returned teardown function will only be called when all effects are destroyed.
It’s best understood with an example. Here’s an implementation of MediaQuery:
import { createSubscriber } from 'svelte/reactivity';
import { on } from 'svelte/events';

export class MediaQuery {
#query;
#subscribe;

constructor(query) {
this.#query = window.matchMedia(`(${query})`);

this.#subscribe = createSubscriber((update) => {
// when the `change` event occurs, re-run any effects that read `this.current`
const off = on(this.#query, 'change', update);

// stop listening when all the effects are destroyed
return () => off();
});
}

get current() {
// This makes the getter reactive, if read in an effect
this.#subscribe();

// Return the current state of the query, whether or not we're in an effect
return this.#query.matches;
}
}@since5.7.0createSubscriber((update: () => voidupdate) => {
// when the `change` event occurs, re-run any effects that read `this.current`
const const off: () => voidoff = on<MediaQueryList, "change">(element: MediaQueryList, type: "change", handler: (this: MediaQueryList, event: MediaQueryListEvent) => any, options?: AddEventListenerOptions | undefined): () => void (+4 overloads)Attaches an event handler to an element and returns a function that removes the handler. Using this
rather than addEventListener will preserve the correct order relative to handlers added declaratively
(with attributes like onclick), which use event delegation for performance reasons
on(this.#query, 'change', update: () => voidupdate);

// stop listening when all the effects are destroyed
return () => const off: () => voidoff();
});
}

get MediaQuery.current: booleancurrent() {
// This makes the getter reactive, if read in an effect
this.#subscribe();

// Return the current state of the query, whether or not we're in an effect
return this.#query.MediaQueryList.matches: booleanMDN Reference
matches;
}
}
```

```
function createSubscriber(
start: (update: () => void) => (() => void) | void
): () => void;
```

Edit this page on GitHub llms.txt

previous next

svelte/reactivity/window svelte/server
