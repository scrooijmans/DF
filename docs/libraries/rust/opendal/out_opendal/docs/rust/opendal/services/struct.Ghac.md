# Struct Ghac Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/services/ghac/backend.rs.html#54-59" class="src">Source</a>

``` rust
pub struct Ghac { /* private fields */ }
```

Expand description

GitHub Action Cache Services support.

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Ghac.html#capabilities" class="doc-anchor">Â§</a>Capabilities

This service can be used to:

- ☒ stat
- ☒ read
- ☒ write
- ☒ create_dir
- ☒ delete
- ☒ copy
- ☐ rename
- ☐ list
- ☐ presign
- ☐ blocking

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Ghac.html#notes" class="doc-anchor">Â§</a>Notes

This service is mainly provided by GitHub actions.

Refer to [Caching dependencies to speed up workflows](https://docs.github.com/en/actions/using-workflows/caching-dependencies-to-speed-up-workflows) for more information.

To make this service work as expected, please make sure to either call `endpoint` and `token` to configure the URL and credentials, or that the following environment has been setup correctly:

- `ACTIONS_CACHE_URL`
- `ACTIONS_RUNTIME_TOKEN`

They can be exposed by following action:

``` yaml
- name: Configure Cache Env
  uses: actions/github-script@v6
  with:
    script: |
      core.exportVariable('ACTIONS_CACHE_URL', process.env.ACTIONS_CACHE_URL || '');
      core.exportVariable('ACTIONS_RUNTIME_TOKEN', process.env.ACTIONS_RUNTIME_TOKEN || '');
```

To make `delete` work as expected, `GITHUB_TOKEN` should also be set via:

``` yaml
env:
  GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
```

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Ghac.html#limitations" class="doc-anchor">Â§</a>Limitations

Unlike other services, ghac doesnâ€™t support create empty files. We provide a `enable_create_simulation()` to support this operation but may result unexpected side effects.

Also, `ghac` is a cache service which means the data store inside could be automatically evicted at any time.

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Ghac.html#configuration" class="doc-anchor">Â§</a>Configuration

- `root`: Set the work dir for backend.

Refer to [`GhacBuilder`](https://opendal.apache.org/docs/rust/opendal/services/struct.Ghac.html "struct opendal::services::Ghac")â€™s public API docs for more information.

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Ghac.html#example" class="doc-anchor">Â§</a>Example

#### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Ghac.html#via-builder" class="doc-anchor">Â§</a>Via Builder

``` rust
use std::sync::Arc;

use anyhow::Result;
use opendal::services::Ghac;
use opendal::Operator;

#[tokio::main]
async fn main() -> Result<()> {
    // Create ghac backend builder.
    let mut builder = Ghac::default()
        // Set the root for ghac, all operations will happen under this root.
        //
        // NOTE: the root must be absolute path.
        .root("/path/to/dir");

    let op: Operator = Operator::new(builder)?.finish();

    Ok(())
}
```

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Ghac.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Ghac.html#impl-GhacBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Ghac.html" class="struct" title="struct opendal::services::Ghac">GhacBuilder</a>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Ghac.html#method.root" class="fn">root</a>(self, root: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-ghac`** only.

set the working directory root of backend

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Ghac.html#method.version" class="fn">version</a>(self, version: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-ghac`** only.

set the version that used by cache.

The version is the unique value that provides namespacing. Itâ€™s better to make sure this value is only used by this backend.

If not set, we will use `opendal` as default.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Ghac.html#method.endpoint" class="fn">endpoint</a>(self, endpoint: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-ghac`** only.

Set the endpoint for ghac service.

For example, this is provided as the `ACTIONS_CACHE_URL` environment variable by the GHA runner.

Default: the value of the `ACTIONS_CACHE_URL` environment variable.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Ghac.html#method.runtime_token" class="fn">runtime_token</a>(self, runtime_token: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-ghac`** only.

Set the runtime token for ghac service.

For example, this is provided as the `ACTIONS_RUNTIME_TOKEN` environment variable by the GHA runner.

Default: the value of the `ACTIONS_RUNTIME_TOKEN` environment variable.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Ghac.html#method.http_client" class="fn">http_client</a>(self, client: <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.HttpClient.html" class="struct" title="struct opendal::raw::HttpClient">HttpClient</a>) -\> Self

ðŸ‘ŽDeprecated since 0.53.0: Use `Operator::update_http_client` instead

Available on **crate feature `services-ghac`** only.

Specify the http client that used by this service.

##### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Ghac.html#notes-1" class="doc-anchor">Â§</a>Notes

This API is part of OpenDALâ€™s Raw API. `HttpClient` could be changed during minor updates.

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Ghac.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Ghac.html#impl-Builder-for-GhacBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Ghac.html" class="struct" title="struct opendal::services::Ghac">GhacBuilder</a>

Available on **crate feature `services-ghac`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Ghac.html#associatedtype.Config" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.GhacConfig.html" class="struct" title="struct opendal::services::GhacConfig">GhacConfig</a>

Associated configuration for this builder.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Ghac.html#method.build" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#tymethod.build" class="fn">build</a>(self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<impl <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\>

Consume the accessor builder to build a service.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Ghac.html#impl-Debug-for-GhacBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Ghac.html" class="struct" title="struct opendal::services::Ghac">GhacBuilder</a>

Available on **crate feature `services-ghac`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Ghac.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Ghac.html#impl-Default-for-GhacBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Ghac.html" class="struct" title="struct opendal::services::Ghac">GhacBuilder</a>

Available on **crate feature `services-ghac`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Ghac.html#method.default" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Ghac.html" class="struct" title="struct opendal::services::Ghac">GhacBuilder</a>

Returns the â€œdefault valueâ€? for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Ghac.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Ghac.html#blanket-implementations" class="anchor">Â§</a>
