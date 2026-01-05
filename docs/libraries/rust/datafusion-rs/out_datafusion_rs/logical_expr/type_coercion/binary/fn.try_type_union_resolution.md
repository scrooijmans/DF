# Function try_type_union_resolution Copy item path

<a href="https://docs.rs/datafusion-expr-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr_common/type_coercion/binary.rs.html#660" class="src">Source</a>

``` rust
pub fn try_type_union_resolution(
    data_types: &[DataType],
) -> Result<Vec<DataType>, DataFusionError>
```

Expand description

Handle type union resolution including struct type and others.
