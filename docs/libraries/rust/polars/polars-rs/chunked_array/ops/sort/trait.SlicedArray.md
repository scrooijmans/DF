# Trait SlicedArray Copy item path

<a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/src/polars_arrow/legacy/array/slice.rs.html#5" class="src">Source</a>

``` rust
pub trait SlicedArray {
    // Required methods
    fn slice_typed(&self, offset: usize, length: usize) -> Self
       where Self: Sized;
    unsafe fn slice_typed_unchecked(&self, offset: usize, length: usize) -> Self
       where Self: Sized;
}
```

Expand description

Utility trait to slice concrete arrow arrays whilst keeping their concrete type. E.g. don’t return `Box<dyn Array>`.

## Required Methods<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/sort/trait.SlicedArray.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/chunked_array/ops/sort/trait.SlicedArray.html#tymethod.slice_typed" class="fn">slice_typed</a>(&self, offset: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, length: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> Self

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Slices this [`Array`](https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html "trait polars_arrow::array::Array").

##### <a href="https://docs.rs/polars/latest/polars/chunked_array/ops/sort/trait.SlicedArray.html#implementation" class="doc-anchor">§</a>Implementation

This operation is `O(1)` over `len`.

##### <a href="https://docs.rs/polars/latest/polars/chunked_array/ops/sort/trait.SlicedArray.html#panic" class="doc-anchor">§</a>Panic

This function panics iff `offset + length > self.len()`.

#### unsafe fn <a href="https://docs.rs/polars/latest/polars/chunked_array/ops/sort/trait.SlicedArray.html#tymethod.slice_typed_unchecked" class="fn">slice_typed_unchecked</a>(&self, offset: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, length: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> Self

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Slices the [`Array`](https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html "trait polars_arrow::array::Array").

##### <a href="https://docs.rs/polars/latest/polars/chunked_array/ops/sort/trait.SlicedArray.html#implementation-1" class="doc-anchor">§</a>Implementation

This operation is `O(1)`.

##### <a href="https://docs.rs/polars/latest/polars/chunked_array/ops/sort/trait.SlicedArray.html#safety" class="doc-anchor">§</a>Safety

The caller must ensure that `offset + length <= self.len()`

## Implementors<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/sort/trait.SlicedArray.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/sort/trait.SlicedArray.html#impl-SlicedArray-for-T" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.SlicedArray.html" class="trait" title="trait polars::prelude::SlicedArray">SlicedArray</a> for T

where T: <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html" class="trait" title="trait polars_arrow::array::Array">Array</a> + <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a>,
