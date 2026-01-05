# API Reference

This reference attempts to document every function, hook, component, and type exported by Svelte Flow. If you are looking for guides, please refer to our <a href="https://svelteflow.dev/learn" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]">learn section</a>.

## How to use this reference<a href="https://svelteflow.dev/api-reference#how-to-use-this-reference" class="x:focus-visible:nextra-focus subheading-anchor" aria-label="Permalink for this section"></a>

We think that documentation should answer two broad questions: “what is this thing?” and “how do I use it?”

To that end, our API reference aims to **concisely** answer that first question, while guides go into more detail on the second. If you find yourself clicking around the reference wondering what the heck any of this means, maybe we have a guide that can help you out!

<a href="https://svelteflow.dev/learn/customization/custom-nodes" class="lg:odd:border-r border-b border-gray-100 hover:bg-gray-100/50 border-solid px-8 py-10 lg:py-16 z-10 cursor-pointer"></a>

Custom nodes

A powerful feature of Svelte Flow is the ability to add custom nodes. Within your custom nodes you can render everything you want. You can define multiple source and target handles and render form inputs or charts for example. In this guide we will implement a node with an input field that updates some text in another part of the application.

Read more <img src="out_svelteflow/api-reference/index_media/ed07040f42acd621a31819d5787c9b8ab8fb9628.svg" class="inline w-4 h-4" />

<a href="https://svelteflow.dev/learn/layouting/sub-flows" class="lg:odd:border-r border-b border-gray-100 hover:bg-gray-100/50 border-solid px-8 py-10 lg:py-16 z-10 cursor-pointer"></a>

Subflows

Is this a flow within a flow? Yes! Sometimes you want to treat parts of a flow as a group or even as a single node. This is where subflows come into play. In this guide you will learn how to create a child-parent relationship between nodes and implement different types of subflows.

Read more <img src="out_svelteflow/api-reference/index_media/ed07040f42acd621a31819d5787c9b8ab8fb9628.svg" class="inline w-4 h-4" />

## A note for JavaScript users<a href="https://svelteflow.dev/api-reference#a-note-for-javascript-users" class="x:focus-visible:nextra-focus subheading-anchor" aria-label="Permalink for this section"></a>

Svelte Flow is written in TypeScript, but we know that not everyone uses it. We encourage developers to use the technology that works best for them, and throughout our documentation there is a blend of TypeScript and JavaScript examples.

For our API reference, however, we use TypeScript’s syntax to document the types of props and functions. Here’s a quick crash course on how to read it:

• `?` means that the field or argument is optional.

• `<T>` in a type definition represents a generic type parameter. Like a function argument but for types! The definition `type Array<T> = ...` means a type called `Array` that takes a generic type parameter `T`.

• `<T>` when referring to a type is like “filling in” a generic type parameter. It’s like calling a function but for types! The type `Array<number>` is the type `Array` with the generic type parameter `T` filled in with the type `number`.

• `T | U` means that the type is either `T` or `U`: this is often called a *union*.

• `T & U` means that the type is both `T` and `U`: this is often called an *intersection*.

The TypeScript folks have their own <a href="https://www.typescriptlang.org/docs/handbook/typescript-in-5-minutes.html" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]" rel="noreferrer" target="_blank">handy guide for reading types <img src="out_svelteflow/api-reference/index_media/926b7c4e31213c3a8240847d8251d75a921b6a21.svg" class="x:inline x:align-baseline x:shrink-0" /></a> that you might find useful. If you’re still stuck on something, feel free to drop by our <a href="https://discord.com/invite/RVmnytFmGW" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]" rel="noreferrer" target="_blank">Discord <img src="out_svelteflow/api-reference/index_media/926b7c4e31213c3a8240847d8251d75a921b6a21.svg" class="x:inline x:align-baseline x:shrink-0" /></a> and ask for help!
