# Struct BatchPartitioner Copy item path

<a href="https://docs.rs/datafusion-physical-plan/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_plan/repartition/mod.rs.html#254" class="src">Source</a>

``` rust
pub struct BatchPartitioner { /* private fields */ }
```

Expand description

A utility that can be used to partition batches based on [`Partitioning`](https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.Partitioning.html "enum datafusion::physical_expr::Partitioning")

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/repartition/struct.BatchPartitioner.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/repartition/struct.BatchPartitioner.html#impl-BatchPartitioner" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/repartition/struct.BatchPartitioner.html" class="struct" title="struct datafusion::physical_plan::repartition::BatchPartitioner">BatchPartitioner</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/repartition/struct.BatchPartitioner.html#method.try_new" class="fn">try_new</a>( partitioning: <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.Partitioning.html" class="enum" title="enum datafusion::physical_expr::Partitioning">Partitioning</a>, timer: <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.Time.html" class="struct" title="struct datafusion::physical_plan::metrics::Time">Time</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/repartition/struct.BatchPartitioner.html" class="struct" title="struct datafusion::physical_plan::repartition::BatchPartitioner">BatchPartitioner</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Create a new [`BatchPartitioner`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/repartition/struct.BatchPartitioner.html "struct datafusion::physical_plan::repartition::BatchPartitioner") with the provided [`Partitioning`](https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.Partitioning.html "enum datafusion::physical_expr::Partitioning")

The time spent repartitioning will be recorded to `timer`

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/repartition/struct.BatchPartitioner.html#method.partition" class="fn">partition</a>\<F\>( &mut self, batch: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html" class="struct" title="struct datafusion::common::arrow::array::RecordBatch">RecordBatch</a>, f: F, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html" class="struct" title="struct datafusion::common::arrow::array::RecordBatch">RecordBatch</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>,

Partition the provided [`RecordBatch`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html "struct datafusion::common::arrow::array::RecordBatch") into one or more partitioned [`RecordBatch`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html "struct datafusion::common::arrow::array::RecordBatch") based on the [`Partitioning`](https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.Partitioning.html "enum datafusion::physical_expr::Partitioning") specified on construction

`f` will be called for each partitioned [`RecordBatch`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html "struct datafusion::common::arrow::array::RecordBatch") with the corresponding partition index. Any error returned by `f` will be immediately returned by this function without attempting to publish further [`RecordBatch`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html "struct datafusion::common::arrow::array::RecordBatch")

The time spent repartitioning, not including time spent in `f` will be recorded to the [`metrics::Time`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.Time.html "struct datafusion::physical_plan::metrics::Time") provided on construction

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/repartition/struct.BatchPartitioner.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/repartition/struct.BatchPartitioner.html#blanket-implementations" class="anchor">§</a>
