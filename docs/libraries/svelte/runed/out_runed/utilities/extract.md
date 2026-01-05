![](out_runed/utilities/extract/index_media/412c1b99df431530f07813ed77ff123e20b0035f.svg) Toggle Sidebar

<img src="out_runed/utilities/extract/index_media/2ef71b57cc18afcd4ed82644358ede113ed6d592.svg" class="size-5" />Search Docs ... ⌘ K

<a href="https://github.com/svecosystem/runed" class="inline-flex select-none items-center justify-center gap-2 whitespace-nowrap text-sm font-semibold tracking-wide active:scale-[99%] disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:size-4 [&amp;_svg]:shrink-0 text-foreground hover:dark:bg-foreground/5 hover:dark:ring-foreground/5 hover:bg-foreground/2 hover:ring-foreground/2 bg-transparent ring ring-transparent h-10 w-10 rounded-md [&amp;&gt;svg]:size-5" aria-label="View this project on GitHub" rel="noopener noreferrer" target="_blank"><img src="out_runed/utilities/extract/index_media/1d22f722c87a133f2381965da1acf7a54aefc45a.svg" /></a>

<img src="out_runed/utilities/extract/index_media/b2f1f2409c2d7a3eaec75643beb67d5092b71fb2.svg" class="size-6" />

# extract

Resolve the value of a getter or static variable

In libraries like Runed, it's common to pass state reactively using getters (functions that return a value), a common pattern to pass reactivity across boundaries.

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-theme="github-light github-dark" tabindex="0"><code>
        // For example...
import { Previous } from &quot;runed&quot;;
 
let count = $state(0);
const previous = new Previous(() =&gt; count);
    </code></pre>
<img src="out_runed/utilities/extract/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

However, some APIs accept either a reactive getter or a static value (including `undefined`):

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-theme="github-light github-dark" tabindex="0"><code>
        let search = $state(&quot;&quot;);
let debounceTime = $state&lt;number | undefined&gt;(500);
 
// with a reactive value
const d1 = new Debounced(
    () =&gt; search,
    () =&gt; debounceTime
);
 
// with a static value
const d2 = new Debounced(() =&gt; search, 500);
 
// no defined value
const d3 = new Debounced(() =&gt; search);
    </code></pre>
<img src="out_runed/utilities/extract/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

When writing utility functions, dealing with both types can lead to verbose and repetitive logic:

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-theme="github-light github-dark" tabindex="0"><code>
        setTimeout(
    /* ... */,
    typeof wait === &quot;function&quot; ? (wait() ?? 250) : (wait ?? 250)
);
    </code></pre>
<img src="out_runed/utilities/extract/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

This is where `extract` comes in.

## <a href="https://runed.dev/docs/utilities/extract#usage" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Usage

The `extract` utility resolves either a getter or static value to a plain value. This helps you write cleaner, safer utilities.

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-theme="github-light github-dark" tabindex="0"><code>
        import { extract } from &quot;runed&quot;;
 
/**
 * Triggers confetti at a given interval.
 * @param intervalProp Time between confetti bursts, in ms. Defaults to 100.
 */
function throwConfetti(intervalProp?: MaybeGetter&lt;number | undefined&gt;) {
    const interval = $derived(extract(intervalProp, 100));
    // ...
}
    </code></pre>
<img src="out_runed/utilities/extract/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

## <a href="https://runed.dev/docs/utilities/extract#behavior" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Behavior

Given a `MaybeGetter<T>`, `extract(input, fallback)` resolves as follows:

| Case                                        | Result                      |
|---------------------------------------------|-----------------------------|
| `input` is a value                          | Returns the value           |
| `input` is `undefined`                      | Returns the fallback        |
| `input` is a function returning a value     | Returns the function result |
| `input` is a function returning `undefined` | Returns the fallback        |

The fallback is *optional*. If you omit it, `extract()` returns `T | undefined`.

## <a href="https://runed.dev/docs/utilities/extract#types" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Types

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-theme="github-light github-dark" tabindex="0"><code>
        function extract&lt;T&gt;(input: MaybeGetter&lt;T | undefined&gt;, fallback: T): T;
function extract&lt;T&gt;(input: MaybeGetter&lt;T | undefined&gt;): T | undefined;
    </code></pre>
<img src="out_runed/utilities/extract/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

## Contributors

<a href="https://github.com/TGlide" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/extract/index_media/4bd89e8b63f277c9d71897d289dcd951b943b3c0.jpg" id="bits-106" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="Thomas G. Lopes" />

