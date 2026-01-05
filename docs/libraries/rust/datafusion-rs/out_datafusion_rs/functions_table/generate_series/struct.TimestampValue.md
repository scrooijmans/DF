# Struct TimestampValue Copy item path

<a href="https://docs.rs/datafusion-functions-table/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_functions_table/generate_series.rs.html#115" class="src">Source</a>

``` rust
pub struct TimestampValue { /* private fields */ }
```

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_table/generate_series/struct.TimestampValue.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_table/generate_series/struct.TimestampValue.html#impl-TimestampValue" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_table/generate_series/struct.TimestampValue.html" class="struct" title="struct datafusion::functions_table::generate_series::TimestampValue">TimestampValue</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_table/generate_series/struct.TimestampValue.html#method.value" class="fn">value</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_table/generate_series/struct.TimestampValue.html#method.tz_str" class="fn">tz_str</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>\>

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_table/generate_series/struct.TimestampValue.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_table/generate_series/struct.TimestampValue.html#impl-Clone-for-TimestampValue" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_table/generate_series/struct.TimestampValue.html" class="struct" title="struct datafusion::functions_table::generate_series::TimestampValue">TimestampValue</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_table/generate_series/struct.TimestampValue.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_table/generate_series/struct.TimestampValue.html" class="struct" title="struct datafusion::functions_table::generate_series::TimestampValue">TimestampValue</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_table/generate_series/struct.TimestampValue.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_table/generate_series/struct.TimestampValue.html#impl-Debug-for-TimestampValue" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_table/generate_series/struct.TimestampValue.html" class="struct" title="struct datafusion::functions_table::generate_series::TimestampValue">TimestampValue</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_table/generate_series/struct.TimestampValue.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_table/generate_series/struct.TimestampValue.html#impl-SeriesValue-for-TimestampValue" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_table/generate_series/trait.SeriesValue.html" class="trait" title="trait datafusion::functions_table::generate_series::SeriesValue">SeriesValue</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_table/generate_series/struct.TimestampValue.html" class="struct" title="struct datafusion::functions_table::generate_series::TimestampValue">TimestampValue</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_table/generate_series/struct.TimestampValue.html#associatedtype.StepType" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_table/generate_series/trait.SeriesValue.html#associatedtype.StepType" class="associatedtype">StepType</a> = <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_table/generate_series/struct.TimestampValue.html#associatedtype.ValueType" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_table/generate_series/trait.SeriesValue.html#associatedtype.ValueType" class="associatedtype">ValueType</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_table/generate_series/struct.TimestampValue.html#method.should_stop" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_table/generate_series/trait.SeriesValue.html#tymethod.should_stop" class="fn">should_stop</a>( &self, end: <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_table/generate_series/struct.TimestampValue.html" class="struct" title="struct datafusion::functions_table::generate_series::TimestampValue">TimestampValue</a>, step: &\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_table/generate_series/struct.TimestampValue.html" class="struct" title="struct datafusion::functions_table::generate_series::TimestampValue">TimestampValue</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_table/generate_series/trait.SeriesValue.html" class="trait" title="trait datafusion::functions_table::generate_series::SeriesValue">SeriesValue</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_table/generate_series/trait.SeriesValue.html#associatedtype.StepType" class="associatedtype" title="type datafusion::functions_table::generate_series::SeriesValue::StepType">StepType</a>, include_end: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Check if we’ve reached the end of the series

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_table/generate_series/struct.TimestampValue.html#method.advance" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_table/generate_series/trait.SeriesValue.html#tymethod.advance" class="fn">advance</a>( &mut self, step: &\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_table/generate_series/struct.TimestampValue.html" class="struct" title="struct datafusion::functions_table::generate_series::TimestampValue">TimestampValue</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_table/generate_series/trait.SeriesValue.html" class="trait" title="trait datafusion::functions_table::generate_series::SeriesValue">SeriesValue</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_table/generate_series/trait.SeriesValue.html#associatedtype.StepType" class="associatedtype" title="type datafusion::functions_table::generate_series::SeriesValue::StepType">StepType</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Advance to the next value in the series

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_table/generate_series/struct.TimestampValue.html#method.create_array" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_table/generate_series/trait.SeriesValue.html#tymethod.create_array" class="fn">create_array</a>( &self, values: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_table/generate_series/struct.TimestampValue.html" class="struct" title="struct datafusion::functions_table::generate_series::TimestampValue">TimestampValue</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_table/generate_series/trait.SeriesValue.html" class="trait" title="trait datafusion::functions_table::generate_series::SeriesValue">SeriesValue</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_table/generate_series/trait.SeriesValue.html#associatedtype.ValueType" class="associatedtype" title="type datafusion::functions_table::generate_series::SeriesValue::ValueType">ValueType</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Create an Arrow array from a vector of values

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_table/generate_series/struct.TimestampValue.html#method.to_value_type" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_table/generate_series/trait.SeriesValue.html#tymethod.to_value_type" class="fn">to_value_type</a>(&self) -\> \<<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_table/generate_series/struct.TimestampValue.html" class="struct" title="struct datafusion::functions_table::generate_series::TimestampValue">TimestampValue</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_table/generate_series/trait.SeriesValue.html" class="trait" title="trait datafusion::functions_table::generate_series::SeriesValue">SeriesValue</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_table/generate_series/trait.SeriesValue.html#associatedtype.ValueType" class="associatedtype" title="type datafusion::functions_table::generate_series::SeriesValue::ValueType">ValueType</a>

Convert self to ValueType for array creation

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_table/generate_series/struct.TimestampValue.html#method.display_value" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_table/generate_series/trait.SeriesValue.html#tymethod.display_value" class="fn">display_value</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>

Display the value for debugging

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_table/generate_series/struct.TimestampValue.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_table/generate_series/struct.TimestampValue.html#blanket-implementations" class="anchor">§</a>
