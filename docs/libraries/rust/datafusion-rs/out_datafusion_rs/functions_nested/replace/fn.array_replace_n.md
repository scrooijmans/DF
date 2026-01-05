# Function array_replace_n Copy item path

<a href="https://docs.rs/datafusion-functions-nested/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_functions_nested/replace.rs.html#49-54" class="src">Source</a>

``` rust
pub fn array_replace_n(array: Expr, from: Expr, to: Expr, max: Expr) -> Expr
```

Available on **crate feature `nested_expressions`** only.

Expand description

replaces the first `max` occurrences of the specified element with another specified element.
