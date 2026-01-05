# Trait SchemaAdapter Copy item path

<a href="https://docs.rs/datafusion-datasource/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_datasource/schema_adapter.rs.html#80" class="src">Source</a>

``` rust
pub trait SchemaAdapter: Send + Sync {
    // Required methods
    fn map_column_index(
        &self,
        index: usize,
        file_schema: &Schema,
    ) -> Option<usize>;
    fn map_schema(
        &self,
        file_schema: &Schema,
    ) -> Result<(Arc<dyn SchemaMapper>, Vec<usize>), DataFusionError>;
}
```

Expand description

Creates [`SchemaMapper`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/schema_adapter/trait.SchemaMapper.html "trait datafusion::datasource::schema_adapter::SchemaMapper")s to map file-level [`RecordBatch`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html "struct datafusion::common::arrow::array::RecordBatch")es to a table schema, which may have a schema obtained from merging multiple file-level schemas.

This is useful for implementing schema evolution in partitioned datasets.

See [`DefaultSchemaAdapterFactory`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/schema_adapter/struct.DefaultSchemaAdapterFactory.html "struct datafusion::datasource::schema_adapter::DefaultSchemaAdapterFactory") for more details and examples.

## Required Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/schema_adapter/trait.SchemaAdapter.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/schema_adapter/trait.SchemaAdapter.html#tymethod.map_column_index" class="fn">map_column_index</a>(&self, index: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, file_schema: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>

Map a column index in the table schema to a column index in a particular file schema

This is used while reading a file to push down projections by mapping projected column indexes from the table schema to the file schema

Panics if index is not in range for the table schema

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/schema_adapter/trait.SchemaAdapter.html#tymethod.map_schema" class="fn">map_schema</a>( &self, file_schema: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<(<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/schema_adapter/trait.SchemaMapper.html" class="trait" title="trait datafusion::datasource::schema_adapter::SchemaMapper">SchemaMapper</a>\>, <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>), <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Creates a mapping for casting columns from the file schema to the table schema.

This is used after reading a record batch. The returned [`SchemaMapper`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/schema_adapter/trait.SchemaMapper.html "trait datafusion::datasource::schema_adapter::SchemaMapper"):

1.  Maps columns to the expected columns indexes
2.  Handles missing values (e.g. fills nulls or a default value) for columns in the in the table schema not in the file schema
3.  Handles different types: if the column in the file schema has a different type than `table_schema`, the mapper will resolve this difference (e.g. by casting to the appropriate type)

Returns:

- a [`SchemaMapper`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/schema_adapter/trait.SchemaMapper.html "trait datafusion::datasource::schema_adapter::SchemaMapper")
- an ordered list of columns to project from the file

## Implementors<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/schema_adapter/trait.SchemaAdapter.html#implementors" class="anchor">§</a>
