# Trait ArithmeticChunked Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/chunked_array/arithmetic/numeric.rs.html#54" class="src">Source</a>

``` rust
pub trait ArithmeticChunked {
    type Scalar;
    type Out;
    type TrueDivOut;

Show 25 methods    // Required methods
    fn wrapping_abs(self) -> Self::Out;
    fn wrapping_neg(self) -> Self::Out;
    fn wrapping_add(self, rhs: Self) -> Self::Out;
    fn wrapping_sub(self, rhs: Self) -> Self::Out;
    fn wrapping_mul(self, rhs: Self) -> Self::Out;
    fn wrapping_floor_div(self, rhs: Self) -> Self::Out;
    fn wrapping_trunc_div(self, rhs: Self) -> Self::Out;
    fn wrapping_mod(self, rhs: Self) -> Self::Out;
    fn wrapping_add_scalar(self, rhs: Self::Scalar) -> Self::Out;
    fn wrapping_sub_scalar(self, rhs: Self::Scalar) -> Self::Out;
    fn wrapping_sub_scalar_lhs(lhs: Self::Scalar, rhs: Self) -> Self::Out;
    fn wrapping_mul_scalar(self, rhs: Self::Scalar) -> Self::Out;
    fn wrapping_floor_div_scalar(self, rhs: Self::Scalar) -> Self::Out;
    fn wrapping_floor_div_scalar_lhs(lhs: Self::Scalar, rhs: Self) -> Self::Out;
    fn wrapping_trunc_div_scalar(self, rhs: Self::Scalar) -> Self::Out;
    fn wrapping_trunc_div_scalar_lhs(lhs: Self::Scalar, rhs: Self) -> Self::Out;
    fn wrapping_mod_scalar(self, rhs: Self::Scalar) -> Self::Out;
    fn wrapping_mod_scalar_lhs(lhs: Self::Scalar, rhs: Self) -> Self::Out;
    fn checked_mul_scalar(self, rhs: Self::Scalar) -> Self::Out;
    fn true_div(self, rhs: Self) -> Self::TrueDivOut;
    fn true_div_scalar(self, rhs: Self::Scalar) -> Self::TrueDivOut;
    fn true_div_scalar_lhs(lhs: Self::Scalar, rhs: Self) -> Self::TrueDivOut;
    fn legacy_div(self, rhs: Self) -> Self::Out;
    fn legacy_div_scalar(self, rhs: Self::Scalar) -> Self::Out;
    fn legacy_div_scalar_lhs(lhs: Self::Scalar, rhs: Self) -> Self::Out;
}
```

## Required Associated Types<a href="https://docs.rs/polars/latest/polars/chunked_array/arithmetic/trait.ArithmeticChunked.html#required-associated-types" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/chunked_array/arithmetic/trait.ArithmeticChunked.html#associatedtype.Scalar" class="associatedtype">Scalar</a>

#### type <a href="https://docs.rs/polars/latest/polars/chunked_array/arithmetic/trait.ArithmeticChunked.html#associatedtype.Out" class="associatedtype">Out</a>

#### type <a href="https://docs.rs/polars/latest/polars/chunked_array/arithmetic/trait.ArithmeticChunked.html#associatedtype.TrueDivOut" class="associatedtype">TrueDivOut</a>

## Required Methods<a href="https://docs.rs/polars/latest/polars/chunked_array/arithmetic/trait.ArithmeticChunked.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/chunked_array/arithmetic/trait.ArithmeticChunked.html#tymethod.wrapping_abs" class="fn">wrapping_abs</a>(self) -\> Self::<a href="https://docs.rs/polars/latest/polars/prelude/trait.ArithmeticChunked.html#associatedtype.Out" class="associatedtype" title="type polars::prelude::ArithmeticChunked::Out">Out</a>

