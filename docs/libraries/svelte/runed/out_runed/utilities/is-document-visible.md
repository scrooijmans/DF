![](out_runed/utilities/is-document-visible/index_media/412c1b99df431530f07813ed77ff123e20b0035f.svg) Toggle Sidebar

<img src="out_runed/utilities/is-document-visible/index_media/2ef71b57cc18afcd4ed82644358ede113ed6d592.svg" class="size-5" />Search Docs ... ⌘ K

<a href="https://github.com/svecosystem/runed" class="inline-flex select-none items-center justify-center gap-2 whitespace-nowrap text-sm font-semibold tracking-wide active:scale-[99%] disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:size-4 [&amp;_svg]:shrink-0 text-foreground hover:dark:bg-foreground/5 hover:dark:ring-foreground/5 hover:bg-foreground/2 hover:ring-foreground/2 bg-transparent ring ring-transparent h-10 w-10 rounded-md [&amp;&gt;svg]:size-5" aria-label="View this project on GitHub" rel="noopener noreferrer" target="_blank"><img src="out_runed/utilities/is-document-visible/index_media/1d22f722c87a133f2381965da1acf7a54aefc45a.svg" /></a>

<img src="out_runed/utilities/is-document-visible/index_media/b2f1f2409c2d7a3eaec75643beb67d5092b71fb2.svg" class="size-6" />

# IsDocumentVisible

Track whether the current document is visible using the Page Visibility API.

`IsDocumentVisible` provides a reactive boolean that reflects the current document visibility state. It listens to the `visibilitychange` event and updates automatically.

## <a href="https://runed.dev/docs/utilities/is-document-visible#demo" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Demo

Document visible: **false**

Became visible count: **0**

## <a href="https://runed.dev/docs/utilities/is-document-visible#usage" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Usage

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="svelte" data-theme="github-light github-dark" tabindex="0"><code>
        &lt;script lang=&quot;ts&quot;&gt;
    import { IsDocumentVisible } from &quot;runed&quot;;
 
    const visible = new IsDocumentVisible();
&lt;/script&gt;
 
&lt;p&gt;Document visible: {visible.current ? &quot;Yes&quot; : &quot;No&quot;}&lt;/p&gt;
    </code></pre>
<img src="out_runed/utilities/is-document-visible/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

## <a href="https://runed.dev/docs/utilities/is-document-visible#type-definition" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Type Definition

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-theme="github-light github-dark" tabindex="0"><code>
        type IsDocumentVisibleOptions = {
    window?: Window;
    document?: Document;
};
 
class IsDocumentVisible {
    constructor(options?: IsDocumentVisibleOptions);
    readonly current: boolean; // true when document is visible, false when hidden
}
    </code></pre>
<img src="out_runed/utilities/is-document-visible/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

## <a href="https://runed.dev/docs/utilities/is-document-visible#notes" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Notes

- Uses the Page Visibility API via `document.hidden` and `visibilitychange`.
- In non-browser contexts, `current` defaults to `false`.

## Contributors

<a href="https://github.com/Asguho" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/is-document-visible/index_media/1b1c7bcf48f7deabc517a1c0e5cdb16c3e60dd0f.png" id="bits-1301" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="Aske Gudmand-Høyer" />

Aske Gudmand-Høyer

© 2025 <a href="https://github.com/svecosystem" class="underline-offset-2 hover:underline" target="_blank">Svecosystem</a>

<a href="https://github.com/svecosystem/runed" class="inline-flex select-none items-center justify-center gap-2 whitespace-nowrap text-sm font-semibold tracking-wide active:scale-[99%] disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:size-4 [&amp;_svg]:shrink-0 text-foreground hover:dark:bg-foreground/5 hover:dark:ring-foreground/5 hover:bg-foreground/2 hover:ring-foreground/2 bg-transparent ring ring-transparent h-10 w-10 rounded-md [&amp;&gt;svg]:size-5" aria-label="View this project on GitHub" rel="noopener noreferrer" target="_blank"><img src="out_runed/utilities/is-document-visible/index_media/1d22f722c87a133f2381965da1acf7a54aefc45a.svg" /></a>

<img src="out_runed/utilities/is-document-visible/index_media/b2f1f2409c2d7a3eaec75643beb67d5092b71fb2.svg" class="size-6" />

# IsDocumentVisible

Track whether the current document is visible using the Page Visibility API.

`IsDocumentVisible` provides a reactive boolean that reflects the current document visibility state. It listens to the `visibilitychange` event and updates automatically.

## <a href="https://runed.dev/docs/utilities/is-document-visible#demo" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Demo

Document visible: **false**

Became visible count: **0**

## <a href="https://runed.dev/docs/utilities/is-document-visible#usage" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Usage

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="svelte" data-theme="github-light github-dark" tabindex="0"><code>
        &lt;script lang=&quot;ts&quot;&gt;
    import { IsDocumentVisible } from &quot;runed&quot;;
 
    const visible = new IsDocumentVisible();
&lt;/script&gt;
 
&lt;p&gt;Document visible: {visible.current ? &quot;Yes&quot; : &quot;No&quot;}&lt;/p&gt;
    </code></pre>
<img src="out_runed/utilities/is-document-visible/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

## <a href="https://runed.dev/docs/utilities/is-document-visible#type-definition" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Type Definition

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-theme="github-light github-dark" tabindex="0"><code>
        type IsDocumentVisibleOptions = {
    window?: Window;
    document?: Document;
};
 
class IsDocumentVisible {
    constructor(options?: IsDocumentVisibleOptions);
    readonly current: boolean; // true when document is visible, false when hidden
}
    </code></pre>
<img src="out_runed/utilities/is-document-visible/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

## <a href="https://runed.dev/docs/utilities/is-document-visible#notes" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Notes

- Uses the Page Visibility API via `document.hidden` and `visibilitychange`.
- In non-browser contexts, `current` defaults to `false`.

## Contributors

<a href="https://github.com/Asguho" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/is-document-visible/index_media/1b1c7bcf48f7deabc517a1c0e5cdb16c3e60dd0f.png" id="bits-1301" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="Aske Gudmand-Høyer" />

Aske Gudmand-Høyer
