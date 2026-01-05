# Function accept Copy item path

<a href="https://docs.rs/datafusion-physical-plan/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_plan/visitor.rs.html#24-27" class="src">Source</a>

``` rust
pub fn accept<V>(
    plan: &dyn ExecutionPlan,
    visitor: &mut V,
) -> Result<(), <V as ExecutionPlanVisitor>::Error>where
    V: ExecutionPlanVisitor,
```

Expand description

Visit all children of this plan, according to the order defined on `ExecutionPlanVisitor`.
