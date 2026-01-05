![](out_runed/utilities/on-click-outside/index_media/412c1b99df431530f07813ed77ff123e20b0035f.svg) Toggle Sidebar

<img src="out_runed/utilities/on-click-outside/index_media/2ef71b57cc18afcd4ed82644358ede113ed6d592.svg" class="size-5" />Search Docs ... ⌘ K

<a href="https://github.com/svecosystem/runed" class="inline-flex select-none items-center justify-center gap-2 whitespace-nowrap text-sm font-semibold tracking-wide active:scale-[99%] disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:size-4 [&amp;_svg]:shrink-0 text-foreground hover:dark:bg-foreground/5 hover:dark:ring-foreground/5 hover:bg-foreground/2 hover:ring-foreground/2 bg-transparent ring ring-transparent h-10 w-10 rounded-md [&amp;&gt;svg]:size-5" aria-label="View this project on GitHub" rel="noopener noreferrer" target="_blank"><img src="out_runed/utilities/on-click-outside/index_media/1d22f722c87a133f2381965da1acf7a54aefc45a.svg" /></a>

<img src="out_runed/utilities/on-click-outside/index_media/b2f1f2409c2d7a3eaec75643beb67d5092b71fb2.svg" class="size-6" />

# onClickOutside

Handle clicks outside of a specified element.

`onClickOutside` detects clicks that occur outside a specified element's boundaries and executes a callback function. It's commonly used for dismissible dropdowns, modals, and other interactive components.

## <a href="https://runed.dev/docs/utilities/on-click-outside#demo" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Demo

container

Has not clicked outside yet.

Status: Enabled

Start

Stop

Reset

## <a href="https://runed.dev/docs/utilities/on-click-outside#basic-usage" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Basic Usage

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="svelte" data-theme="github-light github-dark" tabindex="0"><code>
        &lt;script lang=&quot;ts&quot;&gt;
    import { onClickOutside } from &quot;runed&quot;;
 
    let container = $state&lt;HTMLElement&gt;()!;
 
    onClickOutside(
        () =&gt; container,
        () =&gt; console.log(&quot;clicked outside&quot;)
    );
&lt;/script&gt;
 
&lt;div bind:this={container}&gt;
    &lt;!-- Container content --&gt;
&lt;/div&gt;
&lt;button&gt;I&#39;m outside the container&lt;/button&gt;
    </code></pre>
<img src="out_runed/utilities/on-click-outside/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

## <a href="https://runed.dev/docs/utilities/on-click-outside#advanced-usage" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Advanced Usage

### <a href="https://runed.dev/docs/utilities/on-click-outside#controlled-listener" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Controlled Listener

The function returns control methods to programmatically manage the listener, `start` and `stop` and a reactive read-only property `enabled` to check the current status of the listeners.

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="svelte" data-theme="github-light github-dark" tabindex="0"><code>
        &lt;script lang=&quot;ts&quot;&gt;
    import { onClickOutside } from &quot;runed&quot;;
 
    let dialog = $state&lt;HTMLDialogElement&gt;()!;
 
    const clickOutside = onClickOutside(
        () =&gt; dialog,
        () =&gt; {
            dialog.close();
            clickOutside.stop();
        },
        { immediate: false }
    );
 
    function openDialog() {
        dialog.showModal();
        clickOutside.start();
    }
 
    function closeDialog() {
        dialog.close();
        clickOutside.stop();
    }
&lt;/script&gt;
 
&lt;button onclick={openDialog}&gt;Open Dialog&lt;/button&gt;
&lt;dialog bind:this={dialog}&gt;
    &lt;div&gt;
        &lt;button onclick={closeDialog}&gt;Close Dialog&lt;/button&gt;
    &lt;/div&gt;
&lt;/dialog&gt;
    </code></pre>
<img src="out_runed/utilities/on-click-outside/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

Here's an example of using `onClickOutside` with a `<dialog>`:

Open Dialog

This is a dialog.

Lorem, ipsum dolor sit amet consectetur adipisicing elit. Neque sunt aut sit exercitationem deleniti doloremque quo quasi, expedita omnis dicta eaque, eveniet nesciunt nobis sint atque? Praesentium facilis officiis perferendis.

Close Dialog

## <a href="https://runed.dev/docs/utilities/on-click-outside#options" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Options

immediate

type: boolean

Whether the click outside handler is enabled by default or not. If set to `false`, the handler will not be active until enabled by calling the returned `start` function.

default: true

detectIframe

type: boolean

Controls whether focus events from iframes trigger the callback. Since iframe click events don't bubble to the parent document, you may want to enable this if you need to detect when users interact with iframe content.

default: false

document

type: Document

The document object to use, defaults to the global document.

default: document

window

type: Window

The window object to use, defaults to the global window.

default: window

