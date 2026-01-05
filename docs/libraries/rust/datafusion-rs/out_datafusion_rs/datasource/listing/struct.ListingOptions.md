# Struct ListingOptions Copy item path

<a href="https://docs.rs/datafusion/50.2.0/src/datafusion/datasource/listing/table.rs.html#449-481" class="src">Source</a>

``` rust
pub struct ListingOptions {
    pub file_extension: String,
    pub format: Arc<dyn FileFormat>,
    pub table_partition_cols: Vec<(String, DataType)>,
    pub collect_stat: bool,
    pub target_partitions: usize,
    pub file_sort_order: Vec<Vec<SortExpr>>,
}
```

Expand description

Options for creating a [`ListingTable`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTable.html "struct datafusion::datasource::listing::ListingTable")

## Fields<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingOptions.html#fields" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingOptions.html#structfield.file_extension" class="anchor field">§</a>`file_extension: `<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>

A suffix on which files should be filtered (leave empty to keep all files on the path)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingOptions.html#structfield.format" class="anchor field">§</a>`format: `<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc"><code>Arc</code></a>`<dyn `<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/trait.FileFormat.html" class="trait" title="trait datafusion::datasource::file_format::FileFormat"><code>FileFormat</code></a>`>`

The file format

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingOptions.html#structfield.table_partition_cols" class="anchor field">§</a>`table_partition_cols: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<(`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`, `<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType"><code>DataType</code></a>`)>`

