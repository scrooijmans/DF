![](out_runed/utilities/state-history/index_media/412c1b99df431530f07813ed77ff123e20b0035f.svg) Toggle Sidebar

<img src="out_runed/utilities/state-history/index_media/2ef71b57cc18afcd4ed82644358ede113ed6d592.svg" class="size-5" />Search Docs ... ⌘ K

<a href="https://github.com/svecosystem/runed" class="inline-flex select-none items-center justify-center gap-2 whitespace-nowrap text-sm font-semibold tracking-wide active:scale-[99%] disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:size-4 [&amp;_svg]:shrink-0 text-foreground hover:dark:bg-foreground/5 hover:dark:ring-foreground/5 hover:bg-foreground/2 hover:ring-foreground/2 bg-transparent ring ring-transparent h-10 w-10 rounded-md [&amp;&gt;svg]:size-5" aria-label="View this project on GitHub" rel="noopener noreferrer" target="_blank"><img src="out_runed/utilities/state-history/index_media/1d22f722c87a133f2381965da1acf7a54aefc45a.svg" /></a>

<img src="out_runed/utilities/state-history/index_media/b2f1f2409c2d7a3eaec75643beb67d5092b71fb2.svg" class="size-6" />

# StateHistory

Track state changes with undo/redo capabilities

## <a href="https://runed.dev/docs/utilities/state-history#demo" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Demo

Count: 0

Increment

Decrement

/

Undo

Redo

History (limited to 10 records for demo)

## <a href="https://runed.dev/docs/utilities/state-history#usage" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Usage

`StateHistory` tracks a getter's return value, logging each change into an array. A setter is also required to use the `undo` and `redo` functions.

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-theme="github-light github-dark" tabindex="0"><code>
        import { StateHistory } from &quot;runed&quot;;
 
let count = $state(0);
const history = new StateHistory(() =&gt; count, (c) =&gt; (count = c));
history.log[0]; // { snapshot: 0, timestamp: ... }
    </code></pre>
<img src="out_runed/utilities/state-history/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

Besides `log`, the returned object contains `undo`, `redo`, and `clear` functionality.

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="svelte" data-theme="github-light github-dark" tabindex="0"><code>
        &lt;script lang=&quot;ts&quot;&gt;
    import { StateHistory } from &quot;runed&quot;;
 
    let count = $state(0);
    const history = new StateHistory(() =&gt; count, (c) =&gt; (count = c));
&lt;/script&gt;
 
&lt;p&gt;{count}&lt;/p&gt;
 
&lt;button onclick={() =&gt; count++}&gt;Increment&lt;/button&gt;
&lt;button onclick={() =&gt; count--}&gt;Decrement&lt;/button&gt;
 
&lt;button disabled={!history.canUndo} onclick={history.undo}&gt;Undo&lt;/button&gt;
&lt;button disabled={!history.canRedo} onclick={history.redo}&gt;Redo&lt;/button&gt;
&lt;button onclick={history.clear}&gt;Clear History&lt;/button&gt;
    </code></pre>
<img src="out_runed/utilities/state-history/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

## <a href="https://runed.dev/docs/utilities/state-history#methods" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Methods

### <a href="https://runed.dev/docs/utilities/state-history#undo" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>`undo()`

Reverts the state to the previous value in the history log. The current state is moved to the redo stack.

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-theme="github-light github-dark" tabindex="0"><code>
        let count = $state(0);
const history = new StateHistory(() =&gt; count, (c) =&gt; (count = c));
 
count = 1;
count = 2;
 
history.undo(); // count is now 1
history.undo(); // count is now 0
    </code></pre>
<img src="out_runed/utilities/state-history/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

### <a href="https://runed.dev/docs/utilities/state-history#redo" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>`redo()`

Restores a previously undone state. Moves the next state from the redo stack back to the history log.

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-theme="github-light github-dark" tabindex="0"><code>
        let count = $state(0);
const history = new StateHistory(() =&gt; count, (c) =&gt; (count = c));
 
count = 1;
count = 2;
 
history.undo(); // count is now 1
history.redo();  // count is now 2
    </code></pre>
<img src="out_runed/utilities/state-history/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

### <a href="https://runed.dev/docs/utilities/state-history#clear" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>`clear()`

