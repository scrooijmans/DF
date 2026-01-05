![](out_runed/utilities/watch/index_media/412c1b99df431530f07813ed77ff123e20b0035f.svg) Toggle Sidebar

<img src="out_runed/utilities/watch/index_media/2ef71b57cc18afcd4ed82644358ede113ed6d592.svg" class="size-5" />Search Docs ... ⌘ K

<a href="https://github.com/svecosystem/runed" class="inline-flex select-none items-center justify-center gap-2 whitespace-nowrap text-sm font-semibold tracking-wide active:scale-[99%] disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:size-4 [&amp;_svg]:shrink-0 text-foreground hover:dark:bg-foreground/5 hover:dark:ring-foreground/5 hover:bg-foreground/2 hover:ring-foreground/2 bg-transparent ring ring-transparent h-10 w-10 rounded-md [&amp;&gt;svg]:size-5" aria-label="View this project on GitHub" rel="noopener noreferrer" target="_blank"><img src="out_runed/utilities/watch/index_media/1d22f722c87a133f2381965da1acf7a54aefc45a.svg" /></a>

<img src="out_runed/utilities/watch/index_media/b2f1f2409c2d7a3eaec75643beb67d5092b71fb2.svg" class="size-6" />

# watch

Watch for changes and run a callback

Runes provide a handy way of running a callback when reactive values change: <a href="https://svelte.dev/docs/svelte/$effect" class="text-brand-link hover:text-brand-link-hover leading-7" rel="noopener noreferrer" target="_blank"><code>$effect</code></a>. It automatically detects when inner values change, and re-runs the callback.

`$effect` is great, but sometimes you want to manually specify which values should trigger the callback. Svelte provides an `untrack` function, allowing you to specify that a dependency *shouldn't* be tracked, but it doesn't provide a way to say that *only certain values* should be tracked.

`watch` does exactly that. It accepts a getter function, which returns the dependencies of the effect callback.

## <a href="https://runed.dev/docs/utilities/watch#usage" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Usage

### <a href="https://runed.dev/docs/utilities/watch#watch" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>watch

Runs a callback whenever one of the sources change.

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-theme="github-light github-dark" tabindex="0"><code>
        import { watch } from &quot;runed&quot;;
 
let count = $state(0);
watch(() =&gt; count, () =&gt; {
    console.log(count);
});
    </code></pre>
<img src="out_runed/utilities/watch/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

