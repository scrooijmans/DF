# Function regr_slope Copy item path

<a href="https://docs.rs/datafusion-functions-aggregate/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_functions_aggregate/regr.rs.html#52" class="src">Source</a>

``` rust
pub fn regr_slope(expr_y: Expr, expr_x: Expr) -> Expr
```

Expand description

Compute a linear regression of type [RegrType::Slope](https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/regr/enum.RegrType.html#variant.Slope "variant datafusion::functions_aggregate::regr::RegrType::Slope")
