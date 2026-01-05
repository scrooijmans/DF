# Function any_horizontal Copy item path

<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/src/polars_plan/dsl/functions/horizontal.rs.html#123" class="src">Source</a>

``` rust
pub fn any_horizontal<E>(exprs: E) -> Result<Expr, PolarsError>where
    E: AsRef<[Expr]>,
```

Available on **crate feature `lazy`** only.

Expand description

Create a new column with the bitwise-or of the elements in each row.

The name of the resulting column will be “any”; use [`alias`](https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.alias "method polars::prelude::Expr::alias") to choose a different name.
