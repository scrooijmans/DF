# Struct Ftp Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/services/ftp/backend.rs.html#47-49" class="src">Source</a>

``` rust
pub struct Ftp { /* private fields */ }
```

Expand description

FTP and FTPS services support.

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Ftp.html#capabilities" class="doc-anchor">Â§</a>Capabilities

This service can be used to:

- ☒ stat
- ☒ read
- ☒ write
- ☒ create_dir
- ☒ delete
- ☐ copy
- ☐ rename
- ☒ list
- ☐ ~~presign~~
- ☐ blocking

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Ftp.html#configuration" class="doc-anchor">Â§</a>Configuration

- `endpoint`: Set the endpoint for connection
- `root`: Set the work directory for backend
- `user`: Set the login user
- `password`: Set the login password

You can refer to [`FtpBuilder`](https://opendal.apache.org/docs/rust/opendal/services/struct.Ftp.html "struct opendal::services::Ftp")â€™s docs for more information

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Ftp.html#example" class="doc-anchor">Â§</a>Example

#### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Ftp.html#via-builder" class="doc-anchor">Â§</a>Via Builder

``` rust
use anyhow::Result;
use opendal::services::Ftp;
use opendal::Operator;

#[tokio::main]
async fn main() -> Result<()> {
    let mut builder = Ftp::default()
        .endpoint("127.0.0.1");

    let op: Operator = Operator::new(builder)?.finish();
    Ok(())
}
```

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Ftp.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Ftp.html#impl-FtpBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Ftp.html" class="struct" title="struct opendal::services::Ftp">FtpBuilder</a>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Ftp.html#method.endpoint" class="fn">endpoint</a>(self, endpoint: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-ftp`** only.

set endpoint for ftp backend.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Ftp.html#method.root" class="fn">root</a>(self, root: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-ftp`** only.

set root path for ftp backend.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Ftp.html#method.user" class="fn">user</a>(self, user: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-ftp`** only.

set user for ftp backend.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Ftp.html#method.password" class="fn">password</a>(self, password: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-ftp`** only.

set password for ftp backend.

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Ftp.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Ftp.html#impl-Builder-for-FtpBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Ftp.html" class="struct" title="struct opendal::services::Ftp">FtpBuilder</a>

Available on **crate feature `services-ftp`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Ftp.html#associatedtype.Config" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.FtpConfig.html" class="struct" title="struct opendal::services::FtpConfig">FtpConfig</a>

Associated configuration for this builder.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Ftp.html#method.build" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#tymethod.build" class="fn">build</a>(self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<impl <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\>

Consume the accessor builder to build a service.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Ftp.html#impl-Debug-for-FtpBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Ftp.html" class="struct" title="struct opendal::services::Ftp">FtpBuilder</a>

Available on **crate feature `services-ftp`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Ftp.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Ftp.html#impl-Default-for-FtpBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Ftp.html" class="struct" title="struct opendal::services::Ftp">FtpBuilder</a>

Available on **crate feature `services-ftp`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Ftp.html#method.default" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Ftp.html" class="struct" title="struct opendal::services::Ftp">FtpBuilder</a>

Returns the â€œdefault valueâ€? for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Ftp.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Ftp.html#blanket-implementations" class="anchor">Â§</a>
