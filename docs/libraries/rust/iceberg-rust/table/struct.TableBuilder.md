# Struct TableBuilder Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/table.rs.html#31-39" class="src">Source</a>

``` rust
pub struct TableBuilder { /* private fields */ }
```

Expand description

Builder to create table scan.

## Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/table/struct.TableBuilder.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/table/struct.TableBuilder.html#impl-TableBuilder" class="anchor">§</a>

### impl <a href="https://docs.rs/iceberg/0.7.0/iceberg/table/struct.TableBuilder.html" class="struct" title="struct iceberg::table::TableBuilder">TableBuilder</a>

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/table/struct.TableBuilder.html#method.file_io" class="fn">file_io</a>(self, file_io: <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.FileIO.html" class="struct" title="struct iceberg::io::FileIO">FileIO</a>) -\> Self

required - sets the necessary FileIO to use for the table

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/table/struct.TableBuilder.html#method.metadata_location" class="fn">metadata_location</a>\<T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>\>(self, metadata_location: T) -\> Self

optional - sets the tables metadata location

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/table/struct.TableBuilder.html#method.metadata" class="fn">metadata</a>\<T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/type.TableMetadataRef.html" class="type" title="type iceberg::spec::TableMetadataRef">TableMetadataRef</a>\>\>(self, metadata: T) -\> Self

required - passes in the TableMetadata to use for the Table

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/table/struct.TableBuilder.html#method.identifier" class="fn">identifier</a>(self, identifier: <a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.TableIdent.html" class="struct" title="struct iceberg::TableIdent">TableIdent</a>) -\> Self

required - passes in the TableIdent to use for the Table

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/table/struct.TableBuilder.html#method.readonly" class="fn">readonly</a>(self, readonly: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> Self

specifies if the Table is readonly or not (default not)

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/table/struct.TableBuilder.html#method.disable_cache" class="fn">disable_cache</a>(self) -\> Self

specifies if the Table’s metadata cache will be disabled, so that reads of Manifests and ManifestLists will never get cached.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/table/struct.TableBuilder.html#method.cache_size_bytes" class="fn">cache_size_bytes</a>(self, cache_size_bytes: <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>) -\> Self

optionally set a non-default metadata cache size

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/table/struct.TableBuilder.html#method.build" class="fn">build</a>(self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/table/struct.Table.html" class="struct" title="struct iceberg::table::Table">Table</a>\>

build the Table

## Auto Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/table/struct.TableBuilder.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/table/struct.TableBuilder.html#blanket-implementations" class="anchor">§</a>
