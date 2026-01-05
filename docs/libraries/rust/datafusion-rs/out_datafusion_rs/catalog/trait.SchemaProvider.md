# Trait SchemaProvider Copy item path

<a href="https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/schema.rs.html#37" class="src">Source</a>

``` rust
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

Please see [`CatalogProvider`](https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.CatalogProvider.html "trait datafusion::catalog::CatalogProvider") for details of implementing a custom catalog.

## Required Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.SchemaProvider.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.SchemaProvider.html#tymethod.as_any" class="fn">as_any</a>(&self) -\> &(dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + 'static)

Returns this `SchemaProvider` as [`Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") so that it can be downcast to a specific implementation.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.SchemaProvider.html#tymethod.table_names" class="fn">table_names</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>

Retrieves the list of available table names in this schema.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.SchemaProvider.html#tymethod.table" class="fn">table</a>\<'life0, 'life1, 'async_trait\>( &'life0 self, name: &'life1 <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html" class="trait" title="trait datafusion::datasource::TableProvider">TableProvider</a>\>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where 'life0: 'async_trait, 'life1: 'async_trait, Self: 'async_trait,

Retrieves a specific table from the schema by name, if it exists, otherwise returns `None`.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.SchemaProvider.html#tymethod.table_exist" class="fn">table_exist</a>(&self, name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if table exist in the schema provider, false otherwise.

## Provided Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.SchemaProvider.html#provided-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.SchemaProvider.html#method.owner_name" class="fn">owner_name</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>

Returns the owner of the Schema, default is None. This value is reported as part of \`information_tables.schemata

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.SchemaProvider.html#method.table_type" class="fn">table_type</a>\<'life0, 'life1, 'async_trait\>( &'life0 self, name: &'life1 <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/enum.TableType.html" class="enum" title="enum datafusion::datasource::TableType">TableType</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where 'life0: 'async_trait, 'life1: 'async_trait, Self: 'async_trait,

Retrieves the type of a specific table from the schema by name, if it exists, otherwise returns `None`. Implementations for which this operation is cheap but [Self::table](https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.SchemaProvider.html#tymethod.table "method datafusion_catalog::schema::SchemaProvider::table::table") is expensive can override this to improve operations that only need the type, e.g. `SELECT * FROM information_schema.tables`.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.SchemaProvider.html#method.register_table" class="fn">register_table</a>( &self, name: <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, table: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html" class="trait" title="trait datafusion::datasource::TableProvider">TableProvider</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html" class="trait" title="trait datafusion::datasource::TableProvider">TableProvider</a>\>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

If supported by the implementation, adds a new table named `name` to this schema.

If a table of the same name was already registered, returns “Table already exists” error.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.SchemaProvider.html#method.deregister_table" class="fn">deregister_table</a>( &self, name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html" class="trait" title="trait datafusion::datasource::TableProvider">TableProvider</a>\>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

If supported by the implementation, removes the `name` table from this schema and returns the previously registered [`TableProvider`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html "trait datafusion::datasource::TableProvider"), if any.

If no `name` table exists, returns Ok(None).

## Implementors<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.SchemaProvider.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.SchemaProvider.html#impl-SchemaProvider-for-InformationSchemaProvider" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.SchemaProvider.html" class="trait" title="trait datafusion::catalog::SchemaProvider">SchemaProvider</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/information_schema/struct.InformationSchemaProvider.html" class="struct" title="struct datafusion::catalog::information_schema::InformationSchemaProvider">InformationSchemaProvider</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.SchemaProvider.html#impl-SchemaProvider-for-ListingSchemaProvider" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.SchemaProvider.html" class="trait" title="trait datafusion::catalog::SchemaProvider">SchemaProvider</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/listing_schema/struct.ListingSchemaProvider.html" class="struct" title="struct datafusion::catalog::listing_schema::ListingSchemaProvider">ListingSchemaProvider</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.SchemaProvider.html#impl-SchemaProvider-for-DynamicFileSchemaProvider" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.SchemaProvider.html" class="trait" title="trait datafusion::catalog::SchemaProvider">SchemaProvider</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.DynamicFileSchemaProvider.html" class="struct" title="struct datafusion::catalog::DynamicFileSchemaProvider">DynamicFileSchemaProvider</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.SchemaProvider.html#impl-SchemaProvider-for-MemorySchemaProvider" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.SchemaProvider.html" class="trait" title="trait datafusion::catalog::SchemaProvider">SchemaProvider</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.MemorySchemaProvider.html" class="struct" title="struct datafusion::catalog::MemorySchemaProvider">MemorySchemaProvider</a>
