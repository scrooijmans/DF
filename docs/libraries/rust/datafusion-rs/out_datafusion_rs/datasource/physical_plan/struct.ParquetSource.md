# Struct ParquetSource Copy item path

<a href="https://docs.rs/datafusion-datasource-parquet/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_datasource_parquet/source.rs.html#269" class="src">Source</a>

``` rust
pub struct ParquetSource { /* private fields */ }
```

Available on **crate feature `parquet`** only.

Expand description

Execution plan for reading one or more Parquet files.

``` text
            ▲
            │
            │  Produce a stream of
            │  RecordBatches
            │
┌───────────────────────┐
│                       │
│     DataSourceExec    │
│                       │
└───────────────────────┘
            ▲
            │  Asynchronously read from one
            │  or more parquet files via
            │  ObjectStore interface
            │
            │
  .───────────────────.
 │                     )
 │`───────────────────'│
 │    ObjectStore      │
 │.───────────────────.│
 │                     )
  `───────────────────'
```

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.ParquetSource.html#example-create-a-datasourceexec" class="doc-anchor">§</a>Example: Create a `DataSourceExec`

``` rust

let source = Arc::new(
    ParquetSource::default()
    .with_predicate(predicate)
);
// Create a DataSourceExec for reading `file1.parquet` with a file size of 100MB
let config = FileScanConfigBuilder::new(object_store_url, file_schema, source)
   .with_file(PartitionedFile::new("file1.parquet", 100*1024*1024)).build();
let exec = DataSourceExec::from_data_source(config);
```

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.ParquetSource.html#features" class="doc-anchor">§</a>Features

Supports the following optimizations:

- Concurrent reads: reads from one or more files in parallel as multiple partitions, including concurrently reading multiple row groups from a single file.

- Predicate push down: skips row groups, pages, rows based on metadata and late materialization. See “Predicate Pushdown” below.

- Projection pushdown: reads and decodes only the columns required.

- Limit pushdown: stop execution early after some number of rows are read.

- Custom readers: customize reading parquet files, e.g. to cache metadata, coalesce I/O operations, etc. See [`ParquetFileReaderFactory`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.ParquetFileReaderFactory.html "trait datafusion::datasource::physical_plan::ParquetFileReaderFactory") for more details.

- Schema evolution: read parquet files with different schemas into a unified table schema. See [`SchemaAdapterFactory`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/schema_adapter/trait.SchemaAdapterFactory.html "trait datafusion::datasource::schema_adapter::SchemaAdapterFactory") for more details.

