# Customizing errors | Zod

In Zod, validation errors are surfaced as instances of the `z.core.$ZodError` class.

The `ZodError` class in the `zod` package is a subclass that implements some additional convenience methods.

Instances of `$ZodError` contain an `.issues` array. Each issue contains a human-readable `message` and additional structured metadata about the issue.

Every issue contains a `message` property with a human-readable error message. Error messages can be customized in a number of ways.

Virtually every Zod API accepts an optional error message.

This custom error will show up as the `message` property of any validation issues that originate from this schema.

All `z` functions and schema methods accept custom errors.

If you prefer, you can pass a params object with an `error` parameter instead.

The `error` param optionally accepts a function. An error customization function is known as an **error map** in Zod terminology. The error map will run at parse time if a validation error occurs.

**Note** — In Zod v3, there were separate params for `message` (a string) and `errorMap` (a function). These have been unified in Zod 4 as `error`.

The error map receives a context object you can use to customize the error message based on the validation issue.

For advanced cases, the `iss` object provides additional information you can use to customize the error.

Depending on the API you are using, there may be additional properties available. Use TypeScript's autocomplete to explore the available properties.

Return `undefined` to avoid customizing the error message and fall back to the default message. (More specifically, Zod will yield control to the next error map in the [precedence chain](#error-precedence).) This is useful for selectively customizing certain error messages but not others.

To customize errors on a _per-parse_ basis, pass an error map into the parse method:

This has _lower precedence_ than any schema-level custom messages.

The `iss` object is a [discriminated union](https://www.typescriptlang.org/docs/handbook/2/narrowing.html#discriminated-unions) of all possible issue types. Use the `code` property to discriminate between them.

For a breakdown of all Zod issue codes, see the [`zod/v4/core`](about:/packages/core#issue-types) documentation.

### [Include input in issues](?id=include-input-in-issues)

By default, Zod does not include input data in issues. This is to prevent unintentional logging of potentially sensitive input data. To include the input data in each issue, use the `reportInput` flag:

To specify a global error map, use `z.config()` to set Zod's `customError` configuration setting:

Global error messages have _lower precedence_ than schema-level or per-parse error messages.

The `iss` object is a [discriminated union](https://www.typescriptlang.org/docs/handbook/2/narrowing.html#discriminated-unions) of all possible issue types. Use the `code` property to discriminate between them.

For a breakdown of all Zod issue codes, see the [`zod/v4/core`](about:/packages/core#issue-types) documentation.

To support internationalization of error message, Zod provides several built-in **locales**. These are exported from the `zod/v4/core` package.

**Note** — The regular `zod` library automatically loads the `en` locale automatically. Zod Mini does not load any locale by default; instead all error messages default to `Invalid input`.

To lazily load a locale, consider dynamic imports:

For convenience, all locales are exported as `z.locales` from `"zod"`. In some bundlers, this may not be tree-shakable.

### [Locales](?id=locales)

The following locales are available:

- `ar` — Arabic
- `az` — Azerbaijani
- `be` — Belarusian
- `bg` — Bulgarian
- `ca` — Catalan
- `cs` — Czech
- `da` — Danish
- `de` — German
- `en` — English
- `eo` — Esperanto
- `es` — Spanish
- `fa` — Farsi
- `fi` — Finnish
- `fr` — French
- `frCA` — Canadian French
- `he` — Hebrew
- `hu` — Hungarian
- `id` — Indonesian
- `is` — Icelandic
- `it` — Italian
- `ja` — Japanese
- `ka` — Georgian
- `km` — Khmer
- `ko` — Korean
- `lt` — Lithuanian
- `mk` — Macedonian
- `ms` — Malay
- `nl` — Dutch
- `no` — Norwegian
- `ota` — Türkî
- `ps` — Pashto
- `pl` — Polish
- `pt` — Portuguese
- `ru` — Russian
- `sl` — Slovenian
- `sv` — Swedish
- `ta` — Tamil
- `th` — Thai
- `tr` — Türkçe
- `uk` — Ukrainian
- `ur` — Urdu
- `vi` — Tiếng Việt
- `zhCN` — Simplified Chinese
- `zhTW` — Traditional Chinese
- `yo` — Yorùbá

Below is a quick reference for determining error precedence: if multiple error customizations have been defined, which one takes priority? From _highest to lowest_ priority:

1.  **Schema-level error** — Any error message "hard coded" into a schema definition.

2.  **Per-parse error** — A custom error map passed into the `.parse()` method.

3.  **Global error map** — A custom error map passed into `z.config()`.

4.  **Locale error map** — A custom error map passed into `z.config()`.
