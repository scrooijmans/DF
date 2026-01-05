# Function peak_max_with_start_end Copy item path

<a href="https://docs.rs/polars-ops/0.51.0/x86_64-unknown-linux-gnu/src/polars_ops/chunked_array/peaks.rs.html#36-42" class="src">Source</a>

``` rust
pub fn peak_max_with_start_end<T>(
    ca: &ChunkedArray<T>,
    start: Option<<T as PolarsNumericType>::Native>,
    end: Option<<T as PolarsNumericType>::Native>,
) -> ChunkedArray<BooleanType>where
    T: PolarsNumericType,
    ChunkedArray<T>: for<'a> ChunkCompareIneq<&'a ChunkedArray<T>, Item = ChunkedArray<BooleanType>>,
```

Available on **crate feature `polars-ops`** only.

Expand description

Get a boolean mask of the local maximum peaks.
