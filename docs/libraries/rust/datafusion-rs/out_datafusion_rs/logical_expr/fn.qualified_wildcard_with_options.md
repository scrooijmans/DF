# Function qualified_wildcard_with_options Copy item path

<a href="https://docs.rs/datafusion-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr/expr_fn.rs.html#150-153" class="src">Source</a>

``` rust
pub fn qualified_wildcard_with_options(
    qualifier: impl Into<TableReference>,
    options: WildcardOptions,
) -> SelectExpr
```

Expand description

Create an ‘t.\*’ [`Expr::Wildcard`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#variant.Wildcard "variant datafusion::prelude::Expr::Wildcard") expression with the wildcard options
