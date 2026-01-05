# Type Alias StateTypeFunction Copy item path

<a href="https://docs.rs/datafusion-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr/function.rs.html#64" class="src">Source</a>

``` rust
pub type StateTypeFunction = Arc<dyn Fn(&DataType) -> Result<Arc<Vec<DataType>>, DataFusionError> + Sync + Send>;
```

Expand description

Factory that returns the types used by an aggregator to serialize its state, given its return datatype.

## Aliased Type<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/type.StateTypeFunction.html#aliased-type" class="anchor">§</a>

``` rust
pub struct StateTypeFunction { /* private fields */ }
```
