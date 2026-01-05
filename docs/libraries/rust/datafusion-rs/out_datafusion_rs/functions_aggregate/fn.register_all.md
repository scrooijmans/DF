# Function register_all Copy item path

<a href="https://docs.rs/datafusion-functions-aggregate/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_functions_aggregate/lib.rs.html#185" class="src">Source</a>

``` rust
pub fn register_all(
    registry: &mut dyn FunctionRegistry,
) -> Result<(), DataFusionError>
```

Expand description

Registers all enabled packages with a [`FunctionRegistry`](https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.FunctionRegistry.html "trait datafusion::execution::FunctionRegistry")
