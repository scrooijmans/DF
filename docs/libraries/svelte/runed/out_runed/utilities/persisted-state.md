![](out_runed/utilities/persisted-state/index_media/412c1b99df431530f07813ed77ff123e20b0035f.svg) Toggle Sidebar

<img src="out_runed/utilities/persisted-state/index_media/2ef71b57cc18afcd4ed82644358ede113ed6d592.svg" class="size-5" />Search Docs ... ⌘ K

<a href="https://github.com/svecosystem/runed" class="inline-flex select-none items-center justify-center gap-2 whitespace-nowrap text-sm font-semibold tracking-wide active:scale-[99%] disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:size-4 [&amp;_svg]:shrink-0 text-foreground hover:dark:bg-foreground/5 hover:dark:ring-foreground/5 hover:bg-foreground/2 hover:ring-foreground/2 bg-transparent ring ring-transparent h-10 w-10 rounded-md [&amp;&gt;svg]:size-5" aria-label="View this project on GitHub" rel="noopener noreferrer" target="_blank"><img src="out_runed/utilities/persisted-state/index_media/1d22f722c87a133f2381965da1acf7a54aefc45a.svg" /></a>

<img src="out_runed/utilities/persisted-state/index_media/b2f1f2409c2d7a3eaec75643beb67d5092b71fb2.svg" class="size-6" />

# PersistedState

A reactive state manager that persists and synchronizes state across browser sessions and tabs using Web Storage APIs.

`PersistedState` provides a reactive state container that automatically persists data to browser storage and optionally synchronizes changes across browser tabs in real-time.

## <a href="https://runed.dev/docs/utilities/persisted-state#demo" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Demo

Increment

Decrement

Reset

``` bg-transparent
Count: 0
```

<img src="out_runed/utilities/persisted-state/index_media/a08f0efc16d25c94f055dec10bc7c24422bffe7b.svg" class="size-6" />

##### Note

You can refresh this page and/or open it in another tab to see the count state being persisted and synchronized across sessions and tabs.

## <a href="https://runed.dev/docs/utilities/persisted-state#usage" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Usage

Initialize `PersistedState` by providing a unique key and an initial value for the state.

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="svelte" data-theme="github-light github-dark" tabindex="0"><code>
        &lt;script lang=&quot;ts&quot;&gt;
    import { PersistedState } from &quot;runed&quot;;
 
    const count = new PersistedState(&quot;count&quot;, 0);
&lt;/script&gt;
 
&lt;div&gt;
    &lt;button onclick={() =&gt; count.current++}&gt;Increment&lt;/button&gt;
    &lt;button onclick={() =&gt; count.current--}&gt;Decrement&lt;/button&gt;
    &lt;button onclick={() =&gt; (count.current = 0)}&gt;Reset&lt;/button&gt;
    &lt;p&gt;Count: {count.current}&lt;/p&gt;
&lt;/div&gt;
    </code></pre>
<img src="out_runed/utilities/persisted-state/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

### <a href="https://runed.dev/docs/utilities/persisted-state#complex-objects" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Complex objects

When persisting complex objects, only plain structures are deeply reactive.

This includes arrays, plain objects, and primitive values.

For example:

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-theme="github-light github-dark" tabindex="0"><code>
        const persistedArray = new PersistedState(&quot;foo&quot;, [&quot;a&quot;, &quot;b&quot;]);
persistedArray.current.push(&quot;c&quot;); // This will persist the change
 
const persistedObject = new PersistedState(&quot;bar&quot;, { name: &quot;Bob&quot; });
persistedObject.current.name = &quot;JG&quot;; // This will persist the change
 
class Person {
    name: string;
    constructor(name: string) {
        this.name = name;
    }
}
const persistedComplexObject = new PersistedState(&quot;baz&quot;, new Person(&quot;Bob&quot;));
persistedComplexObject.current.name = &quot;JG&quot;; // This will NOT persist the change
persistedComplexObject.current = new Person(&quot;JG&quot;); // This will persist the change
    </code></pre>
<img src="out_runed/utilities/persisted-state/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

## <a href="https://runed.dev/docs/utilities/persisted-state#configuration-options" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Configuration Options

