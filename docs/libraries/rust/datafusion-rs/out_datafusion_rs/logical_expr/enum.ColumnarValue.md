# Enum ColumnarValue Copy item path

<a href="https://docs.rs/datafusion-expr-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr_common/columnar_value.rs.html#89" class="src">Source</a>

``` rust
pub enum ColumnarValue {
    Array(Arc<dyn Array>),
    Scalar(ScalarValue),
}
```

Expand description

The result of evaluating an expression.

[`ColumnarValue::Scalar`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ColumnarValue.html#variant.Scalar "variant datafusion::logical_expr::ColumnarValue::Scalar") represents a single value repeated any number of times. This is an important performance optimization for handling values that do not change across rows.

[`ColumnarValue::Array`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ColumnarValue.html#variant.Array "variant datafusion::logical_expr::ColumnarValue::Array") represents a column of data, stored as an Arrow [`ArrayRef`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/type.ArrayRef.html "type datafusion::common::arrow::array::ArrayRef")

A slice of `ColumnarValue`s logically represents a table, with each column having the same number of rows. This means that all `Array`s are the same length.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ColumnarValue.html#example" class="doc-anchor">§</a>Example

A `ColumnarValue::Array` with an array of 5 elements and a `ColumnarValue::Scalar` with the value 100

``` text
┌──────────────┐
│ ┌──────────┐ │
│ │   "A"    │ │
│ ├──────────┤ │
│ │   "B"    │ │
│ ├──────────┤ │
│ │   "C"    │ │
│ ├──────────┤ │
│ │   "D"    │ │        ┌──────────────┐
│ ├──────────┤ │        │ ┌──────────┐ │
│ │   "E"    │ │        │ │   100    │ │
│ └──────────┘ │        │ └──────────┘ │
└──────────────┘        └──────────────┘

 ColumnarValue::        ColumnarValue::
      Array                 Scalar
```

Logically represents the following table:

| Column 1 | Column 2 |
|----------|----------|
| A        | 100      |
| B        | 100      |
| C        | 100      |
| D        | 100      |
| E        | 100      |

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ColumnarValue.html#performance-notes" class="doc-anchor">§</a>Performance Notes

When implementing functions or operators, it is important to consider the performance implications of handling scalar values.

Because all functions must handle [`ArrayRef`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/type.ArrayRef.html "type datafusion::common::arrow::array::ArrayRef"), it is convenient to convert [`ColumnarValue::Scalar`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ColumnarValue.html#variant.Scalar "variant datafusion::logical_expr::ColumnarValue::Scalar")s using [`Self::into_array`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ColumnarValue.html#method.into_array "method datafusion::logical_expr::ColumnarValue::into_array"). For example, [`ColumnarValue::values_to_arrays`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ColumnarValue.html#method.values_to_arrays "associated function datafusion::logical_expr::ColumnarValue::values_to_arrays") converts multiple columnar values into arrays of the same length.

However, it is often much more performant to provide a different, implementation that handles scalar values differently

## Variants<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ColumnarValue.html#variants" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ColumnarValue.html#variant.Array" class="anchor">§</a>

### Array(<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>\>)

Array of values

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ColumnarValue.html#variant.Scalar" class="anchor">§</a>

### Scalar(<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>)

A single value

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ColumnarValue.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ColumnarValue.html#impl-ColumnarValue" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ColumnarValue.html" class="enum" title="enum datafusion::logical_expr::ColumnarValue">ColumnarValue</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ColumnarValue.html#method.data_type" class="fn">data_type</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ColumnarValue.html#method.into_array" class="fn">into_array</a>( self, num_rows: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Convert a columnar value into an Arrow [`ArrayRef`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/type.ArrayRef.html "type datafusion::common::arrow::array::ArrayRef") with the specified number of rows. [`Self::Scalar`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ColumnarValue.html#variant.Scalar "variant datafusion::logical_expr::ColumnarValue::Scalar") is converted by repeating the same scalar multiple times which is not as efficient as handling the scalar directly.

See [`Self::values_to_arrays`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ColumnarValue.html#method.values_to_arrays "associated function datafusion::logical_expr::ColumnarValue::values_to_arrays") to convert multiple columnar values into arrays of the same length.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ColumnarValue.html#errors" class="doc-anchor">§</a>Errors

Errors if `self` is a Scalar that fails to be converted into an array of size

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ColumnarValue.html#method.to_array" class="fn">to_array</a>( &self, num_rows: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Convert a columnar value into an Arrow [`ArrayRef`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/type.ArrayRef.html "type datafusion::common::arrow::array::ArrayRef") with the specified number of rows. [`Self::Scalar`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ColumnarValue.html#variant.Scalar "variant datafusion::logical_expr::ColumnarValue::Scalar") is converted by repeating the same scalar multiple times which is not as efficient as handling the scalar directly.

See [`Self::values_to_arrays`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ColumnarValue.html#method.values_to_arrays "associated function datafusion::logical_expr::ColumnarValue::values_to_arrays") to convert multiple columnar values into arrays of the same length.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ColumnarValue.html#errors-1" class="doc-anchor">§</a>Errors

Errors if `self` is a Scalar that fails to be converted into an array of size

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ColumnarValue.html#method.create_null_array" class="fn">create_null_array</a>(num_rows: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ColumnarValue.html" class="enum" title="enum datafusion::logical_expr::ColumnarValue">ColumnarValue</a>

Null columnar values are implemented as a null array in order to pass batch num_rows

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ColumnarValue.html#method.values_to_arrays" class="fn">values_to_arrays</a>( args: &\[<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ColumnarValue.html" class="enum" title="enum datafusion::logical_expr::ColumnarValue">ColumnarValue</a>\], ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>\>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Converts [`ColumnarValue`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ColumnarValue.html "enum datafusion::logical_expr::ColumnarValue")s to [`ArrayRef`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/type.ArrayRef.html "type datafusion::common::arrow::array::ArrayRef")s with the same length.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ColumnarValue.html#performance-note" class="doc-anchor">§</a>Performance Note

This function expands any [`ScalarValue`](https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html "enum datafusion::scalar::ScalarValue") to an array. This expansion permits using a single function in terms of arrays, but it can be inefficient compared to handling the scalar value directly.

Thus, it is recommended to provide specialized implementations for scalar values if performance is a concern.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ColumnarValue.html#errors-2" class="doc-anchor">§</a>Errors

If there are multiple array arguments that have different lengths

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ColumnarValue.html#method.cast_to" class="fn">cast_to</a>( &self, cast_type: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>, cast_options: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/struct.CastOptions.html" class="struct" title="struct datafusion::common::arrow::compute::CastOptions">CastOptions</a>\<'static\>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ColumnarValue.html" class="enum" title="enum datafusion::logical_expr::ColumnarValue">ColumnarValue</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Cast’s this [ColumnarValue](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ColumnarValue.html "enum datafusion::logical_expr::ColumnarValue") to the specified `DataType`

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ColumnarValue.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ColumnarValue.html#impl-Clone-for-ColumnarValue" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ColumnarValue.html" class="enum" title="enum datafusion::logical_expr::ColumnarValue">ColumnarValue</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ColumnarValue.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ColumnarValue.html" class="enum" title="enum datafusion::logical_expr::ColumnarValue">ColumnarValue</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ColumnarValue.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ColumnarValue.html#impl-Debug-for-ColumnarValue" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ColumnarValue.html" class="enum" title="enum datafusion::logical_expr::ColumnarValue">ColumnarValue</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ColumnarValue.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ColumnarValue.html#impl-Display-for-ColumnarValue" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ColumnarValue.html" class="enum" title="enum datafusion::logical_expr::ColumnarValue">ColumnarValue</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ColumnarValue.html#method.fmt-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ColumnarValue.html#impl-From%3CArc%3Cdyn+Array%3E%3E-for-ColumnarValue" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>\>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ColumnarValue.html" class="enum" title="enum datafusion::logical_expr::ColumnarValue">ColumnarValue</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ColumnarValue.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>\>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ColumnarValue.html" class="enum" title="enum datafusion::logical_expr::ColumnarValue">ColumnarValue</a>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ColumnarValue.html#impl-From%3CScalarValue%3E-for-ColumnarValue" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ColumnarValue.html" class="enum" title="enum datafusion::logical_expr::ColumnarValue">ColumnarValue</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ColumnarValue.html#method.from-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ColumnarValue.html" class="enum" title="enum datafusion::logical_expr::ColumnarValue">ColumnarValue</a>

Converts to this type from the input type.

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ColumnarValue.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ColumnarValue.html#blanket-implementations" class="anchor">§</a>
