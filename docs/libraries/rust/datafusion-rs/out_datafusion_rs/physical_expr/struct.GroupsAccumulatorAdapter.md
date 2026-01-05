# Struct GroupsAccumulatorAdapter Copy item path

<a href="https://docs.rs/datafusion-functions-aggregate-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_functions_aggregate_common/aggregate/groups_accumulator.rs.html#92" class="src">Source</a>

``` rust
pub struct GroupsAccumulatorAdapter { /* private fields */ }
```

Expand description

An adapter that implements [`GroupsAccumulator`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.GroupsAccumulator.html "trait datafusion::logical_expr::GroupsAccumulator") for any [`Accumulator`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Accumulator.html "trait datafusion::logical_expr::Accumulator")

While [`Accumulator`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Accumulator.html "trait datafusion::logical_expr::Accumulator") are simpler to implement and can support more general calculations (like retractable window functions), they are not as fast as a specialized `GroupsAccumulator`. This interface bridges the gap so the group by operator only operates in terms of [`Accumulator`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Accumulator.html "trait datafusion::logical_expr::Accumulator").

Internally, this adapter creates a new [`Accumulator`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Accumulator.html "trait datafusion::logical_expr::Accumulator") for each group which stores the state for that group. This both requires an allocation for each Accumulator, internal indices, as well as whatever internal allocations the Accumulator itself requires.

