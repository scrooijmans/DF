# Struct LazyMemoryExec Copy item path

<a href="https://docs.rs/datafusion-physical-plan/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_plan/memory.rs.html#153" class="src">Source</a>

``` rust
pub struct LazyMemoryExec { /* private fields */ }
```

Expand description

Execution plan for lazy in-memory batches of data

This plan generates output batches lazily, it doesn’t have to buffer all batches in memory up front (compared to `MemorySourceConfig`), thus consuming constant memory.

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/memory/struct.LazyMemoryExec.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/memory/struct.LazyMemoryExec.html#impl-LazyMemoryExec" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/memory/struct.LazyMemoryExec.html" class="struct" title="struct datafusion::physical_plan::memory::LazyMemoryExec">LazyMemoryExec</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/memory/struct.LazyMemoryExec.html#method.try_new" class="fn">try_new</a>( schema: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>\>, generators: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/lock_api/0.4.13/x86_64-unknown-linux-gnu/lock_api/rwlock/struct.RwLock.html" class="struct" title="struct lock_api::rwlock::RwLock">RwLock</a>\<<a href="https://docs.rs/parking_lot/0.12.4/x86_64-unknown-linux-gnu/parking_lot/raw_rwlock/struct.RawRwLock.html" class="struct" title="struct parking_lot::raw_rwlock::RawRwLock">RawRwLock</a>, dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/memory/trait.LazyBatchGenerator.html" class="trait" title="trait datafusion::physical_plan::memory::LazyBatchGenerator">LazyBatchGenerator</a>\>\>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/memory/struct.LazyMemoryExec.html" class="struct" title="struct datafusion::physical_plan::memory::LazyMemoryExec">LazyMemoryExec</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Create a new lazy memory execution plan

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/memory/struct.LazyMemoryExec.html#method.try_set_partitioning" class="fn">try_set_partitioning</a>( &mut self, partitioning: <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.Partitioning.html" class="enum" title="enum datafusion::physical_expr::Partitioning">Partitioning</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/memory/struct.LazyMemoryExec.html#method.add_ordering" class="fn">add_ordering</a>( &mut self, ordering: impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.PhysicalSortExpr.html" class="struct" title="struct datafusion::physical_expr::PhysicalSortExpr">PhysicalSortExpr</a>\>, )

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/memory/struct.LazyMemoryExec.html#method.generators" class="fn">generators</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/lock_api/0.4.13/x86_64-unknown-linux-gnu/lock_api/rwlock/struct.RwLock.html" class="struct" title="struct lock_api::rwlock::RwLock">RwLock</a>\<<a href="https://docs.rs/parking_lot/0.12.4/x86_64-unknown-linux-gnu/parking_lot/raw_rwlock/struct.RawRwLock.html" class="struct" title="struct parking_lot::raw_rwlock::RawRwLock">RawRwLock</a>, dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/memory/trait.LazyBatchGenerator.html" class="trait" title="trait datafusion::physical_plan::memory::LazyBatchGenerator">LazyBatchGenerator</a>\>\>\>

Get the batch generators

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/memory/struct.LazyMemoryExec.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/memory/struct.LazyMemoryExec.html#impl-Debug-for-LazyMemoryExec" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/memory/struct.LazyMemoryExec.html" class="struct" title="struct datafusion::physical_plan::memory::LazyMemoryExec">LazyMemoryExec</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/memory/struct.LazyMemoryExec.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/memory/struct.LazyMemoryExec.html#impl-DisplayAs-for-LazyMemoryExec" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.DisplayAs.html" class="trait" title="trait datafusion::physical_plan::DisplayAs">DisplayAs</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/memory/struct.LazyMemoryExec.html" class="struct" title="struct datafusion::physical_plan::memory::LazyMemoryExec">LazyMemoryExec</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/memory/struct.LazyMemoryExec.html#method.fmt_as" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.DisplayAs.html#tymethod.fmt_as" class="fn">fmt_as</a>( &self, t: <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/enum.DisplayFormatType.html" class="enum" title="enum datafusion::physical_plan::DisplayFormatType">DisplayFormatType</a>, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Format according to `DisplayFormatType`, used when verbose representation looks different from the default one [Read more](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.DisplayAs.html#tymethod.fmt_as)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/memory/struct.LazyMemoryExec.html#impl-ExecutionPlan-for-LazyMemoryExec" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/memory/struct.LazyMemoryExec.html" class="struct" title="struct datafusion::physical_plan::memory::LazyMemoryExec">LazyMemoryExec</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/memory/struct.LazyMemoryExec.html#method.name" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#tymethod.name" class="fn">name</a>(&self) -\> &'static <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

