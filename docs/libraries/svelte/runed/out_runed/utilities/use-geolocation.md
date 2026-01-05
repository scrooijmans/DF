![](out_runed/utilities/use-geolocation/index_media/412c1b99df431530f07813ed77ff123e20b0035f.svg) Toggle Sidebar

<img src="out_runed/utilities/use-geolocation/index_media/2ef71b57cc18afcd4ed82644358ede113ed6d592.svg" class="size-5" />Search Docs ... ⌘ K

<a href="https://github.com/svecosystem/runed" class="inline-flex select-none items-center justify-center gap-2 whitespace-nowrap text-sm font-semibold tracking-wide active:scale-[99%] disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:size-4 [&amp;_svg]:shrink-0 text-foreground hover:dark:bg-foreground/5 hover:dark:ring-foreground/5 hover:bg-foreground/2 hover:ring-foreground/2 bg-transparent ring ring-transparent h-10 w-10 rounded-md [&amp;&gt;svg]:size-5" aria-label="View this project on GitHub" rel="noopener noreferrer" target="_blank"><img src="out_runed/utilities/use-geolocation/index_media/1d22f722c87a133f2381965da1acf7a54aefc45a.svg" /></a>

<img src="out_runed/utilities/use-geolocation/index_media/b2f1f2409c2d7a3eaec75643beb67d5092b71fb2.svg" class="size-6" />

# useGeolocation

Reactive access to the browser's Geolocation API.

`useGeolocation` is a reactive wrapper around the browser's <a href="https://developer.mozilla.org/en-US/docs/Web/API/Geolocation_API" class="text-brand-link hover:text-brand-link-hover leading-7" rel="noopener noreferrer" target="_blank">Geolocation API</a>.

## <a href="https://runed.dev/docs/utilities/use-geolocation#demo" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Demo

    Coords: {
      "accuracy": 0,
      "latitude": null,
      "longitude": null,
      "altitude": null,
      "altitudeAccuracy": null,
      "heading": null,
      "speed": null
    }

    Located at: 0

    Error: null

    Is Supported: false

Pause

Resume

## <a href="https://runed.dev/docs/utilities/use-geolocation#usage" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Usage

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="svelte" data-theme="github-light github-dark" tabindex="0"><code>
        &lt;script lang=&quot;ts&quot;&gt;
    import { useGeolocation } from &quot;runed&quot;;
 
    const location = useGeolocation();
&lt;/script&gt;
 
&lt;pre&gt;Coords: {JSON.stringify(location.position.coords, null, 2)}&lt;/pre&gt;
&lt;pre&gt;Located at: {location.position.timestamp}&lt;/pre&gt;
&lt;pre&gt;Error: {JSON.stringify(location.error, null, 2)}&lt;/pre&gt;
&lt;pre&gt;Is Supported: {location.isSupported}&lt;/pre&gt;
&lt;button onclick={location.pause} disabled={location.isPaused}&gt;Pause&lt;/button&gt;
&lt;button onclick={location.resume} disabled={!location.isPaused}&gt;Resume&lt;/button&gt;
    </code></pre>
<img src="out_runed/utilities/use-geolocation/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

## <a href="https://runed.dev/docs/utilities/use-geolocation#type-definitions" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Type Definitions

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-theme="github-light github-dark" tabindex="0"><code>
        type UseGeolocationOptions = Partial&lt;PositionOptions&gt; &amp; {
    /**
     * Whether to start the watcher immediately upon creation.
     * If set to `false`, the watcher will only start tracking the position when `resume()` is called.
     *
     * @defaultValue true
     */
    immediate?: boolean;
};
 
type UseGeolocationReturn = {
    readonly isSupported: boolean;
    readonly position: Omit&lt;GeolocationPosition, &quot;toJSON&quot;&gt;;
    readonly error: GeolocationPositionError | null;
    readonly isPaused: boolean;
    pause: () =&gt; void;
    resume: () =&gt; void;
};
    </code></pre>
<img src="out_runed/utilities/use-geolocation/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

## Contributors

