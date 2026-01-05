# Function binary_to_string_coercion Copy item path

<a href="https://docs.rs/datafusion-expr-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr_common/type_coercion/binary.rs.html#1305-1308" class="src">Source</a>

``` rust
pub fn binary_to_string_coercion(
    lhs_type: &DataType,
    rhs_type: &DataType,
) -> Option<DataType>
```

Expand description

Coercion rules for binary (Binary/LargeBinary) to string (Utf8/LargeUtf8): If one argument is binary and the other is a string then coerce to string (e.g. for `like`)
