# Function like_coercion Copy item path

<a href="https://docs.rs/datafusion-expr-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr_common/type_coercion/binary.rs.html#1360" class="src">Source</a>

``` rust
pub fn like_coercion(
    lhs_type: &DataType,
    rhs_type: &DataType,
) -> Option<DataType>
```

Expand description

Coercion rules for like operations. This is a union of string coercion rules and dictionary coercion rules
