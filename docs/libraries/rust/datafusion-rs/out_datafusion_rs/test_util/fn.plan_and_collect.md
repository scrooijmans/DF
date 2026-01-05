# Function plan_and_collect Copy item path

<a href="https://docs.rs/datafusion/50.2.0/src/datafusion/test_util/mod.rs.html#138-143" class="src">Source</a>

``` rust
pub async fn plan_and_collect(
    ctx: &SessionContext,
    sql: &str,
) -> Result<Vec<RecordBatch>>
```

Expand description

Execute SQL and return results
