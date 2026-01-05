# Trait PolarsNumericType Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/datatypes/mod.rs.html#103" class="src">Source</a>

``` rust
pub trait PolarsNumericType:
    PolarsPhysicalType<OwnedPhysical = Self::Native, Physical<'a> = Self::Native, ZeroablePhysical<'a> = Self::Native, Array = PrimitiveArray<Self::Native>, IsNested = FalseT, HasViews = FalseT, IsStruct = FalseT, IsObject = FalseT>
    + 'static
    + for<'a> PolarsDataType {
    type Native: NumericNative;
}
```

## Required Associated Types<a href="https://docs.rs/polars/latest/polars/datatypes/trait.PolarsNumericType.html#required-associated-types" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/datatypes/trait.PolarsNumericType.html#associatedtype.Native" class="associatedtype">Native</a>: <a href="https://docs.rs/polars/latest/polars/prelude/trait.NumericNative.html" class="trait" title="trait polars::prelude::NumericNative">NumericNative</a>

## Dyn Compatibility<a href="https://docs.rs/polars/latest/polars/datatypes/trait.PolarsNumericType.html#dyn-compatibility" class="anchor">§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementors<a href="https://docs.rs/polars/latest/polars/datatypes/trait.PolarsNumericType.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.PolarsNumericType.html#impl-PolarsNumericType-for-Float32Type" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html" class="trait" title="trait polars::prelude::PolarsNumericType">PolarsNumericType</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Float32Type.html" class="struct" title="struct polars::prelude::Float32Type">Float32Type</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.PolarsNumericType.html#associatedtype.Native-1" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/datatypes/trait.PolarsNumericType.html#associatedtype.Native" class="associatedtype">Native</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.PolarsNumericType.html#impl-PolarsNumericType-for-Float64Type" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html" class="trait" title="trait polars::prelude::PolarsNumericType">PolarsNumericType</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Float64Type.html" class="struct" title="struct polars::prelude::Float64Type">Float64Type</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.PolarsNumericType.html#associatedtype.Native-2" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/datatypes/trait.PolarsNumericType.html#associatedtype.Native" class="associatedtype">Native</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.PolarsNumericType.html#impl-PolarsNumericType-for-Int8Type" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html" class="trait" title="trait polars::prelude::PolarsNumericType">PolarsNumericType</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int8Type.html" class="struct" title="struct polars::prelude::Int8Type">Int8Type</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.PolarsNumericType.html#associatedtype.Native-3" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/datatypes/trait.PolarsNumericType.html#associatedtype.Native" class="associatedtype">Native</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.PolarsNumericType.html#impl-PolarsNumericType-for-Int16Type" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html" class="trait" title="trait polars::prelude::PolarsNumericType">PolarsNumericType</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int16Type.html" class="struct" title="struct polars::prelude::Int16Type">Int16Type</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.PolarsNumericType.html#associatedtype.Native-4" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/datatypes/trait.PolarsNumericType.html#associatedtype.Native" class="associatedtype">Native</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.PolarsNumericType.html#impl-PolarsNumericType-for-Int32Type" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html" class="trait" title="trait polars::prelude::PolarsNumericType">PolarsNumericType</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int32Type.html" class="struct" title="struct polars::prelude::Int32Type">Int32Type</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.PolarsNumericType.html#associatedtype.Native-5" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/datatypes/trait.PolarsNumericType.html#associatedtype.Native" class="associatedtype">Native</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.PolarsNumericType.html#impl-PolarsNumericType-for-Int64Type" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html" class="trait" title="trait polars::prelude::PolarsNumericType">PolarsNumericType</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type">Int64Type</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.PolarsNumericType.html#associatedtype.Native-6" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/datatypes/trait.PolarsNumericType.html#associatedtype.Native" class="associatedtype">Native</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.PolarsNumericType.html#impl-PolarsNumericType-for-Int128Type" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html" class="trait" title="trait polars::prelude::PolarsNumericType">PolarsNumericType</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int128Type.html" class="struct" title="struct polars::prelude::Int128Type">Int128Type</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.PolarsNumericType.html#associatedtype.Native-7" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/datatypes/trait.PolarsNumericType.html#associatedtype.Native" class="associatedtype">Native</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.PolarsNumericType.html#impl-PolarsNumericType-for-UInt8Type" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html" class="trait" title="trait polars::prelude::PolarsNumericType">PolarsNumericType</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt8Type.html" class="struct" title="struct polars::prelude::UInt8Type">UInt8Type</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.PolarsNumericType.html#associatedtype.Native-8" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/datatypes/trait.PolarsNumericType.html#associatedtype.Native" class="associatedtype">Native</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.PolarsNumericType.html#impl-PolarsNumericType-for-UInt16Type" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html" class="trait" title="trait polars::prelude::PolarsNumericType">PolarsNumericType</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt16Type.html" class="struct" title="struct polars::prelude::UInt16Type">UInt16Type</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.PolarsNumericType.html#associatedtype.Native-9" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/datatypes/trait.PolarsNumericType.html#associatedtype.Native" class="associatedtype">Native</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.PolarsNumericType.html#impl-PolarsNumericType-for-UInt32Type" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html" class="trait" title="trait polars::prelude::PolarsNumericType">PolarsNumericType</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt32Type.html" class="struct" title="struct polars::prelude::UInt32Type">UInt32Type</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.PolarsNumericType.html#associatedtype.Native-10" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/datatypes/trait.PolarsNumericType.html#associatedtype.Native" class="associatedtype">Native</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.PolarsNumericType.html#impl-PolarsNumericType-for-UInt64Type" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html" class="trait" title="trait polars::prelude::PolarsNumericType">PolarsNumericType</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt64Type.html" class="struct" title="struct polars::prelude::UInt64Type">UInt64Type</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.PolarsNumericType.html#associatedtype.Native-11" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/datatypes/trait.PolarsNumericType.html#associatedtype.Native" class="associatedtype">Native</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>
