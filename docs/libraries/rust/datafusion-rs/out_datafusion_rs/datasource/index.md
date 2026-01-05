# Module datasource Copy item path

<a href="https://docs.rs/datafusion/50.2.0/src/datafusion/datasource/mod.rs.html#18-273" class="src">Source</a>

Expand description

DataFusion data sources: [`TableProvider`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html "trait datafusion::datasource::TableProvider") and [`ListingTable`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTable.html "struct datafusion::datasource::listing::ListingTable")

## Modules<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/index.html#modules" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/cte_worktable/index.html" class="mod" title="mod datafusion::datasource::cte_worktable">cte_worktable</a>  
CteWorkTable implementation used for recursive queries

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/default_table_source/index.html" class="mod" title="mod datafusion::datasource::default_table_source">default_table_source</a>  
Default TableSource implementation used in DataFusion physical plans

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/dynamic_file/index.html" class="mod" title="mod datafusion::datasource::dynamic_file">dynamic_file</a>  
dynamic_file_schema contains an [`UrlTableFactory`](https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.UrlTableFactory.html "trait datafusion::catalog::UrlTableFactory") implementation that can create a [`ListingTable`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTable.html "struct datafusion::datasource::listing::ListingTable") from the given url.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/empty/index.html" class="mod" title="mod datafusion::datasource::empty">empty</a>  
[`EmptyTable`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/empty/struct.EmptyTable.html "struct datafusion::datasource::empty::EmptyTable") useful for testing.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/index.html" class="mod" title="mod datafusion::datasource::file_format">file_format</a>  
Module containing helper methods for the various file formats See write.rs for write related helper methods

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/index.html" class="mod" title="mod datafusion::datasource::listing">listing</a>  
A table that uses the `ObjectStore` listing capability to get the list of files to process.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing_table_factory/index.html" class="mod" title="mod datafusion::datasource::listing_table_factory">listing_table_factory</a>  
Factory for creating ListingTables with default options

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/memory/index.html" class="mod" title="mod datafusion::datasource::memory">memory</a>  
<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/object_store/index.html" class="mod" title="mod datafusion::datasource::object_store">object_store</a>  
ObjectStoreRegistry holds all the object stores at Runtime with a scheme for each store. This allows the user to extend DataFusion with different storage systems such as S3 or HDFS and query data inside these systems.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/index.html" class="mod" title="mod datafusion::datasource::physical_plan">physical_plan</a>  
Execution plans that read file formats

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/provider/index.html" class="mod" title="mod datafusion::datasource::provider">provider</a>  
Data source traits

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/schema_adapter/index.html" class="mod" title="mod datafusion::datasource::schema_adapter">schema_adapter</a>  
[`SchemaAdapter`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/schema_adapter/trait.SchemaAdapter.html "trait datafusion::datasource::schema_adapter::SchemaAdapter") and [`SchemaAdapterFactory`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/schema_adapter/trait.SchemaAdapterFactory.html "trait datafusion::datasource::schema_adapter::SchemaAdapterFactory") to adapt file-level record batches to a table schema.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/sink/index.html" class="mod" title="mod datafusion::datasource::sink">sink</a>  
Execution plan for writing data to [`DataSink`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/sink/trait.DataSink.html "trait datafusion::datasource::sink::DataSink")s

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/source/index.html" class="mod" title="mod datafusion::datasource::source">source</a>  
[`DataSource`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/source/trait.DataSource.html "trait datafusion::datasource::source::DataSource") and [`DataSourceExec`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/memory/struct.DataSourceExec.html "struct datafusion::datasource::memory::DataSourceExec")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/stream/index.html" class="mod" title="mod datafusion::datasource::stream">stream</a>  
TableProvider for stream sources, such as FIFO files

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/view/index.html" class="mod" title="mod datafusion::datasource::view">view</a>  
View data source which uses a LogicalPlan as it’s input.

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/struct.DefaultTableSource.html" class="struct" title="struct datafusion::datasource::DefaultTableSource">DefaultTableSource</a>  
Implements [`TableSource`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.TableSource.html "trait datafusion::logical_expr::TableSource") for a [`TableProvider`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html "trait datafusion::datasource::TableProvider")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/struct.MemTable.html" class="struct" title="struct datafusion::datasource::MemTable">MemTable</a>  
In-memory data source for presenting a `Vec<RecordBatch>` as a data source that can be queried by DataFusion. This allows data to be pre-loaded into memory and then repeatedly queried without incurring additional file I/O overhead.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/struct.ViewTable.html" class="struct" title="struct datafusion::datasource::ViewTable">ViewTable</a>  
An implementation of `TableProvider` that uses another logical plan.

## Enums<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/index.html#enums" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/enum.TableType.html" class="enum" title="enum datafusion::datasource::TableType">TableType</a>  
Indicates the type of this table for metadata/catalog purposes.

## Traits<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/index.html#traits" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html" class="trait" title="trait datafusion::datasource::TableProvider">TableProvider</a>  
A table which can be queried and modified.

## Functions<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/fn.create_ordering.html" class="fn" title="fn datafusion::datasource::create_ordering">create_ordering</a>  
Converts logical sort expressions to physical sort expressions.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/fn.provider_as_source.html" class="fn" title="fn datafusion::datasource::provider_as_source">provider_as_source</a>  
Wrap TableProvider in TableSource

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/fn.source_as_provider.html" class="fn" title="fn datafusion::datasource::source_as_provider">source_as_provider</a>  
Attempt to downcast a TableSource to DefaultTableSource and access the TableProvider. This will only work with a TableSource created by DataFusion.