- metadata_size_hint: controls the number of bytes read from the end of the file in the initial I/O when the default [`ParquetFileReaderFactory`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.ParquetFileReaderFactory.html "trait datafusion::datasource::physical_plan::ParquetFileReaderFactory"). If a custom reader is used, it supplies the metadata directly and this parameter is ignored. [`ParquetSource::with_metadata_size_hint`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.ParquetSource.html#method.with_metadata_size_hint "method datafusion::datasource::physical_plan::ParquetSource::with_metadata_size_hint") for more details.

- User provided `ParquetAccessPlan`s to skip row groups and/or pages based on external information. See “Implementing External Indexes” below

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.ParquetSource.html#predicate-pushdown" class="doc-anchor">§</a>Predicate Pushdown

`DataSourceExec` uses the provided [`PhysicalExpr`](https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html "trait datafusion::physical_expr::PhysicalExpr") predicate as a filter to skip reading unnecessary data and improve query performance using several techniques:

- Row group pruning: skips entire row groups based on min/max statistics found in [`ParquetMetaData`](https://docs.rs/parquet/56.0.0/x86_64-unknown-linux-gnu/parquet/file/metadata/struct.ParquetMetaData.html "struct parquet::file::metadata::ParquetMetaData") and any Bloom filters that are present.

- Page pruning: skips individual pages within a ColumnChunk using the [Parquet PageIndex](https://github.com/apache/parquet-format/blob/master/PageIndex.md), if present.

- Row filtering: skips rows within a page using a form of late materialization. When possible, predicates are applied by the parquet decoder *during* decode (see [`ArrowPredicate`](https://docs.rs/parquet/56.0.0/x86_64-unknown-linux-gnu/parquet/arrow/arrow_reader/filter/trait.ArrowPredicate.html "trait parquet::arrow::arrow_reader::filter::ArrowPredicate") and [`RowFilter`](https://docs.rs/parquet/56.0.0/x86_64-unknown-linux-gnu/parquet/arrow/arrow_reader/filter/struct.RowFilter.html "struct parquet::arrow::arrow_reader::filter::RowFilter") for more details). This is only enabled if `ParquetScanOptions::pushdown_filters` is set to true.

Note: If the predicate can not be used to accelerate the scan, it is ignored (no error is raised on predicate evaluation errors).

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.ParquetSource.html#example-rewriting-datasourceexec" class="doc-anchor">§</a>Example: rewriting `DataSourceExec`

You can modify a `DataSourceExec` using [`ParquetSource`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.ParquetSource.html "struct datafusion::datasource::physical_plan::ParquetSource"), for example to change files or add a predicate.

``` rust

// Split a single DataSourceExec into multiple DataSourceExecs, one for each file
let exec = parquet_exec();
let data_source = exec.data_source();
let base_config = data_source.as_any().downcast_ref::<FileScanConfig>().unwrap();
let existing_file_groups = &base_config.file_groups;
let new_execs = existing_file_groups
  .iter()
  .map(|file_group| {
    // create a new exec by copying the existing exec's source config
    let new_config = FileScanConfigBuilder::from(base_config.clone())
       .with_file_groups(vec![file_group.clone()])
      .build();

    (DataSourceExec::from_data_source(new_config))
  })
  .collect::<Vec<_>>();
```

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.ParquetSource.html#implementing-external-indexes" class="doc-anchor">§</a>Implementing External Indexes

It is possible to restrict the row groups and selections within those row groups that the DataSourceExec will consider by providing an initial `ParquetAccessPlan` as `extensions` on `PartitionedFile`. This can be used to implement external indexes on top of parquet files and select only portions of the files.

The `DataSourceExec` will try and reduce any provided `ParquetAccessPlan` further based on the contents of `ParquetMetadata` and other settings.

### <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.ParquetSource.html#example-of-providing-a-parquetaccessplan" class="doc-anchor">§</a>Example of providing a ParquetAccessPlan

``` rust

// create an access plan to scan row group 0, 1 and 3 and skip row groups 2 and 4
let mut access_plan = ParquetAccessPlan::new_all(5);
access_plan.skip(2);
access_plan.skip(4);
// provide the plan as extension to the FileScanConfig
let partitioned_file = PartitionedFile::new("my_file.parquet", 1234)
  .with_extensions(Arc::new(access_plan));
// create a FileScanConfig to scan this file
let config = FileScanConfigBuilder::new(ObjectStoreUrl::local_filesystem(), schema(), Arc::new(ParquetSource::default()))
    .with_file(partitioned_file).build();
// this parquet DataSourceExec will not even try to read row groups 2 and 4. Additional
// pruning based on predicates may also happen
let exec = DataSourceExec::from_data_source(config);
```

For a complete example, see the \[`advanced_parquet_index` example\]).

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.ParquetSource.html#execution-overview" class="doc-anchor">§</a>Execution Overview

- Step 1: `DataSourceExec::execute` is called, returning a `FileStream` configured to open parquet files with a `ParquetOpener`.

- Step 2: When the stream is polled, the `ParquetOpener` is called to open the file.

- Step 3: The `ParquetOpener` gets the [`ParquetMetaData`](https://docs.rs/parquet/56.0.0/x86_64-unknown-linux-gnu/parquet/file/metadata/struct.ParquetMetaData.html "struct parquet::file::metadata::ParquetMetaData") (file metadata) via [`ParquetFileReaderFactory`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.ParquetFileReaderFactory.html "trait datafusion::datasource::physical_plan::ParquetFileReaderFactory"), creating a `ParquetAccessPlan` by applying predicates to metadata. The plan and projections are used to determine what pages must be read.

- Step 4: The stream begins reading data, fetching the required parquet pages incrementally decoding them, and applying any row filters (see [`Self::with_pushdown_filters`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.ParquetSource.html#method.with_pushdown_filters "method datafusion::datasource::physical_plan::ParquetSource::with_pushdown_filters")).

- Step 5: As each [`RecordBatch`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html "struct datafusion::common::arrow::array::RecordBatch") is read, it may be adapted by a [`SchemaAdapter`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/schema_adapter/trait.SchemaAdapter.html "trait datafusion::datasource::schema_adapter::SchemaAdapter") to match the table schema. By default missing columns are filled with nulls, but this can be customized via [`SchemaAdapterFactory`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/schema_adapter/trait.SchemaAdapterFactory.html "trait datafusion::datasource::schema_adapter::SchemaAdapterFactory").

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.ParquetSource.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.ParquetSource.html#impl-ParquetSource" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.ParquetSource.html" class="struct" title="struct datafusion::datasource::physical_plan::ParquetSource">ParquetSource</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.ParquetSource.html#method.new" class="fn">new</a>(table_parquet_options: <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableParquetOptions.html" class="struct" title="struct datafusion::config::TableParquetOptions">TableParquetOptions</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.ParquetSource.html" class="struct" title="struct datafusion::datasource::physical_plan::ParquetSource">ParquetSource</a>

Create a new ParquetSource to read the data specified in the file scan configuration with the provided `TableParquetOptions`. if default values are going to be used, use `ParguetConfig::default()` instead

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.ParquetSource.html#method.with_metadata_size_hint" class="fn">with_metadata_size_hint</a>(self, metadata_size_hint: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.ParquetSource.html" class="struct" title="struct datafusion::datasource::physical_plan::ParquetSource">ParquetSource</a>

Set the metadata size hint

This value determines how many bytes at the end of the file the default [`ParquetFileReaderFactory`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.ParquetFileReaderFactory.html "trait datafusion::datasource::physical_plan::ParquetFileReaderFactory") will request in the initial IO. If this is too small, the ParquetSource will need to make additional IO requests to read the footer.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.ParquetSource.html#method.with_predicate" class="fn">with_predicate</a>(&self, predicate: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html" class="trait" title="trait datafusion::physical_expr::PhysicalExpr">PhysicalExpr</a>\>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.ParquetSource.html" class="struct" title="struct datafusion::datasource::physical_plan::ParquetSource">ParquetSource</a>

Set predicate information

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.ParquetSource.html#method.with_encryption_factory" class="fn">with_encryption_factory</a>( self, encryption_factory: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/parquet_encryption/trait.EncryptionFactory.html" class="trait" title="trait datafusion::execution::parquet_encryption::EncryptionFactory">EncryptionFactory</a>\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.ParquetSource.html" class="struct" title="struct datafusion::datasource::physical_plan::ParquetSource">ParquetSource</a>

Set the encryption factory to use to generate file decryption properties

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.ParquetSource.html#method.table_parquet_options" class="fn">table_parquet_options</a>(&self) -\> &<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableParquetOptions.html" class="struct" title="struct datafusion::config::TableParquetOptions">TableParquetOptions</a>

Options passed to the parquet reader for this scan

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.ParquetSource.html#method.predicate" class="fn">predicate</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html" class="trait" title="trait datafusion::physical_expr::PhysicalExpr">PhysicalExpr</a>\>\>

Optional predicate.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.ParquetSource.html#method.parquet_file_reader_factory" class="fn">parquet_file_reader_factory</a>( &self, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.ParquetFileReaderFactory.html" class="trait" title="trait datafusion::datasource::physical_plan::ParquetFileReaderFactory">ParquetFileReaderFactory</a>\>\>

return the optional file reader factory

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.ParquetSource.html#method.with_parquet_file_reader_factory" class="fn">with_parquet_file_reader_factory</a>( self, parquet_file_reader_factory: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.ParquetFileReaderFactory.html" class="trait" title="trait datafusion::datasource::physical_plan::ParquetFileReaderFactory">ParquetFileReaderFactory</a>\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.ParquetSource.html" class="struct" title="struct datafusion::datasource::physical_plan::ParquetSource">ParquetSource</a>

Optional user defined parquet file reader factory.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.ParquetSource.html#method.with_pushdown_filters" class="fn">with_pushdown_filters</a>(self, pushdown_filters: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.ParquetSource.html" class="struct" title="struct datafusion::datasource::physical_plan::ParquetSource">ParquetSource</a>

If true, the predicate will be used during the parquet scan. Defaults to false.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.ParquetSource.html#method.with_reorder_filters" class="fn">with_reorder_filters</a>(self, reorder_filters: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.ParquetSource.html" class="struct" title="struct datafusion::datasource::physical_plan::ParquetSource">ParquetSource</a>

If true, the `RowFilter` made by `pushdown_filters` may try to minimize the cost of filter evaluation by reordering the predicate [`Expr`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html "enum datafusion::prelude::Expr")s. If false, the predicates are applied in the same order as specified in the query. Defaults to false.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.ParquetSource.html#method.with_enable_page_index" class="fn">with_enable_page_index</a>(self, enable_page_index: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.ParquetSource.html" class="struct" title="struct datafusion::datasource::physical_plan::ParquetSource">ParquetSource</a>

If enabled, the reader will read the page index This is used to optimize filter pushdown via `RowSelector` and `RowFilter` by eliminating unnecessary IO and decoding

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.ParquetSource.html#method.with_bloom_filter_on_read" class="fn">with_bloom_filter_on_read</a>( self, bloom_filter_on_read: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.ParquetSource.html" class="struct" title="struct datafusion::datasource::physical_plan::ParquetSource">ParquetSource</a>

If enabled, the reader will read by the bloom filter

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.ParquetSource.html#method.with_bloom_filter_on_write" class="fn">with_bloom_filter_on_write</a>( self, enable_bloom_filter_on_write: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.ParquetSource.html" class="struct" title="struct datafusion::datasource::physical_plan::ParquetSource">ParquetSource</a>

If enabled, the writer will write by the bloom filter

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.ParquetSource.html#method.apply_schema_adapter" class="fn">apply_schema_adapter</a>( self, conf: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.FileScanConfig.html" class="struct" title="struct datafusion::datasource::physical_plan::FileScanConfig">FileScanConfig</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSource.html" class="trait" title="trait datafusion::datasource::physical_plan::FileSource">FileSource</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Applies schema adapter factory from the FileScanConfig if present.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.ParquetSource.html#arguments" class="doc-anchor">§</a>Arguments

- `conf` - FileScanConfig that may contain a schema adapter factory

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.ParquetSource.html#returns" class="doc-anchor">§</a>Returns

The converted FileSource with schema adapter factory applied if provided

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.ParquetSource.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.ParquetSource.html#impl-Clone-for-ParquetSource" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.ParquetSource.html" class="struct" title="struct datafusion::datasource::physical_plan::ParquetSource">ParquetSource</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.ParquetSource.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.ParquetSource.html" class="struct" title="struct datafusion::datasource::physical_plan::ParquetSource">ParquetSource</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.ParquetSource.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.ParquetSource.html#impl-Debug-for-ParquetSource" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.ParquetSource.html" class="struct" title="struct datafusion::datasource::physical_plan::ParquetSource">ParquetSource</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.ParquetSource.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.ParquetSource.html#impl-Default-for-ParquetSource" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.ParquetSource.html" class="struct" title="struct datafusion::datasource::physical_plan::ParquetSource">ParquetSource</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.ParquetSource.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.ParquetSource.html" class="struct" title="struct datafusion::datasource::physical_plan::ParquetSource">ParquetSource</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.ParquetSource.html#impl-FileSource-for-ParquetSource" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSource.html" class="trait" title="trait datafusion::datasource::physical_plan::FileSource">FileSource</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.ParquetSource.html" class="struct" title="struct datafusion::datasource::physical_plan::ParquetSource">ParquetSource</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.ParquetSource.html#method.create_file_opener" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSource.html#tymethod.create_file_opener" class="fn">create_file_opener</a>( &self, object_store: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/object_store/0.12.3/x86_64-unknown-linux-gnu/object_store/trait.ObjectStore.html" class="trait" title="trait object_store::ObjectStore">ObjectStore</a>\>, base_config: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.FileScanConfig.html" class="struct" title="struct datafusion::datasource::physical_plan::FileScanConfig">FileScanConfig</a>, partition: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileOpener.html" class="trait" title="trait datafusion::datasource::physical_plan::FileOpener">FileOpener</a>\>

Creates a `dyn FileOpener` based on given parameters

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.ParquetSource.html#method.as_any" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSource.html#tymethod.as_any" class="fn">as_any</a>(&self) -\> &(dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + 'static)

Any

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.ParquetSource.html#method.filter" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSource.html#method.filter" class="fn">filter</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html" class="trait" title="trait datafusion::physical_expr::PhysicalExpr">PhysicalExpr</a>\>\>

Returns the filter expression that will be applied during the file scan.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.ParquetSource.html#method.with_batch_size" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSource.html#tymethod.with_batch_size" class="fn">with_batch_size</a>(&self, batch_size: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSource.html" class="trait" title="trait datafusion::datasource::physical_plan::FileSource">FileSource</a>\>

Initialize new type with batch size configuration

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.ParquetSource.html#method.with_schema" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSource.html#tymethod.with_schema" class="fn">with_schema</a>(&self, schema: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>\>) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSource.html" class="trait" title="trait datafusion::datasource::physical_plan::FileSource">FileSource</a>\>

Initialize new instance with a new schema

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.ParquetSource.html#method.with_statistics" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSource.html#tymethod.with_statistics" class="fn">with_statistics</a>(&self, statistics: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Statistics.html" class="struct" title="struct datafusion::common::Statistics">Statistics</a>) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSource.html" class="trait" title="trait datafusion::datasource::physical_plan::FileSource">FileSource</a>\>

Initialize new instance with projected statistics

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.ParquetSource.html#method.with_projection" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSource.html#tymethod.with_projection" class="fn">with_projection</a>(&self, \_config: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.FileScanConfig.html" class="struct" title="struct datafusion::datasource::physical_plan::FileScanConfig">FileScanConfig</a>) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSource.html" class="trait" title="trait datafusion::datasource::physical_plan::FileSource">FileSource</a>\>

Initialize new instance with projection information

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.ParquetSource.html#method.metrics" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSource.html#tymethod.metrics" class="fn">metrics</a>(&self) -\> &<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.ExecutionPlanMetricsSet.html" class="struct" title="struct datafusion::physical_plan::metrics::ExecutionPlanMetricsSet">ExecutionPlanMetricsSet</a>

Return execution plan metrics

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.ParquetSource.html#method.statistics" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSource.html#tymethod.statistics" class="fn">statistics</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Statistics.html" class="struct" title="struct datafusion::common::Statistics">Statistics</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Return projected statistics

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.ParquetSource.html#method.file_type" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSource.html#tymethod.file_type" class="fn">file_type</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

String representation of file source such as “csv”, “json”, “parquet”

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.ParquetSource.html#method.fmt_extra" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSource.html#method.fmt_extra" class="fn">fmt_extra</a>( &self, t: <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/enum.DisplayFormatType.html" class="enum" title="enum datafusion::physical_plan::DisplayFormatType">DisplayFormatType</a>, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Format FileType specific information

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.ParquetSource.html#method.try_pushdown_filters" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSource.html#method.try_pushdown_filters" class="fn">try_pushdown_filters</a>( &self, filters: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html" class="trait" title="trait datafusion::physical_expr::PhysicalExpr">PhysicalExpr</a>\>\>, config: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigOptions.html" class="struct" title="struct datafusion::config::ConfigOptions">ConfigOptions</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter_pushdown/struct.FilterPushdownPropagation.html" class="struct" title="struct datafusion::physical_plan::filter_pushdown::FilterPushdownPropagation">FilterPushdownPropagation</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSource.html" class="trait" title="trait datafusion::datasource::physical_plan::FileSource">FileSource</a>\>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Try to push down filters into this FileSource. See [`ExecutionPlan::handle_child_pushdown_result`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.handle_child_pushdown_result "method datafusion::physical_plan::ExecutionPlan::handle_child_pushdown_result") for more details.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.ParquetSource.html#method.with_schema_adapter_factory" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSource.html#method.with_schema_adapter_factory" class="fn">with_schema_adapter_factory</a>( &self, schema_adapter_factory: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/schema_adapter/trait.SchemaAdapterFactory.html" class="trait" title="trait datafusion::datasource::schema_adapter::SchemaAdapterFactory">SchemaAdapterFactory</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSource.html" class="trait" title="trait datafusion::datasource::physical_plan::FileSource">FileSource</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Set optional schema adapter factory. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSource.html#method.with_schema_adapter_factory)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.ParquetSource.html#method.schema_adapter_factory" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSource.html#method.schema_adapter_factory" class="fn">schema_adapter_factory</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/schema_adapter/trait.SchemaAdapterFactory.html" class="trait" title="trait datafusion::datasource::schema_adapter::SchemaAdapterFactory">SchemaAdapterFactory</a>\>\>

Returns the current schema adapter factory if set [Read more](https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSource.html#method.schema_adapter_factory)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.ParquetSource.html#method.repartitioned" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSource.html#method.repartitioned" class="fn">repartitioned</a>( &self, target_partitions: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, repartition_file_min_size: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, output_ordering: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.LexOrdering.html" class="struct" title="struct datafusion::physical_expr::LexOrdering">LexOrdering</a>\>, config: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.FileScanConfig.html" class="struct" title="struct datafusion::datasource::physical_plan::FileScanConfig">FileScanConfig</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.FileScanConfig.html" class="struct" title="struct datafusion::datasource::physical_plan::FileScanConfig">FileScanConfig</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

If supported by the [`FileSource`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSource.html "trait datafusion::datasource::physical_plan::FileSource"), redistribute files across partitions according to their size. Allows custom file formats to implement their own repartitioning logic. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSource.html#method.repartitioned)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.ParquetSource.html#impl-From%3CParquetSource%3E-for-Arc%3Cdyn+FileSource%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.ParquetSource.html" class="struct" title="struct datafusion::datasource::physical_plan::ParquetSource">ParquetSource</a>\> for <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSource.html" class="trait" title="trait datafusion::datasource::physical_plan::FileSource">FileSource</a>\>

Allows easy conversion from ParquetSource to Arc\<dyn FileSource\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.ParquetSource.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(source: <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.ParquetSource.html" class="struct" title="struct datafusion::datasource::physical_plan::ParquetSource">ParquetSource</a>) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSource.html" class="trait" title="trait datafusion::datasource::physical_plan::FileSource">FileSource</a>\>

Converts to this type from the input type.

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.ParquetSource.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.ParquetSource.html#blanket-implementations" class="anchor">§</a>
