# Function is_volatile Copy item path

<a href="https://docs.rs/datafusion-physical-expr-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_expr_common/physical_expr.rs.html#588" class="src">Source</a>

``` rust
pub fn is_volatile(expr: &Arc<dyn PhysicalExpr>) -> bool
```

Expand description

Returns true if the expression is volatile, i.e. whether it can return different results when evaluated multiple times with the same input.

For example the function call `RANDOM()` is volatile as each call will return a different value.

This method recursively checks if any sub-expression is volatile, for example `1 + RANDOM()` will return `true`.
