# Struct Redb Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/services/redb/backend.rs.html#34-38" class="src">Source</a>

``` rust
pub struct Redb { /* private fields */ }
```

Expand description

Redb service support.

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Redb.html#capabilities" class="doc-anchor">Â§</a>Capabilities

This service can be used to:

- ☒ stat
- ☒ read
- ☒ write
- ☒ create_dir
- ☒ delete
- ☒ copy
- ☒ rename
- ☐ ~~list~~
- ☐ ~~presign~~
- ☒ blocking

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Redb.html#configuration" class="doc-anchor">Â§</a>Configuration

- `datadir`: Set the path to the redb data directory.
- `table`: Set the table name for Redb.

You can refer to [`RedbBuilder`](https://opendal.apache.org/docs/rust/opendal/services/struct.Redb.html "struct opendal::services::Redb")â€™s docs for more information.

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Redb.html#example" class="doc-anchor">Â§</a>Example

#### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Redb.html#via-builder" class="doc-anchor">Â§</a>Via Builder

``` rust
use anyhow::Result;
use opendal::services::Redb;
use opendal::Operator;

#[tokio::main]
async fn main() -> Result<()> {
    let mut builder = Redb::default()
        .datadir("/tmp/opendal/redb")
        .table("opendal-redb");

    let op: Operator = Operator::new(builder)?.finish();
    Ok(())
}
```

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Redb.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Redb.html#impl-RedbBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Redb.html" class="struct" title="struct opendal::services::Redb">RedbBuilder</a>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Redb.html#method.database" class="fn">database</a>(self, db: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<Database\>) -\> Self

Available on **crate feature `services-redb`** only.

Set the database for Redb.

This method should be called when you want to use multiple tables of one database because Redb doesnâ€™t allow opening a database that have been opened.

`datadir` and `database` should not be set simultaneously. If both are set, `database` will take precedence.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Redb.html#method.datadir" class="fn">datadir</a>(self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-redb`** only.

Set the path to the redb data directory. Will create if not exists.

Opening redb database via `datadir` takes away the ability to access multiple redb tables. If you need to access multiple redb tables, the correct solution is to create an `Arc<redb::database>` beforehand and then share it via [`database`](https://opendal.apache.org/docs/rust/opendal/services/struct.Redb.html#method.database "method opendal::services::Redb::database") with multiple builders where every builder will open one redb table.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Redb.html#method.table" class="fn">table</a>(self, table: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-redb`** only.

Set the table name for Redb. Will create if not exists.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Redb.html#method.root" class="fn">root</a>(self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-redb`** only.

Set the root for Redb.

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Redb.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Redb.html#impl-Builder-for-RedbBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Redb.html" class="struct" title="struct opendal::services::Redb">RedbBuilder</a>

Available on **crate feature `services-redb`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Redb.html#associatedtype.Config" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.RedbConfig.html" class="struct" title="struct opendal::services::RedbConfig">RedbConfig</a>

Associated configuration for this builder.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Redb.html#method.build" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#tymethod.build" class="fn">build</a>(self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<impl <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\>

Consume the accessor builder to build a service.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Redb.html#impl-Debug-for-RedbBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Redb.html" class="struct" title="struct opendal::services::Redb">RedbBuilder</a>

Available on **crate feature `services-redb`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Redb.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Redb.html#impl-Default-for-RedbBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Redb.html" class="struct" title="struct opendal::services::Redb">RedbBuilder</a>

Available on **crate feature `services-redb`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Redb.html#method.default" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Redb.html" class="struct" title="struct opendal::services::Redb">RedbBuilder</a>

Returns the â€œdefault valueâ€? for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Redb.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Redb.html#blanket-implementations" class="anchor">Â§</a>
