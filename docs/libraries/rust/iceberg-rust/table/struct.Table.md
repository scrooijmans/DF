# Struct Table Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/table.rs.html#155-162" class="src">Source</a>

``` rust
pub struct Table { /* private fields */ }
```

Expand description

Table represents a table in the catalog.

## Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/table/struct.Table.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/table/struct.Table.html#impl-Table" class="anchor">§</a>

### impl <a href="https://docs.rs/iceberg/0.7.0/iceberg/table/struct.Table.html" class="struct" title="struct iceberg::table::Table">Table</a>

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/table/struct.Table.html#method.builder" class="fn">builder</a>() -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/table/struct.TableBuilder.html" class="struct" title="struct iceberg::table::TableBuilder">TableBuilder</a>

Returns a TableBuilder to build a table

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/table/struct.Table.html#method.identifier" class="fn">identifier</a>(&self) -\> &<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.TableIdent.html" class="struct" title="struct iceberg::TableIdent">TableIdent</a>

Returns table identifier.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/table/struct.Table.html#method.metadata" class="fn">metadata</a>(&self) -\> &<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadata.html" class="struct" title="struct iceberg::spec::TableMetadata">TableMetadata</a>

Returns current metadata.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/table/struct.Table.html#method.metadata_ref" class="fn">metadata_ref</a>(&self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/type.TableMetadataRef.html" class="type" title="type iceberg::spec::TableMetadataRef">TableMetadataRef</a>

Returns current metadata ref.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/table/struct.Table.html#method.metadata_location" class="fn">metadata_location</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>

Returns current metadata location.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/table/struct.Table.html#method.metadata_location_result" class="fn">metadata_location_result</a>(&self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>

Returns current metadata location in a result.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/table/struct.Table.html#method.file_io" class="fn">file_io</a>(&self) -\> &<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.FileIO.html" class="struct" title="struct iceberg::io::FileIO">FileIO</a>

Returns file io used in this table.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/table/struct.Table.html#method.scan" class="fn">scan</a>(&self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/scan/struct.TableScanBuilder.html" class="struct" title="struct iceberg::scan::TableScanBuilder">TableScanBuilder</a>\<'\_\>

Creates a table scan.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/table/struct.Table.html#method.inspect" class="fn">inspect</a>(&self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/inspect/struct.MetadataTable.html" class="struct" title="struct iceberg::inspect::MetadataTable">MetadataTable</a>\<'\_\>

Creates a metadata table which provides table-like APIs for inspecting metadata. See [`MetadataTable`](https://docs.rs/iceberg/0.7.0/iceberg/inspect/struct.MetadataTable.html "struct iceberg::inspect::MetadataTable") for more details.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/table/struct.Table.html#method.readonly" class="fn">readonly</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns the flag indicating whether the `Table` is readonly or not

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/table/struct.Table.html#method.reader_builder" class="fn">reader_builder</a>(&self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/struct.ArrowReaderBuilder.html" class="struct" title="struct iceberg::arrow::ArrowReaderBuilder">ArrowReaderBuilder</a>

Create a reader for the table.

## Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/table/struct.Table.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/table/struct.Table.html#impl-Clone-for-Table" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/table/struct.Table.html" class="struct" title="struct iceberg::table::Table">Table</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/table/struct.Table.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/table/struct.Table.html" class="struct" title="struct iceberg::table::Table">Table</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/table/struct.Table.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/table/struct.Table.html#impl-Debug-for-Table" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/table/struct.Table.html" class="struct" title="struct iceberg::table::Table">Table</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/table/struct.Table.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/table/struct.Table.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/table/struct.Table.html#blanket-implementations" class="anchor">§</a>
