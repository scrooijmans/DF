# Struct Gridfs Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/services/gridfs/backend.rs.html#31-33" class="src">Source</a>

``` rust
pub struct Gridfs { /* private fields */ }
```

Expand description

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Gridfs.html#capabilities" class="doc-anchor">Â§</a>Capabilities

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

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Gridfs.html#configuration" class="doc-anchor">Â§</a>Configuration

- `root`: Set the working directory of `OpenDAL`
- `connection_string`: Set the connection string of mongodb server
- `database`: Set the database of mongodb
- `bucket`: Set the bucket of mongodb gridfs
- `chunk_size`: Set the chunk size of mongodb gridfs

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Gridfs.html#example" class="doc-anchor">Â§</a>Example

#### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Gridfs.html#via-builder" class="doc-anchor">Â§</a>Via Builder

``` rust
use anyhow::Result;
use opendal::services::Gridfs;
use opendal::Operator;

#[tokio::main]
async fn main() -> Result<()> {
    let mut builder = Gridfs::default()
        .root("/")
        .connection_string("mongodb://myUser:myPassword@localhost:27017/myAuthDB")
        .database("your_database")
        .bucket("your_bucket")
        // The chunk size in bytes used to break the user file into chunks.
        .chunk_size(255);

    let op = Operator::new(builder)?.finish();
    Ok(())
}
```

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Gridfs.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Gridfs.html#impl-GridfsBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Gridfs.html" class="struct" title="struct opendal::services::Gridfs">GridfsBuilder</a>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Gridfs.html#method.connection_string" class="fn">connection_string</a>(self, v: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-gridfs`** only.

Set the connection_string of the MongoDB service.

This connection string is used to connect to the MongoDB service. It typically follows the format:

###### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Gridfs.html#format" class="doc-anchor">Â§</a>Format

`mongodb://[username:password@]host1[:port1][,...hostN[:portN]][/[defaultauthdb][?options]]`

Examples:

- Connecting to a local MongoDB instance: `mongodb://localhost:27017`
- Using authentication: `mongodb://myUser:myPassword@localhost:27017/myAuthDB`
- Specifying authentication mechanism: `mongodb://myUser:myPassword@localhost:27017/myAuthDB?authMechanism=SCRAM-SHA-256`

###### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Gridfs.html#options" class="doc-anchor">Â§</a>Options

- `authMechanism`: Specifies the authentication method to use. Examples include `SCRAM-SHA-1`, `SCRAM-SHA-256`, and `MONGODB-AWS`.
- â€¦ (any other options you wish to highlight)

For more information, please refer to [MongoDB Connection String URI Format](https://docs.mongodb.com/manual/reference/connection-string/).

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Gridfs.html#method.root" class="fn">root</a>(self, root: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-gridfs`** only.

Set the working directory, all operations will be performed under it.

default: â€œ/â€?

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Gridfs.html#method.database" class="fn">database</a>(self, database: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-gridfs`** only.

Set the database name of the MongoDB GridFs service to read/write.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Gridfs.html#method.bucket" class="fn">bucket</a>(self, bucket: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-gridfs`** only.

Set the bucket name of the MongoDB GridFs service to read/write.

Default to `fs` if not specified.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Gridfs.html#method.chunk_size" class="fn">chunk_size</a>(self, chunk_size: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>) -\> Self

Available on **crate feature `services-gridfs`** only.

Set the chunk size of the MongoDB GridFs service used to break the user file into chunks.

Default to `255 KiB` if not specified.

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Gridfs.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Gridfs.html#impl-Builder-for-GridfsBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Gridfs.html" class="struct" title="struct opendal::services::Gridfs">GridfsBuilder</a>

Available on **crate feature `services-gridfs`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Gridfs.html#associatedtype.Config" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.GridfsConfig.html" class="struct" title="struct opendal::services::GridfsConfig">GridfsConfig</a>

Associated configuration for this builder.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Gridfs.html#method.build" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#tymethod.build" class="fn">build</a>(self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<impl <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\>

Consume the accessor builder to build a service.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Gridfs.html#impl-Default-for-GridfsBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Gridfs.html" class="struct" title="struct opendal::services::Gridfs">GridfsBuilder</a>

Available on **crate feature `services-gridfs`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Gridfs.html#method.default" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Gridfs.html" class="struct" title="struct opendal::services::Gridfs">GridfsBuilder</a>

Returns the â€œdefault valueâ€? for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Gridfs.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Gridfs.html#blanket-implementations" class="anchor">Â§</a>
