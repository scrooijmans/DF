# Function arg_where Copy item path

<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/src/polars_plan/dsl/functions/index.rs.html#19" class="src">Source</a>

``` rust
pub fn arg_where<E>(condition: E) -> Exprwhere
    E: Into<Expr>,
```

Available on **crate feature `lazy`** only.

Expand description

Get the indices where `condition` evaluates `true`.
