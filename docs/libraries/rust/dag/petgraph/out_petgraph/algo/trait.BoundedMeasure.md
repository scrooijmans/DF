# Trait BoundedMeasure Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/algo/mod.rs.html#653-659" class="src">Source</a>

``` rust
pub trait BoundedMeasure: Measure + Sub<Self, Output = Self> {
    // Required methods
    fn min() -> Self;
    fn max() -> Self;
    fn overflowing_add(self, rhs: Self) -> (Self, bool);
    fn from_f32(val: f32) -> Self;
    fn from_f64(val: f64) -> Self;
}
```

## Required Methods<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#tymethod.min" class="fn">min</a>() -\> Self

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#tymethod.max" class="fn">max</a>() -\> Self

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#tymethod.overflowing_add" class="fn">overflowing_add</a>(self, rhs: Self) -\> (Self, <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>)

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#tymethod.from_f32" class="fn">from_f32</a>(val: <a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>) -\> Self

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#tymethod.from_f64" class="fn">from_f64</a>(val: <a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>) -\> Self

## Dyn Compatibility<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#dyn-compatibility" class="anchor">§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementations on Foreign Types<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#foreign-impls" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#impl-BoundedMeasure-for-f32" class="anchor">§</a>

### impl <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html" class="trait" title="trait petgraph::algo::BoundedMeasure">BoundedMeasure</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#method.min" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#tymethod.min" class="fn">min</a>() -\> Self

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#method.max" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#tymethod.max" class="fn">max</a>() -\> Self

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#method.overflowing_add" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#tymethod.overflowing_add" class="fn">overflowing_add</a>(self, rhs: Self) -\> (Self, <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>)

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#method.from_f32" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#tymethod.from_f32" class="fn">from_f32</a>(val: <a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>) -\> Self

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#method.from_f64" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#tymethod.from_f64" class="fn">from_f64</a>(val: <a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>) -\> Self

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#impl-BoundedMeasure-for-f64" class="anchor">§</a>

### impl <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html" class="trait" title="trait petgraph::algo::BoundedMeasure">BoundedMeasure</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#method.min-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#tymethod.min" class="fn">min</a>() -\> Self

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#method.max-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#tymethod.max" class="fn">max</a>() -\> Self

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#method.overflowing_add-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#tymethod.overflowing_add" class="fn">overflowing_add</a>(self, rhs: Self) -\> (Self, <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>)

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#method.from_f32-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#tymethod.from_f32" class="fn">from_f32</a>(val: <a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>) -\> Self

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#method.from_f64-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#tymethod.from_f64" class="fn">from_f64</a>(val: <a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>) -\> Self

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#impl-BoundedMeasure-for-i8" class="anchor">§</a>

### impl <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html" class="trait" title="trait petgraph::algo::BoundedMeasure">BoundedMeasure</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#method.min-2" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#tymethod.min" class="fn">min</a>() -\> Self

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#method.max-2" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#tymethod.max" class="fn">max</a>() -\> Self

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#method.overflowing_add-2" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#tymethod.overflowing_add" class="fn">overflowing_add</a>(self, rhs: Self) -\> (Self, <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>)

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#method.from_f32-2" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#tymethod.from_f32" class="fn">from_f32</a>(val: <a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>) -\> Self

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#method.from_f64-2" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#tymethod.from_f64" class="fn">from_f64</a>(val: <a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>) -\> Self

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#impl-BoundedMeasure-for-i16" class="anchor">§</a>

### impl <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html" class="trait" title="trait petgraph::algo::BoundedMeasure">BoundedMeasure</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#method.min-3" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#tymethod.min" class="fn">min</a>() -\> Self

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#method.max-3" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#tymethod.max" class="fn">max</a>() -\> Self

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#method.overflowing_add-3" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#tymethod.overflowing_add" class="fn">overflowing_add</a>(self, rhs: Self) -\> (Self, <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>)

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#method.from_f32-3" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#tymethod.from_f32" class="fn">from_f32</a>(val: <a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>) -\> Self

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#method.from_f64-3" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#tymethod.from_f64" class="fn">from_f64</a>(val: <a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>) -\> Self

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#impl-BoundedMeasure-for-i32" class="anchor">§</a>

