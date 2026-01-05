![](out_runed/utilities/use-event-listener/index_media/412c1b99df431530f07813ed77ff123e20b0035f.svg) Toggle Sidebar

<img src="out_runed/utilities/use-event-listener/index_media/2ef71b57cc18afcd4ed82644358ede113ed6d592.svg" class="size-5" />Search Docs ... ⌘ K

<a href="https://github.com/svecosystem/runed" class="inline-flex select-none items-center justify-center gap-2 whitespace-nowrap text-sm font-semibold tracking-wide active:scale-[99%] disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:size-4 [&amp;_svg]:shrink-0 text-foreground hover:dark:bg-foreground/5 hover:dark:ring-foreground/5 hover:bg-foreground/2 hover:ring-foreground/2 bg-transparent ring ring-transparent h-10 w-10 rounded-md [&amp;&gt;svg]:size-5" aria-label="View this project on GitHub" rel="noopener noreferrer" target="_blank"><img src="out_runed/utilities/use-event-listener/index_media/1d22f722c87a133f2381965da1acf7a54aefc45a.svg" /></a>

<img src="out_runed/utilities/use-event-listener/index_media/b2f1f2409c2d7a3eaec75643beb67d5092b71fb2.svg" class="size-6" />

# useEventListener

A function that attaches an automatically disposed event listener.

## <a href="https://runed.dev/docs/utilities/use-event-listener#demo" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Demo

You've clicked the document 0 times

## <a href="https://runed.dev/docs/utilities/use-event-listener#usage" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Usage

The `useEventListener` function is particularly useful for attaching event listeners to elements you don't directly control. For instance, if you need to listen for events on the document body or window and can't use `<svelte:body />`, or if you receive an element reference from a parent component.

### <a href="https://runed.dev/docs/utilities/use-event-listener#example-tracking-clicks-on-the-document" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Example: Tracking Clicks on the Document

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-theme="github-light github-dark" tabindex="0"><code>
        // ClickLogger.ts
import { useEventListener } from &quot;runed&quot;;
 
export class ClickLogger {
    #clicks = $state(0);
 
    constructor() {
        useEventListener(
            () =&gt; document.body,
            &quot;click&quot;,
            () =&gt; this.#clicks++
        );
    }
 
    get clicks() {
        return this.#clicks;
    }
}
    </code></pre>
<img src="out_runed/utilities/use-event-listener/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

This `ClickLogger` class tracks the number of clicks on the document body using the `useEventListener` function. Each time a click occurs, the internal counter increments.

### <a href="https://runed.dev/docs/utilities/use-event-listener#svelte-component-usage" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Svelte Component Usage

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="svelte" data-theme="github-light github-dark" tabindex="0"><code>
        &lt;script lang=&quot;ts&quot;&gt;
    import { ClickLogger } from &quot;./ClickLogger.ts&quot;;
 
    const logger = new ClickLogger();
&lt;/script&gt;
 
&lt;p&gt;
    You&#39;ve clicked the document {logger.clicks}
    {logger.clicks === 1 ? &quot;time&quot; : &quot;times&quot;}
&lt;/p&gt;
    </code></pre>
<img src="out_runed/utilities/use-event-listener/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

In the component above, we create an instance of the `ClickLogger` class to monitor clicks on the document. The displayed text updates dynamically based on the recorded click count.

### <a href="https://runed.dev/docs/utilities/use-event-listener#key-points" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Key Points

- **Automatic Cleanup:** The event listener is removed automatically when the component is destroyed or when the element reference changes.
- **Lazy Initialization:** The target element can be defined using a function, enabling flexible and dynamic behavior.
- **Convenient for Global Listeners:** Ideal for scenarios where attaching event listeners directly to the DOM elements is cumbersome or impractical.

## Contributors

<a href="https://github.com/huntabyte" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/use-event-listener/index_media/75bcb1d595b8d37466d561f08b890ffc8f1d5425.jpg" id="bits-1256" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="Hunter Johnston" />

Hunter Johnston<a href="https://github.com/TGlide" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/use-event-listener/index_media/4bd89e8b63f277c9d71897d289dcd951b943b3c0.jpg" id="bits-1258" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="Thomas G. Lopes" />

Thomas G. Lopes<a href="https://github.com/abdel-17" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/use-event-listener/index_media/16484f37d7e5803413d1d654c51fedf5692548b8.png" id="bits-1260" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="Abdelrahman" />

Abdelrahman<a href="https://github.com/Hugos68" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/use-event-listener/index_media/083fdc509dbe74269766dede9274df2e89c38cd1.jpg" id="bits-1262" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="hugos68" />

