# Struct VercelBlob Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/services/vercel_blob/backend.rs.html#43-48" class="src">Source</a>

``` rust
pub struct VercelBlob { /* private fields */ }
```

Expand description

[VercelBlob](https://vercel.com/docs/storage/vercel-blob) services support.

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.VercelBlob.html#capabilities" class="doc-anchor">Â§</a>Capabilities

This service can be used to:

- ☒ stat
- ☒ read
- ☒ write
- ☒ create_dir
- ☒ delete
- ☒ copy
- ☐ rename
- ☒ list
- ☐ presign
- ☐ blocking

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.VercelBlob.html#configuration" class="doc-anchor">Â§</a>Configuration

- `root`: Set the work directory for backend
- `token`: VercelBlob token, environment var `BLOB_READ_WRITE_TOKEN`

You can refer to [`VercelBlobBuilder`](https://opendal.apache.org/docs/rust/opendal/services/struct.VercelBlob.html "struct opendal::services::VercelBlob")â€™s docs for more information

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.VercelBlob.html#example" class="doc-anchor">Â§</a>Example

#### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.VercelBlob.html#via-builder" class="doc-anchor">Â§</a>Via Builder

``` rust
use anyhow::Result;
use opendal::services::VercelBlob;
use opendal::Operator;

#[tokio::main]
async fn main() -> Result<()> {
    // create backend builder
    let mut builder = VercelBlob::default()
        // set the storage bucket for OpenDAL
        .root("/")
        // set the token for OpenDAL
        .token("you_token");

    let op: Operator = Operator::new(builder)?.finish();

    Ok(())
}
```

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.VercelBlob.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.VercelBlob.html#impl-VercelBlobBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.VercelBlob.html" class="struct" title="struct opendal::services::VercelBlob">VercelBlobBuilder</a>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.VercelBlob.html#method.root" class="fn">root</a>(self, root: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-vercel-blob`** only.

Set root of this backend.

All operations will happen under this root.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.VercelBlob.html#method.token" class="fn">token</a>(self, token: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-vercel-blob`** only.

Vercel Blob token.

Get from Vercel environment variable `BLOB_READ_WRITE_TOKEN`. It is required.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.VercelBlob.html#method.http_client" class="fn">http_client</a>(self, client: <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.HttpClient.html" class="struct" title="struct opendal::raw::HttpClient">HttpClient</a>) -\> Self

ðŸ‘ŽDeprecated since 0.53.0: Use `Operator::update_http_client` instead

Available on **crate feature `services-vercel-blob`** only.

Specify the http client that used by this service.

##### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.VercelBlob.html#notes" class="doc-anchor">Â§</a>Notes

This API is part of OpenDALâ€™s Raw API. `HttpClient` could be changed during minor updates.

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.VercelBlob.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.VercelBlob.html#impl-Builder-for-VercelBlobBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.VercelBlob.html" class="struct" title="struct opendal::services::VercelBlob">VercelBlobBuilder</a>

Available on **crate feature `services-vercel-blob`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.VercelBlob.html#method.build" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#tymethod.build" class="fn">build</a>(self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<impl <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\>

Builds the backend and returns the result of VercelBlobBackend.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.VercelBlob.html#associatedtype.Config" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.VercelBlobConfig.html" class="struct" title="struct opendal::services::VercelBlobConfig">VercelBlobConfig</a>

Associated configuration for this builder.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.VercelBlob.html#impl-Debug-for-VercelBlobBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.VercelBlob.html" class="struct" title="struct opendal::services::VercelBlob">VercelBlobBuilder</a>

Available on **crate feature `services-vercel-blob`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.VercelBlob.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.VercelBlob.html#impl-Default-for-VercelBlobBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.VercelBlob.html" class="struct" title="struct opendal::services::VercelBlob">VercelBlobBuilder</a>

Available on **crate feature `services-vercel-blob`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.VercelBlob.html#method.default" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.VercelBlob.html" class="struct" title="struct opendal::services::VercelBlob">VercelBlobBuilder</a>

Returns the â€œdefault valueâ€? for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.VercelBlob.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.VercelBlob.html#blanket-implementations" class="anchor">Â§</a>
