# Function create_udaf Copy item path

<a href="https://docs.rs/datafusion-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr/expr_fn.rs.html#484-491" class="src">Source</a>

``` rust
pub fn create_udaf(
    name: &str,
    input_type: Vec<DataType>,
    return_type: Arc<DataType>,
    volatility: Volatility,
    accumulator: Arc<dyn Fn(AccumulatorArgs<'_>) -> Result<Box<dyn Accumulator>, DataFusionError> + Sync + Send>,
    state_type: Arc<Vec<DataType>>,
) -> AggregateUDF
```

Expand description

Creates a new UDAF with a specific signature, state type and return type. The signature and state type must match the `Accumulator's implementation`.
