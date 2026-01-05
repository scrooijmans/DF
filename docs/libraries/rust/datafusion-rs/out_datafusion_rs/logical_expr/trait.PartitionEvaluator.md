# Trait PartitionEvaluator Copy item path

<a href="https://docs.rs/datafusion-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr/partition_evaluator.rs.html#89" class="src">Source</a>

``` rust
pub trait PartitionEvaluator: Debug + Send {
    // Provided methods
    fn memoize(
        &mut self,
        _state: &mut WindowAggState,
    ) -> Result<(), DataFusionError> { ... }
    fn get_range(
        &self,
        idx: usize,
        _n_rows: usize,
    ) -> Result<Range<usize>, DataFusionError> { ... }
    fn is_causal(&self) -> bool { ... }
    fn evaluate_all(
        &mut self,
        values: &[Arc<dyn Array>],
        num_rows: usize,
    ) -> Result<Arc<dyn Array>, DataFusionError> { ... }
    fn evaluate(
        &mut self,
        _values: &[Arc<dyn Array>],
        _range: &Range<usize>,
    ) -> Result<ScalarValue, DataFusionError> { ... }
    fn evaluate_all_with_rank(
        &self,
        _num_rows: usize,
        _ranks_in_partition: &[Range<usize>],
    ) -> Result<Arc<dyn Array>, DataFusionError> { ... }
    fn supports_bounded_execution(&self) -> bool { ... }
    fn uses_window_frame(&self) -> bool { ... }
    fn include_rank(&self) -> bool { ... }
}
```

Expand description

Partition evaluator for Window Functions

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.PartitionEvaluator.html#background" class="doc-anchor">§</a>Background

An implementation of this trait is created and used for each partition defined by an `OVER` clause and is instantiated by the DataFusion runtime.

For example, evaluating `window_func(val) OVER (PARTITION BY col)` on the following data:

``` text
col | val
--- + ----
 A  | 10
 A  | 10
 C  | 20
 D  | 30
 D  | 30
```

Will instantiate three `PartitionEvaluator`s, one each for the partitions defined by `col=A`, `col=B`, and `col=C`.

``` text
col | val
--- + ----
 A  | 10     <--- partition 1
 A  | 10

col | val
--- + ----
 C  | 20     <--- partition 2

col | val
--- + ----
 D  | 30     <--- partition 3
 D  | 30
```

