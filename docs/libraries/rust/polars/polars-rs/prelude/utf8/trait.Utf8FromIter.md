# Trait Utf8FromIter Copy item path

<a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/src/polars_arrow/legacy/array/utf8.rs.html#55" class="src">Source</a>

``` rust
pub trait Utf8FromIter {
    // Provided method
    fn from_values_iter<I, S>(
        iter: I,
        len: usize,
        size_hint: usize,
    ) -> Utf8Array<i64>
       where S: AsRef<str>,
             I: Iterator<Item = S> { ... }
}
```

## Provided Methods<a href="https://docs.rs/polars/latest/polars/prelude/utf8/trait.Utf8FromIter.html#provided-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/utf8/trait.Utf8FromIter.html#method.from_values_iter" class="fn">from_values_iter</a>\<I, S\>( iter: I, len: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, size_hint: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/utf8/struct.Utf8Array.html" class="struct" title="struct polars_arrow::array::utf8::Utf8Array">Utf8Array</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>

where S: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>, I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html" class="trait" title="trait core::iter::traits::iterator::Iterator">Iterator</a>\<Item = S\>,

## Dyn Compatibility<a href="https://docs.rs/polars/latest/polars/prelude/utf8/trait.Utf8FromIter.html#dyn-compatibility" class="anchor">§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementors<a href="https://docs.rs/polars/latest/polars/prelude/utf8/trait.Utf8FromIter.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/utf8/trait.Utf8FromIter.html#impl-Utf8FromIter-for-Utf8Array%3Ci64%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/utf8/trait.Utf8FromIter.html" class="trait" title="trait polars::prelude::utf8::Utf8FromIter">Utf8FromIter</a> for <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/utf8/struct.Utf8Array.html" class="struct" title="struct polars_arrow::array::utf8::Utf8Array">Utf8Array</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>