Clears the entire history log and redo stack, effectively resetting the state history.

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-theme="github-light github-dark" tabindex="0"><code>
        history.clear();
console.log(history.log); // []
console.log(history.canUndo); // false
console.log(history.canRedo); // false
    </code></pre>
<img src="out_runed/utilities/state-history/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

## <a href="https://runed.dev/docs/utilities/state-history#properties" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Properties

### <a href="https://runed.dev/docs/utilities/state-history#log" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>`log`

An array of `LogEvent<T>` objects containing the state history. Each event has a `snapshot` of the state and a `timestamp`.

### <a href="https://runed.dev/docs/utilities/state-history#canundo" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>`canUndo`

A derived boolean indicating whether undo is possible (true when there's more than one item in the log).

### <a href="https://runed.dev/docs/utilities/state-history#canredo" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>`canRedo`

A derived boolean indicating whether redo is possible (true when the redo stack is not empty).

## Contributors

<a href="https://github.com/huntabyte" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/state-history/index_media/75bcb1d595b8d37466d561f08b890ffc8f1d5425.jpg" id="bits-735" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="Hunter Johnston" />

Hunter Johnston<a href="https://github.com/cristianvogel" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/state-history/index_media/96c7e6dab770aa7cfe360ee8ebcc38f802c6dc15.png" id="bits-737" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="NeverEngineLabs" />

NeverEngineLabs<a href="https://github.com/sacrosanctic" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/state-history/index_media/212aa5a14fda51defb0e520219102701ccd34fa0.jpg" id="bits-739" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="Scott Wu" />

Scott Wu<a href="https://github.com/TGlide" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/state-history/index_media/4bd89e8b63f277c9d71897d289dcd951b943b3c0.jpg" id="bits-741" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="Thomas G. Lopes" />

Thomas G. Lopes<a href="https://github.com/43081j" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/state-history/index_media/06f4edda8a9beea51070ce54837509dcdbea04f6.png" id="bits-743" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="James Garbutt" />

James Garbutt<a href="https://github.com/phocks" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/state-history/index_media/4e32f1eb802665fa2e171be36f7bfe1bb8c9fcb8.jpg" id="bits-745" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="Joshua Byrd" />

Joshua Byrd

© 2025 <a href="https://github.com/svecosystem" class="underline-offset-2 hover:underline" target="_blank">Svecosystem</a>

<a href="https://github.com/svecosystem/runed" class="inline-flex select-none items-center justify-center gap-2 whitespace-nowrap text-sm font-semibold tracking-wide active:scale-[99%] disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:size-4 [&amp;_svg]:shrink-0 text-foreground hover:dark:bg-foreground/5 hover:dark:ring-foreground/5 hover:bg-foreground/2 hover:ring-foreground/2 bg-transparent ring ring-transparent h-10 w-10 rounded-md [&amp;&gt;svg]:size-5" aria-label="View this project on GitHub" rel="noopener noreferrer" target="_blank"><img src="out_runed/utilities/state-history/index_media/1d22f722c87a133f2381965da1acf7a54aefc45a.svg" /></a>

<img src="out_runed/utilities/state-history/index_media/b2f1f2409c2d7a3eaec75643beb67d5092b71fb2.svg" class="size-6" />

# StateHistory

Track state changes with undo/redo capabilities

## <a href="https://runed.dev/docs/utilities/state-history#demo" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Demo

Count: 0

Increment

Decrement

/

Undo

Redo

History (limited to 10 records for demo)

## <a href="https://runed.dev/docs/utilities/state-history#usage" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Usage

`StateHistory` tracks a getter's return value, logging each change into an array. A setter is also required to use the `undo` and `redo` functions.

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-theme="github-light github-dark" tabindex="0"><code>
        import { StateHistory } from &quot;runed&quot;;
 
let count = $state(0);
const history = new StateHistory(() =&gt; count, (c) =&gt; (count = c));
history.log[0]; // { snapshot: 0, timestamp: ... }
    </code></pre>
<img src="out_runed/utilities/state-history/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

Besides `log`, the returned object contains `undo`, `redo`, and `clear` functionality.

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="svelte" data-theme="github-light github-dark" tabindex="0"><code>
        &lt;script lang=&quot;ts&quot;&gt;
    import { StateHistory } from &quot;runed&quot;;
 
    let count = $state(0);
    const history = new StateHistory(() =&gt; count, (c) =&gt; (count = c));
&lt;/script&gt;
 
&lt;p&gt;{count}&lt;/p&gt;
 
&lt;button onclick={() =&gt; count++}&gt;Increment&lt;/button&gt;
&lt;button onclick={() =&gt; count--}&gt;Decrement&lt;/button&gt;
 
&lt;button disabled={!history.canUndo} onclick={history.undo}&gt;Undo&lt;/button&gt;
&lt;button disabled={!history.canRedo} onclick={history.redo}&gt;Redo&lt;/button&gt;
&lt;button onclick={history.clear}&gt;Clear History&lt;/button&gt;
    </code></pre>
<img src="out_runed/utilities/state-history/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

## <a href="https://runed.dev/docs/utilities/state-history#methods" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Methods

### <a href="https://runed.dev/docs/utilities/state-history#undo" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>`undo()`

Reverts the state to the previous value in the history log. The current state is moved to the redo stack.

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-theme="github-light github-dark" tabindex="0"><code>
        let count = $state(0);
const history = new StateHistory(() =&gt; count, (c) =&gt; (count = c));
 
count = 1;
count = 2;
 
history.undo(); // count is now 1
history.undo(); // count is now 0
    </code></pre>
<img src="out_runed/utilities/state-history/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

### <a href="https://runed.dev/docs/utilities/state-history#redo" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>`redo()`

Restores a previously undone state. Moves the next state from the redo stack back to the history log.

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-theme="github-light github-dark" tabindex="0"><code>
        let count = $state(0);
const history = new StateHistory(() =&gt; count, (c) =&gt; (count = c));
 
count = 1;
count = 2;
 
history.undo(); // count is now 1
history.redo();  // count is now 2
    </code></pre>
<img src="out_runed/utilities/state-history/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

### <a href="https://runed.dev/docs/utilities/state-history#clear" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>`clear()`

Clears the entire history log and redo stack, effectively resetting the state history.

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-theme="github-light github-dark" tabindex="0"><code>
        history.clear();
console.log(history.log); // []
console.log(history.canUndo); // false
console.log(history.canRedo); // false
    </code></pre>
<img src="out_runed/utilities/state-history/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

## <a href="https://runed.dev/docs/utilities/state-history#properties" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Properties

### <a href="https://runed.dev/docs/utilities/state-history#log" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>`log`

An array of `LogEvent<T>` objects containing the state history. Each event has a `snapshot` of the state and a `timestamp`.

### <a href="https://runed.dev/docs/utilities/state-history#canundo" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>`canUndo`

A derived boolean indicating whether undo is possible (true when there's more than one item in the log).

### <a href="https://runed.dev/docs/utilities/state-history#canredo" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>`canRedo`

A derived boolean indicating whether redo is possible (true when the redo stack is not empty).

## Contributors

<a href="https://github.com/huntabyte" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/state-history/index_media/75bcb1d595b8d37466d561f08b890ffc8f1d5425.jpg" id="bits-735" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="Hunter Johnston" />

Hunter Johnston<a href="https://github.com/cristianvogel" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/state-history/index_media/96c7e6dab770aa7cfe360ee8ebcc38f802c6dc15.png" id="bits-737" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="NeverEngineLabs" />

NeverEngineLabs<a href="https://github.com/sacrosanctic" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/state-history/index_media/212aa5a14fda51defb0e520219102701ccd34fa0.jpg" id="bits-739" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="Scott Wu" />

Scott Wu<a href="https://github.com/TGlide" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/state-history/index_media/4bd89e8b63f277c9d71897d289dcd951b943b3c0.jpg" id="bits-741" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="Thomas G. Lopes" />

Thomas G. Lopes<a href="https://github.com/43081j" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/state-history/index_media/06f4edda8a9beea51070ce54837509dcdbea04f6.png" id="bits-743" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="James Garbutt" />

James Garbutt<a href="https://github.com/phocks" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/state-history/index_media/4e32f1eb802665fa2e171be36f7bfe1bb8c9fcb8.jpg" id="bits-745" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="Joshua Byrd" />

Joshua Byrd
