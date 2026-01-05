![](out_runed/utilities/is-idle/index_media/412c1b99df431530f07813ed77ff123e20b0035f.svg) Toggle Sidebar

<img src="out_runed/utilities/is-idle/index_media/2ef71b57cc18afcd4ed82644358ede113ed6d592.svg" class="size-5" />Search Docs ... ⌘ K

<a href="https://github.com/svecosystem/runed" class="inline-flex select-none items-center justify-center gap-2 whitespace-nowrap text-sm font-semibold tracking-wide active:scale-[99%] disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:size-4 [&amp;_svg]:shrink-0 text-foreground hover:dark:bg-foreground/5 hover:dark:ring-foreground/5 hover:bg-foreground/2 hover:ring-foreground/2 bg-transparent ring ring-transparent h-10 w-10 rounded-md [&amp;&gt;svg]:size-5" aria-label="View this project on GitHub" rel="noopener noreferrer" target="_blank"><img src="out_runed/utilities/is-idle/index_media/1d22f722c87a133f2381965da1acf7a54aefc45a.svg" /></a>

<img src="out_runed/utilities/is-idle/index_media/b2f1f2409c2d7a3eaec75643beb67d5092b71fb2.svg" class="size-6" />

# IsIdle

Track if a user is idle and the last time they were active.

`IsIdle` tracks user activity and determines if they're idle based on a configurable timeout. It monitors mouse movement, keyboard input, and touch events to detect user interaction.

## <a href="https://runed.dev/docs/utilities/is-idle#demo" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Demo

Idle: false

Last active: 0s ago

By default, the time of inactivity before marking the user as idle is 1 minute.

In this demo, it's 1 second.

## <a href="https://runed.dev/docs/utilities/is-idle#usage" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Usage

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="svelte" data-theme="github-light github-dark" tabindex="0"><code>
        &lt;script lang=&quot;ts&quot;&gt;
    import { AnimationFrames, IsIdle } from &quot;runed&quot;;
 
    const idle = new IsIdle({ timeout: 1000 });
&lt;/script&gt;
 
&lt;p&gt;Idle: {idle.current}&lt;/p&gt;
&lt;p&gt;
    Last active: {new Date(idle.lastActive).toLocaleTimeString()}
&lt;/p&gt;
    </code></pre>
<img src="out_runed/utilities/is-idle/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

## <a href="https://runed.dev/docs/utilities/is-idle#type-definitions" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Type Definitions

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-theme="github-light github-dark" tabindex="0"><code>
        interface IsIdleOptions {
    /**
     * The events that should set the idle state to `true`
     *
     * @default [&#39;mousemove&#39;, &#39;mousedown&#39;, &#39;resize&#39;, &#39;keydown&#39;, &#39;touchstart&#39;, &#39;wheel&#39;]
     */
    events?: MaybeGetter&lt;(keyof WindowEventMap)[]&gt;;
    /**
     * The timeout in milliseconds before the idle state is set to `true`. Defaults to 60 seconds.
     *
     * @default 60000
     */
    timeout?: MaybeGetter&lt;number&gt;;
    /**
     * Detect document visibility changes
     *
     * @default false
     */
    detectVisibilityChanges?: MaybeGetter&lt;boolean&gt;;
    /**
     * The initial state of the idle property
     *
     * @default false
     */
    initialState?: boolean;
}
 
class IsIdle {
    constructor(options?: IsIdleOptions);
    readonly current: boolean;
    readonly lastActive: number;
}
    </code></pre>
<img src="out_runed/utilities/is-idle/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

## Contributors

<a href="https://github.com/huntabyte" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/is-idle/index_media/75bcb1d595b8d37466d561f08b890ffc8f1d5425.jpg" id="bits-1340" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="Hunter Johnston" />

Hunter Johnston<a href="https://github.com/ckroeper" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/is-idle/index_media/2101f8deff39b4b2ee815a32b57b013368903cbe.png" id="bits-1342" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="ckroeper" />

ckroeper<a href="https://github.com/43081j" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/is-idle/index_media/06f4edda8a9beea51070ce54837509dcdbea04f6.png" id="bits-1344" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="James Garbutt" />

James Garbutt<a href="https://github.com/TGlide" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/is-idle/index_media/4bd89e8b63f277c9d71897d289dcd951b943b3c0.jpg" id="bits-1346" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="Thomas G. Lopes" />

Thomas G. Lopes<a href="https://github.com/Tyson910" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/is-idle/index_media/5c4a56d0d111d638e5d7ecc10b00e4958b981b97.png" id="bits-1348" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="Tyson910" />

