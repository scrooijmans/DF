![](out_runed/utilities/context/index_media/412c1b99df431530f07813ed77ff123e20b0035f.svg) Toggle Sidebar

<img src="out_runed/utilities/context/index_media/2ef71b57cc18afcd4ed82644358ede113ed6d592.svg" class="size-5" />Search Docs ... ⌘ K

<a href="https://github.com/svecosystem/runed" class="inline-flex select-none items-center justify-center gap-2 whitespace-nowrap text-sm font-semibold tracking-wide active:scale-[99%] disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:size-4 [&amp;_svg]:shrink-0 text-foreground hover:dark:bg-foreground/5 hover:dark:ring-foreground/5 hover:bg-foreground/2 hover:ring-foreground/2 bg-transparent ring ring-transparent h-10 w-10 rounded-md [&amp;&gt;svg]:size-5" aria-label="View this project on GitHub" rel="noopener noreferrer" target="_blank"><img src="out_runed/utilities/context/index_media/1d22f722c87a133f2381965da1acf7a54aefc45a.svg" /></a>

<img src="out_runed/utilities/context/index_media/b2f1f2409c2d7a3eaec75643beb67d5092b71fb2.svg" class="size-6" />

# Context

A wrapper around Svelte's Context API that provides type safety and improved ergonomics for sharing data between components.

Context allows you to pass data through the component tree without explicitly passing props through every level. It's useful for sharing data that many components need, like themes, authentication state, or localization preferences.

The `Context` class provides a type-safe way to define, set, and retrieve context values.

## <a href="https://runed.dev/docs/utilities/context#usage" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Usage

### Creating a Context

First, create a `Context` instance with the type of value it will hold:

<figure data-metadata="" data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-metadata="" data-theme="github-light github-dark" tabindex="0"><code>
        import { Context } from &quot;runed&quot;;
 
export const myTheme = new Context&lt;&quot;light&quot; | &quot;dark&quot;&gt;(&quot;theme&quot;);
    </code></pre>
<img src="out_runed/utilities/context/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
<figcaption>context.ts</figcaption>
</figure>

Creating a `Context` instance only defines the context - it doesn't actually set any value. The value passed to the constructor (`"theme"` in this example) is just an identifier used for debugging and error messages.

Think of this step as creating a "container" that will later hold your context value. The container is typed (in this case to only accept `"light"` or `"dark"` as values) but remains empty until you explicitly call `myTheme.set()` during component initialization.

This separation between defining and setting context allows you to:

- Keep context definitions in separate files
- Reuse the same context definition across different parts of your app
- Maintain type safety throughout your application
- Set different values for the same context in different component trees

### Setting Context Values

Set the context value in a parent component during initialization.

<figure data-metadata="" data-rehype-pretty-code-figure="">
<pre class="ring" data-language="svelte" data-metadata="" data-theme="github-light github-dark" tabindex="0"><code>
        &lt;script lang=&quot;ts&quot;&gt;
    import { myTheme } from &quot;./context&quot;;
    let { data, children } = $props();
 
    myTheme.set(data.theme);
&lt;/script&gt;
 
{@render children?.()}
    </code></pre>
<img src="out_runed/utilities/context/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
<figcaption>+layout.svelte</figcaption>
</figure>

<img src="out_runed/utilities/context/index_media/a08f0efc16d25c94f055dec10bc7c24422bffe7b.svg" class="size-6" />

##### Note

Context must be set during component initialization, similar to lifecycle functions like `onMount`. You cannot set context inside event handlers or callbacks.

### Reading Context Values

Child components can access the context using `get()` or `getOr()`

<figure data-metadata="" data-rehype-pretty-code-figure="">
<pre class="ring" data-language="svelte" data-metadata="" data-theme="github-light github-dark" tabindex="0"><code>
        &lt;script lang=&quot;ts&quot;&gt;
    import { myTheme } from &quot;./context&quot;;
 
    const theme = myTheme.get();
    // or with a fallback value if the context is not set
    const theme = myTheme.getOr(&quot;light&quot;);
&lt;/script&gt;
    </code></pre>
<img src="out_runed/utilities/context/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
<figcaption>+page.svelte</figcaption>
</figure>

## <a href="https://runed.dev/docs/utilities/context#type-definition" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Type Definition

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-theme="github-light github-dark" tabindex="0"><code>
        class Context&lt;TContext&gt; {
    /**
     * @param name The name of the context.
     * This is used for generating the context key and error messages.
     */
    constructor(name: string) {}
 
    /**
     * The key used to get and set the context.
     *
     * It is not recommended to use this value directly.
     * Instead, use the methods provided by this class.
     */
    get key(): symbol;
 
    /**
     * Checks whether this has been set in the context of a parent component.
     *
     * Must be called during component initialization.
     */
    exists(): boolean;
 
    /**
     * Retrieves the context that belongs to the closest parent component.
     *
     * Must be called during component initialization.
     *
     * @throws An error if the context does not exist.
     */
    get(): TContext;
 
    /**
     * Retrieves the context that belongs to the closest parent component,
     * or the given fallback value if the context does not exist.
     *
     * Must be called during component initialization.
     */
    getOr&lt;TFallback&gt;(fallback: TFallback): TContext | TFallback;
 
    /**
     * Associates the given value with the current component and returns it.
     *
     * Must be called during component initialization.
     */
    set(context: TContext): TContext;
}
    </code></pre>
<img src="out_runed/utilities/context/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

## Contributors

<a href="https://github.com/abdel-17" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/context/index_media/16484f37d7e5803413d1d654c51fedf5692548b8.png" id="bits-426" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="Abdelrahman" />

Abdelrahman<a href="https://github.com/huntabyte" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/context/index_media/75bcb1d595b8d37466d561f08b890ffc8f1d5425.jpg" id="bits-428" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="Hunter Johnston" />

