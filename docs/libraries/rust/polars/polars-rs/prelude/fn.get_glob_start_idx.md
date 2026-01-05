# Function get_glob_start_idx Copy item path

<a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/src/polars_io/path_utils/mod.rs.html#140" class="src">Source</a>

``` rust
pub fn get_glob_start_idx(path: &[u8]) -> Option<usize>
```

Available on **crate feature `polars-io`** only.

Expand description

Get the index of the first occurrence of a glob symbol.