#### fn <a href="https://docs.rs/polars/latest/polars/chunked_array/arithmetic/trait.ArithmeticChunked.html#tymethod.wrapping_neg" class="fn">wrapping_neg</a>(self) -\> Self::<a href="https://docs.rs/polars/latest/polars/prelude/trait.ArithmeticChunked.html#associatedtype.Out" class="associatedtype" title="type polars::prelude::ArithmeticChunked::Out">Out</a>

#### fn <a href="https://docs.rs/polars/latest/polars/chunked_array/arithmetic/trait.ArithmeticChunked.html#tymethod.wrapping_add" class="fn">wrapping_add</a>(self, rhs: Self) -\> Self::<a href="https://docs.rs/polars/latest/polars/prelude/trait.ArithmeticChunked.html#associatedtype.Out" class="associatedtype" title="type polars::prelude::ArithmeticChunked::Out">Out</a>

#### fn <a href="https://docs.rs/polars/latest/polars/chunked_array/arithmetic/trait.ArithmeticChunked.html#tymethod.wrapping_sub" class="fn">wrapping_sub</a>(self, rhs: Self) -\> Self::<a href="https://docs.rs/polars/latest/polars/prelude/trait.ArithmeticChunked.html#associatedtype.Out" class="associatedtype" title="type polars::prelude::ArithmeticChunked::Out">Out</a>

#### fn <a href="https://docs.rs/polars/latest/polars/chunked_array/arithmetic/trait.ArithmeticChunked.html#tymethod.wrapping_mul" class="fn">wrapping_mul</a>(self, rhs: Self) -\> Self::<a href="https://docs.rs/polars/latest/polars/prelude/trait.ArithmeticChunked.html#associatedtype.Out" class="associatedtype" title="type polars::prelude::ArithmeticChunked::Out">Out</a>

#### fn <a href="https://docs.rs/polars/latest/polars/chunked_array/arithmetic/trait.ArithmeticChunked.html#tymethod.wrapping_floor_div" class="fn">wrapping_floor_div</a>(self, rhs: Self) -\> Self::<a href="https://docs.rs/polars/latest/polars/prelude/trait.ArithmeticChunked.html#associatedtype.Out" class="associatedtype" title="type polars::prelude::ArithmeticChunked::Out">Out</a>

#### fn <a href="https://docs.rs/polars/latest/polars/chunked_array/arithmetic/trait.ArithmeticChunked.html#tymethod.wrapping_trunc_div" class="fn">wrapping_trunc_div</a>(self, rhs: Self) -\> Self::<a href="https://docs.rs/polars/latest/polars/prelude/trait.ArithmeticChunked.html#associatedtype.Out" class="associatedtype" title="type polars::prelude::ArithmeticChunked::Out">Out</a>

#### fn <a href="https://docs.rs/polars/latest/polars/chunked_array/arithmetic/trait.ArithmeticChunked.html#tymethod.wrapping_mod" class="fn">wrapping_mod</a>(self, rhs: Self) -\> Self::<a href="https://docs.rs/polars/latest/polars/prelude/trait.ArithmeticChunked.html#associatedtype.Out" class="associatedtype" title="type polars::prelude::ArithmeticChunked::Out">Out</a>

#### fn <a href="https://docs.rs/polars/latest/polars/chunked_array/arithmetic/trait.ArithmeticChunked.html#tymethod.wrapping_add_scalar" class="fn">wrapping_add_scalar</a>(self, rhs: Self::<a href="https://docs.rs/polars/latest/polars/prelude/trait.ArithmeticChunked.html#associatedtype.Scalar" class="associatedtype" title="type polars::prelude::ArithmeticChunked::Scalar">Scalar</a>) -\> Self::<a href="https://docs.rs/polars/latest/polars/prelude/trait.ArithmeticChunked.html#associatedtype.Out" class="associatedtype" title="type polars::prelude::ArithmeticChunked::Out">Out</a>