### impl <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html" class="trait" title="trait petgraph::algo::BoundedMeasure">BoundedMeasure</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#method.min-4" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#tymethod.min" class="fn">min</a>() -\> Self

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#method.max-4" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#tymethod.max" class="fn">max</a>() -\> Self

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#method.overflowing_add-4" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#tymethod.overflowing_add" class="fn">overflowing_add</a>(self, rhs: Self) -\> (Self, <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>)

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#method.from_f32-4" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#tymethod.from_f32" class="fn">from_f32</a>(val: <a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>) -\> Self

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#method.from_f64-4" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#tymethod.from_f64" class="fn">from_f64</a>(val: <a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>) -\> Self

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#impl-BoundedMeasure-for-i64" class="anchor">§</a>

### impl <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html" class="trait" title="trait petgraph::algo::BoundedMeasure">BoundedMeasure</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#method.min-5" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#tymethod.min" class="fn">min</a>() -\> Self

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#method.max-5" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#tymethod.max" class="fn">max</a>() -\> Self

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#method.overflowing_add-5" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#tymethod.overflowing_add" class="fn">overflowing_add</a>(self, rhs: Self) -\> (Self, <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>)

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#method.from_f32-5" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#tymethod.from_f32" class="fn">from_f32</a>(val: <a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>) -\> Self

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#method.from_f64-5" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#tymethod.from_f64" class="fn">from_f64</a>(val: <a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>) -\> Self

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#impl-BoundedMeasure-for-i128" class="anchor">§</a>

### impl <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html" class="trait" title="trait petgraph::algo::BoundedMeasure">BoundedMeasure</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#method.min-6" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#tymethod.min" class="fn">min</a>() -\> Self

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#method.max-6" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#tymethod.max" class="fn">max</a>() -\> Self

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#method.overflowing_add-6" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#tymethod.overflowing_add" class="fn">overflowing_add</a>(self, rhs: Self) -\> (Self, <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>)

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#method.from_f32-6" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#tymethod.from_f32" class="fn">from_f32</a>(val: <a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>) -\> Self

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#method.from_f64-6" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#tymethod.from_f64" class="fn">from_f64</a>(val: <a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>) -\> Self

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#impl-BoundedMeasure-for-isize" class="anchor">§</a>

### impl <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html" class="trait" title="trait petgraph::algo::BoundedMeasure">BoundedMeasure</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#method.min-7" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#tymethod.min" class="fn">min</a>() -\> Self

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#method.max-7" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#tymethod.max" class="fn">max</a>() -\> Self

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#method.overflowing_add-7" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#tymethod.overflowing_add" class="fn">overflowing_add</a>(self, rhs: Self) -\> (Self, <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>)

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#method.from_f32-7" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#tymethod.from_f32" class="fn">from_f32</a>(val: <a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>) -\> Self

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#method.from_f64-7" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#tymethod.from_f64" class="fn">from_f64</a>(val: <a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>) -\> Self

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#impl-BoundedMeasure-for-u8" class="anchor">§</a>

### impl <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html" class="trait" title="trait petgraph::algo::BoundedMeasure">BoundedMeasure</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#method.min-8" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#tymethod.min" class="fn">min</a>() -\> Self

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#method.max-8" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#tymethod.max" class="fn">max</a>() -\> Self

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#method.overflowing_add-8" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#tymethod.overflowing_add" class="fn">overflowing_add</a>(self, rhs: Self) -\> (Self, <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>)

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#method.from_f32-8" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#tymethod.from_f32" class="fn">from_f32</a>(val: <a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>) -\> Self

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#method.from_f64-8" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#tymethod.from_f64" class="fn">from_f64</a>(val: <a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>) -\> Self

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#impl-BoundedMeasure-for-u16" class="anchor">§</a>

