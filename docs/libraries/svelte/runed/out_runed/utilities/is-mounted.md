![](out_runed/utilities/is-mounted/index_media/412c1b99df431530f07813ed77ff123e20b0035f.svg) Toggle Sidebar

<img src="out_runed/utilities/is-mounted/index_media/2ef71b57cc18afcd4ed82644358ede113ed6d592.svg" class="size-5" />Search Docs ... ⌘ K

<a href="https://github.com/svecosystem/runed" class="inline-flex select-none items-center justify-center gap-2 whitespace-nowrap text-sm font-semibold tracking-wide active:scale-[99%] disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:size-4 [&amp;_svg]:shrink-0 text-foreground hover:dark:bg-foreground/5 hover:dark:ring-foreground/5 hover:bg-foreground/2 hover:ring-foreground/2 bg-transparent ring ring-transparent h-10 w-10 rounded-md [&amp;&gt;svg]:size-5" aria-label="View this project on GitHub" rel="noopener noreferrer" target="_blank"><img src="out_runed/utilities/is-mounted/index_media/1d22f722c87a133f2381965da1acf7a54aefc45a.svg" /></a>

<img src="out_runed/utilities/is-mounted/index_media/b2f1f2409c2d7a3eaec75643beb67d5092b71fb2.svg" class="size-6" />

# IsMounted

A class that returns the mounted state of the component it's called in.

## <a href="https://runed.dev/docs/utilities/is-mounted#demo" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Demo

Mounted: **false**

## <a href="https://runed.dev/docs/utilities/is-mounted#usage" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Usage

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="svelte" data-theme="github-light github-dark" tabindex="0"><code>
        &lt;script lang=&quot;ts&quot;&gt;
    import { IsMounted } from &quot;runed&quot;;
 
    const isMounted = new IsMounted();
&lt;/script&gt;
    </code></pre>
<img src="out_runed/utilities/is-mounted/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

Which is a shorthand for one of the following:

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="svelte" data-theme="github-light github-dark" tabindex="0"><code>
        &lt;script lang=&quot;ts&quot;&gt;
    import { onMount } from &quot;svelte&quot;;
 
    const isMounted = $state({ current: false });
 
    onMount(() =&gt; {
        isMounted.current = true;
    });
&lt;/script&gt;
    </code></pre>
<img src="out_runed/utilities/is-mounted/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

or

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="svelte" data-theme="github-light github-dark" tabindex="0"><code>
        &lt;script lang=&quot;ts&quot;&gt;
    import { untrack } from &quot;svelte&quot;;
 
    const isMounted = $state({ current: false });
 
    $effect(() =&gt; {
        untrack(() =&gt; (isMounted.current = true));
    });
&lt;/script&gt;
    </code></pre>
<img src="out_runed/utilities/is-mounted/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

## Contributors

<a href="https://github.com/huntabyte" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/is-mounted/index_media/75bcb1d595b8d37466d561f08b890ffc8f1d5425.jpg" id="bits-1791" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="Hunter Johnston" />

Hunter Johnston<a href="https://github.com/TGlide" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/is-mounted/index_media/4bd89e8b63f277c9d71897d289dcd951b943b3c0.jpg" id="bits-1793" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="Thomas G. Lopes" />

Thomas G. Lopes

© 2025 <a href="https://github.com/svecosystem" class="underline-offset-2 hover:underline" target="_blank">Svecosystem</a>

<a href="https://github.com/svecosystem/runed" class="inline-flex select-none items-center justify-center gap-2 whitespace-nowrap text-sm font-semibold tracking-wide active:scale-[99%] disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:size-4 [&amp;_svg]:shrink-0 text-foreground hover:dark:bg-foreground/5 hover:dark:ring-foreground/5 hover:bg-foreground/2 hover:ring-foreground/2 bg-transparent ring ring-transparent h-10 w-10 rounded-md [&amp;&gt;svg]:size-5" aria-label="View this project on GitHub" rel="noopener noreferrer" target="_blank"><img src="out_runed/utilities/is-mounted/index_media/1d22f722c87a133f2381965da1acf7a54aefc45a.svg" /></a>

<img src="out_runed/utilities/is-mounted/index_media/b2f1f2409c2d7a3eaec75643beb67d5092b71fb2.svg" class="size-6" />

# IsMounted

A class that returns the mounted state of the component it's called in.

## <a href="https://runed.dev/docs/utilities/is-mounted#demo" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Demo

Mounted: **false**

## <a href="https://runed.dev/docs/utilities/is-mounted#usage" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Usage

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="svelte" data-theme="github-light github-dark" tabindex="0"><code>
        &lt;script lang=&quot;ts&quot;&gt;
    import { IsMounted } from &quot;runed&quot;;
 
    const isMounted = new IsMounted();
&lt;/script&gt;
    </code></pre>
<img src="out_runed/utilities/is-mounted/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

Which is a shorthand for one of the following:

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="svelte" data-theme="github-light github-dark" tabindex="0"><code>
        &lt;script lang=&quot;ts&quot;&gt;
    import { onMount } from &quot;svelte&quot;;
 
    const isMounted = $state({ current: false });
 
    onMount(() =&gt; {
        isMounted.current = true;
    });
&lt;/script&gt;
    </code></pre>
<img src="out_runed/utilities/is-mounted/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

or

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="svelte" data-theme="github-light github-dark" tabindex="0"><code>
        &lt;script lang=&quot;ts&quot;&gt;
    import { untrack } from &quot;svelte&quot;;
 
    const isMounted = $state({ current: false });
 
    $effect(() =&gt; {
        untrack(() =&gt; (isMounted.current = true));
    });
&lt;/script&gt;
    </code></pre>
<img src="out_runed/utilities/is-mounted/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

## Contributors

<a href="https://github.com/huntabyte" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/is-mounted/index_media/75bcb1d595b8d37466d561f08b890ffc8f1d5425.jpg" id="bits-1791" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="Hunter Johnston" />

Hunter Johnston<a href="https://github.com/TGlide" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/is-mounted/index_media/4bd89e8b63f277c9d71897d289dcd951b943b3c0.jpg" id="bits-1793" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="Thomas G. Lopes" />

Thomas G. Lopes