#### fn <a href="https://docs.rs/polars/latest/polars/chunked_array/arithmetic/trait.ArithmeticChunked.html#tymethod.wrapping_sub_scalar" class="fn">wrapping_sub_scalar</a>(self, rhs: Self::<a href="https://docs.rs/polars/latest/polars/prelude/trait.ArithmeticChunked.html#associatedtype.Scalar" class="associatedtype" title="type polars::prelude::ArithmeticChunked::Scalar">Scalar</a>) -\> Self::<a href="https://docs.rs/polars/latest/polars/prelude/trait.ArithmeticChunked.html#associatedtype.Out" class="associatedtype" title="type polars::prelude::ArithmeticChunked::Out">Out</a>

#### fn <a href="https://docs.rs/polars/latest/polars/chunked_array/arithmetic/trait.ArithmeticChunked.html#tymethod.wrapping_sub_scalar_lhs" class="fn">wrapping_sub_scalar_lhs</a>(lhs: Self::<a href="https://docs.rs/polars/latest/polars/prelude/trait.ArithmeticChunked.html#associatedtype.Scalar" class="associatedtype" title="type polars::prelude::ArithmeticChunked::Scalar">Scalar</a>, rhs: Self) -\> Self::<a href="https://docs.rs/polars/latest/polars/prelude/trait.ArithmeticChunked.html#associatedtype.Out" class="associatedtype" title="type polars::prelude::ArithmeticChunked::Out">Out</a>

#### fn <a href="https://docs.rs/polars/latest/polars/chunked_array/arithmetic/trait.ArithmeticChunked.html#tymethod.wrapping_mul_scalar" class="fn">wrapping_mul_scalar</a>(self, rhs: Self::<a href="https://docs.rs/polars/latest/polars/prelude/trait.ArithmeticChunked.html#associatedtype.Scalar" class="associatedtype" title="type polars::prelude::ArithmeticChunked::Scalar">Scalar</a>) -\> Self::<a href="https://docs.rs/polars/latest/polars/prelude/trait.ArithmeticChunked.html#associatedtype.Out" class="associatedtype" title="type polars::prelude::ArithmeticChunked::Out">Out</a>

#### fn <a href="https://docs.rs/polars/latest/polars/chunked_array/arithmetic/trait.ArithmeticChunked.html#tymethod.wrapping_floor_div_scalar" class="fn">wrapping_floor_div_scalar</a>(self, rhs: Self::<a href="https://docs.rs/polars/latest/polars/prelude/trait.ArithmeticChunked.html#associatedtype.Scalar" class="associatedtype" title="type polars::prelude::ArithmeticChunked::Scalar">Scalar</a>) -\> Self::<a href="https://docs.rs/polars/latest/polars/prelude/trait.ArithmeticChunked.html#associatedtype.Out" class="associatedtype" title="type polars::prelude::ArithmeticChunked::Out">Out</a>

#### fn <a href="https://docs.rs/polars/latest/polars/chunked_array/arithmetic/trait.ArithmeticChunked.html#tymethod.wrapping_floor_div_scalar_lhs" class="fn">wrapping_floor_div_scalar_lhs</a>(lhs: Self::<a href="https://docs.rs/polars/latest/polars/prelude/trait.ArithmeticChunked.html#associatedtype.Scalar" class="associatedtype" title="type polars::prelude::ArithmeticChunked::Scalar">Scalar</a>, rhs: Self) -\> Self::<a href="https://docs.rs/polars/latest/polars/prelude/trait.ArithmeticChunked.html#associatedtype.Out" class="associatedtype" title="type polars::prelude::ArithmeticChunked::Out">Out</a>

