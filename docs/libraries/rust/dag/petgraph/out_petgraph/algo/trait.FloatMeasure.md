# Trait FloatMeasure Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/algo/mod.rs.html#616-621" class="src">Source</a>

``` rust
pub trait FloatMeasure: Measure + Copy {
    // Required methods
    fn zero() -> Self;
    fn infinite() -> Self;
    fn from_f32(val: f32) -> Self;
    fn from_f64(val: f64) -> Self;
}
```

Expand description

A floating-point measure.

## Required Methods<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.FloatMeasure.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.FloatMeasure.html#tymethod.zero" class="fn">zero</a>() -\> Self

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.FloatMeasure.html#tymethod.infinite" class="fn">infinite</a>() -\> Self

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.FloatMeasure.html#tymethod.from_f32" class="fn">from_f32</a>(val: <a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>) -\> Self

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.FloatMeasure.html#tymethod.from_f64" class="fn">from_f64</a>(val: <a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>) -\> Self

## Dyn Compatibility<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.FloatMeasure.html#dyn-compatibility" class="anchor">§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementations on Foreign Types<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.FloatMeasure.html#foreign-impls" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.FloatMeasure.html#impl-FloatMeasure-for-f32" class="anchor">§</a>

### impl <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.FloatMeasure.html" class="trait" title="trait petgraph::algo::FloatMeasure">FloatMeasure</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.FloatMeasure.html#method.zero" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.FloatMeasure.html#tymethod.zero" class="fn">zero</a>() -\> Self

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.FloatMeasure.html#method.infinite" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.FloatMeasure.html#tymethod.infinite" class="fn">infinite</a>() -\> Self

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.FloatMeasure.html#method.from_f32" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.FloatMeasure.html#tymethod.from_f32" class="fn">from_f32</a>(val: <a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>) -\> Self

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.FloatMeasure.html#method.from_f64" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.FloatMeasure.html#tymethod.from_f64" class="fn">from_f64</a>(val: <a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>) -\> Self

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.FloatMeasure.html#impl-FloatMeasure-for-f64" class="anchor">§</a>

### impl <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.FloatMeasure.html" class="trait" title="trait petgraph::algo::FloatMeasure">FloatMeasure</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.FloatMeasure.html#method.zero-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.FloatMeasure.html#tymethod.zero" class="fn">zero</a>() -\> Self

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.FloatMeasure.html#method.infinite-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.FloatMeasure.html#tymethod.infinite" class="fn">infinite</a>() -\> Self

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.FloatMeasure.html#method.from_f32-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.FloatMeasure.html#tymethod.from_f32" class="fn">from_f32</a>(val: <a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>) -\> Self

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.FloatMeasure.html#method.from_f64-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.FloatMeasure.html#tymethod.from_f64" class="fn">from_f64</a>(val: <a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>) -\> Self

## Implementors<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.FloatMeasure.html#implementors" class="anchor">§</a>
