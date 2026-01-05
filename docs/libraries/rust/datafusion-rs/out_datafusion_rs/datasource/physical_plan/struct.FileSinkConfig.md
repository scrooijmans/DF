# Struct FileSinkConfig Copy item path

<a href="https://docs.rs/datafusion-datasource/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_datasource/file_sink_config.rs.html#93" class="src">Source</a>

``` rust
pub struct FileSinkConfig {
    pub original_url: String,
    pub object_store_url: ObjectStoreUrl,
    pub file_group: FileGroup,
    pub table_paths: Vec<ListingTableUrl>,
    pub output_schema: Arc<Schema>,
    pub table_partition_cols: Vec<(String, DataType)>,
    pub insert_op: InsertOp,
    pub keep_partition_by_columns: bool,
    pub file_extension: String,
}
```

Expand description

The base configurations to provide when creating a physical plan for writing to any given file format.

## Fields<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.FileSinkConfig.html#fields" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.FileSinkConfig.html#structfield.original_url" class="anchor field">§</a>`original_url: `<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>

The unresolved URL specified by the user

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.FileSinkConfig.html#structfield.object_store_url" class="anchor field">§</a>`object_store_url: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/object_store/struct.ObjectStoreUrl.html" class="struct" title="struct datafusion::datasource::object_store::ObjectStoreUrl"><code>ObjectStoreUrl</code></a>

Object store URL, used to get an ObjectStore instance

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.FileSinkConfig.html#structfield.file_group" class="anchor field">§</a>`file_group: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.FileGroup.html" class="struct" title="struct datafusion::datasource::physical_plan::FileGroup"><code>FileGroup</code></a>

A collection of files organized into groups. Each FileGroup contains one or more PartitionedFile objects.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.FileSinkConfig.html#structfield.table_paths" class="anchor field">§</a>`table_paths: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableUrl.html" class="struct" title="struct datafusion::datasource::listing::ListingTableUrl"><code>ListingTableUrl</code></a>`>`

Vector of partition paths

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.FileSinkConfig.html#structfield.output_schema" class="anchor field">§</a>`output_schema: `<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc"><code>Arc</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema"><code>Schema</code></a>`>`

The schema of the output file

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.FileSinkConfig.html#structfield.table_partition_cols" class="anchor field">§</a>`table_partition_cols: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<(`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`, `<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType"><code>DataType</code></a>`)>`

A vector of column names and their corresponding data types, representing the partitioning columns for the file

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.FileSinkConfig.html#structfield.insert_op" class="anchor field">§</a>`insert_op: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/dml/enum.InsertOp.html" class="enum" title="enum datafusion::logical_expr::logical_plan::dml::InsertOp"><code>InsertOp</code></a>

Controls how new data should be written to the file, determining whether to append to, overwrite, or replace records in existing files.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.FileSinkConfig.html#structfield.keep_partition_by_columns" class="anchor field">§</a>`keep_partition_by_columns: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Controls whether partition columns are kept for the file

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.FileSinkConfig.html#structfield.file_extension" class="anchor field">§</a>`file_extension: `<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>

File extension without a dot(.)

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.FileSinkConfig.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.FileSinkConfig.html#impl-FileSinkConfig" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.FileSinkConfig.html" class="struct" title="struct datafusion::datasource::physical_plan::FileSinkConfig">FileSinkConfig</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.FileSinkConfig.html#method.output_schema" class="fn">output_schema</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>\>

Get output schema

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.FileSinkConfig.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.FileSinkConfig.html#impl-Clone-for-FileSinkConfig" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.FileSinkConfig.html" class="struct" title="struct datafusion::datasource::physical_plan::FileSinkConfig">FileSinkConfig</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.FileSinkConfig.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.FileSinkConfig.html" class="struct" title="struct datafusion::datasource::physical_plan::FileSinkConfig">FileSinkConfig</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.FileSinkConfig.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.FileSinkConfig.html#impl-Debug-for-FileSinkConfig" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.FileSinkConfig.html" class="struct" title="struct datafusion::datasource::physical_plan::FileSinkConfig">FileSinkConfig</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.FileSinkConfig.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.FileSinkConfig.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.FileSinkConfig.html#blanket-implementations" class="anchor">§</a>
