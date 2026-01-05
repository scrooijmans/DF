# Struct ListingSchemaProvider Copy item path

<a href="https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/listing_schema.rs.html#52" class="src">Source</a>

``` rust
pub struct ListingSchemaProvider { /* private fields */ }
```

Expand description

A [`SchemaProvider`](https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.SchemaProvider.html "trait datafusion::catalog::SchemaProvider") that scans an [`ObjectStore`](https://docs.rs/object_store/0.12.3/x86_64-unknown-linux-gnu/object_store/trait.ObjectStore.html "trait object_store::ObjectStore") to automatically discover tables

A subfolder relationship is assumed, i.e. given:

- authority = `s3://host.example.com:3000`
- path = `/data/tpch`
- factory = `DeltaTableFactory`

A table called “customer” will be registered for the folder: `s3://host.example.com:3000/data/tpch/customer`

assuming it contains valid deltalake data, i.e:

- `s3://host.example.com:3000/data/tpch/customer/part-00000-xxxx.snappy.parquet`
- `s3://host.example.com:3000/data/tpch/customer/_delta_log/`

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/listing_schema/struct.ListingSchemaProvider.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/listing_schema/struct.ListingSchemaProvider.html#impl-ListingSchemaProvider" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/listing_schema/struct.ListingSchemaProvider.html" class="struct" title="struct datafusion::catalog::listing_schema::ListingSchemaProvider">ListingSchemaProvider</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/listing_schema/struct.ListingSchemaProvider.html#method.new" class="fn">new</a>( authority: <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, path: <a href="https://docs.rs/object_store/0.12.3/x86_64-unknown-linux-gnu/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>, factory: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.TableProviderFactory.html" class="trait" title="trait datafusion::catalog::TableProviderFactory">TableProviderFactory</a>\>, store: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/object_store/0.12.3/x86_64-unknown-linux-gnu/object_store/trait.ObjectStore.html" class="trait" title="trait object_store::ObjectStore">ObjectStore</a>\>, format: <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/listing_schema/struct.ListingSchemaProvider.html" class="struct" title="struct datafusion::catalog::listing_schema::ListingSchemaProvider">ListingSchemaProvider</a>

Create a new `ListingSchemaProvider`

Arguments: `authority`: The scheme (i.e. s3://) + host (i.e. example.com:3000) `path`: The root path that contains subfolders which represent tables `factory`: The `TableProviderFactory` to use to instantiate tables for each subfolder `store`: The `ObjectStore` containing the table data `format`: The `FileFormat` of the tables `has_header`: Indicates whether the created external table has the has_header flag enabled

#### pub async fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/listing_schema/struct.ListingSchemaProvider.html#method.refresh" class="fn">refresh</a>(&self, state: &dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.Session.html" class="trait" title="trait datafusion::catalog::Session">Session</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Reload table information from ObjectStore

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/listing_schema/struct.ListingSchemaProvider.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/listing_schema/struct.ListingSchemaProvider.html#impl-Debug-for-ListingSchemaProvider" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/listing_schema/struct.ListingSchemaProvider.html" class="struct" title="struct datafusion::catalog::listing_schema::ListingSchemaProvider">ListingSchemaProvider</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/listing_schema/struct.ListingSchemaProvider.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/listing_schema/struct.ListingSchemaProvider.html#impl-SchemaProvider-for-ListingSchemaProvider" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.SchemaProvider.html" class="trait" title="trait datafusion::catalog::SchemaProvider">SchemaProvider</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/listing_schema/struct.ListingSchemaProvider.html" class="struct" title="struct datafusion::catalog::listing_schema::ListingSchemaProvider">ListingSchemaProvider</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/listing_schema/struct.ListingSchemaProvider.html#method.as_any" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.SchemaProvider.html#tymethod.as_any" class="fn">as_any</a>(&self) -\> &(dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + 'static)

Returns this `SchemaProvider` as [`Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") so that it can be downcast to a specific implementation.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/listing_schema/struct.ListingSchemaProvider.html#method.table_names" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.SchemaProvider.html#tymethod.table_names" class="fn">table_names</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>

Retrieves the list of available table names in this schema.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/listing_schema/struct.ListingSchemaProvider.html#method.table" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.SchemaProvider.html#tymethod.table" class="fn">table</a>\<'life0, 'life1, 'async_trait\>( &'life0 self, name: &'life1 <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html" class="trait" title="trait datafusion::datasource::TableProvider">TableProvider</a>\>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where 'life0: 'async_trait, 'life1: 'async_trait, <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/listing_schema/struct.ListingSchemaProvider.html" class="struct" title="struct datafusion::catalog::listing_schema::ListingSchemaProvider">ListingSchemaProvider</a>: 'async_trait,

Retrieves a specific table from the schema by name, if it exists, otherwise returns `None`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/listing_schema/struct.ListingSchemaProvider.html#method.register_table" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.SchemaProvider.html#method.register_table" class="fn">register_table</a>( &self, name: <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, table: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html" class="trait" title="trait datafusion::datasource::TableProvider">TableProvider</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html" class="trait" title="trait datafusion::datasource::TableProvider">TableProvider</a>\>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

If supported by the implementation, adds a new table named `name` to this schema. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.SchemaProvider.html#method.register_table)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/listing_schema/struct.ListingSchemaProvider.html#method.deregister_table" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.SchemaProvider.html#method.deregister_table" class="fn">deregister_table</a>( &self, name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html" class="trait" title="trait datafusion::datasource::TableProvider">TableProvider</a>\>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

If supported by the implementation, removes the `name` table from this schema and returns the previously registered [`TableProvider`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html "trait datafusion::datasource::TableProvider"), if any. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.SchemaProvider.html#method.deregister_table)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/listing_schema/struct.ListingSchemaProvider.html#method.table_exist" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.SchemaProvider.html#tymethod.table_exist" class="fn">table_exist</a>(&self, name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if table exist in the schema provider, false otherwise.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/listing_schema/struct.ListingSchemaProvider.html#method.owner_name" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.SchemaProvider.html#method.owner_name" class="fn">owner_name</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>

Returns the owner of the Schema, default is None. This value is reported as part of \`information_tables.schemata

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/listing_schema/struct.ListingSchemaProvider.html#method.table_type" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.SchemaProvider.html#method.table_type" class="fn">table_type</a>\<'life0, 'life1, 'async_trait\>( &'life0 self, name: &'life1 <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/enum.TableType.html" class="enum" title="enum datafusion::datasource::TableType">TableType</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where 'life0: 'async_trait, 'life1: 'async_trait, Self: 'async_trait,

Retrieves the type of a specific table from the schema by name, if it exists, otherwise returns `None`. Implementations for which this operation is cheap but [Self::table](https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.SchemaProvider.html#tymethod.table "method datafusion_catalog::schema::SchemaProvider::table::table") is expensive can override this to improve operations that only need the type, e.g. `SELECT * FROM information_schema.tables`.

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/listing_schema/struct.ListingSchemaProvider.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/listing_schema/struct.ListingSchemaProvider.html#blanket-implementations" class="anchor">§</a>
