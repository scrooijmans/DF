![](out_runed/utilities/resource/index_media/412c1b99df431530f07813ed77ff123e20b0035f.svg) Toggle Sidebar

<img src="out_runed/utilities/resource/index_media/2ef71b57cc18afcd4ed82644358ede113ed6d592.svg" class="size-5" />Search Docs ... ⌘ K

<a href="https://github.com/svecosystem/runed" class="inline-flex select-none items-center justify-center gap-2 whitespace-nowrap text-sm font-semibold tracking-wide active:scale-[99%] disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:size-4 [&amp;_svg]:shrink-0 text-foreground hover:dark:bg-foreground/5 hover:dark:ring-foreground/5 hover:bg-foreground/2 hover:ring-foreground/2 bg-transparent ring ring-transparent h-10 w-10 rounded-md [&amp;&gt;svg]:size-5" aria-label="View this project on GitHub" rel="noopener noreferrer" target="_blank"><img src="out_runed/utilities/resource/index_media/1d22f722c87a133f2381965da1acf7a54aefc45a.svg" /></a>

<img src="out_runed/utilities/resource/index_media/b2f1f2409c2d7a3eaec75643beb67d5092b71fb2.svg" class="size-6" />

# resource

Watch for changes and runs async data fetching

In SvelteKit, using load functions is the primary approach for data fetching. While you can handle reactive data fetching by using `URLSearchParams` for query parameters, there are cases where you might need more flexibility at the component level.

This is where `resource` comes in - it's a utility that seamlessly combines reactive state management with async data fetching.

Built on top of `watch`, it runs after rendering by default, but also provides a pre-render option via `resource.pre()`.

## <a href="https://runed.dev/docs/utilities/resource#demo" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Demo

Post id:

Status: Ready

Refetch Results

## <a href="https://runed.dev/docs/utilities/resource#usage" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Usage

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="svelte" data-theme="github-light github-dark" tabindex="0"><code>
        &lt;script lang=&quot;ts&quot;&gt;
    import { resource } from &quot;runed&quot;;
 
    let id = $state(1);
 
    const searchResource = resource(
        () =&gt; id,
        async (id, prevId, { data, refetching, onCleanup, signal }) =&gt; {
            // data: the previous value returned from the fetcher
 
            // refetching: whether the fetcher is currently refetching
            // or it can be the value you passed to refetch()
 
            // onCleanup: A cleanup function that will be called when the source is invalidated
            // and the fetcher is about to re-run
 
            // signal: AbortSignal for cancelling fetch requests
            const response = await fetch(`api/posts?id=${id}`, { signal });
            return response.json();
        },
        {
            debounce: 300
            // lazy: Skip initial fetch when true
            // once: Only fetch once when true
            // initialValue: Provides an initial value for the resource
            // debounce: Debounce rapid changes
            // throttle: Throttle rapid changes
        }
    );
 
    // The current value of the resource
    searchResource.current;
    // Whether the resource is currently loading
    searchResource.loading;
    // Error if the fetch failed
    searchResource.error;
    // Update the resource value directly, useful for optimistic updates
    searchResource.mutate();
    // Re-run the fetcher with current watching values
    searchResource.refetch();
&lt;/script&gt;
 
&lt;input type=&quot;number&quot; bind:value={id} /&gt;
 
