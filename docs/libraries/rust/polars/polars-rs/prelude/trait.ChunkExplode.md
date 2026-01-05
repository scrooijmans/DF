# Trait ChunkExplode Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/chunked_array/ops/mod.rs.html#85" class="src">Source</a>

``` rust
pub trait ChunkExplode {
    // Required methods
    fn offsets(&self) -> Result<OffsetsBuffer<i64>, PolarsError>;
    fn explode_and_offsets(
        &self,
        skip_empty: bool,
    ) -> Result<(Series, OffsetsBuffer<i64>), PolarsError>;

    // Provided method
    fn explode(&self, skip_empty: bool) -> Result<Series, PolarsError> { ... }
}
```

Expand description

Explode/flatten a List or String Series

## Required Methods<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkExplode.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkExplode.html#tymethod.offsets" class="fn">offsets</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/offset/struct.OffsetsBuffer.html" class="struct" title="struct polars_arrow::offset::OffsetsBuffer">OffsetsBuffer</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkExplode.html#tymethod.explode_and_offsets" class="fn">explode_and_offsets</a>( &self, skip_empty: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<(<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/offset/struct.OffsetsBuffer.html" class="struct" title="struct polars_arrow::offset::OffsetsBuffer">OffsetsBuffer</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>), <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

## Provided Methods<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkExplode.html#provided-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkExplode.html#method.explode" class="fn">explode</a>(&self, skip_empty: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

## Implementors<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkExplode.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkExplode.html#impl-ChunkExplode-for-ChunkedArray%3CFixedSizeListType%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkExplode.html" class="trait" title="trait polars::prelude::ChunkExplode">ChunkExplode</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.FixedSizeListType.html" class="struct" title="struct polars::prelude::FixedSizeListType">FixedSizeListType</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkExplode.html#impl-ChunkExplode-for-ChunkedArray%3CListType%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkExplode.html" class="trait" title="trait polars::prelude::ChunkExplode">ChunkExplode</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ListType.html" class="struct" title="struct polars::prelude::ListType">ListType</a>\>
