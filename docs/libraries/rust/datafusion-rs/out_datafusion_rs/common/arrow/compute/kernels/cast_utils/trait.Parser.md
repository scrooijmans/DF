# Trait Parser Copy item path

<a href="https://docs.rs/arrow-cast/56.0.0/x86_64-unknown-linux-gnu/src/arrow_cast/parse.rs.html#434" class="src">Source</a>

``` rust
pub trait Parser: ArrowPrimitiveType {
    // Required method
    fn parse(string: &str) -> Option<Self::Native>;

    // Provided method
    fn parse_formatted(string: &str, _format: &str) -> Option<Self::Native> { ... }
}
```

Expand description

Specialized parsing implementations to convert strings to Arrow types.

This is used by csv and json reader and can be used directly as well.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/kernels/cast_utils/trait.Parser.html#example" class="doc-anchor">§</a>Example

To parse a string to a [`Date32Type`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Date32Type.html "struct datafusion::common::arrow::datatypes::Date32Type"):

``` rust
use arrow_cast::parse::Parser;
use arrow_array::types::Date32Type;
let date = Date32Type::parse("2021-01-01").unwrap();
assert_eq!(date, 18628);
```

To parse a string to a [`TimestampNanosecondType`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.TimestampNanosecondType.html "struct datafusion::common::arrow::datatypes::TimestampNanosecondType"):

``` rust
use arrow_cast::parse::Parser;
use arrow_array::types::TimestampNanosecondType;
let ts = TimestampNanosecondType::parse("2021-01-01T00:00:00.123456789Z").unwrap();
assert_eq!(ts, 1609459200123456789);
```

## Required Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/kernels/cast_utils/trait.Parser.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/kernels/cast_utils/trait.Parser.html#tymethod.parse" class="fn">parse</a>(string: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<Self::<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type datafusion::common::arrow::array::ArrowPrimitiveType::Native">Native</a>\>

Parse a string to the native type

## Provided Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/kernels/cast_utils/trait.Parser.html#provided-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/kernels/cast_utils/trait.Parser.html#method.parse_formatted" class="fn">parse_formatted</a>(string: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, \_format: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<Self::<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type datafusion::common::arrow::array::ArrowPrimitiveType::Native">Native</a>\>

Parse a string to the native type with a format string

