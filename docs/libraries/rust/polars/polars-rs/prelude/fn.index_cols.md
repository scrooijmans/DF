# Function index_cols Copy item path

<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/src/polars_plan/dsl/functions/selectors.rs.html#100" class="src">Source</a>

``` rust
pub fn index_cols<N>(indices: N) -> Selectorwhere
    N: AsRef<[i64]>,
```

Available on **crate feature `lazy`** only.

Expand description

Select multiple columns by index.
