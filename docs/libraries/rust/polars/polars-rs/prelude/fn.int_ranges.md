# Function int_ranges Copy item path

<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/src/polars_plan/dsl/functions/range.rs.html#26" class="src">Source</a>

``` rust
pub fn int_ranges(
    start: Expr,
    end: Expr,
    step: Expr,
    dtype: impl Into<DataTypeExpr>,
) -> Expr
```

Available on **crate feature `lazy`** only.

Expand description

Generate a range of integers for each row of the input columns.
