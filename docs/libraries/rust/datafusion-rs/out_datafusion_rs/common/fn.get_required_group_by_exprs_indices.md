# Function get_required_group_by_exprs_indices Copy item path

<a href="https://docs.rs/datafusion-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_common/functional_dependencies.rs.html#549-552" class="src">Source</a>

``` rust
pub fn get_required_group_by_exprs_indices(
    schema: &DFSchema,
    group_by_expr_names: &[String],
) -> Option<Vec<usize>>
```

Expand description

Returns indices for the minimal subset of GROUP BY expressions that are functionally equivalent to the original set of GROUP BY expressions.
