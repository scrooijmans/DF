# Function estimate_n_lines_in_chunk Copy item path

<a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/src/polars_io/ndjson/core.rs.html#454" class="src">Source</a>

``` rust
pub fn estimate_n_lines_in_chunk(chunk: &[u8]) -> usize
```

Available on **crate feature `polars-io`** only.

Expand description

Total len divided by max len of first and last non-empty lines. This is intended to be cheaper than `estimate_n_lines_in_file`.
