# Function lead Copy item path

<a href="https://docs.rs/datafusion-functions-window/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_functions_window/lead_lag.rs.html#84-88" class="src">Source</a>

``` rust
pub fn lead(
    arg: Expr,
    shift_offset: Option<i64>,
    default_value: Option<ScalarValue>,
) -> Expr
```

Expand description

Create an expression to represent the `lead` window function

returns value evaluated at the row that is offset rows after the current row within the partition; if there is no such row, instead return default (which must be of the same type as value). Both offset and default are evaluated with respect to the current row. If omitted, offset defaults to 1 and default to null
