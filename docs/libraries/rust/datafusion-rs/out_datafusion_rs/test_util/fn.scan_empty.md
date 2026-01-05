# Function scan_empty Copy item path

<a href="https://docs.rs/datafusion/50.2.0/src/datafusion/test_util/mod.rs.html#62-71" class="src">Source</a>

``` rust
pub fn scan_empty(
    name: Option<&str>,
    table_schema: &Schema,
    projection: Option<Vec<usize>>,
) -> Result<LogicalPlanBuilder>
```

Expand description

Scan an empty data source, mainly used in tests
