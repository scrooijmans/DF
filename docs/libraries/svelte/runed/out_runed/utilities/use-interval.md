![](out_runed/utilities/use-interval/index_media/412c1b99df431530f07813ed77ff123e20b0035f.svg) Toggle Sidebar

<img src="out_runed/utilities/use-interval/index_media/2ef71b57cc18afcd4ed82644358ede113ed6d592.svg" class="size-5" />Search Docs ... ⌘ K

<a href="https://github.com/svecosystem/runed" class="inline-flex select-none items-center justify-center gap-2 whitespace-nowrap text-sm font-semibold tracking-wide active:scale-[99%] disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:size-4 [&amp;_svg]:shrink-0 text-foreground hover:dark:bg-foreground/5 hover:dark:ring-foreground/5 hover:bg-foreground/2 hover:ring-foreground/2 bg-transparent ring ring-transparent h-10 w-10 rounded-md [&amp;&gt;svg]:size-5" aria-label="View this project on GitHub" rel="noopener noreferrer" target="_blank"><img src="out_runed/utilities/use-interval/index_media/1d22f722c87a133f2381965da1acf7a54aefc45a.svg" /></a>

<img src="out_runed/utilities/use-interval/index_media/b2f1f2409c2d7a3eaec75643beb67d5092b71fb2.svg" class="size-6" />

# useInterval

A wrapper for setInterval with controls for pausing, resuming, and tracking ticks.

`useInterval` is a utility function that provides a reactive wrapper around `setInterval` with controls for pausing and resuming the execution, as well as a built-in counter for tracking ticks.

## <a href="https://runed.dev/docs/utilities/use-interval#demo" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Demo

Interval duration (ms)

Pause

Resume

Reset Counter

**Counter:** 0

**Status:** Paused

## <a href="https://runed.dev/docs/utilities/use-interval#usage" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Usage

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="svelte" data-theme="github-light github-dark" tabindex="0"><code>
        &lt;script lang=&quot;ts&quot;&gt;
    import { useInterval } from &quot;runed&quot;;
 
    let delay = $state(1000);
 
    const interval = useInterval(() =&gt; delay, {
        callback: (count) =&gt; {
            console.log(`Tick ${count}`);
        }
    });
&lt;/script&gt;
 
&lt;p&gt;Counter: {interval.counter}&lt;/p&gt;
&lt;p&gt;Interval delay: {delay}ms&lt;/p&gt;
&lt;p&gt;Status: {interval.isActive ? &quot;Running&quot; : &quot;Paused&quot;}&lt;/p&gt;
 
&lt;input type=&quot;number&quot; bind:value={delay} min=&quot;100&quot; step=&quot;100&quot; /&gt;
 
&lt;button onclick={interval.pause} disabled={!interval.isActive}&gt;Pause&lt;/button&gt;
&lt;button onclick={interval.resume} disabled={interval.isActive}&gt;Resume&lt;/button&gt;
&lt;button onclick={interval.reset}&gt;Reset Counter&lt;/button&gt;
    </code></pre>
<img src="out_runed/utilities/use-interval/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

## <a href="https://runed.dev/docs/utilities/use-interval#counter" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Counter

`useInterval` includes a built-in `counter` property that tracks the number of times the interval has ticked:

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="svelte" data-theme="github-light github-dark" tabindex="0"><code>
        &lt;script lang=&quot;ts&quot;&gt;
    import { useInterval } from &quot;runed&quot;;
 
    const interval = useInterval(1000);
&lt;/script&gt;
 
&lt;p&gt;Ticks: {interval.counter}&lt;/p&gt;
&lt;button onclick={interval.reset}&gt;Reset&lt;/button&gt;
    </code></pre>
<img src="out_runed/utilities/use-interval/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

## <a href="https://runed.dev/docs/utilities/use-interval#callback" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Callback

You can provide an optional callback that will be called on each tick with the current counter value:

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="svelte" data-theme="github-light github-dark" tabindex="0"><code>
        &lt;script lang=&quot;ts&quot;&gt;
    import { useInterval } from &quot;runed&quot;;
 
    const interval = useInterval(1000, {
        callback: (count) =&gt; {
            console.log(`Tick number ${count}`);
        }
    });
