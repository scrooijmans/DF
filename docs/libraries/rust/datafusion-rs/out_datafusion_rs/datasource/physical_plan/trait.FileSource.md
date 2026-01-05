# Trait FileSource Copy item path

<a href="https://docs.rs/datafusion-datasource/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_datasource/file.rs.html#54" class="src">Source</a>

``` rust
pub trait FileSource: Send + Sync {
Show 15 methods    // Required methods
    fn create_file_opener(
        &self,
        object_store: Arc<dyn ObjectStore>,
        base_config: &FileScanConfig,
        partition: usize,
    ) -> Arc<dyn FileOpener>;
    fn as_any(&self) -> &(dyn Any + 'static);
    fn with_batch_size(&self, batch_size: usize) -> Arc<dyn FileSource>;
    fn with_schema(&self, schema: Arc<Schema>) -> Arc<dyn FileSource>;
    fn with_projection(&self, config: &FileScanConfig) -> Arc<dyn FileSource>;
    fn with_statistics(&self, statistics: Statistics) -> Arc<dyn FileSource>;
    fn metrics(&self) -> &ExecutionPlanMetricsSet;
    fn statistics(&self) -> Result<Statistics, DataFusionError>;
    fn file_type(&self) -> &str;

    // Provided methods
    fn filter(&self) -> Option<Arc<dyn PhysicalExpr>> { ... }
    fn fmt_extra(
        &self,
        _t: DisplayFormatType,
        _f: &mut Formatter<'_>,
    ) -> Result<(), Error> { ... }
    fn repartitioned(
        &self,
        target_partitions: usize,
        repartition_file_min_size: usize,
        output_ordering: Option<LexOrdering>,
        config: &FileScanConfig,
    ) -> Result<Option<FileScanConfig>, DataFusionError> { ... }
    fn try_pushdown_filters(
        &self,
        filters: Vec<Arc<dyn PhysicalExpr>>,
        _config: &ConfigOptions,
    ) -> Result<FilterPushdownPropagation<Arc<dyn FileSource>>, DataFusionError> { ... }
    fn with_schema_adapter_factory(
        &self,
        _factory: Arc<dyn SchemaAdapterFactory>,
    ) -> Result<Arc<dyn FileSource>, DataFusionError> { ... }
    fn schema_adapter_factory(&self) -> Option<Arc<dyn SchemaAdapterFactory>> { ... }
}
```

Expand description

file format specific behaviors for elements in [`DataSource`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/source/trait.DataSource.html "trait datafusion::datasource::source::DataSource")

See more details on specific implementations:

