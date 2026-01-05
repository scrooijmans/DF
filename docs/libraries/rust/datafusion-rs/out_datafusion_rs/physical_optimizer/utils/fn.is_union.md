# Function is_union Copy item path

<a href="https://docs.rs/datafusion-physical-optimizer/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_optimizer/utils.rs.html#92" class="src">Source</a>

``` rust
pub fn is_union(plan: &Arc<dyn ExecutionPlan>) -> bool
```

Expand description

Checks whether the given operator is a [`UnionExec`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/union/struct.UnionExec.html "struct datafusion::physical_plan::union::UnionExec").
