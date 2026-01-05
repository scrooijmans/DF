# Struct Koofr Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/services/koofr/backend.rs.html#45-50" class="src">Source</a>

``` rust
pub struct Koofr { /* private fields */ }
```

Expand description

[Koofr](https://app.koofr.net/) services support.

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Koofr.html#capabilities" class="doc-anchor">Â§</a>Capabilities

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

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Koofr.html#configuration" class="doc-anchor">Â§</a>Configuration

- `root`: Set the work directory for backend
- `endpoint`: Koofr endpoint
- `email` Koofr email
- `password` Koofr password

You can refer to [`KoofrBuilder`](https://opendal.apache.org/docs/rust/opendal/services/struct.Koofr.html "struct opendal::services::Koofr")â€™s docs for more information

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Koofr.html#example" class="doc-anchor">Â§</a>Example

#### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Koofr.html#via-builder" class="doc-anchor">Â§</a>Via Builder

``` rust
use anyhow::Result;
use opendal::services::Koofr;
use opendal::Operator;

#[tokio::main]
async fn main() -> Result<()> {
    // create backend builder
    let mut builder = Koofr::default()
        // set the storage bucket for OpenDAL
        .root("/")
        // set the bucket for OpenDAL
        .endpoint("https://api.koofr.net/")
        // set the email for OpenDAL
        .email("me@example.com")
        // set the password for OpenDAL
        .password("xxx xxx xxx xxx");

    let op: Operator = Operator::new(builder)?.finish();

    Ok(())
}
```

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Koofr.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Koofr.html#impl-KoofrBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Koofr.html" class="struct" title="struct opendal::services::Koofr">KoofrBuilder</a>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Koofr.html#method.root" class="fn">root</a>(self, root: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-koofr`** only.

Set root of this backend.

All operations will happen under this root.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Koofr.html#method.endpoint" class="fn">endpoint</a>(self, endpoint: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-koofr`** only.

endpoint.

It is required. e.g. `https://api.koofr.net/`

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Koofr.html#method.email" class="fn">email</a>(self, email: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-koofr`** only.

email.

It is required. e.g. `test@example.com`

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Koofr.html#method.password" class="fn">password</a>(self, password: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-koofr`** only.

Koofr application password.

Go to <https://app.koofr.net/app/admin/preferences/password>. Click â€œGenerate Passwordâ€? button to generate a new application password.

##### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Koofr.html#notes" class="doc-anchor">Â§</a>Notes

This is not userâ€™s Koofr account password. Please use the application password instead. Please also remind users of this.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Koofr.html#method.http_client" class="fn">http_client</a>(self, client: <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.HttpClient.html" class="struct" title="struct opendal::raw::HttpClient">HttpClient</a>) -\> Self

ðŸ‘ŽDeprecated since 0.53.0: Use `Operator::update_http_client` instead

Available on **crate feature `services-koofr`** only.

Specify the http client that used by this service.

##### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Koofr.html#notes-1" class="doc-anchor">Â§</a>Notes

This API is part of OpenDALâ€™s Raw API. `HttpClient` could be changed during minor updates.

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Koofr.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Koofr.html#impl-Builder-for-KoofrBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Koofr.html" class="struct" title="struct opendal::services::Koofr">KoofrBuilder</a>

Available on **crate feature `services-koofr`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Koofr.html#method.build" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#tymethod.build" class="fn">build</a>(self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<impl <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\>

Builds the backend and returns the result of KoofrBackend.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Koofr.html#associatedtype.Config" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.KoofrConfig.html" class="struct" title="struct opendal::services::KoofrConfig">KoofrConfig</a>

Associated configuration for this builder.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Koofr.html#impl-Debug-for-KoofrBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Koofr.html" class="struct" title="struct opendal::services::Koofr">KoofrBuilder</a>

Available on **crate feature `services-koofr`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Koofr.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Koofr.html#impl-Default-for-KoofrBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Koofr.html" class="struct" title="struct opendal::services::Koofr">KoofrBuilder</a>

Available on **crate feature `services-koofr`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Koofr.html#method.default" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Koofr.html" class="struct" title="struct opendal::services::Koofr">KoofrBuilder</a>

Returns the â€œdefault valueâ€? for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Koofr.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Koofr.html#blanket-implementations" class="anchor">Â§</a>
