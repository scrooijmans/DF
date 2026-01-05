# Function arrow_primitive_to_literal Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/arrow/value.rs.html#610-620" class="src">Source</a>

``` rust
pub fn arrow_primitive_to_literal(
    primitive_array: &ArrayRef,
    ty: &Type,
) -> Result<Vec<Option<Literal>>>
```

Expand description

Convert arrow primitive array to iceberg primitive value array. This function will assume the schema of arrow struct array is the same as iceberg struct type.
