# Trait TakeChunked Copy item path

<a href="https://docs.rs/polars-ops/0.51.0/x86_64-unknown-linux-gnu/src/polars_ops/chunked_array/gather/chunked.rs.html#18" class="src">Source</a>

``` rust
pub trait TakeChunked {
    // Required methods
    unsafe fn take_chunked_unchecked<const B: u64>(
        &self,
        by: &[ChunkId<B>],
        sorted: IsSorted,
        avoid_sharing: bool,
    ) -> Self;
    unsafe fn take_opt_chunked_unchecked<const B: u64>(
        &self,
        by: &[ChunkId<B>],
        avoid_sharing: bool,
    ) -> Self;
}
```

Available on **crate feature `polars-ops`** only.

Expand description

Gather by [`ChunkId`](https://docs.rs/polars/latest/polars/prelude/struct.ChunkId.html "struct polars::prelude::ChunkId")

## Required Methods<a href="https://docs.rs/polars/latest/polars/prelude/trait.TakeChunked.html#required-methods" class="anchor">§</a>

#### unsafe fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.TakeChunked.html#tymethod.take_chunked_unchecked" class="fn">take_chunked_unchecked</a>\<const B: <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>( &self, by: &\[<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkId.html" class="struct" title="struct polars::prelude::ChunkId">ChunkId</a>\<B\>\], sorted: <a href="https://docs.rs/polars/latest/polars/series/enum.IsSorted.html" class="enum" title="enum polars::series::IsSorted">IsSorted</a>, avoid_sharing: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, ) -\> Self

Gathers elements from a ChunkedArray, specifying for each element a chunk index and index within that chunk through ChunkId. If avoid_sharing is true the returned data should not share references with the original array (like shared buffers in views).

##### <a href="https://docs.rs/polars/latest/polars/prelude/trait.TakeChunked.html#safety" class="doc-anchor">§</a>Safety

This function doesn’t do any bound checks.

#### unsafe fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.TakeChunked.html#tymethod.take_opt_chunked_unchecked" class="fn">take_opt_chunked_unchecked</a>\<const B: <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>( &self, by: &\[<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkId.html" class="struct" title="struct polars::prelude::ChunkId">ChunkId</a>\<B\>\], avoid_sharing: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, ) -\> Self

##### <a href="https://docs.rs/polars/latest/polars/prelude/trait.TakeChunked.html#safety-1" class="doc-anchor">§</a>Safety

This function doesn’t do any bound checks.

## Dyn Compatibility<a href="https://docs.rs/polars/latest/polars/prelude/trait.TakeChunked.html#dyn-compatibility" class="anchor">§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementors<a href="https://docs.rs/polars/latest/polars/prelude/trait.TakeChunked.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.TakeChunked.html#impl-TakeChunked-for-Column" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.TakeChunked.html" class="trait" title="trait polars::prelude::TakeChunked">TakeChunked</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.Column.html" class="enum" title="enum polars::prelude::Column">Column</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.TakeChunked.html#impl-TakeChunked-for-DataFrame" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.TakeChunked.html" class="trait" title="trait polars::prelude::TakeChunked">TakeChunked</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.TakeChunked.html#impl-TakeChunked-for-Series" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.TakeChunked.html" class="trait" title="trait polars::prelude::TakeChunked">TakeChunked</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.TakeChunked.html#impl-TakeChunked-for-ChunkedArray%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.TakeChunked.html" class="trait" title="trait polars::prelude::TakeChunked">TakeChunked</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<T\>

where T: <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html" class="trait" title="trait polars::prelude::PolarsDataType">PolarsDataType</a>, \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html" class="trait" title="trait polars::prelude::PolarsDataType">PolarsDataType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html#associatedtype.Array" class="associatedtype" title="type polars::prelude::PolarsDataType::Array">Array</a>: <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a>,
