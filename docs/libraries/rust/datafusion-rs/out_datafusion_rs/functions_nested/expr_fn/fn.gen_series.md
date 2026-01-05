# Function gen_series Copy item path

<a href="https://docs.rs/datafusion-functions-nested/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_functions_nested/range.rs.html#188-194" class="src">Source</a>

``` rust
pub fn gen_series(start: Expr, stop: Expr, step: Expr) -> Expr
```

Available on **crate feature `nested_expressions`** only.

Expand description

create a list of values in the range between start and stop, include upper bound
