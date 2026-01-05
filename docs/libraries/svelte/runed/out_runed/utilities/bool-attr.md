![](out_runed/utilities/bool-attr/index_media/412c1b99df431530f07813ed77ff123e20b0035f.svg) Toggle Sidebar

<img src="out_runed/utilities/bool-attr/index_media/2ef71b57cc18afcd4ed82644358ede113ed6d592.svg" class="size-5" />Search Docs ... ⌘ K

<a href="https://github.com/svecosystem/runed" class="inline-flex select-none items-center justify-center gap-2 whitespace-nowrap text-sm font-semibold tracking-wide active:scale-[99%] disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:size-4 [&amp;_svg]:shrink-0 text-foreground hover:dark:bg-foreground/5 hover:dark:ring-foreground/5 hover:bg-foreground/2 hover:ring-foreground/2 bg-transparent ring ring-transparent h-10 w-10 rounded-md [&amp;&gt;svg]:size-5" aria-label="View this project on GitHub" rel="noopener noreferrer" target="_blank"><img src="out_runed/utilities/bool-attr/index_media/1d22f722c87a133f2381965da1acf7a54aefc45a.svg" /></a>

<img src="out_runed/utilities/bool-attr/index_media/b2f1f2409c2d7a3eaec75643beb67d5092b71fb2.svg" class="size-6" />

# boolAttr

