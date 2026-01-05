# Trait DecimalType Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/types.rs.html#1317-1318" class="src">Source</a>

``` rust
pub trait DecimalType:
    'static
    + Send
    + Sync
    + ArrowPrimitiveType
    + DecimalTypeSealed {
    const BYTE_LENGTH: usize;
    const MAX_PRECISION: u8;
    const MAX_SCALE: i8;
    const TYPE_CONSTRUCTOR: fn(u8, i8) -> DataType;
    const DEFAULT_TYPE: DataType;
    const PREFIX: &'static str;

    // Required methods
    fn format_decimal(value: Self::Native, precision: u8, scale: i8) -> String;
    fn validate_decimal_precision(
        value: Self::Native,
        precision: u8,
    ) -> Result<(), ArrowError>;
    fn is_valid_decimal_precision(value: Self::Native, precision: u8) -> bool;
}
```

Expand description

A trait over the decimal types, used by [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray") to provide a generic implementation across the various decimal types

Implemented by [`Decimal32Type`](https://docs.rs/arrow/latest/arrow/datatypes/struct.Decimal32Type.html "struct arrow::datatypes::Decimal32Type"), [`Decimal64Type`](https://docs.rs/arrow/latest/arrow/datatypes/struct.Decimal64Type.html "struct arrow::datatypes::Decimal64Type"), [`Decimal128Type`](https://docs.rs/arrow/latest/arrow/datatypes/struct.Decimal128Type.html "struct arrow::datatypes::Decimal128Type") and [`Decimal256Type`](https://docs.rs/arrow/latest/arrow/datatypes/struct.Decimal256Type.html "struct arrow::datatypes::Decimal256Type") for [`Decimal32Array`](https://docs.rs/arrow/latest/arrow/array/type.Decimal32Array.html "type arrow::array::Decimal32Array"), [`Decimal64Array`](https://docs.rs/arrow/latest/arrow/array/type.Decimal64Array.html "type arrow::array::Decimal64Array"), [`Decimal128Array`](https://docs.rs/arrow/latest/arrow/array/type.Decimal128Array.html "type arrow::array::Decimal128Array") and [`Decimal256Array`](https://docs.rs/arrow/latest/arrow/array/type.Decimal256Array.html "type arrow::array::Decimal256Array") respectively

## Required Associated Constants<a href="https://docs.rs/arrow/latest/arrow/array/types/trait.DecimalType.html#required-associated-consts" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/array/types/trait.DecimalType.html#associatedconstant.BYTE_LENGTH" class="constant">BYTE_LENGTH</a>: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Width of the type

#### const <a href="https://docs.rs/arrow/latest/arrow/array/types/trait.DecimalType.html#associatedconstant.MAX_PRECISION" class="constant">MAX_PRECISION</a>: <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>

Maximum number of significant digits

#### const <a href="https://docs.rs/arrow/latest/arrow/array/types/trait.DecimalType.html#associatedconstant.MAX_SCALE" class="constant">MAX_SCALE</a>: <a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>

Maximum no of digits after the decimal point (note the scale can be negative)

#### const <a href="https://docs.rs/arrow/latest/arrow/array/types/trait.DecimalType.html#associatedconstant.TYPE_CONSTRUCTOR" class="constant">TYPE_CONSTRUCTOR</a>: <a href="https://doc.rust-lang.org/nightly/std/primitive.fn.html" class="primitive">fn</a>(<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html" class="enum" title="enum arrow::datatypes::DataType">DataType</a>

fn to create its [`DataType`](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html "enum arrow::datatypes::DataType")

#### const <a href="https://docs.rs/arrow/latest/arrow/array/types/trait.DecimalType.html#associatedconstant.DEFAULT_TYPE" class="constant">DEFAULT_TYPE</a>: <a href="https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html" class="enum" title="enum arrow::datatypes::DataType">DataType</a>

Default values for [`DataType`](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html "enum arrow::datatypes::DataType")

#### const <a href="https://docs.rs/arrow/latest/arrow/array/types/trait.DecimalType.html#associatedconstant.PREFIX" class="constant">PREFIX</a>: &'static <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

“Decimal32”, “Decimal64”, “Decimal128” or “Decimal256”, for use in error messages

## Required Methods<a href="https://docs.rs/arrow/latest/arrow/array/types/trait.DecimalType.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/types/trait.DecimalType.html#tymethod.format_decimal" class="fn">format_decimal</a>(value: Self::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>, precision: <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>, scale: <a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>) -\> <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>

Formats the decimal value with the provided precision and scale

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/types/trait.DecimalType.html#tymethod.validate_decimal_precision" class="fn">validate_decimal_precision</a>( value: Self::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>, precision: <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

Validates that `value` contains no more than `precision` decimal digits

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/types/trait.DecimalType.html#tymethod.is_valid_decimal_precision" class="fn">is_valid_decimal_precision</a>(value: Self::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>, precision: <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Determines whether `value` contains no more than `precision` decimal digits

## Dyn Compatibility<a href="https://docs.rs/arrow/latest/arrow/array/types/trait.DecimalType.html#dyn-compatibility" class="anchor">§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementors<a href="https://docs.rs/arrow/latest/arrow/array/types/trait.DecimalType.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/types/trait.DecimalType.html#impl-DecimalType-for-Decimal32Type" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.DecimalType.html" class="trait" title="trait arrow::datatypes::DecimalType">DecimalType</a> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Decimal32Type.html" class="struct" title="struct arrow::datatypes::Decimal32Type">Decimal32Type</a>

<a href="https://docs.rs/arrow/latest/arrow/array/types/trait.DecimalType.html#associatedconstant.BYTE_LENGTH-1" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/array/types/trait.DecimalType.html#associatedconstant.BYTE_LENGTH" class="constant">BYTE_LENGTH</a>: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a> = 4usize

<a href="https://docs.rs/arrow/latest/arrow/array/types/trait.DecimalType.html#associatedconstant.MAX_PRECISION-1" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/array/types/trait.DecimalType.html#associatedconstant.MAX_PRECISION" class="constant">MAX_PRECISION</a>: <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a> = 9u8

<a href="https://docs.rs/arrow/latest/arrow/array/types/trait.DecimalType.html#associatedconstant.MAX_SCALE-1" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/array/types/trait.DecimalType.html#associatedconstant.MAX_SCALE" class="constant">MAX_SCALE</a>: <a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a> = 9i8

<a href="https://docs.rs/arrow/latest/arrow/array/types/trait.DecimalType.html#associatedconstant.TYPE_CONSTRUCTOR-1" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/array/types/trait.DecimalType.html#associatedconstant.TYPE_CONSTRUCTOR" class="constant">TYPE_CONSTRUCTOR</a>: <a href="https://doc.rust-lang.org/nightly/std/primitive.fn.html" class="primitive">fn</a>(<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html" class="enum" title="enum arrow::datatypes::DataType">DataType</a> = {arrow_schema::DataType::Decimal32 as fn(u8, i8) -\> arrow_schema::DataType}

<a href="https://docs.rs/arrow/latest/arrow/array/types/trait.DecimalType.html#associatedconstant.DEFAULT_TYPE-1" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/array/types/trait.DecimalType.html#associatedconstant.DEFAULT_TYPE" class="constant">DEFAULT_TYPE</a>: <a href="https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html" class="enum" title="enum arrow::datatypes::DataType">DataType</a>

<a href="https://docs.rs/arrow/latest/arrow/array/types/trait.DecimalType.html#associatedconstant.PREFIX-1" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/array/types/trait.DecimalType.html#associatedconstant.PREFIX" class="constant">PREFIX</a>: &'static <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a> = "Decimal32"

<a href="https://docs.rs/arrow/latest/arrow/array/types/trait.DecimalType.html#impl-DecimalType-for-Decimal64Type" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.DecimalType.html" class="trait" title="trait arrow::datatypes::DecimalType">DecimalType</a> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Decimal64Type.html" class="struct" title="struct arrow::datatypes::Decimal64Type">Decimal64Type</a>

<a href="https://docs.rs/arrow/latest/arrow/array/types/trait.DecimalType.html#associatedconstant.BYTE_LENGTH-2" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/array/types/trait.DecimalType.html#associatedconstant.BYTE_LENGTH" class="constant">BYTE_LENGTH</a>: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a> = 8usize

<a href="https://docs.rs/arrow/latest/arrow/array/types/trait.DecimalType.html#associatedconstant.MAX_PRECISION-2" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/array/types/trait.DecimalType.html#associatedconstant.MAX_PRECISION" class="constant">MAX_PRECISION</a>: <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a> = 18u8

<a href="https://docs.rs/arrow/latest/arrow/array/types/trait.DecimalType.html#associatedconstant.MAX_SCALE-2" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/array/types/trait.DecimalType.html#associatedconstant.MAX_SCALE" class="constant">MAX_SCALE</a>: <a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a> = 18i8

<a href="https://docs.rs/arrow/latest/arrow/array/types/trait.DecimalType.html#associatedconstant.TYPE_CONSTRUCTOR-2" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/array/types/trait.DecimalType.html#associatedconstant.TYPE_CONSTRUCTOR" class="constant">TYPE_CONSTRUCTOR</a>: <a href="https://doc.rust-lang.org/nightly/std/primitive.fn.html" class="primitive">fn</a>(<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html" class="enum" title="enum arrow::datatypes::DataType">DataType</a> = {arrow_schema::DataType::Decimal64 as fn(u8, i8) -\> arrow_schema::DataType}

<a href="https://docs.rs/arrow/latest/arrow/array/types/trait.DecimalType.html#associatedconstant.DEFAULT_TYPE-2" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/array/types/trait.DecimalType.html#associatedconstant.DEFAULT_TYPE" class="constant">DEFAULT_TYPE</a>: <a href="https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html" class="enum" title="enum arrow::datatypes::DataType">DataType</a>

<a href="https://docs.rs/arrow/latest/arrow/array/types/trait.DecimalType.html#associatedconstant.PREFIX-2" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/array/types/trait.DecimalType.html#associatedconstant.PREFIX" class="constant">PREFIX</a>: &'static <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a> = "Decimal64"

<a href="https://docs.rs/arrow/latest/arrow/array/types/trait.DecimalType.html#impl-DecimalType-for-Decimal128Type" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.DecimalType.html" class="trait" title="trait arrow::datatypes::DecimalType">DecimalType</a> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Decimal128Type.html" class="struct" title="struct arrow::datatypes::Decimal128Type">Decimal128Type</a>

<a href="https://docs.rs/arrow/latest/arrow/array/types/trait.DecimalType.html#associatedconstant.BYTE_LENGTH-3" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/array/types/trait.DecimalType.html#associatedconstant.BYTE_LENGTH" class="constant">BYTE_LENGTH</a>: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a> = 16usize

<a href="https://docs.rs/arrow/latest/arrow/array/types/trait.DecimalType.html#associatedconstant.MAX_PRECISION-3" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/array/types/trait.DecimalType.html#associatedconstant.MAX_PRECISION" class="constant">MAX_PRECISION</a>: <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a> = 38u8

<a href="https://docs.rs/arrow/latest/arrow/array/types/trait.DecimalType.html#associatedconstant.MAX_SCALE-3" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/array/types/trait.DecimalType.html#associatedconstant.MAX_SCALE" class="constant">MAX_SCALE</a>: <a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a> = 38i8

<a href="https://docs.rs/arrow/latest/arrow/array/types/trait.DecimalType.html#associatedconstant.TYPE_CONSTRUCTOR-3" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/array/types/trait.DecimalType.html#associatedconstant.TYPE_CONSTRUCTOR" class="constant">TYPE_CONSTRUCTOR</a>: <a href="https://doc.rust-lang.org/nightly/std/primitive.fn.html" class="primitive">fn</a>(<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html" class="enum" title="enum arrow::datatypes::DataType">DataType</a> = {arrow_schema::DataType::Decimal128 as fn(u8, i8) -\> arrow_schema::DataType}

<a href="https://docs.rs/arrow/latest/arrow/array/types/trait.DecimalType.html#associatedconstant.DEFAULT_TYPE-3" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/array/types/trait.DecimalType.html#associatedconstant.DEFAULT_TYPE" class="constant">DEFAULT_TYPE</a>: <a href="https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html" class="enum" title="enum arrow::datatypes::DataType">DataType</a>

<a href="https://docs.rs/arrow/latest/arrow/array/types/trait.DecimalType.html#associatedconstant.PREFIX-3" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/array/types/trait.DecimalType.html#associatedconstant.PREFIX" class="constant">PREFIX</a>: &'static <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a> = "Decimal128"

<a href="https://docs.rs/arrow/latest/arrow/array/types/trait.DecimalType.html#impl-DecimalType-for-Decimal256Type" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.DecimalType.html" class="trait" title="trait arrow::datatypes::DecimalType">DecimalType</a> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Decimal256Type.html" class="struct" title="struct arrow::datatypes::Decimal256Type">Decimal256Type</a>

<a href="https://docs.rs/arrow/latest/arrow/array/types/trait.DecimalType.html#associatedconstant.BYTE_LENGTH-4" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/array/types/trait.DecimalType.html#associatedconstant.BYTE_LENGTH" class="constant">BYTE_LENGTH</a>: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a> = 32usize

<a href="https://docs.rs/arrow/latest/arrow/array/types/trait.DecimalType.html#associatedconstant.MAX_PRECISION-4" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/array/types/trait.DecimalType.html#associatedconstant.MAX_PRECISION" class="constant">MAX_PRECISION</a>: <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a> = 76u8

<a href="https://docs.rs/arrow/latest/arrow/array/types/trait.DecimalType.html#associatedconstant.MAX_SCALE-4" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/array/types/trait.DecimalType.html#associatedconstant.MAX_SCALE" class="constant">MAX_SCALE</a>: <a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a> = 76i8

<a href="https://docs.rs/arrow/latest/arrow/array/types/trait.DecimalType.html#associatedconstant.TYPE_CONSTRUCTOR-4" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/array/types/trait.DecimalType.html#associatedconstant.TYPE_CONSTRUCTOR" class="constant">TYPE_CONSTRUCTOR</a>: <a href="https://doc.rust-lang.org/nightly/std/primitive.fn.html" class="primitive">fn</a>(<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html" class="enum" title="enum arrow::datatypes::DataType">DataType</a> = {arrow_schema::DataType::Decimal256 as fn(u8, i8) -\> arrow_schema::DataType}

<a href="https://docs.rs/arrow/latest/arrow/array/types/trait.DecimalType.html#associatedconstant.DEFAULT_TYPE-4" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/array/types/trait.DecimalType.html#associatedconstant.DEFAULT_TYPE" class="constant">DEFAULT_TYPE</a>: <a href="https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html" class="enum" title="enum arrow::datatypes::DataType">DataType</a>

<a href="https://docs.rs/arrow/latest/arrow/array/types/trait.DecimalType.html#associatedconstant.PREFIX-4" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/array/types/trait.DecimalType.html#associatedconstant.PREFIX" class="constant">PREFIX</a>: &'static <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a> = "Decimal256"
