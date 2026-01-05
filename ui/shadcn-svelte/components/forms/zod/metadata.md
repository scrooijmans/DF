# Metadata and registries | Zod

It's often useful to associate a schema with some additional _metadata_ for documentation, code generation, AI structured outputs, form validation, and other purposes.

Metadata in Zod is handled via _registries_. Registries are collections of schemas, each associated with some _strongly-typed_ metadata. To create a simple registry:

To register, lookup, and remove schemas from this registry:

TypeScript enforces that the metadata for each schema matches the registry's **metadata type**.

**Special handling for `id`** — Zod registries treat the `id` property specially. An `Error` will be thrown if multiple schemas are registered with the same `id` value. This is true for all registries, including the global registry.

### [`.register()`](?id=register)

**Note** — This method is special in that it does not return a new schema; instead, it returns the original schema. No other Zod method does this! That includes `.meta()` and `.describe()` (documented below) which return a new instance.

Schemas provide a `.register()` method to more conveniently add it to a registry.

This lets you define metadata "inline" in your schemas.

If a registry is defined without a metadata type, you can use it as a generic "collection", no metadata required.

### [`z.globalRegistry`](?id=zglobalregistry)

For convenience, Zod provides a global registry (`z.globalRegistry`) that can be used to store metadata for JSON Schema generation or other purposes. It accepts the following metadata:

To register some metadata in `z.globalRegistry` for a schema:

To globally augment the `GlobalMeta` interface, use [_declaration merging_](https://www.typescriptlang.org/docs/handbook/declaration-merging.html). Add the following anywhere in your codebase. Creating a `zod.d.ts` file in your project root is a common convention.

### [`.meta()`](?id=meta)

For a more convenient approach, use the `.meta()` method to register a schema in `z.globalRegistry`.

Calling `.meta()` without an argument will _retrieve_ the metadata for a schema.

Metadata is associated with a _specific schema instance._ This is important to keep in mind, especially since Zod methods are immutable—they always return a new instance.

### [`.describe()`](?id=describe)

The `.describe()` method still exists for compatibility with Zod 3, but `.meta()` is now the recommended approach.

The `.describe()` method is a shorthand for registering a schema in `z.globalRegistry` with just a `description` field.

You've already seen a simple example of a custom registry:

Let's look at some more advanced patterns.

### [Referencing inferred types](?id=referencing-inferred-types)

It's often valuable for the metadata type to reference the _inferred type_ of a schema. For instance, you may want an `examples` field to contain examples of the schema's output.

The special symbol `z.$output` is a reference to the schemas inferred output type (`z.infer<typeof schema>`). Similarly you can use `z.$input` to reference the input type.

### [Constraining schema types](?id=constraining-schema-types)

Pass a second generic to `z.registry()` to constrain the schema types that can be added to a registry. This registry only accepts string schemas.
