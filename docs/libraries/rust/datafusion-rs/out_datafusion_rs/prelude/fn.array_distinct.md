# Function array_distinct Copy item path

<a href="https://docs.rs/datafusion-functions-nested/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_functions_nested/set_ops.rs.html#62-68" class="src">Source</a>

``` rust
pub fn array_distinct(array: Expr) -> Expr
```

Available on **crate feature `nested_expressions`** only.

Expand description

returns distinct values from the array after removing duplicates.
