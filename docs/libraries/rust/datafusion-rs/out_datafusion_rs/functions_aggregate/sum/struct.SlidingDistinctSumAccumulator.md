# Struct SlidingDistinctSumAccumulator Copy item path

<a href="https://docs.rs/datafusion-functions-aggregate/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_functions_aggregate/sum.rs.html#412" class="src">Source</a>

``` rust
pub struct SlidingDistinctSumAccumulator { /* private fields */ }
```

Expand description

A sliding‐window accumulator for `SUM(DISTINCT)` over Int64 columns. Maintains a running sum so that `evaluate()` is O(1).

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/sum/struct.SlidingDistinctSumAccumulator.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/sum/struct.SlidingDistinctSumAccumulator.html#impl-SlidingDistinctSumAccumulator" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/sum/struct.SlidingDistinctSumAccumulator.html" class="struct" title="struct datafusion::functions_aggregate::sum::SlidingDistinctSumAccumulator">SlidingDistinctSumAccumulator</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/sum/struct.SlidingDistinctSumAccumulator.html#method.try_new" class="fn">try_new</a>( data_type: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/sum/struct.SlidingDistinctSumAccumulator.html" class="struct" title="struct datafusion::functions_aggregate::sum::SlidingDistinctSumAccumulator">SlidingDistinctSumAccumulator</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Create a new accumulator; only `DataType::Int64` is supported.

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/sum/struct.SlidingDistinctSumAccumulator.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/sum/struct.SlidingDistinctSumAccumulator.html#impl-Accumulator-for-SlidingDistinctSumAccumulator" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Accumulator.html" class="trait" title="trait datafusion::logical_expr::Accumulator">Accumulator</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/sum/struct.SlidingDistinctSumAccumulator.html" class="struct" title="struct datafusion::functions_aggregate::sum::SlidingDistinctSumAccumulator">SlidingDistinctSumAccumulator</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/sum/struct.SlidingDistinctSumAccumulator.html#method.update_batch" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Accumulator.html#tymethod.update_batch" class="fn">update_batch</a>( &mut self, values: &\[<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>\>\], ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Updates the accumulator’s state from its input. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Accumulator.html#tymethod.update_batch)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/sum/struct.SlidingDistinctSumAccumulator.html#method.evaluate" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Accumulator.html#tymethod.evaluate" class="fn">evaluate</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Returns the final aggregate value, consuming the internal state. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Accumulator.html#tymethod.evaluate)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/sum/struct.SlidingDistinctSumAccumulator.html#method.size" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Accumulator.html#tymethod.size" class="fn">size</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the allocated size required for this accumulator, in bytes, including `Self`. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Accumulator.html#tymethod.size)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/sum/struct.SlidingDistinctSumAccumulator.html#method.state" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Accumulator.html#tymethod.state" class="fn">state</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Returns the intermediate state of the accumulator, consuming the intermediate state. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Accumulator.html#tymethod.state)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/sum/struct.SlidingDistinctSumAccumulator.html#method.merge_batch" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Accumulator.html#tymethod.merge_batch" class="fn">merge_batch</a>( &mut self, states: &\[<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>\>\], ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Updates the accumulator’s state from an `Array` containing one or more intermediate values. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Accumulator.html#tymethod.merge_batch)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/sum/struct.SlidingDistinctSumAccumulator.html#method.retract_batch" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Accumulator.html#method.retract_batch" class="fn">retract_batch</a>( &mut self, values: &\[<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>\>\], ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Retracts (removed) an update (caused by the given inputs) to accumulator’s state. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Accumulator.html#method.retract_batch)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/sum/struct.SlidingDistinctSumAccumulator.html#method.supports_retract_batch" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Accumulator.html#method.supports_retract_batch" class="fn">supports_retract_batch</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Does the accumulator support incrementally updating its value by *removing* values. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Accumulator.html#method.supports_retract_batch)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/sum/struct.SlidingDistinctSumAccumulator.html#impl-Debug-for-SlidingDistinctSumAccumulator" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/sum/struct.SlidingDistinctSumAccumulator.html" class="struct" title="struct datafusion::functions_aggregate::sum::SlidingDistinctSumAccumulator">SlidingDistinctSumAccumulator</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/sum/struct.SlidingDistinctSumAccumulator.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/sum/struct.SlidingDistinctSumAccumulator.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/sum/struct.SlidingDistinctSumAccumulator.html#blanket-implementations" class="anchor">§</a>
