# Function nanvl Copy item path

<a href="https://docs.rs/datafusion-functions/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_functions/math/mod.rs.html#243-282" class="src">Source</a>

``` rust
pub fn nanvl(x: Expr, y: Expr) -> Expr
```

Expand description

returns x if x is not NaN otherwise returns y
