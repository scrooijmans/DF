# Function create_udf Copy item path

<a href="https://docs.rs/datafusion-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr/expr_fn.rs.html#388-394" class="src">Source</a>

``` rust
pub fn create_udf(
    name: &str,
    input_types: Vec<DataType>,
    return_type: DataType,
    volatility: Volatility,
    fun: Arc<dyn Fn(&[ColumnarValue]) -> Result<ColumnarValue, DataFusionError> + Sync + Send>,
) -> ScalarUDF
```

Expand description

Convenience method to create a new user defined scalar function (UDF) with a specific signature and specific return type.

Note this function does not expose all available features of [`ScalarUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarUDF.html "struct datafusion::logical_expr::ScalarUDF"), such as

- computing return types based on input types
- multiple [`Signature`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Signature.html "struct datafusion::logical_expr::Signature")s
- aliases

See [`ScalarUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarUDF.html "struct datafusion::logical_expr::ScalarUDF") for details and examples on how to use the full functionality.
