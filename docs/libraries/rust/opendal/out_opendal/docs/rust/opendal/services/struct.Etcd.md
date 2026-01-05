# Struct Etcd Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/services/etcd/backend.rs.html#41-43" class="src">Source</a>

``` rust
pub struct Etcd { /* private fields */ }
```

Expand description

[Etcd](https://etcd.io/) services support.

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Etcd.html#capabilities" class="doc-anchor">Â§</a>Capabilities

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

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Etcd.html#configuration" class="doc-anchor">Â§</a>Configuration

- `root`: Set the working directory of `OpenDAL`
- `endpoints`: Set the network address of etcd servers
- `username`: Set the username of Etcd
- `password`: Set the password for authentication
- `ca_path`: Set the ca path to the etcd connection
- `cert_path`: Set the cert path to the etcd connection
- `key_path`: Set the key path to the etcd connection

You can refer to [`EtcdBuilder`](https://opendal.apache.org/docs/rust/opendal/services/struct.Etcd.html "struct opendal::services::Etcd")â€™s docs for more information

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Etcd.html#example" class="doc-anchor">Â§</a>Example

#### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Etcd.html#via-builder" class="doc-anchor">Â§</a>Via Builder

``` rust
use anyhow::Result;
use opendal::services::Etcd;
use opendal::Operator;

#[tokio::main]
async fn main() -> Result<()> {
    let mut builder = Etcd::default();

    // this will build a Operator accessing etcd which runs on http://127.0.0.1:2379
    let op: Operator = Operator::new(builder)?.finish();
    Ok(())
}
```

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Etcd.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Etcd.html#impl-EtcdBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Etcd.html" class="struct" title="struct opendal::services::Etcd">EtcdBuilder</a>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Etcd.html#method.endpoints" class="fn">endpoints</a>(self, endpoints: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-etcd`** only.

set the network address of etcd service.

default: â€œhttp://127.0.0.1:2379â€?

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Etcd.html#method.username" class="fn">username</a>(self, username: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-etcd`** only.

set the username for etcd

default: no username

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Etcd.html#method.password" class="fn">password</a>(self, password: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-etcd`** only.

set the password for etcd

default: no password

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Etcd.html#method.root" class="fn">root</a>(self, root: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-etcd`** only.

set the working directory, all operations will be performed under it.

default: â€œ/â€?

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Etcd.html#method.ca_path" class="fn">ca_path</a>(self, ca_path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-etcd`** only.

Set the certificate authority file path.

default is None

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Etcd.html#method.cert_path" class="fn">cert_path</a>(self, cert_path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-etcd`** only.

Set the certificate file path.

default is None

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Etcd.html#method.key_path" class="fn">key_path</a>(self, key_path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-etcd`** only.

Set the key file path.

default is None

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Etcd.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Etcd.html#impl-Builder-for-EtcdBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Etcd.html" class="struct" title="struct opendal::services::Etcd">EtcdBuilder</a>

Available on **crate feature `services-etcd`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Etcd.html#associatedtype.Config" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.EtcdConfig.html" class="struct" title="struct opendal::services::EtcdConfig">EtcdConfig</a>

Associated configuration for this builder.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Etcd.html#method.build" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#tymethod.build" class="fn">build</a>(self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<impl <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\>

Consume the accessor builder to build a service.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Etcd.html#impl-Clone-for-EtcdBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Etcd.html" class="struct" title="struct opendal::services::Etcd">EtcdBuilder</a>

Available on **crate feature `services-etcd`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Etcd.html#method.clone" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Etcd.html" class="struct" title="struct opendal::services::Etcd">EtcdBuilder</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Etcd.html#method.clone_from" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Etcd.html#impl-Debug-for-EtcdBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Etcd.html" class="struct" title="struct opendal::services::Etcd">EtcdBuilder</a>

Available on **crate feature `services-etcd`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Etcd.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Etcd.html#impl-Default-for-EtcdBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Etcd.html" class="struct" title="struct opendal::services::Etcd">EtcdBuilder</a>

Available on **crate feature `services-etcd`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Etcd.html#method.default" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Etcd.html" class="struct" title="struct opendal::services::Etcd">EtcdBuilder</a>

Returns the â€œdefault valueâ€? for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Etcd.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Etcd.html#blanket-implementations" class="anchor">Â§</a>
