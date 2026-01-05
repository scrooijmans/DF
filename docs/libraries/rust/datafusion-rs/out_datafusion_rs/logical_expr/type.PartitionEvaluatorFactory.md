# Type Alias PartitionEvaluatorFactory Copy item path

<a href="https://docs.rs/datafusion-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr/function.rs.html#59" class="src">Source</a>

``` rust
pub type PartitionEvaluatorFactory = Arc<dyn Fn() -> Result<Box<dyn PartitionEvaluator>, DataFusionError> + Sync + Send>;
```

Expand description

Factory that creates a PartitionEvaluator for the given window function

## Aliased Type<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/type.PartitionEvaluatorFactory.html#aliased-type" class="anchor">§</a>

``` rust
pub struct PartitionEvaluatorFactory { /* private fields */ }
```
