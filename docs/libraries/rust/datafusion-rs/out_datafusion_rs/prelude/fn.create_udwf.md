# Function create_udwf Copy item path

<a href="https://docs.rs/datafusion-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr/expr_fn.rs.html#608-614" class="src">Source</a>

``` rust
pub fn create_udwf(
    name: &str,
    input_type: DataType,
    return_type: Arc<DataType>,
    volatility: Volatility,
    partition_evaluator_factory: Arc<dyn Fn() -> Result<Box<dyn PartitionEvaluator>, DataFusionError> + Sync + Send>,
) -> WindowUDF
```

Expand description

Creates a new UDWF with a specific signature, state type and return type.

The signature and state type must match the [`PartitionEvaluator`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.PartitionEvaluator.html "trait datafusion::logical_expr::PartitionEvaluator")’s implementation\`.
