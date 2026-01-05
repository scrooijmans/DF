# Function conjunction_opt Copy item path

<a href="https://docs.rs/datafusion-physical-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_expr/utils/mod.rs.html#62-64" class="src">Source</a>

``` rust
pub fn conjunction_opt(
    predicates: impl IntoIterator<Item = Arc<dyn PhysicalExpr>>,
) -> Option<Arc<dyn PhysicalExpr>>
```

Expand description

Create a conjunction of the given predicates. If the input is empty or the return None. If the input contains a single predicate, return Some(predicate). Otherwise, return a Some(..) of a conjunction of the predicates (e.g. `Some(a AND b AND c)`).
