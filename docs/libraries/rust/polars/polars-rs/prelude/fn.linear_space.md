# Function linear_space Copy item path

<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/src/polars_plan/dsl/functions/range.rs.html#114" class="src">Source</a>

``` rust
pub fn linear_space(
    start: Expr,
    end: Expr,
    num_samples: Expr,
    closed: ClosedInterval,
) -> Expr
```

Available on **crate feature `lazy`** only.

Expand description

Generate a series of equally-spaced points.
