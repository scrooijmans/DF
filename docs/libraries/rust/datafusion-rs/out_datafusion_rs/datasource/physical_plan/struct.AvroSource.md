# Struct AvroSource Copy item path

<a href="https://docs.rs/datafusion-datasource-avro/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_datasource_avro/source.rs.html#39" class="src">Source</a>

``` rust
pub struct AvroSource { /* private fields */ }
```

Available on **crate feature `avro`** only.

Expand description

AvroSource holds the extra configuration that is necessary for opening avro files

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.AvroSource.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.AvroSource.html#impl-AvroSource" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.AvroSource.html" class="struct" title="struct datafusion::datasource::physical_plan::AvroSource">AvroSource</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.AvroSource.html#method.new" class="fn">new</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.AvroSource.html" class="struct" title="struct datafusion::datasource::physical_plan::AvroSource">AvroSource</a>

Initialize an AvroSource with default values

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.AvroSource.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.AvroSource.html#impl-Clone-for-AvroSource" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.AvroSource.html" class="struct" title="struct datafusion::datasource::physical_plan::AvroSource">AvroSource</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.AvroSource.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.AvroSource.html" class="struct" title="struct datafusion::datasource::physical_plan::AvroSource">AvroSource</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.AvroSource.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.AvroSource.html#impl-Default-for-AvroSource" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.AvroSource.html" class="struct" title="struct datafusion::datasource::physical_plan::AvroSource">AvroSource</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.AvroSource.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.AvroSource.html" class="struct" title="struct datafusion::datasource::physical_plan::AvroSource">AvroSource</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.AvroSource.html#impl-FileSource-for-AvroSource" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSource.html" class="trait" title="trait datafusion::datasource::physical_plan::FileSource">FileSource</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.AvroSource.html" class="struct" title="struct datafusion::datasource::physical_plan::AvroSource">AvroSource</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.AvroSource.html#method.create_file_opener" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSource.html#tymethod.create_file_opener" class="fn">create_file_opener</a>( &self, object_store: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/object_store/0.12.3/x86_64-unknown-linux-gnu/object_store/trait.ObjectStore.html" class="trait" title="trait object_store::ObjectStore">ObjectStore</a>\>, \_base_config: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.FileScanConfig.html" class="struct" title="struct datafusion::datasource::physical_plan::FileScanConfig">FileScanConfig</a>, \_partition: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileOpener.html" class="trait" title="trait datafusion::datasource::physical_plan::FileOpener">FileOpener</a>\>

Creates a `dyn FileOpener` based on given parameters

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.AvroSource.html#method.as_any" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSource.html#tymethod.as_any" class="fn">as_any</a>(&self) -\> &(dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + 'static)

Any

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.AvroSource.html#method.with_batch_size" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSource.html#tymethod.with_batch_size" class="fn">with_batch_size</a>(&self, batch_size: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSource.html" class="trait" title="trait datafusion::datasource::physical_plan::FileSource">FileSource</a>\>

Initialize new type with batch size configuration

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.AvroSource.html#method.with_schema" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSource.html#tymethod.with_schema" class="fn">with_schema</a>(&self, schema: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>\>) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSource.html" class="trait" title="trait datafusion::datasource::physical_plan::FileSource">FileSource</a>\>

Initialize new instance with a new schema

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.AvroSource.html#method.with_statistics" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSource.html#tymethod.with_statistics" class="fn">with_statistics</a>(&self, statistics: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Statistics.html" class="struct" title="struct datafusion::common::Statistics">Statistics</a>) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSource.html" class="trait" title="trait datafusion::datasource::physical_plan::FileSource">FileSource</a>\>

Initialize new instance with projected statistics

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.AvroSource.html#method.with_projection" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSource.html#tymethod.with_projection" class="fn">with_projection</a>(&self, config: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.FileScanConfig.html" class="struct" title="struct datafusion::datasource::physical_plan::FileScanConfig">FileScanConfig</a>) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSource.html" class="trait" title="trait datafusion::datasource::physical_plan::FileSource">FileSource</a>\>