Different methods on this trait will be called depending on the capabilities described by [`supports_bounded_execution`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.PartitionEvaluator.html#method.supports_bounded_execution "method datafusion_expr::partition_evaluator::PartitionEvaluator::supports_bounded_execution::supports_bounded_execution"), [`uses_window_frame`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.PartitionEvaluator.html#method.uses_window_frame "method datafusion_expr::partition_evaluator::PartitionEvaluator::uses_window_frame::uses_window_frame"), and [`include_rank`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.PartitionEvaluator.html#method.include_rank "method datafusion_expr::partition_evaluator::PartitionEvaluator::include_rank::include_rank"),

When implementing a new `PartitionEvaluator`, implement corresponding evaluator according to table below.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.PartitionEvaluator.html#implementation-table" class="doc-anchor">§</a>Implementation Table

| [`uses_window_frame`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.PartitionEvaluator.html#method.uses_window_frame "method datafusion_expr::partition_evaluator::PartitionEvaluator::uses_window_frame::uses_window_frame") | [`supports_bounded_execution`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.PartitionEvaluator.html#method.supports_bounded_execution "method datafusion_expr::partition_evaluator::PartitionEvaluator::supports_bounded_execution::supports_bounded_execution") | [`include_rank`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.PartitionEvaluator.html#method.include_rank "method datafusion_expr::partition_evaluator::PartitionEvaluator::include_rank::include_rank") | function_to_implement |
|----|----|----|----|
| false (default) | false (default) | false (default) | [`evaluate_all`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.PartitionEvaluator.html#method.evaluate_all "method datafusion_expr::partition_evaluator::PartitionEvaluator::evaluate_all::evaluate_all") |
| false | true | false | [`evaluate`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.PartitionEvaluator.html#method.evaluate "method datafusion_expr::partition_evaluator::PartitionEvaluator::evaluate::evaluate") |
| false | true/false | true | [`evaluate_all_with_rank`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.PartitionEvaluator.html#method.evaluate_all_with_rank "method datafusion_expr::partition_evaluator::PartitionEvaluator::evaluate_all_with_rank::evaluate_all_with_rank") |
| true | true/false | true/false | [`evaluate`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.PartitionEvaluator.html#method.evaluate "method datafusion_expr::partition_evaluator::PartitionEvaluator::evaluate::evaluate") |

## Provided Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.PartitionEvaluator.html#provided-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.PartitionEvaluator.html#method.memoize" class="fn">memoize</a>( &mut self, \_state: &mut <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/window_state/struct.WindowAggState.html" class="struct" title="struct datafusion::logical_expr::window_state::WindowAggState">WindowAggState</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

When the window frame has a fixed beginning (e.g UNBOUNDED PRECEDING), some functions such as FIRST_VALUE, LAST_VALUE and NTH_VALUE do not need the (unbounded) input once they have seen a certain amount of input.

`memoize` is called after each input batch is processed, and such functions can save whatever they need and modify [`WindowAggState`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/window_state/struct.WindowAggState.html "struct datafusion::logical_expr::window_state::WindowAggState") appropriately to allow rows to be pruned

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.PartitionEvaluator.html#method.get_range" class="fn">get_range</a>( &self, idx: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, \_n_rows: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html" class="struct" title="struct core::ops::range::Range">Range</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

If `uses_window_frame` flag is `false`. This method is used to calculate required range for the window function during stateful execution.

Generally there is no required range, hence by default this returns smallest range(current row). e.g seeing current row is enough to calculate window result (such as row_number, rank, etc)

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.PartitionEvaluator.html#method.is_causal" class="fn">is_causal</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Get whether evaluator needs future data for its result (if so returns `false`) or not

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.PartitionEvaluator.html#method.evaluate_all" class="fn">evaluate_all</a>( &mut self, values: &\[<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>\>\], num_rows: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Evaluate a window function on an entire input partition.

This function is called once per input *partition* for window functions that *do not use* values from the window frame, such as `ROW_NUMBER`, `RANK`, `DENSE_RANK`, `PERCENT_RANK`, `CUME_DIST`, `LEAD`, `LAG`).

It produces the result of all rows in a single pass. It expects to receive the entire partition as the `value` and must produce an output column with one output row for every input row.

`num_rows` is required to correctly compute the output in case `values.len() == 0`

Implementing this function is an optimization: certain window functions are not affected by the window frame definition or the query doesn’t have a frame, and `evaluate` skips the (costly) window frame boundary calculation and the overhead of calling `evaluate` for each output row.

For example, the `LAG` built in window function does not use the values of its window frame (it can be computed in one shot on the entire partition with `Self::evaluate_all` regardless of the window defined in the `OVER` clause)

``` sql
lag(x, 1) OVER (ORDER BY z ROWS BETWEEN 2 PRECEDING AND 3 FOLLOWING)
```

However, `avg()` computes the average in the window and thus does use its window frame

``` sql
avg(x) OVER (PARTITION BY y ORDER BY z ROWS BETWEEN 2 PRECEDING AND 3 FOLLOWING)
```

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.PartitionEvaluator.html#method.evaluate" class="fn">evaluate</a>( &mut self, \_values: &\[<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>\>\], \_range: &<a href="https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html" class="struct" title="struct core::ops::range::Range">Range</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Evaluate window function on a range of rows in an input partition.x

This is the simplest and most general function to implement but also the least performant as it creates output one row at a time. It is typically much faster to implement stateful evaluation using one of the other specialized methods on this trait.

Returns a [`ScalarValue`](https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html "enum datafusion::scalar::ScalarValue") that is the value of the window function within `range` for the entire partition. Argument `values` contains the evaluation result of function arguments and evaluation results of ORDER BY expressions. If function has a single argument, `values[1..]` will contain ORDER BY expression results.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.PartitionEvaluator.html#method.evaluate_all_with_rank" class="fn">evaluate_all_with_rank</a>( &self, \_num_rows: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, \_ranks_in_partition: &\[<a href="https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html" class="struct" title="struct core::ops::range::Range">Range</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>\], ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

[`PartitionEvaluator::evaluate_all_with_rank`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.PartitionEvaluator.html#method.evaluate_all_with_rank "method datafusion::logical_expr::PartitionEvaluator::evaluate_all_with_rank") is called for window functions that only need the rank of a row within its window frame.

Evaluate the partition evaluator against the partition using the row ranks. For example, `RANK(col)` produces

``` text
col | rank
--- + ----
 A  | 1
 A  | 1
 C  | 3
 D  | 4
 D  | 5
```

For this case, `num_rows` would be `5` and the `ranks_in_partition` would be called with

``` text
[
  (0,1),
  (2,2),
  (3,4),
]
```

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.PartitionEvaluator.html#method.supports_bounded_execution" class="fn">supports_bounded_execution</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Can the window function be incrementally computed using bounded memory?

See the table on [`Self`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.PartitionEvaluator.html "trait datafusion::logical_expr::PartitionEvaluator") for what functions to implement

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.PartitionEvaluator.html#method.uses_window_frame" class="fn">uses_window_frame</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Does the window function use the values from the window frame, if one is specified?

See the table on [`Self`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.PartitionEvaluator.html "trait datafusion::logical_expr::PartitionEvaluator") for what functions to implement

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.PartitionEvaluator.html#method.include_rank" class="fn">include_rank</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Can this function be evaluated with (only) rank

See the table on [`Self`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.PartitionEvaluator.html "trait datafusion::logical_expr::PartitionEvaluator") for what functions to implement

## Implementors<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.PartitionEvaluator.html#implementors" class="anchor">§</a>
