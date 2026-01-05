# Function fold_exprs Copy item path

<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/src/polars_plan/dsl/functions/horizontal.rs.html#4-12" class="src">Source</a>

``` rust
pub fn fold_exprs<E>(
    acc: Expr,
    f: PlanCallback<(Series, Series), Series>,
    exprs: E,
    returns_scalar: bool,
    return_dtype: Option<DataTypeExpr>,
) -> Exprwhere
    E: AsRef<[Expr]>,
```

Available on **crate feature `lazy`** only.

Expand description

Accumulate over multiple columns horizontally / row wise.
