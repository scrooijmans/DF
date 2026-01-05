# Struct NativeAdapter Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/array/primitive_array.rs.html#1375" class="src">Source</a>

``` rust
pub struct NativeAdapter<T>where
    T: ArrowPrimitiveType,{
    pub native: Option<<T as ArrowPrimitiveType>::Native>,
}
```

Expand description

An optional primitive value

This struct is used as an adapter when creating `PrimitiveArray` from an iterator. `FromIterator` for `PrimitiveArray` takes an iterator where the elements can be `into` this struct. So once implementing `From` or `Into` trait for a type, an iterator of the type can be collected to `PrimitiveArray`.

## Fields<a href="https://docs.rs/arrow/latest/arrow/array/struct.NativeAdapter.html#fields" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/struct.NativeAdapter.html#structfield.native" class="anchor field">§</a>`native: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<<T as `<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType"><code>ArrowPrimitiveType</code></a>`>::`<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native"><code>Native</code></a>`>`

Corresponding Rust native type if available

## Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/array/struct.NativeAdapter.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/struct.NativeAdapter.html#impl-Debug-for-NativeAdapter%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.NativeAdapter.html" class="struct" title="struct arrow::array::NativeAdapter">NativeAdapter</a>\<T\>

where T: <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> + <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>, \<T as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>: <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/struct.NativeAdapter.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/arrow/latest/arrow/array/struct.NativeAdapter.html#impl-From%3C%26Option%3C%3CT+as+ArrowPrimitiveType%3E::Native%3E%3E-for-NativeAdapter%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<&<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<\<T as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.NativeAdapter.html" class="struct" title="struct arrow::array::NativeAdapter">NativeAdapter</a>\<T\>

where T: <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/struct.NativeAdapter.html#method.from-16" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: &<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<\<T as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.NativeAdapter.html" class="struct" title="struct arrow::array::NativeAdapter">NativeAdapter</a>\<T\>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/struct.NativeAdapter.html#impl-From%3COption%3C%3CT+as+ArrowPrimitiveType%3E::Native%3E%3E-for-NativeAdapter%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<\<T as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.NativeAdapter.html" class="struct" title="struct arrow::array::NativeAdapter">NativeAdapter</a>\<T\>

where T: <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/struct.NativeAdapter.html#method.from-15" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<\<T as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.NativeAdapter.html" class="struct" title="struct arrow::array::NativeAdapter">NativeAdapter</a>\<T\>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/struct.NativeAdapter.html#impl-From%3Cf16%3E-for-NativeAdapter%3CFloat16Type%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/half/2.6.0/x86_64-unknown-linux-gnu/half/binary16/struct.f16.html" class="struct" title="struct half::binary16::f16">f16</a>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.NativeAdapter.html" class="struct" title="struct arrow::array::NativeAdapter">NativeAdapter</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Float16Type.html" class="struct" title="struct arrow::datatypes::Float16Type">Float16Type</a>\>

<a href="https://docs.rs/arrow/latest/arrow/array/struct.NativeAdapter.html#method.from-8" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://docs.rs/half/2.6.0/x86_64-unknown-linux-gnu/half/binary16/struct.f16.html" class="struct" title="struct half::binary16::f16">f16</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.NativeAdapter.html" class="struct" title="struct arrow::array::NativeAdapter">NativeAdapter</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Float16Type.html" class="struct" title="struct arrow::datatypes::Float16Type">Float16Type</a>\>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/struct.NativeAdapter.html#impl-From%3Cf32%3E-for-NativeAdapter%3CFloat32Type%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.NativeAdapter.html" class="struct" title="struct arrow::array::NativeAdapter">NativeAdapter</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Float32Type.html" class="struct" title="struct arrow::datatypes::Float32Type">Float32Type</a>\>

<a href="https://docs.rs/arrow/latest/arrow/array/struct.NativeAdapter.html#method.from-9" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.NativeAdapter.html" class="struct" title="struct arrow::array::NativeAdapter">NativeAdapter</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Float32Type.html" class="struct" title="struct arrow::datatypes::Float32Type">Float32Type</a>\>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/struct.NativeAdapter.html#impl-From%3Cf64%3E-for-NativeAdapter%3CFloat64Type%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.NativeAdapter.html" class="struct" title="struct arrow::array::NativeAdapter">NativeAdapter</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Float64Type.html" class="struct" title="struct arrow::datatypes::Float64Type">Float64Type</a>\>

