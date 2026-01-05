# Function array_union Copy item path

<a href="https://docs.rs/datafusion-functions-nested/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_functions_nested/set_ops.rs.html#46-52" class="src">Source</a>

``` rust
pub fn array_union(array1: Expr, array2: Expr) -> Expr
```

Available on **crate feature `nested_expressions`** only.

Expand description

returns an array of the elements in the union of array1 and array2 without duplicates.
