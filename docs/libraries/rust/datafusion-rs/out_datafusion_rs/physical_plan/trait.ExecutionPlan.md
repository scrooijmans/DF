# Trait ExecutionPlan Copy item path

<a href="https://docs.rs/datafusion-physical-plan/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_plan/execution_plan.rs.html#86" class="src">Source</a>

``` rust
pub trait ExecutionPlan:
    Debug
    + DisplayAs
    + Send
    + Sync {
Show 26 methods    // Required methods
    fn name(&self) -> &str;
    fn as_any(&self) -> &(dyn Any + 'static);
    fn properties(&self) -> &PlanProperties;
    fn children(&self) -> Vec<&Arc<dyn ExecutionPlan>>;
    fn with_new_children(
        self: Arc<Self>,
        children: Vec<Arc<dyn ExecutionPlan>>,
    ) -> Result<Arc<dyn ExecutionPlan>, DataFusionError>;
    fn execute(
        &self,
        partition: usize,
        context: Arc<TaskContext>,
    ) -> Result<Pin<Box<dyn RecordBatchStream<Item = Result<RecordBatch, DataFusionError>> + Send>>, DataFusionError>;

    // Provided methods
    fn static_name() -> &'static str
       where Self: Sized { ... }
    fn schema(&self) -> Arc<Schema> { ... }
    fn check_invariants(
        &self,
        check: InvariantLevel,
    ) -> Result<(), DataFusionError> { ... }
    fn required_input_distribution(&self) -> Vec<Distribution> { ... }
    fn required_input_ordering(&self) -> Vec<Option<OrderingRequirements>> { ... }
    fn maintains_input_order(&self) -> Vec<bool> { ... }
    fn benefits_from_input_partitioning(&self) -> Vec<bool> { ... }
    fn reset_state(
        self: Arc<Self>,
    ) -> Result<Arc<dyn ExecutionPlan>, DataFusionError> { ... }
    fn repartitioned(
        &self,
        _target_partitions: usize,
        _config: &ConfigOptions,
    ) -> Result<Option<Arc<dyn ExecutionPlan>>, DataFusionError> { ... }
    fn metrics(&self) -> Option<MetricsSet> { ... }
    fn statistics(&self) -> Result<Statistics, DataFusionError> { ... }
    fn partition_statistics(
        &self,
        partition: Option<usize>,
    ) -> Result<Statistics, DataFusionError> { ... }
    fn supports_limit_pushdown(&self) -> bool { ... }
    fn with_fetch(
        &self,
        _limit: Option<usize>,
    ) -> Option<Arc<dyn ExecutionPlan>> { ... }
    fn fetch(&self) -> Option<usize> { ... }
    fn cardinality_effect(&self) -> CardinalityEffect { ... }
    fn try_swapping_with_projection(
        &self,
        _projection: &ProjectionExec,
    ) -> Result<Option<Arc<dyn ExecutionPlan>>, DataFusionError> { ... }
    fn gather_filters_for_pushdown(
        &self,
        _phase: FilterPushdownPhase,
        parent_filters: Vec<Arc<dyn PhysicalExpr>>,
        _config: &ConfigOptions,
    ) -> Result<FilterDescription, DataFusionError> { ... }
    fn handle_child_pushdown_result(
        &self,
        _phase: FilterPushdownPhase,
        child_pushdown_result: ChildPushdownResult,
        _config: &ConfigOptions,
    ) -> Result<FilterPushdownPropagation<Arc<dyn ExecutionPlan>>, DataFusionError> { ... }
    fn with_new_state(
        &self,
        _state: Arc<dyn Any + Sync + Send>,
    ) -> Option<Arc<dyn ExecutionPlan>> { ... }
}
```

Expand description

Represent nodes in the DataFusion Physical Plan.