For example, a `MinAccumulator` that computes the minimum string value with a [`ScalarValue::Utf8`](https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html#variant.Utf8 "variant datafusion::scalar::ScalarValue::Utf8"). That will require at least two allocations per group (one for the `MinAccumulator` and one for the `ScalarValue::Utf8`).

``` text
                      ┌─────────────────────────────────┐
                      │MinAccumulator {                 │
               ┌─────▶│ min: ScalarValue::Utf8("A")     │───────┐
               │      │}                                │       │
               │      └─────────────────────────────────┘       └───────▶   "A"
   ┌─────┐     │      ┌─────────────────────────────────┐
   │  0  │─────┘      │MinAccumulator {                 │
   ├─────┤     ┌─────▶│ min: ScalarValue::Utf8("Z")     │───────────────▶   "Z"
   │  1  │─────┘      │}                                │
   └─────┘            └─────────────────────────────────┘                   ...
     ...                 ...
   ┌─────┐            ┌────────────────────────────────┐
   │ N-2 │            │MinAccumulator {                │
   ├─────┤            │  min: ScalarValue::Utf8("A")   │────────────────▶   "A"
   │ N-1 │─────┐      │}                               │
   └─────┘     │      └────────────────────────────────┘
               │      ┌────────────────────────────────┐        ┌───────▶   "Q"
               │      │MinAccumulator {                │        │
               └─────▶│  min: ScalarValue::Utf8("Q")   │────────┘
                      │}                               │
                      └────────────────────────────────┘


 Logical group         Current Min/Max value for that group stored
    number             as a ScalarValue which points to an
                       individually allocated String
```

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.GroupsAccumulatorAdapter.html#optimizations" class="doc-anchor">§</a>Optimizations

The adapter minimizes the number of calls to [`Accumulator::update_batch`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Accumulator.html#tymethod.update_batch "method datafusion::logical_expr::Accumulator::update_batch") by first collecting the input rows for each group into a contiguous array using [`compute::take`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/fn.take.html "fn datafusion::common::arrow::compute::take")

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.GroupsAccumulatorAdapter.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.GroupsAccumulatorAdapter.html#impl-GroupsAccumulatorAdapter" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.GroupsAccumulatorAdapter.html" class="struct" title="struct datafusion::physical_expr::GroupsAccumulatorAdapter">GroupsAccumulatorAdapter</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.GroupsAccumulatorAdapter.html#method.new" class="fn">new</a>\<F\>(factory: F) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.GroupsAccumulatorAdapter.html" class="struct" title="struct datafusion::physical_expr::GroupsAccumulatorAdapter">GroupsAccumulatorAdapter</a>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>() -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Accumulator.html" class="trait" title="trait datafusion::logical_expr::Accumulator">Accumulator</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'static,

Create a new adapter that will create a new [`Accumulator`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Accumulator.html "trait datafusion::logical_expr::Accumulator") for each group, using the specified factory function

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.GroupsAccumulatorAdapter.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.GroupsAccumulatorAdapter.html#impl-GroupsAccumulator-for-GroupsAccumulatorAdapter" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.GroupsAccumulator.html" class="trait" title="trait datafusion::logical_expr::GroupsAccumulator">GroupsAccumulator</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.GroupsAccumulatorAdapter.html" class="struct" title="struct datafusion::physical_expr::GroupsAccumulatorAdapter">GroupsAccumulatorAdapter</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.GroupsAccumulatorAdapter.html#method.update_batch" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.GroupsAccumulator.html#tymethod.update_batch" class="fn">update_batch</a>( &mut self, values: &\[<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>\>\], group_indices: &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\], opt_filter: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.BooleanArray.html" class="struct" title="struct datafusion::common::arrow::array::BooleanArray">BooleanArray</a>\>, total_num_groups: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Updates the accumulator’s state from its arguments, encoded as a vector of [`ArrayRef`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/type.ArrayRef.html "type datafusion::common::arrow::array::ArrayRef")s. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.GroupsAccumulator.html#tymethod.update_batch)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.GroupsAccumulatorAdapter.html#method.evaluate" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.GroupsAccumulator.html#tymethod.evaluate" class="fn">evaluate</a>( &mut self, emit_to: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.EmitTo.html" class="enum" title="enum datafusion::logical_expr::EmitTo">EmitTo</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Returns the final aggregate value for each group as a single `RecordBatch`, resetting the internal state. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.GroupsAccumulator.html#tymethod.evaluate)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.GroupsAccumulatorAdapter.html#method.state" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.GroupsAccumulator.html#tymethod.state" class="fn">state</a>( &mut self, emit_to: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.EmitTo.html" class="enum" title="enum datafusion::logical_expr::EmitTo">EmitTo</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>\>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Returns the intermediate aggregate state for this accumulator, used for multi-phase grouping, resetting its internal state. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.GroupsAccumulator.html#tymethod.state)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.GroupsAccumulatorAdapter.html#method.merge_batch" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.GroupsAccumulator.html#tymethod.merge_batch" class="fn">merge_batch</a>( &mut self, values: &\[<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>\>\], group_indices: &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\], opt_filter: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.BooleanArray.html" class="struct" title="struct datafusion::common::arrow::array::BooleanArray">BooleanArray</a>\>, total_num_groups: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Merges intermediate state (the output from [`Self::state`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.GroupsAccumulator.html#tymethod.state "method datafusion_expr_common::groups_accumulator::GroupsAccumulator::state::state")) into this accumulator’s current state. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.GroupsAccumulator.html#tymethod.merge_batch)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.GroupsAccumulatorAdapter.html#method.size" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.GroupsAccumulator.html#tymethod.size" class="fn">size</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Amount of memory used to store the state of this accumulator, in bytes. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.GroupsAccumulator.html#tymethod.size)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.GroupsAccumulatorAdapter.html#method.convert_to_state" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.GroupsAccumulator.html#method.convert_to_state" class="fn">convert_to_state</a>( &self, values: &\[<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>\>\], opt_filter: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.BooleanArray.html" class="struct" title="struct datafusion::common::arrow::array::BooleanArray">BooleanArray</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>\>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Converts an input batch directly to the intermediate aggregate state. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.GroupsAccumulator.html#method.convert_to_state)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.GroupsAccumulatorAdapter.html#method.supports_convert_to_state" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.GroupsAccumulator.html#method.supports_convert_to_state" class="fn">supports_convert_to_state</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns `true` if [`Self::convert_to_state`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.GroupsAccumulator.html#method.convert_to_state "method datafusion_expr_common::groups_accumulator::GroupsAccumulator::convert_to_state::convert_to_state") is implemented to support intermediate aggregate state conversion.

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.GroupsAccumulatorAdapter.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.GroupsAccumulatorAdapter.html#blanket-implementations" class="anchor">§</a>
