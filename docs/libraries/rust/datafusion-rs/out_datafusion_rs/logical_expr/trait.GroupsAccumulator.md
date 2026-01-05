# Trait GroupsAccumulator Copy item path

<a href="https://docs.rs/datafusion-expr-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr_common/groups_accumulator.rs.html#108" class="src">Source</a>

``` rust
pub trait GroupsAccumulator: Send {
    // Required methods
    fn update_batch(
        &mut self,
        values: &[Arc<dyn Array>],
        group_indices: &[usize],
        opt_filter: Option<&BooleanArray>,
        total_num_groups: usize,
    ) -> Result<(), DataFusionError>;
    fn evaluate(
        &mut self,
        emit_to: EmitTo,
    ) -> Result<Arc<dyn Array>, DataFusionError>;
    fn state(
        &mut self,
        emit_to: EmitTo,
    ) -> Result<Vec<Arc<dyn Array>>, DataFusionError>;
    fn merge_batch(
        &mut self,
        values: &[Arc<dyn Array>],
        group_indices: &[usize],
        opt_filter: Option<&BooleanArray>,
        total_num_groups: usize,
    ) -> Result<(), DataFusionError>;
    fn size(&self) -> usize;

    // Provided methods
    fn convert_to_state(
        &self,
        _values: &[Arc<dyn Array>],
        _opt_filter: Option<&BooleanArray>,
    ) -> Result<Vec<Arc<dyn Array>>, DataFusionError> { ... }
    fn supports_convert_to_state(&self) -> bool { ... }
}
```

Expand description

`GroupsAccumulator` implements a single aggregate (e.g. AVG) and stores the state for *all* groups internally.

Logically, a [`GroupsAccumulator`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.GroupsAccumulator.html "trait datafusion::logical_expr::GroupsAccumulator") stores a mapping from each group index to the state of the aggregate for that group. For example an implementation for `min` might look like

``` text
   ┌─────┐
   │  0  │───────────▶   100
   ├─────┤
   │  1  │───────────▶   200
   └─────┘
     ...                 ...
   ┌─────┐
   │ N-2 │───────────▶    50
   ├─────┤
   │ N-1 │───────────▶   200
   └─────┘


 Logical group      Current Min
    number          value for that
                    group
```

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.GroupsAccumulator.html#notes-on-implementing-groupsaccumulator" class="doc-anchor">§</a>Notes on Implementing `GroupsAccumulator`

