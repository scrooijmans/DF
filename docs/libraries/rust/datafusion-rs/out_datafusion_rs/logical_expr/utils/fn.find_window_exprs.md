# Function find_window_exprs Copy item path

<a href="https://docs.rs/datafusion-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr/utils.rs.html#612" class="src">Source</a>

``` rust
pub fn find_window_exprs<'a>(
    exprs: impl IntoIterator<Item = &'a Expr>,
) -> Vec<Expr>
```

Expand description

Collect all deeply nested `Expr::WindowFunction`. They are returned in order of occurrence (depth first), with duplicates omitted.
