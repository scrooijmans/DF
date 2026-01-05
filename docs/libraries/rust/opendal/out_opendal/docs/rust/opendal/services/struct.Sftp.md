# Struct Sftp Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/services/sftp/backend.rs.html#54-56" class="src">Source</a>

``` rust
pub struct Sftp { /* private fields */ }
```

Expand description

SFTP services support. (only works on unix)

If you are interested in working on windows, please refer to [this](https://github.com/apache/opendal/issues/2963) issue. Welcome to leave your comments or make contributions.

Warning: Maximum number of file holdings is depending on the remote system configuration.

For example, the default value is 255 in macOS, and 1024 in linux. If you want to open lots of files, you should pay attention to close the file after using it.

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Sftp.html#capabilities" class="doc-anchor">Â§</a>Capabilities

This service can be used to:

- ☒ stat
- ☒ read
- ☒ write
- ☒ append
- ☒ create_dir
- ☒ delete
- ☒ copy
- ☒ rename
- ☒ list
- ☐ ~~presign~~
- ☐ blocking

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Sftp.html#configuration" class="doc-anchor">Â§</a>Configuration

- `endpoint`: Set the endpoint for connection. The format is same as `openssh`, using either `[user@]hostname` or `ssh://[user@]hostname[:port]`. A username or port that is specified in the endpoint overrides the one set in the builder (but does not change the builder).
- `root`: Set the work directory for backend. It uses the default directory set by the remote `sftp-server` as default
- `user`: Set the login user
- `key`: Set the public key for login
- `known_hosts_strategy`: Set the strategy for known hosts, default to `Strict`
- `enable_copy`: Set whether the remote server has copy-file extension

For security reasons, it doesnâ€™t support password login, you can use public key or ssh-copy-id instead.

You can refer to [`SftpBuilder`](https://opendal.apache.org/docs/rust/opendal/services/struct.Sftp.html "struct opendal::services::Sftp")â€™s docs for more information

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Sftp.html#example" class="doc-anchor">Â§</a>Example

#### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Sftp.html#via-builder" class="doc-anchor">Â§</a>Via Builder

``` rust
use anyhow::Result;
use opendal::services::Sftp;
use opendal::Operator;

#[tokio::main]
async fn main() -> Result<()> {
    let mut builder = Sftp::default()
        .endpoint("127.0.0.1")
        .user("test")
        .key("test_key");

    let op: Operator = Operator::new(builder)?.finish();
    Ok(())
}
```

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Sftp.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Sftp.html#impl-SftpBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Sftp.html" class="struct" title="struct opendal::services::Sftp">SftpBuilder</a>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Sftp.html#method.endpoint" class="fn">endpoint</a>(self, endpoint: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-sftp`** only.

set endpoint for sftp backend. The format is same as `openssh`, using either `[user@]hostname` or `ssh://[user@]hostname[:port]`. A username or port that is specified in the endpoint overrides the one set in the builder (but does not change the builder).

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Sftp.html#method.root" class="fn">root</a>(self, root: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-sftp`** only.

set root path for sftp backend. It uses the default directory set by the remote `sftp-server` as default.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Sftp.html#method.user" class="fn">user</a>(self, user: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-sftp`** only.

set user for sftp backend.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Sftp.html#method.key" class="fn">key</a>(self, key: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-sftp`** only.

set key path for sftp backend.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Sftp.html#method.known_hosts_strategy" class="fn">known_hosts_strategy</a>(self, strategy: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-sftp`** only.

set known_hosts strategy for sftp backend. available values:

- Strict (default)
- Accept
- Add

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Sftp.html#method.enable_copy" class="fn">enable_copy</a>(self, enable_copy: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> Self

Available on **crate feature `services-sftp`** only.

set enable_copy for sftp backend. It requires the server supports copy-file extension.

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Sftp.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Sftp.html#impl-Builder-for-SftpBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Sftp.html" class="struct" title="struct opendal::services::Sftp">SftpBuilder</a>

Available on **crate feature `services-sftp`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Sftp.html#associatedtype.Config" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.SftpConfig.html" class="struct" title="struct opendal::services::SftpConfig">SftpConfig</a>

Associated configuration for this builder.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Sftp.html#method.build" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#tymethod.build" class="fn">build</a>(self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<impl <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\>

Consume the accessor builder to build a service.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Sftp.html#impl-Debug-for-SftpBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Sftp.html" class="struct" title="struct opendal::services::Sftp">SftpBuilder</a>

Available on **crate feature `services-sftp`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Sftp.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Sftp.html#impl-Default-for-SftpBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Sftp.html" class="struct" title="struct opendal::services::Sftp">SftpBuilder</a>

Available on **crate feature `services-sftp`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Sftp.html#method.default" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Sftp.html" class="struct" title="struct opendal::services::Sftp">SftpBuilder</a>

Returns the â€œdefault valueâ€? for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Sftp.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Sftp.html#blanket-implementations" class="anchor">Â§</a>
