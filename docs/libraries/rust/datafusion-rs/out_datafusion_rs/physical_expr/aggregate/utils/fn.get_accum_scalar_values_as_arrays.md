# Function get_accum_scalar_values_as_arrays Copy item path

<a href="https://docs.rs/datafusion-functions-aggregate-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_functions_aggregate_common/utils.rs.html#29-31" class="src">Source</a>

``` rust
pub fn get_accum_scalar_values_as_arrays(
    accum: &mut dyn Accumulator,
) -> Result<Vec<Arc<dyn Array>>, DataFusionError>
```

Expand description

Convert scalar values from an accumulator into arrays.
