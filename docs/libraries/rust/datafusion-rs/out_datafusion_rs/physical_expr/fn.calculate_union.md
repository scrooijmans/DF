# Function calculate_union Copy item path

<a href="https://docs.rs/datafusion-physical-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_expr/equivalence/properties/union.rs.html#87-90" class="src">Source</a>

``` rust
pub fn calculate_union(
    eqps: Vec<EquivalenceProperties>,
    schema: Arc<Schema>,
) -> Result<EquivalenceProperties, DataFusionError>
```

Expand description

Calculates the union (in the sense of `UnionExec`) `EquivalenceProperties` of the given `EquivalenceProperties` in `eqps` according to the given output `schema` (which need not be the same with those of `lhs` and `rhs` as details such as nullability may be different).