&lt;/script&gt;
    </code></pre>
<img src="out_runed/utilities/use-interval/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

## <a href="https://runed.dev/docs/utilities/use-interval#options" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Options

The `useInterval` function accepts an optional second parameter with the following options:

- `immediate` (default: `true`) - Whether to start the interval immediately
- `immediateCallback` (default: `false`) - Whether to execute the callback immediately when resuming
- `callback` - Optional callback function that receives the current counter value on each tick

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="svelte" data-theme="github-light github-dark" tabindex="0"><code>
        &lt;script lang=&quot;ts&quot;&gt;
    import { useInterval } from &quot;runed&quot;;
 
    const interval = useInterval(1000, {
        immediate: false,
        immediateCallback: true,
        callback: (count) =&gt; console.log(count)
    });
&lt;/script&gt;
    </code></pre>
<img src="out_runed/utilities/use-interval/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

## <a href="https://runed.dev/docs/utilities/use-interval#reactive-interval" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Reactive Interval

The interval delay can be reactive, and the timer will automatically restart with the new interval when it changes:

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="svelte" data-theme="github-light github-dark" tabindex="0"><code>
        &lt;script lang=&quot;ts&quot;&gt;
    import { useInterval } from &quot;runed&quot;;
 
    let delay = $state(1000);
 
    const interval = useInterval(() =&gt; delay);
&lt;/script&gt;
 
&lt;input type=&quot;range&quot; bind:value={delay} min=&quot;100&quot; max=&quot;2000&quot; /&gt;&lt;p&gt;Delay: {delay}ms&lt;/p&gt;
    </code></pre>
<img src="out_runed/utilities/use-interval/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

## Contributors

<a href="https://github.com/TGlide" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/use-interval/index_media/4bd89e8b63f277c9d71897d289dcd951b943b3c0.jpg" id="bits-1707" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="Thomas G. Lopes" />

Thomas G. Lopes<a href="https://github.com/paoloricciuti" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/use-interval/index_media/13ed6c5dc1836b247f0850d0c06de9a0c0ce2d7f.jpg" id="bits-1709" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="Paolo Ricciuti" />

Paolo Ricciuti

© 2025 <a href="https://github.com/svecosystem" class="underline-offset-2 hover:underline" target="_blank">Svecosystem</a>

<a href="https://github.com/svecosystem/runed" class="inline-flex select-none items-center justify-center gap-2 whitespace-nowrap text-sm font-semibold tracking-wide active:scale-[99%] disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:size-4 [&amp;_svg]:shrink-0 text-foreground hover:dark:bg-foreground/5 hover:dark:ring-foreground/5 hover:bg-foreground/2 hover:ring-foreground/2 bg-transparent ring ring-transparent h-10 w-10 rounded-md [&amp;&gt;svg]:size-5" aria-label="View this project on GitHub" rel="noopener noreferrer" target="_blank"><img src="out_runed/utilities/use-interval/index_media/1d22f722c87a133f2381965da1acf7a54aefc45a.svg" /></a>

<img src="out_runed/utilities/use-interval/index_media/b2f1f2409c2d7a3eaec75643beb67d5092b71fb2.svg" class="size-6" />

# useInterval

A wrapper for setInterval with controls for pausing, resuming, and tracking ticks.

`useInterval` is a utility function that provides a reactive wrapper around `setInterval` with controls for pausing and resuming the execution, as well as a built-in counter for tracking ticks.

## <a href="https://runed.dev/docs/utilities/use-interval#demo" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Demo

Interval duration (ms)

Pause

Resume

Reset Counter

**Counter:** 0

**Status:** Paused

## <a href="https://runed.dev/docs/utilities/use-interval#usage" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Usage

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="svelte" data-theme="github-light github-dark" tabindex="0"><code>
        &lt;script lang=&quot;ts&quot;&gt;
    import { useInterval } from &quot;runed&quot;;
 
    let delay = $state(1000);
 
    const interval = useInterval(() =&gt; delay, {
        callback: (count) =&gt; {
            console.log(`Tick ${count}`);
        }
    });
