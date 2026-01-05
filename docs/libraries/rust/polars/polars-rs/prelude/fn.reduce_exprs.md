# Function reduce_exprs Copy item path

<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/src/polars_plan/dsl/functions/horizontal.rs.html#33-40" class="src">Source</a>

``` rust
pub fn reduce_exprs<E>(
    f: PlanCallback<(Series, Series), Series>,
    exprs: E,
    returns_scalar: bool,
    return_dtype: Option<DataTypeExpr>,
) -> Exprwhere
    E: AsRef<[Expr]>,
```

Available on **crate feature `lazy`** only.

Expand description

Analogous to [`Iterator::reduce`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.reduce "method core::iter::traits::iterator::Iterator::reduce").

An accumulator is initialized to the series given by the first expression in `exprs`, and then each subsequent value of the accumulator is computed from `f(acc, next_expr_series)`. If `exprs` is empty, an error is returned when `collect` is called.
