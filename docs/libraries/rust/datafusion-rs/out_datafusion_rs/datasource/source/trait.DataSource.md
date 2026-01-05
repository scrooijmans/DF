# Trait DataSource Copy item path

<a href="https://docs.rs/datafusion-datasource/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_datasource/source.rs.html#120" class="src">Source</a>

``` rust
pub trait DataSource:
    Send
    + Sync
    + Debug {
Show 13 methods    // Required methods
    fn open(
        &self,
        partition: usize,
        context: Arc<TaskContext>,
    ) -> Result<Pin<Box<dyn RecordBatchStream<Item = Result<RecordBatch, DataFusionError>> + Send>>, DataFusionError>;
    fn as_any(&self) -> &(dyn Any + 'static);
    fn fmt_as(
        &self,
        t: DisplayFormatType,
        f: &mut Formatter<'_>,
    ) -> Result<(), Error>;
    fn output_partitioning(&self) -> Partitioning;
    fn eq_properties(&self) -> EquivalenceProperties;
    fn statistics(&self) -> Result<Statistics, DataFusionError>;
    fn with_fetch(&self, _limit: Option<usize>) -> Option<Arc<dyn DataSource>>;
    fn fetch(&self) -> Option<usize>;
    fn try_swapping_with_projection(
        &self,
        _projection: &[ProjectionExpr],
    ) -> Result<Option<Arc<dyn DataSource>>, DataFusionError>;

    // Provided methods
    fn repartitioned(
        &self,
        _target_partitions: usize,
        _repartition_file_min_size: usize,
        _output_ordering: Option<LexOrdering>,
    ) -> Result<Option<Arc<dyn DataSource>>, DataFusionError> { ... }
    fn scheduling_type(&self) -> SchedulingType { ... }
    fn metrics(&self) -> ExecutionPlanMetricsSet { ... }
    fn try_pushdown_filters(
        &self,
        filters: Vec<Arc<dyn PhysicalExpr>>,
        _config: &ConfigOptions,
    ) -> Result<FilterPushdownPropagation<Arc<dyn DataSource>>, DataFusionError> { ... }
}
```

Expand description

A source of data, typically a list of files or memory

This trait provides common behaviors for abstract sources of data. It has two common implementations:

