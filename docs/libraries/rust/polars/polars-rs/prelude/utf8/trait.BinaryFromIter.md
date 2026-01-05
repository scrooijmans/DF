# Trait BinaryFromIter Copy item path

<a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/src/polars_arrow/legacy/array/utf8.rs.html#77" class="src">Source</a>

``` rust
pub trait BinaryFromIter {
    // Provided method
    fn from_values_iter<I, S>(
        iter: I,
        len: usize,
        value_cap: usize,
    ) -> BinaryArray<i64>
       where S: AsRef<[u8]>,
             I: Iterator<Item = S> { ... }
}
```

## Provided Methods<a href="https://docs.rs/polars/latest/polars/prelude/utf8/trait.BinaryFromIter.html#provided-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/utf8/trait.BinaryFromIter.html#method.from_values_iter" class="fn">from_values_iter</a>\<I, S\>( iter: I, len: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, value_cap: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/binary/struct.BinaryArray.html" class="struct" title="struct polars_arrow::array::binary::BinaryArray">BinaryArray</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>

where S: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\>, I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html" class="trait" title="trait core::iter::traits::iterator::Iterator">Iterator</a>\<Item = S\>,

## Dyn Compatibility<a href="https://docs.rs/polars/latest/polars/prelude/utf8/trait.BinaryFromIter.html#dyn-compatibility" class="anchor">§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementors<a href="https://docs.rs/polars/latest/polars/prelude/utf8/trait.BinaryFromIter.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/utf8/trait.BinaryFromIter.html#impl-BinaryFromIter-for-BinaryArray%3Ci64%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/utf8/trait.BinaryFromIter.html" class="trait" title="trait polars::prelude::utf8::BinaryFromIter">BinaryFromIter</a> for <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/binary/struct.BinaryArray.html" class="struct" title="struct polars_arrow::array::binary::BinaryArray">BinaryArray</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>
