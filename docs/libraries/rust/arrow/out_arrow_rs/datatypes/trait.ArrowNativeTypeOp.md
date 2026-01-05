# Trait ArrowNativeTypeOp Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/arithmetic.rs.html#41" class="src">Source</a>

``` rust
pub trait ArrowNativeTypeOp: ArrowNativeType {
    const ZERO: Self;
    const ONE: Self;
    const MIN_TOTAL_ORDER: Self;
    const MAX_TOTAL_ORDER: Self;
Show 22 methods
    // Required methods
    fn add_checked(self, rhs: Self) -> Result<Self, ArrowError>;
    fn add_wrapping(self, rhs: Self) -> Self;
    fn sub_checked(self, rhs: Self) -> Result<Self, ArrowError>;
    fn sub_wrapping(self, rhs: Self) -> Self;
    fn mul_checked(self, rhs: Self) -> Result<Self, ArrowError>;
    fn mul_wrapping(self, rhs: Self) -> Self;
    fn div_checked(self, rhs: Self) -> Result<Self, ArrowError>;
    fn div_wrapping(self, rhs: Self) -> Self;
    fn mod_checked(self, rhs: Self) -> Result<Self, ArrowError>;
    fn mod_wrapping(self, rhs: Self) -> Self;
    fn neg_checked(self) -> Result<Self, ArrowError>;
    fn neg_wrapping(self) -> Self;
    fn pow_checked(self, exp: u32) -> Result<Self, ArrowError>;
    fn pow_wrapping(self, exp: u32) -> Self;
    fn is_zero(self) -> bool;
    fn compare(self, rhs: Self) -> Ordering;
    fn is_eq(self, rhs: Self) -> bool;

    // Provided methods
    fn is_ne(self, rhs: Self) -> bool { ... }
    fn is_lt(self, rhs: Self) -> bool { ... }
    fn is_le(self, rhs: Self) -> bool { ... }
    fn is_gt(self, rhs: Self) -> bool { ... }
    fn is_ge(self, rhs: Self) -> bool { ... }
}
```

Expand description

Trait for [`ArrowNativeType`](https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html "trait arrow::datatypes::ArrowNativeType") that adds checked and unchecked arithmetic operations, and totally ordered comparison operations

The APIs with `_wrapping` suffix do not perform overflow-checking. For integer types they will wrap around the boundary of the type. For floating point types they will overflow to INF or -INF preserving the expected sign value

Note `div_wrapping` and `mod_wrapping` will panic for integer types if `rhs` is zero although this may be subject to change <https://github.com/apache/arrow-rs/issues/2647>

The APIs with `_checked` suffix perform overflow-checking. For integer types these will return `Err` instead of wrapping. For floating point types they will overflow to INF or -INF preserving the expected sign value

