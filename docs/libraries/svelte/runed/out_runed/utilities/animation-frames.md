![](out_runed/utilities/animation-frames/index_media/412c1b99df431530f07813ed77ff123e20b0035f.svg) Toggle Sidebar

<img src="out_runed/utilities/animation-frames/index_media/2ef71b57cc18afcd4ed82644358ede113ed6d592.svg" class="size-5" />Search Docs ... ⌘ K

<a href="https://github.com/svecosystem/runed" class="inline-flex select-none items-center justify-center gap-2 whitespace-nowrap text-sm font-semibold tracking-wide active:scale-[99%] disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:size-4 [&amp;_svg]:shrink-0 text-foreground hover:dark:bg-foreground/5 hover:dark:ring-foreground/5 hover:bg-foreground/2 hover:ring-foreground/2 bg-transparent ring ring-transparent h-10 w-10 rounded-md [&amp;&gt;svg]:size-5" aria-label="View this project on GitHub" rel="noopener noreferrer" target="_blank"><img src="out_runed/utilities/animation-frames/index_media/1d22f722c87a133f2381965da1acf7a54aefc45a.svg" /></a>

<img src="out_runed/utilities/animation-frames/index_media/b2f1f2409c2d7a3eaec75643beb67d5092b71fb2.svg" class="size-6" />

# AnimationFrames

A wrapper for requestAnimationFrame with FPS control and frame metrics

`AnimationFrames` provides a declarative API over the browser's <a href="https://developer.mozilla.org/en-US/docs/Web/API/Window/requestAnimationFrame" class="text-brand-link hover:text-brand-link-hover leading-7" rel="noopener noreferrer" target="_blank"><code>requestAnimationFrame</code></a>, offering FPS limiting capabilities and frame metrics while handling cleanup automatically.

## <a href="https://runed.dev/docs/utilities/animation-frames#demo" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Demo

``` text-mono
Frames: 0
FPS: 0
Delta: 0ms
```

![](out_runed/utilities/animation-frames/index_media/6ff2106ab77bb4fa5ddad76193ac26e165bb072f.svg) Start

FPS limit: **10**

Mouse sprite extracted from <a href="https://www.animalwell.net/" class="hover:text-foreground" target="_blank">Animal Well</a>

## <a href="https://runed.dev/docs/utilities/animation-frames#usage" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Usage

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="svelte" data-theme="github-light github-dark" tabindex="0"><code>
        &lt;script lang=&quot;ts&quot;&gt;
    import { AnimationFrames } from &quot;runed&quot;;
    import { Slider } from &quot;../ui/slider&quot;; // Check out shadcn-svelte!
 
    let frames = $state(0);
    let fpsLimit = $state(10);
    let delta = $state(0);
    const animation = new AnimationFrames(
        (args) =&gt; {
            frames++;
            delta = args.delta;
        },
        { fpsLimit: () =&gt; fpsLimit }
    );
 
    const stats = $derived(
        `Frames: ${frames}\nFPS: ${animation.fps.toFixed(0)}\nDelta: ${delta.toFixed(0)}ms`
    );
&lt;/script&gt;
 
&lt;pre&gt;{stats}&lt;/pre&gt;
&lt;button onclick={toggle}&gt;
    {animation.running ? &quot;Stop&quot; : &quot;Start&quot;}
&lt;/button&gt;
&lt;p&gt;
    FPS limit: &lt;b&gt;{fpsLimit}&lt;/b&gt;&lt;i&gt;{fpsLimit === 0 ? &quot; (not limited)&quot; : &quot;&quot;}&lt;/i&gt;
&lt;/p&gt;
&lt;Slider
    value={[fpsLimit]}
    onValueChange={(value) =&gt; (fpsLimit = value[0] ?? 0)}
    min={0}
    max={144} /&gt;
    </code></pre>
<img src="out_runed/utilities/animation-frames/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

## Contributors

<a href="https://github.com/huntabyte" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/animation-frames/index_media/75bcb1d595b8d37466d561f08b890ffc8f1d5425.jpg" id="bits-1531" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="Hunter Johnston" />