Initialize new instance with projection information

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.AvroSource.html#method.metrics" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSource.html#tymethod.metrics" class="fn">metrics</a>(&self) -\> &<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.ExecutionPlanMetricsSet.html" class="struct" title="struct datafusion::physical_plan::metrics::ExecutionPlanMetricsSet">ExecutionPlanMetricsSet</a>

Return execution plan metrics

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.AvroSource.html#method.statistics" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSource.html#tymethod.statistics" class="fn">statistics</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Statistics.html" class="struct" title="struct datafusion::common::Statistics">Statistics</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Return projected statistics

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.AvroSource.html#method.file_type" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSource.html#tymethod.file_type" class="fn">file_type</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

String representation of file source such as “csv”, “json”, “parquet”

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.AvroSource.html#method.repartitioned" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSource.html#method.repartitioned" class="fn">repartitioned</a>( &self, \_target_partitions: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, \_repartition_file_min_size: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, \_output_ordering: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.LexOrdering.html" class="struct" title="struct datafusion::physical_expr::LexOrdering">LexOrdering</a>\>, \_config: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.FileScanConfig.html" class="struct" title="struct datafusion::datasource::physical_plan::FileScanConfig">FileScanConfig</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.FileScanConfig.html" class="struct" title="struct datafusion::datasource::physical_plan::FileScanConfig">FileScanConfig</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

If supported by the [`FileSource`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSource.html "trait datafusion::datasource::physical_plan::FileSource"), redistribute files across partitions according to their size. Allows custom file formats to implement their own repartitioning logic. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSource.html#method.repartitioned)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.AvroSource.html#method.with_schema_adapter_factory" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSource.html#method.with_schema_adapter_factory" class="fn">with_schema_adapter_factory</a>( &self, schema_adapter_factory: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/schema_adapter/trait.SchemaAdapterFactory.html" class="trait" title="trait datafusion::datasource::schema_adapter::SchemaAdapterFactory">SchemaAdapterFactory</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSource.html" class="trait" title="trait datafusion::datasource::physical_plan::FileSource">FileSource</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Set optional schema adapter factory. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSource.html#method.with_schema_adapter_factory)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.AvroSource.html#method.schema_adapter_factory" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSource.html#method.schema_adapter_factory" class="fn">schema_adapter_factory</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/schema_adapter/trait.SchemaAdapterFactory.html" class="trait" title="trait datafusion::datasource::schema_adapter::SchemaAdapterFactory">SchemaAdapterFactory</a>\>\>

Returns the current schema adapter factory if set [Read more](https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSource.html#method.schema_adapter_factory)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.AvroSource.html#method.filter" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSource.html#method.filter" class="fn">filter</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html" class="trait" title="trait datafusion::physical_expr::PhysicalExpr">PhysicalExpr</a>\>\>

Returns the filter expression that will be applied during the file scan.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.AvroSource.html#method.fmt_extra" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSource.html#method.fmt_extra" class="fn">fmt_extra</a>( &self, \_t: <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/enum.DisplayFormatType.html" class="enum" title="enum datafusion::physical_plan::DisplayFormatType">DisplayFormatType</a>, \_f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Format FileType specific information

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.AvroSource.html#method.try_pushdown_filters" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSource.html#method.try_pushdown_filters" class="fn">try_pushdown_filters</a>( &self, filters: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html" class="trait" title="trait datafusion::physical_expr::PhysicalExpr">PhysicalExpr</a>\>\>, \_config: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigOptions.html" class="struct" title="struct datafusion::config::ConfigOptions">ConfigOptions</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter_pushdown/struct.FilterPushdownPropagation.html" class="struct" title="struct datafusion::physical_plan::filter_pushdown::FilterPushdownPropagation">FilterPushdownPropagation</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSource.html" class="trait" title="trait datafusion::datasource::physical_plan::FileSource">FileSource</a>\>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Try to push down filters into this FileSource. See [`ExecutionPlan::handle_child_pushdown_result`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.handle_child_pushdown_result "method datafusion::physical_plan::ExecutionPlan::handle_child_pushdown_result") for more details.

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.AvroSource.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.AvroSource.html#blanket-implementations" class="anchor">§</a>
