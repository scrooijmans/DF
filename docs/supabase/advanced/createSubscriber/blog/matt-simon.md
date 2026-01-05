Description: Svelte 5 introduces $effect.tracking, a rune to manage tracking contexts, such as in effects or templates. This post explains why this rune was added and the problems it solves.

Keywords: mat simon, mat meno, matias simon, matias meno, web development blog, frontend blog

Title: Svelte in Depth: The $effect.tracking rune — Mat Simon

This is the 1st post in a series on Svelte, where we delve into the framework’s more advanced and intricate features, helping developers master the framework beyond the basics.

Svelte 5 is finally here! While many things have become simpler, some new features may be a bit trickier to understand. In this series, we’ll dive into the more advanced aspects in detail. This first post focuses on the `$effect.tracking` rune, exploring what it does and when you might want to use it.

## Definition

The Svelte team has already enhanced the documentation quite a bit, so there’s not much to add. Here’s the official description:

> The `$effect.tracking` rune is an advanced feature that tells you whether or not the code is running inside a tracking context, such as an effect or inside your template.
>
> This allows you to (for example) add things like subscriptions without causing memory leaks, by putting them in child effects.

They then provide an example of how a rune with subscriptions can be implemented. It’s concise, but understanding why and when you might need it may not be immediately obvious.

## Why is it Needed?

To understand why this rune was added, it’s helpful to know which functionality it replaces.

In Svelte 4, there were two reactivity models:

- Simply declaring variables in a Svelte component, letting the compiler handle reactivity.
- **Stores** that could be passed around (these still exist in Svelte 5).

Readable stores have a straightforward contract: you can subscribe to them to receive updates. To subscribe to a store, you would write:

```
const unsubscribe = myStore.subscribe((value) => console.log(value))
// And when you’re done:
unsubscribe()
```

INFO

To avoid boilerplate, Svelte (4 and 5) offers a simple `$` syntax:

```
<script>
import { myStore } from '$lib/my-stores.js'
</script>

Current store value: {$myStore}
```

Under the hood, this subscribes to the store for you and unsubscribes when the component is unmounted.

Thanks to this subscriber pattern, it’s easy to run code both when the store is subscribed to and when all subscribers unsubscribe.

With the built-in `readable` and `writable` store functions, it’s as simple as this:

```
import { readable } from 'svelte/store'

const initialValue = 'hello'

const myStore = readable(initialValue, () => {
console.log('The subscriber count went from 0 to 1')
return () => {
console.log('All subscribers have unsubscribed')
}
})
```

Even without the `readable` helper function from Svelte it would be quite easy to implement this functionality.

## Naive Implementation

Let’s say we wanted to implement similar subscriber count functionality in Svelte 5, with **runes** instead of stores. To do this, we can use the `$effect()` rune which creates a tracking context managed by the component’s lifecycle.

INFO

Because this topic is a bit complex, it’s important to establish a consistent terminology.

- **Tracking Context**: The code inside an `$effect()`. Svelte tracks which pieces of state (and derived state) are accessed inside the tracking context, and re-runs the function when that state later changes.
- **Tracking Scope**: In order for a tracking context to be created and managed, the effect must be created inside a **tracking scope**. Normally this scope is created automatically by Svelte and is tied to the component lifecycle (but you can also create your own with `$effect.root()`; more on this in the next post). The scope is also in charge of cleaning up the effects created inside it.

I want to emphasise that the lifecycle of a tracking **context** is always tied to the **scope** it is created in.

Here’s a naive (_but incorrect_) approach to implementing our own simplified `readable` function with signals:

