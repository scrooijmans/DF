# Struct Postgresql Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/services/postgresql/backend.rs.html#35-37" class="src">Source</a>

``` rust
pub struct Postgresql { /* private fields */ }
```

Expand description

[PostgreSQL](https://www.postgresql.org/) services support.

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Postgresql.html#capabilities" class="doc-anchor">Â§</a>Capabilities

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

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Postgresql.html#configuration" class="doc-anchor">Â§</a>Configuration

- `root`: Set the working directory of `OpenDAL`
- `connection_string`: Set the connection string of postgres server
- `table`: Set the table of postgresql
- `key_field`: Set the key field of postgresql
- `value_field`: Set the value field of postgresql

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Postgresql.html#example" class="doc-anchor">Â§</a>Example

#### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Postgresql.html#via-builder" class="doc-anchor">Â§</a>Via Builder

``` rust
use anyhow::Result;
use opendal::services::Postgresql;
use opendal::Operator;

#[tokio::main]
async fn main() -> Result<()> {
    let mut builder = Postgresql::default()
        .root("/")
        .connection_string("postgresql://you_username:your_password@127.0.0.1:5432/your_database")
        .table("your_table")
        // key field type in the table should be compatible with Rust's &str like text
        .key_field("key")
        // value field type in the table should be compatible with Rust's Vec<u8> like bytea
        .value_field("value");

    let op = Operator::new(builder)?.finish();
    Ok(())
}
```

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Postgresql.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Postgresql.html#impl-PostgresqlBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Postgresql.html" class="struct" title="struct opendal::services::Postgresql">PostgresqlBuilder</a>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Postgresql.html#method.connection_string" class="fn">connection_string</a>(self, v: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-postgresql`** only.

Set the connection url string of the postgresql service.

The URL should be with a scheme of either `postgres://` or `postgresql://`.

- `postgresql://user@localhost`
- `postgresql://user:password@%2Fvar%2Flib%2Fpostgresql/mydb?connect_timeout=10`
- `postgresql://user@host1:1234,host2,host3:5678?target_session_attrs=read-write`
- `postgresql:///mydb?user=user&host=/var/lib/postgresql`

For more information, please visit <https://docs.rs/sqlx/latest/sqlx/postgres/struct.PgConnectOptions.html>.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Postgresql.html#method.root" class="fn">root</a>(self, root: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-postgresql`** only.

Set the working directory, all operations will be performed under it.

default: â€œ/â€?

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Postgresql.html#method.table" class="fn">table</a>(self, table: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-postgresql`** only.

Set the table name of the postgresql service to read/write.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Postgresql.html#method.key_field" class="fn">key_field</a>(self, key_field: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-postgresql`** only.

Set the key field name of the postgresql service to read/write.

Default to `key` if not specified.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Postgresql.html#method.value_field" class="fn">value_field</a>(self, value_field: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-postgresql`** only.

Set the value field name of the postgresql service to read/write.

Default to `value` if not specified.

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Postgresql.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Postgresql.html#impl-Builder-for-PostgresqlBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Postgresql.html" class="struct" title="struct opendal::services::Postgresql">PostgresqlBuilder</a>

Available on **crate feature `services-postgresql`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Postgresql.html#associatedtype.Config" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.PostgresqlConfig.html" class="struct" title="struct opendal::services::PostgresqlConfig">PostgresqlConfig</a>

Associated configuration for this builder.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Postgresql.html#method.build" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#tymethod.build" class="fn">build</a>(self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<impl <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\>

Consume the accessor builder to build a service.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Postgresql.html#impl-Debug-for-PostgresqlBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Postgresql.html" class="struct" title="struct opendal::services::Postgresql">PostgresqlBuilder</a>

Available on **crate feature `services-postgresql`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Postgresql.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Postgresql.html#impl-Default-for-PostgresqlBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Postgresql.html" class="struct" title="struct opendal::services::Postgresql">PostgresqlBuilder</a>

Available on **crate feature `services-postgresql`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Postgresql.html#method.default" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Postgresql.html" class="struct" title="struct opendal::services::Postgresql">PostgresqlBuilder</a>

Returns the â€œdefault valueâ€? for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Postgresql.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Postgresql.html#blanket-implementations" class="anchor">Â§</a>
