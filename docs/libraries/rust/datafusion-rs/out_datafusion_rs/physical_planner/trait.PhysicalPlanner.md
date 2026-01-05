# Trait PhysicalPlanner Copy item path

<a href="https://docs.rs/datafusion/50.2.0/src/datafusion/physical_planner.rs.html#109-129" class="src">Source</a>

``` rust
pub trait PhysicalPlanner: Send + Sync {
    // Required methods
    fn create_physical_plan<'life0, 'life1, 'life2, 'async_trait>(
        &'life0 self,
        logical_plan: &'life1 LogicalPlan,
        session_state: &'life2 SessionState,
    ) -> Pin<Box<dyn Future<Output = Result<Arc<dyn ExecutionPlan>>> + Send + 'async_trait>>
       where Self: 'async_trait,
             'life0: 'async_trait,
             'life1: 'async_trait,
             'life2: 'async_trait;
    fn create_physical_expr(
        &self,
        expr: &Expr,
        input_dfschema: &DFSchema,
        session_state: &SessionState,
    ) -> Result<Arc<dyn PhysicalExpr>>;
}
```

Expand description

Physical query planner that converts a `LogicalPlan` to an `ExecutionPlan` suitable for execution.

## Required Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_planner/trait.PhysicalPlanner.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_planner/trait.PhysicalPlanner.html#tymethod.create_physical_plan" class="fn">create_physical_plan</a>\<'life0, 'life1, 'life2, 'async_trait\>( &'life0 self, logical_plan: &'life1 <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html" class="enum" title="enum datafusion::logical_expr::LogicalPlan">LogicalPlan</a>, session_state: &'life2 <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html" class="struct" title="struct datafusion::execution::session_state::SessionState">SessionState</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a>\>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where Self: 'async_trait, 'life0: 'async_trait, 'life1: 'async_trait, 'life2: 'async_trait,

Create a physical plan from a logical plan

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_planner/trait.PhysicalPlanner.html#tymethod.create_physical_expr" class="fn">create_physical_expr</a>( &self, expr: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>, input_dfschema: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html" class="struct" title="struct datafusion::common::DFSchema">DFSchema</a>, session_state: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html" class="struct" title="struct datafusion::execution::session_state::SessionState">SessionState</a>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html" class="trait" title="trait datafusion::physical_expr::PhysicalExpr">PhysicalExpr</a>\>\>

Create a physical expression from a logical expression suitable for evaluation

`expr`: the expression to convert

`input_dfschema`: the logical plan schema for evaluating `expr`

## Implementors<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_planner/trait.PhysicalPlanner.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_planner/trait.PhysicalPlanner.html#impl-PhysicalPlanner-for-DefaultPhysicalPlanner" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_planner/trait.PhysicalPlanner.html" class="trait" title="trait datafusion::physical_planner::PhysicalPlanner">PhysicalPlanner</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_planner/struct.DefaultPhysicalPlanner.html" class="struct" title="struct datafusion::physical_planner::DefaultPhysicalPlanner">DefaultPhysicalPlanner</a>
