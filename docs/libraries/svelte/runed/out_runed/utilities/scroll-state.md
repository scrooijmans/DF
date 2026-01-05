![](out_runed/utilities/scroll-state/index_media/412c1b99df431530f07813ed77ff123e20b0035f.svg) Toggle Sidebar

<img src="out_runed/utilities/scroll-state/index_media/2ef71b57cc18afcd4ed82644358ede113ed6d592.svg" class="size-5" />Search Docs ... ⌘ K

<a href="https://github.com/svecosystem/runed" class="inline-flex select-none items-center justify-center gap-2 whitespace-nowrap text-sm font-semibold tracking-wide active:scale-[99%] disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:size-4 [&amp;_svg]:shrink-0 text-foreground hover:dark:bg-foreground/5 hover:dark:ring-foreground/5 hover:bg-foreground/2 hover:ring-foreground/2 bg-transparent ring ring-transparent h-10 w-10 rounded-md [&amp;&gt;svg]:size-5" aria-label="View this project on GitHub" rel="noopener noreferrer" target="_blank"><img src="out_runed/utilities/scroll-state/index_media/1d22f722c87a133f2381965da1acf7a54aefc45a.svg" /></a>

<img src="out_runed/utilities/scroll-state/index_media/b2f1f2409c2d7a3eaec75643beb67d5092b71fb2.svg" class="size-6" />

# ScrollState

Track scroll position, direction, and edge states with support for programmatic scrolling.

## <a href="https://runed.dev/docs/utilities/scroll-state#demo" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Demo

Scroll me

## Controls & State

X Position

Set

Y Position

Set

Smooth Scroll

isScrolling No

------------------------------------------------------------------------

### Progress

x 0%

y 0%

------------------------------------------------------------------------

### Arrived

top Yes

right No

bottom No

left Yes

------------------------------------------------------------------------

### Directions

top No

right No

bottom No

left No

## <a href="https://runed.dev/docs/utilities/scroll-state#overview" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Overview

`ScrollState` is a reactive utility that lets you:

- Track scroll positions (`x` / `y`)
- Detect scroll direction (`left`, `right`, `top`, `bottom`)
- Determine if the user has scrolled to an edge (`arrived` state)
- Perform programmatic scrolling (`scrollTo`, `scrollToTop`, `scrollToBottom`)
- Listen to scroll and scroll-end events
- Respect flex, RTL, and reverse layout modes

Inspired by <a href="https://vueuse.org/useScroll" class="text-brand-link hover:text-brand-link-hover leading-7" rel="noopener noreferrer" target="_blank">VueUse's <code>useScroll</code></a>, this utility is built for Svelte and works with DOM elements, the `window`, or `document`.

## <a href="https://runed.dev/docs/utilities/scroll-state#usage" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Usage

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="svelte" data-theme="github-light github-dark" tabindex="0"><code>
        &lt;script lang=&quot;ts&quot;&gt;
    import { ScrollState } from &quot;runed&quot;;
 
    let el = $state&lt;HTMLElement&gt;();
 
    const scroll = new ScrollState({
        element: () =&gt; el
    });
&lt;/script&gt;
 
&lt;div bind:this={el} style=&quot;overflow: auto; height: 200px;&quot;&gt;
    &lt;!-- scrollable content here --&gt;
&lt;/div&gt;
    </code></pre>
<img src="out_runed/utilities/scroll-state/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

You can now access:

- `scroll.x` and `scroll.y` — current scroll positions (reactive, get/set)

- `scroll.directions` — active scroll directions

- `scroll.arrived` — whether the scroll has reached each edge

- `scroll.progress` — percentage that the user has scrolled on the x/y axis

- `scroll.scrollTo(x, y)` — programmatic scroll

- `scroll.scrollToTop()` and `scroll.scrollToBottom()` — helpers

## <a href="https://runed.dev/docs/utilities/scroll-state#options" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Options

You can configure `ScrollState` via the following options:

