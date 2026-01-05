# Function populate_csv_partitions Copy item path

<a href="https://docs.rs/datafusion/50.2.0/src/datafusion/test_util/mod.rs.html#146-172" class="src">Source</a>

``` rust
pub fn populate_csv_partitions(
    tmp_dir: &TempDir,
    partition_count: usize,
    file_extension: &str,
) -> Result<SchemaRef>
```

Expand description

Generate CSV partitions within the supplied directory
