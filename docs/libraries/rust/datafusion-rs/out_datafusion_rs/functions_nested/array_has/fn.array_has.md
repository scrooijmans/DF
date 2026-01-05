# Function array_has Copy item path

<a href="https://docs.rs/datafusion-functions-nested/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_functions_nested/array_has.rs.html#44-49" class="src">Source</a>

``` rust
pub fn array_has(haystack_array: Expr, element: Expr) -> Expr
```

Available on **crate feature `nested_expressions`** only.

Expand description

returns true, if the element appears in the first array, otherwise false.
