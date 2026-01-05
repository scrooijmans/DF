![](out_runed/utilities/previous/index_media/412c1b99df431530f07813ed77ff123e20b0035f.svg) Toggle Sidebar

<img src="out_runed/utilities/previous/index_media/2ef71b57cc18afcd4ed82644358ede113ed6d592.svg" class="size-5" />Search Docs ... ⌘ K

<a href="https://github.com/svecosystem/runed" class="inline-flex select-none items-center justify-center gap-2 whitespace-nowrap text-sm font-semibold tracking-wide active:scale-[99%] disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:size-4 [&amp;_svg]:shrink-0 text-foreground hover:dark:bg-foreground/5 hover:dark:ring-foreground/5 hover:bg-foreground/2 hover:ring-foreground/2 bg-transparent ring ring-transparent h-10 w-10 rounded-md [&amp;&gt;svg]:size-5" aria-label="View this project on GitHub" rel="noopener noreferrer" target="_blank"><img src="out_runed/utilities/previous/index_media/1d22f722c87a133f2381965da1acf7a54aefc45a.svg" /></a>

<img src="out_runed/utilities/previous/index_media/b2f1f2409c2d7a3eaec75643beb67d5092b71fb2.svg" class="size-6" />

# Previous

A utility that tracks and provides access to the previous value of a reactive getter.

The `Previous` utility creates a reactive wrapper that maintains the previous value of a getter function. This is particularly useful when you need to compare state changes or implement transition effects.

## <a href="https://runed.dev/docs/utilities/previous#demo" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Demo

Count: 0

``` m-0
Previous: undefined
```

## <a href="https://runed.dev/docs/utilities/previous#usage" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Usage

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="svelte" data-theme="github-light github-dark" tabindex="0"><code>
        &lt;script lang=&quot;ts&quot;&gt;
    import { Previous } from &quot;runed&quot;;
 
    let count = $state(0);
    const previous = new Previous(() =&gt; count);
&lt;/script&gt;
 
&lt;div&gt;
    &lt;button onclick={() =&gt; count++}&gt;Count: {count}&lt;/button&gt;
    &lt;pre&gt;Previous: {`${previous.current}`}&lt;/pre&gt;
&lt;/div&gt;
    </code></pre>
<img src="out_runed/utilities/previous/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

## <a href="https://runed.dev/docs/utilities/previous#type-definition" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Type Definition

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-theme="github-light github-dark" tabindex="0"><code>
        class Previous&lt;T&gt; {
    constructor(getter: () =&gt; T);
 
    readonly current: T | undefined; // Previous value
}
    </code></pre>
<img src="out_runed/utilities/previous/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

## Contributors

<a href="https://github.com/huntabyte" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/previous/index_media/75bcb1d595b8d37466d561f08b890ffc8f1d5425.jpg" id="bits-672" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="Hunter Johnston" />

Hunter Johnston<a href="https://github.com/TGlide" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/previous/index_media/4bd89e8b63f277c9d71897d289dcd951b943b3c0.jpg" id="bits-674" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="Thomas G. Lopes" />

Thomas G. Lopes<a href="https://github.com/paoloricciuti" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/previous/index_media/13ed6c5dc1836b247f0850d0c06de9a0c0ce2d7f.jpg" id="bits-676" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="Paolo Ricciuti" />

Paolo Ricciuti<a href="https://github.com/aster-void" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/previous/index_media/45c61944f271a6e4ae359c2590b218b189fe70d6.jpg" id="bits-678" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="Yuki Kobayashi" />

Yuki Kobayashi

© 2025 <a href="https://github.com/svecosystem" class="underline-offset-2 hover:underline" target="_blank">Svecosystem</a>

<a href="https://github.com/svecosystem/runed" class="inline-flex select-none items-center justify-center gap-2 whitespace-nowrap text-sm font-semibold tracking-wide active:scale-[99%] disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:size-4 [&amp;_svg]:shrink-0 text-foreground hover:dark:bg-foreground/5 hover:dark:ring-foreground/5 hover:bg-foreground/2 hover:ring-foreground/2 bg-transparent ring ring-transparent h-10 w-10 rounded-md [&amp;&gt;svg]:size-5" aria-label="View this project on GitHub" rel="noopener noreferrer" target="_blank"><img src="out_runed/utilities/previous/index_media/1d22f722c87a133f2381965da1acf7a54aefc45a.svg" /></a>

<img src="out_runed/utilities/previous/index_media/b2f1f2409c2d7a3eaec75643beb67d5092b71fb2.svg" class="size-6" />

# Previous

A utility that tracks and provides access to the previous value of a reactive getter.

The `Previous` utility creates a reactive wrapper that maintains the previous value of a getter function. This is particularly useful when you need to compare state changes or implement transition effects.

## <a href="https://runed.dev/docs/utilities/previous#demo" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Demo

Count: 0

``` m-0
Previous: undefined
```

## <a href="https://runed.dev/docs/utilities/previous#usage" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Usage

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="svelte" data-theme="github-light github-dark" tabindex="0"><code>
        &lt;script lang=&quot;ts&quot;&gt;
    import { Previous } from &quot;runed&quot;;
 
    let count = $state(0);
    const previous = new Previous(() =&gt; count);
&lt;/script&gt;
 
&lt;div&gt;
    &lt;button onclick={() =&gt; count++}&gt;Count: {count}&lt;/button&gt;
    &lt;pre&gt;Previous: {`${previous.current}`}&lt;/pre&gt;
&lt;/div&gt;
    </code></pre>
<img src="out_runed/utilities/previous/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

## <a href="https://runed.dev/docs/utilities/previous#type-definition" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Type Definition

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-theme="github-light github-dark" tabindex="0"><code>
        class Previous&lt;T&gt; {
    constructor(getter: () =&gt; T);
 
    readonly current: T | undefined; // Previous value
}
    </code></pre>
<img src="out_runed/utilities/previous/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

## Contributors

<a href="https://github.com/huntabyte" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/previous/index_media/75bcb1d595b8d37466d561f08b890ffc8f1d5425.jpg" id="bits-672" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="Hunter Johnston" />

Hunter Johnston<a href="https://github.com/TGlide" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/previous/index_media/4bd89e8b63f277c9d71897d289dcd951b943b3c0.jpg" id="bits-674" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="Thomas G. Lopes" />

Thomas G. Lopes<a href="https://github.com/paoloricciuti" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/previous/index_media/13ed6c5dc1836b247f0850d0c06de9a0c0ce2d7f.jpg" id="bits-676" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="Paolo Ricciuti" />

Paolo Ricciuti<a href="https://github.com/aster-void" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/previous/index_media/45c61944f271a6e4ae359c2590b218b189fe70d6.jpg" id="bits-678" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="Yuki Kobayashi" />

Yuki Kobayashi