hugos68

© 2025 <a href="https://github.com/svecosystem" class="underline-offset-2 hover:underline" target="_blank">Svecosystem</a>

<a href="https://github.com/svecosystem/runed" class="inline-flex select-none items-center justify-center gap-2 whitespace-nowrap text-sm font-semibold tracking-wide active:scale-[99%] disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:size-4 [&amp;_svg]:shrink-0 text-foreground hover:dark:bg-foreground/5 hover:dark:ring-foreground/5 hover:bg-foreground/2 hover:ring-foreground/2 bg-transparent ring ring-transparent h-10 w-10 rounded-md [&amp;&gt;svg]:size-5" aria-label="View this project on GitHub" rel="noopener noreferrer" target="_blank"><img src="out_runed/utilities/use-event-listener/index_media/1d22f722c87a133f2381965da1acf7a54aefc45a.svg" /></a>

<img src="out_runed/utilities/use-event-listener/index_media/b2f1f2409c2d7a3eaec75643beb67d5092b71fb2.svg" class="size-6" />

# useEventListener

A function that attaches an automatically disposed event listener.

## <a href="https://runed.dev/docs/utilities/use-event-listener#demo" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Demo

You've clicked the document 0 times

## <a href="https://runed.dev/docs/utilities/use-event-listener#usage" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Usage

The `useEventListener` function is particularly useful for attaching event listeners to elements you don't directly control. For instance, if you need to listen for events on the document body or window and can't use `<svelte:body />`, or if you receive an element reference from a parent component.

### <a href="https://runed.dev/docs/utilities/use-event-listener#example-tracking-clicks-on-the-document" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Example: Tracking Clicks on the Document

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-theme="github-light github-dark" tabindex="0"><code>
        // ClickLogger.ts
import { useEventListener } from &quot;runed&quot;;
 
export class ClickLogger {
    #clicks = $state(0);
 
    constructor() {
        useEventListener(
            () =&gt; document.body,
            &quot;click&quot;,
            () =&gt; this.#clicks++
        );
    }
 
    get clicks() {
        return this.#clicks;
    }
}
    </code></pre>
<img src="out_runed/utilities/use-event-listener/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

This `ClickLogger` class tracks the number of clicks on the document body using the `useEventListener` function. Each time a click occurs, the internal counter increments.

### <a href="https://runed.dev/docs/utilities/use-event-listener#svelte-component-usage" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Svelte Component Usage

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="svelte" data-theme="github-light github-dark" tabindex="0"><code>
        &lt;script lang=&quot;ts&quot;&gt;
    import { ClickLogger } from &quot;./ClickLogger.ts&quot;;
 
    const logger = new ClickLogger();
&lt;/script&gt;
 
&lt;p&gt;
    You&#39;ve clicked the document {logger.clicks}
    {logger.clicks === 1 ? &quot;time&quot; : &quot;times&quot;}
&lt;/p&gt;
    </code></pre>
<img src="out_runed/utilities/use-event-listener/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

In the component above, we create an instance of the `ClickLogger` class to monitor clicks on the document. The displayed text updates dynamically based on the recorded click count.

### <a href="https://runed.dev/docs/utilities/use-event-listener#key-points" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Key Points

- **Automatic Cleanup:** The event listener is removed automatically when the component is destroyed or when the element reference changes.
- **Lazy Initialization:** The target element can be defined using a function, enabling flexible and dynamic behavior.
- **Convenient for Global Listeners:** Ideal for scenarios where attaching event listeners directly to the DOM elements is cumbersome or impractical.

## Contributors

<a href="https://github.com/huntabyte" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/use-event-listener/index_media/75bcb1d595b8d37466d561f08b890ffc8f1d5425.jpg" id="bits-1256" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="Hunter Johnston" />

Hunter Johnston<a href="https://github.com/TGlide" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/use-event-listener/index_media/4bd89e8b63f277c9d71897d289dcd951b943b3c0.jpg" id="bits-1258" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="Thomas G. Lopes" />

Thomas G. Lopes<a href="https://github.com/abdel-17" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/use-event-listener/index_media/16484f37d7e5803413d1d654c51fedf5692548b8.png" id="bits-1260" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="Abdelrahman" />

Abdelrahman<a href="https://github.com/Hugos68" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/use-event-listener/index_media/083fdc509dbe74269766dede9274df2e89c38cd1.jpg" id="bits-1262" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="hugos68" />

hugos68