&lt;/script&gt;
 
&lt;p&gt;Counter: {interval.counter}&lt;/p&gt;
&lt;p&gt;Interval delay: {delay}ms&lt;/p&gt;
&lt;p&gt;Status: {interval.isActive ? &quot;Running&quot; : &quot;Paused&quot;}&lt;/p&gt;
 
&lt;input type=&quot;number&quot; bind:value={delay} min=&quot;100&quot; step=&quot;100&quot; /&gt;
 
&lt;button onclick={interval.pause} disabled={!interval.isActive}&gt;Pause&lt;/button&gt;
&lt;button onclick={interval.resume} disabled={interval.isActive}&gt;Resume&lt;/button&gt;
&lt;button onclick={interval.reset}&gt;Reset Counter&lt;/button&gt;
    </code></pre>
<img src="out_runed/utilities/use-interval/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

## <a href="https://runed.dev/docs/utilities/use-interval#counter" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Counter

`useInterval` includes a built-in `counter` property that tracks the number of times the interval has ticked:

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="svelte" data-theme="github-light github-dark" tabindex="0"><code>
        &lt;script lang=&quot;ts&quot;&gt;
    import { useInterval } from &quot;runed&quot;;
 
    const interval = useInterval(1000);
&lt;/script&gt;
 
&lt;p&gt;Ticks: {interval.counter}&lt;/p&gt;
&lt;button onclick={interval.reset}&gt;Reset&lt;/button&gt;
    </code></pre>
<img src="out_runed/utilities/use-interval/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

## <a href="https://runed.dev/docs/utilities/use-interval#callback" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Callback

You can provide an optional callback that will be called on each tick with the current counter value:

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="svelte" data-theme="github-light github-dark" tabindex="0"><code>
        &lt;script lang=&quot;ts&quot;&gt;
    import { useInterval } from &quot;runed&quot;;
 
    const interval = useInterval(1000, {
        callback: (count) =&gt; {
            console.log(`Tick number ${count}`);
        }
    });
&lt;/script&gt;
    </code></pre>
<img src="out_runed/utilities/use-interval/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

## <a href="https://runed.dev/docs/utilities/use-interval#options" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Options

The `useInterval` function accepts an optional second parameter with the following options:

- `immediate` (default: `true`) - Whether to start the interval immediately
- `immediateCallback` (default: `false`) - Whether to execute the callback immediately when resuming
- `callback` - Optional callback function that receives the current counter value on each tick

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="svelte" data-theme="github-light github-dark" tabindex="0"><code>
        &lt;script lang=&quot;ts&quot;&gt;
    import { useInterval } from &quot;runed&quot;;
 
    const interval = useInterval(1000, {
        immediate: false,
        immediateCallback: true,
        callback: (count) =&gt; console.log(count)
    });
&lt;/script&gt;
    </code></pre>
<img src="out_runed/utilities/use-interval/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

## <a href="https://runed.dev/docs/utilities/use-interval#reactive-interval" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Reactive Interval

The interval delay can be reactive, and the timer will automatically restart with the new interval when it changes:

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="svelte" data-theme="github-light github-dark" tabindex="0"><code>
        &lt;script lang=&quot;ts&quot;&gt;
    import { useInterval } from &quot;runed&quot;;
 
    let delay = $state(1000);
 
    const interval = useInterval(() =&gt; delay);
&lt;/script&gt;
 
&lt;input type=&quot;range&quot; bind:value={delay} min=&quot;100&quot; max=&quot;2000&quot; /&gt;&lt;p&gt;Delay: {delay}ms&lt;/p&gt;
    </code></pre>
<img src="out_runed/utilities/use-interval/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

## Contributors

<a href="https://github.com/TGlide" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/use-interval/index_media/4bd89e8b63f277c9d71897d289dcd951b943b3c0.jpg" id="bits-1707" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="Thomas G. Lopes" />

Thomas G. Lopes<a href="https://github.com/paoloricciuti" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/use-interval/index_media/13ed6c5dc1836b247f0850d0c06de9a0c0ce2d7f.jpg" id="bits-1709" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="Paolo Ricciuti" />

Paolo Ricciuti
