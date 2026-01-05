# Function stddev_udaf Copy item path

<a href="https://docs.rs/datafusion-functions-aggregate/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_functions_aggregate/stddev.rs.html#42-48" class="src">Source</a>

``` rust
pub fn stddev_udaf() -> Arc<AggregateUDF>
```

Expand description

AggregateFunction that returns a [`AggregateUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html "struct datafusion::logical_expr::AggregateUDF") for [`Stddev`](https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/stddev/struct.Stddev.html "struct datafusion::functions_aggregate::stddev::Stddev")
