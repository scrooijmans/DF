# Function private_left_join_multiple_keys Copy item path

<a href="https://docs.rs/polars-ops/0.51.0/x86_64-unknown-linux-gnu/src/polars_ops/frame/join/mod.rs.html#638-642" class="src">Source</a>

``` rust
pub fn private_left_join_multiple_keys(
    a: &DataFrame,
    b: &DataFrame,
    nulls_equal: bool,
) -> Result<(Either<Vec<u32>, Vec<ChunkId>>, Either<Vec<NullableIdxSize>, Vec<ChunkId>>), PolarsError>
```

Available on **crate feature `polars-ops`** only.
