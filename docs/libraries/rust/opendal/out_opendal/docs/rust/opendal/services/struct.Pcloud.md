# Struct Pcloud Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/services/pcloud/backend.rs.html#42-47" class="src">Source</a>

``` rust
pub struct Pcloud { /* private fields */ }
```

Expand description

[pCloud](https://www.pcloud.com/) services support.

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Pcloud.html#capabilities" class="doc-anchor">Â§</a>Capabilities

This service can be used to:

- ☒ stat
- ☒ read
- ☒ write
- ☒ create_dir
- ☒ delete
- ☒ copy
- ☒ rename
- ☒ list
- ☐ presign
- ☐ blocking

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Pcloud.html#configuration" class="doc-anchor">Â§</a>Configuration

- `root`: Set the work directory for backend
- `endpoint`: Pcloud bucket name
- `username` Pcloud username
- `password` Pcloud password

You can refer to [`PcloudBuilder`](https://opendal.apache.org/docs/rust/opendal/services/struct.Pcloud.html "struct opendal::services::Pcloud")â€™s docs for more information

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Pcloud.html#example" class="doc-anchor">Â§</a>Example

#### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Pcloud.html#via-builder" class="doc-anchor">Â§</a>Via Builder

``` rust
use anyhow::Result;
use opendal::services::Pcloud;
use opendal::Operator;

#[tokio::main]
async fn main() -> Result<()> {
    // create backend builder
    let mut builder = Pcloud::default()
        // set the storage bucket for OpenDAL
        .root("/")
        // set the bucket for OpenDAL
        .endpoint("[https](https://api.pcloud.com)")
        // set the username for OpenDAL
        .username("opendal@gmail.com")
        // set the password name for OpenDAL
        .password("opendal");

    let op: Operator = Operator::new(builder)?.finish();

    Ok(())
}
```

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Pcloud.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Pcloud.html#impl-PcloudBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Pcloud.html" class="struct" title="struct opendal::services::Pcloud">PcloudBuilder</a>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Pcloud.html#method.root" class="fn">root</a>(self, root: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-pcloud`** only.

Set root of this backend.

All operations will happen under this root.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Pcloud.html#method.endpoint" class="fn">endpoint</a>(self, endpoint: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-pcloud`** only.

Pcloud endpoint. <https://api.pcloud.com> for United States and <https://eapi.pcloud.com> for Europe ref to [doc.pcloud.com](https://docs.pcloud.com/)

It is required. e.g. `https://api.pcloud.com`

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Pcloud.html#method.username" class="fn">username</a>(self, username: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-pcloud`** only.

Pcloud username.

It is required. your pCloud login email, e.g. `example@gmail.com`

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Pcloud.html#method.password" class="fn">password</a>(self, password: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-pcloud`** only.

Pcloud password.

It is required. your pCloud login password, e.g. `password`

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Pcloud.html#method.http_client" class="fn">http_client</a>(self, client: <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.HttpClient.html" class="struct" title="struct opendal::raw::HttpClient">HttpClient</a>) -\> Self

ðŸ‘ŽDeprecated since 0.53.0: Use `Operator::update_http_client` instead

Available on **crate feature `services-pcloud`** only.

Specify the http client that used by this service.

##### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Pcloud.html#notes" class="doc-anchor">Â§</a>Notes

This API is part of OpenDALâ€™s Raw API. `HttpClient` could be changed during minor updates.

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Pcloud.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Pcloud.html#impl-Builder-for-PcloudBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Pcloud.html" class="struct" title="struct opendal::services::Pcloud">PcloudBuilder</a>

Available on **crate feature `services-pcloud`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Pcloud.html#method.build" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#tymethod.build" class="fn">build</a>(self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<impl <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\>

Builds the backend and returns the result of PcloudBackend.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Pcloud.html#associatedtype.Config" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.PcloudConfig.html" class="struct" title="struct opendal::services::PcloudConfig">PcloudConfig</a>

Associated configuration for this builder.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Pcloud.html#impl-Debug-for-PcloudBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Pcloud.html" class="struct" title="struct opendal::services::Pcloud">PcloudBuilder</a>

Available on **crate feature `services-pcloud`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Pcloud.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Pcloud.html#impl-Default-for-PcloudBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Pcloud.html" class="struct" title="struct opendal::services::Pcloud">PcloudBuilder</a>

Available on **crate feature `services-pcloud`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Pcloud.html#method.default" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Pcloud.html" class="struct" title="struct opendal::services::Pcloud">PcloudBuilder</a>

Returns the â€œdefault valueâ€? for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Pcloud.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Pcloud.html#blanket-implementations" class="anchor">Â§</a>
