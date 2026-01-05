# Type Alias AccumulatorFactoryFunction Copy item path

<a href="https://docs.rs/datafusion-functions-aggregate-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_functions_aggregate_common/accumulator.rs.html#80" class="src">Source</a>

``` rust
pub type AccumulatorFactoryFunction = Arc<dyn Fn(AccumulatorArgs<'_>) -> Result<Box<dyn Accumulator>, DataFusionError> + Sync + Send>;
```

Expand description

Factory that returns an accumulator for the given aggregate function.

## Aliased Type<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/type.AccumulatorFactoryFunction.html#aliased-type" class="anchor">§</a>

``` rust
pub struct AccumulatorFactoryFunction { /* private fields */ }
```
