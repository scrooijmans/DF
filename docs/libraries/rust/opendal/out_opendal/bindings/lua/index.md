- <a href="https://opendal.apache.org/" class="breadcrumbs__link" aria-label="Home page"><img src="out_opendal/bindings/lua/index_media/edf54a765e27bedcc87f5708545b58efaaa38a1a.svg" class="breadcrumbHomeIcon_q3uS" /></a>
- <a href="https://opendal.apache.org/category/bindings/" class="breadcrumbs__link">Bindings</a>
- Lua ðŸš§

On this page

# Lua ðŸš§

## Apache OpenDALâ„¢ Lua Binding (WIP)

[<img src="out_opendal/bindings/lua/index_media/bf7005d08a67101e5f8ec3801da20c235662ba7d.svg" class="img_KBPg" decoding="async" loading="lazy" />](https://opendal.apache.org/bindings/lua/)

<img src="out_opendal/bindings/lua/index_media/f6db506ffa2fa9b49bb043591266c8aa10c32f59.jpg" class="img_KBPg" decoding="async" loading="lazy" />

## Example<a href="https://opendal.apache.org/bindings/lua/#example" class="hash-link" aria-label="Direct link to Example" translate="no" title="Direct link to Example">â€‹</a>

``` prism-code
local opendal = require("opendal")

local op, err = opendal.operator.new("fs",{root="/tmp"})
if err ~= nil then
    print(err)
    return
end
op:write("test.txt","hello world")
print("read: ", op:read("test.txt"))
```

## Lua version<a href="https://opendal.apache.org/bindings/lua/#lua-version" class="hash-link" aria-label="Direct link to Lua version" translate="no" title="Direct link to Lua version">â€‹</a>

You have to enable one of the features: lua54, lua53, lua52, lua51, luajit(52) or luau in `Cargo.toml`, according to the chosen Lua version. Default Lua version is 5.2.

## Build from source<a href="https://opendal.apache.org/bindings/lua/#build-from-source" class="hash-link" aria-label="Direct link to Build from source" translate="no" title="Direct link to Build from source">â€‹</a>

1.  Build OpenDAL Lua Interface

``` prism-code
$ cd bindings/lua
$ cargo build --package opendal-lua --release
```

2.  Install opendal Lua library

``` prism-code
# copy to lua share library directory
# default lua5.2 share library directory is /usr/lib/lua/5.2
$ cp ../../target/release/libopendal_lua.so /usr/lib/lua/5.2/opendal.so
```

## Install from LuaRocks<a href="https://opendal.apache.org/bindings/lua/#install-from-luarocks" class="hash-link" aria-label="Direct link to Install from LuaRocks" translate="no" title="Direct link to Install from LuaRocks">â€‹</a>

``` prism-code
$ luarocks make
```

## Usage<a href="https://opendal.apache.org/bindings/lua/#usage" class="hash-link" aria-label="Direct link to Usage" translate="no" title="Direct link to Usage">â€‹</a>

``` prism-code
$ lua5.2 example/fs.lua
read:   hello world
```

## Test<a href="https://opendal.apache.org/bindings/lua/#test" class="hash-link" aria-label="Direct link to Test" translate="no" title="Direct link to Test">â€‹</a>

``` prism-code
$ busted -o gtest test/opendal_test.lua
[==========] Running tests from scanned files.
[----------] Global test environment setup.
[----------] Running tests from test/opendal_test.lua
[ RUN      ] test/opendal_test.lua @ 24: opendal unit test opendal fs schema operator function in fs schema
[       OK ] test/opendal_test.lua @ 24: opendal unit test opendal fs schema operator function in fs schema (1.52 ms)
[ RUN      ] test/opendal_test.lua @ 36: opendal unit test opendal fs schema meta function in fs schema
[       OK ] test/opendal_test.lua @ 36: opendal unit test opendal fs schema meta function in fs schema (0.24 ms)
[----------] 2 tests from test/opendal_test.lua (3.47 ms total)

[----------] Global test environment teardown.
[==========] 2 tests from 1 test file ran. (3.54 ms total)
[  PASSED  ] 2 tests.
```

## License and Trademarks<a href="https://opendal.apache.org/bindings/lua/#license-and-trademarks" class="hash-link" aria-label="Direct link to License and Trademarks" translate="no" title="Direct link to License and Trademarks">â€‹</a>

Licensed under the Apache License, Version 2.0: <http://www.apache.org/licenses/LICENSE-2.0>

Apache OpenDAL, OpenDAL, and Apache are either registered trademarks or trademarks of the Apache Software Foundation.

<a href="https://github.com/apache/opendal/tree/main/website/docs/20-bindings/lua.mdx" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_opendal/bindings/lua/index_media/82254bca835e54e35c885c6543f8416b5aff021e.svg" class="iconEdit_evxu" />Edit this page</a>

Last updated on **Mar 14, 2025** by **Friends A.**

<a href="https://opendal.apache.org/bindings/java/" class="pagination-nav__link pagination-nav__link--prev"></a>

Previous

Java

<a href="https://opendal.apache.org/bindings/nodejs/" class="pagination-nav__link pagination-nav__link--next"></a>

Next

Node.js

- <a href="https://opendal.apache.org/bindings/lua/#example" class="table-of-contents__link toc-highlight">Example</a>
- <a href="https://opendal.apache.org/bindings/lua/#lua-version" class="table-of-contents__link toc-highlight">Lua version</a>
- <a href="https://opendal.apache.org/bindings/lua/#build-from-source" class="table-of-contents__link toc-highlight">Build from source</a>
- <a href="https://opendal.apache.org/bindings/lua/#install-from-luarocks" class="table-of-contents__link toc-highlight">Install from LuaRocks</a>
- <a href="https://opendal.apache.org/bindings/lua/#usage" class="table-of-contents__link toc-highlight">Usage</a>
- <a href="https://opendal.apache.org/bindings/lua/#test" class="table-of-contents__link toc-highlight">Test</a>
- <a href="https://opendal.apache.org/bindings/lua/#license-and-trademarks" class="table-of-contents__link toc-highlight">License and Trademarks</a>
