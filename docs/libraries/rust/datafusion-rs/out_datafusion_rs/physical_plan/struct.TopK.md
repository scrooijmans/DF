# Struct TopK Copy item path

<a href="https://docs.rs/datafusion-physical-plan/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_plan/topk/mod.rs.html#103" class="src">Source</a>

``` rust
pub struct TopK { /* private fields */ }
```

Expand description

Global TopK

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.TopK.html#background" class="doc-anchor">§</a>Background

“Top K” is a common query optimization used for queries such as “find the top 3 customers by revenue”. The (simplified) SQL for such a query might be:

``` sql
SELECT customer_id, revenue FROM 'sales.csv' ORDER BY revenue DESC limit 3;
```

The simple plan would be:

``` sql
> explain SELECT customer_id, revenue FROM sales ORDER BY revenue DESC limit 3;
+--------------+----------------------------------------+
| plan_type    | plan                                   |
+--------------+----------------------------------------+
| logical_plan | Limit: 3                               |
|              |   Sort: revenue DESC NULLS FIRST       |
|              |     Projection: customer_id, revenue   |
|              |       TableScan: sales                 |
+--------------+----------------------------------------+
```

While this plan produces the correct answer, it will fully sorts the input before discarding everything other than the top 3 elements.

The same answer can be produced by simply keeping track of the top K=3 elements, reducing the total amount of required buffer memory.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.TopK.html#partial-sort-optimization" class="doc-anchor">§</a>Partial Sort Optimization

This implementation additionally optimizes queries where the input is already partially sorted by a common prefix of the requested ordering. Once the top K heap is full, if subsequent rows are guaranteed to be strictly greater (in sort order) on this prefix than the largest row currently stored, the operator safely terminates early.

### <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.TopK.html#example" class="doc-anchor">§</a>Example

For input sorted by `(day DESC)`, but not by `timestamp`, a query such as:

``` sql
SELECT day, timestamp FROM sensor ORDER BY day DESC, timestamp DESC LIMIT 10;
```

can terminate scanning early once sufficient rows from the latest days have been collected, skipping older data.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.TopK.html#structure" class="doc-anchor">§</a>Structure

This operator tracks the top K items using a `TopKHeap`.

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.TopK.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.TopK.html#impl-TopK" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.TopK.html" class="struct" title="struct datafusion::physical_plan::TopK">TopK</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.TopK.html#method.try_new" class="fn">try_new</a>( partition_id: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, schema: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>\>, common_sort_prefix: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.PhysicalSortExpr.html" class="struct" title="struct datafusion::physical_expr::PhysicalSortExpr">PhysicalSortExpr</a>\>, expr: <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.LexOrdering.html" class="struct" title="struct datafusion::physical_expr::LexOrdering">LexOrdering</a>, k: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, batch_size: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, runtime: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/runtime_env/struct.RuntimeEnv.html" class="struct" title="struct datafusion::execution::runtime_env::RuntimeEnv">RuntimeEnv</a>\>, metrics: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.ExecutionPlanMetricsSet.html" class="struct" title="struct datafusion::physical_plan::metrics::ExecutionPlanMetricsSet">ExecutionPlanMetricsSet</a>, filter: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/lock_api/0.4.13/x86_64-unknown-linux-gnu/lock_api/rwlock/struct.RwLock.html" class="struct" title="struct lock_api::rwlock::RwLock">RwLock</a>\<<a href="https://docs.rs/parking_lot/0.12.4/x86_64-unknown-linux-gnu/parking_lot/raw_rwlock/struct.RawRwLock.html" class="struct" title="struct parking_lot::raw_rwlock::RawRwLock">RawRwLock</a>, TopKDynamicFilters\>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.TopK.html" class="struct" title="struct datafusion::physical_plan::TopK">TopK</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Create a new [`TopK`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.TopK.html "struct datafusion::physical_plan::TopK") that stores the top `k` values, as defined by the sort expressions in `expr`.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.TopK.html#method.insert_batch" class="fn">insert_batch</a>( &mut self, batch: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html" class="struct" title="struct datafusion::common::arrow::array::RecordBatch">RecordBatch</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Insert `batch`, remembering if any of its values are among the top k seen so far.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.TopK.html#method.emit" class="fn">emit</a>( self, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.RecordBatchStream.html" class="trait" title="trait datafusion::execution::RecordBatchStream">RecordBatchStream</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html" class="struct" title="struct datafusion::common::arrow::array::RecordBatch">RecordBatch</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>\>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Returns the top k results broken into `batch_size` [`RecordBatch`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html "struct datafusion::common::arrow::array::RecordBatch")es, consuming the heap

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.TopK.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.TopK.html#blanket-implementations" class="anchor">§</a>
