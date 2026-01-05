![](out_runed/utilities/use-intersection-observer/index_media/412c1b99df431530f07813ed77ff123e20b0035f.svg) Toggle Sidebar

<img src="out_runed/utilities/use-intersection-observer/index_media/2ef71b57cc18afcd4ed82644358ede113ed6d592.svg" class="size-5" />Search Docs ... ⌘ K

<a href="https://github.com/svecosystem/runed" class="inline-flex select-none items-center justify-center gap-2 whitespace-nowrap text-sm font-semibold tracking-wide active:scale-[99%] disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:size-4 [&amp;_svg]:shrink-0 text-foreground hover:dark:bg-foreground/5 hover:dark:ring-foreground/5 hover:bg-foreground/2 hover:ring-foreground/2 bg-transparent ring ring-transparent h-10 w-10 rounded-md [&amp;&gt;svg]:size-5" aria-label="View this project on GitHub" rel="noopener noreferrer" target="_blank"><img src="out_runed/utilities/use-intersection-observer/index_media/1d22f722c87a133f2381965da1acf7a54aefc45a.svg" /></a>

<img src="out_runed/utilities/use-intersection-observer/index_media/b2f1f2409c2d7a3eaec75643beb67d5092b71fb2.svg" class="size-6" />

# useIntersectionObserver

Watch for intersection changes of a target element.

## <a href="https://runed.dev/docs/utilities/use-intersection-observer#demo" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Demo

<img src="out_runed/utilities/use-intersection-observer/index_media/0e661a3a21048f0c2e21e6ac705be61661890b52.svg" class="size-3.5" />

Enable

Scroll down 👇

I'm the target! 🎯

Element outside the viewport

## <a href="https://runed.dev/docs/utilities/use-intersection-observer#usage" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Usage

With a reference to an element, you can use the `useIntersectionObserver` utility to watch for intersection changes of the target element.

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="svelte" data-theme="github-light github-dark" tabindex="0"><code>
        &lt;script lang=&quot;ts&quot;&gt;
    import { useIntersectionObserver } from &quot;runed&quot;;
 
    let target = $state&lt;HTMLElement | null&gt;(null);
    let root = $state&lt;HTMLElement | null&gt;(null);
 
    let isIntersecting = $state(false);
 
    useIntersectionObserver(
        () =&gt; target,
        (entries) =&gt; {
            const entry = entries[0];
            if (!entry) return;
            isIntersecting = entry.isIntersecting;
        },
        { root: () =&gt; root }
    );
&lt;/script&gt;
 