`PersistedState` includes an `options` object that allows you to customize the behavior of the state manager.

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-theme="github-light github-dark" tabindex="0"><code>
        const state = new PersistedState(&quot;user-preferences&quot;, initialValue, {
    // Use sessionStorage instead of localStorage (default: &#39;local&#39;)
    storage: &quot;session&quot;,
 
    // Disable cross-tab synchronization (default: true)
    syncTabs: false,
 
    // Start disconnected from storage (default: true)
    connected: false,
 
    // Custom serialization handlers
    serializer: {
        serialize: superjson.stringify,
        deserialize: superjson.parse
    }
});
    </code></pre>
<img src="out_runed/utilities/persisted-state/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

### <a href="https://runed.dev/docs/utilities/persisted-state#storage-options" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Storage Options

- `'local'`: Data persists until explicitly cleared
- `'session'`: Data persists until the browser session ends

### <a href="https://runed.dev/docs/utilities/persisted-state#cross-tab-synchronization" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Cross-Tab Synchronization

When `syncTabs` is enabled (default), changes are automatically synchronized across all browser tabs using the storage event.

### <a href="https://runed.dev/docs/utilities/persisted-state#connection-control" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Connection Control

By default, the state is connected to storage on initialization and any changes to the state will persist to storage and reads from the state will be read from storage.

For more control, you can control when the state connects to storage using the `connected` option and/or the `.connect()` and `.disconnect()` methods:

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-theme="github-light github-dark" tabindex="0"><code>
        // Start disconnected from storage
const state = new PersistedState(&quot;temp-data&quot;, initialValue, {
    connected: false
});
 
// State changes are kept in memory only
state.current = &quot;new value&quot;;
 
// Connect to storage when ready
state.connect(); // Now persists to storage
 
// Check connection status
console.log(state.connected); // true
 
// Disconnect from storage
state.disconnect(); // Removes from storage, keeps value in memory
    </code></pre>
<img src="out_runed/utilities/persisted-state/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

When disconnected:

- State changes are kept in memory only
- Storage changes are not reflected in the state
- Cross-tab synchronization is disabled

Calling `disconnect()` removes the current value from storage but preserves it in memory. Calling `connect()` immediately persists the current in-memory value to storage.

### <a href="https://runed.dev/docs/utilities/persisted-state#custom-serialization" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Custom Serialization

Provide custom `serialize` and `deserialize` functions to handle complex data types:

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-theme="github-light github-dark" tabindex="0"><code>
        import superjson from &quot;superjson&quot;;
 
// Example with Date objects
const lastAccessed = new PersistedState(&quot;last-accessed&quot;, new Date(), {
    serializer: {
        serialize: superjson.stringify,
        deserialize: superjson.parse
    }
});
    </code></pre>
<img src="out_runed/utilities/persisted-state/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

## Contributors

<a href="https://github.com/huntabyte" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/persisted-state/index_media/75bcb1d595b8d37466d561f08b890ffc8f1d5425.jpg" id="bits-619" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="Hunter Johnston" />

Hunter Johnston<a href="https://github.com/43081j" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/persisted-state/index_media/06f4edda8a9beea51070ce54837509dcdbea04f6.png" id="bits-621" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="James Garbutt" />

James Garbutt<a href="https://github.com/eBrainMachine" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/persisted-state/index_media/aede7ef091c7d148974bd8ff42a3c66c76e412cd.png" id="bits-623" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="eBrainMachine" />

eBrainMachine<a href="https://github.com/paoloricciuti" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/persisted-state/index_media/13ed6c5dc1836b247f0850d0c06de9a0c0ce2d7f.jpg" id="bits-625" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="Paolo Ricciuti" />

Paolo Ricciuti<a href="https://github.com/aster-void" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/persisted-state/index_media/45c61944f271a6e4ae359c2590b218b189fe70d6.jpg" id="bits-627" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="Yuki Kobayashi" />

Yuki Kobayashi<a href="https://github.com/ieedan" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/persisted-state/index_media/2324acafb6b6eb6603364d7ea4f1927b0694b63f.jpg" id="bits-629" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="Aidan Bleser" />

Aidan Bleser<a href="https://github.com/gyszalai" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/persisted-state/index_media/2938a9ae343804561ceb21762a4400b31d6dcd48.png" id="bits-631" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="Gyula Szalai" />

