# Struct Memcached Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/services/memcached/backend.rs.html#33-35" class="src">Source</a>

``` rust
pub struct Memcached { /* private fields */ }
```

Expand description

[Memcached](https://memcached.org/) service support.

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Memcached.html#capabilities" class="doc-anchor">Â§</a>Capabilities

This service can be used to:

- ☐ create_dir
- ☒ stat
- ☒ read
- ☒ write
- ☒ delete
- ☐ copy
- ☐ rename
- ☐ list
- ☐ ~~presign~~

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Memcached.html#configuration" class="doc-anchor">Â§</a>Configuration

- `root`: Set the working directory of `OpenDAL`
- `username`: Set the username for authentication.
- `password`: Set the password for authentication.
- `endpoint`: Set the network address of memcached server
- `default_ttl`: Set the ttl for memcached service.

You can refer to [`MemcachedBuilder`](https://opendal.apache.org/docs/rust/opendal/services/struct.Memcached.html "struct opendal::services::Memcached")â€™s docs for more information

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Memcached.html#example" class="doc-anchor">Â§</a>Example

#### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Memcached.html#via-builder" class="doc-anchor">Â§</a>Via Builder

``` rust
use anyhow::Result;
use opendal::services::Memcached;
use opendal::Operator;

#[tokio::main]
async fn main() -> Result<()> {
    // create memcached backend builder
    let mut builder = Memcached::default()
        .endpoint("tcp://127.0.0.1:11211");
        // if you enable authentication, set username and password for authentication
        // builder.username("admin")
        // builder.password("password");

    let op: Operator = Operator::new(builder)?.finish();
    Ok(())
}
```

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Memcached.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Memcached.html#impl-MemcachedBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Memcached.html" class="struct" title="struct opendal::services::Memcached">MemcachedBuilder</a>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Memcached.html#method.endpoint" class="fn">endpoint</a>(self, endpoint: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-memcached`** only.

set the network address of memcached service.

For example: â€œtcp://localhost:11211â€?

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Memcached.html#method.root" class="fn">root</a>(self, root: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-memcached`** only.

set the working directory, all operations will be performed under it.

default: â€œ/â€?

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Memcached.html#method.username" class="fn">username</a>(self, username: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-memcached`** only.

set the username.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Memcached.html#method.password" class="fn">password</a>(self, password: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-memcached`** only.

set the password.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Memcached.html#method.default_ttl" class="fn">default_ttl</a>(self, ttl: <a href="https://doc.rust-lang.org/nightly/core/time/struct.Duration.html" class="struct" title="struct core::time::Duration">Duration</a>) -\> Self

Available on **crate feature `services-memcached`** only.

Set the default ttl for memcached services.

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Memcached.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Memcached.html#impl-Builder-for-MemcachedBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Memcached.html" class="struct" title="struct opendal::services::Memcached">MemcachedBuilder</a>

Available on **crate feature `services-memcached`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Memcached.html#associatedtype.Config" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.MemcachedConfig.html" class="struct" title="struct opendal::services::MemcachedConfig">MemcachedConfig</a>

Associated configuration for this builder.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Memcached.html#method.build" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#tymethod.build" class="fn">build</a>(self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<impl <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\>

Consume the accessor builder to build a service.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Memcached.html#impl-Clone-for-MemcachedBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Memcached.html" class="struct" title="struct opendal::services::Memcached">MemcachedBuilder</a>

Available on **crate feature `services-memcached`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Memcached.html#method.clone" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Memcached.html" class="struct" title="struct opendal::services::Memcached">MemcachedBuilder</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Memcached.html#method.clone_from" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Memcached.html#impl-Default-for-MemcachedBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Memcached.html" class="struct" title="struct opendal::services::Memcached">MemcachedBuilder</a>

Available on **crate feature `services-memcached`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Memcached.html#method.default" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Memcached.html" class="struct" title="struct opendal::services::Memcached">MemcachedBuilder</a>

Returns the â€œdefault valueâ€? for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Memcached.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Memcached.html#blanket-implementations" class="anchor">Â§</a>
