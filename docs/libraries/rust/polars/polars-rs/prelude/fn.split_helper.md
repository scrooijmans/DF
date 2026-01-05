# Function split_helper Copy item path

<a href="https://docs.rs/polars-ops/0.51.0/x86_64-unknown-linux-gnu/src/polars_ops/chunked_array/strings/split.rs.html#155-162" class="src">Source</a>

``` rust
pub fn split_helper<'a, F, I>(
    ca: &'a ChunkedArray<StringType>,
    by: &'a ChunkedArray<StringType>,
    op: F,
) -> Result<ChunkedArray<ListType>, PolarsError>where
    F: Fn(&'a str, &'a str) -> I,
    I: Iterator<Item = &'a str>,
```

Available on **crate feature `polars-ops`** only.