A utility that transforms any value into \`""\` (empty string) or \`undefined\` for use with HTML attributes where presence indicates truth.

The `boolAttr` utility transforms any value into either an empty string (`""`) or `undefined` for use with HTML attributes where presence indicates truth. This is essential for creating proper boolean attributes in HTML, where the presence of the attribute (regardless of value) indicates a truthy state.

## <a href="https://runed.dev/docs/utilities/bool-attr#problem" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Problem

When using boolean values directly in Svelte attributes, they often render as string values:

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="svelte" data-theme="github-light github-dark" tabindex="0"><code>
        &lt;!-- This renders as: &lt;div data-active=&quot;true&quot;&gt; --&gt;
&lt;div data-active={true}&gt;Content&lt;/div&gt;
 
&lt;!-- This renders as: &lt;div data-active=&quot;false&quot;&gt; --&gt;
&lt;div data-active={false}&gt;Content&lt;/div&gt;
    </code></pre>
<img src="out_runed/utilities/bool-attr/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

This creates issues with CSS selectors and attribute-based styling, as both cases result in the attribute being present.

## <a href="https://runed.dev/docs/utilities/bool-attr#solution" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Solution

`boolAttr` ensures proper boolean attribute behavior:

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="svelte" data-theme="github-light github-dark" tabindex="0"><code>
        &lt;script lang=&quot;ts&quot;&gt;
    import { boolAttr } from &quot;runed&quot;;
 
    let isActive = $state(true);
    let isLoading = $state(false);
&lt;/script&gt;
 
&lt;!-- Renders as: &lt;div data-active&gt; --&gt;
&lt;div data-active={boolAttr(isActive)}&gt;Active content&lt;/div&gt;
 
&lt;!-- Renders as: &lt;div&gt; (no data-loading attribute) --&gt;
&lt;div data-loading={boolAttr(isLoading)}&gt;Loading content&lt;/div&gt;
    </code></pre>
<img src="out_runed/utilities/bool-attr/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

## <a href="https://runed.dev/docs/utilities/bool-attr#type-definition" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Type Definition

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-theme="github-light github-dark" tabindex="0"><code>
        function boolAttr(value: unknown): &quot;&quot; | undefined;
    </code></pre>
<img src="out_runed/utilities/bool-attr/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

### <a href="https://runed.dev/docs/utilities/bool-attr#parameters" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Parameters

- **`value`** (`unknown`) - Any value to be converted to a boolean attribute

### <a href="https://runed.dev/docs/utilities/bool-attr#returns" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Returns

- **`""`** (empty string) - When `value` is truthy
- **`undefined`** - When `value` is falsy

## Contributors

<a href="https://github.com/Carbonateb" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/bool-attr/index_media/d5135f3cffad25e11de22726201da7eee09d8b76.png" id="bits-1572" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="Lucas Champagne" />

Lucas Champagne

© 2025 <a href="https://github.com/svecosystem" class="underline-offset-2 hover:underline" target="_blank">Svecosystem</a>

<a href="https://github.com/svecosystem/runed" class="inline-flex select-none items-center justify-center gap-2 whitespace-nowrap text-sm font-semibold tracking-wide active:scale-[99%] disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:size-4 [&amp;_svg]:shrink-0 text-foreground hover:dark:bg-foreground/5 hover:dark:ring-foreground/5 hover:bg-foreground/2 hover:ring-foreground/2 bg-transparent ring ring-transparent h-10 w-10 rounded-md [&amp;&gt;svg]:size-5" aria-label="View this project on GitHub" rel="noopener noreferrer" target="_blank"><img src="out_runed/utilities/bool-attr/index_media/1d22f722c87a133f2381965da1acf7a54aefc45a.svg" /></a>

<img src="out_runed/utilities/bool-attr/index_media/b2f1f2409c2d7a3eaec75643beb67d5092b71fb2.svg" class="size-6" />

# boolAttr

A utility that transforms any value into \`""\` (empty string) or \`undefined\` for use with HTML attributes where presence indicates truth.

The `boolAttr` utility transforms any value into either an empty string (`""`) or `undefined` for use with HTML attributes where presence indicates truth. This is essential for creating proper boolean attributes in HTML, where the presence of the attribute (regardless of value) indicates a truthy state.

## <a href="https://runed.dev/docs/utilities/bool-attr#problem" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Problem

When using boolean values directly in Svelte attributes, they often render as string values:

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="svelte" data-theme="github-light github-dark" tabindex="0"><code>
        &lt;!-- This renders as: &lt;div data-active=&quot;true&quot;&gt; --&gt;
&lt;div data-active={true}&gt;Content&lt;/div&gt;
 
&lt;!-- This renders as: &lt;div data-active=&quot;false&quot;&gt; --&gt;
&lt;div data-active={false}&gt;Content&lt;/div&gt;
    </code></pre>
<img src="out_runed/utilities/bool-attr/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

This creates issues with CSS selectors and attribute-based styling, as both cases result in the attribute being present.

## <a href="https://runed.dev/docs/utilities/bool-attr#solution" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Solution

`boolAttr` ensures proper boolean attribute behavior:

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="svelte" data-theme="github-light github-dark" tabindex="0"><code>
        &lt;script lang=&quot;ts&quot;&gt;
    import { boolAttr } from &quot;runed&quot;;
 
    let isActive = $state(true);
    let isLoading = $state(false);
&lt;/script&gt;
 
&lt;!-- Renders as: &lt;div data-active&gt; --&gt;
&lt;div data-active={boolAttr(isActive)}&gt;Active content&lt;/div&gt;
 
&lt;!-- Renders as: &lt;div&gt; (no data-loading attribute) --&gt;
&lt;div data-loading={boolAttr(isLoading)}&gt;Loading content&lt;/div&gt;
    </code></pre>
<img src="out_runed/utilities/bool-attr/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

## <a href="https://runed.dev/docs/utilities/bool-attr#type-definition" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Type Definition

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-theme="github-light github-dark" tabindex="0"><code>
        function boolAttr(value: unknown): &quot;&quot; | undefined;
    </code></pre>
<img src="out_runed/utilities/bool-attr/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

### <a href="https://runed.dev/docs/utilities/bool-attr#parameters" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Parameters

- **`value`** (`unknown`) - Any value to be converted to a boolean attribute

### <a href="https://runed.dev/docs/utilities/bool-attr#returns" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Returns

- **`""`** (empty string) - When `value` is truthy
- **`undefined`** - When `value` is falsy

## Contributors

<a href="https://github.com/Carbonateb" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/bool-attr/index_media/d5135f3cffad25e11de22726201da7eee09d8b76.png" id="bits-1572" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="Lucas Champagne" />

Lucas Champagne