```
// $lib/utils.svelte.js

// ❌ Don't do this! This is an example of how not to implement this.
export const readable = (initial_value, start) => {
let value = $state(initial_value)

let subscribers = 0
let stop = null

return {
// We return an object with a getter so we don't lose reactivity
get value() {
$effect(() => {
// ✨ Here is the trick ✨
// Every time the value is accessed, we create an effect in
// which we increase the subscriber count. An $effect() will
// run automatically when the scope is created (i.e.: the
// component is mounted)...

if (subscribers === 0) {
// If there are no subscribers yet, invoke the start function
// and store the returned function as the `stop` function.
stop = start()
}

// Increase our subscriber count.
subscribers++

// ...and will automatically be cleaned up when the scope is
// destroyed (i.e.: the component is unmounted). Here we can
// decrease the subscriber count.
return () => {
subscribers--

if (subscribers === 0) {
// If it was the last subscriber, invoke the stop function
stop?.()
stop = null
}
}
})

// Return our reactive value.
return value
}
}
}
```

This function is nearly identical to the built-in `svelte/store` `readable` implementation and would be used like this:

```
<script>
import { readable } from '$lib/utils.svelte.js'

const shoppingCartItems = readable(0, () => {
console.log('We have a subscriber!')
return () => {
console.log('We have no subscribers anymore')
}
})
</script>

Items in cart: {shoppingCartItems.value}
```

If you try this, you’ll see that it works! When this component mounts, the `start` function will be invoked, and the value will be reactive. So, why do we need `$effect.tracking`?

### The Problem

With Svelte 5’s new reactivity via signals, we can now use reactivity outside of components. Svelte proxies the underlying value and handles the logic to update our code when necessary, so we no longer need to subscribe to a store and can simply access the value directly.

However, here’s the issue: when accessing the underlying value outside of a tracking context (i.e., outside an `$effect`), how does the `readable` implementation know that accessing the property shouldn’t increase the subscriber count?

Here’s an example:

```
<script>
// Our readable implementation again
import { readable } from '$lib/utils.svelte.js'

const shoppingCartItems = readable(0, () => {
console.log('Start')
return () => console.log('Stop')
})

// Accessing the reactive value
console.log(shoppingCartItems.value)
</script>
```

The reactive value is read when the component is initialized and our naive `readable` implementation will increase the subscriber count to `1`. But the value is accessed _outside_ of a tracking context. Simply accessing a value in the initializer does _not_ track the value — the code will not rerun if the value changes. If we wanted to track it, we would need to put the access in an `$effect()`. This means that our readable implementation thinks that there is a subscriber although there is none.

Imagine that we use this `readable` in our app, and start polling for updated values as soon as there is a subscriber. With this implementation, our code will _always_ think there is a subscriber and start polling in the background, although there actually are none — defeating the purpose of tracking subscribers in the first place.

INFO

This is not a memory leak however. Because an `$effect` is always tied to the _scope_ it is created in, the effect we used inside our readable to track subscribers will still cleaned up when the scope (probably the component) is destroyed.

## Solution

To solve this issue, we can use the `$effect.tracking` rune. This rune tells us whether we’re currently inside a tracking context, which is exactly what we need. We only want to increase our subscriber count if the value has been accessed in a _tracking context_ (i.e.: an `$effect()`) because only then will the code accessing the value rerun when the value changes.

INFO

The template in a Svelte component also implicitly runs within an `$effect`. This is how Svelte observes reactive changes and updates the UI accordingly.

So, we just need to add this check in our `value` getter:

```
// $lib/utils.svelte.js
export const readable = (initial_value, start) => {
let value = $state(initial_value)

let subscribers = 0
let stop = null

return {
// We return an object with a getter so we don't lose reactivity
get value() {
if ($effect.tracking()) {
$effect(() => {
// Unchanged
})
}
return value
}
}
}
```

The full implementation can be found in the `$effect.tracking` docs.

## Additional Uses

As the name suggests, the primary purpose of the `$effect.tracking()` rune is to check whether your code is running in a tracking context. However, the rune also serves an additional purpose.

Effects are only permitted to run within a scope. If you attempt to run an effect outside a component, you will encounter the following error: `effect_orphan $effect can only be used inside an effect (e.g. during component initialisation)`.

In our `readable` implementation, we want code outside a scope to still be able to access the value. By wrapping our use of `$effect()` inside an `$effect.tracking()` check, we effectively "feed two birds with one scone":

