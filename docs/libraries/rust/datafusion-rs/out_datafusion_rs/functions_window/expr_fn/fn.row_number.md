# Function row_number Copy item path

<a href="https://docs.rs/datafusion-functions-window/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_functions_window/row_number.rs.html#38-42" class="src">Source</a>

``` rust
pub fn row_number() -> Expr
```

Expand description

Create a [`WindowFunction`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#variant.WindowFunction "variant datafusion::prelude::Expr::WindowFunction") expression for `RowNumber` user-defined window function.

Returns a unique row number for each row in window partition beginning at 1.
