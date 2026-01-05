# Trait Accumulator Copy item path

<a href="https://docs.rs/datafusion-expr-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr_common/accumulator.rs.html#51" class="src">Source</a>

``` rust
pub trait Accumulator:
    Send
    + Sync
    + Debug {
    // Required methods
    fn update_batch(
        &mut self,
        values: &[Arc<dyn Array>],
    ) -> Result<(), DataFusionError>;
    fn evaluate(&mut self) -> Result<ScalarValue, DataFusionError>;
    fn size(&self) -> usize;
    fn state(&mut self) -> Result<Vec<ScalarValue>, DataFusionError>;
    fn merge_batch(
        &mut self,
        states: &[Arc<dyn Array>],
    ) -> Result<(), DataFusionError>;

    // Provided methods
    fn retract_batch(
        &mut self,
        _values: &[Arc<dyn Array>],
    ) -> Result<(), DataFusionError> { ... }
    fn supports_retract_batch(&self) -> bool { ... }
}
```

Expand description

Tracks an aggregate function’s state.

`Accumulator`s are stateful objects that implement a single group. They aggregate values from multiple rows together into a final output aggregate.

\[`GroupsAccumulator]` is an additional more performant (but also complex) API that manages state for multiple groups at once.

An accumulator knows how to:

- update its state from inputs via [`update_batch`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Accumulator.html#tymethod.update_batch "method datafusion_expr_common::accumulator::Accumulator::update_batch::update_batch")

- compute the final value from its internal state via [`evaluate`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Accumulator.html#tymethod.evaluate "method datafusion_expr_common::accumulator::Accumulator::evaluate::evaluate")

- retract an update to its state from given inputs via [`retract_batch`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Accumulator.html#method.retract_batch "method datafusion_expr_common::accumulator::Accumulator::retract_batch::retract_batch") (when used as a window aggregate [window function](https://en.wikipedia.org/wiki/Window_function_(SQL)))

- convert its internal state to a vector of aggregate values via [`state`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Accumulator.html#tymethod.state "method datafusion_expr_common::accumulator::Accumulator::state::state") and combine the state from multiple accumulators via [`merge_batch`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Accumulator.html#tymethod.merge_batch "method datafusion_expr_common::accumulator::Accumulator::merge_batch::merge_batch"), as part of efficient multi-phase grouping.

## Required Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#tymethod.update_batch" class="fn">update_batch</a>( &mut self, values: &\[<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>\>\], ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Updates the accumulator’s state from its input.

`values` contains the arguments to this aggregate function.

For example, the `SUM` accumulator maintains a running sum, and `update_batch` adds each of the input values to the running sum.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#tymethod.evaluate" class="fn">evaluate</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Returns the final aggregate value, consuming the internal state.

For example, the `SUM` accumulator maintains a running sum, and `evaluate` will produce that running sum as its output.

This function should not be called twice, otherwise it will result in potentially non-deterministic behavior.

This function gets `&mut self` to allow for the accumulator to build arrow-compatible internal state that can be returned without copying when possible (for example distinct strings)

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#tymethod.size" class="fn">size</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the allocated size required for this accumulator, in bytes, including `Self`.

This value is used to calculate the memory used during execution so DataFusion can stay within its allotted limit.

“Allocated” means that for internal containers such as `Vec`, the `capacity` should be used not the `len`.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#tymethod.state" class="fn">state</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Returns the intermediate state of the accumulator, consuming the intermediate state.

This function should not be called twice, otherwise it will result in potentially non-deterministic behavior.

This function gets `&mut self` to allow for the accumulator to build arrow-compatible internal state that can be returned without copying when possible (for example distinct strings).

Intermediate state is used for “multi-phase” grouping in DataFusion, where an aggregate is computed in parallel with multiple `Accumulator` instances, as described below:

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#multi-phase-grouping" class="doc-anchor">§</a>Multi-Phase Grouping

``` text
                              ▲
                              │                   evaluate() is called to
                              │                   produce the final aggregate
                              │                   value per group
                              │
                 ┌─────────────────────────┐
                 │GroupBy                  │
                 │(AggregateMode::Final)   │      state() is called for each
                 │                         │      group and the resulting
                 └─────────────────────────┘      RecordBatches passed to the
                                                  Final GroupBy via merge_batch()
                              ▲
                              │
             ┌────────────────┴───────────────┐
             │                                │
             │                                │
┌─────────────────────────┐      ┌─────────────────────────┐
│        GroupBy          │      │        GroupBy          │
│(AggregateMode::Partial) │      │(AggregateMode::Partial) │
└─────────────────────────┘      └─────────────────────────┘
             ▲                                ▲
             │                                │    update_batch() is called for
             │                                │    each input RecordBatch
        .─────────.                      .─────────.
     ,─'           '─.                ,─'           '─.
    ;      Input      :              ;      Input      :
    :   Partition 0   ;              :   Partition 1   ;
     ╲               ╱                ╲               ╱
      '─.         ,─'                  '─.         ,─'
         `───────'                        `───────'
```

The partial state is serialized as `Arrays` and then combined with other partial states from different instances of this Accumulator (that ran on different partitions, for example).

The state can be and often is a different type than the output type of the [`Accumulator`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Accumulator.html "trait datafusion::logical_expr::Accumulator") and needs different merge operations (for example, the partial state for `COUNT` needs to be summed together)

Some accumulators can return multiple values for their intermediate states. For example, the average accumulator tracks `sum` and `n`, and this function should return a vector of two values, sum and n.

Note that [`ScalarValue::List`](https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html#variant.List "variant datafusion::scalar::ScalarValue::List") can be used to pass multiple values if the number of intermediate values is not known at planning time (e.g. for `MEDIAN`)

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#multi-phase-repartitioned-grouping" class="doc-anchor">§</a>Multi-phase repartitioned Grouping

Many multi-phase grouping plans contain a Repartition operation as well as shown below:

``` text
               ▲                          ▲
               │                          │
               │                          │
               │                          │
               │                          │
               │                          │
   ┌───────────────────────┐  ┌───────────────────────┐       4. Each AggregateMode::Final
   │GroupBy                │  │GroupBy                │       GroupBy has an entry for its
   │(AggregateMode::Final) │  │(AggregateMode::Final) │       subset of groups (in this case
   │                       │  │                       │       that means half the entries)
   └───────────────────────┘  └───────────────────────┘
               ▲                          ▲
               │                          │
               └─────────────┬────────────┘
                             │
                             │
                             │
                ┌─────────────────────────┐                   3. Repartitioning by hash(group
                │       Repartition       │                   keys) ensures that each distinct
                │         HASH(x)         │                   group key now appears in exactly
                └─────────────────────────┘                   one partition
                             ▲
                             │
             ┌───────────────┴─────────────┐
             │                             │
             │                             │
┌─────────────────────────┐  ┌──────────────────────────┐     2. Each AggregateMode::Partial
│        GroupBy          │  │       GroupBy            │     GroupBy has an entry for *all*
│(AggregateMode::Partial) │  │ (AggregateMode::Partial) │     the groups
└─────────────────────────┘  └──────────────────────────┘
             ▲                             ▲
             │                             │
             │                             │
        .─────────.                   .─────────.
     ,─'           '─.             ,─'           '─.
    ;      Input      :           ;      Input      :         1. Since input data is
    :   Partition 0   ;           :   Partition 1   ;         arbitrarily or RoundRobin
     ╲               ╱             ╲               ╱          distributed, each partition
      '─.         ,─'               '─.         ,─'           likely has all distinct
         `───────'                     `───────'
```

This structure is used so that the `AggregateMode::Partial` accumulators reduces the cardinality of the input as soon as possible. Typically, each partial accumulator sees all groups in the input as the group keys are evenly distributed across the input.

The final output is computed by repartitioning the result of [`Self::state`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Accumulator.html#tymethod.state "method datafusion_expr_common::accumulator::Accumulator::state::state") from each Partial aggregate and `hash(group keys)` so that each distinct group key appears in exactly one of the `AggregateMode::Final` GroupBy nodes. The outputs of the final nodes are then unioned together to produce the overall final output.

Here is an example that shows the distribution of groups in the different phases

``` text
              ┌─────┐                ┌─────┐
              │  1  │                │  3  │
              ├─────┤                ├─────┤
              │  2  │                │  4  │                After repartitioning by
              └─────┘                └─────┘                hash(group keys), each distinct
              ┌─────┐                ┌─────┐                group key now appears in exactly
              │  1  │                │  3  │                one partition
              ├─────┤                ├─────┤
              │  2  │                │  4  │
              └─────┘                └─────┘


─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─

              ┌─────┐                ┌─────┐
              │  2  │                │  2  │
              ├─────┤                ├─────┤
              │  1  │                │  2  │
              ├─────┤                ├─────┤
              │  3  │                │  3  │
              ├─────┤                ├─────┤
              │  4  │                │  1  │
              └─────┘                └─────┘                Input data is arbitrarily or
                ...                    ...                  RoundRobin distributed, each
              ┌─────┐                ┌─────┐                partition likely has all
              │  1  │                │  4  │                distinct group keys
              ├─────┤                ├─────┤
              │  4  │                │  3  │
              ├─────┤                ├─────┤
              │  1  │                │  1  │
              ├─────┤                ├─────┤
              │  4  │                │  3  │
              └─────┘                └─────┘

          group values           group values
          in partition 0         in partition 1
```

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#tymethod.merge_batch" class="fn">merge_batch</a>( &mut self, states: &\[<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>\>\], ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Updates the accumulator’s state from an `Array` containing one or more intermediate values.

For some aggregates (such as `SUM`), merge_batch is the same as `update_batch`, but for some aggregates (such as `COUNT`) the operations differ. See [`Self::state`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Accumulator.html#tymethod.state "method datafusion_expr_common::accumulator::Accumulator::state::state") for more details on how state is used and merged.

The `states` array passed was formed by concatenating the results of calling [`Self::state`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Accumulator.html#tymethod.state "method datafusion_expr_common::accumulator::Accumulator::state::state") on zero or more other `Accumulator` instances.

## Provided Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#provided-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#method.retract_batch" class="fn">retract_batch</a>( &mut self, \_values: &\[<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>\>\], ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Retracts (removed) an update (caused by the given inputs) to accumulator’s state.

This is the inverse operation of [`Self::update_batch`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Accumulator.html#tymethod.update_batch "method datafusion_expr_common::accumulator::Accumulator::update_batch::update_batch") and is used to incrementally calculate window aggregates where the `OVER` clause defines a bounded window.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#example" class="doc-anchor">§</a>Example

For example, given the following input partition

``` text
                    │      current      │
                           window
                    │                   │
               ┌────┬────┬────┬────┬────┬────┬────┬────┬────┐
    Input      │ A  │ B  │ C  │ D  │ E  │ F  │ G  │ H  │ I  │
  partition    └────┴────┴────┴────┼────┴────┴────┴────┼────┘

                                   │         next      │
                                            window
```

First, [`Self::evaluate`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Accumulator.html#tymethod.evaluate "method datafusion_expr_common::accumulator::Accumulator::evaluate::evaluate") will be called to produce the output for the current window.

Then, to advance to the next window:

First, [`Self::retract_batch`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Accumulator.html#method.retract_batch "method datafusion_expr_common::accumulator::Accumulator::retract_batch::retract_batch") will be called with the values that are leaving the window, `[B, C, D]` and then [`Self::update_batch`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Accumulator.html#tymethod.update_batch "method datafusion_expr_common::accumulator::Accumulator::update_batch::update_batch") will be called with the values that are entering the window, `[F, G, H]`.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#method.supports_retract_batch" class="fn">supports_retract_batch</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Does the accumulator support incrementally updating its value by *removing* values.

If this function returns true, [`Self::retract_batch`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Accumulator.html#method.retract_batch "method datafusion_expr_common::accumulator::Accumulator::retract_batch::retract_batch") will be called for sliding window functions such as queries with an `OVER (ROWS BETWEEN 1 PRECEDING AND 2 FOLLOWING)`

## Implementations on Foreign Types<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#foreign-impls" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#impl-Accumulator-for-Float64DistinctAvgAccumulator" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Accumulator.html" class="trait" title="trait datafusion::logical_expr::Accumulator">Accumulator</a> for <a href="https://docs.rs/datafusion-functions-aggregate-common/50.2.0/x86_64-unknown-linux-gnu/datafusion_functions_aggregate_common/aggregate/avg_distinct/numeric/struct.Float64DistinctAvgAccumulator.html" class="struct" title="struct datafusion_functions_aggregate_common::aggregate::avg_distinct::numeric::Float64DistinctAvgAccumulator">Float64DistinctAvgAccumulator</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#method.state" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#tymethod.state" class="fn">state</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#method.update_batch" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#tymethod.update_batch" class="fn">update_batch</a>( &mut self, values: &\[<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>\>\], ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#method.merge_batch" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#tymethod.merge_batch" class="fn">merge_batch</a>( &mut self, states: &\[<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>\>\], ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#method.evaluate" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#tymethod.evaluate" class="fn">evaluate</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#method.size" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#tymethod.size" class="fn">size</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#impl-Accumulator-for-BytesViewDistinctCountAccumulator" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Accumulator.html" class="trait" title="trait datafusion::logical_expr::Accumulator">Accumulator</a> for <a href="https://docs.rs/datafusion-functions-aggregate-common/50.2.0/x86_64-unknown-linux-gnu/datafusion_functions_aggregate_common/aggregate/count_distinct/bytes/struct.BytesViewDistinctCountAccumulator.html" class="struct" title="struct datafusion_functions_aggregate_common::aggregate::count_distinct::bytes::BytesViewDistinctCountAccumulator">BytesViewDistinctCountAccumulator</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#method.state-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#tymethod.state" class="fn">state</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#method.update_batch-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#tymethod.update_batch" class="fn">update_batch</a>( &mut self, values: &\[<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>\>\], ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#method.merge_batch-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#tymethod.merge_batch" class="fn">merge_batch</a>( &mut self, states: &\[<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>\>\], ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#method.evaluate-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#tymethod.evaluate" class="fn">evaluate</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#method.size-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#tymethod.size" class="fn">size</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#impl-Accumulator-for-DictionaryCountAccumulator" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Accumulator.html" class="trait" title="trait datafusion::logical_expr::Accumulator">Accumulator</a> for <a href="https://docs.rs/datafusion-functions-aggregate-common/50.2.0/x86_64-unknown-linux-gnu/datafusion_functions_aggregate_common/aggregate/count_distinct/dict/struct.DictionaryCountAccumulator.html" class="struct" title="struct datafusion_functions_aggregate_common::aggregate::count_distinct::dict::DictionaryCountAccumulator">DictionaryCountAccumulator</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#method.update_batch-2" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#tymethod.update_batch" class="fn">update_batch</a>( &mut self, values: &\[<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>\>\], ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#method.evaluate-2" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#tymethod.evaluate" class="fn">evaluate</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#method.size-2" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#tymethod.size" class="fn">size</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#method.state-2" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#tymethod.state" class="fn">state</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#method.merge_batch-2" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#tymethod.merge_batch" class="fn">merge_batch</a>( &mut self, states: &\[<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>\>\], ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#impl-Accumulator-for-BytesDistinctCountAccumulator%3CO%3E" class="anchor">§</a>

### impl\<O\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Accumulator.html" class="trait" title="trait datafusion::logical_expr::Accumulator">Accumulator</a> for <a href="https://docs.rs/datafusion-functions-aggregate-common/50.2.0/x86_64-unknown-linux-gnu/datafusion_functions_aggregate_common/aggregate/count_distinct/bytes/struct.BytesDistinctCountAccumulator.html" class="struct" title="struct datafusion_functions_aggregate_common::aggregate::count_distinct::bytes::BytesDistinctCountAccumulator">BytesDistinctCountAccumulator</a>\<O\>

where O: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.OffsetSizeTrait.html" class="trait" title="trait datafusion::common::arrow::array::OffsetSizeTrait">OffsetSizeTrait</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#method.state-3" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#tymethod.state" class="fn">state</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#method.update_batch-3" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#tymethod.update_batch" class="fn">update_batch</a>( &mut self, values: &\[<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>\>\], ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#method.merge_batch-3" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#tymethod.merge_batch" class="fn">merge_batch</a>( &mut self, states: &\[<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>\>\], ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#method.evaluate-3" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#tymethod.evaluate" class="fn">evaluate</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#method.size-3" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#tymethod.size" class="fn">size</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#impl-Accumulator-for-DecimalDistinctAvgAccumulator%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Accumulator.html" class="trait" title="trait datafusion::logical_expr::Accumulator">Accumulator</a> for <a href="https://docs.rs/datafusion-functions-aggregate-common/50.2.0/x86_64-unknown-linux-gnu/datafusion_functions_aggregate_common/aggregate/avg_distinct/decimal/struct.DecimalDistinctAvgAccumulator.html" class="struct" title="struct datafusion_functions_aggregate_common::aggregate::avg_distinct::decimal::DecimalDistinctAvgAccumulator">DecimalDistinctAvgAccumulator</a>\<T\>

where T: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/trait.DecimalType.html" class="trait" title="trait datafusion::common::arrow::datatypes::DecimalType">DecimalType</a> + <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowNumericType.html" class="trait" title="trait datafusion::common::arrow::array::ArrowNumericType">ArrowNumericType</a> + <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#method.state-4" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#tymethod.state" class="fn">state</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#method.update_batch-4" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#tymethod.update_batch" class="fn">update_batch</a>( &mut self, values: &\[<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>\>\], ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#method.merge_batch-4" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#tymethod.merge_batch" class="fn">merge_batch</a>( &mut self, states: &\[<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>\>\], ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#method.evaluate-4" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#tymethod.evaluate" class="fn">evaluate</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#method.size-4" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#tymethod.size" class="fn">size</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#impl-Accumulator-for-FloatDistinctCountAccumulator%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Accumulator.html" class="trait" title="trait datafusion::logical_expr::Accumulator">Accumulator</a> for <a href="https://docs.rs/datafusion-functions-aggregate-common/50.2.0/x86_64-unknown-linux-gnu/datafusion_functions_aggregate_common/aggregate/count_distinct/native/struct.FloatDistinctCountAccumulator.html" class="struct" title="struct datafusion_functions_aggregate_common::aggregate::count_distinct::native::FloatDistinctCountAccumulator">FloatDistinctCountAccumulator</a>\<T\>

where T: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait datafusion::common::arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#method.state-5" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#tymethod.state" class="fn">state</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#method.update_batch-5" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#tymethod.update_batch" class="fn">update_batch</a>( &mut self, values: &\[<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>\>\], ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#method.merge_batch-5" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#tymethod.merge_batch" class="fn">merge_batch</a>( &mut self, states: &\[<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>\>\], ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#method.evaluate-5" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#tymethod.evaluate" class="fn">evaluate</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#method.size-5" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#tymethod.size" class="fn">size</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#impl-Accumulator-for-PrimitiveDistinctCountAccumulator%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Accumulator.html" class="trait" title="trait datafusion::logical_expr::Accumulator">Accumulator</a> for <a href="https://docs.rs/datafusion-functions-aggregate-common/50.2.0/x86_64-unknown-linux-gnu/datafusion_functions_aggregate_common/aggregate/count_distinct/native/struct.PrimitiveDistinctCountAccumulator.html" class="struct" title="struct datafusion_functions_aggregate_common::aggregate::count_distinct::native::PrimitiveDistinctCountAccumulator">PrimitiveDistinctCountAccumulator</a>\<T\>

where T: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait datafusion::common::arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a>, \<T as <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait datafusion::common::arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type datafusion::common::arrow::array::ArrowPrimitiveType::Native">Native</a>: <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> + <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#method.state-6" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#tymethod.state" class="fn">state</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#method.update_batch-6" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#tymethod.update_batch" class="fn">update_batch</a>( &mut self, values: &\[<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>\>\], ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#method.merge_batch-6" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#tymethod.merge_batch" class="fn">merge_batch</a>( &mut self, states: &\[<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>\>\], ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#method.evaluate-6" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#tymethod.evaluate" class="fn">evaluate</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#method.size-6" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#tymethod.size" class="fn">size</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#impl-Accumulator-for-DistinctSumAccumulator%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Accumulator.html" class="trait" title="trait datafusion::logical_expr::Accumulator">Accumulator</a> for <a href="https://docs.rs/datafusion-functions-aggregate-common/50.2.0/x86_64-unknown-linux-gnu/datafusion_functions_aggregate_common/aggregate/sum_distinct/numeric/struct.DistinctSumAccumulator.html" class="struct" title="struct datafusion_functions_aggregate_common::aggregate::sum_distinct::numeric::DistinctSumAccumulator">DistinctSumAccumulator</a>\<T\>

where T: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait datafusion::common::arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#method.state-7" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#tymethod.state" class="fn">state</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#method.update_batch-7" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#tymethod.update_batch" class="fn">update_batch</a>( &mut self, values: &\[<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>\>\], ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#method.merge_batch-7" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#tymethod.merge_batch" class="fn">merge_batch</a>( &mut self, states: &\[<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>\>\], ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#method.evaluate-7" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#tymethod.evaluate" class="fn">evaluate</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#method.size-7" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#tymethod.size" class="fn">size</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

## Implementors<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#impl-Accumulator-for-ApproxPercentileAccumulator" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Accumulator.html" class="trait" title="trait datafusion::logical_expr::Accumulator">Accumulator</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/approx_percentile_cont/struct.ApproxPercentileAccumulator.html" class="struct" title="struct datafusion::functions_aggregate::approx_percentile_cont::ApproxPercentileAccumulator">ApproxPercentileAccumulator</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#impl-Accumulator-for-ApproxPercentileWithWeightAccumulator" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Accumulator.html" class="trait" title="trait datafusion::logical_expr::Accumulator">Accumulator</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/approx_percentile_cont_with_weight/struct.ApproxPercentileWithWeightAccumulator.html" class="struct" title="struct datafusion::functions_aggregate::approx_percentile_cont_with_weight::ApproxPercentileWithWeightAccumulator">ApproxPercentileWithWeightAccumulator</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#impl-Accumulator-for-ArrayAggAccumulator" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Accumulator.html" class="trait" title="trait datafusion::logical_expr::Accumulator">Accumulator</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/array_agg/struct.ArrayAggAccumulator.html" class="struct" title="struct datafusion::functions_aggregate::array_agg::ArrayAggAccumulator">ArrayAggAccumulator</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#impl-Accumulator-for-AvgAccumulator" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Accumulator.html" class="trait" title="trait datafusion::logical_expr::Accumulator">Accumulator</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/average/struct.AvgAccumulator.html" class="struct" title="struct datafusion::functions_aggregate::average::AvgAccumulator">AvgAccumulator</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#impl-Accumulator-for-CorrelationAccumulator" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Accumulator.html" class="trait" title="trait datafusion::logical_expr::Accumulator">Accumulator</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/correlation/struct.CorrelationAccumulator.html" class="struct" title="struct datafusion::functions_aggregate::correlation::CorrelationAccumulator">CorrelationAccumulator</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#impl-Accumulator-for-SlidingDistinctCountAccumulator" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Accumulator.html" class="trait" title="trait datafusion::logical_expr::Accumulator">Accumulator</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/count/struct.SlidingDistinctCountAccumulator.html" class="struct" title="struct datafusion::functions_aggregate::count::SlidingDistinctCountAccumulator">SlidingDistinctCountAccumulator</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#impl-Accumulator-for-CovarianceAccumulator" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Accumulator.html" class="trait" title="trait datafusion::logical_expr::Accumulator">Accumulator</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/covariance/struct.CovarianceAccumulator.html" class="struct" title="struct datafusion::functions_aggregate::covariance::CovarianceAccumulator">CovarianceAccumulator</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#impl-Accumulator-for-FirstValueAccumulator" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Accumulator.html" class="trait" title="trait datafusion::logical_expr::Accumulator">Accumulator</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/first_last/struct.FirstValueAccumulator.html" class="struct" title="struct datafusion::functions_aggregate::first_last::FirstValueAccumulator">FirstValueAccumulator</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#impl-Accumulator-for-TrivialFirstValueAccumulator" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Accumulator.html" class="trait" title="trait datafusion::logical_expr::Accumulator">Accumulator</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/first_last/struct.TrivialFirstValueAccumulator.html" class="struct" title="struct datafusion::functions_aggregate::first_last::TrivialFirstValueAccumulator">TrivialFirstValueAccumulator</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#impl-Accumulator-for-TrivialLastValueAccumulator" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Accumulator.html" class="trait" title="trait datafusion::logical_expr::Accumulator">Accumulator</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/first_last/struct.TrivialLastValueAccumulator.html" class="struct" title="struct datafusion::functions_aggregate::first_last::TrivialLastValueAccumulator">TrivialLastValueAccumulator</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#impl-Accumulator-for-MaxAccumulator" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Accumulator.html" class="trait" title="trait datafusion::logical_expr::Accumulator">Accumulator</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/min_max/struct.MaxAccumulator.html" class="struct" title="struct datafusion::functions_aggregate::min_max::MaxAccumulator">MaxAccumulator</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#impl-Accumulator-for-MinAccumulator" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Accumulator.html" class="trait" title="trait datafusion::logical_expr::Accumulator">Accumulator</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/min_max/struct.MinAccumulator.html" class="struct" title="struct datafusion::functions_aggregate::min_max::MinAccumulator">MinAccumulator</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#impl-Accumulator-for-SlidingMaxAccumulator" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Accumulator.html" class="trait" title="trait datafusion::logical_expr::Accumulator">Accumulator</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/min_max/struct.SlidingMaxAccumulator.html" class="struct" title="struct datafusion::functions_aggregate::min_max::SlidingMaxAccumulator">SlidingMaxAccumulator</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#impl-Accumulator-for-SlidingMinAccumulator" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Accumulator.html" class="trait" title="trait datafusion::logical_expr::Accumulator">Accumulator</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/min_max/struct.SlidingMinAccumulator.html" class="struct" title="struct datafusion::functions_aggregate::min_max::SlidingMinAccumulator">SlidingMinAccumulator</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#impl-Accumulator-for-NthValueAccumulator" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Accumulator.html" class="trait" title="trait datafusion::logical_expr::Accumulator">Accumulator</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/nth_value/struct.NthValueAccumulator.html" class="struct" title="struct datafusion::functions_aggregate::nth_value::NthValueAccumulator">NthValueAccumulator</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#impl-Accumulator-for-TrivialNthValueAccumulator" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Accumulator.html" class="trait" title="trait datafusion::logical_expr::Accumulator">Accumulator</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/nth_value/struct.TrivialNthValueAccumulator.html" class="struct" title="struct datafusion::functions_aggregate::nth_value::TrivialNthValueAccumulator">TrivialNthValueAccumulator</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#impl-Accumulator-for-RegrAccumulator" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Accumulator.html" class="trait" title="trait datafusion::logical_expr::Accumulator">Accumulator</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/regr/struct.RegrAccumulator.html" class="struct" title="struct datafusion::functions_aggregate::regr::RegrAccumulator">RegrAccumulator</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#impl-Accumulator-for-StddevAccumulator" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Accumulator.html" class="trait" title="trait datafusion::logical_expr::Accumulator">Accumulator</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/stddev/struct.StddevAccumulator.html" class="struct" title="struct datafusion::functions_aggregate::stddev::StddevAccumulator">StddevAccumulator</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#impl-Accumulator-for-SlidingDistinctSumAccumulator" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Accumulator.html" class="trait" title="trait datafusion::logical_expr::Accumulator">Accumulator</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/sum/struct.SlidingDistinctSumAccumulator.html" class="struct" title="struct datafusion::functions_aggregate::sum::SlidingDistinctSumAccumulator">SlidingDistinctSumAccumulator</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.Accumulator.html#impl-Accumulator-for-VarianceAccumulator" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Accumulator.html" class="trait" title="trait datafusion::logical_expr::Accumulator">Accumulator</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/variance/struct.VarianceAccumulator.html" class="struct" title="struct datafusion::functions_aggregate::variance::VarianceAccumulator">VarianceAccumulator</a>
