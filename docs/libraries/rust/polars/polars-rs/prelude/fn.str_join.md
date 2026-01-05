# Function str_join Copy item path

<a href="https://docs.rs/polars-ops/0.51.0/x86_64-unknown-linux-gnu/src/polars_ops/chunked_array/strings/concat.rs.html#7" class="src">Source</a>

``` rust
pub fn str_join(
    ca: &ChunkedArray<StringType>,
    delimiter: &str,
    ignore_nulls: bool,
) -> ChunkedArray<StringType>
```

Available on **crate feature `polars-ops`** only.