1.  We ensure that the subscriber count doesn't increase if accessed outside of a tracking _context_, and
2.  We avoid encountering an error if the value is accessed outside of a tracking _scope_.

I hope this was helpful and answered any questions you might have had about the `$effect.tracking` rune. Thanks for reading.

Next post in this series

## Need a Break?

I built **Pausly** to help people like us step away from the screen for just a few minutes and move in ways that refresh both body and mind. Whether you’re coding, designing, or writing, a quick break can make all the difference.

Give it a try

Description: This article explores Svelte’s $effect.root rune, detailing its purpose, usage, and common misconceptions. It covers scenarios where $effect.root is essential, explains its lifecycle considerations, and offers alternatives.

Keywords: mat simon, mat meno, matias simon, matias meno, web development blog, frontend blog

Title: Svelte in Depth: The $effect.root rune — Mat Simon

This is the 2nd post in a series on Svelte, where we delve into the framework’s more advanced and intricate features, helping developers master the framework beyond the basics.

The `$effect` rune is straightforward: when an effect function runs, Svelte tracks the pieces of state it accesses and re-runs the function whenever that state changes. However, other effect-related runes are more complex. In the previous post, we discussed the `$effect.tracking` rune; this post will focus on the `$effect.root` rune.

I closely followed the development of Svelte 5, reading through GitHub issues and Discord discussions. The `$effect.root` rune has been a source of confusion for many, myself included, and the documentation is quite minimal:

> The `$effect.root` rune is an advanced feature that creates a non-tracked scope that doesn’t auto-cleanup. This is useful for nested effects that you want to manually control. This rune also allows for the creation of effects outside of the component initialisation phase.

This post aims to clarify when to use it. A common misconception is that this rune is more widely needed than it is, so before explaining _when_ to use it, let’s look at _when not to_.

## Common Misconception

A major driving force behind many design decisions in Svelte 5 has been to allow reactive code outside of components. In Svelte 4, you had to refactor your code to use stores to extract and reuse functionality. With Svelte 5, you can move logic from your `.svelte` file into a `.svelte.js` file and it will work the same. However, understanding how this reactive code operates outside of components can be less intuitive.

Let’s look at a simple example:

```
// $lib/settings.svelte.js
export class Settings {
notifications = $state(false)

constructor() {
$effect(() => console.log(this.notifications))
}
}
```

This is a basic class representing user settings. In the constructor, we use an `$effect` to log the value, which should run each time the notification setting changes.

INFO

In a real-world scenario, this code could manage syncing settings with local storage, for example.

One frequently asked question is whether `$effect.root` is needed if this class is in a separate file. Suppose you want to use this class in your Svelte component as follows:

```
<script>
import { Settings } from '$lib/settings.svelte.js'
const settings = new Settings()
</script>

<button
onclick={() => {
settings.notifications = !settings.notifications
}}
>
Toggle notifications
</button>
```

People often wonder if this setup will work as expected. Will clicking the button log the new values to the console, or is `$effect.root` necessary?

Every effect has to run inside a tracking scope.

To answer this, it’s essential to understand how effects are managed in Svelte. Each effect runs within a **tracking scope** that tracks which pieces of state are accessed within the effect, re-running it when the state changes, and cleaning it up when the scope is destroyed. When a component is created, a **tracking scope** is also created for the initialization code (code that runs synchronously in the script tag) and the template (the component’s HTML).

This means that it doesn’t matter if your `$effect` is defined in your Svelte component or a separate file; as long as the effect _runs_ in a tracking scope (e.g., during component initialization), it will work as expected.

## When it’s Needed

Since components are created within tracking scopes, your effects will work as expected without additional steps. So when is `$effect.root` necessary?

As you may have guessed, `$effect.root` is needed when you want to create a tracking scope outside a component lifecycle.

Let’s revisit our `Settings` example. Allowing multiple components to create their own instances of the `Settings` class doesn’t make sense, as it could lead to conflicting states, with each instance running its own separate effect. Instead, we want a global `settings` singleton that can be accessed from anywhere. The revised code would look like this:

