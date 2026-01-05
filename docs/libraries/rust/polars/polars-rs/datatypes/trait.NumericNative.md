# Trait NumericNative Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/datatypes/mod.rs.html#410-437" class="src">Source</a>

``` rust
pub trait NumericNative:
    TotalOrd
    + PartialOrd
    + TotalHash
    + NativeType
    + Num<Output = Self, Output = Self, Output = Self, Output = Self, Output = Self>
    + NumCast
    + Zero
    + One
    + Sum
    + Add
    + Sub
    + Mul
    + Div
    + Rem
    + AddAssign
    + SubAssign
    + AbsDiff
    + Bounded
    + FromPrimitive
    + IsFloat
    + HasPrimitiveArithmeticKernel<TrueDivT = <Self::TrueDivPolarsType as PolarsNumericType>::Native>
    + FloatSum<f64>
    + AsPrimitive<f64>
    + MinMax
    + IsNull {
    type PolarsType: PolarsNumericType;
    type TrueDivPolarsType: PolarsNumericType;
}
```

## Required Associated Types<a href="https://docs.rs/polars/latest/polars/datatypes/trait.NumericNative.html#required-associated-types" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/datatypes/trait.NumericNative.html#associatedtype.PolarsType" class="associatedtype">PolarsType</a>: <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html" class="trait" title="trait polars::prelude::PolarsNumericType">PolarsNumericType</a>

#### type <a href="https://docs.rs/polars/latest/polars/datatypes/trait.NumericNative.html#associatedtype.TrueDivPolarsType" class="associatedtype">TrueDivPolarsType</a>: <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html" class="trait" title="trait polars::prelude::PolarsNumericType">PolarsNumericType</a>

