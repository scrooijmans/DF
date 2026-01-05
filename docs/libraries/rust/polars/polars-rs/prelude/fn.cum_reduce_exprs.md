# Function cum_reduce_exprs Copy item path

<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/src/polars_plan/dsl/functions/horizontal.rs.html#56-64" class="src">Source</a>

``` rust
pub fn cum_reduce_exprs<E>(
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