{#if searchResults.loading}
    &lt;div&gt;Loading...&lt;/div&gt;
{:else if searchResults.error}
    &lt;div&gt;Error: {searchResults.error.message}&lt;/div&gt;
{:else}
    &lt;ul&gt;
        {#each searchResults.current ?? [] as result}
            &lt;li&gt;{result.title}&lt;/li&gt;
        {/each}
    &lt;/ul&gt;
{/if}
    </code></pre>
<img src="out_runed/utilities/resource/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

## <a href="https://runed.dev/docs/utilities/resource#features" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Features

- **Automatic Request Cancellation**: When dependencies change, in-flight requests are automatically canceled
- **Loading & Error States**: Built-in states for loading and error handling
- **Debouncing & Throttling**: Optional debounce and throttle options for rate limiting
- **Type Safety**: Full TypeScript support with inferred types
- **Multiple Dependencies**: Support for tracking multiple reactive dependencies
- **Custom Cleanup**: Register cleanup functions that run before refetching
- **Pre-render Support**: `resource.pre()` for pre-render execution

## <a href="https://runed.dev/docs/utilities/resource#advanced-usage" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Advanced Usage

### <a href="https://runed.dev/docs/utilities/resource#multiple-dependencies" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Multiple Dependencies

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="svelte" data-theme="github-light github-dark" tabindex="0"><code>
        &lt;script lang=&quot;ts&quot;&gt;
    const results = resource([() =&gt; query, () =&gt; page], async ([query, page]) =&gt; {
        const res = await fetch(`/api/search?q=${query}&amp;page=${page}`);
        return res.json();
    });
&lt;/script&gt;
    </code></pre>
<img src="out_runed/utilities/resource/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

### <a href="https://runed.dev/docs/utilities/resource#custom-cleanup" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Custom Cleanup

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="svelte" data-theme="github-light github-dark" tabindex="0"><code>
        &lt;script lang=&quot;ts&quot;&gt;
    const stream = resource(
        () =&gt; streamId,
        async (id, _, { signal, onCleanup }) =&gt; {
            const eventSource = new EventSource(`/api/stream/${id}`);
            onCleanup(() =&gt; eventSource.close());
 
            const res = await fetch(`/api/stream/${id}/init`, { signal });
            return res.json();
        }
    );
&lt;/script&gt;
    </code></pre>
<img src="out_runed/utilities/resource/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

### <a href="https://runed.dev/docs/utilities/resource#pre-render-execution" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Pre-render Execution

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="svelte" data-theme="github-light github-dark" tabindex="0"><code>
        &lt;script lang=&quot;ts&quot;&gt;
    const data = resource.pre(
        () =&gt; query,
        async (query) =&gt; {
            const res = await fetch(`/api/search?q=${query}`);
            return res.json();
        }
    );
&lt;/script&gt;
    </code></pre>
<img src="out_runed/utilities/resource/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

## <a href="https://runed.dev/docs/utilities/resource#configuration-options" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Configuration Options

### <a href="https://runed.dev/docs/utilities/resource#lazy" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>`lazy`

When true, skips the initial fetch. The resource will only fetch when dependencies change or `refetch()` is called.

### <a href="https://runed.dev/docs/utilities/resource#once" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>`once`

When true, only fetches once. Subsequent dependency changes won't trigger new fetches.

### <a href="https://runed.dev/docs/utilities/resource#initialvalue" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>`initialValue`

Provides an initial value for the resource before the first fetch completes. Useful if you already have the data and don't want to wait for the fetch to complete.

### <a href="https://runed.dev/docs/utilities/resource#debounce" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>`debounce`

Time in milliseconds to debounce rapid changes. Useful for search inputs.

The debounce implementation will cancel pending requests and only execute the last one after the specified delay.

### <a href="https://runed.dev/docs/utilities/resource#throttle" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>`throttle`

Time in milliseconds to throttle rapid changes. Useful for real-time updates.

The throttle implementation will ensure that requests are spaced at least by the specified delay, returning the pending promise if called too soon.

<img src="out_runed/utilities/resource/index_media/a08f0efc16d25c94f055dec10bc7c24422bffe7b.svg" class="size-6" />

##### Note

Note that you should use either debounce or throttle, not both - if both are specified, debounce takes precedence.

## <a href="https://runed.dev/docs/utilities/resource#type-definitions" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Type Definitions

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-theme="github-light github-dark" tabindex="0"><code>
        type ResourceOptions&lt;Data&gt; = {
    /** Skip initial fetch when true */
    lazy?: boolean;
    /** Only fetch once when true */
    once?: boolean;
    /** Initial value for the resource */
    initialValue?: Data;
    /** Debounce time in milliseconds */
    debounce?: number;
    /** Throttle time in milliseconds */
    throttle?: number;
};
 
type ResourceState&lt;Data&gt; = {
    /** Current value of the resource */
    current: Data | undefined;
    /** Whether the resource is currently loading */
    loading: boolean;
    /** Error if the fetch failed */
    error: Error | undefined;
};
 
type ResourceReturn&lt;Data, RefetchInfo = unknown&gt; = ResourceState&lt;Data&gt; &amp; {
    /** Update the resource value directly */
    mutate: (value: Data) =&gt; void;
    /** Re-run the fetcher with current values */
    refetch: (info?: RefetchInfo) =&gt; Promise&lt;Data | undefined&gt;;
};
 
type ResourceFetcherRefetchInfo&lt;Data, RefetchInfo = unknown&gt; = {
    /** Previous data return from fetcher */
    data: Data | undefined;
    /** Whether the fetcher is currently refetching or it can be the value you passed to refetch. */
    refetching: RefetchInfo | boolean;
    /** A cleanup function that will be called when the source is invalidated and the fetcher is about to re-run */
    onCleanup: (fn: () =&gt; void) =&gt; void;
    /** AbortSignal for cancelling fetch requests */
    signal: AbortSignal;
};
 
type ResourceFetcher&lt;Source, Data, RefetchInfo = unknown&gt; = (
    /** Current value of the source */
    value: Source extends Array&lt;unknown&gt;
        ? {
                [K in keyof Source]: Source[K];
            }
        : Source,
    /** Previous value of the source */
    previousValue: Source extends Array&lt;unknown&gt;
        ? {
                [K in keyof Source]: Source[K];
            }
        : Source | undefined,
    info: ResourceFetcherRefetchInfo&lt;Data, RefetchInfo&gt;
) =&gt; Promise&lt;Data&gt;;
 
function resource&lt;
    Source,
    RefetchInfo = unknown,
    Fetcher extends ResourceFetcher&lt;
        Source,
        Awaited&lt;ReturnType&lt;Fetcher&gt;&gt;,
        RefetchInfo
    &gt; = ResourceFetcher&lt;Source, any, RefetchInfo&gt;
&gt;(
    source: Getter&lt;Source&gt;,
    fetcher: Fetcher,
    options?: ResourceOptions&lt;Awaited&lt;ReturnType&lt;Fetcher&gt;&gt;&gt;
): ResourceReturn&lt;Awaited&lt;ReturnType&lt;Fetcher&gt;&gt;, RefetchInfo&gt;;
    </code></pre>
<img src="out_runed/utilities/resource/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

## Contributors

<a href="https://github.com/wd-David" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/resource/index_media/90cd387ff9a5aa1590e09e531ec7d97a0908ca8b.jpg" id="bits-165" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="David Peng" />

David Peng<a href="https://github.com/TGlide" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/resource/index_media/4bd89e8b63f277c9d71897d289dcd951b943b3c0.jpg" id="bits-167" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="Thomas G. Lopes" />

Thomas G. Lopes

© 2025 <a href="https://github.com/svecosystem" class="underline-offset-2 hover:underline" target="_blank">Svecosystem</a>

<a href="https://github.com/svecosystem/runed" class="inline-flex select-none items-center justify-center gap-2 whitespace-nowrap text-sm font-semibold tracking-wide active:scale-[99%] disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:size-4 [&amp;_svg]:shrink-0 text-foreground hover:dark:bg-foreground/5 hover:dark:ring-foreground/5 hover:bg-foreground/2 hover:ring-foreground/2 bg-transparent ring ring-transparent h-10 w-10 rounded-md [&amp;&gt;svg]:size-5" aria-label="View this project on GitHub" rel="noopener noreferrer" target="_blank"><img src="out_runed/utilities/resource/index_media/1d22f722c87a133f2381965da1acf7a54aefc45a.svg" /></a>

<img src="out_runed/utilities/resource/index_media/b2f1f2409c2d7a3eaec75643beb67d5092b71fb2.svg" class="size-6" />

# resource

Watch for changes and runs async data fetching

In SvelteKit, using load functions is the primary approach for data fetching. While you can handle reactive data fetching by using `URLSearchParams` for query parameters, there are cases where you might need more flexibility at the component level.

This is where `resource` comes in - it's a utility that seamlessly combines reactive state management with async data fetching.

Built on top of `watch`, it runs after rendering by default, but also provides a pre-render option via `resource.pre()`.

## <a href="https://runed.dev/docs/utilities/resource#demo" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Demo

Post id:

Status: Ready

Refetch Results

## <a href="https://runed.dev/docs/utilities/resource#usage" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Usage

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="svelte" data-theme="github-light github-dark" tabindex="0"><code>
        &lt;script lang=&quot;ts&quot;&gt;
    import { resource } from &quot;runed&quot;;
 
    let id = $state(1);
 
    const searchResource = resource(
        () =&gt; id,
        async (id, prevId, { data, refetching, onCleanup, signal }) =&gt; {
            // data: the previous value returned from the fetcher
 
            // refetching: whether the fetcher is currently refetching
            // or it can be the value you passed to refetch()
 
            // onCleanup: A cleanup function that will be called when the source is invalidated
            // and the fetcher is about to re-run
 
            // signal: AbortSignal for cancelling fetch requests
            const response = await fetch(`api/posts?id=${id}`, { signal });
            return response.json();
        },
        {
            debounce: 300
            // lazy: Skip initial fetch when true
            // once: Only fetch once when true
            // initialValue: Provides an initial value for the resource
            // debounce: Debounce rapid changes
            // throttle: Throttle rapid changes
        }
    );
 
    // The current value of the resource
    searchResource.current;
    // Whether the resource is currently loading
    searchResource.loading;
    // Error if the fetch failed
    searchResource.error;
    // Update the resource value directly, useful for optimistic updates
    searchResource.mutate();
    // Re-run the fetcher with current watching values
    searchResource.refetch();
&lt;/script&gt;
 
&lt;input type=&quot;number&quot; bind:value={id} /&gt;
 
{#if searchResults.loading}
    &lt;div&gt;Loading...&lt;/div&gt;
{:else if searchResults.error}
    &lt;div&gt;Error: {searchResults.error.message}&lt;/div&gt;
{:else}
    &lt;ul&gt;
        {#each searchResults.current ?? [] as result}
            &lt;li&gt;{result.title}&lt;/li&gt;
        {/each}
    &lt;/ul&gt;
{/if}
    </code></pre>
<img src="out_runed/utilities/resource/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

## <a href="https://runed.dev/docs/utilities/resource#features" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Features

- **Automatic Request Cancellation**: When dependencies change, in-flight requests are automatically canceled
- **Loading & Error States**: Built-in states for loading and error handling
- **Debouncing & Throttling**: Optional debounce and throttle options for rate limiting
- **Type Safety**: Full TypeScript support with inferred types
- **Multiple Dependencies**: Support for tracking multiple reactive dependencies
- **Custom Cleanup**: Register cleanup functions that run before refetching
- **Pre-render Support**: `resource.pre()` for pre-render execution

## <a href="https://runed.dev/docs/utilities/resource#advanced-usage" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Advanced Usage

### <a href="https://runed.dev/docs/utilities/resource#multiple-dependencies" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Multiple Dependencies

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="svelte" data-theme="github-light github-dark" tabindex="0"><code>
        &lt;script lang=&quot;ts&quot;&gt;
    const results = resource([() =&gt; query, () =&gt; page], async ([query, page]) =&gt; {
        const res = await fetch(`/api/search?q=${query}&amp;page=${page}`);
        return res.json();
    });
&lt;/script&gt;
    </code></pre>
<img src="out_runed/utilities/resource/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

### <a href="https://runed.dev/docs/utilities/resource#custom-cleanup" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Custom Cleanup

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="svelte" data-theme="github-light github-dark" tabindex="0"><code>
        &lt;script lang=&quot;ts&quot;&gt;
    const stream = resource(
        () =&gt; streamId,
        async (id, _, { signal, onCleanup }) =&gt; {
            const eventSource = new EventSource(`/api/stream/${id}`);
            onCleanup(() =&gt; eventSource.close());
 
            const res = await fetch(`/api/stream/${id}/init`, { signal });
            return res.json();
        }
    );
&lt;/script&gt;
    </code></pre>
<img src="out_runed/utilities/resource/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

### <a href="https://runed.dev/docs/utilities/resource#pre-render-execution" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Pre-render Execution

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="svelte" data-theme="github-light github-dark" tabindex="0"><code>
        &lt;script lang=&quot;ts&quot;&gt;
    const data = resource.pre(
        () =&gt; query,
        async (query) =&gt; {
            const res = await fetch(`/api/search?q=${query}`);
            return res.json();
        }
    );
&lt;/script&gt;
    </code></pre>
<img src="out_runed/utilities/resource/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

## <a href="https://runed.dev/docs/utilities/resource#configuration-options" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Configuration Options

### <a href="https://runed.dev/docs/utilities/resource#lazy" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>`lazy`

When true, skips the initial fetch. The resource will only fetch when dependencies change or `refetch()` is called.

### <a href="https://runed.dev/docs/utilities/resource#once" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>`once`

When true, only fetches once. Subsequent dependency changes won't trigger new fetches.

### <a href="https://runed.dev/docs/utilities/resource#initialvalue" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>`initialValue`

Provides an initial value for the resource before the first fetch completes. Useful if you already have the data and don't want to wait for the fetch to complete.

### <a href="https://runed.dev/docs/utilities/resource#debounce" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>`debounce`

Time in milliseconds to debounce rapid changes. Useful for search inputs.

The debounce implementation will cancel pending requests and only execute the last one after the specified delay.

### <a href="https://runed.dev/docs/utilities/resource#throttle" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>`throttle`

Time in milliseconds to throttle rapid changes. Useful for real-time updates.

The throttle implementation will ensure that requests are spaced at least by the specified delay, returning the pending promise if called too soon.

<img src="out_runed/utilities/resource/index_media/a08f0efc16d25c94f055dec10bc7c24422bffe7b.svg" class="size-6" />

##### Note

Note that you should use either debounce or throttle, not both - if both are specified, debounce takes precedence.

## <a href="https://runed.dev/docs/utilities/resource#type-definitions" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Type Definitions

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-theme="github-light github-dark" tabindex="0"><code>
        type ResourceOptions&lt;Data&gt; = {
    /** Skip initial fetch when true */
    lazy?: boolean;
    /** Only fetch once when true */
    once?: boolean;
    /** Initial value for the resource */
    initialValue?: Data;
    /** Debounce time in milliseconds */
    debounce?: number;
    /** Throttle time in milliseconds */
    throttle?: number;
};
 
type ResourceState&lt;Data&gt; = {
    /** Current value of the resource */
    current: Data | undefined;
    /** Whether the resource is currently loading */
    loading: boolean;
    /** Error if the fetch failed */
    error: Error | undefined;
};
 
type ResourceReturn&lt;Data, RefetchInfo = unknown&gt; = ResourceState&lt;Data&gt; &amp; {
    /** Update the resource value directly */
    mutate: (value: Data) =&gt; void;
    /** Re-run the fetcher with current values */
    refetch: (info?: RefetchInfo) =&gt; Promise&lt;Data | undefined&gt;;
};
 
type ResourceFetcherRefetchInfo&lt;Data, RefetchInfo = unknown&gt; = {
    /** Previous data return from fetcher */
    data: Data | undefined;
    /** Whether the fetcher is currently refetching or it can be the value you passed to refetch. */
    refetching: RefetchInfo | boolean;
    /** A cleanup function that will be called when the source is invalidated and the fetcher is about to re-run */
    onCleanup: (fn: () =&gt; void) =&gt; void;
    /** AbortSignal for cancelling fetch requests */
    signal: AbortSignal;
};
 
type ResourceFetcher&lt;Source, Data, RefetchInfo = unknown&gt; = (
    /** Current value of the source */
    value: Source extends Array&lt;unknown&gt;
        ? {
                [K in keyof Source]: Source[K];
            }
        : Source,
    /** Previous value of the source */
    previousValue: Source extends Array&lt;unknown&gt;
        ? {
                [K in keyof Source]: Source[K];
            }
        : Source | undefined,
    info: ResourceFetcherRefetchInfo&lt;Data, RefetchInfo&gt;
) =&gt; Promise&lt;Data&gt;;
 
function resource&lt;
    Source,
    RefetchInfo = unknown,
    Fetcher extends ResourceFetcher&lt;
        Source,
        Awaited&lt;ReturnType&lt;Fetcher&gt;&gt;,
        RefetchInfo
    &gt; = ResourceFetcher&lt;Source, any, RefetchInfo&gt;
&gt;(
    source: Getter&lt;Source&gt;,
    fetcher: Fetcher,
    options?: ResourceOptions&lt;Awaited&lt;ReturnType&lt;Fetcher&gt;&gt;&gt;
): ResourceReturn&lt;Awaited&lt;ReturnType&lt;Fetcher&gt;&gt;, RefetchInfo&gt;;
    </code></pre>
<img src="out_runed/utilities/resource/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

## Contributors

<a href="https://github.com/wd-David" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/resource/index_media/90cd387ff9a5aa1590e09e531ec7d97a0908ca8b.jpg" id="bits-165" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="David Peng" />

David Peng<a href="https://github.com/TGlide" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/resource/index_media/4bd89e8b63f277c9d71897d289dcd951b943b3c0.jpg" id="bits-167" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="Thomas G. Lopes" />

Thomas G. Lopes