| Option | Type | Description |
|----|----|----|
| `element` | `MaybeGetter<HTMLElement | Window | Document | null>` | The scroll container (required). |
| `idle` | `MaybeGetter<number | undefined>` | Debounce time (ms) after scroll ends. Default: `200`. |
| `offset` | `{ top?, bottom?, left?, right? }` | Pixel thresholds for "arrived" state detection. Default: `0` for all. |
| `onScroll` | `(e: Event) => void` | Callback for scroll events. |
| `onStop` | `(e: Event) => void` | Callback after scrolling stops. |
| `eventListenerOptions` | `AddEventListenerOptions` | Scroll listener options. Default: `{ passive: true, capture: false }`. |
| `behavior` | `ScrollBehavior` | Scroll behavior: `"auto"`, `"smooth"`, etc. Default: `"auto"`. |
| `onError` | `(error: unknown) => void` | Optional error handler. Default: `console.error`. |

## <a href="https://runed.dev/docs/utilities/scroll-state#notes" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Notes

- Both scroll position (`x`, `y`) and edge arrival state (`arrived`) are reactive values.

- You can programmatically change `scroll.x` and `scroll.y`, and the element will scroll accordingly.

- Layout direction and reverse flex settings are respected when calculating edge states.

- Debounced `onStop` is invoked after scrolling ends and the user is idle.

## Contributors

<a href="https://github.com/ieedan" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/scroll-state/index_media/2324acafb6b6eb6603364d7ea4f1927b0694b63f.jpg" id="bits-1030" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="Aidan Bleser" />

Aidan Bleser<a href="https://github.com/TGlide" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/scroll-state/index_media/4bd89e8b63f277c9d71897d289dcd951b943b3c0.jpg" id="bits-1032" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="Thomas G. Lopes" />

Thomas G. Lopes<a href="https://github.com/wd-David" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/scroll-state/index_media/90cd387ff9a5aa1590e09e531ec7d97a0908ca8b.jpg" id="bits-1034" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="David Peng" />

David Peng<a href="https://github.com/huntabyte" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/scroll-state/index_media/75bcb1d595b8d37466d561f08b890ffc8f1d5425.jpg" id="bits-1036" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="Hunter Johnston" />

Hunter Johnston

© 2025 <a href="https://github.com/svecosystem" class="underline-offset-2 hover:underline" target="_blank">Svecosystem</a>

<a href="https://github.com/svecosystem/runed" class="inline-flex select-none items-center justify-center gap-2 whitespace-nowrap text-sm font-semibold tracking-wide active:scale-[99%] disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:size-4 [&amp;_svg]:shrink-0 text-foreground hover:dark:bg-foreground/5 hover:dark:ring-foreground/5 hover:bg-foreground/2 hover:ring-foreground/2 bg-transparent ring ring-transparent h-10 w-10 rounded-md [&amp;&gt;svg]:size-5" aria-label="View this project on GitHub" rel="noopener noreferrer" target="_blank"><img src="out_runed/utilities/scroll-state/index_media/1d22f722c87a133f2381965da1acf7a54aefc45a.svg" /></a>

<img src="out_runed/utilities/scroll-state/index_media/b2f1f2409c2d7a3eaec75643beb67d5092b71fb2.svg" class="size-6" />

# ScrollState

Track scroll position, direction, and edge states with support for programmatic scrolling.

## <a href="https://runed.dev/docs/utilities/scroll-state#demo" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Demo

Scroll me

## Controls & State

X Position

Set

Y Position

Set

Smooth Scroll

isScrolling No

------------------------------------------------------------------------

### Progress

x 0%

y 0%

------------------------------------------------------------------------

### Arrived

top Yes

right No

bottom No

left Yes

------------------------------------------------------------------------

### Directions

top No

right No

bottom No

left No

## <a href="https://runed.dev/docs/utilities/scroll-state#overview" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Overview

`ScrollState` is a reactive utility that lets you:

