# Function count_udaf Copy item path

<a href="https://docs.rs/datafusion-functions-aggregate/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_functions_aggregate/count.rs.html#63-69" class="src">Source</a>

``` rust
pub fn count_udaf() -> Arc<AggregateUDF>
```

Expand description

AggregateFunction that returns a [`AggregateUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html "struct datafusion::logical_expr::AggregateUDF") for [`Count`](https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/count/struct.Count.html "struct datafusion::functions_aggregate::count::Count")
