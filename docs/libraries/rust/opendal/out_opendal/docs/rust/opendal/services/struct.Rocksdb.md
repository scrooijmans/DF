# Struct Rocksdb Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/services/rocksdb/backend.rs.html#33-35" class="src">Source</a>

``` rust
pub struct Rocksdb { /* private fields */ }
```

Expand description

RocksDB service support.

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Rocksdb.html#capabilities" class="doc-anchor">Â§</a>Capabilities

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

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Rocksdb.html#note" class="doc-anchor">Â§</a>Note

OpenDAL will build rocksdb from source by default.

To link with existing rocksdb lib, please set one of the following:

- `ROCKSDB_LIB_DIR` to the dir that contains `librocksdb.so`
- `ROCKSDB_STATIC` to the dir that contains `librocksdb.a`

If the version of RocksDB is below 6.0, you may encounter compatibility issues. It is advisable to follow the steps provided in the [`INSTALL`](https://github.com/facebook/rocksdb/blob/main/INSTALL.md) file to build rocksdb, rather than relying on system libraries that may be outdated and incompatible.

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Rocksdb.html#configuration" class="doc-anchor">Â§</a>Configuration

- `root`: Set the working directory of `OpenDAL`
- `datadir`: Set the path to the rocksdb data directory

You can refer to [`RocksdbBuilder`](https://opendal.apache.org/docs/rust/opendal/services/struct.Rocksdb.html "struct opendal::services::Rocksdb")â€™s docs for more information.

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Rocksdb.html#example" class="doc-anchor">Â§</a>Example

#### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Rocksdb.html#via-builder" class="doc-anchor">Â§</a>Via Builder

``` rust
use anyhow::Result;
use opendal::services::Rocksdb;
use opendal::Operator;

#[tokio::main]
async fn main() -> Result<()> {
    let mut builder = Rocksdb::default()
        .datadir("/tmp/opendal/rocksdb");

    let op: Operator = Operator::new(builder)?.finish();
    Ok(())
}
```

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Rocksdb.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Rocksdb.html#impl-RocksdbBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Rocksdb.html" class="struct" title="struct opendal::services::Rocksdb">RocksdbBuilder</a>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Rocksdb.html#method.datadir" class="fn">datadir</a>(self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-rocksdb`** only.

Set the path to the rocksdb data directory. Will create if not exists.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Rocksdb.html#method.root" class="fn">root</a>(self, root: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-rocksdb`** only.

set the working directory, all operations will be performed under it.

default: â€œ/â€?

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Rocksdb.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Rocksdb.html#impl-Builder-for-RocksdbBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Rocksdb.html" class="struct" title="struct opendal::services::Rocksdb">RocksdbBuilder</a>

Available on **crate feature `services-rocksdb`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Rocksdb.html#associatedtype.Config" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.RocksdbConfig.html" class="struct" title="struct opendal::services::RocksdbConfig">RocksdbConfig</a>

Associated configuration for this builder.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Rocksdb.html#method.build" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#tymethod.build" class="fn">build</a>(self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<impl <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\>

Consume the accessor builder to build a service.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Rocksdb.html#impl-Clone-for-RocksdbBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Rocksdb.html" class="struct" title="struct opendal::services::Rocksdb">RocksdbBuilder</a>

Available on **crate feature `services-rocksdb`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Rocksdb.html#method.clone" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Rocksdb.html" class="struct" title="struct opendal::services::Rocksdb">RocksdbBuilder</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Rocksdb.html#method.clone_from" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Rocksdb.html#impl-Default-for-RocksdbBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Rocksdb.html" class="struct" title="struct opendal::services::Rocksdb">RocksdbBuilder</a>

Available on **crate feature `services-rocksdb`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Rocksdb.html#method.default" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Rocksdb.html" class="struct" title="struct opendal::services::Rocksdb">RocksdbBuilder</a>

Returns the â€œdefault valueâ€? for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Rocksdb.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Rocksdb.html#blanket-implementations" class="anchor">Â§</a>