- Track scroll positions (`x` / `y`)
- Detect scroll direction (`left`, `right`, `top`, `bottom`)
- Determine if the user has scrolled to an edge (`arrived` state)
- Perform programmatic scrolling (`scrollTo`, `scrollToTop`, `scrollToBottom`)
- Listen to scroll and scroll-end events
- Respect flex, RTL, and reverse layout modes

Inspired by <a href="https://vueuse.org/useScroll" class="text-brand-link hover:text-brand-link-hover leading-7" rel="noopener noreferrer" target="_blank">VueUse's <code>useScroll</code></a>, this utility is built for Svelte and works with DOM elements, the `window`, or `document`.

## <a href="https://runed.dev/docs/utilities/scroll-state#usage" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Usage

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="svelte" data-theme="github-light github-dark" tabindex="0"><code>
        &lt;script lang=&quot;ts&quot;&gt;
    import { ScrollState } from &quot;runed&quot;;
 
    let el = $state&lt;HTMLElement&gt;();
 
    const scroll = new ScrollState({
        element: () =&gt; el
    });
&lt;/script&gt;
 
&lt;div bind:this={el} style=&quot;overflow: auto; height: 200px;&quot;&gt;
    &lt;!-- scrollable content here --&gt;
&lt;/div&gt;
    </code></pre>
<img src="out_runed/utilities/scroll-state/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

You can now access:

- `scroll.x` and `scroll.y` — current scroll positions (reactive, get/set)

- `scroll.directions` — active scroll directions

- `scroll.arrived` — whether the scroll has reached each edge

- `scroll.progress` — percentage that the user has scrolled on the x/y axis

- `scroll.scrollTo(x, y)` — programmatic scroll

- `scroll.scrollToTop()` and `scroll.scrollToBottom()` — helpers

## <a href="https://runed.dev/docs/utilities/scroll-state#options" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Options

You can configure `ScrollState` via the following options:

| Option | Type | Description |
|----|----|----|
| `element` | `MaybeGetter<HTMLElement | Window | Document | null>` | The scroll container (required). |
| `idle` | `MaybeGetter<number | undefined>` | Debounce time (ms) after scroll ends. Default: `200`. |
| `offset` | `{ top?, bottom?, left?, right? }` | Pixel thresholds for "arrived" state detection. Default: `0` for all. |
| `onScroll` | `(e: Event) => void` | Callback for scroll events. |
| `onStop` | `(e: Event) => void` | Callback after scrolling stops. |
| `eventListenerOptions` | `AddEventListenerOptions` | Scroll listener options. Default: `{ passive: true, capture: false }`. |
| `behavior` | `ScrollBehavior` | Scroll behavior: `"auto"`, `"smooth"`, etc. Default: `"auto"`. |
| `onError` | `(error: unknown) => void` | Optional error handler. Default: `console.error`. |

## <a href="https://runed.dev/docs/utilities/scroll-state#notes" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Notes

- Both scroll position (`x`, `y`) and edge arrival state (`arrived`) are reactive values.

- You can programmatically change `scroll.x` and `scroll.y`, and the element will scroll accordingly.

- Layout direction and reverse flex settings are respected when calculating edge states.

- Debounced `onStop` is invoked after scrolling ends and the user is idle.

## Contributors

<a href="https://github.com/ieedan" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/scroll-state/index_media/2324acafb6b6eb6603364d7ea4f1927b0694b63f.jpg" id="bits-1030" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="Aidan Bleser" />

Aidan Bleser<a href="https://github.com/TGlide" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/scroll-state/index_media/4bd89e8b63f277c9d71897d289dcd951b943b3c0.jpg" id="bits-1032" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="Thomas G. Lopes" />

Thomas G. Lopes<a href="https://github.com/wd-David" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/scroll-state/index_media/90cd387ff9a5aa1590e09e531ec7d97a0908ca8b.jpg" id="bits-1034" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="David Peng" />

David Peng<a href="https://github.com/huntabyte" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/scroll-state/index_media/75bcb1d595b8d37466d561f08b890ffc8f1d5425.jpg" id="bits-1036" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="Hunter Johnston" />

Hunter Johnston
