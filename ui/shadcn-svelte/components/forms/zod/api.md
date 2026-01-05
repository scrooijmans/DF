# Defining schemas | Zod

To validate data, you must first define a _schema_. Schemas represent _types_, from simple primitive values to complex nested objects and arrays.

### [Coercion](?id=coercion)

To coerce input data to the appropriate type, use `z.coerce` instead:

The coerced variant of these schemas attempts to convert the input value to the appropriate type.

The input type of these coerced schemas is `unknown` by default. To specify a more specific input type, pass a generic parameter:

Literal schemas represent a [literal type](https://www.typescriptlang.org/docs/handbook/2/everyday-types.html#literal-types), like `"hello world"` or `5`.

To represent the JavaScript literals `null` and `undefined`:

To allow multiple literal values:

To extract the set of allowed values from a literal schema:

Zod provides a handful of built-in string validation and transform APIs. To perform some common string validations:

To perform some simple string transforms:

To validate against some common string formats:

### [Emails](?id=emails)

To validate email addresses:

By default, Zod uses a comparatively strict email regex designed to validate normal email addresses containing common characters. It's roughly equivalent to the rules enforced by Gmail. To learn more about this regex, refer to [this post](https://colinhacks.com/essays/reasonable-email-regex).

To customize the email validation behavior, you can pass a custom regular expression to the `pattern` param.

Zod exports several useful regexes you could use.

### [UUIDs](?id=uuids)

To validate UUIDs:

To specify a particular UUID version:

The RFC 9562/4122 UUID spec requires the first two bits of byte 8 to be `10`. Other UUID-like identifiers do not enforce this constraint. To validate any UUID-like identifier:

### [URLs](?id=urls)

To validate any WHATWG-compatible URL:

As you can see this is quite permissive. Internally this uses the `new URL()` constructor to validate inputs; this behavior may differ across platforms and runtimes but it's the mostly rigorous way to validate URIs/URLs on any given JS runtime/engine.

To validate the hostname against a specific regex:

To validate the protocol against a specific regex, use the `protocol` param.

**Web URLs** — In many cases, you'll want to validate Web URLs specifically. Here's the recommended schema for doing so:

This restricts the protocol to `http`/`https` and ensures the hostname is a valid domain name with the `z.regexes.domain` regular expression:

To normalize URLs, use the `normalize` flag. This will overwrite the input value with the [normalized URL](https://chatgpt.com/share/6881547f-bebc-800f-9093-f5981e277c2c) returned by `new URL()`.

### [ISO datetimes](?id=iso-datetimes)

As you may have noticed, Zod string includes a few date/time related validations. These validations are regular expression based, so they are not as strict as a full date/time library. However, they are very convenient for validating user input.

The `z.iso.datetime()` method enforces ISO 8601; by default, no timezone offsets are allowed:

To allow timezone offsets:

To allow unqualified (timezone-less) datetimes:

To constrain the allowable time `precision`. By default, seconds are optional and arbitrary sub-second precision is allowed.

### [ISO dates](?id=iso-dates)

The `z.iso.date()` method validates strings in the format `YYYY-MM-DD`.

### [ISO times](?id=iso-times)

The `z.iso.time()` method validates strings in the format `HH:MM[:SS[.s+]]`. By default seconds are optional, as are sub-second deciams.

No offsets of any kind are allowed.

Use the `precision` parameter to constrain the allowable decimal precision.

### [IP addresses](?id=ip-addresses)

### [IP blocks (CIDR)](?id=ip-blocks-cidr)

Validate IP address ranges specified with [CIDR notation](https://en.wikipedia.org/wiki/Classless_Inter-Domain_Routing).

### [JWTs](?id=jwts)

Validate [JSON Web Tokens](https://jwt.io/).

### [Hashes](?id=hashes)

To validate cryptographic hash values:

By default, `z.hash()` expects hexadecimal encoding, as is conventional. You can specify a different encoding with the `enc` parameter:

### [Custom formats](?id=custom-formats)

To define your own string formats:

This schema will produce `"invalid_format"` issues, which are more descriptive than the `"custom"` errors produced by refinements or `z.custom()`.

To define a template literal schema:

The `z.templateLiteral` API can handle any number of string literals (e.g. `"hello"`) and schemas. Any schema with an inferred type that's assignable to `string | number | bigint | boolean | null | undefined` can be passed.

Use `z.number()` to validate numbers. It allows any finite number.

Zod implements a handful of number-specific validations:

If (for some reason) you want to validate `NaN`, use `z.nan()`.

To validate integers:

To validate BigInts:

Zod includes a handful of bigint-specific validations.

To validate boolean values:

Use `z.date()` to validate `Date` instances.

To customize the error message:

Zod provides a handful of date-specific validations.

Use `z.enum` to validate inputs against a fixed set of allowable _string_ values.

Careful — If you declare your string array as a variable, Zod won't be able to properly infer the exact values of each element.

To fix this, always pass the array directly into the `z.enum()` function, or use [`as const`](https://www.typescriptlang.org/docs/handbook/release-notes/typescript-3-4.html#const-assertions).

Enum-like object literals (`{ [key: string]: string | number }`) are supported.

You can also pass in an externally-declared TypeScript enum.

**Zod 4** — This replaces the `z.nativeEnum()` API in Zod 3.

Note that using TypeScript's `enum` keyword is [not recommended](https://www.totaltypescript.com/why-i-dont-like-typescript-enums).

### [`.enum`](?id=enum)

To extract the schema's values as an enum-like object:

### [`.exclude()`](?id=exclude)

To create a new enum schema, excluding certain values:

### [`.extract()`](?id=extract)

To create a new enum schema, extracting certain values:

In some cases (e.g. parsing environment variables) it's valuable to parse certain string "boolish" values to a plain `boolean` value. To support this, Zod 4 introduces `z.stringbool()`:

To customize the truthy and falsy values:

By default the schema is _case-insensitive_; all inputs are converted to lowercase before comparison to the `truthy`/`falsy` values. To make it case-sensitive:

To make a schema _optional_ (that is, to allow `undefined` inputs).

This returns a `ZodOptional` instance that wraps the original schema. To extract the inner schema:

To make a schema _nullable_ (that is, to allow `null` inputs).

This returns a `ZodNullable` instance that wraps the original schema. To extract the inner schema:

To make a schema _nullish_ (both optional and nullable):

Refer to the TypeScript manual for more about the concept of [nullish](https://www.typescriptlang.org/docs/handbook/release-notes/typescript-3-7.html#nullish-coalescing).

Zod aims to mirror TypeScript's type system one-to-one. As such, Zod provides APIs to represent the following special types:

No value will pass validation.

To define an object type:

By default, all properties are required. To make certain properties optional:

By default, unrecognized keys are _stripped_ from the parsed result:

### [`z.strictObject`](?id=zstrictobject)

To define a _strict_ schema that throws an error when unknown keys are found:

### [`z.looseObject`](?id=zlooseobject)

To define a _loose_ schema that allows unknown keys to pass through:

### [`.catchall()`](?id=catchall)

To define a _catchall schema_ that will be used to validate any unrecognized keys:

### [`.shape`](?id=shape)

To access the internal schemas:

### [`.keyof()`](?id=keyof)

To create a `ZodEnum` schema from the keys of an object schema:

### [`.extend()`](?id=extend)

To add additional fields to an object schema:

This API can be used to overwrite existing fields! Be careful with this power! If the two schemas share keys, B will override A.

**Alternative: spread syntax** — You can alternatively avoid `.extend()` altogether by creating a new object schema entirely. This makes the strictness level of the resulting schema visually obvious.

You can also use this to merge multiple objects in one go.

This approach has a few advantages:

1.  It uses language-level features ([spread syntax](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Spread_syntax)) instead of library-specific APIs
2.  The same syntax works in Zod and Zod Mini
3.  It's more `tsc`\-efficient — the `.extend()` method can be expensive on large schemas, and due to [a TypeScript limitation](https://github.com/microsoft/TypeScript/pull/61505) it gets quadratically more expensive when calls are chained
4.  If you wish, you can change the strictness level of the resulting schema by using `z.strictObject()` or `z.looseObject()`

### [`.safeExtend()`](?id=safeextend)

The `.safeExtend()` method works similarly to `.extend()`, but it won't let you overwrite an existing properly with a non-assignable schema. In other words, the result of `.safeExtend()` will have an inferred type that [`extends`](https://www.typescriptlang.org/docs/handbook/2/conditional-types.html#conditional-type-constraints) the original (in the TypeScript sense).

Use `.safeExtend()` to extend schemas that contain refinements. (Regular `.extend()` will throw an error when used on schemas with refinements.)

### [`.pick()`](?id=pick)

Inspired by TypeScript's built-in `Pick` and `Omit` utility types, Zod provides dedicated APIs for picking and omitting certain keys from an object schema.

Starting from this initial schema:

To pick certain keys:

### [`.omit()`](?id=omit)

To omit certain keys:

### [`.partial()`](?id=partial)

For convenience, Zod provides a dedicated API for making some or all properties optional, inspired by the built-in TypeScript utility type [`Partial`](https://www.typescriptlang.org/docs/handbook/utility-types.html#partialtype).

To make all fields optional:

To make certain properties optional:

### [`.required()`](?id=required)

Zod provides an API for making some or all properties _required_, inspired by TypeScript's [`Required`](https://www.typescriptlang.org/docs/handbook/utility-types.html#requiredtype) utility type.

To make all properties required:

To make certain properties required:

To define a self-referential type, use a [getter](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Functions/get) on the key. This lets JavaScript resolve the cyclical schema at runtime.

Though recursive schemas are supported, passing cyclical data into Zod will cause an infinite loop.

You can also represent _mutually recursive types_:

All object APIs (`.pick()`, `.omit()`, `.required()`, `.partial()`, etc.) work as you'd expect.

### [Circularity errors](?id=circularity-errors)

Due to TypeScript limitations, recursive type inference can be finicky, and it only works in certain scenarios. Some more complicated types may trigger recursive type errors like this:

In these cases, you can resolve the error with a type annotation on the offending getter:

To define an array schema:

To access the inner schema for an element of the array.

Zod implements a number of array-specific validations:

Unlike arrays, tuples are typically fixed-length arrays that specify different schemas for each index.

To add a variadic ("rest") argument:

Union types (`A | B`) represent a logical "OR". Zod union schemas will check the input against each option in order. The first value that validates successfully is returned.

To extract the internal option schemas:

A [discriminated union](https://www.typescriptlang.org/docs/handbook/2/narrowing.html#discriminated-unions) is a special kind of union in which a) all the options are object schemas that b) share a particular key (the "discriminator"). Based on the value of the discriminator key, TypeScript is able to "narrow" the type signature as you'd expect.

You could represent it with a regular `z.union()`. But regular unions are _naive_—they check the input against each option in order and return the first one that passes. This can be slow for large unions.

So Zod provides a `z.discriminatedUnion()` API that uses a _discriminator key_ to make parsing more efficient.

Each option should be an _object schema_ whose discriminator prop (`status` in the example above) corresponds to some literal value or set of values, usually `z.enum()`, `z.literal()`, `z.null()`, or `z.undefined()`.

Intersection types (`A & B`) represent a logical "AND".

This can be useful for intersecting two object types.

When merging object schemas, prefer [`A.extend(B)`](#extend) over intersections. Using `.extend()` will give you a new object schema, whereas `z.intersection(A, B)` returns a `ZodIntersection` instance which lacks common object methods like `pick` and `omit`.

Record schemas are used to validate types such as `Record<string, string>`.

The key schema can be any Zod schema that is assignable to `string | number | symbol`.

To create an object schemas containing keys defined by an enum:

**Zod 4** — In Zod 4, if you pass a `z.enum` as the first argument to `z.record()`, Zod will exhaustively check that all enum values exist in the input as keys. This behavior agrees with TypeScript:

In Zod 3, exhaustiveness was not checked. To replicate the old behavior, use `z.partialRecord()`.

If you want a _partial_ record type, use `z.partialRecord()`. This skips the special exhaustiveness checks Zod normally runs with `z.enum()` and `z.literal()` key schemas.

Set schemas can be further constrained with the following utility methods.

To validate `File` instances:

**Deprecated** — `z.promise()` is deprecated in Zod 4. There are vanishingly few valid uses cases for a `Promise` schema. If you suspect a value might be a `Promise`, simply `await` it before parsing it with Zod.

You can use `z.instanceof` to check that the input is an instance of a class. This is useful to validate inputs against classes that are exported from third-party libraries.

### [Property](?id=property)

To validate a particular property of a class instance against a Zod schema:

The `z.property()` API works with any data type (but it's most useful when used in conjunction with `z.instanceof()`).

Every Zod schema stores an array of _refinements_. Refinements are a way to perform custom validation that Zod doesn't provide a native API for.

### [`.refine()`](?id=refine)

Refinement functions should never throw. Instead they should return a falsy value to signal failure. Thrown errors are not caught by Zod.

#### [`error`](?id=error)

To customize the error message:

#### [`abort`](?id=abort)

By default, validation issues from checks are considered _continuable_; that is, Zod will execute _all_ checks in sequence, even if one of them causes a validation error. This is usually desirable, as it means Zod can surface as many errors as possible in one go.

To mark a particular refinement as _non-continuable_, use the `abort` parameter. Validation will terminate if the check fails.

#### [`path`](?id=path)

To customize the error path, use the `path` parameter. This is typically only useful in the context of object schemas.

This will set the `path` parameter in the associated issue:

To define an asynchronous refinement, just pass an `async` function:

If you use async refinements, you must use the `.parseAsync` method to parse data! Otherwise Zod will throw an error.

#### [`when`](?id=when)

**Note** — This is a power user feature and can absolutely be abused in ways that will increase the probability of uncaught errors originating from inside your refinements.

By default, refinements don't run if any _non-continuable_ issues have already been encountered. Zod is careful to ensure the type signature of the value is correct before passing it into any refinement functions.

In some cases, you want finer control over when refinements run. For instance consider this "password confirm" check:

An error on `anotherField` will prevent the password confirmation check from executing, even though the check doesn't depend on `anotherField`. To control when a refinement will run, use the `when` parameter:

### [`.superRefine()`](?id=superrefine)

The regular `.refine` API only generates a single issue with a `"custom"` error code, but `.superRefine()` makes it possible to create multiple issues using any of Zod's [internal issue types](https://github.com/colinhacks/zod/blob/main/packages/zod/src/v4/core/errors.ts).

### [`.check()`](?id=check)

**Note** — The `.check()` API is a more low-level API that's generally more complex than `.superRefine()`. It can be faster in performance-sensitive code paths, but it's also more verbose.

**New** — Introduced in Zod 4.1. Refer to the dedicated [Codecs](https://zod.dev/codecs) page for more information.

Codecs are a special kind of schema that implement _bidirectional transformations_ between two other schemas.

A regular `.parse()` operations performs the _forward transform_. It calls the codec's `decode` function.

You can alternatively use the top-level `z.decode()` function. Unlike `.parse()` (which accepts `unknown` input), `z.decode()` expects a strongly-typed input (`string` in this example).

To perform the _reverse transform_, use the inverse: `z.encode()`.

Refer to the dedicated [Codecs](https://zod.dev/codecs) page for more information. That page contains implementations for commonly-needed codecs that you can copy/paste into your project:

- [**`stringToNumber`**](about:/codecs#stringtonumber)
- [**`stringToInt`**](about:/codecs#stringtoint)
- [**`stringToBigInt`**](about:/codecs#stringtobigint)
- [**`numberToBigInt`**](about:/codecs#numbertobigint)
- [**`isoDatetimeToDate`**](about:/codecs#isodatetimetodate)
- [**`epochSecondsToDate`**](about:/codecs#epochsecondstodate)
- [**`epochMillisToDate`**](about:/codecs#epochmillistodate)
- [**`jsonCodec`**](about:/codecs#jsoncodec)
- [**`utf8ToBytes`**](about:/codecs#utf8tobytes)
- [**`bytesToUtf8`**](about:/codecs#bytestoutf8)
- [**`base64ToBytes`**](about:/codecs#base64tobytes)
- [**`base64urlToBytes`**](about:/codecs#base64urltobytes)
- [**`hexToBytes`**](about:/codecs#hextobytes)
- [**`stringToURL`**](about:/codecs#stringtourl)
- [**`stringToHttpURL`**](about:/codecs#stringtohttpurl)
- [**`uriComponent`**](about:/codecs#uricomponent)
- [**`stringToBoolean`**](about:/codecs#stringtoboolean)

Schemas can be chained together into "pipes". Pipes are primarily useful when used in conjunction with [Transforms](#transforms).

**Note** — For bi-directional transforms, use [codecs](https://zod.dev/codecs).

Transforms are a special kind of schema that perform a unidirectional transformation. Instead of validating input, they accept anything and perform some transformation on the data. To define a transform:

Refinement functions should never throw. Thrown errors are not caught by Zod.

To perform validation logic inside a transform, use `ctx`. To report a validation issue, push a new issue onto `ctx.issues` (similar to the [`.check()`](#check) API).

Most commonly, transforms are used in conjunction with [Pipes](#pipes). This combination is useful for performing some initial validation, then transforming the parsed data into another form.

### [`.transform()`](?id=transform)

Piping some schema into a transform is a common pattern, so Zod provides a convenience `.transform()` method.

Transforms can also be async:

If you use async transforms, you must use a `.parseAsync` or `.safeParseAsync` when parsing data! Otherwise Zod will throw an error.

### [`.preprocess()`](?id=preprocess)

Piping a transform into another schema is another common pattern, so Zod provides a convenience `z.preprocess()` function.

To set a default value for a schema:

Alternatively, you can pass a function which will be re-executed whenever a default value needs to be generated:

In Zod, setting a _default_ value will short-circuit the parsing process. If the input is `undefined`, the default value is eagerly returned. As such, the default value must be assignable to the _output type_ of the schema.

Sometimes, it's useful to define a _prefault_ ("pre-parse default") value. If the input is `undefined`, the prefault value will be parsed instead. The parsing process is _not_ short circuited. As such, the prefault value must be assignable to the _input type_ of the schema.

This is also useful if you want to pass some input value through some mutating refinements.

Use `.catch()` to define a fallback value to be returned in the event of a validation error:

Alternatively, you can pass a function which will be re-executed whenever a catch value needs to be generated.

TypeScript's type system is [structural](https://www.typescriptlang.org/docs/handbook/type-compatibility.html), meaning that two types that are structurally equivalent are considered the same.

In some cases, it can be desirable to simulate [nominal typing](https://en.wikipedia.org/wiki/Nominal_type_system) inside TypeScript. This can be achieved with _branded types_ (also known as "opaque types").

Under the hood, this works by attaching a "brand" to the schema's inferred type.

With this brand, any plain (unbranded) data structures are no longer assignable to the inferred type. You have to parse some data with the schema to get branded data.

Note that branded types do not affect the runtime result of `.parse`. It is a static-only construct.

To mark a schema as readonly:

The inferred type of the new schemas will be marked as `readonly`. Note that in TypeScript, this only affects objects, arrays, tuples, `Set`, and `Map`:

Inputs will be parsed like normal, then the result will be frozen with [`Object.freeze()`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/freeze) to prevent modifications.

To validate any JSON-encodable value:

This is a convenience API that returns the following union schema:

Zod provides a `z.function()` utility for defining Zod-validated functions. This way, you can avoid intermixing validation code with your business logic.

Function schemas have an `.implement()` method which accepts a function and returns a new function that automatically validates its inputs and outputs.

This function will throw a `ZodError` if the input is invalid:

If you only care about validating inputs, you can omit the `output` field.

Use the `.implementAsync()` method to create an async function.

You can create a Zod schema for any TypeScript type by using `z.custom()`. This is useful for creating schemas for types that are not supported by Zod out of the box, such as template string literals.

If you don't provide a validation function, Zod will allow any value. This can be dangerous!

You can customize the error message and other options by passing a second argument. This parameter works the same way as the params parameter of [`.refine`](#refine).