&lt;div bind:this={root}&gt;
    &lt;div bind:this={target}&gt;
        {#if isIntersecting}
            &lt;div&gt;Target is intersecting&lt;/div&gt;
        {:else}
            &lt;div&gt;Target is not intersecting&lt;/div&gt;
        {/if}
    &lt;/div&gt;
&lt;/div&gt;
    </code></pre>
<img src="out_runed/utilities/use-intersection-observer/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

### <a href="https://runed.dev/docs/utilities/use-intersection-observer#pause" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Pause

You can pause the intersection observer at any point by calling the `pause` method.

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-theme="github-light github-dark" tabindex="0"><code>
        const observer = useIntersectionObserver(/* ... */);
 
observer.pause();
    </code></pre>
<img src="out_runed/utilities/use-intersection-observer/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

### <a href="https://runed.dev/docs/utilities/use-intersection-observer#resume" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Resume

You can resume the intersection observer at any point by calling the `resume` method.

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-theme="github-light github-dark" tabindex="0"><code>
        const observer = useIntersectionObserver(/* ... */);
 
observer.resume();
    </code></pre>
<img src="out_runed/utilities/use-intersection-observer/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

### <a href="https://runed.dev/docs/utilities/use-intersection-observer#stop" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Stop

You can stop the intersection observer at any point by calling the `stop` method.

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-theme="github-light github-dark" tabindex="0"><code>
        const observer = useIntersectionObserver(/* ... */);
 
observer.stop();
    </code></pre>
<img src="out_runed/utilities/use-intersection-observer/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

### <a href="https://runed.dev/docs/utilities/use-intersection-observer#isactive" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>isActive

You can check if the intersection observer is active by checking the `isActive` property.

<img src="out_runed/utilities/use-intersection-observer/index_media/9f0a101e998d794b6318889157bb9f7ed19c4727.svg" class="size-6" />

##### Warning

This property cannot be destructured as it is a getter. You must access it directly from the observer.

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-theme="github-light github-dark" tabindex="0"><code>
        const observer = useIntersectionObserver(/* ... */);
 
if (observer.isActive) {
    // do something
}
    </code></pre>
<img src="out_runed/utilities/use-intersection-observer/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

## Contributors

<a href="https://github.com/huntabyte" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/use-intersection-observer/index_media/75bcb1d595b8d37466d561f08b890ffc8f1d5425.jpg" id="bits-1135" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="Hunter Johnston" />

Hunter Johnston

© 2025 <a href="https://github.com/svecosystem" class="underline-offset-2 hover:underline" target="_blank">Svecosystem</a>

<a href="https://github.com/svecosystem/runed" class="inline-flex select-none items-center justify-center gap-2 whitespace-nowrap text-sm font-semibold tracking-wide active:scale-[99%] disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:size-4 [&amp;_svg]:shrink-0 text-foreground hover:dark:bg-foreground/5 hover:dark:ring-foreground/5 hover:bg-foreground/2 hover:ring-foreground/2 bg-transparent ring ring-transparent h-10 w-10 rounded-md [&amp;&gt;svg]:size-5" aria-label="View this project on GitHub" rel="noopener noreferrer" target="_blank"><img src="out_runed/utilities/use-intersection-observer/index_media/1d22f722c87a133f2381965da1acf7a54aefc45a.svg" /></a>

<img src="out_runed/utilities/use-intersection-observer/index_media/b2f1f2409c2d7a3eaec75643beb67d5092b71fb2.svg" class="size-6" />

# useIntersectionObserver

Watch for intersection changes of a target element.

## <a href="https://runed.dev/docs/utilities/use-intersection-observer#demo" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Demo

<img src="out_runed/utilities/use-intersection-observer/index_media/0e661a3a21048f0c2e21e6ac705be61661890b52.svg" class="size-3.5" />

Enable

Scroll down 👇

I'm the target! 🎯

Element outside the viewport

## <a href="https://runed.dev/docs/utilities/use-intersection-observer#usage" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Usage

With a reference to an element, you can use the `useIntersectionObserver` utility to watch for intersection changes of the target element.

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="svelte" data-theme="github-light github-dark" tabindex="0"><code>
        &lt;script lang=&quot;ts&quot;&gt;
    import { useIntersectionObserver } from &quot;runed&quot;;
 
    let target = $state&lt;HTMLElement | null&gt;(null);
    let root = $state&lt;HTMLElement | null&gt;(null);
 
    let isIntersecting = $state(false);
 
    useIntersectionObserver(
        () =&gt; target,
        (entries) =&gt; {
            const entry = entries[0];
            if (!entry) return;
            isIntersecting = entry.isIntersecting;
        },
        { root: () =&gt; root }
    );
&lt;/script&gt;
 
&lt;div bind:this={root}&gt;
    &lt;div bind:this={target}&gt;
        {#if isIntersecting}
            &lt;div&gt;Target is intersecting&lt;/div&gt;
        {:else}
            &lt;div&gt;Target is not intersecting&lt;/div&gt;
        {/if}
    &lt;/div&gt;
&lt;/div&gt;
    </code></pre>
<img src="out_runed/utilities/use-intersection-observer/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

### <a href="https://runed.dev/docs/utilities/use-intersection-observer#pause" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Pause

You can pause the intersection observer at any point by calling the `pause` method.

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-theme="github-light github-dark" tabindex="0"><code>
        const observer = useIntersectionObserver(/* ... */);
 
observer.pause();
    </code></pre>
<img src="out_runed/utilities/use-intersection-observer/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

### <a href="https://runed.dev/docs/utilities/use-intersection-observer#resume" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Resume

You can resume the intersection observer at any point by calling the `resume` method.

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-theme="github-light github-dark" tabindex="0"><code>
        const observer = useIntersectionObserver(/* ... */);
 
observer.resume();
    </code></pre>
<img src="out_runed/utilities/use-intersection-observer/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

### <a href="https://runed.dev/docs/utilities/use-intersection-observer#stop" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Stop

You can stop the intersection observer at any point by calling the `stop` method.

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-theme="github-light github-dark" tabindex="0"><code>
        const observer = useIntersectionObserver(/* ... */);
 
observer.stop();
    </code></pre>
<img src="out_runed/utilities/use-intersection-observer/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

### <a href="https://runed.dev/docs/utilities/use-intersection-observer#isactive" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>isActive

You can check if the intersection observer is active by checking the `isActive` property.

<img src="out_runed/utilities/use-intersection-observer/index_media/9f0a101e998d794b6318889157bb9f7ed19c4727.svg" class="size-6" />

##### Warning

This property cannot be destructured as it is a getter. You must access it directly from the observer.

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-theme="github-light github-dark" tabindex="0"><code>
        const observer = useIntersectionObserver(/* ... */);
 
if (observer.isActive) {
    // do something
}
    </code></pre>
<img src="out_runed/utilities/use-intersection-observer/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

## Contributors

<a href="https://github.com/huntabyte" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/use-intersection-observer/index_media/75bcb1d595b8d37466d561f08b890ffc8f1d5425.jpg" id="bits-1135" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="Hunter Johnston" />

Hunter Johnston