```
// $lib/settings.svelte.js

// Not exporting the class anymore since it shouldn’t be instantiated
// outside of this file.
class Settings {
notifications = $state(false)
constructor() {
$effect(() => console.log(this.notifications))
}
}

export const settings = new Settings()
```

```
<script>
import { settings } from './settings.svelte.js'
</script>

<button
onclick={() => {
settings.notifications = !settings.notifications
}}
>
Toggle notifications
</button>
```

If you try to run this code, you’ll see this error: ``effect_orphan `$effect` can only be used inside an effect (e.g. during component initialisation)``. And that’s correct! **You’re trying to create an effect outside of a tracking scope.**

To resolve this, you can create a scope with `$effect.root` inside your class:

```
// $lib/settings.svelte.js
class Settings {
notifications = $state(false)

constructor() {
$effect.root(() => {
$effect(() => console.log(this.notifications))
})
}
}

export const settings = new Settings()
```

Now everything works as expected: The effect in the `Settings` constructor is now managed by its own scope.

WARNING

The `$effect.root` rune only creates a _scope_ to manage effects. If you don’t use `$effect` inside the scope, your code will _not_ re-run when the accessed state changes.

TIP

I used a class to illustrate how effects work when the effect is run in the constructor. But there is a simpler way to achieve the same result:

```
// $lib/settings.svelte.js
export const settings = $state({ notifications: false })

$effect.root(() => {
$effect(() => console.log(settings.notifications))
})
```

Because Svelte 5 has _deeply reactive state_, the `settings` object can be exported directly, without needing a class or proxy.

## The Downside of Using `$effect.root`

You might wonder why you shouldn’t always use `$effect.root`, since it enables effects anywhere in your code. The reason is that a tracking scope also requires cleanup (which will also cleanup all the effects within it). In the `Settings` example, this isn’t an issue since we’re creating a singleton that lives for the entire application’s lifetime. However, if your code could be instantiated multiple times, you need to ensure that the scope is cleaned up when it’s no longer needed.

In the case of a component, Svelte automatically cleans up the scope when the component is unmounted. When you create a scope with `$effect.root`, you receive a `cleanup` function that you need to call when the scope is no longer required:

```
const cleanup = $effect.root(() => {
// Any effects you might run here
})
```

If we wanted to provide an option to destroy our settings instance, we could handle it like this:

```
export class Settings {
notifications = $state(false)

#cleanup

constructor() {
this.#cleanup = $effect.root(() => {
$effect(() => console.log(this.notifications))
})
}

destroy() {
this.#cleanup()
}
}
```

When you’re finished with your `settings` object, you can call `settings.destroy()` to clean up the scope and all effects inside it.

### Avoiding Overuse

Try not to overuse `$effect.root`. I used it in this example to demonstrate how it works and when it’s applicable. However, for a global singleton, it’s usually better to create the settings object in your root layout and pass it down through context:

```
<!-- /routes/+layout.svelte -->
<script>
// Importing the initial Settings implementation
// without $effect.root here.
import { Settings } from '$lib/settings.svelte.js'
import { setContext } from 'svelte'

// Using the tracking scope of the root layout instead of
// creating our own with $effect.root
const settings = new Settings()

setContext('settings', settings)
</script>
```

This avoids all the complications surrounding `$effect.root` and has the added benefit that it makes your code easier to test.

It’s very rare to need the `$effect.root` rune.

In most cases, you’ll _never_ need this rune for web applications. If you think you do, consider structuring your code so it’s tied to a component’s (or layout’s) lifecycle.

If you _do_ need it, ensure either:

- the scope is created once and lives for the application’s lifetime, or
- you properly clean up the scope when it’s no longer needed.

TIP

If you’re unsure, start by simply using an effect in your code. If you don’t encounter an `effect_orphan` error, you probably don’t need `$effect.root`.

I hope this post has clarified the `$effect.root` rune. If there’s a Svelte topic you’d like me to cover, tag me on Bluesky!

Next post in this series

## Need a Break?

