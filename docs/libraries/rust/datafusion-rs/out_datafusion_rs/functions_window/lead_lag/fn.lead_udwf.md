# Function lead_udwf Copy item path

<a href="https://docs.rs/datafusion-functions-window/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_functions_window/lead_lag.rs.html#50-57" class="src">Source</a>

``` rust
pub fn lead_udwf() -> Arc<WindowUDF>
```

Expand description

Returns a [`WindowUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.WindowUDF.html "struct datafusion::logical_expr::WindowUDF") for [`lead`](https://docs.rs/datafusion/50.2.0/datafusion/functions_window/lead_lag/fn.lead.html "fn datafusion::functions_window::lead_lag::lead").

Returns the value from a row that follows the current row by a specified offset within the partition. If no such row exists, then returns the default value.