The expected partition column names in the folder structure. See [Self::with_table_partition_cols](https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingOptions.html#method.with_table_partition_cols "method datafusion::datasource::listing::ListingOptions::with_table_partition_cols") for details

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingOptions.html#structfield.collect_stat" class="anchor field">§</a>`collect_stat: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Set true to try to guess statistics from the files. This can add a lot of overhead as it will usually require files to be opened and at least partially parsed.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingOptions.html#structfield.target_partitions" class="anchor field">§</a>`target_partitions: `<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a>

Group files to avoid that the number of partitions exceeds this limit

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingOptions.html#structfield.file_sort_order" class="anchor field">§</a>`file_sort_order: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.SortExpr.html" class="struct" title="struct datafusion::logical_expr::SortExpr"><code>SortExpr</code></a>`>>`

Optional pre-known sort order(s). Must be `SortExpr`s.

DataFusion may take advantage of this ordering to omit sorts or use more efficient algorithms. Currently sortedness must be provided if it is known by some external mechanism, but may in the future be automatically determined, for example using parquet metadata.

See <https://github.com/apache/datafusion/issues/4177>

NOTE: This attribute stores all equivalent orderings (the outer `Vec`) where each ordering consists of an individual lexicographic ordering (encapsulated by a `Vec<Expr>`). If there aren’t multiple equivalent orderings, the outer `Vec` will have a single element.

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingOptions.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingOptions.html#impl-ListingOptions" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingOptions.html" class="struct" title="struct datafusion::datasource::listing::ListingOptions">ListingOptions</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingOptions.html#method.new" class="fn">new</a>(format: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/trait.FileFormat.html" class="trait" title="trait datafusion::datasource::file_format::FileFormat">FileFormat</a>\>) -\> Self

Creates an options instance with the given format Default values:

- use default file extension filter
- no input partition to discover
- one target partition
- do not collect statistics

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingOptions.html#method.with_session_config_options" class="fn">with_session_config_options</a>(self, config: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html" class="struct" title="struct datafusion::prelude::SessionConfig">SessionConfig</a>) -\> Self

Set options from [`SessionConfig`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html "struct datafusion::prelude::SessionConfig") and returns self.

Currently this sets `target_partitions` and `collect_stat` but if more options are added in the future that need to be coordinated they will be synchronized through this method.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingOptions.html#method.with_file_extension" class="fn">with_file_extension</a>(self, file_extension: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>) -\> Self

Set file extension on [`ListingOptions`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingOptions.html "struct datafusion::datasource::listing::ListingOptions") and returns self.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingOptions.html#example" class="doc-anchor">§</a>Example

``` rust

let listing_options = ListingOptions::new(Arc::new(
    ParquetFormat::default()
  ))
  .with_file_extension(".parquet");

assert_eq!(listing_options.file_extension, ".parquet");
```

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingOptions.html#method.with_file_extension_opt" class="fn">with_file_extension_opt</a>\<S\>(self, file_extension: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<S\>) -\> Self

where S: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>,

Optionally set file extension on [`ListingOptions`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingOptions.html "struct datafusion::datasource::listing::ListingOptions") and returns self.

If `file_extension` is `None`, the file extension will not be changed

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingOptions.html#example-1" class="doc-anchor">§</a>Example

``` rust
let extension = Some(".parquet");
let listing_options = ListingOptions::new(Arc::new(
    ParquetFormat::default()
  ))
  .with_file_extension_opt(extension);

assert_eq!(listing_options.file_extension, ".parquet");
```

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingOptions.html#method.with_table_partition_cols" class="fn">with_table_partition_cols</a>( self, table_partition_cols: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<(<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>)\>, ) -\> Self

Set `table partition columns` on [`ListingOptions`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingOptions.html "struct datafusion::datasource::listing::ListingOptions") and returns self.

“partition columns,” used to support [Hive Partitioning](https://docs.cloudera.com/HDPDocuments/HDP2/HDP-2.1.3/bk_system-admin-guide/content/hive_partitioned_tables.html), are columns added to the data that is read, based on the folder structure where the data resides.

For example, give the following files in your filesystem:

``` text
/mnt/nyctaxi/year=2022/month=01/tripdata.parquet
/mnt/nyctaxi/year=2021/month=12/tripdata.parquet
/mnt/nyctaxi/year=2021/month=11/tripdata.parquet
```

A [`ListingTable`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTable.html "struct datafusion::datasource::listing::ListingTable") created at `/mnt/nyctaxi/` with partition columns “year” and “month” will include new `year` and `month` columns while reading the files. The `year` column would have value `2022` and the `month` column would have value `01` for the rows read from `/mnt/nyctaxi/year=2022/month=01/tripdata.parquet`

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingOptions.html#notes" class="doc-anchor">§</a>Notes

- If only one level (e.g. `year` in the example above) is specified, the other levels are ignored but the files are still read.

- Files that don’t follow this partitioning scheme will be ignored.

- Since the columns have the same value for all rows read from each individual file (such as dates), they are typically dictionary encoded for efficiency. You may use [`wrap_partition_type_in_dict`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/fn.wrap_partition_type_in_dict.html "fn datafusion::datasource::physical_plan::wrap_partition_type_in_dict") to request a dictionary-encoded type.

- The partition columns are solely extracted from the file path. Especially they are NOT part of the parquet files itself.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingOptions.html#example-2" class="doc-anchor">§</a>Example

``` rust

// listing options for files with paths such as  `/mnt/data/col_a=x/col_b=y/data.parquet`
// `col_a` and `col_b` will be included in the data read from those files
let listing_options = ListingOptions::new(Arc::new(
    ParquetFormat::default()
  ))
  .with_table_partition_cols(vec![("col_a".to_string(), DataType::Utf8),
      ("col_b".to_string(), DataType::Utf8)]);

assert_eq!(listing_options.table_partition_cols, vec![("col_a".to_string(), DataType::Utf8),
    ("col_b".to_string(), DataType::Utf8)]);
```

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingOptions.html#method.with_collect_stat" class="fn">with_collect_stat</a>(self, collect_stat: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> Self

Set stat collection on [`ListingOptions`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingOptions.html "struct datafusion::datasource::listing::ListingOptions") and returns self.

``` rust

let listing_options = ListingOptions::new(Arc::new(
    ParquetFormat::default()
  ))
  .with_collect_stat(true);

assert_eq!(listing_options.collect_stat, true);
```

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingOptions.html#method.with_target_partitions" class="fn">with_target_partitions</a>(self, target_partitions: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> Self

Set number of target partitions on [`ListingOptions`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingOptions.html "struct datafusion::datasource::listing::ListingOptions") and returns self.

``` rust

let listing_options = ListingOptions::new(Arc::new(
    ParquetFormat::default()
  ))
  .with_target_partitions(8);

assert_eq!(listing_options.target_partitions, 8);
```

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingOptions.html#method.with_file_sort_order" class="fn">with_file_sort_order</a>(self, file_sort_order: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.SortExpr.html" class="struct" title="struct datafusion::logical_expr::SortExpr">SortExpr</a>\>\>) -\> Self

Set file sort order on [`ListingOptions`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingOptions.html "struct datafusion::datasource::listing::ListingOptions") and returns self.

``` rust

 // Tell datafusion that the files are sorted by column "a"
 let file_sort_order = vec![vec![
   col("a").sort(true, true)
 ]];

let listing_options = ListingOptions::new(Arc::new(
    ParquetFormat::default()
  ))
  .with_file_sort_order(file_sort_order.clone());

assert_eq!(listing_options.file_sort_order, file_sort_order);
```

#### pub async fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingOptions.html#method.infer_schema" class="fn">infer_schema</a>\<'a\>( &'a self, state: &dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.Session.html" class="trait" title="trait datafusion::catalog::Session">Session</a>, table_path: &'a <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableUrl.html" class="struct" title="struct datafusion::datasource::listing::ListingTableUrl">ListingTableUrl</a>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/type.SchemaRef.html" class="type" title="type datafusion::common::arrow::datatypes::SchemaRef">SchemaRef</a>\>

Infer the schema of the files at the given path on the provided object store.

If the table_path contains one or more files (i.e. it is a directory / prefix of files) their schema is merged by calling [`FileFormat::infer_schema`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/trait.FileFormat.html#tymethod.infer_schema "method datafusion::datasource::file_format::FileFormat::infer_schema")

Note: The inferred schema does not include any partitioning columns.

This method is called as part of creating a [`ListingTable`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTable.html "struct datafusion::datasource::listing::ListingTable").

#### pub async fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingOptions.html#method.validate_partitions" class="fn">validate_partitions</a>( &self, state: &dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.Session.html" class="trait" title="trait datafusion::catalog::Session">Session</a>, table_path: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableUrl.html" class="struct" title="struct datafusion::datasource::listing::ListingTableUrl">ListingTableUrl</a>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

Infers the partition columns stored in `LOCATION` and compares them with the columns provided in `PARTITIONED BY` to help prevent accidental corrupts of partitioned tables.

Allows specifying partial partitions.

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingOptions.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingOptions.html#impl-Clone-for-ListingOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingOptions.html" class="struct" title="struct datafusion::datasource::listing::ListingOptions">ListingOptions</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingOptions.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingOptions.html" class="struct" title="struct datafusion::datasource::listing::ListingOptions">ListingOptions</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingOptions.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingOptions.html#impl-Debug-for-ListingOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingOptions.html" class="struct" title="struct datafusion::datasource::listing::ListingOptions">ListingOptions</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingOptions.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingOptions.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingOptions.html#blanket-implementations" class="anchor">§</a>
