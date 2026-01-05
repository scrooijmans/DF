# Function scan_empty_with_partitions Copy item path

<a href="https://docs.rs/datafusion/50.2.0/src/datafusion/test_util/mod.rs.html#74-84" class="src">Source</a>

``` rust
pub fn scan_empty_with_partitions(
    name: Option<&str>,
    table_schema: &Schema,
    projection: Option<Vec<usize>>,
    partitions: usize,
) -> Result<LogicalPlanBuilder>
```

Expand description

Scan an empty data source with configured partition, mainly used in tests.
