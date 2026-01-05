# Trait ChunkBytes Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/chunked_array/ops/mod.rs.html#93" class="src">Source</a>

``` rust
pub trait ChunkBytes {
    // Required method
    fn to_byte_slices(&self) -> Vec<&[u8]>;
}
```

## Required Methods<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkBytes.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkBytes.html#tymethod.to_byte_slices" class="fn">to_byte_slices</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<&\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\>

## Implementors<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkBytes.html#implementors" class="anchor">§</a>
