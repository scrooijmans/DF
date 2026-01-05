# Trait CatalogProvider Copy item path

<a href="https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/catalog.rs.html#108" class="src">Source</a>

``` rust
pub trait CatalogProvider:
    Debug
    + Sync
    + Send {
    // Required methods
    fn as_any(&self) -> &(dyn Any + 'static);
    fn schema_names(&self) -> Vec<String>;
    fn schema(&self, name: &str) -> Option<Arc<dyn SchemaProvider>>;

    // Provided methods
    fn register_schema(
        &self,
        name: &str,
        schema: Arc<dyn SchemaProvider>,
    ) -> Result<Option<Arc<dyn SchemaProvider>>, DataFusionError> { ... }
    fn deregister_schema(
        &self,
        _name: &str,
        _cascade: bool,
    ) -> Result<Option<Arc<dyn SchemaProvider>>, DataFusionError> { ... }
}
```

Expand description

Represents a catalog, comprising a number of named schemas.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.CatalogProvider.html#catalog-overview" class="doc-anchor">§</a>Catalog Overview

To plan and execute queries, DataFusion needs a “Catalog” that provides metadata such as which schemas and tables exist, their columns and data types, and how to access the data.

The Catalog API consists:

- [`CatalogProviderList`](https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.CatalogProviderList.html "trait datafusion::catalog::CatalogProviderList"): a collection of `CatalogProvider`s
- [`CatalogProvider`](https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.CatalogProvider.html "trait datafusion::catalog::CatalogProvider"): a collection of `SchemaProvider`s (sometimes called a “database” in other systems)
- [`SchemaProvider`](https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.SchemaProvider.html "trait datafusion::catalog::SchemaProvider"): a collection of `TableProvider`s (often called a “schema” in other systems)
- [`TableProvider`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html "trait datafusion::datasource::TableProvider"): individual tables

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.CatalogProvider.html#implementing-catalogs" class="doc-anchor">§</a>Implementing Catalogs

To implement a catalog, you implement at least one of the [`CatalogProviderList`](https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.CatalogProviderList.html "trait datafusion::catalog::CatalogProviderList"), [`CatalogProvider`](https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.CatalogProvider.html "trait datafusion::catalog::CatalogProvider") and [`SchemaProvider`](https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.SchemaProvider.html "trait datafusion::catalog::SchemaProvider") traits and register them appropriately in the `SessionContext`.

DataFusion comes with a simple in-memory catalog implementation, `MemoryCatalogProvider`, that is used by default and has no persistence. DataFusion does not include more complex Catalog implementations because catalog management is a key design choice for most data systems, and thus it is unlikely that any general-purpose catalog implementation will work well across many use cases.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.CatalogProvider.html#implementing-remote-catalogs" class="doc-anchor">§</a>Implementing “Remote” catalogs

