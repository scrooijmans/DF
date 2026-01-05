# Function expand_qualified_wildcard Copy item path

<a href="https://docs.rs/datafusion-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr/utils.rs.html#423-427" class="src">Source</a>

``` rust
pub fn expand_qualified_wildcard(
    qualifier: &TableReference,
    schema: &DFSchema,
    wildcard_options: Option<&WildcardOptions>,
) -> Result<Vec<Expr>, DataFusionError>
```

Expand description

Resolves an `Expr::Wildcard` to a collection of qualified `Expr::Column`’s.
