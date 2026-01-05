# Function try_set_sorted_flag Copy item path

<a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/src/polars_io/parquet/read/read_impl.rs.html#63" class="src">Source</a>

``` rust
pub fn try_set_sorted_flag(
    series: &mut Series,
    col_idx: usize,
    sorting_map: &[(usize, IsSorted)],
)
```

Available on **crate feature `polars-io`** only.
