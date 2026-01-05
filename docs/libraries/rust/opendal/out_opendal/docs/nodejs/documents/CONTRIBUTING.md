<a href="https://opendal.apache.org/docs/nodejs/index.html" class="title">Apache OpenDALâ„¢ - v0.49.1</a>

[Homepage](https://opendal.apache.org/)[GitHub](https://github.com/apache/opendal/tree/main/bindings/nodejs)

![](out_opendal/docs/nodejs/documents/CONTRIBUTING_media/f804b37d8f9651dce021c98c3091caf59681a8c3.svg)

Preparing search index...

<a href="https://opendal.apache.org/docs/nodejs/documents/CONTRIBUTING.html#" id="tsd-toolbar-menu-trigger" class="tsd-widget menu" aria-label="Menu" data-toggle="menu"><img src="out_opendal/docs/nodejs/documents/CONTRIBUTING_media/f3fbb7bef6ec1f2e24711c5bb2ef60c9c6ecb0d5.svg" /></a>

- <a href="https://opendal.apache.org/docs/nodejs/documents/CONTRIBUTING.html" aria-current="page">CONTRIBUTING</a>

# Contributing<a href="https://opendal.apache.org/docs/nodejs/documents/CONTRIBUTING.html#contributing" class="tsd-anchor-icon" aria-label="Permalink"><img src="out_opendal/docs/nodejs/documents/CONTRIBUTING_media/1d480b11402974a270d65a1008580bad01955eab.svg" /></a>

- [Contributing](https://opendal.apache.org/docs/nodejs/documents/CONTRIBUTING.html#contributing)
  - [Setup](https://opendal.apache.org/docs/nodejs/documents/CONTRIBUTING.html#setup)
    - [Using a dev container environment](https://opendal.apache.org/docs/nodejs/documents/CONTRIBUTING.html#using-a-dev-container-environment)
    - [Bring your own toolbox](https://opendal.apache.org/docs/nodejs/documents/CONTRIBUTING.html#bring-your-own-toolbox)
  - [Build](https://opendal.apache.org/docs/nodejs/documents/CONTRIBUTING.html#build)
  - [Test](https://opendal.apache.org/docs/nodejs/documents/CONTRIBUTING.html#test)

## Setup<a href="https://opendal.apache.org/docs/nodejs/documents/CONTRIBUTING.html#setup" class="tsd-anchor-icon" aria-label="Permalink"><img src="out_opendal/docs/nodejs/documents/CONTRIBUTING_media/1d480b11402974a270d65a1008580bad01955eab.svg" /></a>

Building `nodejs` bindings requires some extra setup.

For small or first-time contributions, we recommend the dev container method. Prefer to do it yourself? That's fine too!

### Using a dev container environment<a href="https://opendal.apache.org/docs/nodejs/documents/CONTRIBUTING.html#using-a-dev-container-environment" class="tsd-anchor-icon" aria-label="Permalink"><img src="out_opendal/docs/nodejs/documents/CONTRIBUTING_media/1d480b11402974a270d65a1008580bad01955eab.svg" /></a>

OpenDAL provides a pre-configured [dev container](https://containers.dev/) that could be used in [GitHub Codespaces](https://github.com/features/codespaces), [VSCode](https://code.visualstudio.com/), [JetBrains](https://www.jetbrains.com/remote-development/gateway/), [JupyterLab](https://jupyterlab.readthedocs.io/en/stable/). Please pick up your favourite runtime environment.

The fastest way is:

[![Open in GitHub Codespaces](out_opendal/docs/nodejs/documents/CONTRIBUTING_media/c2354a77453c8f3af830734ac62be03d3395706c.svg)](https://codespaces.new/apache/opendal?quickstart=1&machine=standardLinux32gb)

### Bring your own toolbox<a href="https://opendal.apache.org/docs/nodejs/documents/CONTRIBUTING.html#bring-your-own-toolbox" class="tsd-anchor-icon" aria-label="Permalink"><img src="out_opendal/docs/nodejs/documents/CONTRIBUTING_media/1d480b11402974a270d65a1008580bad01955eab.svg" /></a>

The `nodejs` binding requires `Node.js@16+` to be built. We recommend using the latest TLS version for development.

OpenDAL provides a `.node-version` file that specifies the recommended node versions. You can use any compatible tool to install the correct node version, such as [fnm](https://github.com/Schniz/fnm).

Alternatively, you can manually install the LTS node by following these steps:

For Ubuntu and Debian:

``` shell
> curl -fsSL https://deb.nodesource.com/setup_lts.x | sudo -E bash - && sudo apt-get install -y nodejs
Copy
```

For RHEL, CentOS, CloudLinux, Amazon Linux or Fedora:

``` shell
> curl -fsSL https://rpm.nodesource.com/setup_lts.x | sudo bash -
Copy
```

Afterward, you will need to enable `corepack` to ensure that `pnpm` has been set up correctly:

``` shell
> sudo corepack enable
Copy
```

To verify that everything is working properly, run `pnpm --version`:

``` shell
> pnpm --version
8.11.0
Copy
```

### Using [devbox](https://www.jetify.com/devbox/docs/)<a href="https://opendal.apache.org/docs/nodejs/documents/CONTRIBUTING.html#using-devbox" class="tsd-anchor-icon" aria-label="Permalink"><img src="out_opendal/docs/nodejs/documents/CONTRIBUTING_media/1d480b11402974a270d65a1008580bad01955eab.svg" /></a>

Launch Development Environment

``` shell
devbox shell
Copy
```

Contains the following tools:

- `node`: `18`
- `pnpm`: `8.14.0`
- `libiconv`: fix nix `ld` missing issue

[Running Scripts](https://www.jetify.com/devbox/docs/guides/scripts/)

- `devbox run format`
- `devbox run test`
- `devbox run build`
- `devbox run dev`

## Build<a href="https://opendal.apache.org/docs/nodejs/documents/CONTRIBUTING.html#build" class="tsd-anchor-icon" aria-label="Permalink"><img src="out_opendal/docs/nodejs/documents/CONTRIBUTING_media/1d480b11402974a270d65a1008580bad01955eab.svg" /></a>

``` bash
# Install dependencies.
> pnpm install
# Build from source.
> pnpm build
# Build from source with debug info.
> pnpm build:debug
Copy
```

## Test<a href="https://opendal.apache.org/docs/nodejs/documents/CONTRIBUTING.html#test" class="tsd-anchor-icon" aria-label="Permalink"><img src="out_opendal/docs/nodejs/documents/CONTRIBUTING_media/1d480b11402974a270d65a1008580bad01955eab.svg" /></a>

We are using our own developed behavior testing framework. Taking 'service-memory' as an example, you can use the following command to run it.

``` bash
> OPENDAL_TEST=memory pnpm test

â |opendal| tests/service.test.mjs  (8 tests | 2 skipped) 40ms

 Test Files  1 passed (1)
      Tests  6 passed | 2 skipped (8)
   Start at  01:42:07
   Duration  233ms (transform 25ms, setup 0ms, collect 56ms, tests 40ms, environment 0ms, prepare 52ms)
Copy
```

![](out_opendal/docs/nodejs/documents/CONTRIBUTING_media/3d34c9e8a5108625c9958e0712352b388901c004.svg)

### Settings

Member Visibility

- [x] ![](out_opendal/docs/nodejs/documents/CONTRIBUTING_media/bed852a5793c1fd9f7a03de8ebab8248a23ebeaa.svg)Inherited

ThemeOSLightDark

![](out_opendal/docs/nodejs/documents/CONTRIBUTING_media/3d34c9e8a5108625c9958e0712352b388901c004.svg)

### On This Page

[Contributing](https://opendal.apache.org/docs/nodejs/documents/CONTRIBUTING.html#contributing)

- [Setup](https://opendal.apache.org/docs/nodejs/documents/CONTRIBUTING.html#setup)
- - [Using a dev container environment](https://opendal.apache.org/docs/nodejs/documents/CONTRIBUTING.html#using-a-dev-container-environment)
  - [Bring your own toolbox](https://opendal.apache.org/docs/nodejs/documents/CONTRIBUTING.html#bring-your-own-toolbox)
  - [Using devbox](https://opendal.apache.org/docs/nodejs/documents/CONTRIBUTING.html#using-devbox)
- [Build](https://opendal.apache.org/docs/nodejs/documents/CONTRIBUTING.html#build)
- [Test](https://opendal.apache.org/docs/nodejs/documents/CONTRIBUTING.html#test)

<a href="https://opendal.apache.org/" class="tsd-nav-link">Homepage</a><a href="https://github.com/apache/opendal/tree/main/bindings/nodejs" class="tsd-nav-link">GitHub</a>

[Apache OpenDALâ„¢ - v0.49.1](https://opendal.apache.org/docs/nodejs/modules.html)

- Loading...

Copyright Â© 2022-2025, The Apache Software Foundation. Apache OpenDAL, OpenDAL, and Apache are either registered trademarks or trademarks of the Apache Software Foundation.
