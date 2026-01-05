# Function corr_udaf Copy item path

<a href="https://docs.rs/datafusion-functions-aggregate/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_functions_aggregate/correlation.rs.html#50-56" class="src">Source</a>

``` rust
pub fn corr_udaf() -> Arc<AggregateUDF>
```

Expand description

AggregateFunction that returns a [`AggregateUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html "struct datafusion::logical_expr::AggregateUDF") for [`Correlation`](https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/correlation/struct.Correlation.html "struct datafusion::functions_aggregate::correlation::Correlation")
