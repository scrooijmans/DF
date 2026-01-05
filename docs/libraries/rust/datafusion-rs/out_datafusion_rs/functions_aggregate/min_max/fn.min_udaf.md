# Function min_udaf Copy item path

<a href="https://docs.rs/datafusion-functions-aggregate/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_functions_aggregate/min_max.rs.html#1424-1430" class="src">Source</a>

``` rust
pub fn min_udaf() -> Arc<AggregateUDF>
```

Expand description

AggregateFunction that returns a [`AggregateUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html "struct datafusion::logical_expr::AggregateUDF") for [`Min`](https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/min_max/struct.Min.html "struct datafusion::functions_aggregate::min_max::Min")
