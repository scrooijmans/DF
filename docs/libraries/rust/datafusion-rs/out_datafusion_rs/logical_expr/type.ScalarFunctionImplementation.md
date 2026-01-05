# Type Alias ScalarFunctionImplementation Copy item path

<a href="https://docs.rs/datafusion-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr/function.rs.html#50" class="src">Source</a>

``` rust
pub type ScalarFunctionImplementation = Arc<dyn Fn(&[ColumnarValue]) -> Result<ColumnarValue, DataFusionError> + Sync + Send>;
```

Expand description

Scalar function

The Fn param is the wrapped function but be aware that the function will be passed with the slice / vec of columnar values (either scalar or array) with the exception of zero param function, where a singular element vec will be passed. In that case the single element is a null array to indicate the batch’s row count (so that the generative zero-argument function can know the result array size).

## Aliased Type<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/type.ScalarFunctionImplementation.html#aliased-type" class="anchor">§</a>

``` rust
pub struct ScalarFunctionImplementation { /* private fields */ }
```
