# Function strip_prefix Copy item path

<a href="https://docs.rs/polars-ops/0.51.0/x86_64-unknown-linux-gnu/src/polars_ops/chunked_array/strings/strip.rs.html#121" class="src">Source</a>

``` rust
pub fn strip_prefix(
    ca: &ChunkedArray<StringType>,
    prefix: &ChunkedArray<StringType>,
) -> ChunkedArray<StringType>
```

Available on **crate feature `polars-ops`** only.
