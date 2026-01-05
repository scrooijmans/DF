# Module catalog Copy item path

<a href="https://docs.rs/datafusion/50.2.0/src/datafusion/lib.rs.html#777" class="src">Source</a>

Expand description

re-export of [`datafusion_catalog`](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/datafusion_catalog/index.html "mod datafusion_catalog") crate

## Modules<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/index.html#modules" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/cte_worktable/index.html" class="mod" title="mod datafusion::catalog::cte_worktable">cte_worktable</a>  
CteWorkTable implementation used for recursive queries

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/default_table_source/index.html" class="mod" title="mod datafusion::catalog::default_table_source">default_table_source</a>  
Default TableSource implementation used in DataFusion physical plans

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/information_schema/index.html" class="mod" title="mod datafusion::catalog::information_schema">information_schema</a>  
[`InformationSchemaProvider`](https://docs.rs/datafusion/50.2.0/datafusion/catalog/information_schema/struct.InformationSchemaProvider.html "struct datafusion::catalog::information_schema::InformationSchemaProvider") that implements the SQL [Information Schema](https://en.wikipedia.org/wiki/Information_schema) for DataFusion.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/listing_schema/index.html" class="mod" title="mod datafusion::catalog::listing_schema">listing_schema</a>  
[`ListingSchemaProvider`](https://docs.rs/datafusion/50.2.0/datafusion/catalog/listing_schema/struct.ListingSchemaProvider.html "struct datafusion::catalog::listing_schema::ListingSchemaProvider"): [`SchemaProvider`](https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.SchemaProvider.html "trait datafusion::catalog::SchemaProvider") that scans ObjectStores for tables automatically

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/memory/index.html" class="mod" title="mod datafusion::catalog::memory">memory</a>  
<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/stream/index.html" class="mod" title="mod datafusion::catalog::stream">stream</a>  
TableProvider for stream sources, such as FIFO files

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/streaming/index.html" class="mod" title="mod datafusion::catalog::streaming">streaming</a>  
A simplified [`TableProvider`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html "trait datafusion::datasource::TableProvider") for streaming partitioned datasets

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/view/index.html" class="mod" title="mod datafusion::catalog::view">view</a>  
View data source which uses a LogicalPlan as it’s input.

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.DynamicFileCatalog.html" class="struct" title="struct datafusion::catalog::DynamicFileCatalog">DynamicFileCatalog</a>  
Wrap another catalog provider list

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.DynamicFileSchemaProvider.html" class="struct" title="struct datafusion::catalog::DynamicFileSchemaProvider">DynamicFileSchemaProvider</a>  
Implements the [DynamicFileSchemaProvider](https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.DynamicFileSchemaProvider.html "struct datafusion::catalog::DynamicFileSchemaProvider") that can create tables provider from the file path.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.MemTable.html" class="struct" title="struct datafusion::catalog::MemTable">MemTable</a>  
In-memory data source for presenting a `Vec<RecordBatch>` as a data source that can be queried by DataFusion. This allows data to be pre-loaded into memory and then repeatedly queried without incurring additional file I/O overhead.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.MemoryCatalogProvider.html" class="struct" title="struct datafusion::catalog::MemoryCatalogProvider">MemoryCatalogProvider</a>  
Simple in-memory implementation of a catalog.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.MemoryCatalogProviderList.html" class="struct" title="struct datafusion::catalog::MemoryCatalogProviderList">MemoryCatalogProviderList</a>  
Simple in-memory list of catalogs

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.MemorySchemaProvider.html" class="struct" title="struct datafusion::catalog::MemorySchemaProvider">MemorySchemaProvider</a>  
Simple in-memory implementation of a schema.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.TableFunction.html" class="struct" title="struct datafusion::catalog::TableFunction">TableFunction</a>  
A table that uses a function to generate data

## Traits<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/index.html#traits" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.AsyncCatalogProvider.html" class="trait" title="trait datafusion::catalog::AsyncCatalogProvider">AsyncCatalogProvider</a>  
A trait for catalog providers that must resolve schemas asynchronously

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.AsyncCatalogProviderList.html" class="trait" title="trait datafusion::catalog::AsyncCatalogProviderList">AsyncCatalogProviderList</a>  
A trait for catalog provider lists that must resolve catalogs asynchronously

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.AsyncSchemaProvider.html" class="trait" title="trait datafusion::catalog::AsyncSchemaProvider">AsyncSchemaProvider</a>  
A trait for schema providers that must resolve tables asynchronously

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.CatalogProvider.html" class="trait" title="trait datafusion::catalog::CatalogProvider">CatalogProvider</a>  
Represents a catalog, comprising a number of named schemas.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.CatalogProviderList.html" class="trait" title="trait datafusion::catalog::CatalogProviderList">CatalogProviderList</a>  
Represent a list of named [`CatalogProvider`](https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.CatalogProvider.html "trait datafusion::catalog::CatalogProvider")s.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.SchemaProvider.html" class="trait" title="trait datafusion::catalog::SchemaProvider">SchemaProvider</a>  
Represents a schema, comprising a number of named tables.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.Session.html" class="trait" title="trait datafusion::catalog::Session">Session</a>  
Interface for accessing [`SessionState`](https://docs.rs/datafusion/latest/datafusion/execution/session_state/struct.SessionState.html) from the catalog and data source.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.TableFunctionImpl.html" class="trait" title="trait datafusion::catalog::TableFunctionImpl">TableFunctionImpl</a>  
A trait for table function implementations

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.TableProvider.html" class="trait" title="trait datafusion::catalog::TableProvider">TableProvider</a>  
A table which can be queried and modified.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.TableProviderFactory.html" class="trait" title="trait datafusion::catalog::TableProviderFactory">TableProviderFactory</a>  
A factory which creates [`TableProvider`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html "trait datafusion::datasource::TableProvider")s at runtime given a URL.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.UrlTableFactory.html" class="trait" title="trait datafusion::catalog::UrlTableFactory">UrlTableFactory</a>  
[UrlTableFactory](https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.UrlTableFactory.html "trait datafusion::catalog::UrlTableFactory") is a factory that can create a table provider from the given url.
