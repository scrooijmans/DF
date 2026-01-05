# Struct Decimal64Type Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/types.rs.html#1420" class="src">Source</a>

``` rust
pub struct Decimal64Type {}
```

Expand description

The decimal type for a Decimal64Array

## Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.Decimal64Type.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.Decimal64Type.html#impl-ArrowPrimitiveType-for-Decimal64Type" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Decimal64Type.html" class="struct" title="struct arrow::datatypes::Decimal64Type">Decimal64Type</a>

<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.Decimal64Type.html#associatedconstant.DATA_TYPE" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedconstant.DATA_TYPE" class="constant">DATA_TYPE</a>: <a href="https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html" class="enum" title="enum arrow::datatypes::DataType">DataType</a> = \<Self as DecimalType\>::DEFAULT_TYPE

the corresponding Arrow data type of this primitive type.

<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.Decimal64Type.html#associatedtype.Native" class="anchor">§</a>

#### type <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype">Native</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>

Corresponding Rust native type for the primitive type.

<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.Decimal64Type.html#method.default_value" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#method.default_value" class="fn">default_value</a>() -\> Self::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>

Returns a default value of this primitive type. [Read more](https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#method.default_value)

<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.Decimal64Type.html#impl-Debug-for-Decimal64Type" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Decimal64Type.html" class="struct" title="struct arrow::datatypes::Decimal64Type">Decimal64Type</a>

<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.Decimal64Type.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.Decimal64Type.html#impl-DecimalType-for-Decimal64Type" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.DecimalType.html" class="trait" title="trait arrow::datatypes::DecimalType">DecimalType</a> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Decimal64Type.html" class="struct" title="struct arrow::datatypes::Decimal64Type">Decimal64Type</a>

<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.Decimal64Type.html#associatedconstant.BYTE_LENGTH" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.DecimalType.html#associatedconstant.BYTE_LENGTH" class="constant">BYTE_LENGTH</a>: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a> = 8usize

Width of the type

<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.Decimal64Type.html#associatedconstant.MAX_PRECISION" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.DecimalType.html#associatedconstant.MAX_PRECISION" class="constant">MAX_PRECISION</a>: <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a> = 18u8

Maximum number of significant digits

<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.Decimal64Type.html#associatedconstant.MAX_SCALE" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.DecimalType.html#associatedconstant.MAX_SCALE" class="constant">MAX_SCALE</a>: <a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a> = 18i8

Maximum no of digits after the decimal point (note the scale can be negative)

<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.Decimal64Type.html#associatedconstant.TYPE_CONSTRUCTOR" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.DecimalType.html#associatedconstant.TYPE_CONSTRUCTOR" class="constant">TYPE_CONSTRUCTOR</a>: <a href="https://doc.rust-lang.org/nightly/std/primitive.fn.html" class="primitive">fn</a>(<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html" class="enum" title="enum arrow::datatypes::DataType">DataType</a> = {arrow_schema::DataType::Decimal64 as fn(u8, i8) -\> arrow_schema::DataType}

fn to create its [`DataType`](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html "enum arrow::datatypes::DataType")

<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.Decimal64Type.html#associatedconstant.DEFAULT_TYPE" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.DecimalType.html#associatedconstant.DEFAULT_TYPE" class="constant">DEFAULT_TYPE</a>: <a href="https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html" class="enum" title="enum arrow::datatypes::DataType">DataType</a>

Default values for [`DataType`](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html "enum arrow::datatypes::DataType")

<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.Decimal64Type.html#associatedconstant.PREFIX" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.DecimalType.html#associatedconstant.PREFIX" class="constant">PREFIX</a>: &'static <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a> = "Decimal64"

“Decimal32”, “Decimal64”, “Decimal128” or “Decimal256”, for use in error messages

<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.Decimal64Type.html#method.format_decimal" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.DecimalType.html#tymethod.format_decimal" class="fn">format_decimal</a>( value: \<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Decimal64Type.html" class="struct" title="struct arrow::datatypes::Decimal64Type">Decimal64Type</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>, precision: <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>, scale: <a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>

Formats the decimal value with the provided precision and scale

<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.Decimal64Type.html#method.validate_decimal_precision" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.DecimalType.html#tymethod.validate_decimal_precision" class="fn">validate_decimal_precision</a>(num: <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>, precision: <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

Validates that `value` contains no more than `precision` decimal digits

<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.Decimal64Type.html#method.is_valid_decimal_precision" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.DecimalType.html#tymethod.is_valid_decimal_precision" class="fn">is_valid_decimal_precision</a>( value: \<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Decimal64Type.html" class="struct" title="struct arrow::datatypes::Decimal64Type">Decimal64Type</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>, precision: <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Determines whether `value` contains no more than `precision` decimal digits

## Auto Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.Decimal64Type.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.Decimal64Type.html#blanket-implementations" class="anchor">§</a>
