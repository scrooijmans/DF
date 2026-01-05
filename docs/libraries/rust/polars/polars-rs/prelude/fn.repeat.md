# Function repeat Copy item path

<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/src/polars_plan/dsl/functions/repeat.rs.html#7" class="src">Source</a>

``` rust
pub fn repeat<E>(value: E, n: Expr) -> Exprwhere
    E: Into<Expr>,
```

Available on **crate feature `lazy`** only.

Expand description

Create a column of length `n` containing `n` copies of the literal `value`.

Generally you won’t need this function, as `lit(value)` already represents a column containing only `value` whose length is automatically set to the correct number of rows.
