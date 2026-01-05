# Function qualified_wildcard Copy item path

<a href="https://docs.rs/datafusion-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr/expr_fn.rs.html#145" class="src">Source</a>

``` rust
pub fn qualified_wildcard(qualifier: impl Into<TableReference>) -> SelectExpr
```

Expand description

Create an ‘t.\*’ [`Expr::Wildcard`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#variant.Wildcard "variant datafusion::prelude::Expr::Wildcard") expression that matches all columns from a specific table

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/fn.qualified_wildcard.html#example" class="doc-anchor">§</a>Example

``` rust
let p = qualified_wildcard(TableReference::bare("t"));
assert_eq!(p.to_string(), "t.*")
```
