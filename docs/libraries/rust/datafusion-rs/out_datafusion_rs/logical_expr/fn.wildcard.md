# Function wildcard Copy item path

<a href="https://docs.rs/datafusion-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr/expr_fn.rs.html#126" class="src">Source</a>

``` rust
pub fn wildcard() -> SelectExpr
```

Expand description

Create an ‘\*’ [`Expr::Wildcard`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#variant.Wildcard "variant datafusion::prelude::Expr::Wildcard") expression that matches all columns

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/fn.wildcard.html#example" class="doc-anchor">§</a>Example

``` rust
let p = wildcard();
assert_eq!(p.to_string(), "*")
```
