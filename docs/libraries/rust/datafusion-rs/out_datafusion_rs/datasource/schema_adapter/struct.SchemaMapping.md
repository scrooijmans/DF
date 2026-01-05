# Struct SchemaMapping Copy item path

<a href="https://docs.rs/datafusion-datasource/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_datasource/schema_adapter.rs.html#352" class="src">Source</a>

``` rust
pub struct SchemaMapping { /* private fields */ }
```

Expand description

The SchemaMapping struct holds a mapping from the file schema to the table schema and any necessary type conversions.

[`map_batch`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/schema_adapter/struct.SchemaMapping.html#method.map_batch "method datafusion::datasource::schema_adapter::SchemaMapping::map_batch") is used by the ParquetOpener to produce a RecordBatch which has the projected schema, since that’s the schema which is supposed to come out of the execution of this query. Thus `map_batch` uses `projected_table_schema` as it can only operate on the projected fields.

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/schema_adapter/struct.SchemaMapping.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/schema_adapter/struct.SchemaMapping.html#impl-SchemaMapping" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/schema_adapter/struct.SchemaMapping.html" class="struct" title="struct datafusion::datasource::schema_adapter::SchemaMapping">SchemaMapping</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/schema_adapter/struct.SchemaMapping.html#method.new" class="fn">new</a>( projected_table_schema: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>\>, field_mappings: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>\>, cast_column: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(&<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>\>, &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html" class="struct" title="struct datafusion::common::arrow::datatypes::Field">Field</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/schema_adapter/struct.SchemaMapping.html" class="struct" title="struct datafusion::datasource::schema_adapter::SchemaMapping">SchemaMapping</a>

Creates a new SchemaMapping instance

Initializes the field mappings needed to transform file data to the projected table schema

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/schema_adapter/struct.SchemaMapping.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/schema_adapter/struct.SchemaMapping.html#impl-Debug-for-SchemaMapping" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/schema_adapter/struct.SchemaMapping.html" class="struct" title="struct datafusion::datasource::schema_adapter::SchemaMapping">SchemaMapping</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/schema_adapter/struct.SchemaMapping.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/schema_adapter/struct.SchemaMapping.html#impl-SchemaMapper-for-SchemaMapping" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/schema_adapter/trait.SchemaMapper.html" class="trait" title="trait datafusion::datasource::schema_adapter::SchemaMapper">SchemaMapper</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/schema_adapter/struct.SchemaMapping.html" class="struct" title="struct datafusion::datasource::schema_adapter::SchemaMapping">SchemaMapping</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/schema_adapter/struct.SchemaMapping.html#method.map_batch" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/schema_adapter/trait.SchemaMapper.html#tymethod.map_batch" class="fn">map_batch</a>(&self, batch: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html" class="struct" title="struct datafusion::common::arrow::array::RecordBatch">RecordBatch</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html" class="struct" title="struct datafusion::common::arrow::array::RecordBatch">RecordBatch</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Adapts a `RecordBatch` to match the `projected_table_schema` using the stored mapping and conversions. The produced RecordBatch has a schema that contains only the projected columns.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/schema_adapter/struct.SchemaMapping.html#method.map_column_statistics" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/schema_adapter/trait.SchemaMapper.html#tymethod.map_column_statistics" class="fn">map_column_statistics</a>( &self, file_col_statistics: &\[<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.ColumnStatistics.html" class="struct" title="struct datafusion::common::ColumnStatistics">ColumnStatistics</a>\], ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.ColumnStatistics.html" class="struct" title="struct datafusion::common::ColumnStatistics">ColumnStatistics</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Adapts file-level column `Statistics` to match the `table_schema`

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/schema_adapter/struct.SchemaMapping.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/schema_adapter/struct.SchemaMapping.html#blanket-implementations" class="anchor">§</a>
