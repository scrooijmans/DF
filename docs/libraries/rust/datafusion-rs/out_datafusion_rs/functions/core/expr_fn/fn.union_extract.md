# Function union_extract Copy item path

<a href="https://docs.rs/datafusion-functions/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_functions/core/mod.rs.html#118" class="src">Source</a>

``` rust
pub fn union_extract(arg1: Expr, arg2: impl Literal) -> Expr
```

Expand description

Returns the value of the field with the given name from the union when it’s selected, or NULL otherwise
