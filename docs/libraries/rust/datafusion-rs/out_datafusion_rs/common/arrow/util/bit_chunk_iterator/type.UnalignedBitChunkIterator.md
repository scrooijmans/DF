# Type Alias UnalignedBitChunkIterator Copy item path

<a href="https://docs.rs/arrow-buffer/56.0.0/x86_64-unknown-linux-gnu/src/arrow_buffer/util/bit_chunk_iterator.rs.html#174" class="src">Source</a>

``` rust
pub type UnalignedBitChunkIterator<'a> = Chain<Chain<IntoIter<u64>, Cloned<Iter<'a, u64>>>, IntoIter<u64>>;
```

Expand description

Iterator over an [`UnalignedBitChunk`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/util/bit_chunk_iterator/struct.UnalignedBitChunk.html "struct datafusion::common::arrow::util::bit_chunk_iterator::UnalignedBitChunk")

## Aliased Type<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/util/bit_chunk_iterator/type.UnalignedBitChunkIterator.html#aliased-type" class="anchor">§</a>

``` rust
pub struct UnalignedBitChunkIterator<'a> { /* private fields */ }
```