<a href="https://docs.rs/arrow/latest/arrow/array/struct.NativeAdapter.html#method.from-10" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.NativeAdapter.html" class="struct" title="struct arrow::array::NativeAdapter">NativeAdapter</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Float64Type.html" class="struct" title="struct arrow::datatypes::Float64Type">Float64Type</a>\>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/struct.NativeAdapter.html#impl-From%3Ci128%3E-for-NativeAdapter%3CDecimal128Type%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.NativeAdapter.html" class="struct" title="struct arrow::array::NativeAdapter">NativeAdapter</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Decimal128Type.html" class="struct" title="struct arrow::datatypes::Decimal128Type">Decimal128Type</a>\>

<a href="https://docs.rs/arrow/latest/arrow/array/struct.NativeAdapter.html#method.from-13" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.NativeAdapter.html" class="struct" title="struct arrow::array::NativeAdapter">NativeAdapter</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Decimal128Type.html" class="struct" title="struct arrow::datatypes::Decimal128Type">Decimal128Type</a>\>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/struct.NativeAdapter.html#impl-From%3Ci16%3E-for-NativeAdapter%3CInt16Type%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.NativeAdapter.html" class="struct" title="struct arrow::array::NativeAdapter">NativeAdapter</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Int16Type.html" class="struct" title="struct arrow::datatypes::Int16Type">Int16Type</a>\>

<a href="https://docs.rs/arrow/latest/arrow/array/struct.NativeAdapter.html#method.from-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.NativeAdapter.html" class="struct" title="struct arrow::array::NativeAdapter">NativeAdapter</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Int16Type.html" class="struct" title="struct arrow::datatypes::Int16Type">Int16Type</a>\>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/struct.NativeAdapter.html#impl-From%3Ci256%3E-for-NativeAdapter%3CDecimal256Type%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.NativeAdapter.html" class="struct" title="struct arrow::array::NativeAdapter">NativeAdapter</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Decimal256Type.html" class="struct" title="struct arrow::datatypes::Decimal256Type">Decimal256Type</a>\>

<a href="https://docs.rs/arrow/latest/arrow/array/struct.NativeAdapter.html#method.from-14" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.NativeAdapter.html" class="struct" title="struct arrow::array::NativeAdapter">NativeAdapter</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Decimal256Type.html" class="struct" title="struct arrow::datatypes::Decimal256Type">Decimal256Type</a>\>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/struct.NativeAdapter.html#impl-From%3Ci32%3E-for-NativeAdapter%3CDecimal32Type%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.NativeAdapter.html" class="struct" title="struct arrow::array::NativeAdapter">NativeAdapter</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Decimal32Type.html" class="struct" title="struct arrow::datatypes::Decimal32Type">Decimal32Type</a>\>

<a href="https://docs.rs/arrow/latest/arrow/array/struct.NativeAdapter.html#method.from-11" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.NativeAdapter.html" class="struct" title="struct arrow::array::NativeAdapter">NativeAdapter</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Decimal32Type.html" class="struct" title="struct arrow::datatypes::Decimal32Type">Decimal32Type</a>\>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/struct.NativeAdapter.html#impl-From%3Ci32%3E-for-NativeAdapter%3CInt32Type%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.NativeAdapter.html" class="struct" title="struct arrow::array::NativeAdapter">NativeAdapter</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Int32Type.html" class="struct" title="struct arrow::datatypes::Int32Type">Int32Type</a>\>

<a href="https://docs.rs/arrow/latest/arrow/array/struct.NativeAdapter.html#method.from-2" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.NativeAdapter.html" class="struct" title="struct arrow::array::NativeAdapter">NativeAdapter</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Int32Type.html" class="struct" title="struct arrow::datatypes::Int32Type">Int32Type</a>\>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/struct.NativeAdapter.html#impl-From%3Ci64%3E-for-NativeAdapter%3CDecimal64Type%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.NativeAdapter.html" class="struct" title="struct arrow::array::NativeAdapter">NativeAdapter</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Decimal64Type.html" class="struct" title="struct arrow::datatypes::Decimal64Type">Decimal64Type</a>\>

