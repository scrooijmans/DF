# Struct DurationMicrosecondType Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/types.rs.html#254-259" class="src">Source</a>

``` rust
pub struct DurationMicrosecondType {}
```

Expand description

Elapsed time type: microseconds.

## Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.DurationMicrosecondType.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.DurationMicrosecondType.html#impl-ArrowPrimitiveType-for-DurationMicrosecondType" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.DurationMicrosecondType.html" class="struct" title="struct arrow::datatypes::DurationMicrosecondType">DurationMicrosecondType</a>

<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.DurationMicrosecondType.html#associatedconstant.DATA_TYPE" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedconstant.DATA_TYPE" class="constant">DATA_TYPE</a>: <a href="https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html" class="enum" title="enum arrow::datatypes::DataType">DataType</a>

the corresponding Arrow data type of this primitive type.

<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.DurationMicrosecondType.html#associatedtype.Native" class="anchor">§</a>

#### type <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype">Native</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>

Corresponding Rust native type for the primitive type.

<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.DurationMicrosecondType.html#method.default_value" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#method.default_value" class="fn">default_value</a>() -\> Self::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>

Returns a default value of this primitive type. [Read more](https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#method.default_value)

<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.DurationMicrosecondType.html#impl-Debug-for-DurationMicrosecondType" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.DurationMicrosecondType.html" class="struct" title="struct arrow::datatypes::DurationMicrosecondType">DurationMicrosecondType</a>

<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.DurationMicrosecondType.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.DurationMicrosecondType.html#impl-Parser-for-DurationMicrosecondType" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/compute/kernels/cast_utils/trait.Parser.html" class="trait" title="trait arrow::compute::kernels::cast_utils::Parser">Parser</a> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.DurationMicrosecondType.html" class="struct" title="struct arrow::datatypes::DurationMicrosecondType">DurationMicrosecondType</a>

<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.DurationMicrosecondType.html#method.parse" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/compute/kernels/cast_utils/trait.Parser.html#tymethod.parse" class="fn">parse</a>( string: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.DurationMicrosecondType.html" class="struct" title="struct arrow::datatypes::DurationMicrosecondType">DurationMicrosecondType</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>

Parse a string to the native type

<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.DurationMicrosecondType.html#method.parse_formatted" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/compute/kernels/cast_utils/trait.Parser.html#method.parse_formatted" class="fn">parse_formatted</a>(string: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, \_format: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<Self::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>

Parse a string to the native type with a format string [Read more](https://docs.rs/arrow/latest/arrow/compute/kernels/cast_utils/trait.Parser.html#method.parse_formatted)

<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.DurationMicrosecondType.html#impl-ArrowTemporalType-for-DurationMicrosecondType" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowTemporalType.html" class="trait" title="trait arrow::datatypes::ArrowTemporalType">ArrowTemporalType</a> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.DurationMicrosecondType.html" class="struct" title="struct arrow::datatypes::DurationMicrosecondType">DurationMicrosecondType</a>

## Auto Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.DurationMicrosecondType.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.DurationMicrosecondType.html#blanket-implementations" class="anchor">§</a>
