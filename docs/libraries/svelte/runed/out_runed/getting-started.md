![](out_runed/getting-started/index_media/412c1b99df431530f07813ed77ff123e20b0035f.svg) Toggle Sidebar

<img src="out_runed/getting-started/index_media/2ef71b57cc18afcd4ed82644358ede113ed6d592.svg" class="size-5" />Search Docs ... ⌘ K

<a href="https://github.com/svecosystem/runed" class="inline-flex select-none items-center justify-center gap-2 whitespace-nowrap text-sm font-semibold tracking-wide active:scale-[99%] disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:size-4 [&amp;_svg]:shrink-0 text-foreground hover:dark:bg-foreground/5 hover:dark:ring-foreground/5 hover:bg-foreground/2 hover:ring-foreground/2 bg-transparent ring ring-transparent h-10 w-10 rounded-md [&amp;&gt;svg]:size-5" aria-label="View this project on GitHub" rel="noopener noreferrer" target="_blank"><img src="out_runed/getting-started/index_media/1d22f722c87a133f2381965da1acf7a54aefc45a.svg" /></a>

<img src="out_runed/getting-started/index_media/b2f1f2409c2d7a3eaec75643beb67d5092b71fb2.svg" class="size-6" />

# Getting Started

Learn how to install and use Runed in your projects.

## <a href="https://runed.dev/docs/getting-started#installation" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Installation

Install Runed using your favorite package manager:

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="bash" data-theme="github-light github-dark" tabindex="0"><code>
        npm install runed
    </code></pre>
<img src="out_runed/getting-started/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

## <a href="https://runed.dev/docs/getting-started#usage" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Usage

Import one of the utilities you need to either a `.svelte` or `.svelte.js|ts` file and start using it:

<figure data-metadata="" data-rehype-pretty-code-figure="">
<pre class="ring" data-language="svelte" data-metadata="" data-theme="github-light github-dark" tabindex="0"><code>
        &lt;script lang=&quot;ts&quot;&gt;
    import { activeElement } from &quot;runed&quot;;
 
    let inputElement = $state&lt;HTMLInputElement | undefined&gt;();
&lt;/script&gt;
 
&lt;input bind:this={inputElement} /&gt;
 
{#if activeElement.current === inputElement}
    The input element is active!
{/if}
    </code></pre>
<img src="out_runed/getting-started/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
<figcaption>component.svelte</figcaption>
</figure>

or

<figure data-metadata="" data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-metadata="" data-theme="github-light github-dark" tabindex="0"><code>
        import { activeElement } from &quot;runed&quot;;
 
function logActiveElement() {
    $effect(() =&gt; {
        console.log(&quot;Active element is &quot;, activeElement.current);
    });
}
 
logActiveElement();
    </code></pre>
<img src="out_runed/getting-started/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
<figcaption>some-module.svelte.ts</figcaption>
</figure>

© 2025 <a href="https://github.com/svecosystem" class="underline-offset-2 hover:underline" target="_blank">Svecosystem</a>

<a href="https://github.com/svecosystem/runed" class="inline-flex select-none items-center justify-center gap-2 whitespace-nowrap text-sm font-semibold tracking-wide active:scale-[99%] disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:size-4 [&amp;_svg]:shrink-0 text-foreground hover:dark:bg-foreground/5 hover:dark:ring-foreground/5 hover:bg-foreground/2 hover:ring-foreground/2 bg-transparent ring ring-transparent h-10 w-10 rounded-md [&amp;&gt;svg]:size-5" aria-label="View this project on GitHub" rel="noopener noreferrer" target="_blank"><img src="out_runed/getting-started/index_media/1d22f722c87a133f2381965da1acf7a54aefc45a.svg" /></a>

<img src="out_runed/getting-started/index_media/b2f1f2409c2d7a3eaec75643beb67d5092b71fb2.svg" class="size-6" />

# Getting Started

Learn how to install and use Runed in your projects.

## <a href="https://runed.dev/docs/getting-started#installation" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Installation

Install Runed using your favorite package manager:

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="bash" data-theme="github-light github-dark" tabindex="0"><code>
        npm install runed
    </code></pre>
<img src="out_runed/getting-started/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

## <a href="https://runed.dev/docs/getting-started#usage" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Usage

Import one of the utilities you need to either a `.svelte` or `.svelte.js|ts` file and start using it:

<figure data-metadata="" data-rehype-pretty-code-figure="">
<pre class="ring" data-language="svelte" data-metadata="" data-theme="github-light github-dark" tabindex="0"><code>
        &lt;script lang=&quot;ts&quot;&gt;
    import { activeElement } from &quot;runed&quot;;
 
    let inputElement = $state&lt;HTMLInputElement | undefined&gt;();
&lt;/script&gt;
 
&lt;input bind:this={inputElement} /&gt;
 
{#if activeElement.current === inputElement}
    The input element is active!
{/if}
    </code></pre>
<img src="out_runed/getting-started/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
<figcaption>component.svelte</figcaption>
</figure>

or

<figure data-metadata="" data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-metadata="" data-theme="github-light github-dark" tabindex="0"><code>
        import { activeElement } from &quot;runed&quot;;
 
function logActiveElement() {
    $effect(() =&gt; {
        console.log(&quot;Active element is &quot;, activeElement.current);
    });
}
 
logActiveElement();
    </code></pre>
<img src="out_runed/getting-started/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
<figcaption>some-module.svelte.ts</figcaption>
</figure>
