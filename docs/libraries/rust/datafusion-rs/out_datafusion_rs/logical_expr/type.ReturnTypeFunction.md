# Type Alias ReturnTypeFunction Copy item path

<a href="https://docs.rs/datafusion-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr/function.rs.html#54" class="src">Source</a>

``` rust
pub type ReturnTypeFunction = Arc<dyn Fn(&[DataType]) -> Result<Arc<DataType>, DataFusionError> + Sync + Send>;
```

Expand description

Factory that returns the functions’s return type given the input argument types

## Aliased Type<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/type.ReturnTypeFunction.html#aliased-type" class="anchor">§</a>

``` rust
pub struct ReturnTypeFunction { /* private fields */ }
```