## <a href="https://runed.dev/docs/utilities/on-click-outside#type-definitions" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Type Definitions

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-theme="github-light github-dark" tabindex="0"><code>
        export type OnClickOutsideOptions = ConfigurableWindow &amp;
    ConfigurableDocument &amp; {
        /**
         * Whether the click outside handler is enabled by default or not.
         * If set to false, the handler will not be active until enabled by
         * calling the returned `start` function
         *
         * @default true
         */
        immediate?: boolean;
        /**
         * Controls whether focus events from iframes trigger the callback.
         *
         * Since iframe click events don&#39;t bubble to the parent document,
         * you may want to enable this if you need to detect when users
         * interact with iframe content.
         *
         * @default false
         */
        detectIframe?: boolean;
    };
/**
 * A utility that calls a given callback when a click event occurs outside of
 * a specified container element.
 *
 * @template T - The type of the container element, defaults to HTMLElement.
 * @param {MaybeElementGetter&lt;T&gt;} container - The container element or a getter function that returns the container element.
 * @param {() =&gt; void} callback - The callback function to call when a click event occurs outside of the container.
 * @param {OnClickOutsideOptions} [opts={}] - Optional configuration object.
 * @param {ConfigurableDocument} [opts.document=defaultDocument] - The document object to use, defaults to the global document.
 * @param {boolean} [opts.immediate=true] - Whether the click outside handler is enabled by default or not.
 * @param {boolean} [opts.detectIframe=false] - Controls whether focus events from iframes trigger the callback.
 *
 * @see {@link https://runed.dev/docs/utilities/on-click-outside}
 */
export declare function onClickOutside&lt;T extends Element = HTMLElement&gt;(
    container: MaybeElementGetter&lt;T&gt;,
    callback: (event: PointerEvent | FocusEvent) =&gt; void,
    opts?: OnClickOutsideOptions
): {
    /** Stop listening for click events outside the container. */
    stop: () =&gt; boolean;
    /** Start listening for click events outside the container. */
    start: () =&gt; boolean;
    /** Whether the click outside handler is currently enabled or not. */
    readonly enabled: boolean;
};
    </code></pre>
<img src="out_runed/utilities/on-click-outside/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

## Contributors

<a href="https://github.com/huntabyte" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/on-click-outside/index_media/75bcb1d595b8d37466d561f08b890ffc8f1d5425.jpg" id="bits-1403" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="Hunter Johnston" />

Hunter Johnston<a href="https://github.com/RickRyan26" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/on-click-outside/index_media/e9c2d80e60408bbfe96f9a09b63c61b79d358fb0.jpg" id="bits-1405" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="Rick Ryan" />

Rick Ryan

© 2025 <a href="https://github.com/svecosystem" class="underline-offset-2 hover:underline" target="_blank">Svecosystem</a>

<a href="https://github.com/svecosystem/runed" class="inline-flex select-none items-center justify-center gap-2 whitespace-nowrap text-sm font-semibold tracking-wide active:scale-[99%] disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:size-4 [&amp;_svg]:shrink-0 text-foreground hover:dark:bg-foreground/5 hover:dark:ring-foreground/5 hover:bg-foreground/2 hover:ring-foreground/2 bg-transparent ring ring-transparent h-10 w-10 rounded-md [&amp;&gt;svg]:size-5" aria-label="View this project on GitHub" rel="noopener noreferrer" target="_blank"><img src="out_runed/utilities/on-click-outside/index_media/1d22f722c87a133f2381965da1acf7a54aefc45a.svg" /></a>

<img src="out_runed/utilities/on-click-outside/index_media/b2f1f2409c2d7a3eaec75643beb67d5092b71fb2.svg" class="size-6" />

# onClickOutside

Handle clicks outside of a specified element.

`onClickOutside` detects clicks that occur outside a specified element's boundaries and executes a callback function. It's commonly used for dismissible dropdowns, modals, and other interactive components.

## <a href="https://runed.dev/docs/utilities/on-click-outside#demo" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Demo

container

Has not clicked outside yet.

Status: Enabled

Start

Stop

Reset

## <a href="https://runed.dev/docs/utilities/on-click-outside#basic-usage" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Basic Usage

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="svelte" data-theme="github-light github-dark" tabindex="0"><code>
        &lt;script lang=&quot;ts&quot;&gt;
    import { onClickOutside } from &quot;runed&quot;;
 
    let container = $state&lt;HTMLElement&gt;()!;
 
    onClickOutside(
        () =&gt; container,
        () =&gt; console.log(&quot;clicked outside&quot;)
    );
&lt;/script&gt;
 
&lt;div bind:this={container}&gt;
    &lt;!-- Container content --&gt;
&lt;/div&gt;
&lt;button&gt;I&#39;m outside the container&lt;/button&gt;
    </code></pre>
<img src="out_runed/utilities/on-click-outside/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

## <a href="https://runed.dev/docs/utilities/on-click-outside#advanced-usage" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Advanced Usage

