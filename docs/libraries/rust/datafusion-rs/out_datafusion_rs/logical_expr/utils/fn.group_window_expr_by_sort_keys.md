# Function group_window_expr_by_sort_keys Copy item path

<a href="https://docs.rs/datafusion-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr/utils.rs.html#577-579" class="src">Source</a>

``` rust
pub fn group_window_expr_by_sort_keys(
    window_expr: impl IntoIterator<Item = Expr>,
) -> Result<Vec<(Vec<(Sort, bool)>, Vec<Expr>)>, DataFusionError>
```

Expand description

Group a slice of window expression expr by their order by expressions