#### fn <a href="https://docs.rs/polars/latest/polars/chunked_array/arithmetic/trait.ArithmeticChunked.html#tymethod.wrapping_trunc_div_scalar" class="fn">wrapping_trunc_div_scalar</a>(self, rhs: Self::<a href="https://docs.rs/polars/latest/polars/prelude/trait.ArithmeticChunked.html#associatedtype.Scalar" class="associatedtype" title="type polars::prelude::ArithmeticChunked::Scalar">Scalar</a>) -\> Self::<a href="https://docs.rs/polars/latest/polars/prelude/trait.ArithmeticChunked.html#associatedtype.Out" class="associatedtype" title="type polars::prelude::ArithmeticChunked::Out">Out</a>

#### fn <a href="https://docs.rs/polars/latest/polars/chunked_array/arithmetic/trait.ArithmeticChunked.html#tymethod.wrapping_trunc_div_scalar_lhs" class="fn">wrapping_trunc_div_scalar_lhs</a>(lhs: Self::<a href="https://docs.rs/polars/latest/polars/prelude/trait.ArithmeticChunked.html#associatedtype.Scalar" class="associatedtype" title="type polars::prelude::ArithmeticChunked::Scalar">Scalar</a>, rhs: Self) -\> Self::<a href="https://docs.rs/polars/latest/polars/prelude/trait.ArithmeticChunked.html#associatedtype.Out" class="associatedtype" title="type polars::prelude::ArithmeticChunked::Out">Out</a>

#### fn <a href="https://docs.rs/polars/latest/polars/chunked_array/arithmetic/trait.ArithmeticChunked.html#tymethod.wrapping_mod_scalar" class="fn">wrapping_mod_scalar</a>(self, rhs: Self::<a href="https://docs.rs/polars/latest/polars/prelude/trait.ArithmeticChunked.html#associatedtype.Scalar" class="associatedtype" title="type polars::prelude::ArithmeticChunked::Scalar">Scalar</a>) -\> Self::<a href="https://docs.rs/polars/latest/polars/prelude/trait.ArithmeticChunked.html#associatedtype.Out" class="associatedtype" title="type polars::prelude::ArithmeticChunked::Out">Out</a>

#### fn <a href="https://docs.rs/polars/latest/polars/chunked_array/arithmetic/trait.ArithmeticChunked.html#tymethod.wrapping_mod_scalar_lhs" class="fn">wrapping_mod_scalar_lhs</a>(lhs: Self::<a href="https://docs.rs/polars/latest/polars/prelude/trait.ArithmeticChunked.html#associatedtype.Scalar" class="associatedtype" title="type polars::prelude::ArithmeticChunked::Scalar">Scalar</a>, rhs: Self) -\> Self::<a href="https://docs.rs/polars/latest/polars/prelude/trait.ArithmeticChunked.html#associatedtype.Out" class="associatedtype" title="type polars::prelude::ArithmeticChunked::Out">Out</a>

#### fn <a href="https://docs.rs/polars/latest/polars/chunked_array/arithmetic/trait.ArithmeticChunked.html#tymethod.checked_mul_scalar" class="fn">checked_mul_scalar</a>(self, rhs: Self::<a href="https://docs.rs/polars/latest/polars/prelude/trait.ArithmeticChunked.html#associatedtype.Scalar" class="associatedtype" title="type polars::prelude::ArithmeticChunked::Scalar">Scalar</a>) -\> Self::<a href="https://docs.rs/polars/latest/polars/prelude/trait.ArithmeticChunked.html#associatedtype.Out" class="associatedtype" title="type polars::prelude::ArithmeticChunked::Out">Out</a>

#### fn <a href="https://docs.rs/polars/latest/polars/chunked_array/arithmetic/trait.ArithmeticChunked.html#tymethod.true_div" class="fn">true_div</a>(self, rhs: Self) -\> Self::<a href="https://docs.rs/polars/latest/polars/prelude/trait.ArithmeticChunked.html#associatedtype.TrueDivOut" class="associatedtype" title="type polars::prelude::ArithmeticChunked::TrueDivOut">TrueDivOut</a>

