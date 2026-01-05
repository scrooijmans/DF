# Struct VercelArtifacts Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/services/vercel_artifacts/builder.rs.html#34-39" class="src">Source</a>

``` rust
pub struct VercelArtifacts { /* private fields */ }
```

Expand description

[Vercel Cache](https://vercel.com/docs/concepts/monorepos/remote-caching) backend support.

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.VercelArtifacts.html#capabilities" class="doc-anchor">Â§</a>Capabilities

This service can be used to:

- ☐ stat
- ☒ read
- ☒ write
- ☒ create_dir
- ☒ delete
- ☐ ~~copy~~
- ☐ ~~rename~~
- ☐ ~~list~~
- ☐ ~~presign~~
- ☐ blocking

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.VercelArtifacts.html#configuration" class="doc-anchor">Â§</a>Configuration

- `access_token`: set the access_token for Rest API

You can refer to [`VercelArtifactsBuilder`](https://opendal.apache.org/docs/rust/opendal/services/struct.VercelArtifacts.html "struct opendal::services::VercelArtifacts")â€™s docs for more information

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.VercelArtifacts.html#example" class="doc-anchor">Â§</a>Example

#### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.VercelArtifacts.html#via-builder" class="doc-anchor">Â§</a>Via Builder

``` rust
use anyhow::Result;
use opendal::services::VercelArtifacts;
use opendal::Operator;

#[tokio::main]
async fn main() -> Result<()> {
    // create backend builder
    let mut builder = VercelArtifacts::default()
        .access_token("xxx");

    let op: Operator = Operator::new(builder)?.finish();
    Ok(())
}
```

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.VercelArtifacts.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.VercelArtifacts.html#impl-VercelArtifactsBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.VercelArtifacts.html" class="struct" title="struct opendal::services::VercelArtifacts">VercelArtifactsBuilder</a>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.VercelArtifacts.html#method.access_token" class="fn">access_token</a>(self, access_token: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-vercel-artifacts`** only.

set the bearer access token for Vercel

default: no access token, which leads to failure

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.VercelArtifacts.html#method.http_client" class="fn">http_client</a>(self, http_client: <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.HttpClient.html" class="struct" title="struct opendal::raw::HttpClient">HttpClient</a>) -\> Self

ðŸ‘ŽDeprecated since 0.53.0: Use `Operator::update_http_client` instead

Available on **crate feature `services-vercel-artifacts`** only.

Specify the http client that used by this service.

##### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.VercelArtifacts.html#notes" class="doc-anchor">Â§</a>Notes

This API is part of OpenDALâ€™s Raw API. `HttpClient` could be changed during minor updates.

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.VercelArtifacts.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.VercelArtifacts.html#impl-Builder-for-VercelArtifactsBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.VercelArtifacts.html" class="struct" title="struct opendal::services::VercelArtifacts">VercelArtifactsBuilder</a>

Available on **crate feature `services-vercel-artifacts`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.VercelArtifacts.html#associatedtype.Config" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.VercelArtifactsConfig.html" class="struct" title="struct opendal::services::VercelArtifactsConfig">VercelArtifactsConfig</a>

Associated configuration for this builder.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.VercelArtifacts.html#method.build" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#tymethod.build" class="fn">build</a>(self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<impl <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\>

Consume the accessor builder to build a service.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.VercelArtifacts.html#impl-Debug-for-VercelArtifactsBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.VercelArtifacts.html" class="struct" title="struct opendal::services::VercelArtifacts">VercelArtifactsBuilder</a>

Available on **crate feature `services-vercel-artifacts`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.VercelArtifacts.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.VercelArtifacts.html#impl-Default-for-VercelArtifactsBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.VercelArtifacts.html" class="struct" title="struct opendal::services::VercelArtifacts">VercelArtifactsBuilder</a>

Available on **crate feature `services-vercel-artifacts`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.VercelArtifacts.html#method.default" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.VercelArtifacts.html" class="struct" title="struct opendal::services::VercelArtifacts">VercelArtifactsBuilder</a>

Returns the â€œdefault valueâ€? for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.VercelArtifacts.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.VercelArtifacts.html#blanket-implementations" class="anchor">Â§</a>
