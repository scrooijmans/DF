# Trait UnitMeasure Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/algo/mod.rs.html#729-742" class="src">Source</a>

``` rust
pub trait UnitMeasure:
    Measure
    + Sub<Self, Output = Self>
    + Mul<Self, Output = Self>
    + Div<Self, Output = Self>
    + Sum {
    // Required methods
    fn zero() -> Self;
    fn one() -> Self;
    fn from_usize(nb: usize) -> Self;
    fn default_tol() -> Self;
    fn from_f32(val: f32) -> Self;
    fn from_f64(val: f64) -> Self;
}
```

Expand description

A floating-point measure that can be computed from `usize` and with a default measure of proximity.

## Required Methods<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.UnitMeasure.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.UnitMeasure.html#tymethod.zero" class="fn">zero</a>() -\> Self

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.UnitMeasure.html#tymethod.one" class="fn">one</a>() -\> Self

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.UnitMeasure.html#tymethod.from_usize" class="fn">from_usize</a>(nb: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> Self

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.UnitMeasure.html#tymethod.default_tol" class="fn">default_tol</a>() -\> Self

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.UnitMeasure.html#tymethod.from_f32" class="fn">from_f32</a>(val: <a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>) -\> Self

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.UnitMeasure.html#tymethod.from_f64" class="fn">from_f64</a>(val: <a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>) -\> Self

## Dyn Compatibility<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.UnitMeasure.html#dyn-compatibility" class="anchor">§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementations on Foreign Types<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.UnitMeasure.html#foreign-impls" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.UnitMeasure.html#impl-UnitMeasure-for-f32" class="anchor">§</a>

### impl <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.UnitMeasure.html" class="trait" title="trait petgraph::algo::UnitMeasure">UnitMeasure</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.UnitMeasure.html#method.zero" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.UnitMeasure.html#tymethod.zero" class="fn">zero</a>() -\> Self

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.UnitMeasure.html#method.one" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.UnitMeasure.html#tymethod.one" class="fn">one</a>() -\> Self

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.UnitMeasure.html#method.from_usize" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.UnitMeasure.html#tymethod.from_usize" class="fn">from_usize</a>(nb: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> Self

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.UnitMeasure.html#method.default_tol" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.UnitMeasure.html#tymethod.default_tol" class="fn">default_tol</a>() -\> Self

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.UnitMeasure.html#method.from_f32" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.UnitMeasure.html#tymethod.from_f32" class="fn">from_f32</a>(val: <a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>) -\> Self

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.UnitMeasure.html#method.from_f64" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.UnitMeasure.html#tymethod.from_f64" class="fn">from_f64</a>(val: <a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>) -\> Self

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.UnitMeasure.html#impl-UnitMeasure-for-f64" class="anchor">§</a>

### impl <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.UnitMeasure.html" class="trait" title="trait petgraph::algo::UnitMeasure">UnitMeasure</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.UnitMeasure.html#method.zero-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.UnitMeasure.html#tymethod.zero" class="fn">zero</a>() -\> Self

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.UnitMeasure.html#method.one-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.UnitMeasure.html#tymethod.one" class="fn">one</a>() -\> Self

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.UnitMeasure.html#method.from_usize-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.UnitMeasure.html#tymethod.from_usize" class="fn">from_usize</a>(nb: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> Self

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.UnitMeasure.html#method.default_tol-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.UnitMeasure.html#tymethod.default_tol" class="fn">default_tol</a>() -\> Self

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.UnitMeasure.html#method.from_f32-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.UnitMeasure.html#tymethod.from_f32" class="fn">from_f32</a>(val: <a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>) -\> Self

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.UnitMeasure.html#method.from_f64-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.UnitMeasure.html#tymethod.from_f64" class="fn">from_f64</a>(val: <a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>) -\> Self

## Implementors<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.UnitMeasure.html#implementors" class="anchor">§</a>