#### fn <a href="https://docs.rs/polars/latest/polars/chunked_array/arithmetic/trait.ArithmeticChunked.html#tymethod.true_div_scalar" class="fn">true_div_scalar</a>(self, rhs: Self::<a href="https://docs.rs/polars/latest/polars/prelude/trait.ArithmeticChunked.html#associatedtype.Scalar" class="associatedtype" title="type polars::prelude::ArithmeticChunked::Scalar">Scalar</a>) -\> Self::<a href="https://docs.rs/polars/latest/polars/prelude/trait.ArithmeticChunked.html#associatedtype.TrueDivOut" class="associatedtype" title="type polars::prelude::ArithmeticChunked::TrueDivOut">TrueDivOut</a>

#### fn <a href="https://docs.rs/polars/latest/polars/chunked_array/arithmetic/trait.ArithmeticChunked.html#tymethod.true_div_scalar_lhs" class="fn">true_div_scalar_lhs</a>(lhs: Self::<a href="https://docs.rs/polars/latest/polars/prelude/trait.ArithmeticChunked.html#associatedtype.Scalar" class="associatedtype" title="type polars::prelude::ArithmeticChunked::Scalar">Scalar</a>, rhs: Self) -\> Self::<a href="https://docs.rs/polars/latest/polars/prelude/trait.ArithmeticChunked.html#associatedtype.TrueDivOut" class="associatedtype" title="type polars::prelude::ArithmeticChunked::TrueDivOut">TrueDivOut</a>

#### fn <a href="https://docs.rs/polars/latest/polars/chunked_array/arithmetic/trait.ArithmeticChunked.html#tymethod.legacy_div" class="fn">legacy_div</a>(self, rhs: Self) -\> Self::<a href="https://docs.rs/polars/latest/polars/prelude/trait.ArithmeticChunked.html#associatedtype.Out" class="associatedtype" title="type polars::prelude::ArithmeticChunked::Out">Out</a>

#### fn <a href="https://docs.rs/polars/latest/polars/chunked_array/arithmetic/trait.ArithmeticChunked.html#tymethod.legacy_div_scalar" class="fn">legacy_div_scalar</a>(self, rhs: Self::<a href="https://docs.rs/polars/latest/polars/prelude/trait.ArithmeticChunked.html#associatedtype.Scalar" class="associatedtype" title="type polars::prelude::ArithmeticChunked::Scalar">Scalar</a>) -\> Self::<a href="https://docs.rs/polars/latest/polars/prelude/trait.ArithmeticChunked.html#associatedtype.Out" class="associatedtype" title="type polars::prelude::ArithmeticChunked::Out">Out</a>

#### fn <a href="https://docs.rs/polars/latest/polars/chunked_array/arithmetic/trait.ArithmeticChunked.html#tymethod.legacy_div_scalar_lhs" class="fn">legacy_div_scalar_lhs</a>(lhs: Self::<a href="https://docs.rs/polars/latest/polars/prelude/trait.ArithmeticChunked.html#associatedtype.Scalar" class="associatedtype" title="type polars::prelude::ArithmeticChunked::Scalar">Scalar</a>, rhs: Self) -\> Self::<a href="https://docs.rs/polars/latest/polars/prelude/trait.ArithmeticChunked.html#associatedtype.Out" class="associatedtype" title="type polars::prelude::ArithmeticChunked::Out">Out</a>

## Dyn Compatibility<a href="https://docs.rs/polars/latest/polars/chunked_array/arithmetic/trait.ArithmeticChunked.html#dyn-compatibility" class="anchor">§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementors<a href="https://docs.rs/polars/latest/polars/chunked_array/arithmetic/trait.ArithmeticChunked.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/chunked_array/arithmetic/trait.ArithmeticChunked.html#impl-ArithmeticChunked-for-%26ChunkedArray%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.ArithmeticChunked.html" class="trait" title="trait polars::prelude::ArithmeticChunked">ArithmeticChunked</a> for &<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<T\>