Short name for the ExecutionPlan, such as ‘DataSourceExec’. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#tymethod.name)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/memory/struct.LazyMemoryExec.html#method.as_any" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#tymethod.as_any" class="fn">as_any</a>(&self) -\> &(dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + 'static)

Returns the execution plan as [`Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") so that it can be downcast to a specific implementation.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/memory/struct.LazyMemoryExec.html#method.schema" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.schema" class="fn">schema</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>\>

Get the schema for this execution plan

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/memory/struct.LazyMemoryExec.html#method.properties" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#tymethod.properties" class="fn">properties</a>(&self) -\> &<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.PlanProperties.html" class="struct" title="struct datafusion::physical_plan::PlanProperties">PlanProperties</a>

Return properties of the output of the `ExecutionPlan`, such as output ordering(s), partitioning information etc. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#tymethod.properties)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/memory/struct.LazyMemoryExec.html#method.children" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#tymethod.children" class="fn">children</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<&<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a>\>\>

Get a list of children `ExecutionPlan`s that act as inputs to this plan. The returned list will be empty for leaf nodes such as scans, will contain a single value for unary nodes, or two values for binary nodes (such as joins).

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/memory/struct.LazyMemoryExec.html#method.with_new_children" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#tymethod.with_new_children" class="fn">with_new_children</a>( self: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/memory/struct.LazyMemoryExec.html" class="struct" title="struct datafusion::physical_plan::memory::LazyMemoryExec">LazyMemoryExec</a>\>, children: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a>\>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Returns a new `ExecutionPlan` where all existing children were replaced by the `children`, in order

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/memory/struct.LazyMemoryExec.html#method.execute" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#tymethod.execute" class="fn">execute</a>( &self, partition: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, \_context: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/struct.TaskContext.html" class="struct" title="struct datafusion::execution::TaskContext">TaskContext</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.RecordBatchStream.html" class="trait" title="trait datafusion::execution::RecordBatchStream">RecordBatchStream</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html" class="struct" title="struct datafusion::common::arrow::array::RecordBatch">RecordBatch</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>\>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Begin execution of `partition`, returning a [`Stream`](https://docs.rs/futures-core/0.3.31/x86_64-unknown-linux-gnu/futures_core/stream/trait.Stream.html "trait futures_core::stream::Stream") of [`RecordBatch`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html "struct datafusion::common::arrow::array::RecordBatch")es. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#tymethod.execute)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/memory/struct.LazyMemoryExec.html#method.metrics" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.metrics" class="fn">metrics</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.MetricsSet.html" class="struct" title="struct datafusion::physical_plan::metrics::MetricsSet">MetricsSet</a>\>

Return a snapshot of the set of [`Metric`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.Metric.html "struct datafusion::physical_plan::Metric")s for this [`ExecutionPlan`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html "trait datafusion::physical_plan::ExecutionPlan"). If no `Metric`s are available, return None. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.metrics)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/memory/struct.LazyMemoryExec.html#method.statistics" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.statistics" class="fn">statistics</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Statistics.html" class="struct" title="struct datafusion::common::Statistics">Statistics</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

👎Deprecated since 48.0.0: Use `partition_statistics` method instead

Returns statistics for this `ExecutionPlan` node. If statistics are not available, should return [`Statistics::new_unknown`](https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Statistics.html#method.new_unknown "associated function datafusion::common::Statistics::new_unknown") (the default), not an error. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.statistics)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/memory/struct.LazyMemoryExec.html#method.static_name" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.static_name" class="fn">static_name</a>() -\> &'static <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Short name for the ExecutionPlan, such as ‘DataSourceExec’. Like [`name`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#tymethod.name "method datafusion::physical_plan::ExecutionPlan::name") but can be called without an instance.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/memory/struct.LazyMemoryExec.html#method.check_invariants" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.check_invariants" class="fn">check_invariants</a>(&self, check: <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/enum.InvariantLevel.html" class="enum" title="enum datafusion::physical_plan::execution_plan::InvariantLevel">InvariantLevel</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Returns an error if this individual node does not conform to its invariants. These invariants are typically only checked in debug mode. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.check_invariants)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/memory/struct.LazyMemoryExec.html#method.required_input_distribution" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.required_input_distribution" class="fn">required_input_distribution</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.Distribution.html" class="enum" title="enum datafusion::physical_expr::Distribution">Distribution</a>\>

Specifies the data distribution requirements for all the children for this `ExecutionPlan`, By default it’s \[[Distribution::UnspecifiedDistribution](https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.Distribution.html#variant.UnspecifiedDistribution "variant datafusion::physical_expr::Distribution::UnspecifiedDistribution")\] for each child,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/memory/struct.LazyMemoryExec.html#method.required_input_ordering" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.required_input_ordering" class="fn">required_input_ordering</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.OrderingRequirements.html" class="enum" title="enum datafusion::physical_expr::OrderingRequirements">OrderingRequirements</a>\>\>

Specifies the ordering required for all of the children of this `ExecutionPlan`. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.required_input_ordering)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/memory/struct.LazyMemoryExec.html#method.maintains_input_order" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.maintains_input_order" class="fn">maintains_input_order</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>\>

Returns `false` if this `ExecutionPlan`’s implementation may reorder rows within or between partitions. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.maintains_input_order)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/memory/struct.LazyMemoryExec.html#method.benefits_from_input_partitioning" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.benefits_from_input_partitioning" class="fn">benefits_from_input_partitioning</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>\>

Specifies whether the `ExecutionPlan` benefits from increased parallelization at its input for each child. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.benefits_from_input_partitioning)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/memory/struct.LazyMemoryExec.html#method.reset_state" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.reset_state" class="fn">reset_state</a>( self: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<Self\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Reset any internal state within this [`ExecutionPlan`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html "trait datafusion::physical_plan::ExecutionPlan"). [Read more](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.reset_state)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/memory/struct.LazyMemoryExec.html#method.repartitioned" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.repartitioned" class="fn">repartitioned</a>( &self, \_target_partitions: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, \_config: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigOptions.html" class="struct" title="struct datafusion::config::ConfigOptions">ConfigOptions</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a>\>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

If supported, attempt to increase the partitioning of this `ExecutionPlan` to produce `target_partitions` partitions. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.repartitioned)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/memory/struct.LazyMemoryExec.html#method.partition_statistics" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.partition_statistics" class="fn">partition_statistics</a>( &self, partition: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Statistics.html" class="struct" title="struct datafusion::common::Statistics">Statistics</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Returns statistics for a specific partition of this `ExecutionPlan` node. If statistics are not available, should return [`Statistics::new_unknown`](https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Statistics.html#method.new_unknown "associated function datafusion::common::Statistics::new_unknown") (the default), not an error. If `partition` is `None`, it returns statistics for the entire plan.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/memory/struct.LazyMemoryExec.html#method.supports_limit_pushdown" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.supports_limit_pushdown" class="fn">supports_limit_pushdown</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns `true` if a limit can be safely pushed down through this `ExecutionPlan` node. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.supports_limit_pushdown)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/memory/struct.LazyMemoryExec.html#method.with_fetch" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.with_fetch" class="fn">with_fetch</a>(&self, \_limit: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a>\>\>

Returns a fetching variant of this `ExecutionPlan` node, if it supports fetch limits. Returns `None` otherwise.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/memory/struct.LazyMemoryExec.html#method.fetch" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.fetch" class="fn">fetch</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>

Gets the fetch count for the operator, `None` means there is no fetch.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/memory/struct.LazyMemoryExec.html#method.cardinality_effect" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.cardinality_effect" class="fn">cardinality_effect</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/enum.CardinalityEffect.html" class="enum" title="enum datafusion::physical_plan::execution_plan::CardinalityEffect">CardinalityEffect</a>

Gets the effect on cardinality, if known

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/memory/struct.LazyMemoryExec.html#method.try_swapping_with_projection" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.try_swapping_with_projection" class="fn">try_swapping_with_projection</a>( &self, \_projection: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/projection/struct.ProjectionExec.html" class="struct" title="struct datafusion::physical_plan::projection::ProjectionExec">ProjectionExec</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a>\>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Attempts to push down the given projection into the input of this `ExecutionPlan`. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.try_swapping_with_projection)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/memory/struct.LazyMemoryExec.html#method.gather_filters_for_pushdown" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.gather_filters_for_pushdown" class="fn">gather_filters_for_pushdown</a>( &self, \_phase: <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter_pushdown/enum.FilterPushdownPhase.html" class="enum" title="enum datafusion::physical_plan::filter_pushdown::FilterPushdownPhase">FilterPushdownPhase</a>, parent_filters: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html" class="trait" title="trait datafusion::physical_expr::PhysicalExpr">PhysicalExpr</a>\>\>, \_config: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigOptions.html" class="struct" title="struct datafusion::config::ConfigOptions">ConfigOptions</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter_pushdown/struct.FilterDescription.html" class="struct" title="struct datafusion::physical_plan::filter_pushdown::FilterDescription">FilterDescription</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Collect filters that this node can push down to its children. Filters that are being pushed down from parents are passed in, and the node may generate additional filters to push down. For example, given the plan FilterExec -\> HashJoinExec -\> DataSourceExec, what will happen is that we recurse down the plan calling `ExecutionPlan::gather_filters_for_pushdown`: [Read more](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.gather_filters_for_pushdown)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/memory/struct.LazyMemoryExec.html#method.handle_child_pushdown_result" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.handle_child_pushdown_result" class="fn">handle_child_pushdown_result</a>( &self, \_phase: <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter_pushdown/enum.FilterPushdownPhase.html" class="enum" title="enum datafusion::physical_plan::filter_pushdown::FilterPushdownPhase">FilterPushdownPhase</a>, child_pushdown_result: <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter_pushdown/struct.ChildPushdownResult.html" class="struct" title="struct datafusion::physical_plan::filter_pushdown::ChildPushdownResult">ChildPushdownResult</a>, \_config: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigOptions.html" class="struct" title="struct datafusion::config::ConfigOptions">ConfigOptions</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter_pushdown/struct.FilterPushdownPropagation.html" class="struct" title="struct datafusion::physical_plan::filter_pushdown::FilterPushdownPropagation">FilterPushdownPropagation</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a>\>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Handle the result of a child pushdown. This method is called as we recurse back up the plan tree after pushing filters down to child nodes via [`ExecutionPlan::gather_filters_for_pushdown`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.gather_filters_for_pushdown "method datafusion::physical_plan::ExecutionPlan::gather_filters_for_pushdown"). It allows the current node to process the results of filter pushdown from its children, deciding whether to absorb filters, modify the plan, or pass filters back up to its parent. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.handle_child_pushdown_result)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/memory/struct.LazyMemoryExec.html#method.with_new_state" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.with_new_state" class="fn">with_new_state</a>( &self, \_state: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a>\>\>

Injects arbitrary run-time state into this execution plan, returning a new plan instance that incorporates that state *if* it is relevant to the concrete node implementation. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.with_new_state)

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/memory/struct.LazyMemoryExec.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/memory/struct.LazyMemoryExec.html#blanket-implementations" class="anchor">§</a>