You can deeply watch an entire object using \$state.snapshot().

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-theme="github-light github-dark" tabindex="0"><code>
        let user = $state({ name: &#39;bob&#39;, age: 20 });
watch(() =&gt; $state.snapshot(user), () =&gt; {
    console.log(`${user.name} is ${user.age} years old`);
});
    </code></pre>
<img src="out_runed/utilities/watch/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

Or you can watch a specific deep value.

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-theme="github-light github-dark" tabindex="0"><code>
        let user = $state({ name: &#39;bob&#39;, age: 20 });
watch(() =&gt; user.age, () =&gt; {
    console.log(`User is now ${user.age} years old`);
});
    </code></pre>
<img src="out_runed/utilities/watch/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

You can also send in an array of sources.

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-theme="github-light github-dark" tabindex="0"><code>
        let age = $state(20);
let name = $state(&quot;bob&quot;);
watch([() =&gt; age, () =&gt; name], ([age, name], [prevAge, prevName]) =&gt; {
    // ...
});
    </code></pre>
<img src="out_runed/utilities/watch/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

The callback receives two arguments: The current value of the sources, and the previous value.

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-theme="github-light github-dark" tabindex="0"><code>
        let count = $state(0);
watch(() =&gt; count, (curr, prev) =&gt; {
    console.log(`count is ${curr}, was ${prev}`);
});
    </code></pre>
<img src="out_runed/utilities/watch/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

`watch` also accepts an `options` object.

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-theme="github-light github-dark" tabindex="0"><code>
        watch(sources, callback, {
    // First run will only happen after sources change when set to true.
    // By default, its false.
    lazy: true
});
    </code></pre>
<img src="out_runed/utilities/watch/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

### <a href="https://runed.dev/docs/utilities/watch#watchpre" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>watch.pre

`watch.pre` is similar to `watch`, but it uses <a href="https://svelte.dev/docs/svelte/$effect#$effect.pre" class="text-brand-link hover:text-brand-link-hover leading-7" rel="noopener noreferrer" target="_blank"><code>$effect.pre</code></a> under the hood.

### <a href="https://runed.dev/docs/utilities/watch#watchonce" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>watchOnce

In case you want to run the callback only once, you can use `watchOnce` and `watchOnce.pre`. It functions identically to the `watch` and `watch.pre` otherwise, but it does not accept any options object.

## Contributors

<a href="https://github.com/TGlide" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/watch/index_media/4bd89e8b63f277c9d71897d289dcd951b943b3c0.jpg" id="bits-371" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="Thomas G. Lopes" />

Thomas G. Lopes<a href="https://github.com/sukeshpabolu" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/watch/index_media/5ac1e8f2d1039c697836f74df59010f5f5f05535.png" id="bits-373" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="sukeshpabolu" />

sukeshpabolu<a href="https://github.com/braden-w" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/watch/index_media/feee4baa9ea13dbd2a4884eae401823fd58ee00c.jpg" id="bits-375" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="Braden Wong" />

Braden Wong<a href="https://github.com/anxpara" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/watch/index_media/9a6a631ad28fb57036969dbc3099c860db53f012.png" id="bits-377" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="anxpara" />

anxpara<a href="https://github.com/jpenilla" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/watch/index_media/81f3624b4fb97df8b4ef675ac49eac0b07bc6d24.png" id="bits-379" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="Jason Penilla" />

Jason Penilla<a href="https://github.com/huntabyte" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/watch/index_media/75bcb1d595b8d37466d561f08b890ffc8f1d5425.jpg" id="bits-381" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="Hunter Johnston" />

Hunter Johnston

© 2025 <a href="https://github.com/svecosystem" class="underline-offset-2 hover:underline" target="_blank">Svecosystem</a>

<a href="https://github.com/svecosystem/runed" class="inline-flex select-none items-center justify-center gap-2 whitespace-nowrap text-sm font-semibold tracking-wide active:scale-[99%] disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:size-4 [&amp;_svg]:shrink-0 text-foreground hover:dark:bg-foreground/5 hover:dark:ring-foreground/5 hover:bg-foreground/2 hover:ring-foreground/2 bg-transparent ring ring-transparent h-10 w-10 rounded-md [&amp;&gt;svg]:size-5" aria-label="View this project on GitHub" rel="noopener noreferrer" target="_blank"><img src="out_runed/utilities/watch/index_media/1d22f722c87a133f2381965da1acf7a54aefc45a.svg" /></a>

<img src="out_runed/utilities/watch/index_media/b2f1f2409c2d7a3eaec75643beb67d5092b71fb2.svg" class="size-6" />

# watch

Watch for changes and run a callback

Runes provide a handy way of running a callback when reactive values change: <a href="https://svelte.dev/docs/svelte/$effect" class="text-brand-link hover:text-brand-link-hover leading-7" rel="noopener noreferrer" target="_blank"><code>$effect</code></a>. It automatically detects when inner values change, and re-runs the callback.

`$effect` is great, but sometimes you want to manually specify which values should trigger the callback. Svelte provides an `untrack` function, allowing you to specify that a dependency *shouldn't* be tracked, but it doesn't provide a way to say that *only certain values* should be tracked.

`watch` does exactly that. It accepts a getter function, which returns the dependencies of the effect callback.

## <a href="https://runed.dev/docs/utilities/watch#usage" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Usage

### <a href="https://runed.dev/docs/utilities/watch#watch" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>watch

Runs a callback whenever one of the sources change.

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-theme="github-light github-dark" tabindex="0"><code>
        import { watch } from &quot;runed&quot;;
 
let count = $state(0);
watch(() =&gt; count, () =&gt; {
    console.log(count);
});
    </code></pre>
<img src="out_runed/utilities/watch/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

You can deeply watch an entire object using \$state.snapshot().

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-theme="github-light github-dark" tabindex="0"><code>
        let user = $state({ name: &#39;bob&#39;, age: 20 });
watch(() =&gt; $state.snapshot(user), () =&gt; {
    console.log(`${user.name} is ${user.age} years old`);
});
    </code></pre>
<img src="out_runed/utilities/watch/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

Or you can watch a specific deep value.

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-theme="github-light github-dark" tabindex="0"><code>
        let user = $state({ name: &#39;bob&#39;, age: 20 });
watch(() =&gt; user.age, () =&gt; {
    console.log(`User is now ${user.age} years old`);
});
    </code></pre>
<img src="out_runed/utilities/watch/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

You can also send in an array of sources.

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-theme="github-light github-dark" tabindex="0"><code>
        let age = $state(20);
let name = $state(&quot;bob&quot;);
watch([() =&gt; age, () =&gt; name], ([age, name], [prevAge, prevName]) =&gt; {
    // ...
});
    </code></pre>
<img src="out_runed/utilities/watch/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

The callback receives two arguments: The current value of the sources, and the previous value.

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-theme="github-light github-dark" tabindex="0"><code>
        let count = $state(0);
watch(() =&gt; count, (curr, prev) =&gt; {
    console.log(`count is ${curr}, was ${prev}`);
});
    </code></pre>
<img src="out_runed/utilities/watch/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

`watch` also accepts an `options` object.

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-theme="github-light github-dark" tabindex="0"><code>
        watch(sources, callback, {
    // First run will only happen after sources change when set to true.
    // By default, its false.
    lazy: true
});
    </code></pre>
<img src="out_runed/utilities/watch/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

### <a href="https://runed.dev/docs/utilities/watch#watchpre" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>watch.pre

`watch.pre` is similar to `watch`, but it uses <a href="https://svelte.dev/docs/svelte/$effect#$effect.pre" class="text-brand-link hover:text-brand-link-hover leading-7" rel="noopener noreferrer" target="_blank"><code>$effect.pre</code></a> under the hood.

### <a href="https://runed.dev/docs/utilities/watch#watchonce" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>watchOnce

In case you want to run the callback only once, you can use `watchOnce` and `watchOnce.pre`. It functions identically to the `watch` and `watch.pre` otherwise, but it does not accept any options object.

## Contributors

<a href="https://github.com/TGlide" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/watch/index_media/4bd89e8b63f277c9d71897d289dcd951b943b3c0.jpg" id="bits-371" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="Thomas G. Lopes" />

Thomas G. Lopes<a href="https://github.com/sukeshpabolu" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/watch/index_media/5ac1e8f2d1039c697836f74df59010f5f5f05535.png" id="bits-373" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="sukeshpabolu" />

sukeshpabolu<a href="https://github.com/braden-w" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/watch/index_media/feee4baa9ea13dbd2a4884eae401823fd58ee00c.jpg" id="bits-375" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="Braden Wong" />

Braden Wong<a href="https://github.com/anxpara" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/watch/index_media/9a6a631ad28fb57036969dbc3099c860db53f012.png" id="bits-377" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="anxpara" />

anxpara<a href="https://github.com/jpenilla" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/watch/index_media/81f3624b4fb97df8b4ef675ac49eac0b07bc6d24.png" id="bits-379" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="Jason Penilla" />

Jason Penilla<a href="https://github.com/huntabyte" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/watch/index_media/75bcb1d595b8d37466d561f08b890ffc8f1d5425.jpg" id="bits-381" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="Hunter Johnston" />

Hunter Johnston
