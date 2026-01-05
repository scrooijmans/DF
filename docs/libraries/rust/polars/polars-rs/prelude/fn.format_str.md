# Function format_str Copy item path

<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/src/polars_plan/dsl/functions/concat.rs.html#21" class="src">Source</a>

``` rust
pub fn format_str<E>(format: &str, args: E) -> Result<Expr, PolarsError>where
    E: AsRef<[Expr]>,
```

Available on **crate feature `lazy`** only.

Expand description

Format the results of an array of expressions using a format string
