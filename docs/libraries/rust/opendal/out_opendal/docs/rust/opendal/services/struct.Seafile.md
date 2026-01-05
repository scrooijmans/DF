# Struct Seafile Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/services/seafile/backend.rs.html#44-49" class="src">Source</a>

``` rust
pub struct Seafile { /* private fields */ }
```

Expand description

[seafile](https://www.seafile.com) services support.

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Seafile.html#capabilities" class="doc-anchor">Â§</a>Capabilities

This service can be used to:

- ☒ stat
- ☒ read
- ☒ write
- ☒ create_dir
- ☒ delete
- ☐ copy
- ☐ rename
- ☒ list
- ☐ presign
- ☐ blocking

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Seafile.html#configuration" class="doc-anchor">Â§</a>Configuration

- `root`: Set the work directory for backend
- `endpoint`: Seafile endpoint address
- `username` Seafile username
- `password` Seafile password
- `repo_name` Seafile repo name

You can refer to [`SeafileBuilder`](https://opendal.apache.org/docs/rust/opendal/services/struct.Seafile.html "struct opendal::services::Seafile")â€™s docs for more information

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Seafile.html#example" class="doc-anchor">Â§</a>Example

#### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Seafile.html#via-builder" class="doc-anchor">Â§</a>Via Builder

``` rust
use anyhow::Result;
use opendal::services::Seafile;
use opendal::Operator;

#[tokio::main]
async fn main() -> Result<()> {
    // create backend builder
    let mut builder = Seafile::default()
        // set the storage bucket for OpenDAL
        .root("/")
        // set the endpoint for OpenDAL
        .endpoint("http://127.0.0.1:80")
        // set the username for OpenDAL
        .username("xxxxxxxxxx")
        // set the password name for OpenDAL
        .password("opendal")
        // set the repo_name for OpenDAL
        .repo_name("xxxxxxxxxxxxx");

    let op: Operator = Operator::new(builder)?.finish();

    Ok(())
}
```

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Seafile.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Seafile.html#impl-SeafileBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Seafile.html" class="struct" title="struct opendal::services::Seafile">SeafileBuilder</a>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Seafile.html#method.root" class="fn">root</a>(self, root: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-seafile`** only.

Set root of this backend.

All operations will happen under this root.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Seafile.html#method.endpoint" class="fn">endpoint</a>(self, endpoint: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-seafile`** only.

endpoint of this backend.

It is required. e.g. `http://127.0.0.1:80`

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Seafile.html#method.username" class="fn">username</a>(self, username: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-seafile`** only.

username of this backend.

It is required. e.g. `me@example.com`

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Seafile.html#method.password" class="fn">password</a>(self, password: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-seafile`** only.

password of this backend.

It is required. e.g. `asecret`

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Seafile.html#method.repo_name" class="fn">repo_name</a>(self, repo_name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-seafile`** only.

Set repo name of this backend.

It is required. e.g. `myrepo`

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Seafile.html#method.http_client" class="fn">http_client</a>(self, client: <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.HttpClient.html" class="struct" title="struct opendal::raw::HttpClient">HttpClient</a>) -\> Self

ðŸ‘ŽDeprecated since 0.53.0: Use `Operator::update_http_client` instead

Available on **crate feature `services-seafile`** only.

Specify the http client that used by this service.

##### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Seafile.html#notes" class="doc-anchor">Â§</a>Notes

This API is part of OpenDALâ€™s Raw API. `HttpClient` could be changed during minor updates.

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Seafile.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Seafile.html#impl-Builder-for-SeafileBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Seafile.html" class="struct" title="struct opendal::services::Seafile">SeafileBuilder</a>

Available on **crate feature `services-seafile`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Seafile.html#method.build" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#tymethod.build" class="fn">build</a>(self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<impl <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\>

Builds the backend and returns the result of SeafileBackend.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Seafile.html#associatedtype.Config" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.SeafileConfig.html" class="struct" title="struct opendal::services::SeafileConfig">SeafileConfig</a>

Associated configuration for this builder.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Seafile.html#impl-Debug-for-SeafileBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Seafile.html" class="struct" title="struct opendal::services::Seafile">SeafileBuilder</a>

Available on **crate feature `services-seafile`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Seafile.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Seafile.html#impl-Default-for-SeafileBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Seafile.html" class="struct" title="struct opendal::services::Seafile">SeafileBuilder</a>

Available on **crate feature `services-seafile`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Seafile.html#method.default" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Seafile.html" class="struct" title="struct opendal::services::Seafile">SeafileBuilder</a>

Returns the â€œdefault valueâ€? for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Seafile.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Seafile.html#blanket-implementations" class="anchor">Â§</a>