Hunter Johnston<a href="https://github.com/TGlide" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/animation-frames/index_media/4bd89e8b63f277c9d71897d289dcd951b943b3c0.jpg" id="bits-1533" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="Thomas G. Lopes" />

Thomas G. Lopes

© 2025 <a href="https://github.com/svecosystem" class="underline-offset-2 hover:underline" target="_blank">Svecosystem</a>

<a href="https://github.com/svecosystem/runed" class="inline-flex select-none items-center justify-center gap-2 whitespace-nowrap text-sm font-semibold tracking-wide active:scale-[99%] disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:size-4 [&amp;_svg]:shrink-0 text-foreground hover:dark:bg-foreground/5 hover:dark:ring-foreground/5 hover:bg-foreground/2 hover:ring-foreground/2 bg-transparent ring ring-transparent h-10 w-10 rounded-md [&amp;&gt;svg]:size-5" aria-label="View this project on GitHub" rel="noopener noreferrer" target="_blank"><img src="out_runed/utilities/animation-frames/index_media/1d22f722c87a133f2381965da1acf7a54aefc45a.svg" /></a>

<img src="out_runed/utilities/animation-frames/index_media/b2f1f2409c2d7a3eaec75643beb67d5092b71fb2.svg" class="size-6" />

# AnimationFrames

A wrapper for requestAnimationFrame with FPS control and frame metrics

`AnimationFrames` provides a declarative API over the browser's <a href="https://developer.mozilla.org/en-US/docs/Web/API/Window/requestAnimationFrame" class="text-brand-link hover:text-brand-link-hover leading-7" rel="noopener noreferrer" target="_blank"><code>requestAnimationFrame</code></a>, offering FPS limiting capabilities and frame metrics while handling cleanup automatically.

## <a href="https://runed.dev/docs/utilities/animation-frames#demo" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Demo

``` text-mono
Frames: 0
FPS: 0
Delta: 0ms
```

![](out_runed/utilities/animation-frames/index_media/6ff2106ab77bb4fa5ddad76193ac26e165bb072f.svg) Start

FPS limit: **10**

Mouse sprite extracted from <a href="https://www.animalwell.net/" class="hover:text-foreground" target="_blank">Animal Well</a>

## <a href="https://runed.dev/docs/utilities/animation-frames#usage" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Usage

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="svelte" data-theme="github-light github-dark" tabindex="0"><code>
        &lt;script lang=&quot;ts&quot;&gt;
    import { AnimationFrames } from &quot;runed&quot;;
    import { Slider } from &quot;../ui/slider&quot;; // Check out shadcn-svelte!
 
    let frames = $state(0);
    let fpsLimit = $state(10);
    let delta = $state(0);
    const animation = new AnimationFrames(
        (args) =&gt; {
            frames++;
            delta = args.delta;
        },
        { fpsLimit: () =&gt; fpsLimit }
    );
 
    const stats = $derived(
        `Frames: ${frames}\nFPS: ${animation.fps.toFixed(0)}\nDelta: ${delta.toFixed(0)}ms`
    );
&lt;/script&gt;
 
&lt;pre&gt;{stats}&lt;/pre&gt;
&lt;button onclick={toggle}&gt;
    {animation.running ? &quot;Stop&quot; : &quot;Start&quot;}
&lt;/button&gt;
&lt;p&gt;
    FPS limit: &lt;b&gt;{fpsLimit}&lt;/b&gt;&lt;i&gt;{fpsLimit === 0 ? &quot; (not limited)&quot; : &quot;&quot;}&lt;/i&gt;
&lt;/p&gt;
&lt;Slider
    value={[fpsLimit]}
    onValueChange={(value) =&gt; (fpsLimit = value[0] ?? 0)}
    min={0}
    max={144} /&gt;
    </code></pre>
<img src="out_runed/utilities/animation-frames/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

## Contributors

<a href="https://github.com/huntabyte" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/animation-frames/index_media/75bcb1d595b8d37466d561f08b890ffc8f1d5425.jpg" id="bits-1531" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="Hunter Johnston" />

Hunter Johnston<a href="https://github.com/TGlide" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/animation-frames/index_media/4bd89e8b63f277c9d71897d289dcd951b943b3c0.jpg" id="bits-1533" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="Thomas G. Lopes" />

Thomas G. Lopes
