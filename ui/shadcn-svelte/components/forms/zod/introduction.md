# Intro | Zod

## Featured sponsor: Jazz

## [Introduction](?id=introduction)

Zod is a TypeScript-first validation library. Using Zod, you can define _schemas_ you can use to validate data, from a simple `string` to a complex nested object.

```
import * as z from "zod";

const User = z.object({
  name: z.string(),
});

// some untrusted data...
const input = { /* stuff */ };

// the parsed result is validated and type safe!
const data = User.parse(input);

// so you can use it with confidence :)
console.log(data.name);
```

## [Features](?id=features)

- Zero external dependencies
- Works in Node.js and all modern browsers
- Tiny: 2kb core bundle (gzipped)
- Immutable API: methods return a new instance
- Concise interface
- Works with TypeScript and plain JS
- Built-in JSON Schema conversion
- Extensive ecosystem

## [Installation](?id=installation)

```
npm install zod
```

Zod is also available as `@zod/zod` on [jsr.io](https://jsr.io/@zod/zod).

Zod provides an MCP server that can be used by agents to search Zod's docs. To add to your editor, follow [these instructions](https://share.inkeep.com/zod/mcp). Zod also provides an [llms.txt](https://zod.dev/llms.txt) file.

## [Requirements](?id=requirements)

Zod is tested against _TypeScript v5.5_ and later. Older versions may work but are not officially supported.

### [`"strict"`](?id=strict)

You must enable `strict` mode in your `tsconfig.json`. This is a best practice for all TypeScript projects.

```
// tsconfig.json
{
  // ...
  "compilerOptions": {
    // ...
    "strict": true
  }
}
```

## [Ecosystem](?id=ecosystem)

Zod has a thriving ecosystem of libraries, tools, and integrations. Refer to the [Ecosystem page](https://zod.dev/ecosystem) for a complete list of libraries that support Zod or are built on top of it.

- [Resources](https://zod.dev/ecosystem?id=resources)
- [API Libraries](https://zod.dev/ecosystem?id=api-libraries)
- [Form Integrations](https://zod.dev/ecosystem?id=form-integrations)
- [Zod to X](https://zod.dev/ecosystem?id=zod-to-x)
- [X to Zod](https://zod.dev/ecosystem?id=x-to-zod)
- [Mocking Libraries](https://zod.dev/ecosystem?id=mocking-libraries)
- [Powered by Zod](https://zod.dev/ecosystem?id=powered-by-zod)

I also contribute to the following projects, which I'd like to highlight:

- [tRPC](https://trpc.io/) - End-to-end typesafe APIs, with support for Zod schemas
- [React Hook Form](https://react-hook-form.com/) - Hook-based form validation with a [Zod resolver](https://react-hook-form.com/docs/useform#resolver)
- [zshy](https://github.com/colinhacks/zshy) - Originally created as Zod's internal build tool. Bundler-free, batteries-included build tool for TypeScript libraries. Powered by `tsc`.

## [Sponsors](?id=sponsors)

Sponsorship at any level is appreciated and encouraged. If you built a paid product using Zod, consider one of the [corporate tiers](https://github.com/sponsors/colinhacks).

### [Platinum](?id=platinum)

### [Gold](?id=gold)

[![Retool logo (light theme)](https://github.com/colinhacks/zod/assets/3084745/5ef4c11b-efeb-4495-90a8-41b83f798600)![Retool logo (dark theme)](https://github.com/colinhacks/zod/assets/3084745/ac65013f-aeb4-48dd-a2ee-41040b69cbe6)](https://retool.com/?utm_source=github&utm_medium=referral&utm_campaign=zod)

Build AI apps and workflows with Retool AI

[retool.com](https://retool.com/?utm_source=github&utm_medium=referral&utm_campaign=zod)

### [Silver](?id=silver)

### [Bronze](?id=bronze)