where T: <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html" class="trait" title="trait polars::prelude::PolarsNumericType">PolarsNumericType</a>,

<a href="https://docs.rs/polars/latest/polars/chunked_array/arithmetic/trait.ArithmeticChunked.html#associatedtype.Scalar-1" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/chunked_array/arithmetic/trait.ArithmeticChunked.html#associatedtype.Scalar" class="associatedtype">Scalar</a> = \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html" class="trait" title="trait polars::prelude::PolarsNumericType">PolarsNumericType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html#associatedtype.Native" class="associatedtype" title="type polars::prelude::PolarsNumericType::Native">Native</a>

<a href="https://docs.rs/polars/latest/polars/chunked_array/arithmetic/trait.ArithmeticChunked.html#associatedtype.Out-1" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/chunked_array/arithmetic/trait.ArithmeticChunked.html#associatedtype.Out" class="associatedtype">Out</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<T\>

<a href="https://docs.rs/polars/latest/polars/chunked_array/arithmetic/trait.ArithmeticChunked.html#associatedtype.TrueDivOut-1" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/chunked_array/arithmetic/trait.ArithmeticChunked.html#associatedtype.TrueDivOut" class="associatedtype">TrueDivOut</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<\<\<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html" class="trait" title="trait polars::prelude::PolarsNumericType">PolarsNumericType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html#associatedtype.Native" class="associatedtype" title="type polars::prelude::PolarsNumericType::Native">Native</a> as <a href="https://docs.rs/polars/latest/polars/prelude/trait.NumericNative.html" class="trait" title="trait polars::prelude::NumericNative">NumericNative</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.NumericNative.html#associatedtype.TrueDivPolarsType" class="associatedtype" title="type polars::prelude::NumericNative::TrueDivPolarsType">TrueDivPolarsType</a>\>

<a href="https://docs.rs/polars/latest/polars/chunked_array/arithmetic/trait.ArithmeticChunked.html#impl-ArithmeticChunked-for-ChunkedArray%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.ArithmeticChunked.html" class="trait" title="trait polars::prelude::ArithmeticChunked">ArithmeticChunked</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<T\>

where T: <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html" class="trait" title="trait polars::prelude::PolarsNumericType">PolarsNumericType</a>,

<a href="https://docs.rs/polars/latest/polars/chunked_array/arithmetic/trait.ArithmeticChunked.html#associatedtype.Scalar-2" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/chunked_array/arithmetic/trait.ArithmeticChunked.html#associatedtype.Scalar" class="associatedtype">Scalar</a> = \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html" class="trait" title="trait polars::prelude::PolarsNumericType">PolarsNumericType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html#associatedtype.Native" class="associatedtype" title="type polars::prelude::PolarsNumericType::Native">Native</a>

<a href="https://docs.rs/polars/latest/polars/chunked_array/arithmetic/trait.ArithmeticChunked.html#associatedtype.Out-2" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/chunked_array/arithmetic/trait.ArithmeticChunked.html#associatedtype.Out" class="associatedtype">Out</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<T\>

<a href="https://docs.rs/polars/latest/polars/chunked_array/arithmetic/trait.ArithmeticChunked.html#associatedtype.TrueDivOut-2" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/chunked_array/arithmetic/trait.ArithmeticChunked.html#associatedtype.TrueDivOut" class="associatedtype">TrueDivOut</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<\<\<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html" class="trait" title="trait polars::prelude::PolarsNumericType">PolarsNumericType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html#associatedtype.Native" class="associatedtype" title="type polars::prelude::PolarsNumericType::Native">Native</a> as <a href="https://docs.rs/polars/latest/polars/prelude/trait.NumericNative.html" class="trait" title="trait polars::prelude::NumericNative">NumericNative</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.NumericNative.html#associatedtype.TrueDivPolarsType" class="associatedtype" title="type polars::prelude::NumericNative::TrueDivPolarsType">TrueDivPolarsType</a>\>
