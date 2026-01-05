# Function write_partitioned_dataset Copy item path

<a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/src/polars_io/partition.rs.html#45-52" class="src">Source</a>

``` rust
pub fn write_partitioned_dataset(
    df: &mut DataFrame,
    addr: PlPathRef<'_>,
    partition_by: Vec<PlSmallStr>,
    file_write_options: &(dyn WriteDataFrameToFile + Send + Sync),
    cloud_options: Option<&CloudOptions>,
    chunk_size: usize,
) -> Result<(), PolarsError>
```

Available on **crate feature `polars-io`** only.

Expand description

Write a partitioned parquet dataset. This functionality is unstable.
