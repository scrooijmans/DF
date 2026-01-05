# Trait ChunkNestingUtils Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/chunked_array/ops/nesting_utils.rs.html#13" class="src">Source</a>

``` rust
pub trait ChunkNestingUtils: Sized {
    // Required methods
    fn propagate_nulls(&self) -> Option<Self>;
    fn trim_lists_to_normalized_offsets(&self) -> Option<Self>;
    fn find_validity_mismatch(&self, other: &Series, idxs: &mut Vec<u32>);
}
```

Expand description

Utility methods for dealing with nested chunked arrays.

## Required Methods<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkNestingUtils.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkNestingUtils.html#tymethod.propagate_nulls" class="fn">propagate_nulls</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<Self\>

Propagate nulls of nested datatype to all levels of nesting.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkNestingUtils.html#tymethod.trim_lists_to_normalized_offsets" class="fn">trim_lists_to_normalized_offsets</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<Self\>

Trim all lists of unused start and end elements recursively.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkNestingUtils.html#tymethod.find_validity_mismatch" class="fn">find_validity_mismatch</a>(&self, other: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, idxs: &mut <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>)

Find the indices of the values where the validity mismatches.

This is done recursively.

## Dyn Compatibility<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkNestingUtils.html#dyn-compatibility" class="anchor">§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementors<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkNestingUtils.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkNestingUtils.html#impl-ChunkNestingUtils-for-ChunkedArray%3CFixedSizeListType%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkNestingUtils.html" class="trait" title="trait polars::prelude::ChunkNestingUtils">ChunkNestingUtils</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.FixedSizeListType.html" class="struct" title="struct polars::prelude::FixedSizeListType">FixedSizeListType</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkNestingUtils.html#impl-ChunkNestingUtils-for-ChunkedArray%3CListType%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkNestingUtils.html" class="trait" title="trait polars::prelude::ChunkNestingUtils">ChunkNestingUtils</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ListType.html" class="struct" title="struct polars::prelude::ListType">ListType</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkNestingUtils.html#impl-ChunkNestingUtils-for-ChunkedArray%3CStructType%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkNestingUtils.html" class="trait" title="trait polars::prelude::ChunkNestingUtils">ChunkNestingUtils</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructType.html" class="struct" title="struct polars::prelude::StructType">StructType</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkNestingUtils.html#impl-ChunkNestingUtils-for-NullChunked" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkNestingUtils.html" class="trait" title="trait polars::prelude::ChunkNestingUtils">ChunkNestingUtils</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html" class="struct" title="struct polars::prelude::NullChunked">NullChunked</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkNestingUtils.html#impl-ChunkNestingUtils-for-ChunkedArray%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkNestingUtils.html" class="trait" title="trait polars::prelude::ChunkNestingUtils">ChunkNestingUtils</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<T\>

where T: <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html" class="trait" title="trait polars::prelude::PolarsDataType">PolarsDataType</a>\<IsNested = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>\>,
