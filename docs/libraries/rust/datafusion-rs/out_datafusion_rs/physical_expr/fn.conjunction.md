# Function conjunction Copy item path

<a href="https://docs.rs/datafusion-physical-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_expr/utils/mod.rs.html#52-54" class="src">Source</a>

``` rust
pub fn conjunction(
    predicates: impl IntoIterator<Item = Arc<dyn PhysicalExpr>>,
) -> Arc<dyn PhysicalExpr>
```

Expand description

Create a conjunction of the given predicates. If the input is empty, return a literal true. If the input contains a single predicate, return the predicate. Otherwise, return a conjunction of the predicates (e.g. `a AND b AND c`).
