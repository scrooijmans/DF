# Function find_join_exprs Copy item path

<a href="https://docs.rs/datafusion-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr/utils.rs.html#1192" class="src">Source</a>

``` rust
pub fn find_join_exprs(
    exprs: Vec<&Expr>,
) -> Result<(Vec<Expr>, Vec<Expr>), DataFusionError>
```

Expand description

Looks for correlating expressions: for example, a binary expression with one field from the subquery, and one not in the subquery (closed upon from outer scope)

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/utils/fn.find_join_exprs.html#arguments" class="doc-anchor">§</a>Arguments

- `exprs` - List of expressions that may or may not be joins

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/utils/fn.find_join_exprs.html#return-value" class="doc-anchor">§</a>Return value

Tuple of (expressions containing joins, remaining non-join expressions)
