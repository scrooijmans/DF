# Function get_target_functional_dependencies Copy item path

<a href="https://docs.rs/datafusion-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_common/functional_dependencies.rs.html#513-516" class="src">Source</a>

``` rust
pub fn get_target_functional_dependencies(
    schema: &DFSchema,
    group_by_expr_names: &[String],
) -> Option<Vec<usize>>
```

Expand description

Returns target indices, for the determinant keys that are inside group by expressions.