When not implemented, the format string is unused, and this method is equivalent to [parse](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/kernels/cast_utils/trait.Parser.html#tymethod.parse)

## Dyn Compatibility<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/kernels/cast_utils/trait.Parser.html#dyn-compatibility" class="anchor">§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementors<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/kernels/cast_utils/trait.Parser.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/kernels/cast_utils/trait.Parser.html#impl-Parser-for-Date32Type" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/kernels/cast_utils/trait.Parser.html" class="trait" title="trait datafusion::common::arrow::compute::kernels::cast_utils::Parser">Parser</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Date32Type.html" class="struct" title="struct datafusion::common::arrow::datatypes::Date32Type">Date32Type</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/kernels/cast_utils/trait.Parser.html#impl-Parser-for-Date64Type" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/kernels/cast_utils/trait.Parser.html" class="trait" title="trait datafusion::common::arrow::compute::kernels::cast_utils::Parser">Parser</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Date64Type.html" class="struct" title="struct datafusion::common::arrow::datatypes::Date64Type">Date64Type</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/kernels/cast_utils/trait.Parser.html#impl-Parser-for-DurationMicrosecondType" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/kernels/cast_utils/trait.Parser.html" class="trait" title="trait datafusion::common::arrow::compute::kernels::cast_utils::Parser">Parser</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.DurationMicrosecondType.html" class="struct" title="struct datafusion::common::arrow::datatypes::DurationMicrosecondType">DurationMicrosecondType</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/kernels/cast_utils/trait.Parser.html#impl-Parser-for-DurationMillisecondType" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/kernels/cast_utils/trait.Parser.html" class="trait" title="trait datafusion::common::arrow::compute::kernels::cast_utils::Parser">Parser</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.DurationMillisecondType.html" class="struct" title="struct datafusion::common::arrow::datatypes::DurationMillisecondType">DurationMillisecondType</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/kernels/cast_utils/trait.Parser.html#impl-Parser-for-DurationNanosecondType" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/kernels/cast_utils/trait.Parser.html" class="trait" title="trait datafusion::common::arrow::compute::kernels::cast_utils::Parser">Parser</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.DurationNanosecondType.html" class="struct" title="struct datafusion::common::arrow::datatypes::DurationNanosecondType">DurationNanosecondType</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/kernels/cast_utils/trait.Parser.html#impl-Parser-for-DurationSecondType" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/kernels/cast_utils/trait.Parser.html" class="trait" title="trait datafusion::common::arrow::compute::kernels::cast_utils::Parser">Parser</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.DurationSecondType.html" class="struct" title="struct datafusion::common::arrow::datatypes::DurationSecondType">DurationSecondType</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/kernels/cast_utils/trait.Parser.html#impl-Parser-for-Float16Type" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/kernels/cast_utils/trait.Parser.html" class="trait" title="trait datafusion::common::arrow::compute::kernels::cast_utils::Parser">Parser</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Float16Type.html" class="struct" title="struct datafusion::common::arrow::datatypes::Float16Type">Float16Type</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/kernels/cast_utils/trait.Parser.html#impl-Parser-for-Float32Type" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/kernels/cast_utils/trait.Parser.html" class="trait" title="trait datafusion::common::arrow::compute::kernels::cast_utils::Parser">Parser</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Float32Type.html" class="struct" title="struct datafusion::common::arrow::datatypes::Float32Type">Float32Type</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/kernels/cast_utils/trait.Parser.html#impl-Parser-for-Float64Type" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/kernels/cast_utils/trait.Parser.html" class="trait" title="trait datafusion::common::arrow::compute::kernels::cast_utils::Parser">Parser</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Float64Type.html" class="struct" title="struct datafusion::common::arrow::datatypes::Float64Type">Float64Type</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/kernels/cast_utils/trait.Parser.html#impl-Parser-for-Int8Type" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/kernels/cast_utils/trait.Parser.html" class="trait" title="trait datafusion::common::arrow::compute::kernels::cast_utils::Parser">Parser</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Int8Type.html" class="struct" title="struct datafusion::common::arrow::datatypes::Int8Type">Int8Type</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/kernels/cast_utils/trait.Parser.html#impl-Parser-for-Int16Type" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/kernels/cast_utils/trait.Parser.html" class="trait" title="trait datafusion::common::arrow::compute::kernels::cast_utils::Parser">Parser</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Int16Type.html" class="struct" title="struct datafusion::common::arrow::datatypes::Int16Type">Int16Type</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/kernels/cast_utils/trait.Parser.html#impl-Parser-for-Int32Type" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/kernels/cast_utils/trait.Parser.html" class="trait" title="trait datafusion::common::arrow::compute::kernels::cast_utils::Parser">Parser</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Int32Type.html" class="struct" title="struct datafusion::common::arrow::datatypes::Int32Type">Int32Type</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/kernels/cast_utils/trait.Parser.html#impl-Parser-for-Int64Type" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/kernels/cast_utils/trait.Parser.html" class="trait" title="trait datafusion::common::arrow::compute::kernels::cast_utils::Parser">Parser</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Int64Type.html" class="struct" title="struct datafusion::common::arrow::datatypes::Int64Type">Int64Type</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/kernels/cast_utils/trait.Parser.html#impl-Parser-for-Time32MillisecondType" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/kernels/cast_utils/trait.Parser.html" class="trait" title="trait datafusion::common::arrow::compute::kernels::cast_utils::Parser">Parser</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Time32MillisecondType.html" class="struct" title="struct datafusion::common::arrow::datatypes::Time32MillisecondType">Time32MillisecondType</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/kernels/cast_utils/trait.Parser.html#impl-Parser-for-Time32SecondType" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/kernels/cast_utils/trait.Parser.html" class="trait" title="trait datafusion::common::arrow::compute::kernels::cast_utils::Parser">Parser</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Time32SecondType.html" class="struct" title="struct datafusion::common::arrow::datatypes::Time32SecondType">Time32SecondType</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/kernels/cast_utils/trait.Parser.html#impl-Parser-for-Time64MicrosecondType" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/kernels/cast_utils/trait.Parser.html" class="trait" title="trait datafusion::common::arrow::compute::kernels::cast_utils::Parser">Parser</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Time64MicrosecondType.html" class="struct" title="struct datafusion::common::arrow::datatypes::Time64MicrosecondType">Time64MicrosecondType</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/kernels/cast_utils/trait.Parser.html#impl-Parser-for-Time64NanosecondType" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/kernels/cast_utils/trait.Parser.html" class="trait" title="trait datafusion::common::arrow::compute::kernels::cast_utils::Parser">Parser</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Time64NanosecondType.html" class="struct" title="struct datafusion::common::arrow::datatypes::Time64NanosecondType">Time64NanosecondType</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/kernels/cast_utils/trait.Parser.html#impl-Parser-for-TimestampMicrosecondType" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/kernels/cast_utils/trait.Parser.html" class="trait" title="trait datafusion::common::arrow::compute::kernels::cast_utils::Parser">Parser</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.TimestampMicrosecondType.html" class="struct" title="struct datafusion::common::arrow::datatypes::TimestampMicrosecondType">TimestampMicrosecondType</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/kernels/cast_utils/trait.Parser.html#impl-Parser-for-TimestampMillisecondType" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/kernels/cast_utils/trait.Parser.html" class="trait" title="trait datafusion::common::arrow::compute::kernels::cast_utils::Parser">Parser</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.TimestampMillisecondType.html" class="struct" title="struct datafusion::common::arrow::datatypes::TimestampMillisecondType">TimestampMillisecondType</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/kernels/cast_utils/trait.Parser.html#impl-Parser-for-TimestampNanosecondType" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/kernels/cast_utils/trait.Parser.html" class="trait" title="trait datafusion::common::arrow::compute::kernels::cast_utils::Parser">Parser</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.TimestampNanosecondType.html" class="struct" title="struct datafusion::common::arrow::datatypes::TimestampNanosecondType">TimestampNanosecondType</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/kernels/cast_utils/trait.Parser.html#impl-Parser-for-TimestampSecondType" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/kernels/cast_utils/trait.Parser.html" class="trait" title="trait datafusion::common::arrow::compute::kernels::cast_utils::Parser">Parser</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.TimestampSecondType.html" class="struct" title="struct datafusion::common::arrow::datatypes::TimestampSecondType">TimestampSecondType</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/kernels/cast_utils/trait.Parser.html#impl-Parser-for-UInt8Type" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/kernels/cast_utils/trait.Parser.html" class="trait" title="trait datafusion::common::arrow::compute::kernels::cast_utils::Parser">Parser</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.UInt8Type.html" class="struct" title="struct datafusion::common::arrow::datatypes::UInt8Type">UInt8Type</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/kernels/cast_utils/trait.Parser.html#impl-Parser-for-UInt16Type" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/kernels/cast_utils/trait.Parser.html" class="trait" title="trait datafusion::common::arrow::compute::kernels::cast_utils::Parser">Parser</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.UInt16Type.html" class="struct" title="struct datafusion::common::arrow::datatypes::UInt16Type">UInt16Type</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/kernels/cast_utils/trait.Parser.html#impl-Parser-for-UInt32Type" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/kernels/cast_utils/trait.Parser.html" class="trait" title="trait datafusion::common::arrow::compute::kernels::cast_utils::Parser">Parser</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.UInt32Type.html" class="struct" title="struct datafusion::common::arrow::datatypes::UInt32Type">UInt32Type</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/kernels/cast_utils/trait.Parser.html#impl-Parser-for-UInt64Type" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/kernels/cast_utils/trait.Parser.html" class="trait" title="trait datafusion::common::arrow::compute::kernels::cast_utils::Parser">Parser</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.UInt64Type.html" class="struct" title="struct datafusion::common::arrow::datatypes::UInt64Type">UInt64Type</a>