Calling [`execute`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#tymethod.execute "method datafusion::physical_plan::ExecutionPlan::execute") produces an `async` [`SendableRecordBatchStream`](https://docs.rs/datafusion/50.2.0/datafusion/execution/type.SendableRecordBatchStream.html "type datafusion::execution::SendableRecordBatchStream") of [`RecordBatch`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html "struct datafusion::common::arrow::array::RecordBatch") that incrementally computes a partition of the `ExecutionPlan`’s output from its input. See [`Partitioning`](https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.Partitioning.html "enum datafusion::physical_expr::Partitioning") for more details on partitioning.

Methods such as [`Self::schema`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.schema "method datafusion_physical_plan::execution_plan::ExecutionPlan::schema::schema") and [`Self::properties`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#tymethod.properties "method datafusion_physical_plan::execution_plan::ExecutionPlan::properties::properties") communicate properties of the output to the DataFusion optimizer, and methods such as [`required_input_distribution`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.required_input_distribution "method datafusion::physical_plan::ExecutionPlan::required_input_distribution") and [`required_input_ordering`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.required_input_ordering "method datafusion::physical_plan::ExecutionPlan::required_input_ordering") express requirements of the `ExecutionPlan` from its input.

[`ExecutionPlan`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html "trait datafusion::physical_plan::ExecutionPlan") can be displayed in a simplified form using the return value from [`displayable`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/fn.displayable.html "fn datafusion::physical_plan::displayable") in addition to the (normally quite verbose) `Debug` output.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#examples" class="doc-anchor">§</a>Examples

See [`datafusion-examples`](https://github.com/apache/datafusion/tree/main/datafusion-examples) for examples, including [`memory_pool_execution_plan.rs`](https://github.com/apache/datafusion/blob/main/datafusion-examples/examples/memory_pool_execution_plan.rs) which shows how to implement a custom `ExecutionPlan` with memory tracking and spilling support.

## Required Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#tymethod.name" class="fn">name</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

Short name for the ExecutionPlan, such as ‘DataSourceExec’.

Implementation note: this method can just proxy to [`static_name`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.static_name "associated function datafusion::physical_plan::ExecutionPlan::static_name") if no special action is needed. It doesn’t provide a default implementation like that because this method doesn’t require the `Sized` constrain to allow a wilder range of use cases.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#tymethod.as_any" class="fn">as_any</a>(&self) -\> &(dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + 'static)

Returns the execution plan as [`Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") so that it can be downcast to a specific implementation.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#tymethod.properties" class="fn">properties</a>(&self) -\> &<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.PlanProperties.html" class="struct" title="struct datafusion::physical_plan::PlanProperties">PlanProperties</a>

Return properties of the output of the `ExecutionPlan`, such as output ordering(s), partitioning information etc.

This information is available via methods on [`ExecutionPlanProperties`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlanProperties.html "trait datafusion::physical_plan::ExecutionPlanProperties") trait, which is implemented for all `ExecutionPlan`s.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#tymethod.children" class="fn">children</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<&<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a>\>\>

Get a list of children `ExecutionPlan`s that act as inputs to this plan. The returned list will be empty for leaf nodes such as scans, will contain a single value for unary nodes, or two values for binary nodes (such as joins).

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#tymethod.with_new_children" class="fn">with_new_children</a>( self: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<Self\>, children: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a>\>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Returns a new `ExecutionPlan` where all existing children were replaced by the `children`, in order

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#tymethod.execute" class="fn">execute</a>( &self, partition: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, context: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/struct.TaskContext.html" class="struct" title="struct datafusion::execution::TaskContext">TaskContext</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.RecordBatchStream.html" class="trait" title="trait datafusion::execution::RecordBatchStream">RecordBatchStream</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html" class="struct" title="struct datafusion::common::arrow::array::RecordBatch">RecordBatch</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>\>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Begin execution of `partition`, returning a [`Stream`](https://docs.rs/futures-core/0.3.31/x86_64-unknown-linux-gnu/futures_core/stream/trait.Stream.html "trait futures_core::stream::Stream") of [`RecordBatch`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html "struct datafusion::common::arrow::array::RecordBatch")es.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#notes" class="doc-anchor">§</a>Notes

The `execute` method itself is not `async` but it returns an `async` [`futures::stream::Stream`](https://docs.rs/futures-core/0.3.31/x86_64-unknown-linux-gnu/futures_core/stream/trait.Stream.html "trait futures_core::stream::Stream"). This `Stream` should incrementally compute the output, `RecordBatch` by `RecordBatch` (in a streaming fashion). Most `ExecutionPlan`s should not do any work before the first `RecordBatch` is requested from the stream.

[`RecordBatchStreamAdapter`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/stream/struct.RecordBatchStreamAdapter.html "struct datafusion::physical_plan::stream::RecordBatchStreamAdapter") can be used to convert an `async` [`Stream`](https://docs.rs/futures-core/0.3.31/x86_64-unknown-linux-gnu/futures_core/stream/trait.Stream.html "trait futures_core::stream::Stream") into a [`SendableRecordBatchStream`](https://docs.rs/datafusion/50.2.0/datafusion/execution/type.SendableRecordBatchStream.html "type datafusion::execution::SendableRecordBatchStream").

Using `async` `Streams` allows for network I/O during execution and takes advantage of Rust’s built in support for `async` continuations and crate ecosystem.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#error-handling" class="doc-anchor">§</a>Error handling

Any error that occurs during execution is sent as an `Err` in the output stream.

`ExecutionPlan` implementations in DataFusion cancel additional work immediately once an error occurs. The rationale is that if the overall query will return an error, any additional work such as continued polling of inputs will be wasted as it will be thrown away.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#cancellation--aborting-execution" class="doc-anchor">§</a>Cancellation / Aborting Execution

The [`Stream`](https://docs.rs/futures-core/0.3.31/x86_64-unknown-linux-gnu/futures_core/stream/trait.Stream.html "trait futures_core::stream::Stream") that is returned must ensure that any allocated resources are freed when the stream itself is dropped. This is particularly important for [`spawn`](https://docs.rs/tokio/1.47.1/x86_64-unknown-linux-gnu/tokio/task/spawn/fn.spawn.html "fn tokio::task::spawn::spawn")ed tasks or threads. Unless care is taken to “abort” such tasks, they may continue to consume resources even after the plan is dropped, generating intermediate results that are never used. Thus, [`spawn`](https://docs.rs/tokio/1.47.1/x86_64-unknown-linux-gnu/tokio/task/spawn/fn.spawn.html "fn tokio::task::spawn::spawn") is disallowed, and instead use [`SpawnedTask`](https://docs.rs/datafusion/50.2.0/datafusion/common/runtime/struct.SpawnedTask.html "struct datafusion::common::runtime::SpawnedTask").

To enable timely cancellation, the [`Stream`](https://docs.rs/futures-core/0.3.31/x86_64-unknown-linux-gnu/futures_core/stream/trait.Stream.html "trait futures_core::stream::Stream") that is returned must not block the CPU indefinitely and must yield back to the tokio runtime regularly. In a typical [`ExecutionPlan`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html "trait datafusion::physical_plan::ExecutionPlan"), this automatically happens unless there are special circumstances; e.g. when the computational complexity of processing a batch is superlinear. See this [general guideline](https://ryhl.io/blog/async-what-is-blocking/) for more context on this point, which explains why one should avoid spending a long time without reaching an `await`/yield point in asynchronous runtimes. This can be achieved by using the utilities from the [`coop`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/coop/index.html "mod datafusion::physical_plan::coop") module, by manually returning [`Poll::Pending`](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html#variant.Pending "variant core::task::poll::Poll::Pending") and setting up wakers appropriately, or by calling [`tokio::task::yield_now()`](https://docs.rs/tokio/1.47.1/x86_64-unknown-linux-gnu/tokio/task/yield_now/fn.yield_now.html "fn tokio::task::yield_now::yield_now") when appropriate. In special cases that warrant manual yielding, determination for “regularly” may be made using the [Tokio task budget](https://docs.rs/tokio/latest/tokio/task/coop/index.html), a timer (being careful with the overhead-heavy system call needed to take the time), or by counting rows or batches.

The [cancellation benchmark](https://github.com/apache/datafusion/blob/main/benchmarks/README.md#cancellation) tracks some cases of how quickly queries can be cancelled.

For more details see [`SpawnedTask`](https://docs.rs/datafusion/50.2.0/datafusion/common/runtime/struct.SpawnedTask.html "struct datafusion::common::runtime::SpawnedTask"), [`JoinSet`](https://docs.rs/datafusion/50.2.0/datafusion/common/runtime/struct.JoinSet.html "struct datafusion::common::runtime::JoinSet") and [`RecordBatchReceiverStreamBuilder`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/stream/struct.RecordBatchReceiverStreamBuilder.html "struct datafusion::physical_plan::stream::RecordBatchReceiverStreamBuilder") for structures to help ensure all background tasks are cancelled.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#implementation-examples" class="doc-anchor">§</a>Implementation Examples

While `async` `Stream`s have a non trivial learning curve, the [`futures`](https://docs.rs/futures/0.3.31/x86_64-unknown-linux-gnu/futures/index.html "mod futures") crate provides [`StreamExt`](https://docs.rs/futures-util/0.3.31/x86_64-unknown-linux-gnu/futures_util/stream/stream/trait.StreamExt.html "trait futures_util::stream::stream::StreamExt") and [`TryStreamExt`](https://docs.rs/futures-util/0.3.31/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/trait.TryStreamExt.html "trait futures_util::stream::try_stream::TryStreamExt") which help simplify many common operations.

Here are some common patterns:

###### <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#return-precomputed-recordbatch" class="doc-anchor">§</a>Return Precomputed `RecordBatch`

We can return a precomputed `RecordBatch` as a `Stream`:

``` rust
struct MyPlan {
    batch: RecordBatch,
}

impl MyPlan {
    fn execute(
        &self,
        partition: usize,
        context: Arc<TaskContext>
    ) -> Result<SendableRecordBatchStream> {
        // use functions from futures crate convert the batch into a stream
        let fut = futures::future::ready(Ok(self.batch.clone()));
        let stream = futures::stream::once(fut);
        Ok(Box::pin(RecordBatchStreamAdapter::new(self.batch.schema(), stream)))
    }
}
```

###### <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#lazily-async-compute-recordbatch" class="doc-anchor">§</a>Lazily (async) Compute `RecordBatch`

We can also lazily compute a `RecordBatch` when the returned `Stream` is polled

``` rust
struct MyPlan {
    schema: SchemaRef,
}

/// Returns a single batch when the returned stream is polled
async fn get_batch() -> Result<RecordBatch> {
    todo!()
}

impl MyPlan {
    fn execute(
        &self,
        partition: usize,
        context: Arc<TaskContext>
    ) -> Result<SendableRecordBatchStream> {
        let fut = get_batch();
        let stream = futures::stream::once(fut);
        Ok(Box::pin(RecordBatchStreamAdapter::new(self.schema.clone(), stream)))
    }
}
```

###### <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#lazily-async-create-a-stream" class="doc-anchor">§</a>Lazily (async) create a Stream

If you need to create the return `Stream` using an `async` function, you can do so by flattening the result:

``` rust
struct MyPlan {
    schema: SchemaRef,
}

/// async function that returns a stream
async fn get_batch_stream() -> Result<SendableRecordBatchStream> {
    todo!()
}

impl MyPlan {
    fn execute(
        &self,
        partition: usize,
        context: Arc<TaskContext>
    ) -> Result<SendableRecordBatchStream> {
        // A future that yields a stream
        let fut = get_batch_stream();
        // Use TryStreamExt::try_flatten to flatten the stream of streams
        let stream = futures::stream::once(fut).try_flatten();
        Ok(Box::pin(RecordBatchStreamAdapter::new(self.schema.clone(), stream)))
    }
}
```

## Provided Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#provided-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.static_name" class="fn">static_name</a>() -\> &'static <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Short name for the ExecutionPlan, such as ‘DataSourceExec’. Like [`name`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#tymethod.name "method datafusion::physical_plan::ExecutionPlan::name") but can be called without an instance.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.schema" class="fn">schema</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>\>

Get the schema for this execution plan

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.check_invariants" class="fn">check_invariants</a>(&self, check: <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/enum.InvariantLevel.html" class="enum" title="enum datafusion::physical_plan::execution_plan::InvariantLevel">InvariantLevel</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Returns an error if this individual node does not conform to its invariants. These invariants are typically only checked in debug mode.

A default set of invariants is provided in the [check_default_invariants](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/fn.check_default_invariants.html "fn datafusion::physical_plan::execution_plan::check_default_invariants") function. The default implementation of `check_invariants` calls this function. Extension nodes can provide their own invariants.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.required_input_distribution" class="fn">required_input_distribution</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.Distribution.html" class="enum" title="enum datafusion::physical_expr::Distribution">Distribution</a>\>

Specifies the data distribution requirements for all the children for this `ExecutionPlan`, By default it’s \[[Distribution::UnspecifiedDistribution](https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.Distribution.html#variant.UnspecifiedDistribution "variant datafusion::physical_expr::Distribution::UnspecifiedDistribution")\] for each child,

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.required_input_ordering" class="fn">required_input_ordering</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.OrderingRequirements.html" class="enum" title="enum datafusion::physical_expr::OrderingRequirements">OrderingRequirements</a>\>\>

Specifies the ordering required for all of the children of this `ExecutionPlan`.

For each child, it’s the local ordering requirement within each partition rather than the global ordering

NOTE that checking `!is_empty()` does **not** check for a required input ordering. Instead, the correct check is that at least one entry must be `Some`

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.maintains_input_order" class="fn">maintains_input_order</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>\>

Returns `false` if this `ExecutionPlan`’s implementation may reorder rows within or between partitions.

For example, Projection, Filter, and Limit maintain the order of inputs – they may transform values (Projection) or not produce the same number of rows that went in (Filter and Limit), but the rows that are produced go in the same way.

DataFusion uses this metadata to apply certain optimizations such as automatically repartitioning correctly.

The default implementation returns `false`

WARNING: if you override this default, you *MUST* ensure that the `ExecutionPlan`’s maintains the ordering invariant or else DataFusion may produce incorrect results.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.benefits_from_input_partitioning" class="fn">benefits_from_input_partitioning</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>\>

Specifies whether the `ExecutionPlan` benefits from increased parallelization at its input for each child.

If returns `true`, the `ExecutionPlan` would benefit from partitioning its corresponding child (and thus from more parallelism). For `ExecutionPlan` that do very little work the overhead of extra parallelism may outweigh any benefits

The default implementation returns `true` unless this `ExecutionPlan` has signalled it requires a single child input partition.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.reset_state" class="fn">reset_state</a>( self: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<Self\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Reset any internal state within this [`ExecutionPlan`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html "trait datafusion::physical_plan::ExecutionPlan").

This method is called when an [`ExecutionPlan`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html "trait datafusion::physical_plan::ExecutionPlan") needs to be re-executed, such as in recursive queries. Unlike [`ExecutionPlan::with_new_children`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#tymethod.with_new_children "method datafusion::physical_plan::ExecutionPlan::with_new_children"), this method ensures that any stateful components (e.g., [`DynamicFilterPhysicalExpr`](https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/expressions/struct.DynamicFilterPhysicalExpr.html "struct datafusion::physical_expr::expressions::DynamicFilterPhysicalExpr")) are reset to their initial state.

The default implementation simply calls [`ExecutionPlan::with_new_children`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#tymethod.with_new_children "method datafusion::physical_plan::ExecutionPlan::with_new_children") with the existing children, effectively creating a new instance of the [`ExecutionPlan`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html "trait datafusion::physical_plan::ExecutionPlan") with the same children but without necessarily resetting any internal state. Implementations that require resetting of some internal state should override this method to provide the necessary logic.

This method should *not* reset state recursively for children, as it is expected that it will be called from within a walk of the execution plan tree so that it will be called on each child later or was already called on each child.

Note to implementers: unlike [`ExecutionPlan::with_new_children`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#tymethod.with_new_children "method datafusion::physical_plan::ExecutionPlan::with_new_children") this method does not accept new children as an argument, thus it is expected that any cached plan properties will remain valid after the reset.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.repartitioned" class="fn">repartitioned</a>( &self, \_target_partitions: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, \_config: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigOptions.html" class="struct" title="struct datafusion::config::ConfigOptions">ConfigOptions</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a>\>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

If supported, attempt to increase the partitioning of this `ExecutionPlan` to produce `target_partitions` partitions.

If the `ExecutionPlan` does not support changing its partitioning, returns `Ok(None)` (the default).

It is the `ExecutionPlan` can increase its partitioning, but not to the `target_partitions`, it may return an ExecutionPlan with fewer partitions. This might happen, for example, if each new partition would be too small to be efficiently processed individually.

The DataFusion optimizer attempts to use as many threads as possible by repartitioning its inputs to match the target number of threads available (`target_partitions`). Some data sources, such as the built in CSV and Parquet readers, implement this method as they are able to read from their input files in parallel, regardless of how the source data is split amongst files.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.metrics" class="fn">metrics</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.MetricsSet.html" class="struct" title="struct datafusion::physical_plan::metrics::MetricsSet">MetricsSet</a>\>

Return a snapshot of the set of [`Metric`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.Metric.html "struct datafusion::physical_plan::Metric")s for this [`ExecutionPlan`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html "trait datafusion::physical_plan::ExecutionPlan"). If no `Metric`s are available, return None.

While the values of the metrics in the returned [`MetricsSet`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.MetricsSet.html "struct datafusion::physical_plan::metrics::MetricsSet")s may change as execution progresses, the specific metrics will not.

Once `self.execute()` has returned (technically the future is resolved) for all available partitions, the set of metrics should be complete. If this function is called prior to `execute()` new metrics may appear in subsequent calls.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.statistics" class="fn">statistics</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Statistics.html" class="struct" title="struct datafusion::common::Statistics">Statistics</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

👎Deprecated since 48.0.0: Use `partition_statistics` method instead

Returns statistics for this `ExecutionPlan` node. If statistics are not available, should return [`Statistics::new_unknown`](https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Statistics.html#method.new_unknown "associated function datafusion::common::Statistics::new_unknown") (the default), not an error.

For TableScan executors, which supports filter pushdown, special attention needs to be paid to whether the stats returned by this method are exact or not

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.partition_statistics" class="fn">partition_statistics</a>( &self, partition: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Statistics.html" class="struct" title="struct datafusion::common::Statistics">Statistics</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Returns statistics for a specific partition of this `ExecutionPlan` node. If statistics are not available, should return [`Statistics::new_unknown`](https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Statistics.html#method.new_unknown "associated function datafusion::common::Statistics::new_unknown") (the default), not an error. If `partition` is `None`, it returns statistics for the entire plan.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.supports_limit_pushdown" class="fn">supports_limit_pushdown</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns `true` if a limit can be safely pushed down through this `ExecutionPlan` node.

If this method returns `true`, and the query plan contains a limit at the output of this node, DataFusion will push the limit to the input of this node.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.with_fetch" class="fn">with_fetch</a>(&self, \_limit: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a>\>\>

Returns a fetching variant of this `ExecutionPlan` node, if it supports fetch limits. Returns `None` otherwise.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.fetch" class="fn">fetch</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>

Gets the fetch count for the operator, `None` means there is no fetch.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.cardinality_effect" class="fn">cardinality_effect</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/enum.CardinalityEffect.html" class="enum" title="enum datafusion::physical_plan::execution_plan::CardinalityEffect">CardinalityEffect</a>

Gets the effect on cardinality, if known

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.try_swapping_with_projection" class="fn">try_swapping_with_projection</a>( &self, \_projection: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/projection/struct.ProjectionExec.html" class="struct" title="struct datafusion::physical_plan::projection::ProjectionExec">ProjectionExec</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a>\>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Attempts to push down the given projection into the input of this `ExecutionPlan`.

If the operator supports this optimization, the resulting plan will be: `self_new <- projection <- source`, starting from `projection <- self <- source`. Otherwise, it returns the current `ExecutionPlan` as-is.

Returns `Ok(Some(...))` if pushdown is applied, `Ok(None)` if it is not supported or not possible, or `Err` on failure.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.gather_filters_for_pushdown" class="fn">gather_filters_for_pushdown</a>( &self, \_phase: <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter_pushdown/enum.FilterPushdownPhase.html" class="enum" title="enum datafusion::physical_plan::filter_pushdown::FilterPushdownPhase">FilterPushdownPhase</a>, parent_filters: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html" class="trait" title="trait datafusion::physical_expr::PhysicalExpr">PhysicalExpr</a>\>\>, \_config: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigOptions.html" class="struct" title="struct datafusion::config::ConfigOptions">ConfigOptions</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter_pushdown/struct.FilterDescription.html" class="struct" title="struct datafusion::physical_plan::filter_pushdown::FilterDescription">FilterDescription</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Collect filters that this node can push down to its children. Filters that are being pushed down from parents are passed in, and the node may generate additional filters to push down. For example, given the plan FilterExec -\> HashJoinExec -\> DataSourceExec, what will happen is that we recurse down the plan calling `ExecutionPlan::gather_filters_for_pushdown`:

1.  `FilterExec::gather_filters_for_pushdown` is called with no parent filters so it only returns that `FilterExec` wants to push down its own predicate.
2.  `HashJoinExec::gather_filters_for_pushdown` is called with the filter from `FilterExec`, which it only allows to push down to one side of the join (unless it’s on the join key) but it also adds its own filters (e.g. pushing down a bloom filter of the hash table to the scan side of the join).
3.  `DataSourceExec::gather_filters_for_pushdown` is called with both filters from `HashJoinExec` and `FilterExec`, however `DataSourceExec::gather_filters_for_pushdown` doesn’t actually do anything since it has no children and no additional filters to push down. It’s only once [`ExecutionPlan::handle_child_pushdown_result`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.handle_child_pushdown_result "method datafusion::physical_plan::ExecutionPlan::handle_child_pushdown_result") is called on `DataSourceExec` as we recurse up the plan that `DataSourceExec` can actually bind the filters.

The default implementation bars all parent filters from being pushed down and adds no new filters. This is the safest option, making filter pushdown opt-in on a per-node pasis.

There are two different phases in filter pushdown, which some operators may handle the same and some differently. Depending on the phase the operator may or may not be allowed to modify the plan. See [`FilterPushdownPhase`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter_pushdown/enum.FilterPushdownPhase.html "enum datafusion::physical_plan::filter_pushdown::FilterPushdownPhase") for more details.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.handle_child_pushdown_result" class="fn">handle_child_pushdown_result</a>( &self, \_phase: <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter_pushdown/enum.FilterPushdownPhase.html" class="enum" title="enum datafusion::physical_plan::filter_pushdown::FilterPushdownPhase">FilterPushdownPhase</a>, child_pushdown_result: <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter_pushdown/struct.ChildPushdownResult.html" class="struct" title="struct datafusion::physical_plan::filter_pushdown::ChildPushdownResult">ChildPushdownResult</a>, \_config: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigOptions.html" class="struct" title="struct datafusion::config::ConfigOptions">ConfigOptions</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter_pushdown/struct.FilterPushdownPropagation.html" class="struct" title="struct datafusion::physical_plan::filter_pushdown::FilterPushdownPropagation">FilterPushdownPropagation</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a>\>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Handle the result of a child pushdown. This method is called as we recurse back up the plan tree after pushing filters down to child nodes via [`ExecutionPlan::gather_filters_for_pushdown`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.gather_filters_for_pushdown "method datafusion::physical_plan::ExecutionPlan::gather_filters_for_pushdown"). It allows the current node to process the results of filter pushdown from its children, deciding whether to absorb filters, modify the plan, or pass filters back up to its parent.

**Purpose and Context:** Filter pushdown is a critical optimization in DataFusion that aims to reduce the amount of data processed by applying filters as early as possible in the query plan. This method is part of the second phase of filter pushdown, where results are propagated back up the tree after being pushed down. Each node can inspect the pushdown results from its children and decide how to handle any unapplied filters, potentially optimizing the plan structure or filter application.

**Behavior in Different Nodes:**

- For a `DataSourceExec`, this often means absorbing the filters to apply them during the scan phase (late materialization), reducing the data read from the source.
- A `FilterExec` may absorb any filters its children could not handle, combining them with its own predicate. If no filters remain (i.e., the predicate becomes trivially true), it may remove itself from the plan altogether. It typically marks parent filters as supported, indicating they have been handled.
- A `HashJoinExec` might ignore the pushdown result if filters need to be applied during the join operation. It passes the parent filters back up wrapped in [`FilterPushdownPropagation::if_any`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter_pushdown/struct.FilterPushdownPropagation.html#method.if_any "associated function datafusion::physical_plan::filter_pushdown::FilterPushdownPropagation::if_any"), discarding any self-filters from children.

**Example Walkthrough:** Consider a query plan: `FilterExec (f1) -> HashJoinExec -> DataSourceExec`.

1.  **Downward Phase (`gather_filters_for_pushdown`):** Starting at `FilterExec`, the filter `f1` is gathered and pushed down to `HashJoinExec`. `HashJoinExec` may allow `f1` to pass to one side of the join or add its own filters (e.g., a min-max filter from the build side), then pushes filters to `DataSourceExec`. `DataSourceExec`, being a leaf node, has no children to push to, so it prepares to handle filters in the upward phase.
2.  **Upward Phase (`handle_child_pushdown_result`):** Starting at `DataSourceExec`, it absorbs applicable filters from `HashJoinExec` for late materialization during scanning, marking them as supported. `HashJoinExec` receives the result, decides whether to apply any remaining filters during the join, and passes unhandled filters back up to `FilterExec`. `FilterExec` absorbs any unhandled filters, updates its predicate if necessary, or removes itself if the predicate becomes trivial (e.g., `lit(true)`), and marks filters as supported for its parent.

The default implementation is a no-op that passes the result of pushdown from the children to its parent transparently, ensuring no filters are lost if a node does not override this behavior.

**Notes for Implementation:** When returning filters via [`FilterPushdownPropagation`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter_pushdown/struct.FilterPushdownPropagation.html "struct datafusion::physical_plan::filter_pushdown::FilterPushdownPropagation"), the order of filters need not match the order they were passed in via `child_pushdown_result`. However, preserving the order is recommended for debugging and ease of reasoning about the resulting plans.

**Helper Methods for Customization:** There are various helper methods to simplify implementing this method:

- [`FilterPushdownPropagation::if_any`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter_pushdown/struct.FilterPushdownPropagation.html#method.if_any "associated function datafusion::physical_plan::filter_pushdown::FilterPushdownPropagation::if_any"): Marks all parent filters as supported as long as at least one child supports them.
- [`FilterPushdownPropagation::if_all`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter_pushdown/struct.FilterPushdownPropagation.html#method.if_all "associated function datafusion::physical_plan::filter_pushdown::FilterPushdownPropagation::if_all"): Marks all parent filters as supported as long as all children support them.
- [`FilterPushdownPropagation::with_parent_pushdown_result`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter_pushdown/struct.FilterPushdownPropagation.html#method.with_parent_pushdown_result "associated function datafusion::physical_plan::filter_pushdown::FilterPushdownPropagation::with_parent_pushdown_result"): Allows adding filters to the propagation result, indicating which filters are supported by the current node.
- [`FilterPushdownPropagation::with_updated_node`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter_pushdown/struct.FilterPushdownPropagation.html#method.with_updated_node "method datafusion::physical_plan::filter_pushdown::FilterPushdownPropagation::with_updated_node"): Allows updating the current node in the propagation result, used if the node has modified its plan based on the pushdown results.

**Filter Pushdown Phases:** There are two different phases in filter pushdown (`Pre` and others), which some operators may handle differently. Depending on the phase, the operator may or may not be allowed to modify the plan. See [`FilterPushdownPhase`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter_pushdown/enum.FilterPushdownPhase.html "enum datafusion::physical_plan::filter_pushdown::FilterPushdownPhase") for more details on phase-specific behavior.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.with_new_state" class="fn">with_new_state</a>( &self, \_state: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a>\>\>

Injects arbitrary run-time state into this execution plan, returning a new plan instance that incorporates that state *if* it is relevant to the concrete node implementation.

This is a generic entry point: the `state` can be any type wrapped in `Arc<dyn Any + Send + Sync>`. A node that cares about the state should down-cast it to the concrete type it expects and, if successful, return a modified copy of itself that captures the provided value. If the state is not applicable, the default behaviour is to return `None` so that parent nodes can continue propagating the attempt further down the plan tree.

For example, [`WorkTableExec`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/work_table/struct.WorkTableExec.html "struct datafusion::physical_plan::work_table::WorkTableExec") down-casts the supplied state to an `Arc<WorkTable>` in order to wire up the working table used during recursive-CTE execution. Similar patterns can be followed by custom nodes that need late-bound dependencies or shared state.

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#impl-DynTreeNode-for-dyn+ExecutionPlan" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.DynTreeNode.html" class="trait" title="trait datafusion::common::tree_node::DynTreeNode">DynTreeNode</a> for dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.arc_children" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.DynTreeNode.html#tymethod.arc_children" class="fn">arc_children</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<&<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a>\>\>

Returns all children of the specified `TreeNode`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.with_new_arc_children" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.DynTreeNode.html#tymethod.with_new_arc_children" class="fn">with_new_arc_children</a>( &self, arc_self: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a>\>, new_children: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a>\>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Constructs a new node with the specified children.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#impl-ExecutionPlanProperties-for-%26dyn+ExecutionPlan" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlanProperties.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlanProperties">ExecutionPlanProperties</a> for &dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.output_partitioning" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlanProperties.html#tymethod.output_partitioning" class="fn">output_partitioning</a>(&self) -\> &<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.Partitioning.html" class="enum" title="enum datafusion::physical_expr::Partitioning">Partitioning</a>

Specifies how the output of this `ExecutionPlan` is split into partitions.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.output_ordering" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlanProperties.html#tymethod.output_ordering" class="fn">output_ordering</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.LexOrdering.html" class="struct" title="struct datafusion::physical_expr::LexOrdering">LexOrdering</a>\>

If the output of this `ExecutionPlan` within each partition is sorted, returns `Some(keys)` describing the ordering. A `None` return value indicates no assumptions should be made on the output ordering. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlanProperties.html#tymethod.output_ordering)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.boundedness" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlanProperties.html#tymethod.boundedness" class="fn">boundedness</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/enum.Boundedness.html" class="enum" title="enum datafusion::physical_plan::execution_plan::Boundedness">Boundedness</a>

Boundedness information of the stream corresponding to this `ExecutionPlan`. For more details, see [`Boundedness`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/enum.Boundedness.html "enum datafusion::physical_plan::execution_plan::Boundedness").

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.pipeline_behavior" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlanProperties.html#tymethod.pipeline_behavior" class="fn">pipeline_behavior</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/enum.EmissionType.html" class="enum" title="enum datafusion::physical_plan::execution_plan::EmissionType">EmissionType</a>

Indicates how the stream of this `ExecutionPlan` emits its results. For more details, see [`EmissionType`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/enum.EmissionType.html "enum datafusion::physical_plan::execution_plan::EmissionType").

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.equivalence_properties" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlanProperties.html#tymethod.equivalence_properties" class="fn">equivalence_properties</a>(&self) -\> &<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.EquivalenceProperties.html" class="struct" title="struct datafusion::physical_expr::EquivalenceProperties">EquivalenceProperties</a>

Get the [`EquivalenceProperties`](https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.EquivalenceProperties.html "struct datafusion::physical_expr::EquivalenceProperties") within the plan. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlanProperties.html#tymethod.equivalence_properties)

## Implementors<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#impl-ExecutionPlan-for-DataSourceExec" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/memory/struct.DataSourceExec.html" class="struct" title="struct datafusion::datasource::memory::DataSourceExec">DataSourceExec</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#impl-ExecutionPlan-for-DataSinkExec" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/sink/struct.DataSinkExec.html" class="struct" title="struct datafusion::datasource::sink::DataSinkExec">DataSinkExec</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#impl-ExecutionPlan-for-OutputRequirementExec" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/output_requirements/struct.OutputRequirementExec.html" class="struct" title="struct datafusion::physical_optimizer::output_requirements::OutputRequirementExec">OutputRequirementExec</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#impl-ExecutionPlan-for-AggregateExec" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/aggregates/struct.AggregateExec.html" class="struct" title="struct datafusion::physical_plan::aggregates::AggregateExec">AggregateExec</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#impl-ExecutionPlan-for-AnalyzeExec" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/analyze/struct.AnalyzeExec.html" class="struct" title="struct datafusion::physical_plan::analyze::AnalyzeExec">AnalyzeExec</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#impl-ExecutionPlan-for-AsyncFuncExec" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/async_func/struct.AsyncFuncExec.html" class="struct" title="struct datafusion::physical_plan::async_func::AsyncFuncExec">AsyncFuncExec</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#impl-ExecutionPlan-for-CoalesceBatchesExec" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/coalesce_batches/struct.CoalesceBatchesExec.html" class="struct" title="struct datafusion::physical_plan::coalesce_batches::CoalesceBatchesExec">CoalesceBatchesExec</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#impl-ExecutionPlan-for-CoalescePartitionsExec" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/coalesce_partitions/struct.CoalescePartitionsExec.html" class="struct" title="struct datafusion::physical_plan::coalesce_partitions::CoalescePartitionsExec">CoalescePartitionsExec</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#impl-ExecutionPlan-for-CooperativeExec" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/coop/struct.CooperativeExec.html" class="struct" title="struct datafusion::physical_plan::coop::CooperativeExec">CooperativeExec</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#impl-ExecutionPlan-for-EmptyExec" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/empty/struct.EmptyExec.html" class="struct" title="struct datafusion::physical_plan::empty::EmptyExec">EmptyExec</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#impl-ExecutionPlan-for-ExplainExec" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/explain/struct.ExplainExec.html" class="struct" title="struct datafusion::physical_plan::explain::ExplainExec">ExplainExec</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#impl-ExecutionPlan-for-FilterExec" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter/struct.FilterExec.html" class="struct" title="struct datafusion::physical_plan::filter::FilterExec">FilterExec</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#impl-ExecutionPlan-for-CrossJoinExec" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.CrossJoinExec.html" class="struct" title="struct datafusion::physical_plan::joins::CrossJoinExec">CrossJoinExec</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#impl-ExecutionPlan-for-HashJoinExec" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.HashJoinExec.html" class="struct" title="struct datafusion::physical_plan::joins::HashJoinExec">HashJoinExec</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#impl-ExecutionPlan-for-NestedLoopJoinExec" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.NestedLoopJoinExec.html" class="struct" title="struct datafusion::physical_plan::joins::NestedLoopJoinExec">NestedLoopJoinExec</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#impl-ExecutionPlan-for-SortMergeJoinExec" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.SortMergeJoinExec.html" class="struct" title="struct datafusion::physical_plan::joins::SortMergeJoinExec">SortMergeJoinExec</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#impl-ExecutionPlan-for-SymmetricHashJoinExec" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.SymmetricHashJoinExec.html" class="struct" title="struct datafusion::physical_plan::joins::SymmetricHashJoinExec">SymmetricHashJoinExec</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#impl-ExecutionPlan-for-GlobalLimitExec" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/limit/struct.GlobalLimitExec.html" class="struct" title="struct datafusion::physical_plan::limit::GlobalLimitExec">GlobalLimitExec</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#impl-ExecutionPlan-for-LocalLimitExec" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/limit/struct.LocalLimitExec.html" class="struct" title="struct datafusion::physical_plan::limit::LocalLimitExec">LocalLimitExec</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#impl-ExecutionPlan-for-LazyMemoryExec" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/memory/struct.LazyMemoryExec.html" class="struct" title="struct datafusion::physical_plan::memory::LazyMemoryExec">LazyMemoryExec</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#impl-ExecutionPlan-for-PlaceholderRowExec" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/placeholder_row/struct.PlaceholderRowExec.html" class="struct" title="struct datafusion::physical_plan::placeholder_row::PlaceholderRowExec">PlaceholderRowExec</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#impl-ExecutionPlan-for-ProjectionExec" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/projection/struct.ProjectionExec.html" class="struct" title="struct datafusion::physical_plan::projection::ProjectionExec">ProjectionExec</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#impl-ExecutionPlan-for-RecursiveQueryExec" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/recursive_query/struct.RecursiveQueryExec.html" class="struct" title="struct datafusion::physical_plan::recursive_query::RecursiveQueryExec">RecursiveQueryExec</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#impl-ExecutionPlan-for-RepartitionExec" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/repartition/struct.RepartitionExec.html" class="struct" title="struct datafusion::physical_plan::repartition::RepartitionExec">RepartitionExec</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#impl-ExecutionPlan-for-PartialSortExec" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/sorts/partial_sort/struct.PartialSortExec.html" class="struct" title="struct datafusion::physical_plan::sorts::partial_sort::PartialSortExec">PartialSortExec</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#impl-ExecutionPlan-for-SortExec" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/sorts/sort/struct.SortExec.html" class="struct" title="struct datafusion::physical_plan::sorts::sort::SortExec">SortExec</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#impl-ExecutionPlan-for-SortPreservingMergeExec" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/sorts/sort_preserving_merge/struct.SortPreservingMergeExec.html" class="struct" title="struct datafusion::physical_plan::sorts::sort_preserving_merge::SortPreservingMergeExec">SortPreservingMergeExec</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#impl-ExecutionPlan-for-StreamingTableExec" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/streaming/struct.StreamingTableExec.html" class="struct" title="struct datafusion::physical_plan::streaming::StreamingTableExec">StreamingTableExec</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#impl-ExecutionPlan-for-BarrierExec" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/test/exec/struct.BarrierExec.html" class="struct" title="struct datafusion::physical_plan::test::exec::BarrierExec">BarrierExec</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#impl-ExecutionPlan-for-BlockingExec" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/test/exec/struct.BlockingExec.html" class="struct" title="struct datafusion::physical_plan::test::exec::BlockingExec">BlockingExec</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#impl-ExecutionPlan-for-ErrorExec" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/test/exec/struct.ErrorExec.html" class="struct" title="struct datafusion::physical_plan::test::exec::ErrorExec">ErrorExec</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#impl-ExecutionPlan-for-MockExec" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/test/exec/struct.MockExec.html" class="struct" title="struct datafusion::physical_plan::test::exec::MockExec">MockExec</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#impl-ExecutionPlan-for-PanicExec" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/test/exec/struct.PanicExec.html" class="struct" title="struct datafusion::physical_plan::test::exec::PanicExec">PanicExec</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#impl-ExecutionPlan-for-StatisticsExec" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/test/exec/struct.StatisticsExec.html" class="struct" title="struct datafusion::physical_plan::test::exec::StatisticsExec">StatisticsExec</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#impl-ExecutionPlan-for-TestMemoryExec" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/test/struct.TestMemoryExec.html" class="struct" title="struct datafusion::physical_plan::test::TestMemoryExec">TestMemoryExec</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#impl-ExecutionPlan-for-InterleaveExec" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/union/struct.InterleaveExec.html" class="struct" title="struct datafusion::physical_plan::union::InterleaveExec">InterleaveExec</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#impl-ExecutionPlan-for-UnionExec" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/union/struct.UnionExec.html" class="struct" title="struct datafusion::physical_plan::union::UnionExec">UnionExec</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#impl-ExecutionPlan-for-UnnestExec" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/unnest/struct.UnnestExec.html" class="struct" title="struct datafusion::physical_plan::unnest::UnnestExec">UnnestExec</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#impl-ExecutionPlan-for-BoundedWindowAggExec" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/windows/struct.BoundedWindowAggExec.html" class="struct" title="struct datafusion::physical_plan::windows::BoundedWindowAggExec">BoundedWindowAggExec</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#impl-ExecutionPlan-for-WindowAggExec" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/windows/struct.WindowAggExec.html" class="struct" title="struct datafusion::physical_plan::windows::WindowAggExec">WindowAggExec</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#impl-ExecutionPlan-for-WorkTableExec" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/work_table/struct.WorkTableExec.html" class="struct" title="struct datafusion::physical_plan::work_table::WorkTableExec">WorkTableExec</a>
