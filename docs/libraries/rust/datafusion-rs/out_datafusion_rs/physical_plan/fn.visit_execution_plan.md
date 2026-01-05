# Function visit_execution_plan Copy item path

<a href="https://docs.rs/datafusion-physical-plan/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_plan/visitor.rs.html#84-87" class="src">Source</a>

``` rust
pub fn visit_execution_plan<V>(
    plan: &dyn ExecutionPlan,
    visitor: &mut V,
) -> Result<(), <V as ExecutionPlanVisitor>::Error>where
    V: ExecutionPlanVisitor,
```

Expand description

Recursively calls `pre_visit` and `post_visit` for this node and all of its children, as described on [`ExecutionPlanVisitor`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlanVisitor.html "trait datafusion::physical_plan::ExecutionPlanVisitor")
