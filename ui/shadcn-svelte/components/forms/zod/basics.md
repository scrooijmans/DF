# Basic usage | Zod

This page will walk you through the basics of creating schemas, parsing data, and using inferred types. For complete documentation on Zod's schema API, refer to [Defining schemas](https://zod.dev/api).

Before you can do anything else, you need to define a schema. For the purposes of this guide, we'll use a simple object schema.

Given any Zod schema, use `.parse` to validate an input. If it's valid, Zod returns a strongly-typed _deep clone_ of the input.

**Note** — If your schema uses certain asynchronous APIs like `async` [refinements](about:/api#refinements) or [transforms](about:/api#transforms), you'll need to use the `.parseAsync()` method instead.

When validation fails, the `.parse()` method will throw a `ZodError` instance with granular information about the validation issues.

To avoid a `try/catch` block, you can use the `.safeParse()` method to get back a plain result object containing either the successfully parsed data or a `ZodError`. The result type is a [discriminated union](https://www.typescriptlang.org/docs/handbook/2/narrowing.html#discriminated-unions), so you can handle both cases conveniently.

**Note** — If your schema uses certain asynchronous APIs like `async` [refinements](about:/api#refinements) or [transforms](about:/api#transforms), you'll need to use the `.safeParseAsync()` method instead.

Zod infers a static type from your schema definitions. You can extract this type with the `z.infer<>` utility and use it however you like.

In some cases, the input & output types of a schema can diverge. For instance, the `.transform()` API can convert the input from one type to another. In these cases, you can extract the input and output types independently:

---

Now that we have the basics covered, let's jump into the Schema API.
