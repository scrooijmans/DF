# Function data_types_with_scalar_udf Copy item path

<a href="https://docs.rs/datafusion-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr/type_coercion/functions.rs.html#48-51" class="src">Source</a>

``` rust
pub fn data_types_with_scalar_udf(
    current_types: &[DataType],
    func: &ScalarUDF,
) -> Result<Vec<DataType>, DataFusionError>
```

Expand description

Performs type coercion for scalar function arguments.

Returns the data types to which each argument must be coerced to match `signature`.

For more details on coercion in general, please see the [`type_coercion`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/type_coercion/index.html "mod datafusion::logical_expr::type_coercion") module.