Gyula Szalai<a href="https://github.com/Not-Jayden" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/persisted-state/index_media/b0fb45ada1c1e0d6d338655b9bb4932aea0b1d6e.png" id="bits-633" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="Jayden Carey" />

Jayden Carey

© 2025 <a href="https://github.com/svecosystem" class="underline-offset-2 hover:underline" target="_blank">Svecosystem</a>

<a href="https://github.com/svecosystem/runed" class="inline-flex select-none items-center justify-center gap-2 whitespace-nowrap text-sm font-semibold tracking-wide active:scale-[99%] disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:size-4 [&amp;_svg]:shrink-0 text-foreground hover:dark:bg-foreground/5 hover:dark:ring-foreground/5 hover:bg-foreground/2 hover:ring-foreground/2 bg-transparent ring ring-transparent h-10 w-10 rounded-md [&amp;&gt;svg]:size-5" aria-label="View this project on GitHub" rel="noopener noreferrer" target="_blank"><img src="out_runed/utilities/persisted-state/index_media/1d22f722c87a133f2381965da1acf7a54aefc45a.svg" /></a>

<img src="out_runed/utilities/persisted-state/index_media/b2f1f2409c2d7a3eaec75643beb67d5092b71fb2.svg" class="size-6" />

# PersistedState

A reactive state manager that persists and synchronizes state across browser sessions and tabs using Web Storage APIs.

`PersistedState` provides a reactive state container that automatically persists data to browser storage and optionally synchronizes changes across browser tabs in real-time.

## <a href="https://runed.dev/docs/utilities/persisted-state#demo" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Demo

Increment

Decrement

Reset

``` bg-transparent
Count: 0
```

<img src="out_runed/utilities/persisted-state/index_media/a08f0efc16d25c94f055dec10bc7c24422bffe7b.svg" class="size-6" />

##### Note

You can refresh this page and/or open it in another tab to see the count state being persisted and synchronized across sessions and tabs.

## <a href="https://runed.dev/docs/utilities/persisted-state#usage" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Usage

Initialize `PersistedState` by providing a unique key and an initial value for the state.

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="svelte" data-theme="github-light github-dark" tabindex="0"><code>
        &lt;script lang=&quot;ts&quot;&gt;
    import { PersistedState } from &quot;runed&quot;;
 
    const count = new PersistedState(&quot;count&quot;, 0);
&lt;/script&gt;
 
&lt;div&gt;
    &lt;button onclick={() =&gt; count.current++}&gt;Increment&lt;/button&gt;
    &lt;button onclick={() =&gt; count.current--}&gt;Decrement&lt;/button&gt;
    &lt;button onclick={() =&gt; (count.current = 0)}&gt;Reset&lt;/button&gt;
    &lt;p&gt;Count: {count.current}&lt;/p&gt;
&lt;/div&gt;
    </code></pre>
<img src="out_runed/utilities/persisted-state/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

### <a href="https://runed.dev/docs/utilities/persisted-state#complex-objects" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Complex objects

When persisting complex objects, only plain structures are deeply reactive.

This includes arrays, plain objects, and primitive values.

For example:

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-theme="github-light github-dark" tabindex="0"><code>
        const persistedArray = new PersistedState(&quot;foo&quot;, [&quot;a&quot;, &quot;b&quot;]);
persistedArray.current.push(&quot;c&quot;); // This will persist the change
 
const persistedObject = new PersistedState(&quot;bar&quot;, { name: &quot;Bob&quot; });
persistedObject.current.name = &quot;JG&quot;; // This will persist the change
 
class Person {
    name: string;
    constructor(name: string) {
        this.name = name;
    }
}
const persistedComplexObject = new PersistedState(&quot;baz&quot;, new Person(&quot;Bob&quot;));
persistedComplexObject.current.name = &quot;JG&quot;; // This will NOT persist the change
persistedComplexObject.current = new Person(&quot;JG&quot;); // This will persist the change
    </code></pre>
<img src="out_runed/utilities/persisted-state/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

## <a href="https://runed.dev/docs/utilities/persisted-state#configuration-options" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Configuration Options

