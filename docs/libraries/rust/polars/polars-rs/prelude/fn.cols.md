# Function cols Copy item path

<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/src/polars_plan/dsl/functions/selectors.rs.html#50-53" class="src">Source</a>

``` rust
pub fn cols<I, S>(names: I) -> Selectorwhere
    I: IntoIterator<Item = S>,
    S: Into<PlSmallStr>,
```

Available on **crate feature `lazy`** only.

Expand description

Select multiple columns by name.
