# Function split_to_struct Copy item path

<a href="https://docs.rs/polars-ops/0.51.0/x86_64-unknown-linux-gnu/src/polars_ops/chunked_array/strings/split.rs.html#57-66" class="src">Source</a>

``` rust
pub fn split_to_struct<'a, F, I>(
    ca: &'a ChunkedArray<StringType>,
    by: &'a ChunkedArray<StringType>,
    n: usize,
    op: F,
    keep_remainder: bool,
) -> Result<ChunkedArray<StructType>, PolarsError>where
    F: Fn(&'a str, &'a str) -> I,
    I: Iterator<Item = &'a str>,
```

Available on **crate feature `polars-ops`** only.
