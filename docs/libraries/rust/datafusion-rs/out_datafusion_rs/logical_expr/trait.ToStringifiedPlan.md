# Trait ToStringifiedPlan Copy item path

<a href="https://docs.rs/datafusion-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_common/display/mod.rs.html#133" class="src">Source</a>

``` rust
pub trait ToStringifiedPlan {
    // Required method
    fn to_stringified(&self, plan_type: PlanType) -> StringifiedPlan;
}
```

Expand description

Trait for something that can be formatted as a stringified plan

## Required Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ToStringifiedPlan.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ToStringifiedPlan.html#tymethod.to_stringified" class="fn">to_stringified</a>(&self, plan_type: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.PlanType.html" class="enum" title="enum datafusion::logical_expr::PlanType">PlanType</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.StringifiedPlan.html" class="struct" title="struct datafusion::logical_expr::StringifiedPlan">StringifiedPlan</a>

Create a stringified plan with the specified type

## Implementors<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ToStringifiedPlan.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ToStringifiedPlan.html#impl-ToStringifiedPlan-for-LogicalPlan" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ToStringifiedPlan.html" class="trait" title="trait datafusion::logical_expr::ToStringifiedPlan">ToStringifiedPlan</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html" class="enum" title="enum datafusion::logical_expr::LogicalPlan">LogicalPlan</a>
