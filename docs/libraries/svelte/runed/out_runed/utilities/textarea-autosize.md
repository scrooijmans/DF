![](out_runed/utilities/textarea-autosize/index_media/412c1b99df431530f07813ed77ff123e20b0035f.svg) Toggle Sidebar

<img src="out_runed/utilities/textarea-autosize/index_media/2ef71b57cc18afcd4ed82644358ede113ed6d592.svg" class="size-5" />Search Docs ... ⌘ K

<a href="https://github.com/svecosystem/runed" class="inline-flex select-none items-center justify-center gap-2 whitespace-nowrap text-sm font-semibold tracking-wide active:scale-[99%] disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:size-4 [&amp;_svg]:shrink-0 text-foreground hover:dark:bg-foreground/5 hover:dark:ring-foreground/5 hover:bg-foreground/2 hover:ring-foreground/2 bg-transparent ring ring-transparent h-10 w-10 rounded-md [&amp;&gt;svg]:size-5" aria-label="View this project on GitHub" rel="noopener noreferrer" target="_blank"><img src="out_runed/utilities/textarea-autosize/index_media/1d22f722c87a133f2381965da1acf7a54aefc45a.svg" /></a>

<img src="out_runed/utilities/textarea-autosize/index_media/b2f1f2409c2d7a3eaec75643beb67d5092b71fb2.svg" class="size-6" />

# TextareaAutosize

Automatically adjust a textarea's height based on its content.

## <a href="https://runed.dev/docs/utilities/textarea-autosize#demo" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Demo

Behold, the growing textarea!

## <a href="https://runed.dev/docs/utilities/textarea-autosize#overview" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Overview

`TextareaAutosize` is a utility that makes `<textarea>` elements grow or shrink automatically based on their content, without causing layout shifts. It:

- Mirrors the actual textarea off-screen for accurate measurement
- Syncs dimensions when the content, width, or element changes
- Prevents overflow until a maximum height (if configured)
- Supports both reactive state and static values

## <a href="https://runed.dev/docs/utilities/textarea-autosize#usage" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Usage

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="svelte" data-theme="github-light github-dark" tabindex="0"><code>
        &lt;script lang=&quot;ts&quot;&gt;
    import { TextareaAutosize } from &quot;runed&quot;;
 
    let el = $state&lt;HTMLTextAreaElement&gt;(null!);
    let value = $state(&quot;&quot;);
 
    new TextareaAutosize({
        element: () =&gt; el,
        input: () =&gt; value
    });
&lt;/script&gt;
 
&lt;textarea bind:this={el} bind:value&gt;&lt;/textarea&gt;
    </code></pre>
<img src="out_runed/utilities/textarea-autosize/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

As you type, the textarea will automatically resize vertically to fit the content.

## <a href="https://runed.dev/docs/utilities/textarea-autosize#options" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Options

You can customize behavior via the following options:

| Option | Type | Description |
|----|----|----|
| `element` | `Getter<HTMLElement | undefined>` | The target textarea (required). |
| `input` | `Getter<string>` | Reactive input value (required). |
| `onResize` | `() => void` | Called whenever the height is updated. |
| `styleProp` | `"height"` \| `"minHeight"` | CSS property to control size. `"height"` resizes both ways. `"minHeight"` grows only. Default: `"height"`. |
| `maxHeight` | `number` | Maximum height in pixels before scroll appears. Default: unlimited. |

## <a href="https://runed.dev/docs/utilities/textarea-autosize#behavior" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Behavior

Internally, `TextareaAutosize`:

- Creates an invisible, off-screen `<textarea>` clone
- Copies computed styles from the actual textarea
- Measures scroll height of the clone to determine needed height
- Applies the height (or `minHeight`) to the real textarea
- Recalculates on content changes, element resizes, and width changes

## <a href="https://runed.dev/docs/utilities/textarea-autosize#examples" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Examples

### <a href="https://runed.dev/docs/utilities/textarea-autosize#grow-only-behavior" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Grow-only Behavior

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-theme="github-light github-dark" tabindex="0"><code>
        new TextareaAutosize({
    element: () =&gt; el,
    input: () =&gt; value,
    styleProp: &quot;minHeight&quot;
});
    </code></pre>
<img src="out_runed/utilities/textarea-autosize/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

This lets the textarea expand as needed, but won't shrink smaller than its current size.

## Contributors

<a href="https://github.com/TGlide" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/textarea-autosize/index_media/4bd89e8b63f277c9d71897d289dcd951b943b3c0.jpg" id="bits-1075" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="Thomas G. Lopes" />