- [`ArrowSource`](https://docs.rs/datafusion/latest/datafusion/datasource/physical_plan/struct.ArrowSource.html)
- [`AvroSource`](https://docs.rs/datafusion/latest/datafusion/datasource/physical_plan/struct.AvroSource.html)
- [`CsvSource`](https://docs.rs/datafusion/latest/datafusion/datasource/physical_plan/struct.CsvSource.html)
- [`JsonSource`](https://docs.rs/datafusion/latest/datafusion/datasource/physical_plan/struct.JsonSource.html)
- [`ParquetSource`](https://docs.rs/datafusion/latest/datafusion/datasource/physical_plan/struct.ParquetSource.html)

## Required Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSource.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSource.html#tymethod.create_file_opener" class="fn">create_file_opener</a>( &self, object_store: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/object_store/0.12.3/x86_64-unknown-linux-gnu/object_store/trait.ObjectStore.html" class="trait" title="trait object_store::ObjectStore">ObjectStore</a>\>, base_config: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.FileScanConfig.html" class="struct" title="struct datafusion::datasource::physical_plan::FileScanConfig">FileScanConfig</a>, partition: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileOpener.html" class="trait" title="trait datafusion::datasource::physical_plan::FileOpener">FileOpener</a>\>

Creates a `dyn FileOpener` based on given parameters

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSource.html#tymethod.as_any" class="fn">as_any</a>(&self) -\> &(dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + 'static)

Any

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSource.html#tymethod.with_batch_size" class="fn">with_batch_size</a>(&self, batch_size: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSource.html" class="trait" title="trait datafusion::datasource::physical_plan::FileSource">FileSource</a>\>

Initialize new type with batch size configuration

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSource.html#tymethod.with_schema" class="fn">with_schema</a>(&self, schema: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>\>) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSource.html" class="trait" title="trait datafusion::datasource::physical_plan::FileSource">FileSource</a>\>

Initialize new instance with a new schema

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSource.html#tymethod.with_projection" class="fn">with_projection</a>(&self, config: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.FileScanConfig.html" class="struct" title="struct datafusion::datasource::physical_plan::FileScanConfig">FileScanConfig</a>) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSource.html" class="trait" title="trait datafusion::datasource::physical_plan::FileSource">FileSource</a>\>

Initialize new instance with projection information

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSource.html#tymethod.with_statistics" class="fn">with_statistics</a>(&self, statistics: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Statistics.html" class="struct" title="struct datafusion::common::Statistics">Statistics</a>) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSource.html" class="trait" title="trait datafusion::datasource::physical_plan::FileSource">FileSource</a>\>

Initialize new instance with projected statistics

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSource.html#tymethod.metrics" class="fn">metrics</a>(&self) -\> &<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.ExecutionPlanMetricsSet.html" class="struct" title="struct datafusion::physical_plan::metrics::ExecutionPlanMetricsSet">ExecutionPlanMetricsSet</a>

Return execution plan metrics

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSource.html#tymethod.statistics" class="fn">statistics</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Statistics.html" class="struct" title="struct datafusion::common::Statistics">Statistics</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Return projected statistics

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSource.html#tymethod.file_type" class="fn">file_type</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

String representation of file source such as “csv”, “json”, “parquet”

## Provided Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSource.html#provided-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSource.html#method.filter" class="fn">filter</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html" class="trait" title="trait datafusion::physical_expr::PhysicalExpr">PhysicalExpr</a>\>\>

Returns the filter expression that will be applied during the file scan.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSource.html#method.fmt_extra" class="fn">fmt_extra</a>( &self, \_t: <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/enum.DisplayFormatType.html" class="enum" title="enum datafusion::physical_plan::DisplayFormatType">DisplayFormatType</a>, \_f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Format FileType specific information

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSource.html#method.repartitioned" class="fn">repartitioned</a>( &self, target_partitions: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, repartition_file_min_size: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, output_ordering: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.LexOrdering.html" class="struct" title="struct datafusion::physical_expr::LexOrdering">LexOrdering</a>\>, config: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.FileScanConfig.html" class="struct" title="struct datafusion::datasource::physical_plan::FileScanConfig">FileScanConfig</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.FileScanConfig.html" class="struct" title="struct datafusion::datasource::physical_plan::FileScanConfig">FileScanConfig</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

If supported by the [`FileSource`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSource.html "trait datafusion::datasource::physical_plan::FileSource"), redistribute files across partitions according to their size. Allows custom file formats to implement their own repartitioning logic.

The default implementation uses [`FileGroupPartitioner`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.FileGroupPartitioner.html "struct datafusion::datasource::physical_plan::FileGroupPartitioner"). See that struct for more details.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSource.html#method.try_pushdown_filters" class="fn">try_pushdown_filters</a>( &self, filters: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html" class="trait" title="trait datafusion::physical_expr::PhysicalExpr">PhysicalExpr</a>\>\>, \_config: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigOptions.html" class="struct" title="struct datafusion::config::ConfigOptions">ConfigOptions</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter_pushdown/struct.FilterPushdownPropagation.html" class="struct" title="struct datafusion::physical_plan::filter_pushdown::FilterPushdownPropagation">FilterPushdownPropagation</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSource.html" class="trait" title="trait datafusion::datasource::physical_plan::FileSource">FileSource</a>\>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Try to push down filters into this FileSource. See [`ExecutionPlan::handle_child_pushdown_result`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.handle_child_pushdown_result "method datafusion::physical_plan::ExecutionPlan::handle_child_pushdown_result") for more details.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSource.html#method.with_schema_adapter_factory" class="fn">with_schema_adapter_factory</a>( &self, \_factory: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/schema_adapter/trait.SchemaAdapterFactory.html" class="trait" title="trait datafusion::datasource::schema_adapter::SchemaAdapterFactory">SchemaAdapterFactory</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSource.html" class="trait" title="trait datafusion::datasource::physical_plan::FileSource">FileSource</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Set optional schema adapter factory.

[`SchemaAdapterFactory`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/schema_adapter/trait.SchemaAdapterFactory.html "trait datafusion::datasource::schema_adapter::SchemaAdapterFactory") allows user to specify how fields from the file get mapped to that of the table schema. If you implement this method, you should also implement [`schema_adapter_factory`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSource.html#method.schema_adapter_factory "method datafusion_datasource::file::FileSource::schema_adapter_factory::schema_adapter_factory").

The default implementation returns a not implemented error.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSource.html#method.schema_adapter_factory" class="fn">schema_adapter_factory</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/schema_adapter/trait.SchemaAdapterFactory.html" class="trait" title="trait datafusion::datasource::schema_adapter::SchemaAdapterFactory">SchemaAdapterFactory</a>\>\>

Returns the current schema adapter factory if set

Default implementation returns `None`.

## Implementors<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSource.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSource.html#impl-FileSource-for-ArrowSource" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSource.html" class="trait" title="trait datafusion::datasource::physical_plan::FileSource">FileSource</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.ArrowSource.html" class="struct" title="struct datafusion::datasource::physical_plan::ArrowSource">ArrowSource</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSource.html#impl-FileSource-for-AvroSource" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSource.html" class="trait" title="trait datafusion::datasource::physical_plan::FileSource">FileSource</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.AvroSource.html" class="struct" title="struct datafusion::datasource::physical_plan::AvroSource">AvroSource</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSource.html#impl-FileSource-for-CsvSource" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSource.html" class="trait" title="trait datafusion::datasource::physical_plan::FileSource">FileSource</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.CsvSource.html" class="struct" title="struct datafusion::datasource::physical_plan::CsvSource">CsvSource</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSource.html#impl-FileSource-for-JsonSource" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSource.html" class="trait" title="trait datafusion::datasource::physical_plan::FileSource">FileSource</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.JsonSource.html" class="struct" title="struct datafusion::datasource::physical_plan::JsonSource">JsonSource</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSource.html#impl-FileSource-for-ParquetSource" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSource.html" class="trait" title="trait datafusion::datasource::physical_plan::FileSource">FileSource</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.ParquetSource.html" class="struct" title="struct datafusion::datasource::physical_plan::ParquetSource">ParquetSource</a>
