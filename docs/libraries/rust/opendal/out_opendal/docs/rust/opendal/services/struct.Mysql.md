# Struct Mysql Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/services/mysql/backend.rs.html#34-36" class="src">Source</a>

``` rust
pub struct Mysql { /* private fields */ }
```

Expand description

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Mysql.html#capabilities" class="doc-anchor">Â§</a>Capabilities

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

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Mysql.html#configuration" class="doc-anchor">Â§</a>Configuration

- `root`: Set the working directory of `OpenDAL`
- `connection_string`: Set the connection string of mysql server
- `table`: Set the table of mysql
- `key_field`: Set the key field of mysql
- `value_field`: Set the value field of mysql

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Mysql.html#example" class="doc-anchor">Â§</a>Example

#### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Mysql.html#via-builder" class="doc-anchor">Â§</a>Via Builder

``` rust
use anyhow::Result;
use opendal::services::Mysql;
use opendal::Operator;

#[tokio::main]
async fn main() -> Result<()> {
    let mut builder = Mysql::default()
        .root("/")
        .connection_string("mysql://you_username:your_password@127.0.0.1:5432/your_database")
        .table("your_table")
        // key field type in the table should be compatible with Rust's &str like text
        .key_field("key")
        // value field type in the table should be compatible with Rust's Vec<u8> like bytea
        .value_field("value");

    let op = Operator::new(builder)?.finish();
    Ok(())
}
```

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Mysql.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Mysql.html#impl-MysqlBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Mysql.html" class="struct" title="struct opendal::services::Mysql">MysqlBuilder</a>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Mysql.html#method.connection_string" class="fn">connection_string</a>(self, v: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-mysql`** only.

Set the connection_string of the mysql service.

This connection string is used to connect to the mysql service. There are url based formats:

###### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Mysql.html#url" class="doc-anchor">Â§</a>Url

This format resembles the url format of the mysql client. The format is: `[scheme://][user[:[password]]@]host[:port][/schema][?attribute1=value1&attribute2=value2...`

- `mysql://user@localhost`
- `mysql://user:password@localhost`
- `mysql://user:password@localhost:3306`
- `mysql://user:password@localhost:3306/db`

For more information, please refer to <https://docs.rs/sqlx/latest/sqlx/mysql/struct.MySqlConnectOptions.html>.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Mysql.html#method.root" class="fn">root</a>(self, root: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-mysql`** only.

set the working directory, all operations will be performed under it.

default: â€œ/â€?

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Mysql.html#method.table" class="fn">table</a>(self, table: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-mysql`** only.

Set the table name of the mysql service to read/write.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Mysql.html#method.key_field" class="fn">key_field</a>(self, key_field: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-mysql`** only.

Set the key field name of the mysql service to read/write.

Default to `key` if not specified.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Mysql.html#method.value_field" class="fn">value_field</a>(self, value_field: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-mysql`** only.

Set the value field name of the mysql service to read/write.

Default to `value` if not specified.

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Mysql.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Mysql.html#impl-Builder-for-MysqlBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Mysql.html" class="struct" title="struct opendal::services::Mysql">MysqlBuilder</a>

Available on **crate feature `services-mysql`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Mysql.html#associatedtype.Config" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.MysqlConfig.html" class="struct" title="struct opendal::services::MysqlConfig">MysqlConfig</a>

Associated configuration for this builder.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Mysql.html#method.build" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#tymethod.build" class="fn">build</a>(self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<impl <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\>

Consume the accessor builder to build a service.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Mysql.html#impl-Debug-for-MysqlBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Mysql.html" class="struct" title="struct opendal::services::Mysql">MysqlBuilder</a>

Available on **crate feature `services-mysql`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Mysql.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Mysql.html#impl-Default-for-MysqlBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Mysql.html" class="struct" title="struct opendal::services::Mysql">MysqlBuilder</a>

Available on **crate feature `services-mysql`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Mysql.html#method.default" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Mysql.html" class="struct" title="struct opendal::services::Mysql">MysqlBuilder</a>

Returns the â€œdefault valueâ€? for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Mysql.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Mysql.html#blanket-implementations" class="anchor">Â§</a>
