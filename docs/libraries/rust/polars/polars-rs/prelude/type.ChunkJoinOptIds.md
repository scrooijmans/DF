# Type Alias ChunkJoinOptIds Copy item path

<a href="https://docs.rs/polars-ops/0.51.0/x86_64-unknown-linux-gnu/src/polars_ops/frame/join/args.rs.html#10" class="src">Source</a>

``` rust
pub type ChunkJoinOptIds = Either<Vec<NullableIdxSize>, Vec<ChunkId>>;
```

Available on **crate feature `polars-ops`** only.

## Aliased Type<a href="https://docs.rs/polars/latest/polars/prelude/type.ChunkJoinOptIds.html#aliased-type" class="anchor">§</a>

``` rust
pub enum ChunkJoinOptIds {
    Left(Vec<NullableIdxSize>),
    Right(Vec<ChunkId<24>>),
}
```

## Variants<a href="https://docs.rs/polars/latest/polars/prelude/type.ChunkJoinOptIds.html#variants" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/type.ChunkJoinOptIds.html#variant.Left" class="anchor">§</a>

### Left(<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullableIdxSize.html" class="struct" title="struct polars::prelude::NullableIdxSize">NullableIdxSize</a>\>)

A value of type `L`.

<a href="https://docs.rs/polars/latest/polars/prelude/type.ChunkJoinOptIds.html#variant.Right" class="anchor">§</a>

### Right(<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkId.html" class="struct" title="struct polars::prelude::ChunkId">ChunkId</a>\<24\>\>)

A value of type `R`.
