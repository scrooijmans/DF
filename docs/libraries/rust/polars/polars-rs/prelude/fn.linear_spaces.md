# Function linear_spaces Copy item path

<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/src/polars_plan/dsl/functions/range.rs.html#122-128" class="src">Source</a>

``` rust
pub fn linear_spaces(
    start: Expr,
    end: Expr,
    num_samples: Expr,
    closed: ClosedInterval,
    as_array: bool,
) -> Result<Expr, PolarsError>
```

Available on **crate feature `lazy`** only.

Expand description

Create a column of linearly-spaced sequences from ‘start’, ‘end’, and ‘num_samples’ expressions.
