![](out_runed/utilities/use-search-params/index_media/412c1b99df431530f07813ed77ff123e20b0035f.svg) Toggle Sidebar

<img src="out_runed/utilities/use-search-params/index_media/2ef71b57cc18afcd4ed82644358ede113ed6d592.svg" class="size-5" />Search Docs ... ⌘ K

<a href="https://github.com/svecosystem/runed" class="inline-flex select-none items-center justify-center gap-2 whitespace-nowrap text-sm font-semibold tracking-wide active:scale-[99%] disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:size-4 [&amp;_svg]:shrink-0 text-foreground hover:dark:bg-foreground/5 hover:dark:ring-foreground/5 hover:bg-foreground/2 hover:ring-foreground/2 bg-transparent ring ring-transparent h-10 w-10 rounded-md [&amp;&gt;svg]:size-5" aria-label="View this project on GitHub" rel="noopener noreferrer" target="_blank"><img src="out_runed/utilities/use-search-params/index_media/1d22f722c87a133f2381965da1acf7a54aefc45a.svg" /></a>

<img src="out_runed/utilities/use-search-params/index_media/b2f1f2409c2d7a3eaec75643beb67d5092b71fb2.svg" class="size-6" />

# useSearchParams

Reactive, schema-validated URL search params for Svelte/SvelteKit

`useSearchParams` provides a reactive, type-safe, and schema-driven way to manage URL search parameters in Svelte/SvelteKit apps. It supports validation, default values, compression, debouncing, and history control.

## <a href="https://runed.dev/docs/utilities/use-search-params#demo" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Demo

Add Field

<img src="out_runed/utilities/use-search-params/index_media/a08f0efc16d25c94f055dec10bc7c24422bffe7b.svg" class="size-6" />

##### Note

You can refresh this page and/or open it in another tab to see the count state being persisted and synchronized across sessions and tabs.

## <a href="https://runed.dev/docs/utilities/use-search-params#requirements" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Requirements

- **`@sveltejs/kit`** must be installed in your project.
- Uses <a href="https://standardschema.dev/" class="text-brand-link hover:text-brand-link-hover leading-7" rel="noopener noreferrer" target="_blank">Standard Schema</a> for schema validation and type inference.

## <a href="https://runed.dev/docs/utilities/use-search-params#usage" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Usage

Define schema:

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-theme="github-light github-dark" tabindex="0"><code>
        // schemas.ts
import { z } from &quot;zod&quot;;
// Version 3.24.0+
 
export const productSearchSchema = z.object({
    page: z.coerce.number().default(1),
    filter: z.string().default(&quot;&quot;),
    sort: z.enum([&quot;newest&quot;, &quot;oldest&quot;, &quot;price&quot;]).default(&quot;newest&quot;)
});
    </code></pre>
<img src="out_runed/utilities/use-search-params/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

In your svelte code:

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="svelte" data-theme="github-light github-dark" tabindex="0"><code>
        &lt;script lang=&quot;ts&quot;&gt;
import { useSearchParams } from &quot;runed/kit&quot;;
import { productSearchSchema } from &#39;./schemas&#39;;
 
const params = useSearchParams(productSearchSchema);
// Access parameters directly
const page = $derived(params.page); // number (defaults to 1)
const sort = $derived(params.sort); // &#39;newest&#39; | &#39;oldest&#39; | &#39;price&#39;
 