<a href="https://github.com/huntabyte" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/use-geolocation/index_media/75bcb1d595b8d37466d561f08b890ffc8f1d5425.jpg" id="bits-1495" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="Hunter Johnston" />

Hunter Johnston

© 2025 <a href="https://github.com/svecosystem" class="underline-offset-2 hover:underline" target="_blank">Svecosystem</a>

<a href="https://github.com/svecosystem/runed" class="inline-flex select-none items-center justify-center gap-2 whitespace-nowrap text-sm font-semibold tracking-wide active:scale-[99%] disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:size-4 [&amp;_svg]:shrink-0 text-foreground hover:dark:bg-foreground/5 hover:dark:ring-foreground/5 hover:bg-foreground/2 hover:ring-foreground/2 bg-transparent ring ring-transparent h-10 w-10 rounded-md [&amp;&gt;svg]:size-5" aria-label="View this project on GitHub" rel="noopener noreferrer" target="_blank"><img src="out_runed/utilities/use-geolocation/index_media/1d22f722c87a133f2381965da1acf7a54aefc45a.svg" /></a>

<img src="out_runed/utilities/use-geolocation/index_media/b2f1f2409c2d7a3eaec75643beb67d5092b71fb2.svg" class="size-6" />

# useGeolocation

Reactive access to the browser's Geolocation API.

`useGeolocation` is a reactive wrapper around the browser's <a href="https://developer.mozilla.org/en-US/docs/Web/API/Geolocation_API" class="text-brand-link hover:text-brand-link-hover leading-7" rel="noopener noreferrer" target="_blank">Geolocation API</a>.

## <a href="https://runed.dev/docs/utilities/use-geolocation#demo" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Demo

    Coords: {
      "accuracy": 0,
      "latitude": null,
      "longitude": null,
      "altitude": null,
      "altitudeAccuracy": null,
      "heading": null,
      "speed": null
    }

    Located at: 0

    Error: null

    Is Supported: false

Pause

Resume

## <a href="https://runed.dev/docs/utilities/use-geolocation#usage" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Usage

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="svelte" data-theme="github-light github-dark" tabindex="0"><code>
        &lt;script lang=&quot;ts&quot;&gt;
    import { useGeolocation } from &quot;runed&quot;;
 
    const location = useGeolocation();
&lt;/script&gt;
 
&lt;pre&gt;Coords: {JSON.stringify(location.position.coords, null, 2)}&lt;/pre&gt;
&lt;pre&gt;Located at: {location.position.timestamp}&lt;/pre&gt;
&lt;pre&gt;Error: {JSON.stringify(location.error, null, 2)}&lt;/pre&gt;
&lt;pre&gt;Is Supported: {location.isSupported}&lt;/pre&gt;
&lt;button onclick={location.pause} disabled={location.isPaused}&gt;Pause&lt;/button&gt;
&lt;button onclick={location.resume} disabled={!location.isPaused}&gt;Resume&lt;/button&gt;
    </code></pre>
<img src="out_runed/utilities/use-geolocation/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

## <a href="https://runed.dev/docs/utilities/use-geolocation#type-definitions" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Type Definitions

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-theme="github-light github-dark" tabindex="0"><code>
        type UseGeolocationOptions = Partial&lt;PositionOptions&gt; &amp; {
    /**
     * Whether to start the watcher immediately upon creation.
     * If set to `false`, the watcher will only start tracking the position when `resume()` is called.
     *
     * @defaultValue true
     */
    immediate?: boolean;
};
 
type UseGeolocationReturn = {
    readonly isSupported: boolean;
    readonly position: Omit&lt;GeolocationPosition, &quot;toJSON&quot;&gt;;
    readonly error: GeolocationPositionError | null;
    readonly isPaused: boolean;
    pause: () =&gt; void;
    resume: () =&gt; void;
};
    </code></pre>
<img src="out_runed/utilities/use-geolocation/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

## Contributors

<a href="https://github.com/huntabyte" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/use-geolocation/index_media/75bcb1d595b8d37466d561f08b890ffc8f1d5425.jpg" id="bits-1495" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="Hunter Johnston" />

Hunter Johnston
