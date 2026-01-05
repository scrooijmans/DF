# JSON Schema | Zod

💎

**New** — Zod 4 introduces a new feature: native [JSON Schema](https://json-schema.org/) conversion. JSON Schema is a standard for describing the structure of JSON (with JSON). It's widely used in [OpenAPI](https://www.openapis.org/) definitions and defining [structured outputs](https://platform.openai.com/docs/guides/structured-outputs?api-mode=chat) for AI.

To convert a Zod schema to JSON Schema, use the `z.toJSONSchema()` function.

All schema & checks are converted to their closest JSON Schema equivalent. Some types have no analog and cannot be reasonably represented. See the [`unrepresentable`](#unrepresentable) section below for more information on handling these cases.

Zod converts the following schema types to the equivalent JSON Schema `format`:

These schemas are supported via `contentEncoding`:

All other string formats are supported via `pattern`:

Zod converts the following numeric types to JSON Schema:

By default, `z.object()` schemas contain `additionalProperties: "false"`. This is an accurate representation of Zod's default behavior, as plain `z.object()` schema strip additional properties.

When converting to JSON Schema in `"input"` mode, `additionalProperties` is not set. See the [`io` docs](#io) for more information.

By contrast:

- `z.looseObject()` will _never_ set `additionalProperties: false`
- `z.strictObject()` will _always_ set `additionalProperties: false`

Zod converts `z.file()` to the following OpenAPI-friendly schema:

Size and MIME checks are also represented:

Zod converts both `undefined`/`null` to `{ type: "null" }` in JSON Schema.

Similarly, `nullable` is represented via a union with `null`::

Optional schemas are represented as-is, though they are decorated with an `optional` annotation.

A second argument can be used to customize the conversion logic.

Below is a quick reference for each supported parameter. Each one is explained in more detail below.

### [`target`](?id=target)

To set the target JSON Schema version, use the `target` parameter. By default, Zod will target Draft 2020-12.

### [`metadata`](?id=metadata)

In Zod, metadata is stored in registries. Zod exports a global registry `z.globalRegistry` that can be used to store common metadata fields like `id`, `title`, `description`, and `examples`.

All metadata fields get copied into the resulting JSON Schema.

### [`unrepresentable`](?id=unrepresentable)

The following APIs are not representable in JSON Schema. By default, Zod will throw an error if they are encountered. It is unsound to attempt a conversion to JSON Schema; you should modify your schemas as they have no equivalent in JSON. An error will be thrown if any of these are encountered.

By default, Zod will throw an error if any of these are encountered.

You can change this behavior by setting the `unrepresentable` option to `"any"`. This will convert any unrepresentable types to `{}` (the equivalent of `unknown` in JSON Schema).

### [`cycles`](?id=cycles)

How to handle cycles. If a cycle is encountered as `z.toJSONSchema()` traverses the schema, it will be represented using `$ref`.

If instead you want to throw an error, set the `cycles` option to `"throw"`.

### [`reused`](?id=reused)

How to handle schemas that occur multiple times in the same schema. By default, Zod will inline these schemas.

Instead you can set the `reused` option to `"ref"` to extract these schemas into `$defs`.

### [`override`](?id=override)

To define some custom override logic, use `override`. The provided callback has access to the original Zod schema and the default JSON Schema. _This function should directly modify `ctx.jsonSchema`._

Note that unrepresentable types will throw an `Error` before this functions is called. If you are trying to define custom behavior for an unrepresentable type, you'll need to use set the `unrepresentable: "any"` alongside `override`.

### [`io`](?id=io)

Some schema types have different input and output types, e.g. `ZodPipe`, `ZodDefault`, and coerced primitives. By default, the result of `z.toJSONSchema` represents the _output type_; use `"io": "input"` to extract the input type instead.

Passing a schema into `z.toJSONSchema()` will return a _self-contained_ JSON Schema.

In other cases, you may have a set of Zod schemas you'd like to represent using multiple interlinked JSON Schemas, perhaps to write to `.json` files and serve from a web server.

To achieve this, you can pass a [registry](about:/metadata#registries) into `z.toJSONSchema()`.

**Important** — All schemas should have a registered `id` property in the registry! Any schemas without an `id` will be ignored.

By default, the `$ref` URIs are simple relative paths like `"User"`. To make these absolute URIs, use the `uri` option. This expects a function that converts an `id` to a fully-qualified URI.