I built **Pausly** to help people like us step away from the screen for just a few minutes and move in ways that refresh both body and mind. Whether you’re coding, designing, or writing, a quick break can make all the difference.

Give it a try

Description: Learn how the createSubscriber() function in Svelte helps manage reactive values from external sources. Discover which problems it solves, when to use it, and how it works under the hood.

Keywords: mat simon, mat meno, matias simon, matias meno, web development blog, frontend blog

Title: Svelte in Depth: The createSubscriber() function — Mat Simon

This is the 3rd post in a series on Svelte, where we delve into the framework’s more advanced and intricate features, helping developers master the framework beyond the basics.

One of the more obscure features that Svelte pushed out in their Advent of Svelte is the `createSubscriber()` function. Because it is quite similar to what you can achieve with `$effect.tracking()`, I thought I’d write a short post about it.

As always, let’s start with what the docs say:

> Returns a `subscribe` function that, if called in an effect (including expressions in the template), calls its `start` callback with an `update` function. Whenever update is called, the effect re-runs.

The Svelte team then acknowledges that this is a bit hard to understand and provides an example with a `MediaQuery`.

## When would you use it?

The best way to understand how to use this function is to look at the example they provide and try solving it _without_ `createSubscriber()`. The example wants to create a reactive variable that reflects the current state of a media query. We can use the `window.matchMedia()` function to check if it matches and subscribe to changes:

```
const query = window.matchMedia('(width > 600px)')

// Read whether the media query currently matches
let queryMatches = query.matches

query.addEventListener('change', () => {
// Update the value when the media query changes
queryMatches = query.matches
})
```

A naive approach to turn this into a Svelte reactive `$state` _without_ `createSubscriber()` would be like this:

```
// ❌ Careful, this implementation has a bug!
class Layout {
#query = window.matchMedia('(width > 400px)')
// Initialize with the current state
current = $state(this.#getLayout())

constructor() {
// Subscribe to changes
this.#query.addEventListener('change', () => {
this.current = this.#getLayout()
})
}

#getLayout() {
return this.#query.matches ? 'desktop' : 'mobile'
}
}
```

By putting our value into a `$state` variable, we added reactivity. The issue with this approach is that we are creating an event listener that is never cleaned up! You could add a `destroy` method to the class that removes the event listener and make sure that it’s invoked when the component using this class is destroyed, but this is very clumsy and error-prone.

What we actually want is this: we only care about updating our value and creating a subscriber if the value is being read inside an effect! In the post about the `effect.tracking()` rune, we learned how to do this, so let’s apply what we learned here. Please revisit the post if you’re unfamiliar with the concept.

TIP

We’ll also use the `on` convenience function from Svelte to make it easier to add and remove event listeners.

Let’s update our code, so that it is able to track subscribers and will remove any event listeners when done:

This is getting pretty complex but don’t worry. `createSubscriber()` makes things a lot easier. If you just want to learn how to use it go to the next chapter.

```
import { on } from 'svelte/events'
import { tick } from 'svelte'

// ❌ This implementation is still not complete!
class Layout {
#query = window.matchMedia('(width > 400px)')

// Initialize with the current state.
// This time, we’re making the value private
// so we can wrap it in a getter.
#value = $state(this.#getLayout())

// Track the amount of subscribers.
#subscribers = 0

// Will be set to the function removing the event listener.
#removeEventListener

// We don’t setup listeners in the constructor anymore.
// constructor() { }

// Wrap the value in a getter, so we can setup the listeners
// when we have subscribers.
get current() {
// If in a tracking context ...
if ($effect.tracking()) {
$effect(() => {
// ...and there’s no subscribers yet...
if (this.#subscribers === 0) {
// ...update the value immediately...
this.#value = this.#getLayout()

// ...and setup an event listener.
this.#removeEventListener = on(this.#query, 'change', () => {
// Update the state value with the new media query value
// whenever the media query changes.
this.#value = this.#getLayout()
})
}

this.#subscribers++

return () => {
tick().then(() => {
this.#subscribers--
if (this.#subscribers === 0) {
// Cleanup if there are no more subscribers.
this.#removeEventListener?.()
this.#removeEventListener = undefined
}
})
}
})
}

return this.#value
}

#getLayout() {
return this.#query.matches ? 'desktop' : 'mobile'
}
}
```