// Update parameters directly
params.page = 2; // Updates URL to include ?page=2
// Updates URL to include ?page=3&amp;sort=oldest
params.update({ page: 3, sort: &#39;oldest&#39; });
// Resets all parameters to their default values
params.reset();
// Returns URLSearchParams object with all current parameter values
params.toURLSearchParams();
/**
 * You can watch for changes to the URLSearchParams object
 * For example:
 * watch(() =&gt; params.toURLSearchParams(), () =&gt; {
 *   // Do something whenever the URLSearchParams object changes
 * })
 */
&lt;/script&gt;
 
&lt;-- Great for binding to input fields --&gt;
&lt;input type=&quot;text&quot; bind:value={params.filter} /&gt;
    </code></pre>
<img src="out_runed/utilities/use-search-params/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

In your load function:

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-theme="github-light github-dark" tabindex="0"><code>
        import { validateSearchParams } from &quot;runed/kit&quot;;
import { productSearchSchema } from &quot;./schemas&quot;;
 
export const load = ({ url, fetch }) =&gt; {
    // Get validated search params as URLSearchParams object
    // If you use a custom compressedParamName in useSearchParams, provide it here too:
    const { searchParams } = validateSearchParams(url, productSearchSchema);
 
    // Use URLSearchParams directly with fetch
    const response = await fetch(`/api/products?${searchParams.toString()}`);
    return {
        products: await response.json()
    };
};
    </code></pre>
<img src="out_runed/utilities/use-search-params/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

## <a href="https://runed.dev/docs/utilities/use-search-params#features" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Features

- **Schema Validation**: Define types, defaults, and structure for each param.
- **Default Values**: Missing params are auto-filled with defaults.
- **Type Safety**: All values are parsed and validated to the schema.
- **Compression**: Store all params in a single compressed `_data` param for cleaner URLs.
- **Debounce**: Optionally debounce updates for smoother UX.
- **History Control**: Choose between push/replace state.
- **In-memory Mode**: Params are kept in memory, not in the URL.
- **Invalid Param Handling**: Invalid values are replaced with defaults and removed from the URL.

### <a href="https://runed.dev/docs/utilities/use-search-params#usesearchparams" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>`useSearchParams`

Hook to create a reactive search params object with property access

This client-side hook automatically updates the URL when parameters change. It provides type-safe access to URL search parameters through direct property access.

**Parameters:**

- `schema`: A validation schema compatible with StandardSchemaV1
- `options`: Configuration options that affect URL behavior

**Returns:**

- A reactive object for working with typed search parameters

#### <a href="https://runed.dev/docs/utilities/use-search-params#available-options" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Available options:

- `showDefaults` (boolean): When true, parameters with default values will be shown in the URL. When false (default), parameters with default values will be omitted from the URL.
- `debounce` (number): Milliseconds to delay URL updates when parameters change. Useful to avoid cluttering browser history when values change rapidly (default: 0, no debounce).
- `pushHistory` (boolean): Controls whether URL updates create new browser history entries. If true (default), each update adds a new entry to the browser history. If false, updates replace the current URL without creating new history entries.
- `compress` (boolean): When true, all parameters are compressed into a single parameter using lz-string compression. This helps reduce URL length and provides basic obfuscation (default: false). Use validateSearchParams with the same compressedParamName option when handling compressed URLs server-side.
- `compressedParamName` (string): The name of the parameter used to store compressed data when compression is enabled. Customize this to avoid conflicts with parameters in your schema. Default is `_data`.
- `updateURL` (boolean): When true (default), the URL is updated when parameters change. When false, only in-memory parameters are updated.
- `noScroll` (boolean): When true, the scroll position is preserved when the URL is updated. This prevents the page from jumping to the top on URL changes. Default is false.

Example with <a href="https://zod.dev/" class="text-brand-link hover:text-brand-link-hover leading-7" rel="noopener noreferrer" target="_blank">Zod</a>:

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-theme="github-light github-dark" tabindex="0"><code>
        import { z } from &quot;zod&quot;;
 
const productSearchSchema = z.object({
    page: z.number().default(1),
    filter: z.string().default(&quot;&quot;),
    sort: z.enum([&quot;newest&quot;, &quot;oldest&quot;, &quot;price&quot;]).default(&quot;newest&quot;)
});
const params = useSearchParams(productSearchSchema);
// Access parameters directly
const page = $derived(params.page); // number (defaults to 1)
const sort = $derived(params.sort); // &#39;newest&#39; | &#39;oldest&#39; | &#39;price&#39;
    </code></pre>
<img src="out_runed/utilities/use-search-params/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

Example with options:

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-theme="github-light github-dark" tabindex="0"><code>
        // Show default values in URL, debounce updates by 300ms,
// don&#39;t create new history entries, and compress params
const params = useSearchParams(schema, {
  showDefaults: true,
  debounce: 300,
  pushHistory: false,
  compress: true,
  compressedParamName: &#39;_compressed&#39; // Custom name to avoid conflicts
});
// Great for binding to input fields (updates URL without cluttering history)
&lt;input type=&quot;text&quot; bind:value={params.search} /&gt;
// Resulting URL will be something like: /?_compressed=N4IgDgTg9g...
    </code></pre>
<img src="out_runed/utilities/use-search-params/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

Example with <a href="https://valibot.dev/" class="text-brand-link hover:text-brand-link-hover leading-7" rel="noopener noreferrer" target="_blank">Valibot</a>:

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-theme="github-light github-dark" tabindex="0"><code>
        import * as v from &quot;valibot&quot;;
const productSearchSchema = v.object({
    page: v.optional(v.fallback(v.number(), 1), 1),
    filter: v.optional(v.fallback(v.string(), &quot;&quot;), &quot;&quot;),
    sort: v.optional(v.fallback(v.picklist([&quot;newest&quot;, &quot;oldest&quot;, &quot;price&quot;]), &quot;newest&quot;), &quot;newest&quot;)
});
const params = useSearchParams(productSearchSchema);
    </code></pre>
<img src="out_runed/utilities/use-search-params/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

Example with <a href="https://arktype.io/" class="text-brand-link hover:text-brand-link-hover leading-7" rel="noopener noreferrer" target="_blank">Arktype</a>:

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-theme="github-light github-dark" tabindex="0"><code>
        import { type } from &quot;arktype&quot;;
const productSearchSchema = type({
    page: &quot;number = 1&quot;,
    filter: &#39;string = &quot;&quot;&#39;,
    sort: &#39;&quot;newest&quot; | &quot;oldest&quot; | &quot;price&quot; = &quot;newest&quot;&#39;
});
const params = useSearchParams(productSearchSchema);
    </code></pre>
<img src="out_runed/utilities/use-search-params/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

Or with our built-in schema creator (no additional dependencies)

### <a href="https://runed.dev/docs/utilities/use-search-params#createsearchparamsschema" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>`createSearchParamsSchema`

Creates a simple schema compatible with <a href="https://standardschema.dev/" class="text-brand-link hover:text-brand-link-hover leading-7" rel="noopener noreferrer" target="_blank">Standard Schema</a> without requiring external validation libraries.

This is a lightweight alternative to using full schema validation libraries like Zod, Valibot, or Arktype. Use this when you need basic type conversion and default values without adding dependencies.

Limitations:

- For 'array' type: supports basic arrays, but doesn't validate array items
- For 'object' type: supports generic objects, but doesn't validate nested properties
- No custom validation rules or transformations
- **No granular reactivity**: Changes to nested properties require reassigning the entire value
  - ❌ `params.config.theme = 'dark'` (won't trigger URL update)
  - ✅ `params.config = {...params.config, theme: 'dark'}` (will trigger URL update)

For complex validation needs (nested validation, refined rules, etc.), use a dedicated validation library instead.

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-theme="github-light github-dark" tabindex="0"><code>
        const productSearchSchema = createSearchParamsSchema({
    // Basic types with defaults
    page: { type: &quot;number&quot;, default: 1 },
    filter: { type: &quot;string&quot;, default: &quot;&quot; },
    sort: { type: &quot;string&quot;, default: &quot;newest&quot; },
 
    // Array type with specific element type
    tags: {
        type: &quot;array&quot;,
        default: [&quot;new&quot;],
        arrayType: &quot;&quot; // Specify string[] type
    },
 
    // Object type with specific shape
    config: {
        type: &quot;object&quot;,
        default: { theme: &quot;light&quot; },
        objectType: { theme: &quot;&quot; } // Specify { theme: string } type
    }
});
    </code></pre>
<img src="out_runed/utilities/use-search-params/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

URL storage format:

- Arrays are stored as JSON strings: `?tags=["sale","featured"]`
- Objects are stored as JSON strings: `?config={"theme":"dark","fontSize":14}`
- Dates are stored as ISO8601 strings: `?createdAt=2023-12-01T10:30:00.000Z` (or `YYYY-MM-DD` with date-only format)
- Primitive values are stored directly: `?page=2&filter=red`

#### <a href="https://runed.dev/docs/utilities/use-search-params#date-format-support" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Date Format Support

You can control how Date parameters are serialized in URLs using two approaches:

**Option 1: Using `dateFormat` property in schema**

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-theme="github-light github-dark" tabindex="0"><code>
        const schema = createSearchParamsSchema({
    // Date-only format (YYYY-MM-DD) - great for birth dates, event dates
    birthDate: {
        type: &quot;date&quot;,
        default: new Date(&quot;1990-01-15&quot;),
        dateFormat: &quot;date&quot;
    },
 
    // Full datetime format (ISO8601) - great for timestamps
    createdAt: {
        type: &quot;date&quot;,
        default: new Date(),
        dateFormat: &quot;datetime&quot;
    },
 
    // No format specified - defaults to &#39;datetime&#39;
    updatedAt: {
        type: &quot;date&quot;,
        default: new Date()
    }
});
 
const params = useSearchParams(schema);
// URL: ?birthDate=1990-01-15&amp;createdAt=2023-01-01T10:30:00.000Z&amp;updatedAt=2023-12-25T18:30:00.000Z
    </code></pre>
<img src="out_runed/utilities/use-search-params/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

**Option 2: Using `dateFormats` option (works with any validator)**

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-theme="github-light github-dark" tabindex="0"><code>
        // Works with Zod, Valibot, Arktype, or createSearchParamsSchema
const params = useSearchParams(zodSchema, {
    dateFormats: {
        birthDate: &quot;date&quot;, // YYYY-MM-DD
        createdAt: &quot;datetime&quot; // ISO8601
    }
});
    </code></pre>
<img src="out_runed/utilities/use-search-params/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

**Date Format Details:**

- **`'date'` format**: Serializes as `YYYY-MM-DD` (e.g., `2025-10-21`)
  - More readable in URLs
  - Perfect for calendar dates, birth dates, event dates
  - Parsed as Date object with time set to midnight UTC
- **`'datetime'` format** (default): Serializes as full ISO8601 (e.g., `2025-10-21T18:18:14.196Z`)
  - Preserves exact time information
  - Perfect for timestamps, created/updated times
  - Full precision date and time

**Practical Example:**

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="svelte" data-theme="github-light github-dark" tabindex="0"><code>
        &lt;script lang=&quot;ts&quot;&gt;
    import { useSearchParams, createSearchParamsSchema } from &quot;runed/kit&quot;;
 
    const schema = createSearchParamsSchema({
        eventDate: {
            type: &quot;date&quot;,
            default: new Date(&quot;2025-01-01&quot;),
            dateFormat: &quot;date&quot;
        },
        createdAt: {
            type: &quot;date&quot;,
            default: new Date(),
            dateFormat: &quot;datetime&quot;
        }
    });
 
    const params = useSearchParams(schema);
&lt;/script&gt;
 
&lt;label&gt;
    Event Date:
    &lt;input
        type=&quot;date&quot;
        value={params.eventDate.toISOString().split(&quot;T&quot;)[0]}
        oninput={(e) =&gt; (params.eventDate = new Date(e.target.value))} /&gt;
&lt;/label&gt;
 
&lt;label&gt;
    Created At:
    &lt;input
        type=&quot;datetime-local&quot;
        value={params.createdAt.toISOString().slice(0, 16)}
        oninput={(e) =&gt; (params.createdAt = new Date(e.target.value))} /&gt;
&lt;/label&gt;
 
&lt;!-- URL will be: ?eventDate=2025-01-01&amp;createdAt=2025-10-21T18:18:14.196Z --&gt;
    </code></pre>
<img src="out_runed/utilities/use-search-params/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

### <a href="https://runed.dev/docs/utilities/use-search-params#validatesearchparams" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>`validateSearchParams`

A utility function to extract, validate and convert URL search parameters to <a href="https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams" class="text-brand-link hover:text-brand-link-hover leading-7" rel="noopener noreferrer" target="_blank">URLSearchParams</a>

This function makes it easy to use the same schema validation in both client-side components (via `useSearchParams`) and server-side load functions. Unlike \`useSearchParams, this function doesn't modify the URL - it only validates parameters and returns them as a new URLSearchParams object.

Handles both standard URL parameters and compressed parameters (when compression is enabled).

**Parameters:**

- `url`: The URL object from SvelteKit load function
- `schema`: A validation schema (createSearchParamsSchema, Zod, Valibot, etc.)
- `options`: Optional configuration (like custom `compressedParamName` and `dateFormats`)

**Returns:**

- An object with `searchParams` and `data` properties, `searchParams` being the validated `URLSearchParams` and `data` being the validated object

**Available options:**

- `compressedParamName` (string): Custom name for compressed parameter (default: `_data`)
- `dateFormats` (object): Map of field names to date formats (`'date'` or `'datetime'`)

Example with SvelteKit page or layout load function:

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-theme="github-light github-dark" tabindex="0"><code>
        import { validateSearchParams } from &quot;runed/kit&quot;;
import { productSchema } from &quot;./schemas&quot;;
 
export const load = ({ url, fetch }) =&gt; {
    // Get validated search params as URLSearchParams object
    // If you use a custom compressedParamName in useSearchParams, provide it here too:
    const { searchParams } = validateSearchParams(url, productSchema, {
        compressedParamName: &quot;_compressed&quot;,
        dateFormats: {
            birthDate: &quot;date&quot;, // Serialize as YYYY-MM-DD
            createdAt: &quot;datetime&quot; // Serialize as ISO8601
        }
    });
 
    // Use URLSearchParams directly with fetch
    const response = await fetch(`/api/products?${searchParams.toString()}`);
    return {
        products: await response.json()
    };
};
    </code></pre>
<img src="out_runed/utilities/use-search-params/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

### <a href="https://runed.dev/docs/utilities/use-search-params#advanced-custom-serialization-with-zod-codecs" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Advanced: Custom Serialization with Zod Codecs

For advanced use cases where you need full control over how values are converted between URL strings and JavaScript types, you can use <a href="https://zod.dev/?id=codec" class="text-brand-link hover:text-brand-link-hover leading-7" rel="noopener noreferrer" target="_blank">Zod codecs</a> (Zod v4.1.0+). Codecs allow you to define custom bidirectional transformations that work seamlessly with URL parameters.

#### <a href="https://runed.dev/docs/utilities/use-search-params#why-use-codecs" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Why Use Codecs?

While the built-in `dateFormats` option works well for common cases, codecs give you complete control over serialization. Use codecs when you need to:

- **Custom date formats**: Store dates as Unix timestamps, custom date strings, or other formats
- **Complex type conversions**: Convert between incompatible types (e.g., number IDs ↔ full objects)
- **Data transformation**: Apply transformations during serialization (e.g., normalize, encrypt)
- **Legacy API compatibility**: Match existing URL parameter formats from other systems
- **Optimization**: Use more compact representations (e.g., `1234567890` instead of `2009-02-13T23:31:30.000Z`)

#### <a href="https://runed.dev/docs/utilities/use-search-params#how-codecs-work" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>How Codecs Work

A Zod codec defines two transformations:

- **`decode`**: Converts URL string → JavaScript type (when reading from URL)
- **`encode`**: Converts JavaScript type → URL string (when writing to URL)

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-theme="github-light github-dark" tabindex="0"><code>
        import { z } from &quot;zod&quot;;
 
// Example 1: Unix timestamp codec (stores Date as number)
const unixTimestampCodec = z.codec(
    z.coerce.number(), // Input: number from URL string
    z.date(), // Output: Date object in your app
    {
        decode: (timestamp) =&gt; new Date(timestamp * 1000), // number → Date
        encode: (date) =&gt; Math.floor(date.getTime() / 1000) // Date → number
    }
);
 
// Example 2: Date-only codec (stores Date as YYYY-MM-DD)
const dateOnlyCodec = z.codec(
    z.string(), // Input: string from URL
    z.date(), // Output: Date object in your app
    {
        decode: (str) =&gt; new Date(str + &quot;T00:00:00.000Z&quot;), // &quot;2025-01-15&quot; → Date
        encode: (date) =&gt; date.toISOString().split(&quot;T&quot;)[0] // Date → &quot;2025-01-15&quot;
    }
);
 
// Example 3: Product ID codec (stores number as base36 string for shorter URLs)
const compactIdCodec = z.codec(
    z.string(), // Input: base36 string from URL
    z.number(), // Output: number in your app
    {
        decode: (str) =&gt; parseInt(str, 36), // &quot;abc123&quot; → 225249695
        encode: (num) =&gt; num.toString(36) // 225249695 → &quot;abc123&quot;
    }
);
    </code></pre>
<img src="out_runed/utilities/use-search-params/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

#### <a href="https://runed.dev/docs/utilities/use-search-params#using-codecs-in-your-schema" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Using Codecs in Your Schema

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-theme="github-light github-dark" tabindex="0"><code>
        import { z } from &quot;zod&quot;;
 
const searchSchema = z.object({
    // Regular fields work as before
    query: z.string().default(&quot;&quot;),
    page: z.coerce.number().default(1),
 
    // Unix timestamp - more compact than ISO datetime
    createdAfter: unixTimestampCodec.default(new Date(&quot;2024-01-01&quot;)),
 
    // Date-only format - cleaner for calendar dates
    birthDate: dateOnlyCodec.default(new Date(&quot;1990-01-15&quot;)),
 
    // Compact product IDs
    productId: compactIdCodec.optional()
});
 
const params = useSearchParams(searchSchema);
    </code></pre>
<img src="out_runed/utilities/use-search-params/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

#### <a href="https://runed.dev/docs/utilities/use-search-params#real-world-example-event-search" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Real-World Example: Event Search

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="svelte" data-theme="github-light github-dark" tabindex="0"><code>
        &lt;script lang=&quot;ts&quot;&gt;
    import { z } from &quot;zod&quot;;
    import { useSearchParams } from &quot;runed/kit&quot;;
 
    // Define reusable codecs
    const unixTimestamp = z.codec(z.coerce.number(), z.date(), {
        decode: (ts) =&gt; new Date(ts * 1000),
        encode: (date) =&gt; Math.floor(date.getTime() / 1000)
    });
 
    const dateOnly = z.codec(z.string(), z.date(), {
        decode: (str) =&gt; new Date(str + &quot;T00:00:00.000Z&quot;),
        encode: (date) =&gt; date.toISOString().split(&quot;T&quot;)[0]
    });
 
    // Schema with multiple codec types
    const eventSearchSchema = z.object({
        query: z.string().default(&quot;&quot;),
        // Date-only for event date (more readable in URL)
        eventDate: dateOnly.default(new Date()),
        // Unix timestamp for filters (more compact)
        createdAfter: unixTimestamp.optional(),
        updatedSince: unixTimestamp.optional()
    });
 
    const params = useSearchParams(eventSearchSchema);
 
    // Work with Date objects in your app
    $inspect(params.eventDate); // Date object
    $inspect(params.createdAfter); // Date object or undefined
&lt;/script&gt;
 
&lt;!-- Bind to native date inputs --&gt;
&lt;label&gt;
    Event Date:
    &lt;input
        type=&quot;date&quot;
        value={params.eventDate.toISOString().split(&quot;T&quot;)[0]}
        oninput={(e) =&gt; (params.eventDate = new Date(e.target.value))} /&gt;
&lt;/label&gt;
 
&lt;label&gt;
    Created After:
    &lt;input
        type=&quot;date&quot;
        value={params.createdAfter?.toISOString().split(&quot;T&quot;)[0] ?? &quot;&quot;}
        oninput={(e) =&gt;
            (params.createdAfter = e.target.value ? new Date(e.target.value) : undefined)} /&gt;
&lt;/label&gt;
 
&lt;!-- Clean URLs:
     Without codecs: ?eventDate=2025-01-15T00:00:00.000Z&amp;createdAfter=2024-01-01T00:00:00.000Z
     With codecs:    ?eventDate=2025-01-15&amp;createdAfter=1704067200
--&gt;
    </code></pre>
<img src="out_runed/utilities/use-search-params/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

#### <a href="https://runed.dev/docs/utilities/use-search-params#codec-benefits-summary" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Codec Benefits Summary

| Feature | Built-in dateFormats | Zod Codecs |
|----|----|----|
| **Setup complexity** | Simple | More configuration needed |
| **Date formats** | `date` and `datetime` | Any custom format (Unix, relative, custom string) |
| **URL size** | Standard | Can be optimized (e.g., Unix timestamps) |
| **Type conversions** | Date only | Any type (numbers, objects, arrays, etc.) |
| **Validation** | Basic | Full Zod validation + transformation |
| **Reusability** | Per-field config | Create reusable codec definitions |
| **Legacy compatibility** | Limited | Full control over format |
| **Works with validators** | All (Zod, Valibot, etc.) | Zod only (v4.1.0+) |
| **Server-side usage** | Use `dateFormats` option | Automatic with `validateSearchParams` |

**When to use `dateFormats`**: Most applications with standard date handling needs

**When to use codecs**: When you need custom formats, compact representations, or complex type conversions

#### <a href="https://runed.dev/docs/utilities/use-search-params#server-side-usage-with-codecs" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Server-Side Usage with Codecs

Codecs work automatically with `validateSearchParams`:

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-theme="github-light github-dark" tabindex="0"><code>
        // +page.server.ts
import { validateSearchParams } from &quot;runed/kit&quot;;
import { eventSearchSchema } from &quot;./schemas&quot;; // Schema with codecs
 
export const load = ({ url }) =&gt; {
    // Codecs are automatically applied during validation
    const { searchParams, data } = validateSearchParams(url, eventSearchSchema);
 
    // data.eventDate is a Date object (decoded from URL string)
    // searchParams contains properly encoded values for API calls
    return {
        events: await fetchEvents(searchParams)
    };
};
    </code></pre>
<img src="out_runed/utilities/use-search-params/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

## <a href="https://runed.dev/docs/utilities/use-search-params#reactivity-limitations" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Reactivity Limitations

### <a href="https://runed.dev/docs/utilities/use-search-params#understanding-reactivity-scope" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Understanding Reactivity Scope

`useSearchParams` provides **top-level reactivity only**. This means:

✅ **Works (Direct property assignment)**:

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="svelte" data-theme="github-light github-dark" tabindex="0"><code>
        &lt;script&gt;
    const params = useSearchParams(schema);
 
    // These trigger URL updates
    params.page = 2;
    params.filter = &quot;active&quot;;
    params.config = { theme: &quot;dark&quot;, size: &quot;large&quot; };
    params.items = [...params.items, newItem];
&lt;/script&gt;
    </code></pre>
<img src="out_runed/utilities/use-search-params/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

❌ **Doesn't work (Nested property mutations)**:

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="svelte" data-theme="github-light github-dark" tabindex="0"><code>
        &lt;script&gt;
    const params = useSearchParams(schema);
 
    // These DON&#39;T trigger URL updates
    params.config.theme = &quot;dark&quot;; // Nested object property
    params.items.push(newItem); // Array method
    params.items[0].name = &quot;updated&quot;; // Array item property
    delete params.config.oldProp; // Property deletion
&lt;/script&gt;
    </code></pre>
<img src="out_runed/utilities/use-search-params/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

### <a href="https://runed.dev/docs/utilities/use-search-params#why-this-design-choice" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Why This Design Choice

`useSearchParams` was designed to prioritize **simplicity, type safety, and ease of use** over deep reactivity. This design choice offers several benefits:

#### <a href="https://runed.dev/docs/utilities/use-search-params#-what-you-get" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>✅ **What You Get**

- **Simple, predictable API**: `params.page = 2` always works
- **Full TypeScript support**: Perfect autocomplete and type checking
- **Clean URLs**: Objects serialize to readable JSON strings
- **Performance**: No overhead from tracking deep object changes
- **Reliability**: No edge cases with complex nested proxy behaviors

## <a href="https://runed.dev/docs/utilities/use-search-params#type-definitions" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Type Definitions

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-theme="github-light github-dark" tabindex="0"><code>
        interface SearchParamsOptions {
    /**
     * If true, parameters set to their default values will be shown in the URL.
     * If false, parameters with default values will be omitted from the URL.
     * @default false
     */
    showDefaults?: boolean;
 
    /**
     * The number of milliseconds to delay URL updates when parameters change.
     * This helps avoid cluttering browser history when values change rapidly
     * (like during typing in an input field).
     * @default 0 (no debounce)
     */
    debounce?: number;
 
    /**
     * Controls whether URL updates create new browser history entries.
     * If true (default), each update adds a new entry to the browser history.
     * If false, updates replace the current URL without creating new history entries.
     * @default true
     */
    pushHistory?: boolean;
 
    /**
     * Enable lz-string compression for all parameters.
     * When true, all parameters are compressed into a single parameter in the URL.
     * This helps reduce URL length and provides basic parameter obfuscation.
     * @default false
     */
    compress?: boolean;
 
    /**
     * The name of the parameter used to store compressed data when compression is enabled.
     * You can customize this to avoid conflicts with your schema parameters.
     * For example, if your schema already uses &#39;_data&#39;, you might want to use &#39;_compressed&#39; or another unique name.
     * @default &#39;_data&#39;
     */
    compressedParamName?: string;
 
    /**
     * Controls whether to update the URL when parameters change.
     * If true (default), changes to parameters will update the URL.
     * If false, parameters will only be stored in memory without updating the URL.
     * Note: When false, compress option will be ignored.
     * @default true
     */
    updateURL?: boolean;
 
    /**
     * If true, the page will not scroll to the top when the URL is updated.
     * This is useful if you want to maintain the user&#39;s scroll position during parameter changes.
     * @default false
     */
    noScroll?: boolean;
 
    /**
     * Specifies which date fields should use date-only format (YYYY-MM-DD) instead of full ISO8601 datetime.
     *
     * Map field names to their desired format:
     * - &#39;date&#39;: Serializes as YYYY-MM-DD (e.g., &quot;2025-10-21&quot;)
     * - &#39;datetime&#39;: Serializes as full ISO8601 (e.g., &quot;2025-10-21T18:18:14.196Z&quot;)
     *
     * Example:
     * { dateFormats: { birthDate: &#39;date&#39;, createdAt: &#39;datetime&#39; } }
     *
     * @default undefined (all dates use datetime format)
     */
    dateFormats?: Record&lt;string, &quot;date&quot; | &quot;datetime&quot;&gt;;
}
 
type ReturnUseSearchParams&lt;Schema extends StandardSchemaV1&gt; = {
    [key: string]: any; // Typed, reactive params
    /**
     * Update multiple parameters at once
     *
     * This is more efficient than setting multiple parameters individually
     * because it only triggers one URL update or one in-memory store update.
     *
     * @param values An object containing parameter key-value pairs to update.
     *  For example: params.update({ page: 1, sort: &#39;newest&#39; })
     */
    update(values: Partial&lt;StandardSchemaV1.InferOutput&lt;Schema&gt;&gt;): void;
    /**
     * Reset all parameters to their default values
     *
     * This method removes all current URL parameters or in-memory parameters
     * and optionally sets parameters with non-default values back to their defaults.
     *
     * @param showDefaults Whether to show default values in the URL or in-memory store after reset.
     * If not provided, uses the instance&#39;s showDefaults option.
     */
    reset(showDefaults?: boolean): void;
    /**
     * Convert the current schema parameters to a URLSearchParams object
     * This includes all values defined in the schema, regardless of their presence in the URL
     * @returns URLSearchParams object containing all current parameter values
     */
    toURLSearchParams(): URLSearchParams;
};
 
/**
 * Schema type for createSearchParamsSchema
 * Allows specifying more precise types for arrays, objects, and dates
 */
type SchemaTypeConfig&lt;ArrayType = unknown, ObjectType = unknown&gt; =
    | { type: &quot;string&quot;; default?: string }
    | { type: &quot;number&quot;; default?: number }
    | { type: &quot;boolean&quot;; default?: boolean }
    | { type: &quot;array&quot;; default?: ArrayType[]; arrayType?: ArrayType }
    | { type: &quot;object&quot;; default?: ObjectType; objectType?: ObjectType }
    | { type: &quot;date&quot;; default?: Date; dateFormat?: &quot;date&quot; | &quot;datetime&quot; };
    </code></pre>
<img src="out_runed/utilities/use-search-params/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

## Contributors

<a href="https://github.com/huntabyte" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/use-search-params/index_media/75bcb1d595b8d37466d561f08b890ffc8f1d5425.jpg" id="bits-308" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="Hunter Johnston" />

Hunter Johnston<a href="https://github.com/wd-David" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/use-search-params/index_media/90cd387ff9a5aa1590e09e531ec7d97a0908ca8b.jpg" id="bits-310" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="David Peng" />

David Peng<a href="https://github.com/techniq" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/use-search-params/index_media/4df82d649af69ede5022dbd05f0c601727693146.png" id="bits-312" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="Sean Lynch" />

Sean Lynch<a href="https://github.com/fseda" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/use-search-params/index_media/c93fe3ebcfe06a448ecbd023c32df68b41ba61ce.jpg" id="bits-314" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="Felipe Seda" />

Felipe Seda

© 2025 <a href="https://github.com/svecosystem" class="underline-offset-2 hover:underline" target="_blank">Svecosystem</a>

<a href="https://github.com/svecosystem/runed" class="inline-flex select-none items-center justify-center gap-2 whitespace-nowrap text-sm font-semibold tracking-wide active:scale-[99%] disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:size-4 [&amp;_svg]:shrink-0 text-foreground hover:dark:bg-foreground/5 hover:dark:ring-foreground/5 hover:bg-foreground/2 hover:ring-foreground/2 bg-transparent ring ring-transparent h-10 w-10 rounded-md [&amp;&gt;svg]:size-5" aria-label="View this project on GitHub" rel="noopener noreferrer" target="_blank"><img src="out_runed/utilities/use-search-params/index_media/1d22f722c87a133f2381965da1acf7a54aefc45a.svg" /></a>

<img src="out_runed/utilities/use-search-params/index_media/b2f1f2409c2d7a3eaec75643beb67d5092b71fb2.svg" class="size-6" />

# useSearchParams

Reactive, schema-validated URL search params for Svelte/SvelteKit

`useSearchParams` provides a reactive, type-safe, and schema-driven way to manage URL search parameters in Svelte/SvelteKit apps. It supports validation, default values, compression, debouncing, and history control.

## <a href="https://runed.dev/docs/utilities/use-search-params#demo" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Demo

Add Field

<img src="out_runed/utilities/use-search-params/index_media/a08f0efc16d25c94f055dec10bc7c24422bffe7b.svg" class="size-6" />

##### Note

You can refresh this page and/or open it in another tab to see the count state being persisted and synchronized across sessions and tabs.

## <a href="https://runed.dev/docs/utilities/use-search-params#requirements" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Requirements

- **`@sveltejs/kit`** must be installed in your project.
- Uses <a href="https://standardschema.dev/" class="text-brand-link hover:text-brand-link-hover leading-7" rel="noopener noreferrer" target="_blank">Standard Schema</a> for schema validation and type inference.

## <a href="https://runed.dev/docs/utilities/use-search-params#usage" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Usage

Define schema:

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-theme="github-light github-dark" tabindex="0"><code>
        // schemas.ts
import { z } from &quot;zod&quot;;
// Version 3.24.0+
 
export const productSearchSchema = z.object({
    page: z.coerce.number().default(1),
    filter: z.string().default(&quot;&quot;),
    sort: z.enum([&quot;newest&quot;, &quot;oldest&quot;, &quot;price&quot;]).default(&quot;newest&quot;)
});
    </code></pre>
<img src="out_runed/utilities/use-search-params/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

In your svelte code:

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="svelte" data-theme="github-light github-dark" tabindex="0"><code>
        &lt;script lang=&quot;ts&quot;&gt;
import { useSearchParams } from &quot;runed/kit&quot;;
import { productSearchSchema } from &#39;./schemas&#39;;
 
const params = useSearchParams(productSearchSchema);
// Access parameters directly
const page = $derived(params.page); // number (defaults to 1)
const sort = $derived(params.sort); // &#39;newest&#39; | &#39;oldest&#39; | &#39;price&#39;
 
// Update parameters directly
params.page = 2; // Updates URL to include ?page=2
// Updates URL to include ?page=3&amp;sort=oldest
params.update({ page: 3, sort: &#39;oldest&#39; });
// Resets all parameters to their default values
params.reset();
// Returns URLSearchParams object with all current parameter values
params.toURLSearchParams();
/**
 * You can watch for changes to the URLSearchParams object
 * For example:
 * watch(() =&gt; params.toURLSearchParams(), () =&gt; {
 *   // Do something whenever the URLSearchParams object changes
 * })
 */
&lt;/script&gt;
 
&lt;-- Great for binding to input fields --&gt;
&lt;input type=&quot;text&quot; bind:value={params.filter} /&gt;
    </code></pre>
<img src="out_runed/utilities/use-search-params/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

In your load function:

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-theme="github-light github-dark" tabindex="0"><code>
        import { validateSearchParams } from &quot;runed/kit&quot;;
import { productSearchSchema } from &quot;./schemas&quot;;
 
export const load = ({ url, fetch }) =&gt; {
    // Get validated search params as URLSearchParams object
    // If you use a custom compressedParamName in useSearchParams, provide it here too:
    const { searchParams } = validateSearchParams(url, productSearchSchema);
 
    // Use URLSearchParams directly with fetch
    const response = await fetch(`/api/products?${searchParams.toString()}`);
    return {
        products: await response.json()
    };
};
    </code></pre>
<img src="out_runed/utilities/use-search-params/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

## <a href="https://runed.dev/docs/utilities/use-search-params#features" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Features

- **Schema Validation**: Define types, defaults, and structure for each param.
- **Default Values**: Missing params are auto-filled with defaults.
- **Type Safety**: All values are parsed and validated to the schema.
- **Compression**: Store all params in a single compressed `_data` param for cleaner URLs.
- **Debounce**: Optionally debounce updates for smoother UX.
- **History Control**: Choose between push/replace state.
- **In-memory Mode**: Params are kept in memory, not in the URL.
- **Invalid Param Handling**: Invalid values are replaced with defaults and removed from the URL.

### <a href="https://runed.dev/docs/utilities/use-search-params#usesearchparams" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>`useSearchParams`

Hook to create a reactive search params object with property access

This client-side hook automatically updates the URL when parameters change. It provides type-safe access to URL search parameters through direct property access.

**Parameters:**

- `schema`: A validation schema compatible with StandardSchemaV1
- `options`: Configuration options that affect URL behavior

**Returns:**

- A reactive object for working with typed search parameters

#### <a href="https://runed.dev/docs/utilities/use-search-params#available-options" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Available options:

- `showDefaults` (boolean): When true, parameters with default values will be shown in the URL. When false (default), parameters with default values will be omitted from the URL.
- `debounce` (number): Milliseconds to delay URL updates when parameters change. Useful to avoid cluttering browser history when values change rapidly (default: 0, no debounce).
- `pushHistory` (boolean): Controls whether URL updates create new browser history entries. If true (default), each update adds a new entry to the browser history. If false, updates replace the current URL without creating new history entries.
- `compress` (boolean): When true, all parameters are compressed into a single parameter using lz-string compression. This helps reduce URL length and provides basic obfuscation (default: false). Use validateSearchParams with the same compressedParamName option when handling compressed URLs server-side.
- `compressedParamName` (string): The name of the parameter used to store compressed data when compression is enabled. Customize this to avoid conflicts with parameters in your schema. Default is `_data`.
- `updateURL` (boolean): When true (default), the URL is updated when parameters change. When false, only in-memory parameters are updated.
- `noScroll` (boolean): When true, the scroll position is preserved when the URL is updated. This prevents the page from jumping to the top on URL changes. Default is false.

Example with <a href="https://zod.dev/" class="text-brand-link hover:text-brand-link-hover leading-7" rel="noopener noreferrer" target="_blank">Zod</a>:

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-theme="github-light github-dark" tabindex="0"><code>
        import { z } from &quot;zod&quot;;
 
const productSearchSchema = z.object({
    page: z.number().default(1),
    filter: z.string().default(&quot;&quot;),
    sort: z.enum([&quot;newest&quot;, &quot;oldest&quot;, &quot;price&quot;]).default(&quot;newest&quot;)
});
const params = useSearchParams(productSearchSchema);
// Access parameters directly
const page = $derived(params.page); // number (defaults to 1)
const sort = $derived(params.sort); // &#39;newest&#39; | &#39;oldest&#39; | &#39;price&#39;
    </code></pre>
<img src="out_runed/utilities/use-search-params/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

Example with options:

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-theme="github-light github-dark" tabindex="0"><code>
        // Show default values in URL, debounce updates by 300ms,
// don&#39;t create new history entries, and compress params
const params = useSearchParams(schema, {
  showDefaults: true,
  debounce: 300,
  pushHistory: false,
  compress: true,
  compressedParamName: &#39;_compressed&#39; // Custom name to avoid conflicts
});
// Great for binding to input fields (updates URL without cluttering history)
&lt;input type=&quot;text&quot; bind:value={params.search} /&gt;
// Resulting URL will be something like: /?_compressed=N4IgDgTg9g...
    </code></pre>
<img src="out_runed/utilities/use-search-params/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

Example with <a href="https://valibot.dev/" class="text-brand-link hover:text-brand-link-hover leading-7" rel="noopener noreferrer" target="_blank">Valibot</a>:

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-theme="github-light github-dark" tabindex="0"><code>
        import * as v from &quot;valibot&quot;;
const productSearchSchema = v.object({
    page: v.optional(v.fallback(v.number(), 1), 1),
    filter: v.optional(v.fallback(v.string(), &quot;&quot;), &quot;&quot;),
    sort: v.optional(v.fallback(v.picklist([&quot;newest&quot;, &quot;oldest&quot;, &quot;price&quot;]), &quot;newest&quot;), &quot;newest&quot;)
});
const params = useSearchParams(productSearchSchema);
    </code></pre>
<img src="out_runed/utilities/use-search-params/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

Example with <a href="https://arktype.io/" class="text-brand-link hover:text-brand-link-hover leading-7" rel="noopener noreferrer" target="_blank">Arktype</a>:

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-theme="github-light github-dark" tabindex="0"><code>
        import { type } from &quot;arktype&quot;;
const productSearchSchema = type({
    page: &quot;number = 1&quot;,
    filter: &#39;string = &quot;&quot;&#39;,
    sort: &#39;&quot;newest&quot; | &quot;oldest&quot; | &quot;price&quot; = &quot;newest&quot;&#39;
});
const params = useSearchParams(productSearchSchema);
    </code></pre>
<img src="out_runed/utilities/use-search-params/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

Or with our built-in schema creator (no additional dependencies)

### <a href="https://runed.dev/docs/utilities/use-search-params#createsearchparamsschema" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>`createSearchParamsSchema`

Creates a simple schema compatible with <a href="https://standardschema.dev/" class="text-brand-link hover:text-brand-link-hover leading-7" rel="noopener noreferrer" target="_blank">Standard Schema</a> without requiring external validation libraries.

This is a lightweight alternative to using full schema validation libraries like Zod, Valibot, or Arktype. Use this when you need basic type conversion and default values without adding dependencies.

Limitations:

- For 'array' type: supports basic arrays, but doesn't validate array items
- For 'object' type: supports generic objects, but doesn't validate nested properties
- No custom validation rules or transformations
- **No granular reactivity**: Changes to nested properties require reassigning the entire value
  - ❌ `params.config.theme = 'dark'` (won't trigger URL update)
  - ✅ `params.config = {...params.config, theme: 'dark'}` (will trigger URL update)

For complex validation needs (nested validation, refined rules, etc.), use a dedicated validation library instead.

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-theme="github-light github-dark" tabindex="0"><code>
        const productSearchSchema = createSearchParamsSchema({
    // Basic types with defaults
    page: { type: &quot;number&quot;, default: 1 },
    filter: { type: &quot;string&quot;, default: &quot;&quot; },
    sort: { type: &quot;string&quot;, default: &quot;newest&quot; },
 
    // Array type with specific element type
    tags: {
        type: &quot;array&quot;,
        default: [&quot;new&quot;],
        arrayType: &quot;&quot; // Specify string[] type
    },
 
    // Object type with specific shape
    config: {
        type: &quot;object&quot;,
        default: { theme: &quot;light&quot; },
        objectType: { theme: &quot;&quot; } // Specify { theme: string } type
    }
});
    </code></pre>
<img src="out_runed/utilities/use-search-params/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

URL storage format:

- Arrays are stored as JSON strings: `?tags=["sale","featured"]`
- Objects are stored as JSON strings: `?config={"theme":"dark","fontSize":14}`
- Dates are stored as ISO8601 strings: `?createdAt=2023-12-01T10:30:00.000Z` (or `YYYY-MM-DD` with date-only format)
- Primitive values are stored directly: `?page=2&filter=red`

#### <a href="https://runed.dev/docs/utilities/use-search-params#date-format-support" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Date Format Support

You can control how Date parameters are serialized in URLs using two approaches:

**Option 1: Using `dateFormat` property in schema**

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-theme="github-light github-dark" tabindex="0"><code>
        const schema = createSearchParamsSchema({
    // Date-only format (YYYY-MM-DD) - great for birth dates, event dates
    birthDate: {
        type: &quot;date&quot;,
        default: new Date(&quot;1990-01-15&quot;),
        dateFormat: &quot;date&quot;
    },
 
    // Full datetime format (ISO8601) - great for timestamps
    createdAt: {
        type: &quot;date&quot;,
        default: new Date(),
        dateFormat: &quot;datetime&quot;
    },
 
    // No format specified - defaults to &#39;datetime&#39;
    updatedAt: {
        type: &quot;date&quot;,
        default: new Date()
    }
});
 
const params = useSearchParams(schema);
// URL: ?birthDate=1990-01-15&amp;createdAt=2023-01-01T10:30:00.000Z&amp;updatedAt=2023-12-25T18:30:00.000Z
    </code></pre>
<img src="out_runed/utilities/use-search-params/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

**Option 2: Using `dateFormats` option (works with any validator)**

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-theme="github-light github-dark" tabindex="0"><code>
        // Works with Zod, Valibot, Arktype, or createSearchParamsSchema
const params = useSearchParams(zodSchema, {
    dateFormats: {
        birthDate: &quot;date&quot;, // YYYY-MM-DD
        createdAt: &quot;datetime&quot; // ISO8601
    }
});
    </code></pre>
<img src="out_runed/utilities/use-search-params/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

**Date Format Details:**

- **`'date'` format**: Serializes as `YYYY-MM-DD` (e.g., `2025-10-21`)
  - More readable in URLs
  - Perfect for calendar dates, birth dates, event dates
  - Parsed as Date object with time set to midnight UTC
- **`'datetime'` format** (default): Serializes as full ISO8601 (e.g., `2025-10-21T18:18:14.196Z`)
  - Preserves exact time information
  - Perfect for timestamps, created/updated times
  - Full precision date and time

**Practical Example:**

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="svelte" data-theme="github-light github-dark" tabindex="0"><code>
        &lt;script lang=&quot;ts&quot;&gt;
    import { useSearchParams, createSearchParamsSchema } from &quot;runed/kit&quot;;
 
    const schema = createSearchParamsSchema({
        eventDate: {
            type: &quot;date&quot;,
            default: new Date(&quot;2025-01-01&quot;),
            dateFormat: &quot;date&quot;
        },
        createdAt: {
            type: &quot;date&quot;,
            default: new Date(),
            dateFormat: &quot;datetime&quot;
        }
    });
 
    const params = useSearchParams(schema);
&lt;/script&gt;
 
&lt;label&gt;
    Event Date:
    &lt;input
        type=&quot;date&quot;
        value={params.eventDate.toISOString().split(&quot;T&quot;)[0]}
        oninput={(e) =&gt; (params.eventDate = new Date(e.target.value))} /&gt;
&lt;/label&gt;
 
&lt;label&gt;
    Created At:
    &lt;input
        type=&quot;datetime-local&quot;
        value={params.createdAt.toISOString().slice(0, 16)}
        oninput={(e) =&gt; (params.createdAt = new Date(e.target.value))} /&gt;
&lt;/label&gt;
 
&lt;!-- URL will be: ?eventDate=2025-01-01&amp;createdAt=2025-10-21T18:18:14.196Z --&gt;
    </code></pre>
<img src="out_runed/utilities/use-search-params/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

### <a href="https://runed.dev/docs/utilities/use-search-params#validatesearchparams" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>`validateSearchParams`

A utility function to extract, validate and convert URL search parameters to <a href="https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams" class="text-brand-link hover:text-brand-link-hover leading-7" rel="noopener noreferrer" target="_blank">URLSearchParams</a>

This function makes it easy to use the same schema validation in both client-side components (via `useSearchParams`) and server-side load functions. Unlike \`useSearchParams, this function doesn't modify the URL - it only validates parameters and returns them as a new URLSearchParams object.

Handles both standard URL parameters and compressed parameters (when compression is enabled).

**Parameters:**

- `url`: The URL object from SvelteKit load function
- `schema`: A validation schema (createSearchParamsSchema, Zod, Valibot, etc.)
- `options`: Optional configuration (like custom `compressedParamName` and `dateFormats`)

**Returns:**

- An object with `searchParams` and `data` properties, `searchParams` being the validated `URLSearchParams` and `data` being the validated object

**Available options:**

- `compressedParamName` (string): Custom name for compressed parameter (default: `_data`)
- `dateFormats` (object): Map of field names to date formats (`'date'` or `'datetime'`)

Example with SvelteKit page or layout load function:

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-theme="github-light github-dark" tabindex="0"><code>
        import { validateSearchParams } from &quot;runed/kit&quot;;
import { productSchema } from &quot;./schemas&quot;;
 
export const load = ({ url, fetch }) =&gt; {
    // Get validated search params as URLSearchParams object
    // If you use a custom compressedParamName in useSearchParams, provide it here too:
    const { searchParams } = validateSearchParams(url, productSchema, {
        compressedParamName: &quot;_compressed&quot;,
        dateFormats: {
            birthDate: &quot;date&quot;, // Serialize as YYYY-MM-DD
            createdAt: &quot;datetime&quot; // Serialize as ISO8601
        }
    });
 
    // Use URLSearchParams directly with fetch
    const response = await fetch(`/api/products?${searchParams.toString()}`);
    return {
        products: await response.json()
    };
};
    </code></pre>
<img src="out_runed/utilities/use-search-params/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

### <a href="https://runed.dev/docs/utilities/use-search-params#advanced-custom-serialization-with-zod-codecs" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Advanced: Custom Serialization with Zod Codecs

For advanced use cases where you need full control over how values are converted between URL strings and JavaScript types, you can use <a href="https://zod.dev/?id=codec" class="text-brand-link hover:text-brand-link-hover leading-7" rel="noopener noreferrer" target="_blank">Zod codecs</a> (Zod v4.1.0+). Codecs allow you to define custom bidirectional transformations that work seamlessly with URL parameters.

#### <a href="https://runed.dev/docs/utilities/use-search-params#why-use-codecs" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Why Use Codecs?

While the built-in `dateFormats` option works well for common cases, codecs give you complete control over serialization. Use codecs when you need to:

- **Custom date formats**: Store dates as Unix timestamps, custom date strings, or other formats
- **Complex type conversions**: Convert between incompatible types (e.g., number IDs ↔ full objects)
- **Data transformation**: Apply transformations during serialization (e.g., normalize, encrypt)
- **Legacy API compatibility**: Match existing URL parameter formats from other systems
- **Optimization**: Use more compact representations (e.g., `1234567890` instead of `2009-02-13T23:31:30.000Z`)

#### <a href="https://runed.dev/docs/utilities/use-search-params#how-codecs-work" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>How Codecs Work

A Zod codec defines two transformations:

- **`decode`**: Converts URL string → JavaScript type (when reading from URL)
- **`encode`**: Converts JavaScript type → URL string (when writing to URL)

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-theme="github-light github-dark" tabindex="0"><code>
        import { z } from &quot;zod&quot;;
 
// Example 1: Unix timestamp codec (stores Date as number)
const unixTimestampCodec = z.codec(
    z.coerce.number(), // Input: number from URL string
    z.date(), // Output: Date object in your app
    {
        decode: (timestamp) =&gt; new Date(timestamp * 1000), // number → Date
        encode: (date) =&gt; Math.floor(date.getTime() / 1000) // Date → number
    }
);
 
// Example 2: Date-only codec (stores Date as YYYY-MM-DD)
const dateOnlyCodec = z.codec(
    z.string(), // Input: string from URL
    z.date(), // Output: Date object in your app
    {
        decode: (str) =&gt; new Date(str + &quot;T00:00:00.000Z&quot;), // &quot;2025-01-15&quot; → Date
        encode: (date) =&gt; date.toISOString().split(&quot;T&quot;)[0] // Date → &quot;2025-01-15&quot;
    }
);
 
// Example 3: Product ID codec (stores number as base36 string for shorter URLs)
const compactIdCodec = z.codec(
    z.string(), // Input: base36 string from URL
    z.number(), // Output: number in your app
    {
        decode: (str) =&gt; parseInt(str, 36), // &quot;abc123&quot; → 225249695
        encode: (num) =&gt; num.toString(36) // 225249695 → &quot;abc123&quot;
    }
);
    </code></pre>
<img src="out_runed/utilities/use-search-params/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

#### <a href="https://runed.dev/docs/utilities/use-search-params#using-codecs-in-your-schema" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Using Codecs in Your Schema

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-theme="github-light github-dark" tabindex="0"><code>
        import { z } from &quot;zod&quot;;
 
const searchSchema = z.object({
    // Regular fields work as before
    query: z.string().default(&quot;&quot;),
    page: z.coerce.number().default(1),
 
    // Unix timestamp - more compact than ISO datetime
    createdAfter: unixTimestampCodec.default(new Date(&quot;2024-01-01&quot;)),
 
    // Date-only format - cleaner for calendar dates
    birthDate: dateOnlyCodec.default(new Date(&quot;1990-01-15&quot;)),
 
    // Compact product IDs
    productId: compactIdCodec.optional()
});
 
const params = useSearchParams(searchSchema);
    </code></pre>
<img src="out_runed/utilities/use-search-params/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

#### <a href="https://runed.dev/docs/utilities/use-search-params#real-world-example-event-search" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Real-World Example: Event Search

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="svelte" data-theme="github-light github-dark" tabindex="0"><code>
        &lt;script lang=&quot;ts&quot;&gt;
    import { z } from &quot;zod&quot;;
    import { useSearchParams } from &quot;runed/kit&quot;;
 
    // Define reusable codecs
    const unixTimestamp = z.codec(z.coerce.number(), z.date(), {
        decode: (ts) =&gt; new Date(ts * 1000),
        encode: (date) =&gt; Math.floor(date.getTime() / 1000)
    });
 
    const dateOnly = z.codec(z.string(), z.date(), {
        decode: (str) =&gt; new Date(str + &quot;T00:00:00.000Z&quot;),
        encode: (date) =&gt; date.toISOString().split(&quot;T&quot;)[0]
    });
 
    // Schema with multiple codec types
    const eventSearchSchema = z.object({
        query: z.string().default(&quot;&quot;),
        // Date-only for event date (more readable in URL)
        eventDate: dateOnly.default(new Date()),
        // Unix timestamp for filters (more compact)
        createdAfter: unixTimestamp.optional(),
        updatedSince: unixTimestamp.optional()
    });
 
    const params = useSearchParams(eventSearchSchema);
 
    // Work with Date objects in your app
    $inspect(params.eventDate); // Date object
    $inspect(params.createdAfter); // Date object or undefined
&lt;/script&gt;
 
&lt;!-- Bind to native date inputs --&gt;
&lt;label&gt;
    Event Date:
    &lt;input
        type=&quot;date&quot;
        value={params.eventDate.toISOString().split(&quot;T&quot;)[0]}
        oninput={(e) =&gt; (params.eventDate = new Date(e.target.value))} /&gt;
&lt;/label&gt;
 
&lt;label&gt;
    Created After:
    &lt;input
        type=&quot;date&quot;
        value={params.createdAfter?.toISOString().split(&quot;T&quot;)[0] ?? &quot;&quot;}
        oninput={(e) =&gt;
            (params.createdAfter = e.target.value ? new Date(e.target.value) : undefined)} /&gt;
&lt;/label&gt;
 
&lt;!-- Clean URLs:
     Without codecs: ?eventDate=2025-01-15T00:00:00.000Z&amp;createdAfter=2024-01-01T00:00:00.000Z
     With codecs:    ?eventDate=2025-01-15&amp;createdAfter=1704067200
--&gt;
    </code></pre>
<img src="out_runed/utilities/use-search-params/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

#### <a href="https://runed.dev/docs/utilities/use-search-params#codec-benefits-summary" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Codec Benefits Summary

| Feature | Built-in dateFormats | Zod Codecs |
|----|----|----|
| **Setup complexity** | Simple | More configuration needed |
| **Date formats** | `date` and `datetime` | Any custom format (Unix, relative, custom string) |
| **URL size** | Standard | Can be optimized (e.g., Unix timestamps) |
| **Type conversions** | Date only | Any type (numbers, objects, arrays, etc.) |
| **Validation** | Basic | Full Zod validation + transformation |
| **Reusability** | Per-field config | Create reusable codec definitions |
| **Legacy compatibility** | Limited | Full control over format |
| **Works with validators** | All (Zod, Valibot, etc.) | Zod only (v4.1.0+) |
| **Server-side usage** | Use `dateFormats` option | Automatic with `validateSearchParams` |

**When to use `dateFormats`**: Most applications with standard date handling needs

**When to use codecs**: When you need custom formats, compact representations, or complex type conversions

#### <a href="https://runed.dev/docs/utilities/use-search-params#server-side-usage-with-codecs" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Server-Side Usage with Codecs

Codecs work automatically with `validateSearchParams`:

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-theme="github-light github-dark" tabindex="0"><code>
        // +page.server.ts
import { validateSearchParams } from &quot;runed/kit&quot;;
import { eventSearchSchema } from &quot;./schemas&quot;; // Schema with codecs
 
export const load = ({ url }) =&gt; {
    // Codecs are automatically applied during validation
    const { searchParams, data } = validateSearchParams(url, eventSearchSchema);
 
    // data.eventDate is a Date object (decoded from URL string)
    // searchParams contains properly encoded values for API calls
    return {
        events: await fetchEvents(searchParams)
    };
};
    </code></pre>
<img src="out_runed/utilities/use-search-params/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

## <a href="https://runed.dev/docs/utilities/use-search-params#reactivity-limitations" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Reactivity Limitations

### <a href="https://runed.dev/docs/utilities/use-search-params#understanding-reactivity-scope" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Understanding Reactivity Scope

`useSearchParams` provides **top-level reactivity only**. This means:

✅ **Works (Direct property assignment)**:

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="svelte" data-theme="github-light github-dark" tabindex="0"><code>
        &lt;script&gt;
    const params = useSearchParams(schema);
 
    // These trigger URL updates
    params.page = 2;
    params.filter = &quot;active&quot;;
    params.config = { theme: &quot;dark&quot;, size: &quot;large&quot; };
    params.items = [...params.items, newItem];
&lt;/script&gt;
    </code></pre>
<img src="out_runed/utilities/use-search-params/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

❌ **Doesn't work (Nested property mutations)**:

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="svelte" data-theme="github-light github-dark" tabindex="0"><code>
        &lt;script&gt;
    const params = useSearchParams(schema);
 
    // These DON&#39;T trigger URL updates
    params.config.theme = &quot;dark&quot;; // Nested object property
    params.items.push(newItem); // Array method
    params.items[0].name = &quot;updated&quot;; // Array item property
    delete params.config.oldProp; // Property deletion
&lt;/script&gt;
    </code></pre>
<img src="out_runed/utilities/use-search-params/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

### <a href="https://runed.dev/docs/utilities/use-search-params#why-this-design-choice" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Why This Design Choice

`useSearchParams` was designed to prioritize **simplicity, type safety, and ease of use** over deep reactivity. This design choice offers several benefits:

#### <a href="https://runed.dev/docs/utilities/use-search-params#-what-you-get" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>✅ **What You Get**

- **Simple, predictable API**: `params.page = 2` always works
- **Full TypeScript support**: Perfect autocomplete and type checking
- **Clean URLs**: Objects serialize to readable JSON strings
- **Performance**: No overhead from tracking deep object changes
- **Reliability**: No edge cases with complex nested proxy behaviors

## <a href="https://runed.dev/docs/utilities/use-search-params#type-definitions" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Type Definitions

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-theme="github-light github-dark" tabindex="0"><code>
        interface SearchParamsOptions {
    /**
     * If true, parameters set to their default values will be shown in the URL.
     * If false, parameters with default values will be omitted from the URL.
     * @default false
     */
    showDefaults?: boolean;
 
    /**
     * The number of milliseconds to delay URL updates when parameters change.
     * This helps avoid cluttering browser history when values change rapidly
     * (like during typing in an input field).
     * @default 0 (no debounce)
     */
    debounce?: number;
 
    /**
     * Controls whether URL updates create new browser history entries.
     * If true (default), each update adds a new entry to the browser history.
     * If false, updates replace the current URL without creating new history entries.
     * @default true
     */
    pushHistory?: boolean;
 
    /**
     * Enable lz-string compression for all parameters.
     * When true, all parameters are compressed into a single parameter in the URL.
     * This helps reduce URL length and provides basic parameter obfuscation.
     * @default false
     */
    compress?: boolean;
 
    /**
     * The name of the parameter used to store compressed data when compression is enabled.
     * You can customize this to avoid conflicts with your schema parameters.
     * For example, if your schema already uses &#39;_data&#39;, you might want to use &#39;_compressed&#39; or another unique name.
     * @default &#39;_data&#39;
     */
    compressedParamName?: string;
 
    /**
     * Controls whether to update the URL when parameters change.
     * If true (default), changes to parameters will update the URL.
     * If false, parameters will only be stored in memory without updating the URL.
     * Note: When false, compress option will be ignored.
     * @default true
     */
    updateURL?: boolean;
 
    /**
     * If true, the page will not scroll to the top when the URL is updated.
     * This is useful if you want to maintain the user&#39;s scroll position during parameter changes.
     * @default false
     */
    noScroll?: boolean;
 
    /**
     * Specifies which date fields should use date-only format (YYYY-MM-DD) instead of full ISO8601 datetime.
     *
     * Map field names to their desired format:
     * - &#39;date&#39;: Serializes as YYYY-MM-DD (e.g., &quot;2025-10-21&quot;)
     * - &#39;datetime&#39;: Serializes as full ISO8601 (e.g., &quot;2025-10-21T18:18:14.196Z&quot;)
     *
     * Example:
     * { dateFormats: { birthDate: &#39;date&#39;, createdAt: &#39;datetime&#39; } }
     *
     * @default undefined (all dates use datetime format)
     */
    dateFormats?: Record&lt;string, &quot;date&quot; | &quot;datetime&quot;&gt;;
}
 
type ReturnUseSearchParams&lt;Schema extends StandardSchemaV1&gt; = {
    [key: string]: any; // Typed, reactive params
    /**
     * Update multiple parameters at once
     *
     * This is more efficient than setting multiple parameters individually
     * because it only triggers one URL update or one in-memory store update.
     *
     * @param values An object containing parameter key-value pairs to update.
     *  For example: params.update({ page: 1, sort: &#39;newest&#39; })
     */
    update(values: Partial&lt;StandardSchemaV1.InferOutput&lt;Schema&gt;&gt;): void;
    /**
     * Reset all parameters to their default values
     *
     * This method removes all current URL parameters or in-memory parameters
     * and optionally sets parameters with non-default values back to their defaults.
     *
     * @param showDefaults Whether to show default values in the URL or in-memory store after reset.
     * If not provided, uses the instance&#39;s showDefaults option.
     */
    reset(showDefaults?: boolean): void;
    /**
     * Convert the current schema parameters to a URLSearchParams object
     * This includes all values defined in the schema, regardless of their presence in the URL
     * @returns URLSearchParams object containing all current parameter values
     */
    toURLSearchParams(): URLSearchParams;
};
 
/**
 * Schema type for createSearchParamsSchema
 * Allows specifying more precise types for arrays, objects, and dates
 */
type SchemaTypeConfig&lt;ArrayType = unknown, ObjectType = unknown&gt; =
    | { type: &quot;string&quot;; default?: string }
    | { type: &quot;number&quot;; default?: number }
    | { type: &quot;boolean&quot;; default?: boolean }
    | { type: &quot;array&quot;; default?: ArrayType[]; arrayType?: ArrayType }
    | { type: &quot;object&quot;; default?: ObjectType; objectType?: ObjectType }
    | { type: &quot;date&quot;; default?: Date; dateFormat?: &quot;date&quot; | &quot;datetime&quot; };
    </code></pre>
<img src="out_runed/utilities/use-search-params/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

## Contributors

<a href="https://github.com/huntabyte" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/use-search-params/index_media/75bcb1d595b8d37466d561f08b890ffc8f1d5425.jpg" id="bits-308" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="Hunter Johnston" />

Hunter Johnston<a href="https://github.com/wd-David" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/use-search-params/index_media/90cd387ff9a5aa1590e09e531ec7d97a0908ca8b.jpg" id="bits-310" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="David Peng" />

David Peng<a href="https://github.com/techniq" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/use-search-params/index_media/4df82d649af69ede5022dbd05f0c601727693146.png" id="bits-312" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="Sean Lynch" />

Sean Lynch<a href="https://github.com/fseda" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/use-search-params/index_media/c93fe3ebcfe06a448ecbd023c32df68b41ba61ce.jpg" id="bits-314" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="Felipe Seda" />

Felipe Seda
