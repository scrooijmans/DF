# Function flatten Copy item path

<a href="https://docs.rs/datafusion-functions-nested/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_functions_nested/flatten.rs.html#38-44" class="src">Source</a>

``` rust
pub fn flatten(array: Expr) -> Expr
```

Available on **crate feature `nested_expressions`** only.

Expand description

flattens an array of arrays into a single array.