All aggregates must first implement the simpler [`Accumulator`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Accumulator.html "trait datafusion::logical_expr::Accumulator") trait, which handles state for a single group. Implementing `GroupsAccumulator` is optional and is harder to implement than `Accumulator`, but can be much faster for queries with many group values. See the [Aggregating Millions of Groups Fast blog](https://arrow.apache.org/blog/2023/08/05/datafusion_fast_grouping/) for more background.

[`NullState`](https://docs.rs/datafusion/latest/datafusion/physical_expr/struct.NullState.html) can help keep the state for groups that have not seen any values and produce the correct output for those groups.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.GroupsAccumulator.html#details" class="doc-anchor">§</a>Details

Each group is assigned a `group_index` by the hash table and each accumulator manages the specific state, one per `group_index`.

`group_index`es are contiguous (there aren’t gaps), and thus it is expected that each `GroupsAccumulator` will use something like `Vec<..>` to store the group states.

## Required Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.GroupsAccumulator.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.GroupsAccumulator.html#tymethod.update_batch" class="fn">update_batch</a>( &mut self, values: &\[<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>\>\], group_indices: &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\], opt_filter: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.BooleanArray.html" class="struct" title="struct datafusion::common::arrow::array::BooleanArray">BooleanArray</a>\>, total_num_groups: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Updates the accumulator’s state from its arguments, encoded as a vector of [`ArrayRef`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/type.ArrayRef.html "type datafusion::common::arrow::array::ArrayRef")s.

- `values`: the input arguments to the accumulator

- `group_indices`: The group indices to which each row in `values` belongs.

- `opt_filter`: if present, only update aggregate state using `values[i]` if `opt_filter[i]` is true

- `total_num_groups`: the number of groups (the largest group_index is thus `total_num_groups - 1`).

Note that subsequent calls to update_batch may have larger total_num_groups as new groups are seen.

See [`NullState`](https://docs.rs/datafusion/latest/datafusion/physical_expr/struct.NullState.html) to help keep the state for groups that have not seen any values and produce the correct output for those groups.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.GroupsAccumulator.html#tymethod.evaluate" class="fn">evaluate</a>( &mut self, emit_to: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.EmitTo.html" class="enum" title="enum datafusion::logical_expr::EmitTo">EmitTo</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Returns the final aggregate value for each group as a single `RecordBatch`, resetting the internal state.

The rows returned *must* be in group_index order: The value for group_index 0, followed by 1, etc. Any group_index that did not have values, should be null.

For example, a `SUM` accumulator maintains a running sum for each group, and `evaluate` will produce that running sum as its output for all groups, in group_index order

If `emit_to` is [`EmitTo::All`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.EmitTo.html#variant.All "variant datafusion::logical_expr::EmitTo::All"), the accumulator should return all groups and release / reset its internal state equivalent to when it was first created.

If `emit_to` is [`EmitTo::First`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.EmitTo.html#variant.First "variant datafusion::logical_expr::EmitTo::First"), only the first `n` groups should be emitted and the state for those first groups removed. State for the remaining groups must be retained for future use. The group_indices on subsequent calls to `update_batch` or `merge_batch` will be shifted down by `n`. See [`EmitTo::First`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.EmitTo.html#variant.First "variant datafusion::logical_expr::EmitTo::First") for more details.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.GroupsAccumulator.html#tymethod.state" class="fn">state</a>( &mut self, emit_to: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.EmitTo.html" class="enum" title="enum datafusion::logical_expr::EmitTo">EmitTo</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>\>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Returns the intermediate aggregate state for this accumulator, used for multi-phase grouping, resetting its internal state.

See [`Accumulator::state`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Accumulator.html#tymethod.state "method datafusion::logical_expr::Accumulator::state") for more information on multi-phase aggregation.

For example, `AVG` might return two arrays: `SUM` and `COUNT` but the `MIN` aggregate would just return a single array.

Note more sophisticated internal state can be passed as single `StructArray` rather than multiple arrays.

See [`Self::evaluate`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.GroupsAccumulator.html#tymethod.evaluate "method datafusion_expr_common::groups_accumulator::GroupsAccumulator::evaluate::evaluate") for details on the required output order and `emit_to`.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.GroupsAccumulator.html#tymethod.merge_batch" class="fn">merge_batch</a>( &mut self, values: &\[<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>\>\], group_indices: &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\], opt_filter: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.BooleanArray.html" class="struct" title="struct datafusion::common::arrow::array::BooleanArray">BooleanArray</a>\>, total_num_groups: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Merges intermediate state (the output from [`Self::state`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.GroupsAccumulator.html#tymethod.state "method datafusion_expr_common::groups_accumulator::GroupsAccumulator::state::state")) into this accumulator’s current state.

For some aggregates (such as `SUM`), `merge_batch` is the same as `update_batch`, but for some aggregates (such as `COUNT`, where the partial counts must be summed) the operations differ. See [`Self::state`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.GroupsAccumulator.html#tymethod.state "method datafusion_expr_common::groups_accumulator::GroupsAccumulator::state::state") for more details on how state is used and merged.

- `values`: arrays produced from previously calling `state` on other accumulators.

Other arguments are the same as for [`Self::update_batch`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.GroupsAccumulator.html#tymethod.update_batch "method datafusion_expr_common::groups_accumulator::GroupsAccumulator::update_batch::update_batch").

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.GroupsAccumulator.html#tymethod.size" class="fn">size</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Amount of memory used to store the state of this accumulator, in bytes.

This function is called once per batch, so it should be `O(n)` to compute, not `O(num_groups)`

## Provided Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.GroupsAccumulator.html#provided-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.GroupsAccumulator.html#method.convert_to_state" class="fn">convert_to_state</a>( &self, \_values: &\[<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>\>\], \_opt_filter: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.BooleanArray.html" class="struct" title="struct datafusion::common::arrow::array::BooleanArray">BooleanArray</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>\>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Converts an input batch directly to the intermediate aggregate state.

This is the equivalent of treating each input row as its own group. It is invoked when the Partial phase of a multi-phase aggregation is not reducing the cardinality enough to warrant spending more effort on pre-aggregation (see `Background` section below), and switches to passing intermediate state directly on to the next aggregation phase.

Examples:

- `COUNT`: an array of 1s for each row in the input batch.
- `SUM/MIN/MAX`: the input values themselves.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.GroupsAccumulator.html#arguments" class="doc-anchor">§</a>Arguments

- `values`: the input arguments to the accumulator
- `opt_filter`: if present, any row where `opt_filter[i]` is false should be ignored

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.GroupsAccumulator.html#background" class="doc-anchor">§</a>Background

In a multi-phase aggregation (see [`Accumulator::state`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Accumulator.html#tymethod.state "method datafusion::logical_expr::Accumulator::state")), the initial Partial phase reduces the cardinality of the input data as soon as possible in the plan.

This strategy is very effective for queries with a small number of groups, as most of the data is aggregated immediately and only a small amount of data must be repartitioned (see [`Accumulator::state`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Accumulator.html#tymethod.state "method datafusion::logical_expr::Accumulator::state") for background)

However, for queries with a large number of groups, the Partial phase often does not reduce the cardinality enough to warrant the memory and CPU cost of actually performing the aggregation. For such cases, the HashAggregate operator will dynamically switch to passing intermediate state directly to the next aggregation phase with minimal processing using this method.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.GroupsAccumulator.html#method.supports_convert_to_state" class="fn">supports_convert_to_state</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns `true` if [`Self::convert_to_state`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.GroupsAccumulator.html#method.convert_to_state "method datafusion_expr_common::groups_accumulator::GroupsAccumulator::convert_to_state::convert_to_state") is implemented to support intermediate aggregate state conversion.

## Implementations on Foreign Types<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.GroupsAccumulator.html#foreign-impls" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.GroupsAccumulator.html#impl-GroupsAccumulator-for-BooleanGroupsAccumulator%3CF%3E" class="anchor">§</a>

### impl\<F\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.GroupsAccumulator.html" class="trait" title="trait datafusion::logical_expr::GroupsAccumulator">GroupsAccumulator</a> for <a href="https://docs.rs/datafusion-functions-aggregate-common/50.2.0/x86_64-unknown-linux-gnu/datafusion_functions_aggregate_common/aggregate/groups_accumulator/bool_op/struct.BooleanGroupsAccumulator.html" class="struct" title="struct datafusion_functions_aggregate_common::aggregate::groups_accumulator::bool_op::BooleanGroupsAccumulator">BooleanGroupsAccumulator</a>\<F\>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.GroupsAccumulator.html#method.update_batch" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.GroupsAccumulator.html#tymethod.update_batch" class="fn">update_batch</a>( &mut self, values: &\[<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>\>\], group_indices: &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\], opt_filter: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.BooleanArray.html" class="struct" title="struct datafusion::common::arrow::array::BooleanArray">BooleanArray</a>\>, total_num_groups: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.GroupsAccumulator.html#method.evaluate" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.GroupsAccumulator.html#tymethod.evaluate" class="fn">evaluate</a>( &mut self, emit_to: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.EmitTo.html" class="enum" title="enum datafusion::logical_expr::EmitTo">EmitTo</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.GroupsAccumulator.html#method.state" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.GroupsAccumulator.html#tymethod.state" class="fn">state</a>( &mut self, emit_to: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.EmitTo.html" class="enum" title="enum datafusion::logical_expr::EmitTo">EmitTo</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>\>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.GroupsAccumulator.html#method.merge_batch" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.GroupsAccumulator.html#tymethod.merge_batch" class="fn">merge_batch</a>( &mut self, values: &\[<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>\>\], group_indices: &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\], opt_filter: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.BooleanArray.html" class="struct" title="struct datafusion::common::arrow::array::BooleanArray">BooleanArray</a>\>, total_num_groups: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.GroupsAccumulator.html#method.size" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.GroupsAccumulator.html#tymethod.size" class="fn">size</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.GroupsAccumulator.html#method.convert_to_state-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.GroupsAccumulator.html#method.convert_to_state" class="fn">convert_to_state</a>( &self, values: &\[<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>\>\], opt_filter: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.BooleanArray.html" class="struct" title="struct datafusion::common::arrow::array::BooleanArray">BooleanArray</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>\>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.GroupsAccumulator.html#method.supports_convert_to_state-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.GroupsAccumulator.html#method.supports_convert_to_state" class="fn">supports_convert_to_state</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.GroupsAccumulator.html#impl-GroupsAccumulator-for-PrimitiveGroupsAccumulator%3CT,+F%3E" class="anchor">§</a>

### impl\<T, F\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.GroupsAccumulator.html" class="trait" title="trait datafusion::logical_expr::GroupsAccumulator">GroupsAccumulator</a> for <a href="https://docs.rs/datafusion-functions-aggregate-common/50.2.0/x86_64-unknown-linux-gnu/datafusion_functions_aggregate_common/aggregate/groups_accumulator/prim_op/struct.PrimitiveGroupsAccumulator.html" class="struct" title="struct datafusion_functions_aggregate_common::aggregate::groups_accumulator::prim_op::PrimitiveGroupsAccumulator">PrimitiveGroupsAccumulator</a>\<T, F\>

where T: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait datafusion::common::arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>, F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(&mut \<T as <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait datafusion::common::arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type datafusion::common::arrow::array::ArrowPrimitiveType::Native">Native</a>, \<T as <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait datafusion::common::arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type datafusion::common::arrow::array::ArrowPrimitiveType::Native">Native</a>) + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.GroupsAccumulator.html#method.convert_to_state-2" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.GroupsAccumulator.html#method.convert_to_state" class="fn">convert_to_state</a>( &self, values: &\[<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>\>\], opt_filter: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.BooleanArray.html" class="struct" title="struct datafusion::common::arrow::array::BooleanArray">BooleanArray</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>\>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Converts an input batch directly to a state batch

The state is:

- self.prim_fn for all non null, non filtered values
- null otherwise

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.GroupsAccumulator.html#method.update_batch-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.GroupsAccumulator.html#tymethod.update_batch" class="fn">update_batch</a>( &mut self, values: &\[<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>\>\], group_indices: &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\], opt_filter: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.BooleanArray.html" class="struct" title="struct datafusion::common::arrow::array::BooleanArray">BooleanArray</a>\>, total_num_groups: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.GroupsAccumulator.html#method.evaluate-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.GroupsAccumulator.html#tymethod.evaluate" class="fn">evaluate</a>( &mut self, emit_to: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.EmitTo.html" class="enum" title="enum datafusion::logical_expr::EmitTo">EmitTo</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.GroupsAccumulator.html#method.state-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.GroupsAccumulator.html#tymethod.state" class="fn">state</a>( &mut self, emit_to: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.EmitTo.html" class="enum" title="enum datafusion::logical_expr::EmitTo">EmitTo</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>\>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.GroupsAccumulator.html#method.merge_batch-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.GroupsAccumulator.html#tymethod.merge_batch" class="fn">merge_batch</a>( &mut self, values: &\[<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>\>\], group_indices: &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\], opt_filter: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.BooleanArray.html" class="struct" title="struct datafusion::common::arrow::array::BooleanArray">BooleanArray</a>\>, total_num_groups: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.GroupsAccumulator.html#method.supports_convert_to_state-2" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.GroupsAccumulator.html#method.supports_convert_to_state" class="fn">supports_convert_to_state</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.GroupsAccumulator.html#method.size-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.GroupsAccumulator.html#tymethod.size" class="fn">size</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

## Implementors<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.GroupsAccumulator.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.GroupsAccumulator.html#impl-GroupsAccumulator-for-CorrelationGroupsAccumulator" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.GroupsAccumulator.html" class="trait" title="trait datafusion::logical_expr::GroupsAccumulator">GroupsAccumulator</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/correlation/struct.CorrelationGroupsAccumulator.html" class="struct" title="struct datafusion::functions_aggregate::correlation::CorrelationGroupsAccumulator">CorrelationGroupsAccumulator</a>

GroupsAccumulator implementation for `corr(x, y)` that computes the Pearson correlation coefficient between two numeric columns.

Online algorithm for correlation:

r = (n \* sum_xy - sum_x \* sum_y) / sqrt((n \* sum_xx - sum_x^2) \* (n \* sum_yy - sum_y^2)) where: n = number of observations sum_x = sum of x values sum_y = sum of y values  
sum_xy = sum of (x \* y) sum_xx = sum of x^2 values sum_yy = sum of y^2 values

Reference: <https://en.wikipedia.org/wiki/Pearson_correlation_coefficient#For_a_sample>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.GroupsAccumulator.html#impl-GroupsAccumulator-for-StddevGroupsAccumulator" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.GroupsAccumulator.html" class="trait" title="trait datafusion::logical_expr::GroupsAccumulator">GroupsAccumulator</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/stddev/struct.StddevGroupsAccumulator.html" class="struct" title="struct datafusion::functions_aggregate::stddev::StddevGroupsAccumulator">StddevGroupsAccumulator</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.GroupsAccumulator.html#impl-GroupsAccumulator-for-VarianceGroupsAccumulator" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.GroupsAccumulator.html" class="trait" title="trait datafusion::logical_expr::GroupsAccumulator">GroupsAccumulator</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/variance/struct.VarianceGroupsAccumulator.html" class="struct" title="struct datafusion::functions_aggregate::variance::VarianceGroupsAccumulator">VarianceGroupsAccumulator</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.GroupsAccumulator.html#impl-GroupsAccumulator-for-GroupsAccumulatorAdapter" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.GroupsAccumulator.html" class="trait" title="trait datafusion::logical_expr::GroupsAccumulator">GroupsAccumulator</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.GroupsAccumulatorAdapter.html" class="struct" title="struct datafusion::physical_expr::GroupsAccumulatorAdapter">GroupsAccumulatorAdapter</a>