### <a href="https://runed.dev/docs/utilities/on-click-outside#controlled-listener" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Controlled Listener

The function returns control methods to programmatically manage the listener, `start` and `stop` and a reactive read-only property `enabled` to check the current status of the listeners.

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="svelte" data-theme="github-light github-dark" tabindex="0"><code>
        &lt;script lang=&quot;ts&quot;&gt;
    import { onClickOutside } from &quot;runed&quot;;
 
    let dialog = $state&lt;HTMLDialogElement&gt;()!;
 
    const clickOutside = onClickOutside(
        () =&gt; dialog,
        () =&gt; {
            dialog.close();
            clickOutside.stop();
        },
        { immediate: false }
    );
 
    function openDialog() {
        dialog.showModal();
        clickOutside.start();
    }
 
    function closeDialog() {
        dialog.close();
        clickOutside.stop();
    }
&lt;/script&gt;
 
&lt;button onclick={openDialog}&gt;Open Dialog&lt;/button&gt;
&lt;dialog bind:this={dialog}&gt;
    &lt;div&gt;
        &lt;button onclick={closeDialog}&gt;Close Dialog&lt;/button&gt;
    &lt;/div&gt;
&lt;/dialog&gt;
    </code></pre>
<img src="out_runed/utilities/on-click-outside/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

Here's an example of using `onClickOutside` with a `<dialog>`:

Open Dialog

This is a dialog.

Lorem, ipsum dolor sit amet consectetur adipisicing elit. Neque sunt aut sit exercitationem deleniti doloremque quo quasi, expedita omnis dicta eaque, eveniet nesciunt nobis sint atque? Praesentium facilis officiis perferendis.

Close Dialog

## <a href="https://runed.dev/docs/utilities/on-click-outside#options" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Options

immediate

type: boolean

Whether the click outside handler is enabled by default or not. If set to `false`, the handler will not be active until enabled by calling the returned `start` function.

default: true

detectIframe

type: boolean

Controls whether focus events from iframes trigger the callback. Since iframe click events don't bubble to the parent document, you may want to enable this if you need to detect when users interact with iframe content.

default: false

document

type: Document

The document object to use, defaults to the global document.

default: document

window

type: Window

The window object to use, defaults to the global window.

default: window

## <a href="https://runed.dev/docs/utilities/on-click-outside#type-definitions" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Type Definitions

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-theme="github-light github-dark" tabindex="0"><code>
        export type OnClickOutsideOptions = ConfigurableWindow &amp;
    ConfigurableDocument &amp; {
        /**
         * Whether the click outside handler is enabled by default or not.
         * If set to false, the handler will not be active until enabled by
         * calling the returned `start` function
         *
         * @default true
         */
        immediate?: boolean;
        /**
         * Controls whether focus events from iframes trigger the callback.
         *
         * Since iframe click events don&#39;t bubble to the parent document,
         * you may want to enable this if you need to detect when users
         * interact with iframe content.
         *
         * @default false
         */
        detectIframe?: boolean;
    };
/**
 * A utility that calls a given callback when a click event occurs outside of
 * a specified container element.
 *
 * @template T - The type of the container element, defaults to HTMLElement.
 * @param {MaybeElementGetter&lt;T&gt;} container - The container element or a getter function that returns the container element.
 * @param {() =&gt; void} callback - The callback function to call when a click event occurs outside of the container.
 * @param {OnClickOutsideOptions} [opts={}] - Optional configuration object.
 * @param {ConfigurableDocument} [opts.document=defaultDocument] - The document object to use, defaults to the global document.
 * @param {boolean} [opts.immediate=true] - Whether the click outside handler is enabled by default or not.
 * @param {boolean} [opts.detectIframe=false] - Controls whether focus events from iframes trigger the callback.
 *
 * @see {@link https://runed.dev/docs/utilities/on-click-outside}
 */
export declare function onClickOutside&lt;T extends Element = HTMLElement&gt;(
    container: MaybeElementGetter&lt;T&gt;,
    callback: (event: PointerEvent | FocusEvent) =&gt; void,
    opts?: OnClickOutsideOptions
): {
    /** Stop listening for click events outside the container. */
    stop: () =&gt; boolean;
    /** Start listening for click events outside the container. */
    start: () =&gt; boolean;
    /** Whether the click outside handler is currently enabled or not. */
    readonly enabled: boolean;
};
    </code></pre>
<img src="out_runed/utilities/on-click-outside/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

## Contributors

<a href="https://github.com/huntabyte" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/on-click-outside/index_media/75bcb1d595b8d37466d561f08b890ffc8f1d5425.jpg" id="bits-1403" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="Hunter Johnston" />

Hunter Johnston<a href="https://github.com/RickRyan26" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/on-click-outside/index_media/e9c2d80e60408bbfe96f9a09b63c61b79d358fb0.jpg" id="bits-1405" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="Rick Ryan" />

Rick Ryan
