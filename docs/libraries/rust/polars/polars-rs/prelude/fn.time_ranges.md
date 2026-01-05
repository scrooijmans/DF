# Function time_ranges Copy item path

<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/src/polars_plan/dsl/functions/range.rs.html#106" class="src">Source</a>

``` rust
pub fn time_ranges(
    start: Expr,
    end: Expr,
    interval: Duration,
    closed: ClosedWindow,
) -> Expr
```

Available on **crate feature `lazy`** only.

Expand description

Create a column of time ranges from a `start` and `stop` expression.
