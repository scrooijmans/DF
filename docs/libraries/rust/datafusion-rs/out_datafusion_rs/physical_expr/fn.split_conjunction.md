# Function split_conjunction Copy item path

<a href="https://docs.rs/datafusion-physical-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_expr/utils/mod.rs.html#42-44" class="src">Source</a>

``` rust
pub fn split_conjunction(
    predicate: &Arc<dyn PhysicalExpr>,
) -> Vec<&Arc<dyn PhysicalExpr>>
```

Expand description

Assume the predicate is in the form of CNF, split the predicate to a Vec of PhysicalExprs.

For example, split “a1 = a2 AND b1 \<= b2 AND c1 != c2” into \[“a1 = a2”, “b1 \<= b2”, “c1 != c2”\]
