# Struct Ipmfs Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/services/ipmfs/builder.rs.html#70-75" class="src">Source</a>

``` rust
pub struct Ipmfs { /* private fields */ }
```

Expand description

IPFS file system support based on [IPFS MFS](https://docs.ipfs.tech/concepts/file-systems/) API.

## <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Ipmfs.html#capabilities" class="doc-anchor">Â§</a>Capabilities

This service can be used to:

- ☒ read
- ☒ write
- ☒ list
- ☐ presign
- ☐ blocking

## <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Ipmfs.html#configuration" class="doc-anchor">Â§</a>Configuration

- `root`: Set the work directory for backend
- `endpoint`: Customizable endpoint setting

You can refer to [`IpmfsBuilder`](https://opendal.apache.org/docs/rust/opendal/services/struct.Ipmfs.html "struct opendal::services::Ipmfs")â€™s docs for more information

## <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Ipmfs.html#example" class="doc-anchor">Â§</a>Example

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Ipmfs.html#via-builder" class="doc-anchor">Â§</a>Via Builder

``` rust
use anyhow::Result;
use opendal::services::Ipmfs;
use opendal::Operator;

#[tokio::main]
async fn main() -> Result<()> {
    // create backend builder
    let mut builder = Ipmfs::default()
        // set the storage bucket for OpenDAL
        .endpoint("http://127.0.0.1:5001");

    let op: Operator = Operator::new(builder)?.finish();

    Ok(())
}
```

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Ipmfs.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Ipmfs.html#impl-IpmfsBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Ipmfs.html" class="struct" title="struct opendal::services::Ipmfs">IpmfsBuilder</a>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Ipmfs.html#method.root" class="fn">root</a>(self, root: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-ipmfs`** only.

Set root for ipfs.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Ipmfs.html#method.endpoint" class="fn">endpoint</a>(self, endpoint: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-ipmfs`** only.

Set endpoint for ipfs.

Default: http://localhost:5001

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Ipmfs.html#method.http_client" class="fn">http_client</a>(self, client: <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.HttpClient.html" class="struct" title="struct opendal::raw::HttpClient">HttpClient</a>) -\> Self

ðŸ‘ŽDeprecated since 0.53.0: Use `Operator::update_http_client` instead

Available on **crate feature `services-ipmfs`** only.

Specify the http client that used by this service.

##### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Ipmfs.html#notes" class="doc-anchor">Â§</a>Notes

This API is part of OpenDALâ€™s Raw API. `HttpClient` could be changed during minor updates.

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Ipmfs.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Ipmfs.html#impl-Builder-for-IpmfsBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Ipmfs.html" class="struct" title="struct opendal::services::Ipmfs">IpmfsBuilder</a>

Available on **crate feature `services-ipmfs`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Ipmfs.html#associatedtype.Config" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.IpmfsConfig.html" class="struct" title="struct opendal::services::IpmfsConfig">IpmfsConfig</a>

Associated configuration for this builder.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Ipmfs.html#method.build" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#tymethod.build" class="fn">build</a>(self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<impl <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\>

Consume the accessor builder to build a service.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Ipmfs.html#impl-Debug-for-IpmfsBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Ipmfs.html" class="struct" title="struct opendal::services::Ipmfs">IpmfsBuilder</a>

Available on **crate feature `services-ipmfs`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Ipmfs.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Ipmfs.html#impl-Default-for-IpmfsBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Ipmfs.html" class="struct" title="struct opendal::services::Ipmfs">IpmfsBuilder</a>

Available on **crate feature `services-ipmfs`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Ipmfs.html#method.default" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Ipmfs.html" class="struct" title="struct opendal::services::Ipmfs">IpmfsBuilder</a>

Returns the â€œdefault valueâ€? for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Ipmfs.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Ipmfs.html#blanket-implementations" class="anchor">Â§</a>
