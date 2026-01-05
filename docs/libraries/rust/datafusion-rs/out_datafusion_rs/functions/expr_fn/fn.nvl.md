# Function nvl Copy item path

<a href="https://docs.rs/datafusion-functions/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_functions/core/mod.rs.html#62-110" class="src">Source</a>

``` rust
pub fn nvl(arg1: Expr, arg2: Expr) -> Expr
```

Expand description

Returns value2 if value1 is NULL; otherwise it returns value1
