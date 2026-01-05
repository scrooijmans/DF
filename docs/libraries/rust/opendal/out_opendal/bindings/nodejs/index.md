- <a href="https://opendal.apache.org/" class="breadcrumbs__link" aria-label="Home page"><img src="out_opendal/bindings/nodejs/index_media/edf54a765e27bedcc87f5708545b58efaaa38a1a.svg" class="breadcrumbHomeIcon_q3uS" /></a>
- <a href="https://opendal.apache.org/category/bindings/" class="breadcrumbs__link">Bindings</a>
- Node.js

On this page

# Node.js

## Apache OpenDALâ„¢ Node.js Binding

[<img src="out_opendal/bindings/nodejs/index_media/745e4981547972eb827f306a82346e6d4bd99dd0.svg" class="img_KBPg" decoding="async" loading="lazy" />](https://www.npmjs.com/package/opendal) [<img src="out_opendal/bindings/nodejs/index_media/26ef2491d77947894db35d459e7696869fa6c633.svg" class="img_KBPg" decoding="async" loading="lazy" alt="npm" />](https://www.npmjs.com/package/opendal) [<img src="out_opendal/bindings/nodejs/index_media/b8908a5bd08b3de6e77ea4c1c7aa4f9e733185d5.svg" class="img_KBPg" decoding="async" loading="lazy" alt="Website" />](https://opendal.apache.org/docs/nodejs/)

<img src="out_opendal/bindings/nodejs/index_media/f6db506ffa2fa9b49bb043591266c8aa10c32f59.jpg" class="img_KBPg" decoding="async" loading="lazy" />

## Useful Links<a href="https://opendal.apache.org/bindings/nodejs/#useful-links" class="hash-link" aria-label="Direct link to Useful Links" translate="no" title="Direct link to Useful Links">â€‹</a>

- [Documentation](https://opendal.apache.org/docs/nodejs/)
- [Upgrade Guide](https://github.com/apache/opendal/blob/main/bindings/nodejs/upgrade.md)

## Installation<a href="https://opendal.apache.org/bindings/nodejs/#installation" class="hash-link" aria-label="Direct link to Installation" translate="no" title="Direct link to Installation">â€‹</a>

``` prism-code
npm install opendal
```

## Docs<a href="https://opendal.apache.org/bindings/nodejs/#docs" class="hash-link" aria-label="Direct link to Docs" translate="no" title="Direct link to Docs">â€‹</a>

To build the docs locally, please run the following commands:

``` prism-code
# Only need to run once unless you want to update the docs theme
pnpm run build:theme

# Build the docs
pnpm run docs
```

## Tests<a href="https://opendal.apache.org/bindings/nodejs/#tests" class="hash-link" aria-label="Direct link to Tests" translate="no" title="Direct link to Tests">â€‹</a>

Services behavior tests read necessary configs from env vars or the `.env` file.

You can copy [.env.example](https://opendal.apache.org/.env.example) to `$(pwd)/.env` and change the values on need, or directly set env vars with `export KEY=VALUE`.

Take `fs` for example, we need to enable bench on `fs` on `/tmp`:

``` prism-code
OPENDAL_TEST=fs
OPENDAL_FS_ROOT=/tmp
```

You can run service behavior tests of enabled with the following command:

``` prism-code
pnpm build && pnpm test
```

## Usage<a href="https://opendal.apache.org/bindings/nodejs/#usage" class="hash-link" aria-label="Direct link to Usage" translate="no" title="Direct link to Usage">â€‹</a>

``` prism-code
import { Operator } from "opendal";

async function main() {
  const op = new Operator("fs", { root: "/tmp" });
  await op.write("test", "Hello, World!");
  const bs = await op.read("test");
  console.log(new TextDecoder().decode(bs));
  const meta = await op.stat("test");
  console.log(`contentLength: ${meta.contentLength}`);
}

main();
```

## Usage with Next.js<a href="https://opendal.apache.org/bindings/nodejs/#usage-with-nextjs" class="hash-link" aria-label="Direct link to Usage with Next.js" translate="no" title="Direct link to Usage with Next.js">â€‹</a>

Config automatically be bundled by [Next.js](https://nextjs.org/docs/app/api-reference/config/next-config-js/serverExternalPackages).

``` prism-code
/** @type {import('next').NextConfig} */
const nextConfig = {
  serverExternalPackages: ["opendal"],
};

module.exports = nextConfig;
```

## Contributing<a href="https://opendal.apache.org/bindings/nodejs/#contributing" class="hash-link" aria-label="Direct link to Contributing" translate="no" title="Direct link to Contributing">â€‹</a>

- Start with [Contributing Guide](https://opendal.apache.org/bindings/nodejs/CONTRIBUTING.md).
- Submit [Issues](https://github.com/apache/opendal/issues/new) for bug report or feature requests.
- Asking questions in the [Discussions](https://github.com/apache/opendal/discussions/new?category=q-a).
  - Talk to community at [Discord](https://opendal.apache.org/discord).

## License and Trademarks<a href="https://opendal.apache.org/bindings/nodejs/#license-and-trademarks" class="hash-link" aria-label="Direct link to License and Trademarks" translate="no" title="Direct link to License and Trademarks">â€‹</a>

Licensed under the Apache License, Version 2.0: <http://www.apache.org/licenses/LICENSE-2.0>

Apache OpenDAL, OpenDAL, and Apache are either registered trademarks or trademarks of the Apache Software Foundation.

<a href="https://github.com/apache/opendal/tree/main/website/docs/20-bindings/nodejs.mdx" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_opendal/bindings/nodejs/index_media/82254bca835e54e35c885c6543f8416b5aff021e.svg" class="iconEdit_evxu" />Edit this page</a>

Last updated on **Mar 10, 2025** by **Xuanwo**

<a href="https://opendal.apache.org/bindings/lua/" class="pagination-nav__link pagination-nav__link--prev"></a>

Previous

Lua ðŸš§

<a href="https://opendal.apache.org/bindings/ocaml/" class="pagination-nav__link pagination-nav__link--next"></a>

Next

OCaml ðŸš§

- <a href="https://opendal.apache.org/bindings/nodejs/#useful-links" class="table-of-contents__link toc-highlight">Useful Links</a>
- <a href="https://opendal.apache.org/bindings/nodejs/#installation" class="table-of-contents__link toc-highlight">Installation</a>
- <a href="https://opendal.apache.org/bindings/nodejs/#docs" class="table-of-contents__link toc-highlight">Docs</a>
- <a href="https://opendal.apache.org/bindings/nodejs/#tests" class="table-of-contents__link toc-highlight">Tests</a>
- <a href="https://opendal.apache.org/bindings/nodejs/#usage" class="table-of-contents__link toc-highlight">Usage</a>
- <a href="https://opendal.apache.org/bindings/nodejs/#usage-with-nextjs" class="table-of-contents__link toc-highlight">Usage with Next.js</a>
- <a href="https://opendal.apache.org/bindings/nodejs/#contributing" class="table-of-contents__link toc-highlight">Contributing</a>
- <a href="https://opendal.apache.org/bindings/nodejs/#license-and-trademarks" class="table-of-contents__link toc-highlight">License and Trademarks</a>
