# Function arrow_struct_to_literal Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/arrow/value.rs.html#596-606" class="src">Source</a>

``` rust
pub fn arrow_struct_to_literal(
    struct_array: &ArrayRef,
    ty: &StructType,
) -> Result<Vec<Option<Literal>>>
```

Expand description

Convert arrow struct array to iceberg struct value array. This function will assume the schema of arrow struct array is the same as iceberg struct type.
