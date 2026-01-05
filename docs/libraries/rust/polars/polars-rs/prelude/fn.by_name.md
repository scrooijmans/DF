# Function by_name Copy item path

<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/src/polars_plan/dsl/functions/selectors.rs.html#70" class="src">Source</a>

``` rust
pub fn by_name<S, I>(names: I, strict: bool) -> Selectorwhere
    S: Into<PlSmallStr>,
    I: IntoIterator<Item = S>,
```

Available on **crate feature `lazy`** only.

Expand description

Select multiple columns by dtype.
