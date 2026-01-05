# Module schema_adapter Copy item path

<a href="https://docs.rs/datafusion-datasource/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_datasource/mod.rs.html#41" class="src">Source</a>

Expand description

[`SchemaAdapter`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/schema_adapter/trait.SchemaAdapter.html "trait datafusion::datasource::schema_adapter::SchemaAdapter") and [`SchemaAdapterFactory`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/schema_adapter/trait.SchemaAdapterFactory.html "trait datafusion::datasource::schema_adapter::SchemaAdapterFactory") to adapt file-level record batches to a table schema.

Adapter provides a method of translating the RecordBatches that come out of the physical format into how they should be used by DataFusion. For instance, a schema can be stored external to a parquet file that maps parquet logical types to arrow types.

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/schema_adapter/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/schema_adapter/struct.DefaultSchemaAdapterFactory.html" class="struct" title="struct datafusion::datasource::schema_adapter::DefaultSchemaAdapterFactory">DefaultSchemaAdapterFactory</a>  
Default [`SchemaAdapterFactory`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/schema_adapter/trait.SchemaAdapterFactory.html "trait datafusion::datasource::schema_adapter::SchemaAdapterFactory") for mapping schemas.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/schema_adapter/struct.SchemaMapping.html" class="struct" title="struct datafusion::datasource::schema_adapter::SchemaMapping">SchemaMapping</a>  
The SchemaMapping struct holds a mapping from the file schema to the table schema and any necessary type conversions.

## Traits<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/schema_adapter/index.html#traits" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/schema_adapter/trait.SchemaAdapter.html" class="trait" title="trait datafusion::datasource::schema_adapter::SchemaAdapter">SchemaAdapter</a>  
Creates [`SchemaMapper`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/schema_adapter/trait.SchemaMapper.html "trait datafusion::datasource::schema_adapter::SchemaMapper")s to map file-level [`RecordBatch`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html "struct datafusion::common::arrow::array::RecordBatch")es to a table schema, which may have a schema obtained from merging multiple file-level schemas.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/schema_adapter/trait.SchemaAdapterFactory.html" class="trait" title="trait datafusion::datasource::schema_adapter::SchemaAdapterFactory">SchemaAdapterFactory</a>  
Factory for creating [`SchemaAdapter`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/schema_adapter/trait.SchemaAdapter.html "trait datafusion::datasource::schema_adapter::SchemaAdapter")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/schema_adapter/trait.SchemaMapper.html" class="trait" title="trait datafusion::datasource::schema_adapter::SchemaMapper">SchemaMapper</a>  
Maps, columns from a specific file schema to the table schema.

## Type Aliases<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/schema_adapter/index.html#types" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/schema_adapter/type.CastColumnFn.html" class="type" title="type datafusion::datasource::schema_adapter::CastColumnFn">CastColumnFn</a>  
Function used by [`SchemaMapping`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/schema_adapter/struct.SchemaMapping.html "struct datafusion::datasource::schema_adapter::SchemaMapping") to adapt a column from the file schema to the table schema.
