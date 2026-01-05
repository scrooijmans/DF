# datafusion::catalog - Rust

[datafusion](../index.html)

## Module catalog 

[Source](about:blank/src/datafusion/lib.rs.html#777)

Expand description

re-export of [`datafusion_catalog`](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/datafusion_catalog/index.html "mod datafusion_catalog") crate

## Modules[§](#modules)

[cte_worktable](cte_worktable/index.html "mod datafusion::catalog::cte_worktable")

CteWorkTable implementation used for recursive queries

[default_table_source](default_table_source/index.html "mod datafusion::catalog::default_table_source")

Default TableSource implementation used in DataFusion physical plans

[information_schema](information_schema/index.html "mod datafusion::catalog::information_schema")

[`InformationSchemaProvider`](information_schema/struct.InformationSchemaProvider.html "struct datafusion::catalog::information_schema::InformationSchemaProvider") that implements the SQL [Information Schema](https://en.wikipedia.org/wiki/Information_schema) for DataFusion.

[listing_schema](listing_schema/index.html "mod datafusion::catalog::listing_schema")

[`ListingSchemaProvider`](listing_schema/struct.ListingSchemaProvider.html "struct datafusion::catalog::listing_schema::ListingSchemaProvider"): [`SchemaProvider`](trait.SchemaProvider.html "trait datafusion::catalog::SchemaProvider") that scans ObjectStores for tables automatically

[memory](memory/index.html "mod datafusion::catalog::memory")

[stream](stream/index.html "mod datafusion::catalog::stream")

TableProvider for stream sources, such as FIFO files

[streaming](streaming/index.html "mod datafusion::catalog::streaming")

A simplified [`TableProvider`](../datasource/trait.TableProvider.html "trait datafusion::datasource::TableProvider") for streaming partitioned datasets

[view](view/index.html "mod datafusion::catalog::view")

View data source which uses a LogicalPlan as it’s input.

## Structs[§](#structs)

[DynamicFileCatalog](struct.DynamicFileCatalog.html "struct datafusion::catalog::DynamicFileCatalog")

Wrap another catalog provider list

[DynamicFileSchemaProvider](struct.DynamicFileSchemaProvider.html "struct datafusion::catalog::DynamicFileSchemaProvider")

Implements the [DynamicFileSchemaProvider](struct.DynamicFileSchemaProvider.html "struct datafusion::catalog::DynamicFileSchemaProvider") that can create tables provider from the file path.

[MemTable](struct.MemTable.html "struct datafusion::catalog::MemTable")

In-memory data source for presenting a `Vec<RecordBatch>` as a data source that can be queried by DataFusion. This allows data to be pre-loaded into memory and then repeatedly queried without incurring additional file I/O overhead.

[MemoryCatalogProvider](struct.MemoryCatalogProvider.html "struct datafusion::catalog::MemoryCatalogProvider")

Simple in-memory implementation of a catalog.

[MemoryCatalogProviderList](struct.MemoryCatalogProviderList.html "struct datafusion::catalog::MemoryCatalogProviderList")

Simple in-memory list of catalogs

[MemorySchemaProvider](struct.MemorySchemaProvider.html "struct datafusion::catalog::MemorySchemaProvider")

Simple in-memory implementation of a schema.

[TableFunction](struct.TableFunction.html "struct datafusion::catalog::TableFunction")

A table that uses a function to generate data

## Traits[§](#traits)

[AsyncCatalogProvider](trait.AsyncCatalogProvider.html "trait datafusion::catalog::AsyncCatalogProvider")

A trait for catalog providers that must resolve schemas asynchronously

[AsyncCatalogProviderList](trait.AsyncCatalogProviderList.html "trait datafusion::catalog::AsyncCatalogProviderList")

A trait for catalog provider lists that must resolve catalogs asynchronously

[AsyncSchemaProvider](trait.AsyncSchemaProvider.html "trait datafusion::catalog::AsyncSchemaProvider")

A trait for schema providers that must resolve tables asynchronously

[CatalogProvider](trait.CatalogProvider.html "trait datafusion::catalog::CatalogProvider")

Represents a catalog, comprising a number of named schemas.

[CatalogProviderList](trait.CatalogProviderList.html "trait datafusion::catalog::CatalogProviderList")

Represent a list of named [`CatalogProvider`](trait.CatalogProvider.html "trait datafusion::catalog::CatalogProvider")s.

[SchemaProvider](trait.SchemaProvider.html "trait datafusion::catalog::SchemaProvider")

Represents a schema, comprising a number of named tables.

[Session](trait.Session.html "trait datafusion::catalog::Session")

Interface for accessing [`SessionState`](https://docs.rs/datafusion/latest/datafusion/execution/session_state/struct.SessionState.html) from the catalog and data source.

[TableFunctionImpl](trait.TableFunctionImpl.html "trait datafusion::catalog::TableFunctionImpl")

A trait for table function implementations

[TableProvider](trait.TableProvider.html "trait datafusion::catalog::TableProvider")

A table which can be queried and modified.

[TableProviderFactory](trait.TableProviderFactory.html "trait datafusion::catalog::TableProviderFactory")

A factory which creates [`TableProvider`](../datasource/trait.TableProvider.html "trait datafusion::datasource::TableProvider")s at runtime given a URL.

[UrlTableFactory](trait.UrlTableFactory.html "trait datafusion::catalog::UrlTableFactory")

[UrlTableFactory](trait.UrlTableFactory.html "trait datafusion::catalog::UrlTableFactory") is a factory that can create a table provider from the given url.
