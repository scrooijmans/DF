# Function arg_sort_by Copy item path

<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/src/polars_plan/dsl/functions/index.rs.html#9" class="src">Source</a>

``` rust
pub fn arg_sort_by<E>(by: E, sort_options: SortMultipleOptions) -> Exprwhere
    E: AsRef<[Expr]>,
```

Available on **crate feature `lazy`** only.

Expand description

Find the indexes that would sort these series in order of appearance.

That means that the first `Series` will be used to determine the ordering until duplicates are found. Once duplicates are found, the next `Series` will be used and so on.
