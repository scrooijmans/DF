# Struct Webdav Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/services/webdav/backend.rs.html#40-45" class="src">Source</a>

``` rust
pub struct Webdav { /* private fields */ }
```

Expand description

[WebDAV](https://datatracker.ietf.org/doc/html/rfc4918) backend support.

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Webdav.html#capabilities" class="doc-anchor">Â§</a>Capabilities

This service can be used to:

- ☒ stat
- ☒ read
- ☒ write
- ☒ create_dir
- ☒ delete
- ☒ copy
- ☒ rename
- ☒ list
- ☐ ~~presign~~
- ☐ blocking

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Webdav.html#notes" class="doc-anchor">Â§</a>Notes

Bazel Remote Caching and Ccache HTTP Storage is also part of this service. Users can use `webdav` to connect those services.

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Webdav.html#configuration" class="doc-anchor">Â§</a>Configuration

- `endpoint`: set the endpoint for webdav
- `root`: Set the work directory for backend

You can refer to [`WebdavBuilder`](https://opendal.apache.org/docs/rust/opendal/services/struct.Webdav.html "struct opendal::services::Webdav")â€™s docs for more information

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Webdav.html#example" class="doc-anchor">Â§</a>Example

#### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Webdav.html#via-builder" class="doc-anchor">Â§</a>Via Builder

``` rust
use anyhow::Result;
use opendal::services::Webdav;
use opendal::Operator;

#[tokio::main]
async fn main() -> Result<()> {
    // create backend builder
    let mut builder = Webdav::default()
        .endpoint("127.0.0.1")
        .username("xxx")
        .password("xxx");

    let op: Operator = Operator::new(builder)?.finish();

    Ok(())
}
```

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Webdav.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Webdav.html#impl-WebdavBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Webdav.html" class="struct" title="struct opendal::services::Webdav">WebdavBuilder</a>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Webdav.html#method.endpoint" class="fn">endpoint</a>(self, endpoint: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-webdav`** only.

Set endpoint for http backend.

For example: `https://example.com`

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Webdav.html#method.username" class="fn">username</a>(self, username: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-webdav`** only.

set the username for Webdav

default: no username

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Webdav.html#method.password" class="fn">password</a>(self, password: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-webdav`** only.

set the password for Webdav

default: no password

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Webdav.html#method.token" class="fn">token</a>(self, token: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-webdav`** only.

set the bearer token for Webdav

default: no access token

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Webdav.html#method.root" class="fn">root</a>(self, root: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-webdav`** only.

Set root path of http backend.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Webdav.html#method.http_client" class="fn">http_client</a>(self, client: <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.HttpClient.html" class="struct" title="struct opendal::raw::HttpClient">HttpClient</a>) -\> Self

ðŸ‘ŽDeprecated since 0.53.0: Use `Operator::update_http_client` instead

Available on **crate feature `services-webdav`** only.

Specify the http client that used by this service.

##### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Webdav.html#notes-1" class="doc-anchor">Â§</a>Notes

This API is part of OpenDALâ€™s Raw API. `HttpClient` could be changed during minor updates.

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Webdav.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Webdav.html#impl-Builder-for-WebdavBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Webdav.html" class="struct" title="struct opendal::services::Webdav">WebdavBuilder</a>

Available on **crate feature `services-webdav`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Webdav.html#associatedtype.Config" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.WebdavConfig.html" class="struct" title="struct opendal::services::WebdavConfig">WebdavConfig</a>

Associated configuration for this builder.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Webdav.html#method.build" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#tymethod.build" class="fn">build</a>(self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<impl <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\>

Consume the accessor builder to build a service.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Webdav.html#impl-Debug-for-WebdavBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Webdav.html" class="struct" title="struct opendal::services::Webdav">WebdavBuilder</a>

Available on **crate feature `services-webdav`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Webdav.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Webdav.html#impl-Default-for-WebdavBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Webdav.html" class="struct" title="struct opendal::services::Webdav">WebdavBuilder</a>

Available on **crate feature `services-webdav`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Webdav.html#method.default" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Webdav.html" class="struct" title="struct opendal::services::Webdav">WebdavBuilder</a>

Returns the â€œdefault valueâ€? for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Webdav.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Webdav.html#blanket-implementations" class="anchor">Â§</a>
