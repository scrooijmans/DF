# Struct DynamicFileSchemaProvider Copy item path

<a href="https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/dynamic_file/catalog.rs.html#121" class="src">Source</a>

``` rust
pub struct DynamicFileSchemaProvider { /* private fields */ }
```

Expand description

Implements the [DynamicFileSchemaProvider](https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.DynamicFileSchemaProvider.html "struct datafusion::catalog::DynamicFileSchemaProvider") that can create tables provider from the file path.

The provider will try to create a table provider from the file path if the table provider isn’t exist in the inner schema provider.

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.DynamicFileSchemaProvider.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.DynamicFileSchemaProvider.html#impl-DynamicFileSchemaProvider" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.DynamicFileSchemaProvider.html" class="struct" title="struct datafusion::catalog::DynamicFileSchemaProvider">DynamicFileSchemaProvider</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.DynamicFileSchemaProvider.html#method.new" class="fn">new</a>( inner: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.SchemaProvider.html" class="trait" title="trait datafusion::catalog::SchemaProvider">SchemaProvider</a>\>, factory: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.UrlTableFactory.html" class="trait" title="trait datafusion::catalog::UrlTableFactory">UrlTableFactory</a>\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.DynamicFileSchemaProvider.html" class="struct" title="struct datafusion::catalog::DynamicFileSchemaProvider">DynamicFileSchemaProvider</a>

Create a new [DynamicFileSchemaProvider](https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.DynamicFileSchemaProvider.html "struct datafusion::catalog::DynamicFileSchemaProvider") with the given inner schema provider.

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.DynamicFileSchemaProvider.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.DynamicFileSchemaProvider.html#impl-Debug-for-DynamicFileSchemaProvider" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.DynamicFileSchemaProvider.html" class="struct" title="struct datafusion::catalog::DynamicFileSchemaProvider">DynamicFileSchemaProvider</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.DynamicFileSchemaProvider.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.DynamicFileSchemaProvider.html#impl-SchemaProvider-for-DynamicFileSchemaProvider" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.SchemaProvider.html" class="trait" title="trait datafusion::catalog::SchemaProvider">SchemaProvider</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.DynamicFileSchemaProvider.html" class="struct" title="struct datafusion::catalog::DynamicFileSchemaProvider">DynamicFileSchemaProvider</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.DynamicFileSchemaProvider.html#method.as_any" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.SchemaProvider.html#tymethod.as_any" class="fn">as_any</a>(&self) -\> &(dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + 'static)

Returns this `SchemaProvider` as [`Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") so that it can be downcast to a specific implementation.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.DynamicFileSchemaProvider.html#method.table_names" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.SchemaProvider.html#tymethod.table_names" class="fn">table_names</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>

Retrieves the list of available table names in this schema.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.DynamicFileSchemaProvider.html#method.table" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.SchemaProvider.html#tymethod.table" class="fn">table</a>\<'life0, 'life1, 'async_trait\>( &'life0 self, name: &'life1 <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html" class="trait" title="trait datafusion::datasource::TableProvider">TableProvider</a>\>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where 'life0: 'async_trait, 'life1: 'async_trait, <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.DynamicFileSchemaProvider.html" class="struct" title="struct datafusion::catalog::DynamicFileSchemaProvider">DynamicFileSchemaProvider</a>: 'async_trait,

Retrieves a specific table from the schema by name, if it exists, otherwise returns `None`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.DynamicFileSchemaProvider.html#method.register_table" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.SchemaProvider.html#method.register_table" class="fn">register_table</a>( &self, name: <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, table: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html" class="trait" title="trait datafusion::datasource::TableProvider">TableProvider</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html" class="trait" title="trait datafusion::datasource::TableProvider">TableProvider</a>\>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

If supported by the implementation, adds a new table named `name` to this schema. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.SchemaProvider.html#method.register_table)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.DynamicFileSchemaProvider.html#method.deregister_table" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.SchemaProvider.html#method.deregister_table" class="fn">deregister_table</a>( &self, name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html" class="trait" title="trait datafusion::datasource::TableProvider">TableProvider</a>\>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

If supported by the implementation, removes the `name` table from this schema and returns the previously registered [`TableProvider`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html "trait datafusion::datasource::TableProvider"), if any. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.SchemaProvider.html#method.deregister_table)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.DynamicFileSchemaProvider.html#method.table_exist" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.SchemaProvider.html#tymethod.table_exist" class="fn">table_exist</a>(&self, name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if table exist in the schema provider, false otherwise.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.DynamicFileSchemaProvider.html#method.owner_name" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.SchemaProvider.html#method.owner_name" class="fn">owner_name</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>

Returns the owner of the Schema, default is None. This value is reported as part of \`information_tables.schemata

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.DynamicFileSchemaProvider.html#method.table_type" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.SchemaProvider.html#method.table_type" class="fn">table_type</a>\<'life0, 'life1, 'async_trait\>( &'life0 self, name: &'life1 <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/enum.TableType.html" class="enum" title="enum datafusion::datasource::TableType">TableType</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where 'life0: 'async_trait, 'life1: 'async_trait, Self: 'async_trait,

Retrieves the type of a specific table from the schema by name, if it exists, otherwise returns `None`. Implementations for which this operation is cheap but [Self::table](https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.SchemaProvider.html#tymethod.table "method datafusion_catalog::schema::SchemaProvider::table::table") is expensive can override this to improve operations that only need the type, e.g. `SELECT * FROM information_schema.tables`.

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.DynamicFileSchemaProvider.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.DynamicFileSchemaProvider.html#blanket-implementations" class="anchor">§</a>
