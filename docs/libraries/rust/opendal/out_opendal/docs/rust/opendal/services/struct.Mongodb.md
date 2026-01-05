# Struct Mongodb Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/services/mongodb/backend.rs.html#31-33" class="src">Source</a>

``` rust
pub struct Mongodb { /* private fields */ }
```

Expand description

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Mongodb.html#capabilities" class="doc-anchor">Â§</a>Capabilities

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

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Mongodb.html#configuration" class="doc-anchor">Â§</a>Configuration

- `root`: Set the working directory of `OpenDAL`
- `connection_string`: Set the connection string of mongodb server
- `database`: Set the database of mongodb
- `collection`: Set the collection of mongodb
- `key_field`: Set the key field of mongodb
- `value_field`: Set the value field of mongodb

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Mongodb.html#example" class="doc-anchor">Â§</a>Example

#### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Mongodb.html#via-builder" class="doc-anchor">Â§</a>Via Builder

``` rust
use anyhow::Result;
use opendal::services::Mongodb;
use opendal::Operator;

#[tokio::main]
async fn main() -> Result<()> {
    let mut builder = Mongodb::default()
        .root("/")
        .connection_string("mongodb://myUser:myPassword@localhost:27017/myAuthDB")
        .database("your_database")
        .collection("your_collection")
        // key field type in the table should be compatible with Rust's &str like text
        .key_field("key")
        // value field type in the table should be compatible with Rust's Vec<u8> like bytea
        .value_field("value");

    let op = Operator::new(builder)?.finish();
    Ok(())
}
```

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Mongodb.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Mongodb.html#impl-MongodbBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Mongodb.html" class="struct" title="struct opendal::services::Mongodb">MongodbBuilder</a>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Mongodb.html#method.connection_string" class="fn">connection_string</a>(self, v: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-mongodb`** only.

Set the connection_string of the MongoDB service.

This connection string is used to connect to the MongoDB service. It typically follows the format:

###### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Mongodb.html#format" class="doc-anchor">Â§</a>Format

`mongodb://[username:password@]host1[:port1][,...hostN[:portN]][/[defaultauthdb][?options]]`

Examples:

- Connecting to a local MongoDB instance: `mongodb://localhost:27017`
- Using authentication: `mongodb://myUser:myPassword@localhost:27017/myAuthDB`
- Specifying authentication mechanism: `mongodb://myUser:myPassword@localhost:27017/myAuthDB?authMechanism=SCRAM-SHA-256`

###### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Mongodb.html#options" class="doc-anchor">Â§</a>Options

- `authMechanism`: Specifies the authentication method to use. Examples include `SCRAM-SHA-1`, `SCRAM-SHA-256`, and `MONGODB-AWS`.
- â€¦ (any other options you wish to highlight)

For more information, please refer to [MongoDB Connection String URI Format](https://docs.mongodb.com/manual/reference/connection-string/).

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Mongodb.html#method.root" class="fn">root</a>(self, root: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-mongodb`** only.

Set the working directory, all operations will be performed under it.

default: â€œ/â€?

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Mongodb.html#method.database" class="fn">database</a>(self, database: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-mongodb`** only.

Set the database name of the MongoDB service to read/write.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Mongodb.html#method.collection" class="fn">collection</a>(self, collection: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-mongodb`** only.

Set the collection name of the MongoDB service to read/write.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Mongodb.html#method.key_field" class="fn">key_field</a>(self, key_field: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-mongodb`** only.

Set the key field name of the MongoDB service to read/write.

Default to `key` if not specified.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Mongodb.html#method.value_field" class="fn">value_field</a>(self, value_field: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-mongodb`** only.

Set the value field name of the MongoDB service to read/write.

Default to `value` if not specified.

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Mongodb.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Mongodb.html#impl-Builder-for-MongodbBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Mongodb.html" class="struct" title="struct opendal::services::Mongodb">MongodbBuilder</a>

Available on **crate feature `services-mongodb`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Mongodb.html#associatedtype.Config" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.MongodbConfig.html" class="struct" title="struct opendal::services::MongodbConfig">MongodbConfig</a>

Associated configuration for this builder.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Mongodb.html#method.build" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#tymethod.build" class="fn">build</a>(self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<impl <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\>

Consume the accessor builder to build a service.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Mongodb.html#impl-Default-for-MongodbBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Mongodb.html" class="struct" title="struct opendal::services::Mongodb">MongodbBuilder</a>

Available on **crate feature `services-mongodb`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Mongodb.html#method.default" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Mongodb.html" class="struct" title="struct opendal::services::Mongodb">MongodbBuilder</a>

Returns the â€œdefault valueâ€? for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Mongodb.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Mongodb.html#blanket-implementations" class="anchor">Â§</a>
