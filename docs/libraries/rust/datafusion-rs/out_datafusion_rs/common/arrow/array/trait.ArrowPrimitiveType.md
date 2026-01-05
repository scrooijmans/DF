# Trait ArrowPrimitiveType Copy item path

<a href="https://docs.rs/arrow-array/56.0.0/x86_64-unknown-linux-gnu/src/arrow_array/types.rs.html#67" class="src">Source</a>

``` rust
pub trait ArrowPrimitiveType: PrimitiveTypeSealed + 'static {
    type Native: ArrowNativeTypeOp;

    const DATA_TYPE: DataType;

    // Provided method
    fn default_value() -> Self::Native { ... }
}
```

Expand description

Trait for [primitive values](https://arrow.apache.org/docs/format/Columnar.html#fixed-size-primitive-layout).

This trait bridges the dynamic-typed nature of Arrow (via [`DataType`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html "enum datafusion::common::arrow::datatypes::DataType")) with the static-typed nature of rust types ([`ArrowNativeType`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/trait.ArrowNativeType.html "trait datafusion::common::arrow::datatypes::ArrowNativeType")) for all types that implement [`ArrowNativeType`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/trait.ArrowNativeType.html "trait datafusion::common::arrow::datatypes::ArrowNativeType").

## Required Associated Constants<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#required-associated-consts" class="anchor">§</a>

#### const <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedconstant.DATA_TYPE" class="constant">DATA_TYPE</a>: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>

the corresponding Arrow data type of this primitive type.

## Required Associated Types<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#required-associated-types" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype">Native</a>: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowNativeTypeOp.html" class="trait" title="trait datafusion::common::arrow::array::ArrowNativeTypeOp">ArrowNativeTypeOp</a>

Corresponding Rust native type for the primitive type.

## Provided Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#provided-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#method.default_value" class="fn">default_value</a>() -\> Self::<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type datafusion::common::arrow::array::ArrowPrimitiveType::Native">Native</a>

Returns a default value of this primitive type.

This is useful for aggregate array ops like `sum()`, `mean()`.

## Dyn Compatibility<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#dyn-compatibility" class="anchor">§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementors<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#impl-ArrowPrimitiveType-for-Date32Type" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait datafusion::common::arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Date32Type.html" class="struct" title="struct datafusion::common::arrow::datatypes::Date32Type">Date32Type</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedconstant.DATA_TYPE-1" class="anchor">§</a>

#### const <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedconstant.DATA_TYPE" class="constant">DATA_TYPE</a>: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a> = DataType::Date32

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native-1" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype">Native</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#impl-ArrowPrimitiveType-for-Date64Type" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait datafusion::common::arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Date64Type.html" class="struct" title="struct datafusion::common::arrow::datatypes::Date64Type">Date64Type</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedconstant.DATA_TYPE-2" class="anchor">§</a>

#### const <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedconstant.DATA_TYPE" class="constant">DATA_TYPE</a>: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a> = DataType::Date64

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native-2" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype">Native</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#impl-ArrowPrimitiveType-for-Decimal32Type" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait datafusion::common::arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Decimal32Type.html" class="struct" title="struct datafusion::common::arrow::datatypes::Decimal32Type">Decimal32Type</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedconstant.DATA_TYPE-3" class="anchor">§</a>

#### const <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedconstant.DATA_TYPE" class="constant">DATA_TYPE</a>: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a> = \<Self as DecimalType\>::DEFAULT_TYPE

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native-3" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype">Native</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#impl-ArrowPrimitiveType-for-Decimal64Type" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait datafusion::common::arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Decimal64Type.html" class="struct" title="struct datafusion::common::arrow::datatypes::Decimal64Type">Decimal64Type</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedconstant.DATA_TYPE-4" class="anchor">§</a>

#### const <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedconstant.DATA_TYPE" class="constant">DATA_TYPE</a>: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a> = \<Self as DecimalType\>::DEFAULT_TYPE

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native-4" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype">Native</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#impl-ArrowPrimitiveType-for-Decimal128Type" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait datafusion::common::arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Decimal128Type.html" class="struct" title="struct datafusion::common::arrow::datatypes::Decimal128Type">Decimal128Type</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedconstant.DATA_TYPE-5" class="anchor">§</a>

#### const <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedconstant.DATA_TYPE" class="constant">DATA_TYPE</a>: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a> = \<Self as DecimalType\>::DEFAULT_TYPE

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native-5" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype">Native</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#impl-ArrowPrimitiveType-for-Decimal256Type" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait datafusion::common::arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Decimal256Type.html" class="struct" title="struct datafusion::common::arrow::datatypes::Decimal256Type">Decimal256Type</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedconstant.DATA_TYPE-6" class="anchor">§</a>

#### const <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedconstant.DATA_TYPE" class="constant">DATA_TYPE</a>: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a> = \<Self as DecimalType\>::DEFAULT_TYPE

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native-6" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype">Native</a> = <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.i256.html" class="struct" title="struct datafusion::common::arrow::datatypes::i256">i256</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#impl-ArrowPrimitiveType-for-DurationMicrosecondType" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait datafusion::common::arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.DurationMicrosecondType.html" class="struct" title="struct datafusion::common::arrow::datatypes::DurationMicrosecondType">DurationMicrosecondType</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedconstant.DATA_TYPE-7" class="anchor">§</a>

#### const <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedconstant.DATA_TYPE" class="constant">DATA_TYPE</a>: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native-7" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype">Native</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#impl-ArrowPrimitiveType-for-DurationMillisecondType" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait datafusion::common::arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.DurationMillisecondType.html" class="struct" title="struct datafusion::common::arrow::datatypes::DurationMillisecondType">DurationMillisecondType</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedconstant.DATA_TYPE-8" class="anchor">§</a>

#### const <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedconstant.DATA_TYPE" class="constant">DATA_TYPE</a>: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native-8" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype">Native</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#impl-ArrowPrimitiveType-for-DurationNanosecondType" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait datafusion::common::arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.DurationNanosecondType.html" class="struct" title="struct datafusion::common::arrow::datatypes::DurationNanosecondType">DurationNanosecondType</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedconstant.DATA_TYPE-9" class="anchor">§</a>

#### const <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedconstant.DATA_TYPE" class="constant">DATA_TYPE</a>: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native-9" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype">Native</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#impl-ArrowPrimitiveType-for-DurationSecondType" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait datafusion::common::arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.DurationSecondType.html" class="struct" title="struct datafusion::common::arrow::datatypes::DurationSecondType">DurationSecondType</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedconstant.DATA_TYPE-10" class="anchor">§</a>

#### const <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedconstant.DATA_TYPE" class="constant">DATA_TYPE</a>: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native-10" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype">Native</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#impl-ArrowPrimitiveType-for-Float16Type" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait datafusion::common::arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Float16Type.html" class="struct" title="struct datafusion::common::arrow::datatypes::Float16Type">Float16Type</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedconstant.DATA_TYPE-11" class="anchor">§</a>

#### const <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedconstant.DATA_TYPE" class="constant">DATA_TYPE</a>: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a> = DataType::Float16

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native-11" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype">Native</a> = <a href="https://docs.rs/half/2.6.0/x86_64-unknown-linux-gnu/half/binary16/struct.f16.html" class="struct" title="struct half::binary16::f16">f16</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#impl-ArrowPrimitiveType-for-Float32Type" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait datafusion::common::arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Float32Type.html" class="struct" title="struct datafusion::common::arrow::datatypes::Float32Type">Float32Type</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedconstant.DATA_TYPE-12" class="anchor">§</a>

#### const <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedconstant.DATA_TYPE" class="constant">DATA_TYPE</a>: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a> = DataType::Float32

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native-12" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype">Native</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#impl-ArrowPrimitiveType-for-Float64Type" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait datafusion::common::arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Float64Type.html" class="struct" title="struct datafusion::common::arrow::datatypes::Float64Type">Float64Type</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedconstant.DATA_TYPE-13" class="anchor">§</a>

#### const <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedconstant.DATA_TYPE" class="constant">DATA_TYPE</a>: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a> = DataType::Float64

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native-13" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype">Native</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#impl-ArrowPrimitiveType-for-Int8Type" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait datafusion::common::arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Int8Type.html" class="struct" title="struct datafusion::common::arrow::datatypes::Int8Type">Int8Type</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedconstant.DATA_TYPE-14" class="anchor">§</a>

#### const <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedconstant.DATA_TYPE" class="constant">DATA_TYPE</a>: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a> = DataType::Int8

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native-14" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype">Native</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#impl-ArrowPrimitiveType-for-Int16Type" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait datafusion::common::arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Int16Type.html" class="struct" title="struct datafusion::common::arrow::datatypes::Int16Type">Int16Type</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedconstant.DATA_TYPE-15" class="anchor">§</a>

#### const <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedconstant.DATA_TYPE" class="constant">DATA_TYPE</a>: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a> = DataType::Int16

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native-15" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype">Native</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#impl-ArrowPrimitiveType-for-Int32Type" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait datafusion::common::arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Int32Type.html" class="struct" title="struct datafusion::common::arrow::datatypes::Int32Type">Int32Type</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedconstant.DATA_TYPE-16" class="anchor">§</a>

#### const <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedconstant.DATA_TYPE" class="constant">DATA_TYPE</a>: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a> = DataType::Int32

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native-16" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype">Native</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#impl-ArrowPrimitiveType-for-Int64Type" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait datafusion::common::arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Int64Type.html" class="struct" title="struct datafusion::common::arrow::datatypes::Int64Type">Int64Type</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedconstant.DATA_TYPE-17" class="anchor">§</a>

#### const <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedconstant.DATA_TYPE" class="constant">DATA_TYPE</a>: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a> = DataType::Int64

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native-17" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype">Native</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#impl-ArrowPrimitiveType-for-IntervalDayTimeType" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait datafusion::common::arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalDayTimeType.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalDayTimeType">IntervalDayTimeType</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedconstant.DATA_TYPE-18" class="anchor">§</a>

#### const <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedconstant.DATA_TYPE" class="constant">DATA_TYPE</a>: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native-18" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype">Native</a> = <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalDayTime.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalDayTime">IntervalDayTime</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#impl-ArrowPrimitiveType-for-IntervalMonthDayNanoType" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait datafusion::common::arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNanoType.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNanoType">IntervalMonthDayNanoType</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedconstant.DATA_TYPE-19" class="anchor">§</a>

#### const <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedconstant.DATA_TYPE" class="constant">DATA_TYPE</a>: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native-19" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype">Native</a> = <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#impl-ArrowPrimitiveType-for-IntervalYearMonthType" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait datafusion::common::arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalYearMonthType.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalYearMonthType">IntervalYearMonthType</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedconstant.DATA_TYPE-20" class="anchor">§</a>

#### const <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedconstant.DATA_TYPE" class="constant">DATA_TYPE</a>: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native-20" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype">Native</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#impl-ArrowPrimitiveType-for-Time32MillisecondType" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait datafusion::common::arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Time32MillisecondType.html" class="struct" title="struct datafusion::common::arrow::datatypes::Time32MillisecondType">Time32MillisecondType</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedconstant.DATA_TYPE-21" class="anchor">§</a>

#### const <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedconstant.DATA_TYPE" class="constant">DATA_TYPE</a>: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native-21" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype">Native</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#impl-ArrowPrimitiveType-for-Time32SecondType" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait datafusion::common::arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Time32SecondType.html" class="struct" title="struct datafusion::common::arrow::datatypes::Time32SecondType">Time32SecondType</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedconstant.DATA_TYPE-22" class="anchor">§</a>

#### const <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedconstant.DATA_TYPE" class="constant">DATA_TYPE</a>: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native-22" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype">Native</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#impl-ArrowPrimitiveType-for-Time64MicrosecondType" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait datafusion::common::arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Time64MicrosecondType.html" class="struct" title="struct datafusion::common::arrow::datatypes::Time64MicrosecondType">Time64MicrosecondType</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedconstant.DATA_TYPE-23" class="anchor">§</a>

#### const <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedconstant.DATA_TYPE" class="constant">DATA_TYPE</a>: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native-23" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype">Native</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#impl-ArrowPrimitiveType-for-Time64NanosecondType" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait datafusion::common::arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Time64NanosecondType.html" class="struct" title="struct datafusion::common::arrow::datatypes::Time64NanosecondType">Time64NanosecondType</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedconstant.DATA_TYPE-24" class="anchor">§</a>

#### const <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedconstant.DATA_TYPE" class="constant">DATA_TYPE</a>: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native-24" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype">Native</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#impl-ArrowPrimitiveType-for-TimestampMicrosecondType" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait datafusion::common::arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.TimestampMicrosecondType.html" class="struct" title="struct datafusion::common::arrow::datatypes::TimestampMicrosecondType">TimestampMicrosecondType</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedconstant.DATA_TYPE-25" class="anchor">§</a>

#### const <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedconstant.DATA_TYPE" class="constant">DATA_TYPE</a>: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native-25" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype">Native</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#impl-ArrowPrimitiveType-for-TimestampMillisecondType" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait datafusion::common::arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.TimestampMillisecondType.html" class="struct" title="struct datafusion::common::arrow::datatypes::TimestampMillisecondType">TimestampMillisecondType</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedconstant.DATA_TYPE-26" class="anchor">§</a>

#### const <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedconstant.DATA_TYPE" class="constant">DATA_TYPE</a>: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native-26" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype">Native</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#impl-ArrowPrimitiveType-for-TimestampNanosecondType" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait datafusion::common::arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.TimestampNanosecondType.html" class="struct" title="struct datafusion::common::arrow::datatypes::TimestampNanosecondType">TimestampNanosecondType</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedconstant.DATA_TYPE-27" class="anchor">§</a>

#### const <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedconstant.DATA_TYPE" class="constant">DATA_TYPE</a>: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native-27" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype">Native</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#impl-ArrowPrimitiveType-for-TimestampSecondType" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait datafusion::common::arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.TimestampSecondType.html" class="struct" title="struct datafusion::common::arrow::datatypes::TimestampSecondType">TimestampSecondType</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedconstant.DATA_TYPE-28" class="anchor">§</a>

#### const <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedconstant.DATA_TYPE" class="constant">DATA_TYPE</a>: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native-28" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype">Native</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#impl-ArrowPrimitiveType-for-UInt8Type" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait datafusion::common::arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.UInt8Type.html" class="struct" title="struct datafusion::common::arrow::datatypes::UInt8Type">UInt8Type</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedconstant.DATA_TYPE-29" class="anchor">§</a>

#### const <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedconstant.DATA_TYPE" class="constant">DATA_TYPE</a>: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a> = DataType::UInt8

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native-29" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype">Native</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#impl-ArrowPrimitiveType-for-UInt16Type" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait datafusion::common::arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.UInt16Type.html" class="struct" title="struct datafusion::common::arrow::datatypes::UInt16Type">UInt16Type</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedconstant.DATA_TYPE-30" class="anchor">§</a>

#### const <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedconstant.DATA_TYPE" class="constant">DATA_TYPE</a>: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a> = DataType::UInt16

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native-30" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype">Native</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#impl-ArrowPrimitiveType-for-UInt32Type" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait datafusion::common::arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.UInt32Type.html" class="struct" title="struct datafusion::common::arrow::datatypes::UInt32Type">UInt32Type</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedconstant.DATA_TYPE-31" class="anchor">§</a>

#### const <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedconstant.DATA_TYPE" class="constant">DATA_TYPE</a>: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a> = DataType::UInt32

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native-31" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype">Native</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#impl-ArrowPrimitiveType-for-UInt64Type" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait datafusion::common::arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.UInt64Type.html" class="struct" title="struct datafusion::common::arrow::datatypes::UInt64Type">UInt64Type</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedconstant.DATA_TYPE-32" class="anchor">§</a>

#### const <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedconstant.DATA_TYPE" class="constant">DATA_TYPE</a>: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a> = DataType::UInt64

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native-32" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype">Native</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>