Hunter Johnston

© 2025 <a href="https://github.com/svecosystem" class="underline-offset-2 hover:underline" target="_blank">Svecosystem</a>

<a href="https://github.com/svecosystem/runed" class="inline-flex select-none items-center justify-center gap-2 whitespace-nowrap text-sm font-semibold tracking-wide active:scale-[99%] disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:size-4 [&amp;_svg]:shrink-0 text-foreground hover:dark:bg-foreground/5 hover:dark:ring-foreground/5 hover:bg-foreground/2 hover:ring-foreground/2 bg-transparent ring ring-transparent h-10 w-10 rounded-md [&amp;&gt;svg]:size-5" aria-label="View this project on GitHub" rel="noopener noreferrer" target="_blank"><img src="out_runed/utilities/context/index_media/1d22f722c87a133f2381965da1acf7a54aefc45a.svg" /></a>

<img src="out_runed/utilities/context/index_media/b2f1f2409c2d7a3eaec75643beb67d5092b71fb2.svg" class="size-6" />

# Context

A wrapper around Svelte's Context API that provides type safety and improved ergonomics for sharing data between components.

Context allows you to pass data through the component tree without explicitly passing props through every level. It's useful for sharing data that many components need, like themes, authentication state, or localization preferences.

The `Context` class provides a type-safe way to define, set, and retrieve context values.

## <a href="https://runed.dev/docs/utilities/context#usage" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Usage

### Creating a Context

First, create a `Context` instance with the type of value it will hold:

<figure data-metadata="" data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-metadata="" data-theme="github-light github-dark" tabindex="0"><code>
        import { Context } from &quot;runed&quot;;
 
export const myTheme = new Context&lt;&quot;light&quot; | &quot;dark&quot;&gt;(&quot;theme&quot;);
    </code></pre>
<img src="out_runed/utilities/context/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
<figcaption>context.ts</figcaption>
</figure>

Creating a `Context` instance only defines the context - it doesn't actually set any value. The value passed to the constructor (`"theme"` in this example) is just an identifier used for debugging and error messages.

Think of this step as creating a "container" that will later hold your context value. The container is typed (in this case to only accept `"light"` or `"dark"` as values) but remains empty until you explicitly call `myTheme.set()` during component initialization.

This separation between defining and setting context allows you to:

- Keep context definitions in separate files
- Reuse the same context definition across different parts of your app
- Maintain type safety throughout your application
- Set different values for the same context in different component trees

### Setting Context Values

Set the context value in a parent component during initialization.

<figure data-metadata="" data-rehype-pretty-code-figure="">
<pre class="ring" data-language="svelte" data-metadata="" data-theme="github-light github-dark" tabindex="0"><code>
        &lt;script lang=&quot;ts&quot;&gt;
    import { myTheme } from &quot;./context&quot;;
    let { data, children } = $props();
 
    myTheme.set(data.theme);
&lt;/script&gt;
 
{@render children?.()}
    </code></pre>
<img src="out_runed/utilities/context/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
<figcaption>+layout.svelte</figcaption>
</figure>

<img src="out_runed/utilities/context/index_media/a08f0efc16d25c94f055dec10bc7c24422bffe7b.svg" class="size-6" />

##### Note

Context must be set during component initialization, similar to lifecycle functions like `onMount`. You cannot set context inside event handlers or callbacks.

### Reading Context Values

Child components can access the context using `get()` or `getOr()`

<figure data-metadata="" data-rehype-pretty-code-figure="">
<pre class="ring" data-language="svelte" data-metadata="" data-theme="github-light github-dark" tabindex="0"><code>
        &lt;script lang=&quot;ts&quot;&gt;
    import { myTheme } from &quot;./context&quot;;
 
    const theme = myTheme.get();
    // or with a fallback value if the context is not set
    const theme = myTheme.getOr(&quot;light&quot;);
&lt;/script&gt;
    </code></pre>
<img src="out_runed/utilities/context/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
<figcaption>+page.svelte</figcaption>
</figure>

## <a href="https://runed.dev/docs/utilities/context#type-definition" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Type Definition

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-theme="github-light github-dark" tabindex="0"><code>
        class Context&lt;TContext&gt; {
    /**
     * @param name The name of the context.
     * This is used for generating the context key and error messages.
     */
    constructor(name: string) {}
 
    /**
     * The key used to get and set the context.
     *
     * It is not recommended to use this value directly.
     * Instead, use the methods provided by this class.
     */
    get key(): symbol;
 
    /**
     * Checks whether this has been set in the context of a parent component.
     *
     * Must be called during component initialization.
     */
    exists(): boolean;
 
    /**
     * Retrieves the context that belongs to the closest parent component.
     *
     * Must be called during component initialization.
     *
     * @throws An error if the context does not exist.
     */
    get(): TContext;
 
    /**
     * Retrieves the context that belongs to the closest parent component,
     * or the given fallback value if the context does not exist.
     *
     * Must be called during component initialization.
     */
    getOr&lt;TFallback&gt;(fallback: TFallback): TContext | TFallback;
 
    /**
     * Associates the given value with the current component and returns it.
     *
     * Must be called during component initialization.
     */
    set(context: TContext): TContext;
}
    </code></pre>
<img src="out_runed/utilities/context/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

## Contributors

<a href="https://github.com/abdel-17" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/context/index_media/16484f37d7e5803413d1d654c51fedf5692548b8.png" id="bits-426" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="Abdelrahman" />

Abdelrahman<a href="https://github.com/huntabyte" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/context/index_media/75bcb1d595b8d37466d561f08b890ffc8f1d5425.jpg" id="bits-428" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="Hunter Johnston" />

Hunter Johnston
