# Function materialize_projection Copy item path

<a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/src/polars_io/utils/other.rs.html#161-166" class="src">Source</a>

``` rust
pub fn materialize_projection(
    with_columns: Option<&[PlSmallStr]>,
    schema: &Schema<DataType>,
    hive_partitions: Option<&[Series]>,
    has_row_index: bool,
) -> Option<Vec<usize>>
```

Available on **crate feature `polars-io`** only.
