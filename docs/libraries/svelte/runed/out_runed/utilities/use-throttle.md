![](out_runed/utilities/use-throttle/index_media/412c1b99df431530f07813ed77ff123e20b0035f.svg) Toggle Sidebar

<img src="out_runed/utilities/use-throttle/index_media/2ef71b57cc18afcd4ed82644358ede113ed6d592.svg" class="size-5" />Search Docs ... ⌘ K

<a href="https://github.com/svecosystem/runed" class="inline-flex select-none items-center justify-center gap-2 whitespace-nowrap text-sm font-semibold tracking-wide active:scale-[99%] disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:size-4 [&amp;_svg]:shrink-0 text-foreground hover:dark:bg-foreground/5 hover:dark:ring-foreground/5 hover:bg-foreground/2 hover:ring-foreground/2 bg-transparent ring ring-transparent h-10 w-10 rounded-md [&amp;&gt;svg]:size-5" aria-label="View this project on GitHub" rel="noopener noreferrer" target="_blank"><img src="out_runed/utilities/use-throttle/index_media/1d22f722c87a133f2381965da1acf7a54aefc45a.svg" /></a>

<img src="out_runed/utilities/use-throttle/index_media/b2f1f2409c2d7a3eaec75643beb67d5092b71fb2.svg" class="size-6" />

# useThrottle

A higher-order function that throttles the execution of a function.

## <a href="https://runed.dev/docs/utilities/use-throttle#demo" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Demo

Throttle duration (ms)

Search

Search for something above!

## <a href="https://runed.dev/docs/utilities/use-throttle#usage" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Usage

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="svelte" data-theme="github-light github-dark" tabindex="0"><code>
        &lt;script lang=&quot;ts&quot;&gt;
    import { useThrottle } from &quot;runed&quot;;
 
    let search = $state(&quot;&quot;);
    let throttledSearch = $state(&quot;&quot;);
    let durationMs = $state(1000);
 
    const throttledUpdate = useThrottle(
        () =&gt; {
            throttledSearch = search;
        },
        () =&gt; durationMs
    );
&lt;/script&gt;
 
&lt;div&gt;
    &lt;input
        bind:value={
            () =&gt; search,
            (v) =&gt; {
                search = v;
                throttledUpdate();
            }
        } /&gt;
    &lt;p&gt;You searched for: &lt;b&gt;{throttledSearch}&lt;/b&gt;&lt;/p&gt;
&lt;/div&gt;
    </code></pre>
<img src="out_runed/utilities/use-throttle/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

## Contributors

<a href="https://github.com/Not-Jayden" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/use-throttle/index_media/b0fb45ada1c1e0d6d338655b9bb4932aea0b1d6e.png" id="bits-1744" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="Jayden Carey" />

Jayden Carey<a href="https://github.com/ejb" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/use-throttle/index_media/9b6430190704cba50968f46c66397aacd2d05e37.jpg" id="bits-1746" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="Elliot Bentley" />

Elliot Bentley

© 2025 <a href="https://github.com/svecosystem" class="underline-offset-2 hover:underline" target="_blank">Svecosystem</a>

<a href="https://github.com/svecosystem/runed" class="inline-flex select-none items-center justify-center gap-2 whitespace-nowrap text-sm font-semibold tracking-wide active:scale-[99%] disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:size-4 [&amp;_svg]:shrink-0 text-foreground hover:dark:bg-foreground/5 hover:dark:ring-foreground/5 hover:bg-foreground/2 hover:ring-foreground/2 bg-transparent ring ring-transparent h-10 w-10 rounded-md [&amp;&gt;svg]:size-5" aria-label="View this project on GitHub" rel="noopener noreferrer" target="_blank"><img src="out_runed/utilities/use-throttle/index_media/1d22f722c87a133f2381965da1acf7a54aefc45a.svg" /></a>

<img src="out_runed/utilities/use-throttle/index_media/b2f1f2409c2d7a3eaec75643beb67d5092b71fb2.svg" class="size-6" />

# useThrottle

A higher-order function that throttles the execution of a function.

## <a href="https://runed.dev/docs/utilities/use-throttle#demo" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Demo

Throttle duration (ms)

Search

Search for something above!

## <a href="https://runed.dev/docs/utilities/use-throttle#usage" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Usage

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="svelte" data-theme="github-light github-dark" tabindex="0"><code>
        &lt;script lang=&quot;ts&quot;&gt;
    import { useThrottle } from &quot;runed&quot;;
 
    let search = $state(&quot;&quot;);
    let throttledSearch = $state(&quot;&quot;);
    let durationMs = $state(1000);
 
    const throttledUpdate = useThrottle(
        () =&gt; {
            throttledSearch = search;
        },
        () =&gt; durationMs
    );
&lt;/script&gt;
 
&lt;div&gt;
    &lt;input
        bind:value={
            () =&gt; search,
            (v) =&gt; {
                search = v;
                throttledUpdate();
            }
        } /&gt;
    &lt;p&gt;You searched for: &lt;b&gt;{throttledSearch}&lt;/b&gt;&lt;/p&gt;
&lt;/div&gt;
    </code></pre>
<img src="out_runed/utilities/use-throttle/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

## Contributors

<a href="https://github.com/Not-Jayden" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/use-throttle/index_media/b0fb45ada1c1e0d6d338655b9bb4932aea0b1d6e.png" id="bits-1744" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="Jayden Carey" />

Jayden Carey<a href="https://github.com/ejb" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/use-throttle/index_media/9b6430190704cba50968f46c66397aacd2d05e37.jpg" id="bits-1746" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="Elliot Bentley" />

Elliot Bentley
