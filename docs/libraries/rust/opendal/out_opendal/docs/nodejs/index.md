<a href="https://opendal.apache.org/docs/nodejs/index.html" class="title">Apache OpenDALâ„¢ - v0.49.1</a>

[Homepage](https://opendal.apache.org/)[GitHub](https://github.com/apache/opendal/tree/main/bindings/nodejs)

![](out_opendal/docs/nodejs/index_media/adbc65b898facc8d26b81ca3fd54e6f3e8620382.svg)

Preparing search index...

<a href="https://opendal.apache.org/docs/nodejs/index.html#" id="tsd-toolbar-menu-trigger" class="tsd-widget menu" aria-label="Menu" data-toggle="menu"><img src="out_opendal/docs/nodejs/index_media/b07d0d4afaa2b2033a91224d031b6e40242f77cc.svg" /></a>

# Apache OpenDALâ„¢ - v0.49.1

# Apache OpenDALâ„¢ Node.js Binding<a href="#apache-opendalâ¢-nodejs-binding" class="tsd-anchor-icon" aria-label="Permalink"><img src="out_opendal/docs/nodejs/index_media/5bf0b58351cfdbdcfa5f83b7732e0f2ed81a4779.svg" /></a>

[![](out_opendal/docs/nodejs/index_media/745e4981547972eb827f306a82346e6d4bd99dd0.svg)](https://www.npmjs.com/package/opendal) [![npm](out_opendal/docs/nodejs/index_media/26ef2491d77947894db35d459e7696869fa6c633.svg)](https://www.npmjs.com/package/opendal) [![Website](out_opendal/docs/nodejs/index_media/b8908a5bd08b3de6e77ea4c1c7aa4f9e733185d5.svg)](https://opendal.apache.org/docs/nodejs/)

![](out_opendal/docs/nodejs/index_media/f6db506ffa2fa9b49bb043591266c8aa10c32f59.jpg)

## Useful Links<a href="https://opendal.apache.org/docs/nodejs/index.html#useful-links" class="tsd-anchor-icon" aria-label="Permalink"><img src="out_opendal/docs/nodejs/index_media/5bf0b58351cfdbdcfa5f83b7732e0f2ed81a4779.svg" /></a>

- [Documentation](https://opendal.apache.org/docs/nodejs/)
- [Upgrade Guide](https://opendal.apache.org/docs/nodejs/documents/upgrade.html)

## Installation<a href="https://opendal.apache.org/docs/nodejs/index.html#installation" class="tsd-anchor-icon" aria-label="Permalink"><img src="out_opendal/docs/nodejs/index_media/5bf0b58351cfdbdcfa5f83b7732e0f2ed81a4779.svg" /></a>

``` shell
npm install opendal
Copy
```

## Docs<a href="https://opendal.apache.org/docs/nodejs/index.html#docs" class="tsd-anchor-icon" aria-label="Permalink"><img src="out_opendal/docs/nodejs/index_media/5bf0b58351cfdbdcfa5f83b7732e0f2ed81a4779.svg" /></a>

To build the docs locally, please run the following commands:

``` shell
# Only need to run once unless you want to update the docs theme
pnpm run build:theme

# Build the docs
pnpm run docs
Copy
```

## Tests<a href="https://opendal.apache.org/docs/nodejs/index.html#tests" class="tsd-anchor-icon" aria-label="Permalink"><img src="out_opendal/docs/nodejs/index_media/5bf0b58351cfdbdcfa5f83b7732e0f2ed81a4779.svg" /></a>

Services behavior tests read necessary configs from env vars or the `.env` file.

You can copy [.env.example](https://opendal.apache.org/.env.example) to `$(pwd)/.env` and change the values on need, or directly set env vars with `export KEY=VALUE`.

Take `fs` for example, we need to enable bench on `fs` on `/tmp`:

``` properties
OPENDAL_TEST=fs
OPENDAL_FS_ROOT=/tmp
Copy
```

You can run service behavior tests of enabled with the following command:

``` shell
pnpm build && pnpm test
Copy
```

## Usage<a href="https://opendal.apache.org/docs/nodejs/index.html#usage" class="tsd-anchor-icon" aria-label="Permalink"><img src="out_opendal/docs/nodejs/index_media/5bf0b58351cfdbdcfa5f83b7732e0f2ed81a4779.svg" /></a>

``` javascript
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
Copy
```

## Usage with Next.js<a href="https://opendal.apache.org/docs/nodejs/index.html#usage-with-nextjs" class="tsd-anchor-icon" aria-label="Permalink"><img src="out_opendal/docs/nodejs/index_media/5bf0b58351cfdbdcfa5f83b7732e0f2ed81a4779.svg" /></a>

Config automatically be bundled by [Next.js](https://nextjs.org/docs/app/api-reference/config/next-config-js/serverExternalPackages).

``` javascript
/** @type {import('next').NextConfig} */
const nextConfig = {
  serverExternalPackages: ["opendal"],
};

module.exports = nextConfig;
Copy
```

## Contributing<a href="https://opendal.apache.org/docs/nodejs/index.html#contributing" class="tsd-anchor-icon" aria-label="Permalink"><img src="out_opendal/docs/nodejs/index_media/5bf0b58351cfdbdcfa5f83b7732e0f2ed81a4779.svg" /></a>

- Start with [Contributing Guide](https://opendal.apache.org/docs/nodejs/documents/CONTRIBUTING.html).
- Submit [Issues](https://github.com/apache/opendal/issues/new) for bug report or feature requests.
- Asking questions in the [Discussions](https://github.com/apache/opendal/discussions/new?category=q-a).
  - Talk to community at [Discord](https://opendal.apache.org/discord).

## License and Trademarks<a href="https://opendal.apache.org/docs/nodejs/index.html#license-and-trademarks" class="tsd-anchor-icon" aria-label="Permalink"><img src="out_opendal/docs/nodejs/index_media/5bf0b58351cfdbdcfa5f83b7732e0f2ed81a4779.svg" /></a>

Licensed under the Apache License, Version 2.0: <http://www.apache.org/licenses/LICENSE-2.0>

Apache OpenDAL, OpenDAL, and Apache are either registered trademarks or trademarks of the Apache Software Foundation.

![](out_opendal/docs/nodejs/index_media/49fab8c44b3664a93049af55747d9aab4d9d56c7.svg)

### Settings

Member Visibility

- [x] ![](out_opendal/docs/nodejs/index_media/bed852a5793c1fd9f7a03de8ebab8248a23ebeaa.svg)Inherited

ThemeOSLightDark

![](out_opendal/docs/nodejs/index_media/49fab8c44b3664a93049af55747d9aab4d9d56c7.svg)

### On This Page

[Apache OpenDALâ„¢ Node.js Binding](#apache-opendalâ¢-nodejs-binding)

- [Useful Links](https://opendal.apache.org/docs/nodejs/index.html#useful-links)
- [Installation](https://opendal.apache.org/docs/nodejs/index.html#installation)
- [Docs](https://opendal.apache.org/docs/nodejs/index.html#docs)
- [Tests](https://opendal.apache.org/docs/nodejs/index.html#tests)
- [Usage](https://opendal.apache.org/docs/nodejs/index.html#usage)
- [Usage with Next.js](https://opendal.apache.org/docs/nodejs/index.html#usage-with-nextjs)
- [Contributing](https://opendal.apache.org/docs/nodejs/index.html#contributing)
- [License and Trademarks](https://opendal.apache.org/docs/nodejs/index.html#license-and-trademarks)

<a href="https://opendal.apache.org/" class="tsd-nav-link">Homepage</a><a href="https://github.com/apache/opendal/tree/main/bindings/nodejs" class="tsd-nav-link">GitHub</a>

[Apache OpenDALâ„¢ - v0.49.1](https://opendal.apache.org/docs/nodejs/modules.html)

- Loading...

Copyright Â© 2022-2025, The Apache Software Foundation. Apache OpenDAL, OpenDAL, and Apache are either registered trademarks or trademarks of the Apache Software Foundation.
