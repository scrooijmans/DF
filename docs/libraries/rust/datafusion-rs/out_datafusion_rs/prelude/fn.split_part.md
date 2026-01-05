# Function split_part Copy item path

<a href="https://docs.rs/datafusion-functions/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_functions/string/mod.rs.html#70-145" class="src">Source</a>

``` rust
pub fn split_part(string: Expr, delimiter: Expr, index: Expr) -> Expr
```

Expand description

Splits a string based on a delimiter and picks out the desired field based on the index.
