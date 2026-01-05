# Function array_resize Copy item path

<a href="https://docs.rs/datafusion-functions-nested/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_functions_nested/resize.rs.html#43-49" class="src">Source</a>

``` rust
pub fn array_resize(array: Expr, size: Expr, value: Expr) -> Expr
```

Available on **crate feature `nested_expressions`** only.

Expand description

returns an array with the specified size filled with the given value.