See [`remote_catalog`](https://github.com/apache/datafusion/blob/main/datafusion-examples/examples/remote_catalog.rs) for an end to end example of how to implement a remote catalog.

Sometimes catalog information is stored remotely and requires a network call to retrieve. For example, the [Delta Lake](https://delta.io/) table format stores table metadata in files on S3 that must be first downloaded to discover what schemas and tables exist.

The [`CatalogProvider`](https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.CatalogProvider.html "trait datafusion::catalog::CatalogProvider") can support this use case, but it takes some care. The planning APIs in DataFusion are not `async` and thus network IO can not be performed “lazily” / “on demand” during query planning. The rationale for this design is that using remote procedure calls for all catalog accesses required for query planning would likely result in multiple network calls per plan, resulting in very poor planning performance.

To implement [`CatalogProvider`](https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.CatalogProvider.html "trait datafusion::catalog::CatalogProvider") and [`SchemaProvider`](https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.SchemaProvider.html "trait datafusion::catalog::SchemaProvider") for remote catalogs, you need to provide an in memory snapshot of the required metadata. Most systems typically either already have this information cached locally or can batch access to the remote catalog to retrieve multiple schemas and tables in a single network call.

Note that [`SchemaProvider::table`](https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.SchemaProvider.html#tymethod.table "method datafusion::catalog::SchemaProvider::table") **is** an `async` function in order to simplify implementing simple [`SchemaProvider`](https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.SchemaProvider.html "trait datafusion::catalog::SchemaProvider")s. For many table formats it is easy to list all available tables but there is additional non trivial access required to read table details (e.g. statistics).

The pattern that DataFusion itself uses to plan SQL queries is to walk over the query to find all table references, performing required remote catalog lookups in parallel, storing the results in a cached snapshot, and then plans the query using that snapshot.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.CatalogProvider.html#example-catalog-implementations" class="doc-anchor">§</a>Example Catalog Implementations

Here are some examples of how to implement custom catalogs:

- [`datafusion-cli`](https://datafusion.apache.org/user-guide/cli/index.html): [`DynamicFileCatalogProvider`](https://github.com/apache/datafusion/blob/31b9b48b08592b7d293f46e75707aad7dadd7cbc/datafusion-cli/src/catalog.rs#L75) catalog provider that treats files and directories on a filesystem as tables.

- The [`catalog.rs`](https://github.com/apache/datafusion/blob/main/datafusion-examples/examples/catalog.rs): a simple directory based catalog.

- [delta-rs](https://github.com/delta-io/delta-rs): [`UnityCatalogProvider`](https://github.com/delta-io/delta-rs/blob/951436ecec476ce65b5ed3b58b50fb0846ca7b91/crates/deltalake-core/src/data_catalog/unity/datafusion.rs#L111-L123) implementation that can read from Delta Lake tables

## Required Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.CatalogProvider.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.CatalogProvider.html#tymethod.as_any" class="fn">as_any</a>(&self) -\> &(dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + 'static)

Returns the catalog provider as [`Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") so that it can be downcast to a specific implementation.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.CatalogProvider.html#tymethod.schema_names" class="fn">schema_names</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>

Retrieves the list of available schema names in this catalog.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.CatalogProvider.html#tymethod.schema" class="fn">schema</a>(&self, name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.SchemaProvider.html" class="trait" title="trait datafusion::catalog::SchemaProvider">SchemaProvider</a>\>\>

Retrieves a specific schema from the catalog by name, provided it exists.

## Provided Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.CatalogProvider.html#provided-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.CatalogProvider.html#method.register_schema" class="fn">register_schema</a>( &self, name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, schema: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.SchemaProvider.html" class="trait" title="trait datafusion::catalog::SchemaProvider">SchemaProvider</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.SchemaProvider.html" class="trait" title="trait datafusion::catalog::SchemaProvider">SchemaProvider</a>\>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Adds a new schema to this catalog.

If a schema of the same name existed before, it is replaced in the catalog and returned.

By default returns a “Not Implemented” error

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.CatalogProvider.html#method.deregister_schema" class="fn">deregister_schema</a>( &self, \_name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, \_cascade: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.SchemaProvider.html" class="trait" title="trait datafusion::catalog::SchemaProvider">SchemaProvider</a>\>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Removes a schema from this catalog. Implementations of this method should return errors if the schema exists but cannot be dropped. For example, in DataFusion’s default in-memory catalog, `MemoryCatalogProvider`, a non-empty schema will only be successfully dropped when `cascade` is true. This is equivalent to how DROP SCHEMA works in PostgreSQL.

Implementations of this method should return None if schema with `name` does not exist.

By default returns a “Not Implemented” error

## Implementors<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.CatalogProvider.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.CatalogProvider.html#impl-CatalogProvider-for-MemoryCatalogProvider" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.CatalogProvider.html" class="trait" title="trait datafusion::catalog::CatalogProvider">CatalogProvider</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.MemoryCatalogProvider.html" class="struct" title="struct datafusion::catalog::MemoryCatalogProvider">MemoryCatalogProvider</a>
