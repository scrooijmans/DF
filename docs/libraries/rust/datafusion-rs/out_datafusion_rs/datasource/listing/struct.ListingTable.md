# Struct ListingTable Copy item path

<a href="https://docs.rs/datafusion/50.2.0/src/datafusion/datasource/listing/table.rs.html#942-969" class="src">Source</a>

``` rust
pub struct ListingTable { /* private fields */ }
```

Expand description

Built in [`TableProvider`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html "trait datafusion::datasource::TableProvider") that reads data from one or more files as a single table.

The files are read using an [`ObjectStore`](https://docs.rs/object_store/0.12.3/x86_64-unknown-linux-gnu/object_store/trait.ObjectStore.html "trait object_store::ObjectStore") instance, for example from local files or objects from AWS S3.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTable.html#features" class="doc-anchor">§</a>Features:

- Reading multiple files as a single table
- Hive style partitioning (e.g., directories named `date=2024-06-01`)
- Merges schemas from files with compatible but not identical schemas (see [`ListingTableConfig::file_schema`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableConfig.html#structfield.file_schema "field datafusion::datasource::listing::ListingTableConfig::file_schema"))
- `limit`, `filter` and `projection` pushdown for formats that support it (e.g., Parquet)
- Statistics collection and pruning based on file metadata
- Pre-existing sort order (see [`ListingOptions::file_sort_order`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingOptions.html#structfield.file_sort_order "field datafusion::datasource::listing::ListingOptions::file_sort_order"))
- Metadata caching to speed up repeated queries (see [`FileMetadataCache`](https://docs.rs/datafusion/50.2.0/datafusion/execution/cache/cache_manager/trait.FileMetadataCache.html "trait datafusion::execution::cache::cache_manager::FileMetadataCache"))
- Statistics caching (see [`FileStatisticsCache`](https://docs.rs/datafusion/50.2.0/datafusion/execution/cache/cache_manager/type.FileStatisticsCache.html "type datafusion::execution::cache::cache_manager::FileStatisticsCache"))

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTable.html#reading-directories-and-hive-style-partitioning" class="doc-anchor">§</a>Reading Directories and Hive Style Partitioning

For example, given the `table1` directory (or object store prefix)

``` text
table1
 ├── file1.parquet
 └── file2.parquet
```

A `ListingTable` would read the files `file1.parquet` and `file2.parquet` as a single table, merging the schemas if the files have compatible but not identical schemas.

Given the `table2` directory (or object store prefix)

``` text
table2
 ├── date=2024-06-01
 │    ├── file3.parquet
 │    └── file4.parquet
 └── date=2024-06-02
      └── file5.parquet
```

A `ListingTable` would read the files `file3.parquet`, `file4.parquet`, and `file5.parquet` as a single table, again merging schemas if necessary.

Given the hive style partitioning structure (e.g,. directories named `date=2024-06-01` and `date=2026-06-02`), `ListingTable` also adds a `date` column when reading the table:

- The files in `table2/date=2024-06-01` will have the value `2024-06-01`
- The files in `table2/date=2024-06-02` will have the value `2024-06-02`.

If the query has a predicate like `WHERE date = '2024-06-01'` only the corresponding directory will be read.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTable.html#see-also" class="doc-anchor">§</a>See Also

1.  [`ListingTableConfig`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableConfig.html "struct datafusion::datasource::listing::ListingTableConfig"): Configuration options
2.  [`DataSourceExec`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/memory/struct.DataSourceExec.html "struct datafusion::datasource::memory::DataSourceExec"): `ExecutionPlan` used by `ListingTable`

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTable.html#caching-metadata" class="doc-anchor">§</a>Caching Metadata

Some formats, such as Parquet, use the `FileMetadataCache` to cache file metadata that is needed to execute but expensive to read, such as row groups and statistics. The cache is scoped to the [`SessionContext`](https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html "struct datafusion::execution::context::SessionContext") and can be configured via the [runtime config options](https://datafusion.apache.org/user-guide/configs.html#runtime-configuration-settings).

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTable.html#example-read-a-directory-of-parquet-files-using-a-listingtable" class="doc-anchor">§</a>Example: Read a directory of parquet files using a [`ListingTable`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTable.html "struct datafusion::datasource::listing::ListingTable")

``` rust
let ctx = SessionContext::new();
let session_state = ctx.state();
let table_path = "/path/to/parquet";

// Parse the path
let table_path = ListingTableUrl::parse(table_path)?;

// Create default parquet options
let file_format = ParquetFormat::new();
let listing_options = ListingOptions::new(Arc::new(file_format))
  .with_file_extension(".parquet");

// Resolve the schema
let resolved_schema = listing_options
   .infer_schema(&session_state, &table_path)
   .await?;

let config = ListingTableConfig::new(table_path)
  .with_listing_options(listing_options)
  .with_schema(resolved_schema);

// Create a new TableProvider
let provider = Arc::new(ListingTable::try_new(config)?);

// This provider can now be read as a dataframe:
let df = ctx.read_table(provider.clone());

// or registered as a named table:
ctx.register_table("my_table", provider);
```

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTable.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTable.html#impl-ListingTable" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTable.html" class="struct" title="struct datafusion::datasource::listing::ListingTable">ListingTable</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTable.html#method.try_new" class="fn">try_new</a>(config: <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableConfig.html" class="struct" title="struct datafusion::datasource::listing::ListingTableConfig">ListingTableConfig</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<Self\>

Create new [`ListingTable`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTable.html "struct datafusion::datasource::listing::ListingTable")

See documentation and example on [`ListingTable`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTable.html "struct datafusion::datasource::listing::ListingTable") and [`ListingTableConfig`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableConfig.html "struct datafusion::datasource::listing::ListingTableConfig")

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTable.html#method.with_constraints" class="fn">with_constraints</a>(self, constraints: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Constraints.html" class="struct" title="struct datafusion::common::Constraints">Constraints</a>) -\> Self

Assign constraints

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTable.html#method.with_column_defaults" class="fn">with_column_defaults</a>( self, column_defaults: <a href="https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html" class="struct" title="struct std::collections::hash::map::HashMap">HashMap</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>\>, ) -\> Self

Assign column defaults

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTable.html#method.with_cache" class="fn">with_cache</a>(self, cache: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/cache/cache_manager/type.FileStatisticsCache.html" class="type" title="type datafusion::execution::cache::cache_manager::FileStatisticsCache">FileStatisticsCache</a>\>) -\> Self

Set the [`FileStatisticsCache`](https://docs.rs/datafusion/50.2.0/datafusion/execution/cache/cache_manager/type.FileStatisticsCache.html "type datafusion::execution::cache::cache_manager::FileStatisticsCache") used to cache parquet file statistics.

Setting a statistics cache on the `SessionContext` can avoid refetching statistics multiple times in the same session.

If `None`, creates a new [`DefaultFileStatisticsCache`](https://docs.rs/datafusion/50.2.0/datafusion/execution/cache/cache_unit/struct.DefaultFileStatisticsCache.html "struct datafusion::execution::cache::cache_unit::DefaultFileStatisticsCache") scoped to this query.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTable.html#method.with_definition" class="fn">with_definition</a>(self, definition: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>) -\> Self

Specify the SQL definition for this table, if any

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTable.html#method.table_paths" class="fn">table_paths</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableUrl.html" class="struct" title="struct datafusion::datasource::listing::ListingTableUrl">ListingTableUrl</a>\>

Get paths ref

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTable.html#method.options" class="fn">options</a>(&self) -\> &<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingOptions.html" class="struct" title="struct datafusion::datasource::listing::ListingOptions">ListingOptions</a>

Get options ref

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTable.html#method.schema_source" class="fn">schema_source</a>(&self) -\> SchemaSource

Get the schema source

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTable.html#method.with_schema_adapter_factory" class="fn">with_schema_adapter_factory</a>( self, schema_adapter_factory: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/schema_adapter/trait.SchemaAdapterFactory.html" class="trait" title="trait datafusion::datasource::schema_adapter::SchemaAdapterFactory">SchemaAdapterFactory</a>\>, ) -\> Self

Set the [`SchemaAdapterFactory`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/schema_adapter/trait.SchemaAdapterFactory.html "trait datafusion::datasource::schema_adapter::SchemaAdapterFactory") for this [`ListingTable`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTable.html "struct datafusion::datasource::listing::ListingTable")

The schema adapter factory is used to create schema adapters that can handle schema evolution and type conversions when reading files with different schemas than the table schema.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTable.html#example-adding-schema-evolution-support" class="doc-anchor">§</a>Example: Adding Schema Evolution Support

``` rust
let table_with_evolution = table
    .with_schema_adapter_factory(Arc::new(DefaultSchemaAdapterFactory));
```

See [`ListingTableConfig::with_schema_adapter_factory`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableConfig.html#method.with_schema_adapter_factory "method datafusion::datasource::listing::ListingTableConfig::with_schema_adapter_factory") for an example of custom SchemaAdapterFactory.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTable.html#method.schema_adapter_factory" class="fn">schema_adapter_factory</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/schema_adapter/trait.SchemaAdapterFactory.html" class="trait" title="trait datafusion::datasource::schema_adapter::SchemaAdapterFactory">SchemaAdapterFactory</a>\>\>

Get the [`SchemaAdapterFactory`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/schema_adapter/trait.SchemaAdapterFactory.html "trait datafusion::datasource::schema_adapter::SchemaAdapterFactory") for this table

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTable.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTable.html#impl-Clone-for-ListingTable" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTable.html" class="struct" title="struct datafusion::datasource::listing::ListingTable">ListingTable</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTable.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTable.html" class="struct" title="struct datafusion::datasource::listing::ListingTable">ListingTable</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTable.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTable.html#impl-Debug-for-ListingTable" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTable.html" class="struct" title="struct datafusion::datasource::listing::ListingTable">ListingTable</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTable.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTable.html#impl-TableProvider-for-ListingTable" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html" class="trait" title="trait datafusion::datasource::TableProvider">TableProvider</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTable.html" class="struct" title="struct datafusion::datasource::listing::ListingTable">ListingTable</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTable.html#method.as_any" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html#tymethod.as_any" class="fn">as_any</a>(&self) -\> &dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a>

Returns the table provider as [`Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") so that it can be downcast to a specific implementation.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTable.html#method.schema" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html#tymethod.schema" class="fn">schema</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/type.SchemaRef.html" class="type" title="type datafusion::common::arrow::datatypes::SchemaRef">SchemaRef</a>

Get a reference to the schema for this table

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTable.html#method.constraints" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html#method.constraints" class="fn">constraints</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Constraints.html" class="struct" title="struct datafusion::common::Constraints">Constraints</a>\>

Get a reference to the constraints of the table. Returns: [Read more](https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html#method.constraints)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTable.html#method.table_type" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html#tymethod.table_type" class="fn">table_type</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/enum.TableType.html" class="enum" title="enum datafusion::datasource::TableType">TableType</a>

Get the type of this table for metadata/catalog purposes.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTable.html#method.scan" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html#tymethod.scan" class="fn">scan</a>\<'life0, 'life1, 'life2, 'life3, 'async_trait\>( &'life0 self, state: &'life1 dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.Session.html" class="trait" title="trait datafusion::catalog::Session">Session</a>, projection: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&'life2 <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>\>, filters: &'life3 \[<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>\], limit: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a>\>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where Self: 'async_trait, 'life0: 'async_trait, 'life1: 'async_trait, 'life2: 'async_trait, 'life3: 'async_trait,

Create an [`ExecutionPlan`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html "trait datafusion::physical_plan::ExecutionPlan") for scanning the table with optionally specified `projection`, `filter` and `limit`, described below. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html#tymethod.scan)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTable.html#method.supports_filters_pushdown" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html#method.supports_filters_pushdown" class="fn">supports_filters_pushdown</a>( &self, filters: &\[&<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>\], ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TableProviderFilterPushDown.html" class="enum" title="enum datafusion::logical_expr::TableProviderFilterPushDown">TableProviderFilterPushDown</a>\>\>

Specify if DataFusion should provide filter expressions to the TableProvider to apply *during* the scan. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html#method.supports_filters_pushdown)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTable.html#method.get_table_definition" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html#method.get_table_definition" class="fn">get_table_definition</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>

Get the create statement used to create this table, if available.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTable.html#method.insert_into" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html#method.insert_into" class="fn">insert_into</a>\<'life0, 'life1, 'async_trait\>( &'life0 self, state: &'life1 dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.Session.html" class="trait" title="trait datafusion::catalog::Session">Session</a>, input: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a>\>, insert_op: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/dml/enum.InsertOp.html" class="enum" title="enum datafusion::logical_expr::logical_plan::dml::InsertOp">InsertOp</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a>\>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where Self: 'async_trait, 'life0: 'async_trait, 'life1: 'async_trait,

Return an [`ExecutionPlan`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html "trait datafusion::physical_plan::ExecutionPlan") to insert data into this table, if supported. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html#method.insert_into)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTable.html#method.get_column_default" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html#method.get_column_default" class="fn">get_column_default</a>(&self, column: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>\>

Get the default value for a column, if available.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTable.html#method.get_logical_plan" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html#method.get_logical_plan" class="fn">get_logical_plan</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html" class="enum" title="enum alloc::borrow::Cow">Cow</a>\<'\_, <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html" class="enum" title="enum datafusion::logical_expr::LogicalPlan">LogicalPlan</a>\>\>

Get the [`LogicalPlan`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html "enum datafusion::logical_expr::LogicalPlan") of this table, if available.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTable.html#method.statistics" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html#method.statistics" class="fn">statistics</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Statistics.html" class="struct" title="struct datafusion::common::Statistics">Statistics</a>\>

Get statistics for this table, if available Although not presently used in mainline DataFusion, this allows implementation specific behavior for downstream repositories, in conjunction with specialized optimizer rules to perform operations such as re-ordering of joins.

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTable.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTable.html#blanket-implementations" class="anchor">§</a>
