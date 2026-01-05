# Trait SchemaAdapterFactory Copy item path

<a href="https://docs.rs/datafusion-datasource/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_datasource/schema_adapter.rs.html#46" class="src">Source</a>

``` rust
pub trait SchemaAdapterFactory:
    Debug
    + Send
    + Sync
    + 'static {
    // Required method
    fn create(
        &self,
        projected_table_schema: Arc<Schema>,
        table_schema: Arc<Schema>,
    ) -> Box<dyn SchemaAdapter>;

    // Provided method
    fn create_with_projected_schema(
        &self,
        projected_table_schema: Arc<Schema>,
    ) -> Box<dyn SchemaAdapter> { ... }
}
```

Expand description

Factory for creating [`SchemaAdapter`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/schema_adapter/trait.SchemaAdapter.html "trait datafusion::datasource::schema_adapter::SchemaAdapter")

This interface provides a way to implement custom schema adaptation logic for DataSourceExec (for example, to fill missing columns with default value other than null).

Most users should use [`DefaultSchemaAdapterFactory`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/schema_adapter/struct.DefaultSchemaAdapterFactory.html "struct datafusion::datasource::schema_adapter::DefaultSchemaAdapterFactory"). See that struct for more details and examples.

## Required Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/schema_adapter/trait.SchemaAdapterFactory.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/schema_adapter/trait.SchemaAdapterFactory.html#tymethod.create" class="fn">create</a>( &self, projected_table_schema: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>\>, table_schema: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/schema_adapter/trait.SchemaAdapter.html" class="trait" title="trait datafusion::datasource::schema_adapter::SchemaAdapter">SchemaAdapter</a>\>

Create a [`SchemaAdapter`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/schema_adapter/trait.SchemaAdapter.html "trait datafusion::datasource::schema_adapter::SchemaAdapter")

Arguments:

- `projected_table_schema`: The schema for the table, projected to include only the fields being output (projected) by the this mapping.

- `table_schema`: The entire table schema for the table

## Provided Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/schema_adapter/trait.SchemaAdapterFactory.html#provided-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/schema_adapter/trait.SchemaAdapterFactory.html#method.create_with_projected_schema" class="fn">create_with_projected_schema</a>( &self, projected_table_schema: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/schema_adapter/trait.SchemaAdapter.html" class="trait" title="trait datafusion::datasource::schema_adapter::SchemaAdapter">SchemaAdapter</a>\>

Create a [`SchemaAdapter`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/schema_adapter/trait.SchemaAdapter.html "trait datafusion::datasource::schema_adapter::SchemaAdapter") using only the projected table schema.

This is a convenience method for cases where the table schema and the projected table schema are the same.

## Implementors<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/schema_adapter/trait.SchemaAdapterFactory.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/schema_adapter/trait.SchemaAdapterFactory.html#impl-SchemaAdapterFactory-for-DefaultSchemaAdapterFactory" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/schema_adapter/trait.SchemaAdapterFactory.html" class="trait" title="trait datafusion::datasource::schema_adapter::SchemaAdapterFactory">SchemaAdapterFactory</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/schema_adapter/struct.DefaultSchemaAdapterFactory.html" class="struct" title="struct datafusion::datasource::schema_adapter::DefaultSchemaAdapterFactory">DefaultSchemaAdapterFactory</a>
