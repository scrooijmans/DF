# Codecs | Zod

All Zod schemas can process inputs in both the forward and backward direction:

- **Forward**: `Input` to `Output`
  - `.parse()`
  - `.decode()`
- **Backward**: `Output` to `Input`
  - `.encode()`

In most cases, this is a distinction without a difference. The input and output types are identical, so there's no difference between "forward" and "backward".

However, some schema types cause the input and output types to diverge, notably `z.codec()`. Codecs are a special type of schema that defines a _bi-directional transformation_ between two other schemas.

In these cases, `z.decode()` and `z.encode()` behave quite differently.

**Note** —There's nothing special about the directions or terminology here. Instead of _encoding_ with an `A -> B` codec, you could instead _decode_ with a `B -> A` codec. The use of the terms "decode" and "encode" is just a convention.

This is particularly useful when parsing data at a network boundary. You can share a single Zod schema between your client and server, then use this single schema to convert between a network-friendly format (say, JSON) and a richer JavaScript representation.

![Codecs encoding and decoding data across a network boundary](https://zod.dev/codecs/codecs-network-light.svg)![Codecs encoding and decoding data across a network boundary](https://zod.dev/codecs/codecs-network-dark.svg)

### [Composability](?id=composability)

**Note** — You can use `z.encode()` and `z.decode()` with any schema. It doesn't have to be a ZodCodec.

Codecs are a schema like any other. You can nest them inside objects, arrays, pipes, etc. There are no rules on where you can use them!

### [Type-safe inputs](?id=type-safe-inputs)

While `.parse()` and `.decode()` behave identically at _runtime_, they have different type signatures. The `.parse()` method accepts `unknown` as input, and returns a value that matches the schema's inferred _output type_. By constrast, the `z.decode()` and `z.encode()` functions have _strongly-typed inputs_.

Why the difference? Encoding and decoding imply _transformation_. In many cases, the inputs to these methods Here's a diagram demonstrating the differences between the type signatures for `parse()`, `decode()`, and `encode()`.

![Codec directionality diagram showing bidirectional transformation between input and output schemas](https://zod.dev/_next/image?url=%2Fcodecs%2Fcodecs-light.png&w=1920&q=100)![Codec directionality diagram showing bidirectional transformation between input and output schemas](https://zod.dev/_next/image?url=%2Fcodecs%2Fcodecs-dark.png&w=1920&q=100)

### [Async and safe variants](?id=async-and-safe-variants)

As with `.transform()` and `.refine()`, codecs support async transforms.

As with regular `parse()`, there are "safe" and "async" variants of `decode()` and `encode()`.

There are some subtleties to how certain Zod schemas "reverse" their parse behavior.

### [Codecs](?id=codecs)

This one is fairly self-explanatory. Codecs encapsulate a bi-directional transformation between two types. During `z.decode()`, the `decode` transform is executed. During `z.encode()`, the `encode` transform is executed.

### [Pipes](?id=pipes)

**Fun fact** — Codecs are actually implemented internally as _subclass_ of pipes that have been augmented with "interstitial" transform logic.

During regular decoding, a `ZodPipe<A, B>` schema will first parse the data with `A`, then pass it into `B`. As you might expect, during encoding, the data is first encoded with `B`, then passed into `A`.

### [Refinements](?id=refinements)

All checks (`.refine()`, `.min()`, `.max()`, etc.) are still executed in both directions.

To avoid unexpected errors in your custom `.refine()` logic, Zod performs two "passes" during `z.encode()`. The first pass ensures the input type conforms to the expected type (no `invalid_type` errors). If that passes, Zod performs the second pass which executes the refinement logic.

This approach also supports "mutating transforms" like `z.string().trim()` or `z.string().toLowerCase()`:

### [Defaults and prefaults](?id=defaults-and-prefaults)

Defaults and prefaults are only applied in the "forward" direction.

When you attach a default value to a schema, the input becomes optional (`| undefined`) but the output does not. As such, `undefined` is not a valid input to `z.encode()` and defaults/prefaults will not be applied.

### [Catch](?id=catch)

Similarly, `.catch()` is only applied in the "forward" direction.

### [Stringbool](?id=stringbool)

**Note** — [Stringbool](about:/api#stringbool) pre-dates the introduction of codecs in Zod. It has since been internally re-implemented as a codec.

The `z.stringbool()` API converts string values (`"true"`, `"false"`, `"yes"`, `"no"`, etc.) into `boolean`. By default, it will convert `true` to `"true"` and `false` to `"false"` during `z.encode()`..

If you specify a custom set of `truthy` and `falsy` values, the _first element in the array_ will be used instead.

### [Transforms](?id=transforms)

⚠️ — The `.transform()` API implements a _unidirectional_ transformation. If any `.transform()` exists anywhere in your schema, attempting a `z.encode()` operation will throw a _runtime error_ (not a `ZodError`).

Below are implementations for a bunch of commonly-needed codecs. For the sake of customizability, these are not included as first-class APIs in Zod itself. Instead, you should copy/paste them into your project and modify them as needed.

**Note** — All of these codec implementations have been tested for correctness.

### [`stringToNumber`](?id=stringtonumber)

Converts string representations of numbers to JavaScript `number` type using `parseFloat()`.

### [`stringToInt`](?id=stringtoint)

Converts string representations of integers to JavaScript `number` type using `parseInt()`.

### [`stringToBigInt`](?id=stringtobigint)

Converts string representations to JavaScript `bigint` type.

### [`numberToBigInt`](?id=numbertobigint)

Converts JavaScript `number` to `bigint` type.

### [`isoDatetimeToDate`](?id=isodatetimetodate)

Converts ISO datetime strings to JavaScript `Date` objects.

### [`epochSecondsToDate`](?id=epochsecondstodate)

Converts Unix timestamps (seconds since epoch) to JavaScript `Date` objects.

### [`epochMillisToDate`](?id=epochmillistodate)

Converts Unix timestamps (milliseconds since epoch) to JavaScript `Date` objects.

### [`json(schema)`](?id=jsonschema)

Parses JSON strings into structured data and serializes back to JSON. This generic function accepts an output schema to validate the parsed JSON data.

Usage example with a specific schema:

### [`utf8ToBytes`](?id=utf8tobytes)

Converts UTF-8 strings to `Uint8Array` byte arrays.

### [`bytesToUtf8`](?id=bytestoutf8)

Converts `Uint8Array` byte arrays to UTF-8 strings.

### [`base64ToBytes`](?id=base64tobytes)

Converts base64 strings to `Uint8Array` byte arrays and vice versa.

### [`base64urlToBytes`](?id=base64urltobytes)

Converts base64url strings (URL-safe base64) to `Uint8Array` byte arrays.

### [`hexToBytes`](?id=hextobytes)

Converts hexadecimal strings to `Uint8Array` byte arrays and vice versa.

### [`stringToURL`](?id=stringtourl)

Converts URL strings to JavaScript `URL` objects.

### [`stringToHttpURL`](?id=stringtohttpurl)

Converts HTTP/HTTPS URL strings to JavaScript `URL` objects.

### [`uriComponent`](?id=uricomponent)

Encodes and decodes URI components using `encodeURIComponent()` and `decodeURIComponent()`.
