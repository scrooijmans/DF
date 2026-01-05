# Function try_infer_udf_output_dtype Copy item path

<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/src/polars_plan/dsl/udf.rs.html#87-90" class="src">Source</a>

``` rust
pub fn try_infer_udf_output_dtype(
    f: &dyn Fn(&[Column]) -> Result<Column, PolarsError>,
    input_fields: &[Field],
) -> Result<DataType, PolarsError>
```

Available on **crate feature `lazy`** only.

Expand description

Try to infer the output datatype of a UDF.

This will call the UDF in a few ways and see if it can get an output type without erroring.
