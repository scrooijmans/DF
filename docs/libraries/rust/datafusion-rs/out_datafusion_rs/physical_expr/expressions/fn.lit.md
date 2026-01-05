# Function lit Copy item path

<a href="https://docs.rs/datafusion-physical-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_expr/expressions/literal.rs.html#140" class="src">Source</a>

``` rust
pub fn lit<T>(value: T) -> Arc<dyn PhysicalExpr>where
    T: Literal,
```

Expand description

Create a literal expression
