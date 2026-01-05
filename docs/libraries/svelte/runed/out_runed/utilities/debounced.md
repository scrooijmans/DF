![](out_runed/utilities/debounced/index_media/412c1b99df431530f07813ed77ff123e20b0035f.svg) Toggle Sidebar

<img src="out_runed/utilities/debounced/index_media/2ef71b57cc18afcd4ed82644358ede113ed6d592.svg" class="size-5" />Search Docs ... ⌘ K

<a href="https://github.com/svecosystem/runed" class="inline-flex select-none items-center justify-center gap-2 whitespace-nowrap text-sm font-semibold tracking-wide active:scale-[99%] disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:size-4 [&amp;_svg]:shrink-0 text-foreground hover:dark:bg-foreground/5 hover:dark:ring-foreground/5 hover:bg-foreground/2 hover:ring-foreground/2 bg-transparent ring ring-transparent h-10 w-10 rounded-md [&amp;&gt;svg]:size-5" aria-label="View this project on GitHub" rel="noopener noreferrer" target="_blank"><img src="out_runed/utilities/debounced/index_media/1d22f722c87a133f2381965da1acf7a54aefc45a.svg" /></a>

<img src="out_runed/utilities/debounced/index_media/b2f1f2409c2d7a3eaec75643beb67d5092b71fb2.svg" class="size-6" />

# Debounced

A wrapper over \`useDebounce\` that returns a debounced state.

## <a href="https://runed.dev/docs/utilities/debounced#demo" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Demo

Search for something above!

## <a href="https://runed.dev/docs/utilities/debounced#usage" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Usage

This is a simple wrapper over <a href="https://runed.dev/docs/utilities/use-debounce" class="text-brand-link hover:text-brand-link-hover leading-7" rel="noopener noreferrer" target="_blank"><code>useDebounce</code></a> that returns a debounced state.

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="svelte" data-theme="github-light github-dark" tabindex="0"><code>
        &lt;script lang=&quot;ts&quot;&gt;
    import { Debounced } from &quot;runed&quot;;
 
    let search = $state(&quot;&quot;);
    const debounced = new Debounced(() =&gt; search, 500);
&lt;/script&gt;
 
&lt;div&gt;
    &lt;input bind:value={search} /&gt;
    &lt;p&gt;You searched for: &lt;b&gt;{debounced.current}&lt;/b&gt;&lt;/p&gt;
&lt;/div&gt;
    </code></pre>
<img src="out_runed/utilities/debounced/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

You may cancel the pending update, run it immediately, or set a new value. Setting a new value immediately also cancels any pending updates.

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-theme="github-light github-dark" tabindex="0"><code>
        let count = $state(0);
const debounced = new Debounced(() =&gt; count, 500);
count = 1;
debounced.cancel();
// after a while...
console.log(debounced.current); // Still 0!
 
count = 2;
console.log(debounced.current); // Still 0!
debounced.setImmediately(count);
console.log(debounced.current); // 2
 
count = 3;
console.log(debounced.current); // 2
await debounced.updateImmediately();
console.log(debounced.current); // 3
    </code></pre>
<img src="out_runed/utilities/debounced/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

## Contributors

<a href="https://github.com/huntabyte" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/debounced/index_media/75bcb1d595b8d37466d561f08b890ffc8f1d5425.jpg" id="bits-467" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="Hunter Johnston" />

Hunter Johnston<a href="https://github.com/TGlide" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/debounced/index_media/4bd89e8b63f277c9d71897d289dcd951b943b3c0.jpg" id="bits-469" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="Thomas G. Lopes" />

Thomas G. Lopes<a href="https://github.com/aster-void" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/debounced/index_media/45c61944f271a6e4ae359c2590b218b189fe70d6.jpg" id="bits-471" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="Yuki Kobayashi" />

Yuki Kobayashi<a href="https://github.com/Coronon" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/debounced/index_media/525501a25f12e9efbe89359218a602dabfc784f1.jpg" id="bits-473" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="Rubin Raithel" />

Rubin Raithel

© 2025 <a href="https://github.com/svecosystem" class="underline-offset-2 hover:underline" target="_blank">Svecosystem</a>

<a href="https://github.com/svecosystem/runed" class="inline-flex select-none items-center justify-center gap-2 whitespace-nowrap text-sm font-semibold tracking-wide active:scale-[99%] disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:size-4 [&amp;_svg]:shrink-0 text-foreground hover:dark:bg-foreground/5 hover:dark:ring-foreground/5 hover:bg-foreground/2 hover:ring-foreground/2 bg-transparent ring ring-transparent h-10 w-10 rounded-md [&amp;&gt;svg]:size-5" aria-label="View this project on GitHub" rel="noopener noreferrer" target="_blank"><img src="out_runed/utilities/debounced/index_media/1d22f722c87a133f2381965da1acf7a54aefc45a.svg" /></a>

<img src="out_runed/utilities/debounced/index_media/b2f1f2409c2d7a3eaec75643beb67d5092b71fb2.svg" class="size-6" />

# Debounced

A wrapper over \`useDebounce\` that returns a debounced state.

## <a href="https://runed.dev/docs/utilities/debounced#demo" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Demo

Search for something above!

## <a href="https://runed.dev/docs/utilities/debounced#usage" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Usage

This is a simple wrapper over <a href="https://runed.dev/docs/utilities/use-debounce" class="text-brand-link hover:text-brand-link-hover leading-7" rel="noopener noreferrer" target="_blank"><code>useDebounce</code></a> that returns a debounced state.

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="svelte" data-theme="github-light github-dark" tabindex="0"><code>
        &lt;script lang=&quot;ts&quot;&gt;
    import { Debounced } from &quot;runed&quot;;
 
    let search = $state(&quot;&quot;);
    const debounced = new Debounced(() =&gt; search, 500);
&lt;/script&gt;
 
&lt;div&gt;
    &lt;input bind:value={search} /&gt;
    &lt;p&gt;You searched for: &lt;b&gt;{debounced.current}&lt;/b&gt;&lt;/p&gt;
&lt;/div&gt;
    </code></pre>
<img src="out_runed/utilities/debounced/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

You may cancel the pending update, run it immediately, or set a new value. Setting a new value immediately also cancels any pending updates.

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-theme="github-light github-dark" tabindex="0"><code>
        let count = $state(0);
const debounced = new Debounced(() =&gt; count, 500);
count = 1;
debounced.cancel();
// after a while...
console.log(debounced.current); // Still 0!
 
count = 2;
console.log(debounced.current); // Still 0!
debounced.setImmediately(count);
console.log(debounced.current); // 2
 
count = 3;
console.log(debounced.current); // 2
await debounced.updateImmediately();
console.log(debounced.current); // 3
    </code></pre>
<img src="out_runed/utilities/debounced/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

## Contributors

<a href="https://github.com/huntabyte" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/debounced/index_media/75bcb1d595b8d37466d561f08b890ffc8f1d5425.jpg" id="bits-467" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="Hunter Johnston" />

Hunter Johnston<a href="https://github.com/TGlide" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/debounced/index_media/4bd89e8b63f277c9d71897d289dcd951b943b3c0.jpg" id="bits-469" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="Thomas G. Lopes" />

Thomas G. Lopes<a href="https://github.com/aster-void" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/debounced/index_media/45c61944f271a6e4ae359c2590b218b189fe70d6.jpg" id="bits-471" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="Yuki Kobayashi" />

Yuki Kobayashi<a href="https://github.com/Coronon" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/debounced/index_media/525501a25f12e9efbe89359218a602dabfc784f1.jpg" id="bits-473" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="Rubin Raithel" />

Rubin Raithel