### impl <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html" class="trait" title="trait petgraph::algo::BoundedMeasure">BoundedMeasure</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#method.min-9" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#tymethod.min" class="fn">min</a>() -\> Self

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#method.max-9" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#tymethod.max" class="fn">max</a>() -\> Self

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#method.overflowing_add-9" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#tymethod.overflowing_add" class="fn">overflowing_add</a>(self, rhs: Self) -\> (Self, <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>)

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#method.from_f32-9" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#tymethod.from_f32" class="fn">from_f32</a>(val: <a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>) -\> Self

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#method.from_f64-9" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#tymethod.from_f64" class="fn">from_f64</a>(val: <a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>) -\> Self

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#impl-BoundedMeasure-for-u32" class="anchor">§</a>

### impl <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html" class="trait" title="trait petgraph::algo::BoundedMeasure">BoundedMeasure</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#method.min-10" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#tymethod.min" class="fn">min</a>() -\> Self

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#method.max-10" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#tymethod.max" class="fn">max</a>() -\> Self

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#method.overflowing_add-10" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#tymethod.overflowing_add" class="fn">overflowing_add</a>(self, rhs: Self) -\> (Self, <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>)

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#method.from_f32-10" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#tymethod.from_f32" class="fn">from_f32</a>(val: <a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>) -\> Self

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#method.from_f64-10" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#tymethod.from_f64" class="fn">from_f64</a>(val: <a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>) -\> Self

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#impl-BoundedMeasure-for-u64" class="anchor">§</a>

### impl <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html" class="trait" title="trait petgraph::algo::BoundedMeasure">BoundedMeasure</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#method.min-11" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#tymethod.min" class="fn">min</a>() -\> Self

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#method.max-11" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#tymethod.max" class="fn">max</a>() -\> Self

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#method.overflowing_add-11" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#tymethod.overflowing_add" class="fn">overflowing_add</a>(self, rhs: Self) -\> (Self, <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>)

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#method.from_f32-11" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#tymethod.from_f32" class="fn">from_f32</a>(val: <a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>) -\> Self

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#method.from_f64-11" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#tymethod.from_f64" class="fn">from_f64</a>(val: <a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>) -\> Self

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#impl-BoundedMeasure-for-u128" class="anchor">§</a>

### impl <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html" class="trait" title="trait petgraph::algo::BoundedMeasure">BoundedMeasure</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#method.min-12" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#tymethod.min" class="fn">min</a>() -\> Self

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#method.max-12" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#tymethod.max" class="fn">max</a>() -\> Self

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#method.overflowing_add-12" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#tymethod.overflowing_add" class="fn">overflowing_add</a>(self, rhs: Self) -\> (Self, <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>)

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#method.from_f32-12" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#tymethod.from_f32" class="fn">from_f32</a>(val: <a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>) -\> Self

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#method.from_f64-12" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#tymethod.from_f64" class="fn">from_f64</a>(val: <a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>) -\> Self

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#impl-BoundedMeasure-for-usize" class="anchor">§</a>

### impl <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html" class="trait" title="trait petgraph::algo::BoundedMeasure">BoundedMeasure</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#method.min-13" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#tymethod.min" class="fn">min</a>() -\> Self

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#method.max-13" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#tymethod.max" class="fn">max</a>() -\> Self

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#method.overflowing_add-13" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#tymethod.overflowing_add" class="fn">overflowing_add</a>(self, rhs: Self) -\> (Self, <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>)

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#method.from_f32-13" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#tymethod.from_f32" class="fn">from_f32</a>(val: <a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>) -\> Self

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#method.from_f64-13" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#tymethod.from_f64" class="fn">from_f64</a>(val: <a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>) -\> Self

## Implementors<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.BoundedMeasure.html#implementors" class="anchor">§</a>
