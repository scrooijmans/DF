# Function scan_partitioned Copy item path

<a href="https://docs.rs/datafusion-physical-plan/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_plan/test.rs.html#470" class="src">Source</a>

``` rust
pub fn scan_partitioned(partitions: usize) -> Arc<dyn ExecutionPlan>
```

Expand description

Returns a `DataSourceExec` that scans `partitions` of 100 batches each