This is already pretty complicated but we’re still not done yet!

But there is still an issue with this implementation that you might have noticed. Although we now create a subscriber to listen to changes to the query, we’re never updating the value when we’re outside an effect. This means that if you create an instance of this class and read its value, you cannot actually trust that the `.current` value reflects the actual state. We could just update `this.#value` every time the `.current` getter is invoked, but this seems a bit silly and inefficient. This is exactly why `createSubscriber()` exists.

## The `createSubscriber()` function

Instead of shoehorning a reactive `$state` value into our class to get reactivity, this function allows you to manage reactivity a lot more directly.

What we actually want is a direct way to tell any effect that is reading the value to re-run whenever the underlying data changes.

And this is where their example comes into play. I’ll copy it here and annotate accordingly:

```
import { createSubscriber } from 'svelte/reactivity'
import { on } from 'svelte/events'

export class MediaQuery {
#query
#subscribe

constructor(query) {
this.#query = window.matchMedia(`(${query})`)

// We’re creating a subscriber in the constructor. This returns a
// function that we can invoke whenever we read a value for which
// we want to manually manage reactivity.
this.#subscribe = createSubscriber((update) => {
// The body of this function is the "start" function. It will
// be invoked as soon as there is a subscriber.
// The update function that is provided can be invoked any
// time, and will trigger any effect in which the `#subscribe`
// has been invoked to re-run.

// We’re using the `on` convenience function from Svelte to add
// an event listener to our query. Whenever the `change` event
// is fired, we want the `update` function to be invoked.
// Note that we’re not updating any value here.
// (This is just syntactic sugar for
// `this.#query.addEventListener(’change’, update)`)
const off = on(this.#query, 'change', update)

// The returned function will be invoked when there are no
// more subscribers. In this case, we want to remove the event
// listener.
return () => off()
})
}

get current() {
// Any time the current value is read, we invoke the `#subscribe`
// function from the `createSubscriber`. This means that the
// effect accessing the `current` property will re-run (and thus,
// accessing this property again) any time the `update` function
// is invoked.
this.#subscribe()

// Instead of having to wrap the value in a $state property, we
// can now simply return the value directly.
return this.#query.matches
}
}
```

As you can see, this makes the intent of the code a lot clearer. After all, all we want is to return whether the query matches, and we want effects accessing the value to re-run whenever the underlying data changes.

## How does it work?

To be able to use `createSubscriber()` it’s not necessary to understand how it works, but in case you’re interested, here’s a quick explanation - it’s really simple!

`createSubscriber()` is nothing special. It’s just a helper function that Svelte provides. You could build the same function yourself. The crucial question is this:

How is the function able to trigger a re-run of effects?

The answer is actually really simple! The function creates a hidden `$state` value named `version`. When the `subscribe` function is invoked this value is being read, and so the effect re-runs when the `version` value changes. Any time you then invoke the `update` function, `version` is incremented, and all effects in which the `subscribe` function was invoked will re-run.

INFO

Svelte uses the technique of creating a hidden `version` state variable in many other places. The `SvelteMap` and `SvelteSet` classes for example also have a secret version state that they increase whenever changes to the underlying data are being made.

To be able to run the `start` function when the first effect subscribes, and cleanup when there are no subscribers left, the `createSubscriber()` function uses the same technique we used before with `$effect.tracking` to count the subscribers.

## Conclusion

Whenever you want to expose a value from an external source (for example a media query) that is not reactive but can change over time, and you want to expose it as a reactive value, `createSubscriber()` is a good choice. Even if media queries didn’t have a `change` event, you could setup an interval and periodically check the value and invoke the `update` function when appropriate.

## Need a Break?

I built **Pausly** to help people like us step away from the screen for just a few minutes and move in ways that refresh both body and mind. Whether you’re coding, designing, or writing, a quick break can make all the difference.

Give it a try
