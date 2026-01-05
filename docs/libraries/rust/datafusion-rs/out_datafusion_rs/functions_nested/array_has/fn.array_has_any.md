# Function array_has_any Copy item path

<a href="https://docs.rs/datafusion-functions-nested/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_functions_nested/array_has.rs.html#56-61" class="src">Source</a>

``` rust
pub fn array_has_any(haystack_array: Expr, needle_array: Expr) -> Expr
```

Available on **crate feature `nested_expressions`** only.

Expand description

returns true if at least one element of the second array appears in the first array; otherwise, it returns false.