Tyson910

© 2025 <a href="https://github.com/svecosystem" class="underline-offset-2 hover:underline" target="_blank">Svecosystem</a>

<a href="https://github.com/svecosystem/runed" class="inline-flex select-none items-center justify-center gap-2 whitespace-nowrap text-sm font-semibold tracking-wide active:scale-[99%] disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:size-4 [&amp;_svg]:shrink-0 text-foreground hover:dark:bg-foreground/5 hover:dark:ring-foreground/5 hover:bg-foreground/2 hover:ring-foreground/2 bg-transparent ring ring-transparent h-10 w-10 rounded-md [&amp;&gt;svg]:size-5" aria-label="View this project on GitHub" rel="noopener noreferrer" target="_blank"><img src="out_runed/utilities/is-idle/index_media/1d22f722c87a133f2381965da1acf7a54aefc45a.svg" /></a>

<img src="out_runed/utilities/is-idle/index_media/b2f1f2409c2d7a3eaec75643beb67d5092b71fb2.svg" class="size-6" />

# IsIdle

Track if a user is idle and the last time they were active.

`IsIdle` tracks user activity and determines if they're idle based on a configurable timeout. It monitors mouse movement, keyboard input, and touch events to detect user interaction.

## <a href="https://runed.dev/docs/utilities/is-idle#demo" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Demo

Idle: false

Last active: 0s ago

By default, the time of inactivity before marking the user as idle is 1 minute.

In this demo, it's 1 second.

## <a href="https://runed.dev/docs/utilities/is-idle#usage" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Usage

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="svelte" data-theme="github-light github-dark" tabindex="0"><code>
        &lt;script lang=&quot;ts&quot;&gt;
    import { AnimationFrames, IsIdle } from &quot;runed&quot;;
 
    const idle = new IsIdle({ timeout: 1000 });
&lt;/script&gt;
 
&lt;p&gt;Idle: {idle.current}&lt;/p&gt;
&lt;p&gt;
    Last active: {new Date(idle.lastActive).toLocaleTimeString()}
&lt;/p&gt;
    </code></pre>
<img src="out_runed/utilities/is-idle/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

## <a href="https://runed.dev/docs/utilities/is-idle#type-definitions" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Type Definitions

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-theme="github-light github-dark" tabindex="0"><code>
        interface IsIdleOptions {
    /**
     * The events that should set the idle state to `true`
     *
     * @default [&#39;mousemove&#39;, &#39;mousedown&#39;, &#39;resize&#39;, &#39;keydown&#39;, &#39;touchstart&#39;, &#39;wheel&#39;]
     */
    events?: MaybeGetter&lt;(keyof WindowEventMap)[]&gt;;
    /**
     * The timeout in milliseconds before the idle state is set to `true`. Defaults to 60 seconds.
     *
     * @default 60000
     */
    timeout?: MaybeGetter&lt;number&gt;;
    /**
     * Detect document visibility changes
     *
     * @default false
     */
    detectVisibilityChanges?: MaybeGetter&lt;boolean&gt;;
    /**
     * The initial state of the idle property
     *
     * @default false
     */
    initialState?: boolean;
}
 
class IsIdle {
    constructor(options?: IsIdleOptions);
    readonly current: boolean;
    readonly lastActive: number;
}
    </code></pre>
<img src="out_runed/utilities/is-idle/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

## Contributors

<a href="https://github.com/huntabyte" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/is-idle/index_media/75bcb1d595b8d37466d561f08b890ffc8f1d5425.jpg" id="bits-1340" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="Hunter Johnston" />

Hunter Johnston<a href="https://github.com/ckroeper" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/is-idle/index_media/2101f8deff39b4b2ee815a32b57b013368903cbe.png" id="bits-1342" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="ckroeper" />

ckroeper<a href="https://github.com/43081j" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/is-idle/index_media/06f4edda8a9beea51070ce54837509dcdbea04f6.png" id="bits-1344" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="James Garbutt" />

James Garbutt<a href="https://github.com/TGlide" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/is-idle/index_media/4bd89e8b63f277c9d71897d289dcd951b943b3c0.jpg" id="bits-1346" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="Thomas G. Lopes" />

Thomas G. Lopes<a href="https://github.com/Tyson910" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/is-idle/index_media/5c4a56d0d111d638e5d7ecc10b00e4958b981b97.png" id="bits-1348" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="Tyson910" />

Tyson910
