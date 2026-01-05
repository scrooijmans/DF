# Function regr_avgy Copy item path

<a href="https://docs.rs/datafusion-functions-aggregate/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_functions_aggregate/regr.rs.html#57" class="src">Source</a>

``` rust
pub fn regr_avgy(expr_y: Expr, expr_x: Expr) -> Expr
```

Expand description

Compute a linear regression of type [RegrType::AvgY](https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/regr/enum.RegrType.html#variant.AvgY "variant datafusion::functions_aggregate::regr::RegrType::AvgY")
