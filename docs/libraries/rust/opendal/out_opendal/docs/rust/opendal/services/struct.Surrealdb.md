# Struct Surrealdb Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/services/surrealdb/backend.rs.html#33-35" class="src">Source</a>

``` rust
pub struct Surrealdb { /* private fields */ }
```

Expand description

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Surrealdb.html#capabilities" class="doc-anchor">Â§</a>Capabilities

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

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Surrealdb.html#configuration" class="doc-anchor">Â§</a>Configuration

- `root`: Set the working directory of `OpenDAL`
- `connection_string`: Set the connection string of surrealdb server
- `username`: set the username of surrealdb
- `password`: set the password of surrealdb
- `namespace`: set the namespace of surrealdb
- `database`: set the database of surrealdb
- `table`: Set the table of surrealdb
- `key_field`: Set the key field of surrealdb
- `value_field`: Set the value field of surrealdb
- 

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Surrealdb.html#example" class="doc-anchor">Â§</a>Example

#### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Surrealdb.html#via-builder" class="doc-anchor">Â§</a>Via Builder

``` rust
use anyhow::Result;
use opendal::services::Surrealdb;
use opendal::Operator;

#[tokio::main]
async fn main() -> Result<()> {
    let mut builder = Surrealdb::default()
        .root("/")
        .connection_string("ws://127.0.0.1:8000")
        .username("username")
        .password("password")
        .namespace("namespace")
        .database("database")
        .table("table")
        .key_field("key")
        .value_field("value");

    let op = Operator::new(builder)?.finish();
    Ok(())
}
```

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Surrealdb.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Surrealdb.html#impl-SurrealdbBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Surrealdb.html" class="struct" title="struct opendal::services::Surrealdb">SurrealdbBuilder</a>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Surrealdb.html#method.connection_string" class="fn">connection_string</a>(self, connection_string: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-surrealdb`** only.

Set the connection_string of the surrealdb service.

This connection string is used to connect to the surrealdb service. There are url based formats:

###### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Surrealdb.html#url" class="doc-anchor">Â§</a>Url

- `ws://ip:port`
- `wss://ip:port`
- `http://ip:port`
- `https://ip:port`

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Surrealdb.html#method.root" class="fn">root</a>(self, root: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-surrealdb`** only.

set the working directory, all operations will be performed under it.

default: â€œ/â€?

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Surrealdb.html#method.table" class="fn">table</a>(self, table: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-surrealdb`** only.

Set the table name of the surrealdb service for read/write.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Surrealdb.html#method.username" class="fn">username</a>(self, username: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-surrealdb`** only.

Set the username of the surrealdb service for signin.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Surrealdb.html#method.password" class="fn">password</a>(self, password: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-surrealdb`** only.

Set the password of the surrealdb service for signin.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Surrealdb.html#method.namespace" class="fn">namespace</a>(self, namespace: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-surrealdb`** only.

Set the namespace of the surrealdb service for read/write.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Surrealdb.html#method.database" class="fn">database</a>(self, database: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-surrealdb`** only.

Set the database of the surrealdb service for read/write.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Surrealdb.html#method.key_field" class="fn">key_field</a>(self, key_field: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-surrealdb`** only.

Set the key field name of the surrealdb service for read/write.

Default to `key` if not specified.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Surrealdb.html#method.value_field" class="fn">value_field</a>(self, value_field: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-surrealdb`** only.

Set the value field name of the surrealdb service for read/write.

Default to `value` if not specified.

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Surrealdb.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Surrealdb.html#impl-Builder-for-SurrealdbBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Surrealdb.html" class="struct" title="struct opendal::services::Surrealdb">SurrealdbBuilder</a>

Available on **crate feature `services-surrealdb`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Surrealdb.html#associatedtype.Config" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.SurrealdbConfig.html" class="struct" title="struct opendal::services::SurrealdbConfig">SurrealdbConfig</a>

Associated configuration for this builder.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Surrealdb.html#method.build" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#tymethod.build" class="fn">build</a>(self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<impl <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\>

Consume the accessor builder to build a service.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Surrealdb.html#impl-Debug-for-SurrealdbBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Surrealdb.html" class="struct" title="struct opendal::services::Surrealdb">SurrealdbBuilder</a>

Available on **crate feature `services-surrealdb`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Surrealdb.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Surrealdb.html#impl-Default-for-SurrealdbBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Surrealdb.html" class="struct" title="struct opendal::services::Surrealdb">SurrealdbBuilder</a>

Available on **crate feature `services-surrealdb`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Surrealdb.html#method.default" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Surrealdb.html" class="struct" title="struct opendal::services::Surrealdb">SurrealdbBuilder</a>

Returns the â€œdefault valueâ€? for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Surrealdb.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Surrealdb.html#blanket-implementations" class="anchor">Â§</a>
