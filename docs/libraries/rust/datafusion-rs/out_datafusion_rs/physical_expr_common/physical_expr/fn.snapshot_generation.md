# Function snapshot_generation Copy item path

<a href="https://docs.rs/datafusion-physical-expr-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_expr_common/physical_expr.rs.html#560" class="src">Source</a>

``` rust
pub fn snapshot_generation(expr: &Arc<dyn PhysicalExpr>) -> u64
```

Expand description

Check the generation of this `PhysicalExpr`. Dynamic `PhysicalExpr`s may have a generation that is incremented every time the state of the `PhysicalExpr` changes. If the generation changes that means this `PhysicalExpr` or one of its children has changed since the last time it was evaluated.

This algorithm will not produce collisions as long as the structure of the `PhysicalExpr` does not change and no `PhysicalExpr` decrements its own generation.
