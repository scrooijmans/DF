# Struct Redis Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/services/redis/backend.rs.html#46-48" class="src">Source</a>

``` rust
pub struct Redis { /* private fields */ }
```

Expand description

[Redis](https://redis.io/) services support.

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Redis.html#capabilities" class="doc-anchor">Â§</a>Capabilities

This service can be used to:

- ☒ stat
- ☒ read
- ☒ write
- ☒ delete
- ☐ ~~create_dir~~
- ☐ ~~copy~~
- ☐ ~~rename~~
- ☐ ~~list~~
- ☐ ~~presign~~

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Redis.html#configuration" class="doc-anchor">Â§</a>Configuration

- `root`: Set the working directory of `OpenDAL`
- `endpoint`: Set the network address of redis server
- `cluster_endpoints`: Set the network address of redis cluster server. This parameter is mutually exclusive with the `endpoint` parameter.
- `username`: Set the username of Redis
- `password`: Set the password for authentication
- `db`: Set the DB of redis

You can refer to [`RedisBuilder`](https://opendal.apache.org/docs/rust/opendal/services/struct.Redis.html "struct opendal::services::Redis")â€™s docs for more information

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Redis.html#example" class="doc-anchor">Â§</a>Example

#### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Redis.html#via-builder" class="doc-anchor">Â§</a>Via Builder

``` rust
use anyhow::Result;
use opendal::services::Redis;
use opendal::Operator;

#[tokio::main]
async fn main() -> Result<()> {
    let mut builder = Redis::default();

    // this will build a Operator accessing Redis which runs on tcp://localhost:6379
    let op: Operator = Operator::new(builder)?.finish();
    Ok(())
}
```

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Redis.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Redis.html#impl-RedisBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Redis.html" class="struct" title="struct opendal::services::Redis">RedisBuilder</a>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Redis.html#method.endpoint" class="fn">endpoint</a>(self, endpoint: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-redis`** only.

set the network address of redis service.

currently supported schemes:

- no scheme: will be seen as â€œtcpâ€?
- â€œtcpâ€? or â€œredisâ€?: unsecured redis connections
- â€œredissâ€?: secured redis connections
- â€œunixâ€? or â€œredis+unixâ€?: unix socket connection

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Redis.html#method.cluster_endpoints" class="fn">cluster_endpoints</a>(self, cluster_endpoints: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-redis`** only.

set the network address of redis cluster service. This parameter is mutually exclusive with the endpoint parameter.

currently supported schemes:

- no scheme: will be seen as â€œtcpâ€?
- â€œtcpâ€? or â€œredisâ€?: unsecured redis connections
- â€œredissâ€?: secured redis connections
- â€œunixâ€? or â€œredis+unixâ€?: unix socket connection

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Redis.html#method.username" class="fn">username</a>(self, username: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-redis`** only.

set the username for redis

default: no username

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Redis.html#method.password" class="fn">password</a>(self, password: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-redis`** only.

set the password for redis

default: no password

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Redis.html#method.db" class="fn">db</a>(self, db: <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>) -\> Self

Available on **crate feature `services-redis`** only.

set the db used in redis

default: 0

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Redis.html#method.default_ttl" class="fn">default_ttl</a>(self, ttl: <a href="https://doc.rust-lang.org/nightly/core/time/struct.Duration.html" class="struct" title="struct core::time::Duration">Duration</a>) -\> Self

Available on **crate feature `services-redis`** only.

Set the default ttl for redis services.

If set, we will specify `EX` for write operations.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Redis.html#method.root" class="fn">root</a>(self, root: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-redis`** only.

set the working directory, all operations will be performed under it.

default: â€œ/â€?

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Redis.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Redis.html#impl-Builder-for-RedisBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Redis.html" class="struct" title="struct opendal::services::Redis">RedisBuilder</a>

Available on **crate feature `services-redis`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Redis.html#associatedtype.Config" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.RedisConfig.html" class="struct" title="struct opendal::services::RedisConfig">RedisConfig</a>

Associated configuration for this builder.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Redis.html#method.build" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#tymethod.build" class="fn">build</a>(self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<impl <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\>

Consume the accessor builder to build a service.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Redis.html#impl-Clone-for-RedisBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Redis.html" class="struct" title="struct opendal::services::Redis">RedisBuilder</a>

Available on **crate feature `services-redis`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Redis.html#method.clone" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Redis.html" class="struct" title="struct opendal::services::Redis">RedisBuilder</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Redis.html#method.clone_from" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Redis.html#impl-Debug-for-RedisBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Redis.html" class="struct" title="struct opendal::services::Redis">RedisBuilder</a>

Available on **crate feature `services-redis`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Redis.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Redis.html#impl-Default-for-RedisBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Redis.html" class="struct" title="struct opendal::services::Redis">RedisBuilder</a>

Available on **crate feature `services-redis`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Redis.html#method.default" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Redis.html" class="struct" title="struct opendal::services::Redis">RedisBuilder</a>

Returns the â€œdefault valueâ€? for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Redis.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Redis.html#blanket-implementations" class="anchor">Â§</a>
