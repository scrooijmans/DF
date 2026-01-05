# SchemaProvider in datafusion::catalog - Rust

## Trait SchemaProvider 

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/schema.rs.html#37)

```
pub trait SchemaProvider:
    Debug
    + Sync
    + Send {
    // Required methods
    fn as_any(&self) -> &(dyn Any + 'static);
    fn table_names(&self) -> Vec<String>;
    fn table<'life0, 'life1, 'async_trait>(
        &'life0 self,
        name: &'life1 str,
    ) -> Pin<Box<dyn Future<Output = Result<Option<Arc<dyn TableProvider>>, DataFusionError>> + Send + 'async_trait>>
       where 'life0: 'async_trait,
             'life1: 'async_trait,
             Self: 'async_trait;
    fn table_exist(&self, name: &str) -> bool;

    // Provided methods
    fn owner_name(&self) -> Option<&str> { ... }
    fn table_type<'life0, 'life1, 'async_trait>(
        &'life0 self,
        name: &'life1 str,
    ) -> Pin<Box<dyn Future<Output = Result<Option<TableType>, DataFusionError>> + Send + 'async_trait>>
       where 'life0: 'async_trait,
             'life1: 'async_trait,
             Self: 'async_trait { ... }
    fn register_table(
        &self,
        name: String,
        table: Arc<dyn TableProvider>,
    ) -> Result<Option<Arc<dyn TableProvider>>, DataFusionError> { ... }
    fn deregister_table(
        &self,
        name: &str,
    ) -> Result<Option<Arc<dyn TableProvider>>, DataFusionError> { ... }
}
```

Expand description

Represents a schema, comprising a number of named tables.

Please see [`CatalogProvider`](trait.CatalogProvider.html "trait datafusion::catalog::CatalogProvider") for details of implementing a custom catalog.

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/schema.rs.html#46)

Returns this `SchemaProvider` as [`Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") so that it can be downcast to a specific implementation.

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/schema.rs.html#49)

Retrieves the list of available table names in this schema.

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/schema.rs.html#53-56)

Retrieves a specific table from the schema by name, if it exists, otherwise returns `None`.

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/schema.rs.html#90)

Returns true if table exist in the schema provider, false otherwise.

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/schema.rs.html#40)

Returns the owner of the Schema, default is None. This value is reported as part of \`information_tables.schemata

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/schema.rs.html#36)

Retrieves the type of a specific table from the schema by name, if it exists, otherwise returns `None`. Implementations for which this operation is cheap but [Self::table](about:blank/trait.SchemaProvider.html#tymethod.table "method datafusion_catalog::schema::SchemaProvider::table::table") is expensive can override this to improve operations that only need the type, e.g. `SELECT * FROM information_schema.tables`.

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/schema.rs.html#72-76)

If supported by the implementation, adds a new table named `name` to this schema.

If a table of the same name was already registered, returns “Table already exists” error.

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/schema.rs.html#85)

If supported by the implementation, removes the `name` table from this schema and returns the previously registered [`TableProvider`](../datasource/trait.TableProvider.html "trait datafusion::datasource::TableProvider"), if any.

If no `name` table exists, returns Ok(None).

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/information_schema.rs.html#487)
[§](#impl-SchemaProvider-for-InformationSchemaProvider)

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/listing_schema.rs.html#158)
[§](#impl-SchemaProvider-for-ListingSchemaProvider)

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/dynamic_file/catalog.rs.html#139)
[§](#impl-SchemaProvider-for-DynamicFileSchemaProvider)

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/memory/schema.rs.html#49)
[§](#impl-SchemaProvider-for-MemorySchemaProvider)