<a href="https://docs.rs/arrow/latest/arrow/array/struct.NativeAdapter.html#method.from-12" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.NativeAdapter.html" class="struct" title="struct arrow::array::NativeAdapter">NativeAdapter</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Decimal64Type.html" class="struct" title="struct arrow::datatypes::Decimal64Type">Decimal64Type</a>\>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/struct.NativeAdapter.html#impl-From%3Ci64%3E-for-NativeAdapter%3CInt64Type%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.NativeAdapter.html" class="struct" title="struct arrow::array::NativeAdapter">NativeAdapter</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Int64Type.html" class="struct" title="struct arrow::datatypes::Int64Type">Int64Type</a>\>

<a href="https://docs.rs/arrow/latest/arrow/array/struct.NativeAdapter.html#method.from-3" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.NativeAdapter.html" class="struct" title="struct arrow::array::NativeAdapter">NativeAdapter</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Int64Type.html" class="struct" title="struct arrow::datatypes::Int64Type">Int64Type</a>\>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/struct.NativeAdapter.html#impl-From%3Ci8%3E-for-NativeAdapter%3CInt8Type%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.NativeAdapter.html" class="struct" title="struct arrow::array::NativeAdapter">NativeAdapter</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Int8Type.html" class="struct" title="struct arrow::datatypes::Int8Type">Int8Type</a>\>

<a href="https://docs.rs/arrow/latest/arrow/array/struct.NativeAdapter.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.NativeAdapter.html" class="struct" title="struct arrow::array::NativeAdapter">NativeAdapter</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Int8Type.html" class="struct" title="struct arrow::datatypes::Int8Type">Int8Type</a>\>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/struct.NativeAdapter.html#impl-From%3Cu16%3E-for-NativeAdapter%3CUInt16Type%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.NativeAdapter.html" class="struct" title="struct arrow::array::NativeAdapter">NativeAdapter</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.UInt16Type.html" class="struct" title="struct arrow::datatypes::UInt16Type">UInt16Type</a>\>

<a href="https://docs.rs/arrow/latest/arrow/array/struct.NativeAdapter.html#method.from-5" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.NativeAdapter.html" class="struct" title="struct arrow::array::NativeAdapter">NativeAdapter</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.UInt16Type.html" class="struct" title="struct arrow::datatypes::UInt16Type">UInt16Type</a>\>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/struct.NativeAdapter.html#impl-From%3Cu32%3E-for-NativeAdapter%3CUInt32Type%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.NativeAdapter.html" class="struct" title="struct arrow::array::NativeAdapter">NativeAdapter</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.UInt32Type.html" class="struct" title="struct arrow::datatypes::UInt32Type">UInt32Type</a>\>

<a href="https://docs.rs/arrow/latest/arrow/array/struct.NativeAdapter.html#method.from-6" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.NativeAdapter.html" class="struct" title="struct arrow::array::NativeAdapter">NativeAdapter</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.UInt32Type.html" class="struct" title="struct arrow::datatypes::UInt32Type">UInt32Type</a>\>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/struct.NativeAdapter.html#impl-From%3Cu64%3E-for-NativeAdapter%3CUInt64Type%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.NativeAdapter.html" class="struct" title="struct arrow::array::NativeAdapter">NativeAdapter</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.UInt64Type.html" class="struct" title="struct arrow::datatypes::UInt64Type">UInt64Type</a>\>

<a href="https://docs.rs/arrow/latest/arrow/array/struct.NativeAdapter.html#method.from-7" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.NativeAdapter.html" class="struct" title="struct arrow::array::NativeAdapter">NativeAdapter</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.UInt64Type.html" class="struct" title="struct arrow::datatypes::UInt64Type">UInt64Type</a>\>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/struct.NativeAdapter.html#impl-From%3Cu8%3E-for-NativeAdapter%3CUInt8Type%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.NativeAdapter.html" class="struct" title="struct arrow::array::NativeAdapter">NativeAdapter</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.UInt8Type.html" class="struct" title="struct arrow::datatypes::UInt8Type">UInt8Type</a>\>

<a href="https://docs.rs/arrow/latest/arrow/array/struct.NativeAdapter.html#method.from-4" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.NativeAdapter.html" class="struct" title="struct arrow::array::NativeAdapter">NativeAdapter</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.UInt8Type.html" class="struct" title="struct arrow::datatypes::UInt8Type">UInt8Type</a>\>

Converts to this type from the input type.

## Auto Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/array/struct.NativeAdapter.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/arrow/latest/arrow/array/struct.NativeAdapter.html#blanket-implementations" class="anchor">§</a>