1.  [`FileScanConfig`](https://docs.rs/datafusion/latest/datafusion/datasource/physical_plan/struct.FileScanConfig.html): lists of files
2.  [`MemorySourceConfig`](https://docs.rs/datafusion/latest/datafusion/datasource/memory/struct.MemorySourceConfig.html): in memory list of `RecordBatch`

File format specific behaviors are defined by [`FileSource`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSource.html "trait datafusion::datasource::physical_plan::FileSource")

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/source/trait.DataSource.html#see-also" class="doc-anchor">§</a>See Also

- [`FileSource`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSource.html "trait datafusion::datasource::physical_plan::FileSource") for file format specific implementations (Parquet, Json, etc)
- [`DataSourceExec`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/memory/struct.DataSourceExec.html "struct datafusion::datasource::memory::DataSourceExec"): The [`ExecutionPlan`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html "trait datafusion::physical_plan::ExecutionPlan") that reads from a `DataSource`

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/source/trait.DataSource.html#notes" class="doc-anchor">§</a>Notes

Requires `Debug` to assist debugging

The following diagram shows how DataSource, FileSource, and DataSourceExec are related

``` text
                      ┌─────────────────────┐                              -----► execute path
                      │                     │                              ┄┄┄┄┄► init path
                      │   DataSourceExec    │  
                      │                     │    
                      └───────▲─────────────┘
                              ┊  │
                              ┊  │
                      ┌──────────▼──────────┐                            ┌──────────-──────────┐
                      │                     │                            |                     |
                      │  DataSource(trait)  │                            | TableProvider(trait)|
                      │                     │                            |                     |
                      └───────▲─────────────┘                            └─────────────────────┘
                              ┊  │                                                  ┊
              ┌───────────────┿──┴────────────────┐                                 ┊
              |   ┌┄┄┄┄┄┄┄┄┄┄┄┘                   |                                 ┊
              |   ┊                               |                                 ┊
   ┌──────────▼──────────┐             ┌──────────▼──────────┐                      ┊
   │                     │             │                     │           ┌──────────▼──────────┐
   │   FileScanConfig    │             │ MemorySourceConfig  │           |                     |
   │                     │             │                     │           |  FileFormat(trait)  |
   └──────────────▲──────┘             └─────────────────────┘           |                     |
              │   ┊                                                      └─────────────────────┘
              │   ┊                                                                 ┊
              │   ┊                                                                 ┊
   ┌──────────▼──────────┐                                               ┌──────────▼──────────┐
   │                     │                                               │     ArrowSource     │
   │ FileSource(trait)   ◄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄│          ...        │
   │                     │                                               │    ParquetSource    │
   └─────────────────────┘                                               └─────────────────────┘
              │
              │
              │
              │
   ┌──────────▼──────────┐
   │     ArrowSource     │
   │          ...        │
   │    ParquetSource    │
   └─────────────────────┘
              |
FileOpener (called by FileStream)
              │
   ┌──────────▼──────────┐
   │                     │
   │     RecordBatch     │
   │                     │
   └─────────────────────┘
```

## Required Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/source/trait.DataSource.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/source/trait.DataSource.html#tymethod.open" class="fn">open</a>( &self, partition: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, context: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/struct.TaskContext.html" class="struct" title="struct datafusion::execution::TaskContext">TaskContext</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.RecordBatchStream.html" class="trait" title="trait datafusion::execution::RecordBatchStream">RecordBatchStream</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html" class="struct" title="struct datafusion::common::arrow::array::RecordBatch">RecordBatch</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>\>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/source/trait.DataSource.html#tymethod.as_any" class="fn">as_any</a>(&self) -\> &(dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + 'static)

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/source/trait.DataSource.html#tymethod.fmt_as" class="fn">fmt_as</a>( &self, t: <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/enum.DisplayFormatType.html" class="enum" title="enum datafusion::physical_plan::DisplayFormatType">DisplayFormatType</a>, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Format this source for display in explain plans

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/source/trait.DataSource.html#tymethod.output_partitioning" class="fn">output_partitioning</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.Partitioning.html" class="enum" title="enum datafusion::physical_expr::Partitioning">Partitioning</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/source/trait.DataSource.html#tymethod.eq_properties" class="fn">eq_properties</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.EquivalenceProperties.html" class="struct" title="struct datafusion::physical_expr::EquivalenceProperties">EquivalenceProperties</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/source/trait.DataSource.html#tymethod.statistics" class="fn">statistics</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Statistics.html" class="struct" title="struct datafusion::common::Statistics">Statistics</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/source/trait.DataSource.html#tymethod.with_fetch" class="fn">with_fetch</a>(&self, \_limit: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/source/trait.DataSource.html" class="trait" title="trait datafusion::datasource::source::DataSource">DataSource</a>\>\>

Return a copy of this DataSource with a new fetch limit

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/source/trait.DataSource.html#tymethod.fetch" class="fn">fetch</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/source/trait.DataSource.html#tymethod.try_swapping_with_projection" class="fn">try_swapping_with_projection</a>( &self, \_projection: &\[<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/projection/struct.ProjectionExpr.html" class="struct" title="struct datafusion::physical_plan::projection::ProjectionExpr">ProjectionExpr</a>\], ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/source/trait.DataSource.html" class="trait" title="trait datafusion::datasource::source::DataSource">DataSource</a>\>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

## Provided Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/source/trait.DataSource.html#provided-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/source/trait.DataSource.html#method.repartitioned" class="fn">repartitioned</a>( &self, \_target_partitions: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, \_repartition_file_min_size: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, \_output_ordering: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.LexOrdering.html" class="struct" title="struct datafusion::physical_expr::LexOrdering">LexOrdering</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/source/trait.DataSource.html" class="trait" title="trait datafusion::datasource::source::DataSource">DataSource</a>\>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Return a copy of this DataSource with a new partitioning scheme.

Returns `Ok(None)` (the default) if the partitioning cannot be changed. Refer to [`ExecutionPlan::repartitioned`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.repartitioned "method datafusion::physical_plan::ExecutionPlan::repartitioned") for details on when None should be returned.

Repartitioning should not change the output ordering, if this ordering exists. Refer to [`MemorySourceConfig::repartition_preserving_order`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/memory/struct.MemorySourceConfig.html "struct datafusion::datasource::memory::MemorySourceConfig") and the FileSource’s [`FileGroupPartitioner::repartition_file_groups`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.FileGroupPartitioner.html#method.repartition_file_groups "method datafusion::datasource::physical_plan::FileGroupPartitioner::repartition_file_groups") for examples.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/source/trait.DataSource.html#method.scheduling_type" class="fn">scheduling_type</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/enum.SchedulingType.html" class="enum" title="enum datafusion::physical_plan::execution_plan::SchedulingType">SchedulingType</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/source/trait.DataSource.html#method.metrics" class="fn">metrics</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.ExecutionPlanMetricsSet.html" class="struct" title="struct datafusion::physical_plan::metrics::ExecutionPlanMetricsSet">ExecutionPlanMetricsSet</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/source/trait.DataSource.html#method.try_pushdown_filters" class="fn">try_pushdown_filters</a>( &self, filters: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html" class="trait" title="trait datafusion::physical_expr::PhysicalExpr">PhysicalExpr</a>\>\>, \_config: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigOptions.html" class="struct" title="struct datafusion::config::ConfigOptions">ConfigOptions</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter_pushdown/struct.FilterPushdownPropagation.html" class="struct" title="struct datafusion::physical_plan::filter_pushdown::FilterPushdownPropagation">FilterPushdownPropagation</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/source/trait.DataSource.html" class="trait" title="trait datafusion::datasource::source::DataSource">DataSource</a>\>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Try to push down filters into this DataSource. See [`ExecutionPlan::handle_child_pushdown_result`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.handle_child_pushdown_result "method datafusion::physical_plan::ExecutionPlan::handle_child_pushdown_result") for more details.

## Implementors<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/source/trait.DataSource.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/source/trait.DataSource.html#impl-DataSource-for-MemorySourceConfig" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/source/trait.DataSource.html" class="trait" title="trait datafusion::datasource::source::DataSource">DataSource</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/memory/struct.MemorySourceConfig.html" class="struct" title="struct datafusion::datasource::memory::MemorySourceConfig">MemorySourceConfig</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/source/trait.DataSource.html#impl-DataSource-for-FileScanConfig" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/source/trait.DataSource.html" class="trait" title="trait datafusion::datasource::source::DataSource">DataSource</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.FileScanConfig.html" class="struct" title="struct datafusion::datasource::physical_plan::FileScanConfig">FileScanConfig</a>