Thomas G. Lopes<a href="https://github.com/wd-David" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/textarea-autosize/index_media/90cd387ff9a5aa1590e09e531ec7d97a0908ca8b.jpg" id="bits-1077" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="David Peng" />

David Peng

© 2025 <a href="https://github.com/svecosystem" class="underline-offset-2 hover:underline" target="_blank">Svecosystem</a>

<a href="https://github.com/svecosystem/runed" class="inline-flex select-none items-center justify-center gap-2 whitespace-nowrap text-sm font-semibold tracking-wide active:scale-[99%] disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:size-4 [&amp;_svg]:shrink-0 text-foreground hover:dark:bg-foreground/5 hover:dark:ring-foreground/5 hover:bg-foreground/2 hover:ring-foreground/2 bg-transparent ring ring-transparent h-10 w-10 rounded-md [&amp;&gt;svg]:size-5" aria-label="View this project on GitHub" rel="noopener noreferrer" target="_blank"><img src="out_runed/utilities/textarea-autosize/index_media/1d22f722c87a133f2381965da1acf7a54aefc45a.svg" /></a>

<img src="out_runed/utilities/textarea-autosize/index_media/b2f1f2409c2d7a3eaec75643beb67d5092b71fb2.svg" class="size-6" />

# TextareaAutosize

Automatically adjust a textarea's height based on its content.

## <a href="https://runed.dev/docs/utilities/textarea-autosize#demo" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Demo

Behold, the growing textarea!

## <a href="https://runed.dev/docs/utilities/textarea-autosize#overview" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Overview

`TextareaAutosize` is a utility that makes `<textarea>` elements grow or shrink automatically based on their content, without causing layout shifts. It:

- Mirrors the actual textarea off-screen for accurate measurement
- Syncs dimensions when the content, width, or element changes
- Prevents overflow until a maximum height (if configured)
- Supports both reactive state and static values

## <a href="https://runed.dev/docs/utilities/textarea-autosize#usage" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Usage

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="svelte" data-theme="github-light github-dark" tabindex="0"><code>
        &lt;script lang=&quot;ts&quot;&gt;
    import { TextareaAutosize } from &quot;runed&quot;;
 
    let el = $state&lt;HTMLTextAreaElement&gt;(null!);
    let value = $state(&quot;&quot;);
 
    new TextareaAutosize({
        element: () =&gt; el,
        input: () =&gt; value
    });
&lt;/script&gt;
 
&lt;textarea bind:this={el} bind:value&gt;&lt;/textarea&gt;
    </code></pre>
<img src="out_runed/utilities/textarea-autosize/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

As you type, the textarea will automatically resize vertically to fit the content.

## <a href="https://runed.dev/docs/utilities/textarea-autosize#options" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Options

You can customize behavior via the following options:

| Option | Type | Description |
|----|----|----|
| `element` | `Getter<HTMLElement | undefined>` | The target textarea (required). |
| `input` | `Getter<string>` | Reactive input value (required). |
| `onResize` | `() => void` | Called whenever the height is updated. |
| `styleProp` | `"height"` \| `"minHeight"` | CSS property to control size. `"height"` resizes both ways. `"minHeight"` grows only. Default: `"height"`. |
| `maxHeight` | `number` | Maximum height in pixels before scroll appears. Default: unlimited. |

## <a href="https://runed.dev/docs/utilities/textarea-autosize#behavior" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Behavior

Internally, `TextareaAutosize`:

- Creates an invisible, off-screen `<textarea>` clone
- Copies computed styles from the actual textarea
- Measures scroll height of the clone to determine needed height
- Applies the height (or `minHeight`) to the real textarea
- Recalculates on content changes, element resizes, and width changes

## <a href="https://runed.dev/docs/utilities/textarea-autosize#examples" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Examples

### <a href="https://runed.dev/docs/utilities/textarea-autosize#grow-only-behavior" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Grow-only Behavior

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-theme="github-light github-dark" tabindex="0"><code>
        new TextareaAutosize({
    element: () =&gt; el,
    input: () =&gt; value,
    styleProp: &quot;minHeight&quot;
});
    </code></pre>
<img src="out_runed/utilities/textarea-autosize/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

This lets the textarea expand as needed, but won't shrink smaller than its current size.

## Contributors

<a href="https://github.com/TGlide" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/textarea-autosize/index_media/4bd89e8b63f277c9d71897d289dcd951b943b3c0.jpg" id="bits-1075" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="Thomas G. Lopes" />

Thomas G. Lopes<a href="https://github.com/wd-David" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/textarea-autosize/index_media/90cd387ff9a5aa1590e09e531ec7d97a0908ca8b.jpg" id="bits-1077" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="David Peng" />

David Peng
