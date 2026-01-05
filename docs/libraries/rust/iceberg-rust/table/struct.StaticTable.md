# Struct StaticTable Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/table.rs.html#271" class="src">Source</a>

``` rust
pub struct StaticTable(/* private fields */);
```

Expand description

`StaticTable` is a read-only table struct that can be created from a metadata file or from `TableMetaData` without a catalog. It can only be used to read metadata and for table scan.

## <a href="https://docs.rs/iceberg/0.7.0/iceberg/table/struct.StaticTable.html#examples" class="doc-anchor">§</a>Examples

``` rust
let metadata_file_location = "s3://bucket_name/path/to/metadata.json";
let file_io = FileIO::from_path(&metadata_file_location)
    .unwrap()
    .build()
    .unwrap();
let static_identifier = TableIdent::from_strs(["static_ns", "static_table"]).unwrap();
let static_table =
    StaticTable::from_metadata_file(&metadata_file_location, static_identifier, file_io)
        .await
        .unwrap();
let snapshot_id = static_table
    .metadata()
    .current_snapshot()
    .unwrap()
    .snapshot_id();
```

## Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/table/struct.StaticTable.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/table/struct.StaticTable.html#impl-StaticTable" class="anchor">§</a>

### impl <a href="https://docs.rs/iceberg/0.7.0/iceberg/table/struct.StaticTable.html" class="struct" title="struct iceberg::table::StaticTable">StaticTable</a>

#### pub async fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/table/struct.StaticTable.html#method.from_metadata" class="fn">from_metadata</a>( metadata: <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadata.html" class="struct" title="struct iceberg::spec::TableMetadata">TableMetadata</a>, table_ident: <a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.TableIdent.html" class="struct" title="struct iceberg::TableIdent">TableIdent</a>, file_io: <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.FileIO.html" class="struct" title="struct iceberg::io::FileIO">FileIO</a>, ) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<Self\>

Creates a static table from a given `TableMetadata` and `FileIO`

#### pub async fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/table/struct.StaticTable.html#method.from_metadata_file" class="fn">from_metadata_file</a>( metadata_location: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, table_ident: <a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.TableIdent.html" class="struct" title="struct iceberg::TableIdent">TableIdent</a>, file_io: <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.FileIO.html" class="struct" title="struct iceberg::io::FileIO">FileIO</a>, ) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<Self\>

Creates a static table directly from metadata file and `FileIO`

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/table/struct.StaticTable.html#method.scan" class="fn">scan</a>(&self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/scan/struct.TableScanBuilder.html" class="struct" title="struct iceberg::scan::TableScanBuilder">TableScanBuilder</a>\<'\_\>

Create a TableScanBuilder for the static table.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/table/struct.StaticTable.html#method.metadata" class="fn">metadata</a>(&self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/type.TableMetadataRef.html" class="type" title="type iceberg::spec::TableMetadataRef">TableMetadataRef</a>

Get TableMetadataRef for the static table

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/table/struct.StaticTable.html#method.into_table" class="fn">into_table</a>(self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/table/struct.Table.html" class="struct" title="struct iceberg::table::Table">Table</a>

Consumes the `StaticTable` and return it as a `Table` Please use this method carefully as the Table it returns remains detached from a catalog and can’t be used to perform modifications on the table.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/table/struct.StaticTable.html#method.reader_builder" class="fn">reader_builder</a>(&self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/struct.ArrowReaderBuilder.html" class="struct" title="struct iceberg::arrow::ArrowReaderBuilder">ArrowReaderBuilder</a>

Create a reader for the table.

## Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/table/struct.StaticTable.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/table/struct.StaticTable.html#impl-Clone-for-StaticTable" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/table/struct.StaticTable.html" class="struct" title="struct iceberg::table::StaticTable">StaticTable</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/table/struct.StaticTable.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/table/struct.StaticTable.html" class="struct" title="struct iceberg::table::StaticTable">StaticTable</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/table/struct.StaticTable.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/table/struct.StaticTable.html#impl-Debug-for-StaticTable" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/table/struct.StaticTable.html" class="struct" title="struct iceberg::table::StaticTable">StaticTable</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/table/struct.StaticTable.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/table/struct.StaticTable.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/table/struct.StaticTable.html#blanket-implementations" class="anchor">§</a>
