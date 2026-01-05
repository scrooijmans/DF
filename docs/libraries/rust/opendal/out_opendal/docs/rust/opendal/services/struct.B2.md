# Struct B2 Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/services/b2/backend.rs.html#45-50" class="src">Source</a>

``` rust
pub struct B2 { /* private fields */ }
```

Expand description

[b2](https://www.backblaze.com/cloud-storage) services support.

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.B2.html#capabilities" class="doc-anchor">Â§</a>Capabilities

This service can be used to:

- ☒ stat
- ☒ read
- ☒ write
- ☒ create_dir
- ☒ delete
- ☒ copy
- ☐ rename
- ☒ list
- ☒ presign
- ☐ blocking

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.B2.html#configuration" class="doc-anchor">Â§</a>Configuration

- `root`: Set the work directory for backend
- `key_id`: B2 application key keyID
- `application_key` B2 application key applicationKey
- `bucket` B2 bucket name
- `bucket_id` B2 bucket_id

You can refer to [`B2Builder`](https://opendal.apache.org/docs/rust/opendal/services/struct.B2.html "struct opendal::services::B2")â€™s docs for more information

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.B2.html#example" class="doc-anchor">Â§</a>Example

#### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.B2.html#via-builder" class="doc-anchor">Â§</a>Via Builder

``` rust
use anyhow::Result;
use opendal::services::B2;
use opendal::Operator;

#[tokio::main]
async fn main() -> Result<()> {
    // create backend builder
    let mut builder = B2::default()
        // set the storage bucket for OpenDAL
        .root("/")
        // set the key_id for OpenDAL
        .application_key_id("xxxxxxxxxx")
        // set the key_id for OpenDAL
        .application_key("xxxxxxxxxx")
        // set the     bucket name for OpenDAL
        .bucket("opendal")
        // set the bucket_id for OpenDAL
        .bucket_id("xxxxxxxxxxxxx");

    let op: Operator = Operator::new(builder)?.finish();

    Ok(())
}
```

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.B2.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.B2.html#impl-B2Builder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.B2.html" class="struct" title="struct opendal::services::B2">B2Builder</a>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.B2.html#method.root" class="fn">root</a>(self, root: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-b2`** only.

Set root of this backend.

All operations will happen under this root.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.B2.html#method.application_key_id" class="fn">application_key_id</a>(self, application_key_id: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-b2`** only.

application_key_id of this backend.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.B2.html#method.application_key" class="fn">application_key</a>(self, application_key: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-b2`** only.

application_key of this backend.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.B2.html#method.bucket" class="fn">bucket</a>(self, bucket: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-b2`** only.

Set bucket name of this backend. You can find it in <https://secure.backblaze.com/b2_buckets.html>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.B2.html#method.bucket_id" class="fn">bucket_id</a>(self, bucket_id: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-b2`** only.

Set bucket id of this backend. You can find it in <https://secure.backblaze.com/b2_buckets.html>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.B2.html#method.http_client" class="fn">http_client</a>(self, client: <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.HttpClient.html" class="struct" title="struct opendal::raw::HttpClient">HttpClient</a>) -\> Self

ðŸ‘ŽDeprecated since 0.53.0: Use `Operator::update_http_client` instead

Available on **crate feature `services-b2`** only.

Specify the http client that used by this service.

##### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.B2.html#notes" class="doc-anchor">Â§</a>Notes

This API is part of OpenDALâ€™s Raw API. `HttpClient` could be changed during minor updates.

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.B2.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.B2.html#impl-Builder-for-B2Builder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.B2.html" class="struct" title="struct opendal::services::B2">B2Builder</a>

Available on **crate feature `services-b2`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.B2.html#method.build" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#tymethod.build" class="fn">build</a>(self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<impl <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\>

Builds the backend and returns the result of B2Backend.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.B2.html#associatedtype.Config" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.B2Config.html" class="struct" title="struct opendal::services::B2Config">B2Config</a>

Associated configuration for this builder.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.B2.html#impl-Debug-for-B2Builder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.B2.html" class="struct" title="struct opendal::services::B2">B2Builder</a>

Available on **crate feature `services-b2`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.B2.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.B2.html#impl-Default-for-B2Builder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.B2.html" class="struct" title="struct opendal::services::B2">B2Builder</a>

Available on **crate feature `services-b2`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.B2.html#method.default" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.B2.html" class="struct" title="struct opendal::services::B2">B2Builder</a>

Returns the â€œdefault valueâ€? for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.B2.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.B2.html#blanket-implementations" class="anchor">Â§</a>
