# Function array_dims_inner Copy item path

<a href="https://docs.rs/datafusion-functions-nested/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_functions_nested/dimension.rs.html#193" class="src">Source</a>

``` rust
pub fn array_dims_inner(
    args: &[Arc<dyn Array>],
) -> Result<Arc<dyn Array>, DataFusionError>
```

Available on **crate feature `nested_expressions`** only.

Expand description

Array_dims SQL function
