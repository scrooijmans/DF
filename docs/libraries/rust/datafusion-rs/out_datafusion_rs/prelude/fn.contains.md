# Function contains Copy item path

<a href="https://docs.rs/datafusion-functions/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_functions/string/mod.rs.html#70-145" class="src">Source</a>

``` rust
pub fn contains(string: Expr, search_string: Expr) -> Expr
```

Expand description

Return true if `search_string` is found within `string`.
