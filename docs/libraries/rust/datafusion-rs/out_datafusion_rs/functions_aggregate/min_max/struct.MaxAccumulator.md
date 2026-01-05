# Struct MaxAccumulator Copy item path

<a href="https://docs.rs/datafusion-functions-aggregate/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_functions_aggregate/min_max.rs.html#740" class="src">Source</a>

``` rust
pub struct MaxAccumulator { /* private fields */ }
```

Expand description

An accumulator to compute the maximum value

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/min_max/struct.MaxAccumulator.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/min_max/struct.MaxAccumulator.html#impl-MaxAccumulator" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/min_max/struct.MaxAccumulator.html" class="struct" title="struct datafusion::functions_aggregate::min_max::MaxAccumulator">MaxAccumulator</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/min_max/struct.MaxAccumulator.html#method.try_new" class="fn">try_new</a>(datatype: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/min_max/struct.MaxAccumulator.html" class="struct" title="struct datafusion::functions_aggregate::min_max::MaxAccumulator">MaxAccumulator</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

new max accumulator

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/min_max/struct.MaxAccumulator.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/min_max/struct.MaxAccumulator.html#impl-Accumulator-for-MaxAccumulator" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Accumulator.html" class="trait" title="trait datafusion::logical_expr::Accumulator">Accumulator</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/min_max/struct.MaxAccumulator.html" class="struct" title="struct datafusion::functions_aggregate::min_max::MaxAccumulator">MaxAccumulator</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/min_max/struct.MaxAccumulator.html#method.update_batch" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Accumulator.html#tymethod.update_batch" class="fn">update_batch</a>( &mut self, values: &\[<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>\>\], ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Updates the accumulator’s state from its input. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Accumulator.html#tymethod.update_batch)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/min_max/struct.MaxAccumulator.html#method.merge_batch" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Accumulator.html#tymethod.merge_batch" class="fn">merge_batch</a>( &mut self, states: &\[<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>\>\], ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Updates the accumulator’s state from an `Array` containing one or more intermediate values. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Accumulator.html#tymethod.merge_batch)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/min_max/struct.MaxAccumulator.html#method.state" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Accumulator.html#tymethod.state" class="fn">state</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Returns the intermediate state of the accumulator, consuming the intermediate state. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Accumulator.html#tymethod.state)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/min_max/struct.MaxAccumulator.html#method.evaluate" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Accumulator.html#tymethod.evaluate" class="fn">evaluate</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Returns the final aggregate value, consuming the internal state. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Accumulator.html#tymethod.evaluate)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/min_max/struct.MaxAccumulator.html#method.size" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Accumulator.html#tymethod.size" class="fn">size</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the allocated size required for this accumulator, in bytes, including `Self`. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Accumulator.html#tymethod.size)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/min_max/struct.MaxAccumulator.html#method.retract_batch" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Accumulator.html#method.retract_batch" class="fn">retract_batch</a>( &mut self, \_values: &\[<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>\>\], ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Retracts (removed) an update (caused by the given inputs) to accumulator’s state. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Accumulator.html#method.retract_batch)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/min_max/struct.MaxAccumulator.html#method.supports_retract_batch" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Accumulator.html#method.supports_retract_batch" class="fn">supports_retract_batch</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Does the accumulator support incrementally updating its value by *removing* values. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Accumulator.html#method.supports_retract_batch)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/min_max/struct.MaxAccumulator.html#impl-Clone-for-MaxAccumulator" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/min_max/struct.MaxAccumulator.html" class="struct" title="struct datafusion::functions_aggregate::min_max::MaxAccumulator">MaxAccumulator</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/min_max/struct.MaxAccumulator.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/min_max/struct.MaxAccumulator.html" class="struct" title="struct datafusion::functions_aggregate::min_max::MaxAccumulator">MaxAccumulator</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/min_max/struct.MaxAccumulator.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/min_max/struct.MaxAccumulator.html#impl-Debug-for-MaxAccumulator" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/min_max/struct.MaxAccumulator.html" class="struct" title="struct datafusion::functions_aggregate::min_max::MaxAccumulator">MaxAccumulator</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/min_max/struct.MaxAccumulator.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/min_max/struct.MaxAccumulator.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/min_max/struct.MaxAccumulator.html#blanket-implementations" class="anchor">§</a>
