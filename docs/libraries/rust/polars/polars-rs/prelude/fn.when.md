# Function when Copy item path

<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/src/polars_plan/dsl/arity.rs.html#134" class="src">Source</a>

``` rust
pub fn when<E>(condition: E) -> Whenwhere
    E: Into<Expr>,
```

Available on **crate feature `lazy`** only.

Expand description

Start a `when-then-otherwise` expression.