## Dyn Compatibility<a href="https://docs.rs/polars/latest/polars/datatypes/trait.NumericNative.html#dyn-compatibility" class="anchor">§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementations on Foreign Types<a href="https://docs.rs/polars/latest/polars/datatypes/trait.NumericNative.html#foreign-impls" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.NumericNative.html#impl-NumericNative-for-f32" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.NumericNative.html" class="trait" title="trait polars::prelude::NumericNative">NumericNative</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.NumericNative.html#associatedtype.PolarsType-1" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/datatypes/trait.NumericNative.html#associatedtype.PolarsType" class="associatedtype">PolarsType</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.Float32Type.html" class="struct" title="struct polars::prelude::Float32Type">Float32Type</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.NumericNative.html#associatedtype.TrueDivPolarsType-1" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/datatypes/trait.NumericNative.html#associatedtype.TrueDivPolarsType" class="associatedtype">TrueDivPolarsType</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.Float32Type.html" class="struct" title="struct polars::prelude::Float32Type">Float32Type</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.NumericNative.html#impl-NumericNative-for-f64" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.NumericNative.html" class="trait" title="trait polars::prelude::NumericNative">NumericNative</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.NumericNative.html#associatedtype.PolarsType-2" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/datatypes/trait.NumericNative.html#associatedtype.PolarsType" class="associatedtype">PolarsType</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.Float64Type.html" class="struct" title="struct polars::prelude::Float64Type">Float64Type</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.NumericNative.html#associatedtype.TrueDivPolarsType-2" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/datatypes/trait.NumericNative.html#associatedtype.TrueDivPolarsType" class="associatedtype">TrueDivPolarsType</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.Float64Type.html" class="struct" title="struct polars::prelude::Float64Type">Float64Type</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.NumericNative.html#impl-NumericNative-for-i8" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.NumericNative.html" class="trait" title="trait polars::prelude::NumericNative">NumericNative</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.NumericNative.html#associatedtype.PolarsType-3" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/datatypes/trait.NumericNative.html#associatedtype.PolarsType" class="associatedtype">PolarsType</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int8Type.html" class="struct" title="struct polars::prelude::Int8Type">Int8Type</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.NumericNative.html#associatedtype.TrueDivPolarsType-3" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/datatypes/trait.NumericNative.html#associatedtype.TrueDivPolarsType" class="associatedtype">TrueDivPolarsType</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.Float64Type.html" class="struct" title="struct polars::prelude::Float64Type">Float64Type</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.NumericNative.html#impl-NumericNative-for-i16" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.NumericNative.html" class="trait" title="trait polars::prelude::NumericNative">NumericNative</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.NumericNative.html#associatedtype.PolarsType-4" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/datatypes/trait.NumericNative.html#associatedtype.PolarsType" class="associatedtype">PolarsType</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int16Type.html" class="struct" title="struct polars::prelude::Int16Type">Int16Type</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.NumericNative.html#associatedtype.TrueDivPolarsType-4" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/datatypes/trait.NumericNative.html#associatedtype.TrueDivPolarsType" class="associatedtype">TrueDivPolarsType</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.Float64Type.html" class="struct" title="struct polars::prelude::Float64Type">Float64Type</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.NumericNative.html#impl-NumericNative-for-i32" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.NumericNative.html" class="trait" title="trait polars::prelude::NumericNative">NumericNative</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.NumericNative.html#associatedtype.PolarsType-5" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/datatypes/trait.NumericNative.html#associatedtype.PolarsType" class="associatedtype">PolarsType</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int32Type.html" class="struct" title="struct polars::prelude::Int32Type">Int32Type</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.NumericNative.html#associatedtype.TrueDivPolarsType-5" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/datatypes/trait.NumericNative.html#associatedtype.TrueDivPolarsType" class="associatedtype">TrueDivPolarsType</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.Float64Type.html" class="struct" title="struct polars::prelude::Float64Type">Float64Type</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.NumericNative.html#impl-NumericNative-for-i64" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.NumericNative.html" class="trait" title="trait polars::prelude::NumericNative">NumericNative</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.NumericNative.html#associatedtype.PolarsType-6" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/datatypes/trait.NumericNative.html#associatedtype.PolarsType" class="associatedtype">PolarsType</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type">Int64Type</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.NumericNative.html#associatedtype.TrueDivPolarsType-6" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/datatypes/trait.NumericNative.html#associatedtype.TrueDivPolarsType" class="associatedtype">TrueDivPolarsType</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.Float64Type.html" class="struct" title="struct polars::prelude::Float64Type">Float64Type</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.NumericNative.html#impl-NumericNative-for-i128" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.NumericNative.html" class="trait" title="trait polars::prelude::NumericNative">NumericNative</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.NumericNative.html#associatedtype.PolarsType-7" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/datatypes/trait.NumericNative.html#associatedtype.PolarsType" class="associatedtype">PolarsType</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int128Type.html" class="struct" title="struct polars::prelude::Int128Type">Int128Type</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.NumericNative.html#associatedtype.TrueDivPolarsType-7" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/datatypes/trait.NumericNative.html#associatedtype.TrueDivPolarsType" class="associatedtype">TrueDivPolarsType</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.Float64Type.html" class="struct" title="struct polars::prelude::Float64Type">Float64Type</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.NumericNative.html#impl-NumericNative-for-u8" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.NumericNative.html" class="trait" title="trait polars::prelude::NumericNative">NumericNative</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.NumericNative.html#associatedtype.PolarsType-8" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/datatypes/trait.NumericNative.html#associatedtype.PolarsType" class="associatedtype">PolarsType</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt8Type.html" class="struct" title="struct polars::prelude::UInt8Type">UInt8Type</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.NumericNative.html#associatedtype.TrueDivPolarsType-8" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/datatypes/trait.NumericNative.html#associatedtype.TrueDivPolarsType" class="associatedtype">TrueDivPolarsType</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.Float64Type.html" class="struct" title="struct polars::prelude::Float64Type">Float64Type</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.NumericNative.html#impl-NumericNative-for-u16" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.NumericNative.html" class="trait" title="trait polars::prelude::NumericNative">NumericNative</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.NumericNative.html#associatedtype.PolarsType-9" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/datatypes/trait.NumericNative.html#associatedtype.PolarsType" class="associatedtype">PolarsType</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt16Type.html" class="struct" title="struct polars::prelude::UInt16Type">UInt16Type</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.NumericNative.html#associatedtype.TrueDivPolarsType-9" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/datatypes/trait.NumericNative.html#associatedtype.TrueDivPolarsType" class="associatedtype">TrueDivPolarsType</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.Float64Type.html" class="struct" title="struct polars::prelude::Float64Type">Float64Type</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.NumericNative.html#impl-NumericNative-for-u32" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.NumericNative.html" class="trait" title="trait polars::prelude::NumericNative">NumericNative</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.NumericNative.html#associatedtype.PolarsType-10" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/datatypes/trait.NumericNative.html#associatedtype.PolarsType" class="associatedtype">PolarsType</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt32Type.html" class="struct" title="struct polars::prelude::UInt32Type">UInt32Type</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.NumericNative.html#associatedtype.TrueDivPolarsType-10" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/datatypes/trait.NumericNative.html#associatedtype.TrueDivPolarsType" class="associatedtype">TrueDivPolarsType</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.Float64Type.html" class="struct" title="struct polars::prelude::Float64Type">Float64Type</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.NumericNative.html#impl-NumericNative-for-u64" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.NumericNative.html" class="trait" title="trait polars::prelude::NumericNative">NumericNative</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.NumericNative.html#associatedtype.PolarsType-11" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/datatypes/trait.NumericNative.html#associatedtype.PolarsType" class="associatedtype">PolarsType</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt64Type.html" class="struct" title="struct polars::prelude::UInt64Type">UInt64Type</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.NumericNative.html#associatedtype.TrueDivPolarsType-11" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/datatypes/trait.NumericNative.html#associatedtype.TrueDivPolarsType" class="associatedtype">TrueDivPolarsType</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.Float64Type.html" class="struct" title="struct polars::prelude::Float64Type">Float64Type</a>

## Implementors<a href="https://docs.rs/polars/latest/polars/datatypes/trait.NumericNative.html#implementors" class="anchor">§</a>
