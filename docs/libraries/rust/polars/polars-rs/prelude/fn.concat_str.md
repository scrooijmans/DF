# Function concat_str Copy item path

<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/src/polars_plan/dsl/functions/concat.rs.html#5" class="src">Source</a>

``` rust
pub fn concat_str<E>(s: E, separator: &str, ignore_nulls: bool) -> Exprwhere
    E: AsRef<[Expr]>,
```

Available on **crate feature `lazy`** only.

Expand description

Horizontally concat string columns in linear time
