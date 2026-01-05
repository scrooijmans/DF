# Function hor_str_concat Copy item path

<a href="https://docs.rs/polars-ops/0.51.0/x86_64-unknown-linux-gnu/src/polars_ops/chunked_array/strings/concat.rs.html#58-62" class="src">Source</a>

``` rust
pub fn hor_str_concat(
    cas: &[&ChunkedArray<StringType>],
    delimiter: &str,
    ignore_nulls: bool,
) -> Result<ChunkedArray<StringType>, PolarsError>
```

Available on **crate feature `polars-ops`** only.

Expand description

Horizontally concatenate all strings.

Each array should have length 1 or a length equal to the maximum length.
