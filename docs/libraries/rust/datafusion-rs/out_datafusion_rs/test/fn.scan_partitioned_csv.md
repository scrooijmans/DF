# Function scan_partitioned_csv Copy item path

<a href="https://docs.rs/datafusion/50.2.0/src/datafusion/test/mod.rs.html#80-101" class="src">Source</a>

``` rust
pub fn scan_partitioned_csv(
    partitions: usize,
    work_dir: &Path,
) -> Result<Arc<DataSourceExec>>
```

Available on **non-WebAssembly** only.

Expand description

Returns a [`DataSourceExec`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/memory/struct.DataSourceExec.html "struct datafusion::datasource::memory::DataSourceExec") that scans “aggregate_test_100.csv” with `partitions` partitions
