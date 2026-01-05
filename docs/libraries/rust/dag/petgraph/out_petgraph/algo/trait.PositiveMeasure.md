# Trait PositiveMeasure Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/algo/mod.rs.html#779-782" class="src">Source</a>

``` rust
pub trait PositiveMeasure: Measure + Copy {
    // Required methods
    fn zero() -> Self;
    fn max() -> Self;
}
```

Expand description

Some measure of positive numbers, assuming positive float-pointing numbers

## Required Methods<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.PositiveMeasure.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.PositiveMeasure.html#tymethod.zero" class="fn">zero</a>() -\> Self

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.PositiveMeasure.html#tymethod.max" class="fn">max</a>() -\> Self

## Dyn Compatibility<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.PositiveMeasure.html#dyn-compatibility" class="anchor">§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementations on Foreign Types<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.PositiveMeasure.html#foreign-impls" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.PositiveMeasure.html#impl-PositiveMeasure-for-f32" class="anchor">§</a>

### impl <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.PositiveMeasure.html" class="trait" title="trait petgraph::algo::PositiveMeasure">PositiveMeasure</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.PositiveMeasure.html#method.zero" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.PositiveMeasure.html#tymethod.zero" class="fn">zero</a>() -\> Self

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.PositiveMeasure.html#method.max" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.PositiveMeasure.html#tymethod.max" class="fn">max</a>() -\> Self

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.PositiveMeasure.html#impl-PositiveMeasure-for-f64" class="anchor">§</a>

### impl <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.PositiveMeasure.html" class="trait" title="trait petgraph::algo::PositiveMeasure">PositiveMeasure</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.PositiveMeasure.html#method.zero-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.PositiveMeasure.html#tymethod.zero" class="fn">zero</a>() -\> Self

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.PositiveMeasure.html#method.max-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.PositiveMeasure.html#tymethod.max" class="fn">max</a>() -\> Self

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.PositiveMeasure.html#impl-PositiveMeasure-for-u8" class="anchor">§</a>

### impl <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.PositiveMeasure.html" class="trait" title="trait petgraph::algo::PositiveMeasure">PositiveMeasure</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.PositiveMeasure.html#method.zero-2" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.PositiveMeasure.html#tymethod.zero" class="fn">zero</a>() -\> Self

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.PositiveMeasure.html#method.max-2" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.PositiveMeasure.html#tymethod.max" class="fn">max</a>() -\> Self

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.PositiveMeasure.html#impl-PositiveMeasure-for-u16" class="anchor">§</a>

### impl <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.PositiveMeasure.html" class="trait" title="trait petgraph::algo::PositiveMeasure">PositiveMeasure</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.PositiveMeasure.html#method.zero-3" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.PositiveMeasure.html#tymethod.zero" class="fn">zero</a>() -\> Self

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.PositiveMeasure.html#method.max-3" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.PositiveMeasure.html#tymethod.max" class="fn">max</a>() -\> Self

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.PositiveMeasure.html#impl-PositiveMeasure-for-u32" class="anchor">§</a>

### impl <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.PositiveMeasure.html" class="trait" title="trait petgraph::algo::PositiveMeasure">PositiveMeasure</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.PositiveMeasure.html#method.zero-4" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.PositiveMeasure.html#tymethod.zero" class="fn">zero</a>() -\> Self

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.PositiveMeasure.html#method.max-4" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.PositiveMeasure.html#tymethod.max" class="fn">max</a>() -\> Self

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.PositiveMeasure.html#impl-PositiveMeasure-for-u64" class="anchor">§</a>

### impl <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.PositiveMeasure.html" class="trait" title="trait petgraph::algo::PositiveMeasure">PositiveMeasure</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.PositiveMeasure.html#method.zero-5" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.PositiveMeasure.html#tymethod.zero" class="fn">zero</a>() -\> Self

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.PositiveMeasure.html#method.max-5" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.PositiveMeasure.html#tymethod.max" class="fn">max</a>() -\> Self

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.PositiveMeasure.html#impl-PositiveMeasure-for-u128" class="anchor">§</a>

### impl <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.PositiveMeasure.html" class="trait" title="trait petgraph::algo::PositiveMeasure">PositiveMeasure</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.PositiveMeasure.html#method.zero-6" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.PositiveMeasure.html#tymethod.zero" class="fn">zero</a>() -\> Self

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.PositiveMeasure.html#method.max-6" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.PositiveMeasure.html#tymethod.max" class="fn">max</a>() -\> Self

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.PositiveMeasure.html#impl-PositiveMeasure-for-usize" class="anchor">§</a>

### impl <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.PositiveMeasure.html" class="trait" title="trait petgraph::algo::PositiveMeasure">PositiveMeasure</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.PositiveMeasure.html#method.zero-7" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.PositiveMeasure.html#tymethod.zero" class="fn">zero</a>() -\> Self

<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.PositiveMeasure.html#method.max-7" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.PositiveMeasure.html#tymethod.max" class="fn">max</a>() -\> Self

## Implementors<a href="https://docs.rs/petgraph/latest/petgraph/algo/trait.PositiveMeasure.html#implementors" class="anchor">§</a>