Comparison of integer types is as per normal integer comparison rules, floating point values are compared as per IEEE 754’s totalOrder predicate see [`f32::total_cmp`](https://doc.rust-lang.org/nightly/std/primitive.f32.html#method.total_cmp "method f32::total_cmp")

## Required Associated Constants<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#required-associated-consts" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.ZERO" class="constant">ZERO</a>: Self

The additive identity

#### const <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.ONE" class="constant">ONE</a>: Self

The multiplicative identity

#### const <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.MIN_TOTAL_ORDER" class="constant">MIN_TOTAL_ORDER</a>: Self

The minimum value and identity for the `max` aggregation. Note that the aggregation uses the total order predicate for floating point values, which means that this value is a negative NaN.

#### const <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.MAX_TOTAL_ORDER" class="constant">MAX_TOTAL_ORDER</a>: Self

The maximum value and identity for the `min` aggregation. Note that the aggregation uses the total order predicate for floating point values, which means that this value is a positive NaN.

## Required Methods<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.add_checked" class="fn">add_checked</a>(self, rhs: Self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<Self, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

Checked addition operation

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.add_wrapping" class="fn">add_wrapping</a>(self, rhs: Self) -\> Self

Wrapping addition operation

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.sub_checked" class="fn">sub_checked</a>(self, rhs: Self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<Self, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

Checked subtraction operation

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.sub_wrapping" class="fn">sub_wrapping</a>(self, rhs: Self) -\> Self

Wrapping subtraction operation

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.mul_checked" class="fn">mul_checked</a>(self, rhs: Self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<Self, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

Checked multiplication operation

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.mul_wrapping" class="fn">mul_wrapping</a>(self, rhs: Self) -\> Self

Wrapping multiplication operation

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.div_checked" class="fn">div_checked</a>(self, rhs: Self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<Self, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

Checked division operation

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.div_wrapping" class="fn">div_wrapping</a>(self, rhs: Self) -\> Self

Wrapping division operation

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.mod_checked" class="fn">mod_checked</a>(self, rhs: Self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<Self, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

Checked remainder operation

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.mod_wrapping" class="fn">mod_wrapping</a>(self, rhs: Self) -\> Self

Wrapping remainder operation

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.neg_checked" class="fn">neg_checked</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<Self, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

Checked negation operation

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.neg_wrapping" class="fn">neg_wrapping</a>(self) -\> Self

Wrapping negation operation

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.pow_checked" class="fn">pow_checked</a>(self, exp: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<Self, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

Checked exponentiation operation

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.pow_wrapping" class="fn">pow_wrapping</a>(self, exp: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>) -\> Self

Wrapping exponentiation operation

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.is_zero" class="fn">is_zero</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if zero else false

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.compare" class="fn">compare</a>(self, rhs: Self) -\> <a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>

Compare operation

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.is_eq" class="fn">is_eq</a>(self, rhs: Self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Equality operation

## Provided Methods<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#provided-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.is_ne" class="fn">is_ne</a>(self, rhs: Self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Not equal operation

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.is_lt" class="fn">is_lt</a>(self, rhs: Self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Less than operation

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.is_le" class="fn">is_le</a>(self, rhs: Self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Less than equals operation

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.is_gt" class="fn">is_gt</a>(self, rhs: Self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Greater than operation

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.is_ge" class="fn">is_ge</a>(self, rhs: Self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Greater than equals operation

## Dyn Compatibility<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#dyn-compatibility" class="anchor">§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementations on Foreign Types<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#foreign-impls" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#impl-ArrowNativeTypeOp-for-f32" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowNativeTypeOp.html" class="trait" title="trait arrow::array::ArrowNativeTypeOp">ArrowNativeTypeOp</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.ZERO-1" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.ZERO" class="constant">ZERO</a>: <a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a> = 0f32

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.ONE-1" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.ONE" class="constant">ONE</a>: <a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a> = 1f32

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.MIN_TOTAL_ORDER-1" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.MIN_TOTAL_ORDER" class="constant">MIN_TOTAL_ORDER</a>: <a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a> = NaN_f32

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.MAX_TOTAL_ORDER-1" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.MAX_TOTAL_ORDER" class="constant">MAX_TOTAL_ORDER</a>: <a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a> = NaN_f32

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.add_checked" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.add_checked" class="fn">add_checked</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.add_wrapping" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.add_wrapping" class="fn">add_wrapping</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.sub_checked" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.sub_checked" class="fn">sub_checked</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.sub_wrapping" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.sub_wrapping" class="fn">sub_wrapping</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.mul_checked" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.mul_checked" class="fn">mul_checked</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.mul_wrapping" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.mul_wrapping" class="fn">mul_wrapping</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.div_checked" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.div_checked" class="fn">div_checked</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.div_wrapping" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.div_wrapping" class="fn">div_wrapping</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.mod_checked" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.mod_checked" class="fn">mod_checked</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.mod_wrapping" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.mod_wrapping" class="fn">mod_wrapping</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.neg_checked" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.neg_checked" class="fn">neg_checked</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.neg_wrapping" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.neg_wrapping" class="fn">neg_wrapping</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.pow_checked" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.pow_checked" class="fn">pow_checked</a>(self, exp: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.pow_wrapping" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.pow_wrapping" class="fn">pow_wrapping</a>(self, exp: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.is_zero" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.is_zero" class="fn">is_zero</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.compare" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.compare" class="fn">compare</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.is_eq" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.is_eq" class="fn">is_eq</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#impl-ArrowNativeTypeOp-for-f64" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowNativeTypeOp.html" class="trait" title="trait arrow::array::ArrowNativeTypeOp">ArrowNativeTypeOp</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.ZERO-2" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.ZERO" class="constant">ZERO</a>: <a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a> = 0f64

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.ONE-2" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.ONE" class="constant">ONE</a>: <a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a> = 1f64

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.MIN_TOTAL_ORDER-2" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.MIN_TOTAL_ORDER" class="constant">MIN_TOTAL_ORDER</a>: <a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a> = NaN_f64

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.MAX_TOTAL_ORDER-2" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.MAX_TOTAL_ORDER" class="constant">MAX_TOTAL_ORDER</a>: <a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a> = NaN_f64

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.add_checked-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.add_checked" class="fn">add_checked</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.add_wrapping-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.add_wrapping" class="fn">add_wrapping</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.sub_checked-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.sub_checked" class="fn">sub_checked</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.sub_wrapping-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.sub_wrapping" class="fn">sub_wrapping</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.mul_checked-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.mul_checked" class="fn">mul_checked</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.mul_wrapping-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.mul_wrapping" class="fn">mul_wrapping</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.div_checked-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.div_checked" class="fn">div_checked</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.div_wrapping-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.div_wrapping" class="fn">div_wrapping</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.mod_checked-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.mod_checked" class="fn">mod_checked</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.mod_wrapping-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.mod_wrapping" class="fn">mod_wrapping</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.neg_checked-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.neg_checked" class="fn">neg_checked</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.neg_wrapping-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.neg_wrapping" class="fn">neg_wrapping</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.pow_checked-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.pow_checked" class="fn">pow_checked</a>(self, exp: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.pow_wrapping-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.pow_wrapping" class="fn">pow_wrapping</a>(self, exp: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.is_zero-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.is_zero" class="fn">is_zero</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.compare-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.compare" class="fn">compare</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.is_eq-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.is_eq" class="fn">is_eq</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#impl-ArrowNativeTypeOp-for-i8" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowNativeTypeOp.html" class="trait" title="trait arrow::array::ArrowNativeTypeOp">ArrowNativeTypeOp</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.ZERO-3" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.ZERO" class="constant">ZERO</a>: <a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a> = 0i8

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.ONE-3" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.ONE" class="constant">ONE</a>: <a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a> = 1i8

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.MIN_TOTAL_ORDER-3" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.MIN_TOTAL_ORDER" class="constant">MIN_TOTAL_ORDER</a>: <a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a> = -128i8

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.MAX_TOTAL_ORDER-3" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.MAX_TOTAL_ORDER" class="constant">MAX_TOTAL_ORDER</a>: <a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a> = 127i8

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.add_checked-2" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.add_checked" class="fn">add_checked</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.add_wrapping-2" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.add_wrapping" class="fn">add_wrapping</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.sub_checked-2" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.sub_checked" class="fn">sub_checked</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.sub_wrapping-2" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.sub_wrapping" class="fn">sub_wrapping</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.mul_checked-2" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.mul_checked" class="fn">mul_checked</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.mul_wrapping-2" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.mul_wrapping" class="fn">mul_wrapping</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.div_checked-2" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.div_checked" class="fn">div_checked</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.div_wrapping-2" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.div_wrapping" class="fn">div_wrapping</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.mod_checked-2" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.mod_checked" class="fn">mod_checked</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.mod_wrapping-2" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.mod_wrapping" class="fn">mod_wrapping</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.neg_checked-2" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.neg_checked" class="fn">neg_checked</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.pow_checked-2" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.pow_checked" class="fn">pow_checked</a>(self, exp: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.pow_wrapping-2" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.pow_wrapping" class="fn">pow_wrapping</a>(self, exp: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.neg_wrapping-2" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.neg_wrapping" class="fn">neg_wrapping</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.is_zero-2" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.is_zero" class="fn">is_zero</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.compare-2" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.compare" class="fn">compare</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.is_eq-2" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.is_eq" class="fn">is_eq</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#impl-ArrowNativeTypeOp-for-i16" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowNativeTypeOp.html" class="trait" title="trait arrow::array::ArrowNativeTypeOp">ArrowNativeTypeOp</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.ZERO-4" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.ZERO" class="constant">ZERO</a>: <a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a> = 0i16

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.ONE-4" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.ONE" class="constant">ONE</a>: <a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a> = 1i16

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.MIN_TOTAL_ORDER-4" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.MIN_TOTAL_ORDER" class="constant">MIN_TOTAL_ORDER</a>: <a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a> = -32_768i16

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.MAX_TOTAL_ORDER-4" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.MAX_TOTAL_ORDER" class="constant">MAX_TOTAL_ORDER</a>: <a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a> = 32_767i16

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.add_checked-3" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.add_checked" class="fn">add_checked</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.add_wrapping-3" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.add_wrapping" class="fn">add_wrapping</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.sub_checked-3" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.sub_checked" class="fn">sub_checked</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.sub_wrapping-3" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.sub_wrapping" class="fn">sub_wrapping</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.mul_checked-3" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.mul_checked" class="fn">mul_checked</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.mul_wrapping-3" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.mul_wrapping" class="fn">mul_wrapping</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.div_checked-3" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.div_checked" class="fn">div_checked</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.div_wrapping-3" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.div_wrapping" class="fn">div_wrapping</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.mod_checked-3" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.mod_checked" class="fn">mod_checked</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.mod_wrapping-3" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.mod_wrapping" class="fn">mod_wrapping</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.neg_checked-3" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.neg_checked" class="fn">neg_checked</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.pow_checked-3" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.pow_checked" class="fn">pow_checked</a>(self, exp: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.pow_wrapping-3" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.pow_wrapping" class="fn">pow_wrapping</a>(self, exp: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.neg_wrapping-3" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.neg_wrapping" class="fn">neg_wrapping</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.is_zero-3" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.is_zero" class="fn">is_zero</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.compare-3" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.compare" class="fn">compare</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.is_eq-3" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.is_eq" class="fn">is_eq</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#impl-ArrowNativeTypeOp-for-i32" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowNativeTypeOp.html" class="trait" title="trait arrow::array::ArrowNativeTypeOp">ArrowNativeTypeOp</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.ZERO-5" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.ZERO" class="constant">ZERO</a>: <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a> = 0i32

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.ONE-5" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.ONE" class="constant">ONE</a>: <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a> = 1i32

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.MIN_TOTAL_ORDER-5" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.MIN_TOTAL_ORDER" class="constant">MIN_TOTAL_ORDER</a>: <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a> = -2_147_483_648i32

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.MAX_TOTAL_ORDER-5" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.MAX_TOTAL_ORDER" class="constant">MAX_TOTAL_ORDER</a>: <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a> = 2_147_483_647i32

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.add_checked-4" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.add_checked" class="fn">add_checked</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.add_wrapping-4" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.add_wrapping" class="fn">add_wrapping</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.sub_checked-4" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.sub_checked" class="fn">sub_checked</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.sub_wrapping-4" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.sub_wrapping" class="fn">sub_wrapping</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.mul_checked-4" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.mul_checked" class="fn">mul_checked</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.mul_wrapping-4" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.mul_wrapping" class="fn">mul_wrapping</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.div_checked-4" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.div_checked" class="fn">div_checked</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.div_wrapping-4" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.div_wrapping" class="fn">div_wrapping</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.mod_checked-4" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.mod_checked" class="fn">mod_checked</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.mod_wrapping-4" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.mod_wrapping" class="fn">mod_wrapping</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.neg_checked-4" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.neg_checked" class="fn">neg_checked</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.pow_checked-4" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.pow_checked" class="fn">pow_checked</a>(self, exp: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.pow_wrapping-4" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.pow_wrapping" class="fn">pow_wrapping</a>(self, exp: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.neg_wrapping-4" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.neg_wrapping" class="fn">neg_wrapping</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.is_zero-4" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.is_zero" class="fn">is_zero</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.compare-4" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.compare" class="fn">compare</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.is_eq-4" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.is_eq" class="fn">is_eq</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#impl-ArrowNativeTypeOp-for-i64" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowNativeTypeOp.html" class="trait" title="trait arrow::array::ArrowNativeTypeOp">ArrowNativeTypeOp</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.ZERO-6" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.ZERO" class="constant">ZERO</a>: <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a> = 0i64

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.ONE-6" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.ONE" class="constant">ONE</a>: <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a> = 1i64

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.MIN_TOTAL_ORDER-6" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.MIN_TOTAL_ORDER" class="constant">MIN_TOTAL_ORDER</a>: <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a> = -9_223_372_036_854_775_808i64

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.MAX_TOTAL_ORDER-6" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.MAX_TOTAL_ORDER" class="constant">MAX_TOTAL_ORDER</a>: <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a> = 9_223_372_036_854_775_807i64

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.add_checked-5" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.add_checked" class="fn">add_checked</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.add_wrapping-5" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.add_wrapping" class="fn">add_wrapping</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.sub_checked-5" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.sub_checked" class="fn">sub_checked</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.sub_wrapping-5" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.sub_wrapping" class="fn">sub_wrapping</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.mul_checked-5" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.mul_checked" class="fn">mul_checked</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.mul_wrapping-5" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.mul_wrapping" class="fn">mul_wrapping</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.div_checked-5" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.div_checked" class="fn">div_checked</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.div_wrapping-5" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.div_wrapping" class="fn">div_wrapping</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.mod_checked-5" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.mod_checked" class="fn">mod_checked</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.mod_wrapping-5" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.mod_wrapping" class="fn">mod_wrapping</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.neg_checked-5" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.neg_checked" class="fn">neg_checked</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.pow_checked-5" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.pow_checked" class="fn">pow_checked</a>(self, exp: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.pow_wrapping-5" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.pow_wrapping" class="fn">pow_wrapping</a>(self, exp: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.neg_wrapping-5" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.neg_wrapping" class="fn">neg_wrapping</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.is_zero-5" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.is_zero" class="fn">is_zero</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.compare-5" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.compare" class="fn">compare</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.is_eq-5" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.is_eq" class="fn">is_eq</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#impl-ArrowNativeTypeOp-for-i128" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowNativeTypeOp.html" class="trait" title="trait arrow::array::ArrowNativeTypeOp">ArrowNativeTypeOp</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.ZERO-7" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.ZERO" class="constant">ZERO</a>: <a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a> = 0i128

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.ONE-7" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.ONE" class="constant">ONE</a>: <a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a> = 1i128

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.MIN_TOTAL_ORDER-7" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.MIN_TOTAL_ORDER" class="constant">MIN_TOTAL_ORDER</a>: <a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a> = -170_141_183_460_469_231_731_687_303_715_884_105_728i128

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.MAX_TOTAL_ORDER-7" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.MAX_TOTAL_ORDER" class="constant">MAX_TOTAL_ORDER</a>: <a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a> = 170_141_183_460_469_231_731_687_303_715_884_105_727i128

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.add_checked-6" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.add_checked" class="fn">add_checked</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.add_wrapping-6" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.add_wrapping" class="fn">add_wrapping</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.sub_checked-6" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.sub_checked" class="fn">sub_checked</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.sub_wrapping-6" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.sub_wrapping" class="fn">sub_wrapping</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.mul_checked-6" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.mul_checked" class="fn">mul_checked</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.mul_wrapping-6" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.mul_wrapping" class="fn">mul_wrapping</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.div_checked-6" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.div_checked" class="fn">div_checked</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.div_wrapping-6" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.div_wrapping" class="fn">div_wrapping</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.mod_checked-6" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.mod_checked" class="fn">mod_checked</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.mod_wrapping-6" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.mod_wrapping" class="fn">mod_wrapping</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.neg_checked-6" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.neg_checked" class="fn">neg_checked</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.pow_checked-6" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.pow_checked" class="fn">pow_checked</a>(self, exp: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.pow_wrapping-6" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.pow_wrapping" class="fn">pow_wrapping</a>(self, exp: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.neg_wrapping-6" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.neg_wrapping" class="fn">neg_wrapping</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.is_zero-6" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.is_zero" class="fn">is_zero</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.compare-6" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.compare" class="fn">compare</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.is_eq-6" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.is_eq" class="fn">is_eq</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#impl-ArrowNativeTypeOp-for-u8" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowNativeTypeOp.html" class="trait" title="trait arrow::array::ArrowNativeTypeOp">ArrowNativeTypeOp</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.ZERO-8" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.ZERO" class="constant">ZERO</a>: <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a> = 0u8

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.ONE-8" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.ONE" class="constant">ONE</a>: <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a> = 1u8

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.MIN_TOTAL_ORDER-8" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.MIN_TOTAL_ORDER" class="constant">MIN_TOTAL_ORDER</a>: <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a> = 0u8

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.MAX_TOTAL_ORDER-8" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.MAX_TOTAL_ORDER" class="constant">MAX_TOTAL_ORDER</a>: <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a> = 255u8

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.add_checked-7" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.add_checked" class="fn">add_checked</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.add_wrapping-7" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.add_wrapping" class="fn">add_wrapping</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.sub_checked-7" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.sub_checked" class="fn">sub_checked</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.sub_wrapping-7" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.sub_wrapping" class="fn">sub_wrapping</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.mul_checked-7" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.mul_checked" class="fn">mul_checked</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.mul_wrapping-7" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.mul_wrapping" class="fn">mul_wrapping</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.div_checked-7" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.div_checked" class="fn">div_checked</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.div_wrapping-7" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.div_wrapping" class="fn">div_wrapping</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.mod_checked-7" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.mod_checked" class="fn">mod_checked</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.mod_wrapping-7" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.mod_wrapping" class="fn">mod_wrapping</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.neg_checked-7" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.neg_checked" class="fn">neg_checked</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.pow_checked-7" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.pow_checked" class="fn">pow_checked</a>(self, exp: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.pow_wrapping-7" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.pow_wrapping" class="fn">pow_wrapping</a>(self, exp: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.neg_wrapping-7" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.neg_wrapping" class="fn">neg_wrapping</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.is_zero-7" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.is_zero" class="fn">is_zero</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.compare-7" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.compare" class="fn">compare</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.is_eq-7" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.is_eq" class="fn">is_eq</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#impl-ArrowNativeTypeOp-for-u16" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowNativeTypeOp.html" class="trait" title="trait arrow::array::ArrowNativeTypeOp">ArrowNativeTypeOp</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.ZERO-9" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.ZERO" class="constant">ZERO</a>: <a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a> = 0u16

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.ONE-9" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.ONE" class="constant">ONE</a>: <a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a> = 1u16

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.MIN_TOTAL_ORDER-9" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.MIN_TOTAL_ORDER" class="constant">MIN_TOTAL_ORDER</a>: <a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a> = 0u16

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.MAX_TOTAL_ORDER-9" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.MAX_TOTAL_ORDER" class="constant">MAX_TOTAL_ORDER</a>: <a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a> = 65_535u16

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.add_checked-8" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.add_checked" class="fn">add_checked</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.add_wrapping-8" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.add_wrapping" class="fn">add_wrapping</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.sub_checked-8" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.sub_checked" class="fn">sub_checked</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.sub_wrapping-8" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.sub_wrapping" class="fn">sub_wrapping</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.mul_checked-8" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.mul_checked" class="fn">mul_checked</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.mul_wrapping-8" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.mul_wrapping" class="fn">mul_wrapping</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.div_checked-8" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.div_checked" class="fn">div_checked</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.div_wrapping-8" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.div_wrapping" class="fn">div_wrapping</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.mod_checked-8" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.mod_checked" class="fn">mod_checked</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.mod_wrapping-8" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.mod_wrapping" class="fn">mod_wrapping</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.neg_checked-8" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.neg_checked" class="fn">neg_checked</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.pow_checked-8" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.pow_checked" class="fn">pow_checked</a>(self, exp: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.pow_wrapping-8" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.pow_wrapping" class="fn">pow_wrapping</a>(self, exp: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.neg_wrapping-8" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.neg_wrapping" class="fn">neg_wrapping</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.is_zero-8" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.is_zero" class="fn">is_zero</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.compare-8" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.compare" class="fn">compare</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.is_eq-8" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.is_eq" class="fn">is_eq</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#impl-ArrowNativeTypeOp-for-u32" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowNativeTypeOp.html" class="trait" title="trait arrow::array::ArrowNativeTypeOp">ArrowNativeTypeOp</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.ZERO-10" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.ZERO" class="constant">ZERO</a>: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a> = 0u32

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.ONE-10" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.ONE" class="constant">ONE</a>: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a> = 1u32

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.MIN_TOTAL_ORDER-10" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.MIN_TOTAL_ORDER" class="constant">MIN_TOTAL_ORDER</a>: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a> = 0u32

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.MAX_TOTAL_ORDER-10" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.MAX_TOTAL_ORDER" class="constant">MAX_TOTAL_ORDER</a>: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a> = 4_294_967_295u32

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.add_checked-9" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.add_checked" class="fn">add_checked</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.add_wrapping-9" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.add_wrapping" class="fn">add_wrapping</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.sub_checked-9" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.sub_checked" class="fn">sub_checked</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.sub_wrapping-9" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.sub_wrapping" class="fn">sub_wrapping</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.mul_checked-9" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.mul_checked" class="fn">mul_checked</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.mul_wrapping-9" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.mul_wrapping" class="fn">mul_wrapping</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.div_checked-9" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.div_checked" class="fn">div_checked</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.div_wrapping-9" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.div_wrapping" class="fn">div_wrapping</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.mod_checked-9" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.mod_checked" class="fn">mod_checked</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.mod_wrapping-9" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.mod_wrapping" class="fn">mod_wrapping</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.neg_checked-9" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.neg_checked" class="fn">neg_checked</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.pow_checked-9" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.pow_checked" class="fn">pow_checked</a>(self, exp: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.pow_wrapping-9" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.pow_wrapping" class="fn">pow_wrapping</a>(self, exp: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.neg_wrapping-9" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.neg_wrapping" class="fn">neg_wrapping</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.is_zero-9" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.is_zero" class="fn">is_zero</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.compare-9" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.compare" class="fn">compare</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.is_eq-9" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.is_eq" class="fn">is_eq</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#impl-ArrowNativeTypeOp-for-u64" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowNativeTypeOp.html" class="trait" title="trait arrow::array::ArrowNativeTypeOp">ArrowNativeTypeOp</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.ZERO-11" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.ZERO" class="constant">ZERO</a>: <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a> = 0u64

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.ONE-11" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.ONE" class="constant">ONE</a>: <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a> = 1u64

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.MIN_TOTAL_ORDER-11" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.MIN_TOTAL_ORDER" class="constant">MIN_TOTAL_ORDER</a>: <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a> = 0u64

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.MAX_TOTAL_ORDER-11" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.MAX_TOTAL_ORDER" class="constant">MAX_TOTAL_ORDER</a>: <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a> = 18_446_744_073_709_551_615u64

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.add_checked-10" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.add_checked" class="fn">add_checked</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.add_wrapping-10" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.add_wrapping" class="fn">add_wrapping</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.sub_checked-10" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.sub_checked" class="fn">sub_checked</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.sub_wrapping-10" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.sub_wrapping" class="fn">sub_wrapping</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.mul_checked-10" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.mul_checked" class="fn">mul_checked</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.mul_wrapping-10" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.mul_wrapping" class="fn">mul_wrapping</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.div_checked-10" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.div_checked" class="fn">div_checked</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.div_wrapping-10" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.div_wrapping" class="fn">div_wrapping</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.mod_checked-10" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.mod_checked" class="fn">mod_checked</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.mod_wrapping-10" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.mod_wrapping" class="fn">mod_wrapping</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.neg_checked-10" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.neg_checked" class="fn">neg_checked</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.pow_checked-10" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.pow_checked" class="fn">pow_checked</a>(self, exp: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.pow_wrapping-10" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.pow_wrapping" class="fn">pow_wrapping</a>(self, exp: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.neg_wrapping-10" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.neg_wrapping" class="fn">neg_wrapping</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.is_zero-10" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.is_zero" class="fn">is_zero</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.compare-10" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.compare" class="fn">compare</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.is_eq-10" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.is_eq" class="fn">is_eq</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#impl-ArrowNativeTypeOp-for-f16" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowNativeTypeOp.html" class="trait" title="trait arrow::array::ArrowNativeTypeOp">ArrowNativeTypeOp</a> for <a href="https://docs.rs/half/2.6.0/x86_64-unknown-linux-gnu/half/binary16/struct.f16.html" class="struct" title="struct half::binary16::f16">f16</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.ZERO-12" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.ZERO" class="constant">ZERO</a>: <a href="https://docs.rs/half/2.6.0/x86_64-unknown-linux-gnu/half/binary16/struct.f16.html" class="struct" title="struct half::binary16::f16">f16</a> = f16::ZERO

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.ONE-12" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.ONE" class="constant">ONE</a>: <a href="https://docs.rs/half/2.6.0/x86_64-unknown-linux-gnu/half/binary16/struct.f16.html" class="struct" title="struct half::binary16::f16">f16</a> = f16::ONE

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.MIN_TOTAL_ORDER-12" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.MIN_TOTAL_ORDER" class="constant">MIN_TOTAL_ORDER</a>: <a href="https://docs.rs/half/2.6.0/x86_64-unknown-linux-gnu/half/binary16/struct.f16.html" class="struct" title="struct half::binary16::f16">f16</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.MAX_TOTAL_ORDER-12" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.MAX_TOTAL_ORDER" class="constant">MAX_TOTAL_ORDER</a>: <a href="https://docs.rs/half/2.6.0/x86_64-unknown-linux-gnu/half/binary16/struct.f16.html" class="struct" title="struct half::binary16::f16">f16</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.add_checked-11" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.add_checked" class="fn">add_checked</a>(self, rhs: <a href="https://docs.rs/half/2.6.0/x86_64-unknown-linux-gnu/half/binary16/struct.f16.html" class="struct" title="struct half::binary16::f16">f16</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/half/2.6.0/x86_64-unknown-linux-gnu/half/binary16/struct.f16.html" class="struct" title="struct half::binary16::f16">f16</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.add_wrapping-11" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.add_wrapping" class="fn">add_wrapping</a>(self, rhs: <a href="https://docs.rs/half/2.6.0/x86_64-unknown-linux-gnu/half/binary16/struct.f16.html" class="struct" title="struct half::binary16::f16">f16</a>) -\> <a href="https://docs.rs/half/2.6.0/x86_64-unknown-linux-gnu/half/binary16/struct.f16.html" class="struct" title="struct half::binary16::f16">f16</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.sub_checked-11" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.sub_checked" class="fn">sub_checked</a>(self, rhs: <a href="https://docs.rs/half/2.6.0/x86_64-unknown-linux-gnu/half/binary16/struct.f16.html" class="struct" title="struct half::binary16::f16">f16</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/half/2.6.0/x86_64-unknown-linux-gnu/half/binary16/struct.f16.html" class="struct" title="struct half::binary16::f16">f16</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.sub_wrapping-11" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.sub_wrapping" class="fn">sub_wrapping</a>(self, rhs: <a href="https://docs.rs/half/2.6.0/x86_64-unknown-linux-gnu/half/binary16/struct.f16.html" class="struct" title="struct half::binary16::f16">f16</a>) -\> <a href="https://docs.rs/half/2.6.0/x86_64-unknown-linux-gnu/half/binary16/struct.f16.html" class="struct" title="struct half::binary16::f16">f16</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.mul_checked-11" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.mul_checked" class="fn">mul_checked</a>(self, rhs: <a href="https://docs.rs/half/2.6.0/x86_64-unknown-linux-gnu/half/binary16/struct.f16.html" class="struct" title="struct half::binary16::f16">f16</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/half/2.6.0/x86_64-unknown-linux-gnu/half/binary16/struct.f16.html" class="struct" title="struct half::binary16::f16">f16</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.mul_wrapping-11" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.mul_wrapping" class="fn">mul_wrapping</a>(self, rhs: <a href="https://docs.rs/half/2.6.0/x86_64-unknown-linux-gnu/half/binary16/struct.f16.html" class="struct" title="struct half::binary16::f16">f16</a>) -\> <a href="https://docs.rs/half/2.6.0/x86_64-unknown-linux-gnu/half/binary16/struct.f16.html" class="struct" title="struct half::binary16::f16">f16</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.div_checked-11" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.div_checked" class="fn">div_checked</a>(self, rhs: <a href="https://docs.rs/half/2.6.0/x86_64-unknown-linux-gnu/half/binary16/struct.f16.html" class="struct" title="struct half::binary16::f16">f16</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/half/2.6.0/x86_64-unknown-linux-gnu/half/binary16/struct.f16.html" class="struct" title="struct half::binary16::f16">f16</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.div_wrapping-11" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.div_wrapping" class="fn">div_wrapping</a>(self, rhs: <a href="https://docs.rs/half/2.6.0/x86_64-unknown-linux-gnu/half/binary16/struct.f16.html" class="struct" title="struct half::binary16::f16">f16</a>) -\> <a href="https://docs.rs/half/2.6.0/x86_64-unknown-linux-gnu/half/binary16/struct.f16.html" class="struct" title="struct half::binary16::f16">f16</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.mod_checked-11" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.mod_checked" class="fn">mod_checked</a>(self, rhs: <a href="https://docs.rs/half/2.6.0/x86_64-unknown-linux-gnu/half/binary16/struct.f16.html" class="struct" title="struct half::binary16::f16">f16</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/half/2.6.0/x86_64-unknown-linux-gnu/half/binary16/struct.f16.html" class="struct" title="struct half::binary16::f16">f16</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.mod_wrapping-11" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.mod_wrapping" class="fn">mod_wrapping</a>(self, rhs: <a href="https://docs.rs/half/2.6.0/x86_64-unknown-linux-gnu/half/binary16/struct.f16.html" class="struct" title="struct half::binary16::f16">f16</a>) -\> <a href="https://docs.rs/half/2.6.0/x86_64-unknown-linux-gnu/half/binary16/struct.f16.html" class="struct" title="struct half::binary16::f16">f16</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.neg_checked-11" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.neg_checked" class="fn">neg_checked</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/half/2.6.0/x86_64-unknown-linux-gnu/half/binary16/struct.f16.html" class="struct" title="struct half::binary16::f16">f16</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.neg_wrapping-11" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.neg_wrapping" class="fn">neg_wrapping</a>(self) -\> <a href="https://docs.rs/half/2.6.0/x86_64-unknown-linux-gnu/half/binary16/struct.f16.html" class="struct" title="struct half::binary16::f16">f16</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.pow_checked-11" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.pow_checked" class="fn">pow_checked</a>(self, exp: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/half/2.6.0/x86_64-unknown-linux-gnu/half/binary16/struct.f16.html" class="struct" title="struct half::binary16::f16">f16</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.pow_wrapping-11" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.pow_wrapping" class="fn">pow_wrapping</a>(self, exp: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>) -\> <a href="https://docs.rs/half/2.6.0/x86_64-unknown-linux-gnu/half/binary16/struct.f16.html" class="struct" title="struct half::binary16::f16">f16</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.is_zero-11" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.is_zero" class="fn">is_zero</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.compare-11" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.compare" class="fn">compare</a>(self, rhs: <a href="https://docs.rs/half/2.6.0/x86_64-unknown-linux-gnu/half/binary16/struct.f16.html" class="struct" title="struct half::binary16::f16">f16</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#method.is_eq-11" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#tymethod.is_eq" class="fn">is_eq</a>(self, rhs: <a href="https://docs.rs/half/2.6.0/x86_64-unknown-linux-gnu/half/binary16/struct.f16.html" class="struct" title="struct half::binary16::f16">f16</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

## Implementors<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#impl-ArrowNativeTypeOp-for-IntervalDayTime" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowNativeTypeOp.html" class="trait" title="trait arrow::array::ArrowNativeTypeOp">ArrowNativeTypeOp</a> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.IntervalDayTime.html" class="struct" title="struct arrow::datatypes::IntervalDayTime">IntervalDayTime</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.ZERO-13" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.ZERO" class="constant">ZERO</a>: <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.IntervalDayTime.html" class="struct" title="struct arrow::datatypes::IntervalDayTime">IntervalDayTime</a> = IntervalDayTime::ZERO

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.ONE-13" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.ONE" class="constant">ONE</a>: <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.IntervalDayTime.html" class="struct" title="struct arrow::datatypes::IntervalDayTime">IntervalDayTime</a> = IntervalDayTime::ONE

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.MIN_TOTAL_ORDER-13" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.MIN_TOTAL_ORDER" class="constant">MIN_TOTAL_ORDER</a>: <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.IntervalDayTime.html" class="struct" title="struct arrow::datatypes::IntervalDayTime">IntervalDayTime</a> = IntervalDayTime::MIN

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.MAX_TOTAL_ORDER-13" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.MAX_TOTAL_ORDER" class="constant">MAX_TOTAL_ORDER</a>: <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.IntervalDayTime.html" class="struct" title="struct arrow::datatypes::IntervalDayTime">IntervalDayTime</a> = IntervalDayTime::MAX

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#impl-ArrowNativeTypeOp-for-IntervalMonthDayNano" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowNativeTypeOp.html" class="trait" title="trait arrow::array::ArrowNativeTypeOp">ArrowNativeTypeOp</a> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.ZERO-14" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.ZERO" class="constant">ZERO</a>: <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a> = IntervalMonthDayNano::ZERO

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.ONE-14" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.ONE" class="constant">ONE</a>: <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a> = IntervalMonthDayNano::ONE

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.MIN_TOTAL_ORDER-14" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.MIN_TOTAL_ORDER" class="constant">MIN_TOTAL_ORDER</a>: <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a> = IntervalMonthDayNano::MIN

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.MAX_TOTAL_ORDER-14" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.MAX_TOTAL_ORDER" class="constant">MAX_TOTAL_ORDER</a>: <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a> = IntervalMonthDayNano::MAX

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#impl-ArrowNativeTypeOp-for-i256" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowNativeTypeOp.html" class="trait" title="trait arrow::array::ArrowNativeTypeOp">ArrowNativeTypeOp</a> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.ZERO-15" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.ZERO" class="constant">ZERO</a>: <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a> = i256::ZERO

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.ONE-15" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.ONE" class="constant">ONE</a>: <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a> = i256::ONE

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.MIN_TOTAL_ORDER-15" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.MIN_TOTAL_ORDER" class="constant">MIN_TOTAL_ORDER</a>: <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a> = i256::MIN

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.MAX_TOTAL_ORDER-15" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeTypeOp.html#associatedconstant.MAX_TOTAL_ORDER" class="constant">MAX_TOTAL_ORDER</a>: <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a> = i256::MAX
