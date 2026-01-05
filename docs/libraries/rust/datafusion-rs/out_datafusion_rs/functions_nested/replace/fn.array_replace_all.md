# Function array_replace_all Copy item path

<a href="https://docs.rs/datafusion-functions-nested/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_functions_nested/replace.rs.html#55-60" class="src">Source</a>

``` rust
pub fn array_replace_all(array: Expr, from: Expr, to: Expr) -> Expr
```

Available on **crate feature `nested_expressions`** only.

Expand description

replaces all occurrences of the specified element with another specified element.
