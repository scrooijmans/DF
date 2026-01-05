# Struct Sqlite Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/services/sqlite/backend.rs.html#40-42" class="src">Source</a>

``` rust
pub struct Sqlite { /* private fields */ }
```

Expand description

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Sqlite.html#capabilities" class="doc-anchor">Â§</a>Capabilities

This service can be used to:

- ☒ stat
- ☒ read
- ☒ write
- ☒ create_dir
- ☒ delete
- ☐ copy
- ☐ rename
- ☒ list
- ☐ blocking

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Sqlite.html#configuration" class="doc-anchor">Â§</a>Configuration

- `root`: Set the working directory of `OpenDAL`
- `connection_string`: Set the connection string of sqlite database
- `table`: Set the table of sqlite
- `key_field`: Set the key field of sqlite
- `value_field`: Set the value field of sqlite

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Sqlite.html#example" class="doc-anchor">Â§</a>Example

#### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Sqlite.html#via-builder" class="doc-anchor">Â§</a>Via Builder

``` rust
use anyhow::Result;
use opendal::services::Sqlite;
use opendal::Operator;

#[tokio::main]
async fn main() -> Result<()> {
    let mut builder = Sqlite::default()
        .root("/")
        .connection_string("file//abc.db")
        .table("your_table")
        // key field type in the table should be compatible with Rust's &str like text
        .key_field("key")
        // value field type in the table should be compatible with Rust's Vec<u8> like bytea
        .value_field("value");

    let op = Operator::new(builder)?.finish();
    Ok(())
}
```

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Sqlite.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Sqlite.html#impl-SqliteBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Sqlite.html" class="struct" title="struct opendal::services::Sqlite">SqliteBuilder</a>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Sqlite.html#method.connection_string" class="fn">connection_string</a>(self, v: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-sqlite`** only.

Set the connection_string of the sqlite service.

This connection string is used to connect to the sqlite service. There are url based formats:

###### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Sqlite.html#url" class="doc-anchor">Â§</a>Url

This format resembles the url format of the sqlite client:

- `sqlite::memory:`
- `sqlite:data.db`
- `sqlite://data.db`

For more information, please visit <https://docs.rs/sqlx/latest/sqlx/sqlite/struct.SqliteConnectOptions.html>.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Sqlite.html#method.root" class="fn">root</a>(self, root: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-sqlite`** only.

set the working directory, all operations will be performed under it.

default: â€œ/â€?

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Sqlite.html#method.table" class="fn">table</a>(self, table: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-sqlite`** only.

Set the table name of the sqlite service to read/write.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Sqlite.html#method.key_field" class="fn">key_field</a>(self, key_field: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-sqlite`** only.

Set the key field name of the sqlite service to read/write.

Default to `key` if not specified.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Sqlite.html#method.value_field" class="fn">value_field</a>(self, value_field: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-sqlite`** only.

Set the value field name of the sqlite service to read/write.

Default to `value` if not specified.

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Sqlite.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Sqlite.html#impl-Builder-for-SqliteBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Sqlite.html" class="struct" title="struct opendal::services::Sqlite">SqliteBuilder</a>

Available on **crate feature `services-sqlite`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Sqlite.html#associatedtype.Config" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.SqliteConfig.html" class="struct" title="struct opendal::services::SqliteConfig">SqliteConfig</a>

Associated configuration for this builder.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Sqlite.html#method.build" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#tymethod.build" class="fn">build</a>(self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<impl <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\>

Consume the accessor builder to build a service.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Sqlite.html#impl-Debug-for-SqliteBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Sqlite.html" class="struct" title="struct opendal::services::Sqlite">SqliteBuilder</a>

Available on **crate feature `services-sqlite`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Sqlite.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Sqlite.html#impl-Default-for-SqliteBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Sqlite.html" class="struct" title="struct opendal::services::Sqlite">SqliteBuilder</a>

Available on **crate feature `services-sqlite`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Sqlite.html#method.default" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Sqlite.html" class="struct" title="struct opendal::services::Sqlite">SqliteBuilder</a>

Returns the â€œdefault valueâ€? for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Sqlite.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Sqlite.html#blanket-implementations" class="anchor">Â§</a>