Thomas G. Lopes<a href="https://github.com/43081j" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/extract/index_media/06f4edda8a9beea51070ce54837509dcdbea04f6.png" id="bits-108" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="James Garbutt" />

James Garbutt

© 2025 <a href="https://github.com/svecosystem" class="underline-offset-2 hover:underline" target="_blank">Svecosystem</a>

<a href="https://github.com/svecosystem/runed" class="inline-flex select-none items-center justify-center gap-2 whitespace-nowrap text-sm font-semibold tracking-wide active:scale-[99%] disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:size-4 [&amp;_svg]:shrink-0 text-foreground hover:dark:bg-foreground/5 hover:dark:ring-foreground/5 hover:bg-foreground/2 hover:ring-foreground/2 bg-transparent ring ring-transparent h-10 w-10 rounded-md [&amp;&gt;svg]:size-5" aria-label="View this project on GitHub" rel="noopener noreferrer" target="_blank"><img src="out_runed/utilities/extract/index_media/1d22f722c87a133f2381965da1acf7a54aefc45a.svg" /></a>

<img src="out_runed/utilities/extract/index_media/b2f1f2409c2d7a3eaec75643beb67d5092b71fb2.svg" class="size-6" />

# extract

Resolve the value of a getter or static variable

In libraries like Runed, it's common to pass state reactively using getters (functions that return a value), a common pattern to pass reactivity across boundaries.

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-theme="github-light github-dark" tabindex="0"><code>
        // For example...
import { Previous } from &quot;runed&quot;;
 
let count = $state(0);
const previous = new Previous(() =&gt; count);
    </code></pre>
<img src="out_runed/utilities/extract/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

However, some APIs accept either a reactive getter or a static value (including `undefined`):

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-theme="github-light github-dark" tabindex="0"><code>
        let search = $state(&quot;&quot;);
let debounceTime = $state&lt;number | undefined&gt;(500);
 
// with a reactive value
const d1 = new Debounced(
    () =&gt; search,
    () =&gt; debounceTime
);
 
// with a static value
const d2 = new Debounced(() =&gt; search, 500);
 
// no defined value
const d3 = new Debounced(() =&gt; search);
    </code></pre>
<img src="out_runed/utilities/extract/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

When writing utility functions, dealing with both types can lead to verbose and repetitive logic:

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-theme="github-light github-dark" tabindex="0"><code>
        setTimeout(
    /* ... */,
    typeof wait === &quot;function&quot; ? (wait() ?? 250) : (wait ?? 250)
);
    </code></pre>
<img src="out_runed/utilities/extract/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

This is where `extract` comes in.

## <a href="https://runed.dev/docs/utilities/extract#usage" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Usage

The `extract` utility resolves either a getter or static value to a plain value. This helps you write cleaner, safer utilities.

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-theme="github-light github-dark" tabindex="0"><code>
        import { extract } from &quot;runed&quot;;
 
/**
 * Triggers confetti at a given interval.
 * @param intervalProp Time between confetti bursts, in ms. Defaults to 100.
 */
function throwConfetti(intervalProp?: MaybeGetter&lt;number | undefined&gt;) {
    const interval = $derived(extract(intervalProp, 100));
    // ...
}
    </code></pre>
<img src="out_runed/utilities/extract/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

## <a href="https://runed.dev/docs/utilities/extract#behavior" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Behavior

Given a `MaybeGetter<T>`, `extract(input, fallback)` resolves as follows:

| Case                                        | Result                      |
|---------------------------------------------|-----------------------------|
| `input` is a value                          | Returns the value           |
| `input` is `undefined`                      | Returns the fallback        |
| `input` is a function returning a value     | Returns the function result |
| `input` is a function returning `undefined` | Returns the fallback        |

The fallback is *optional*. If you omit it, `extract()` returns `T | undefined`.

## <a href="https://runed.dev/docs/utilities/extract#types" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Types

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-theme="github-light github-dark" tabindex="0"><code>
        function extract&lt;T&gt;(input: MaybeGetter&lt;T | undefined&gt;, fallback: T): T;
function extract&lt;T&gt;(input: MaybeGetter&lt;T | undefined&gt;): T | undefined;
    </code></pre>
<img src="out_runed/utilities/extract/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

## Contributors

<a href="https://github.com/TGlide" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/extract/index_media/4bd89e8b63f277c9d71897d289dcd951b943b3c0.jpg" id="bits-106" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="Thomas G. Lopes" />

Thomas G. Lopes<a href="https://github.com/43081j" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/extract/index_media/06f4edda8a9beea51070ce54837509dcdbea04f6.png" id="bits-108" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="James Garbutt" />

James Garbutt
