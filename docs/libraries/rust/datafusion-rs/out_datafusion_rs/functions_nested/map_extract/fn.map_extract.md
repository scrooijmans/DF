# Function map_extract Copy item path

<a href="https://docs.rs/datafusion-functions-nested/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_functions_nested/map_extract.rs.html#37-43" class="src">Source</a>

``` rust
pub fn map_extract(map: Expr, key: Expr) -> Expr
```

Available on **crate feature `nested_expressions`** only.

Expand description

Return a list containing the value for a given key or an empty list if the key is not contained in the map.
