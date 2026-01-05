# Struct Http Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/services/http/backend.rs.html#36-41" class="src">Source</a>

``` rust
pub struct Http { /* private fields */ }
```

Expand description

HTTP Read-only service support like [Nginx](https://www.nginx.com/) and [Caddy](https://caddyserver.com/).

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Http.html#capabilities" class="doc-anchor">Â§</a>Capabilities

This service can be used to:

- ☒ stat
- ☒ read
- ☐ ~~write~~
- ☐ ~~create_dir~~
- ☐ ~~delete~~
- ☐ ~~copy~~
- ☐ ~~rename~~
- ☐ ~~list~~
- ☐ ~~presign~~
- ☐ blocking

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Http.html#notes" class="doc-anchor">Â§</a>Notes

Only `read` and `stat` are supported. We can use this service to visit any HTTP Server like nginx, caddy.

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Http.html#configuration" class="doc-anchor">Â§</a>Configuration

- `endpoint`: set the endpoint for http
- `root`: Set the work directory for backend

You can refer to [`HttpBuilder`](https://opendal.apache.org/docs/rust/opendal/services/struct.Http.html "struct opendal::services::Http")â€™s docs for more information

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Http.html#example" class="doc-anchor">Â§</a>Example

#### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Http.html#via-builder" class="doc-anchor">Â§</a>Via Builder

``` rust
use anyhow::Result;
use opendal::services::Http;
use opendal::Operator;

#[tokio::main]
async fn main() -> Result<()> {
    // create http backend builder
    let mut builder = Http::default().endpoint("127.0.0.1");

    let op: Operator = Operator::new(builder)?.finish();
    Ok(())
}
```

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Http.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Http.html#impl-HttpBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Http.html" class="struct" title="struct opendal::services::Http">HttpBuilder</a>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Http.html#method.endpoint" class="fn">endpoint</a>(self, endpoint: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-http`** only.

Set endpoint for http backend.

For example: `https://example.com`

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Http.html#method.username" class="fn">username</a>(self, username: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-http`** only.

set username for http backend

default: no username

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Http.html#method.password" class="fn">password</a>(self, password: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-http`** only.

set password for http backend

default: no password

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Http.html#method.token" class="fn">token</a>(self, token: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-http`** only.

set bearer token for http backend

default: no access token

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Http.html#method.root" class="fn">root</a>(self, root: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-http`** only.

Set root path of http backend.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Http.html#method.http_client" class="fn">http_client</a>(self, client: <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.HttpClient.html" class="struct" title="struct opendal::raw::HttpClient">HttpClient</a>) -\> Self

ðŸ‘ŽDeprecated since 0.53.0: Use `Operator::update_http_client` instead

Available on **crate feature `services-http`** only.

Specify the http client that used by this service.

##### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Http.html#notes-1" class="doc-anchor">Â§</a>Notes

This API is part of OpenDALâ€™s Raw API. `HttpClient` could be changed during minor updates.

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Http.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Http.html#impl-Builder-for-HttpBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Http.html" class="struct" title="struct opendal::services::Http">HttpBuilder</a>

Available on **crate feature `services-http`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Http.html#associatedtype.Config" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.HttpConfig.html" class="struct" title="struct opendal::services::HttpConfig">HttpConfig</a>

Associated configuration for this builder.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Http.html#method.build" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#tymethod.build" class="fn">build</a>(self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<impl <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\>

Consume the accessor builder to build a service.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Http.html#impl-Debug-for-HttpBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Http.html" class="struct" title="struct opendal::services::Http">HttpBuilder</a>

Available on **crate feature `services-http`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Http.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Http.html#impl-Default-for-HttpBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Http.html" class="struct" title="struct opendal::services::Http">HttpBuilder</a>

Available on **crate feature `services-http`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Http.html#method.default" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Http.html" class="struct" title="struct opendal::services::Http">HttpBuilder</a>

Returns the â€œdefault valueâ€? for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Http.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Http.html#blanket-implementations" class="anchor">Â§</a>
