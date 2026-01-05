# Trait Zero Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/matrix_graph.rs.html#151-157" class="src">Source</a>

``` rust
pub trait Zero {
    // Required methods
    fn zero() -> Self;
    fn is_zero(&self) -> bool;
}
```

Expand description

Base trait for types that can be wrapped in a [`NotZero`](https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.NotZero.html).

Implementors must provide a singleton object that will be used to mark empty edges in a [`MatrixGraph`](https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html).

Note that this trait is already implemented for the base numeric types.

## Required Methods<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Zero.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Zero.html#tymethod.zero" class="fn">zero</a>() -\> Self

Return the singleton object which can be used as a sentinel value.

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Zero.html#tymethod.is_zero" class="fn">is_zero</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Return true if `self` is equal to the sentinel value.

## Dyn Compatibility<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Zero.html#dyn-compatibility" class="anchor">§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementations on Foreign Types<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Zero.html#foreign-impls" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Zero.html#impl-Zero-for-f32" class="anchor">§</a>

### impl <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Zero.html" class="trait" title="trait petgraph::matrix_graph::Zero">Zero</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Zero.html#method.zero" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Zero.html#tymethod.zero" class="fn">zero</a>() -\> Self

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Zero.html#method.is_zero" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Zero.html#tymethod.is_zero" class="fn">is_zero</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Zero.html#impl-Zero-for-f64" class="anchor">§</a>

### impl <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Zero.html" class="trait" title="trait petgraph::matrix_graph::Zero">Zero</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Zero.html#method.zero-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Zero.html#tymethod.zero" class="fn">zero</a>() -\> Self

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Zero.html#method.is_zero-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Zero.html#tymethod.is_zero" class="fn">is_zero</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Zero.html#impl-Zero-for-i8" class="anchor">§</a>

### impl <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Zero.html" class="trait" title="trait petgraph::matrix_graph::Zero">Zero</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Zero.html#method.zero-2" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Zero.html#tymethod.zero" class="fn">zero</a>() -\> Self

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Zero.html#method.is_zero-2" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Zero.html#tymethod.is_zero" class="fn">is_zero</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Zero.html#impl-Zero-for-i16" class="anchor">§</a>

### impl <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Zero.html" class="trait" title="trait petgraph::matrix_graph::Zero">Zero</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Zero.html#method.zero-3" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Zero.html#tymethod.zero" class="fn">zero</a>() -\> Self

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Zero.html#method.is_zero-3" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Zero.html#tymethod.is_zero" class="fn">is_zero</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Zero.html#impl-Zero-for-i32" class="anchor">§</a>

### impl <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Zero.html" class="trait" title="trait petgraph::matrix_graph::Zero">Zero</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Zero.html#method.zero-4" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Zero.html#tymethod.zero" class="fn">zero</a>() -\> Self

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Zero.html#method.is_zero-4" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Zero.html#tymethod.is_zero" class="fn">is_zero</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Zero.html#impl-Zero-for-i64" class="anchor">§</a>

### impl <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Zero.html" class="trait" title="trait petgraph::matrix_graph::Zero">Zero</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Zero.html#method.zero-5" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Zero.html#tymethod.zero" class="fn">zero</a>() -\> Self

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Zero.html#method.is_zero-5" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Zero.html#tymethod.is_zero" class="fn">is_zero</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Zero.html#impl-Zero-for-isize" class="anchor">§</a>

### impl <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Zero.html" class="trait" title="trait petgraph::matrix_graph::Zero">Zero</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Zero.html#method.zero-6" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Zero.html#tymethod.zero" class="fn">zero</a>() -\> Self

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Zero.html#method.is_zero-6" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Zero.html#tymethod.is_zero" class="fn">is_zero</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Zero.html#impl-Zero-for-u8" class="anchor">§</a>

### impl <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Zero.html" class="trait" title="trait petgraph::matrix_graph::Zero">Zero</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Zero.html#method.zero-7" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Zero.html#tymethod.zero" class="fn">zero</a>() -\> Self

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Zero.html#method.is_zero-7" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Zero.html#tymethod.is_zero" class="fn">is_zero</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Zero.html#impl-Zero-for-u16" class="anchor">§</a>

### impl <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Zero.html" class="trait" title="trait petgraph::matrix_graph::Zero">Zero</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Zero.html#method.zero-8" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Zero.html#tymethod.zero" class="fn">zero</a>() -\> Self

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Zero.html#method.is_zero-8" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Zero.html#tymethod.is_zero" class="fn">is_zero</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Zero.html#impl-Zero-for-u32" class="anchor">§</a>

### impl <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Zero.html" class="trait" title="trait petgraph::matrix_graph::Zero">Zero</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Zero.html#method.zero-9" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Zero.html#tymethod.zero" class="fn">zero</a>() -\> Self

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Zero.html#method.is_zero-9" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Zero.html#tymethod.is_zero" class="fn">is_zero</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Zero.html#impl-Zero-for-u64" class="anchor">§</a>

### impl <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Zero.html" class="trait" title="trait petgraph::matrix_graph::Zero">Zero</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Zero.html#method.zero-10" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Zero.html#tymethod.zero" class="fn">zero</a>() -\> Self

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Zero.html#method.is_zero-10" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Zero.html#tymethod.is_zero" class="fn">is_zero</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Zero.html#impl-Zero-for-usize" class="anchor">§</a>

### impl <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Zero.html" class="trait" title="trait petgraph::matrix_graph::Zero">Zero</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Zero.html#method.zero-11" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Zero.html#tymethod.zero" class="fn">zero</a>() -\> Self

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Zero.html#method.is_zero-11" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Zero.html#tymethod.is_zero" class="fn">is_zero</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

## Implementors<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Zero.html#implementors" class="anchor">§</a>