`PersistedState` includes an `options` object that allows you to customize the behavior of the state manager.

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-theme="github-light github-dark" tabindex="0"><code>
        const state = new PersistedState(&quot;user-preferences&quot;, initialValue, {
    // Use sessionStorage instead of localStorage (default: &#39;local&#39;)
    storage: &quot;session&quot;,
 
    // Disable cross-tab synchronization (default: true)
    syncTabs: false,
 
    // Start disconnected from storage (default: true)
    connected: false,
 
    // Custom serialization handlers
    serializer: {
        serialize: superjson.stringify,
        deserialize: superjson.parse
    }
});
    </code></pre>
<img src="out_runed/utilities/persisted-state/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

### <a href="https://runed.dev/docs/utilities/persisted-state#storage-options" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Storage Options

- `'local'`: Data persists until explicitly cleared
- `'session'`: Data persists until the browser session ends

### <a href="https://runed.dev/docs/utilities/persisted-state#cross-tab-synchronization" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Cross-Tab Synchronization

When `syncTabs` is enabled (default), changes are automatically synchronized across all browser tabs using the storage event.

### <a href="https://runed.dev/docs/utilities/persisted-state#connection-control" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Connection Control

By default, the state is connected to storage on initialization and any changes to the state will persist to storage and reads from the state will be read from storage.

For more control, you can control when the state connects to storage using the `connected` option and/or the `.connect()` and `.disconnect()` methods:

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-theme="github-light github-dark" tabindex="0"><code>
        // Start disconnected from storage
const state = new PersistedState(&quot;temp-data&quot;, initialValue, {
    connected: false
});
 
// State changes are kept in memory only
state.current = &quot;new value&quot;;
 
// Connect to storage when ready
state.connect(); // Now persists to storage
 
// Check connection status
console.log(state.connected); // true
 
// Disconnect from storage
state.disconnect(); // Removes from storage, keeps value in memory
    </code></pre>
<img src="out_runed/utilities/persisted-state/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

When disconnected:

- State changes are kept in memory only
- Storage changes are not reflected in the state
- Cross-tab synchronization is disabled

Calling `disconnect()` removes the current value from storage but preserves it in memory. Calling `connect()` immediately persists the current in-memory value to storage.

### <a href="https://runed.dev/docs/utilities/persisted-state#custom-serialization" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Custom Serialization

Provide custom `serialize` and `deserialize` functions to handle complex data types:

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-theme="github-light github-dark" tabindex="0"><code>
        import superjson from &quot;superjson&quot;;
 
// Example with Date objects
const lastAccessed = new PersistedState(&quot;last-accessed&quot;, new Date(), {
    serializer: {
        serialize: superjson.stringify,
        deserialize: superjson.parse
    }
});
    </code></pre>
<img src="out_runed/utilities/persisted-state/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

## Contributors

<a href="https://github.com/huntabyte" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/persisted-state/index_media/75bcb1d595b8d37466d561f08b890ffc8f1d5425.jpg" id="bits-619" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="Hunter Johnston" />

Hunter Johnston<a href="https://github.com/43081j" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/persisted-state/index_media/06f4edda8a9beea51070ce54837509dcdbea04f6.png" id="bits-621" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="James Garbutt" />

James Garbutt<a href="https://github.com/eBrainMachine" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/persisted-state/index_media/aede7ef091c7d148974bd8ff42a3c66c76e412cd.png" id="bits-623" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="eBrainMachine" />

eBrainMachine<a href="https://github.com/paoloricciuti" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/persisted-state/index_media/13ed6c5dc1836b247f0850d0c06de9a0c0ce2d7f.jpg" id="bits-625" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="Paolo Ricciuti" />

Paolo Ricciuti<a href="https://github.com/aster-void" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/persisted-state/index_media/45c61944f271a6e4ae359c2590b218b189fe70d6.jpg" id="bits-627" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="Yuki Kobayashi" />

Yuki Kobayashi<a href="https://github.com/ieedan" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/persisted-state/index_media/2324acafb6b6eb6603364d7ea4f1927b0694b63f.jpg" id="bits-629" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="Aidan Bleser" />

Aidan Bleser<a href="https://github.com/gyszalai" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/persisted-state/index_media/2938a9ae343804561ceb21762a4400b31d6dcd48.png" id="bits-631" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="Gyula Szalai" />

Gyula Szalai<a href="https://github.com/Not-Jayden" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/persisted-state/index_media/b0fb45ada1c1e0d6d338655b9bb4932aea0b1d6e.png" id="bits-633" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="Jayden Carey" />

Jayden Carey
