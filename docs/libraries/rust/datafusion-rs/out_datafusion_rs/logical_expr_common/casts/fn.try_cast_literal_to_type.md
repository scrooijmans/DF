# Function try_cast_literal_to_type Copy item path

<a href="https://docs.rs/datafusion-expr-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr_common/casts.rs.html#34-37" class="src">Source</a>

``` rust
pub fn try_cast_literal_to_type(
    lit_value: &ScalarValue,
    target_type: &DataType,
) -> Option<ScalarValue>
```

Expand description

Convert a literal value from one data type to another
