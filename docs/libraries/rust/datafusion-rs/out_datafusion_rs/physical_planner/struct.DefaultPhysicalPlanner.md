# Struct DefaultPhysicalPlanner Copy item path

<a href="https://docs.rs/datafusion/50.2.0/src/datafusion/physical_planner.rs.html#173-175" class="src">Source</a>

``` rust
pub struct DefaultPhysicalPlanner { /* private fields */ }
```

Expand description

Default single node physical query planner that converts a `LogicalPlan` to an `ExecutionPlan` suitable for execution.

This planner will first flatten the `LogicalPlan` tree via a depth first approach, which allows it to identify the leaves of the tree.

Tasks are spawned from these leaves and traverse back up the tree towards the root, converting each `LogicalPlan` node it reaches into their equivalent `ExecutionPlan` node. When these tasks reach a common node, they will terminate until the last task reaches the node which will then continue building up the tree.

Up to [`planning_concurrency`](https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ExecutionOptions.html#structfield.planning_concurrency "field datafusion::config::ExecutionOptions::planning_concurrency") tasks are buffered at once to execute concurrently.

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_planner/struct.DefaultPhysicalPlanner.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_planner/struct.DefaultPhysicalPlanner.html#impl-DefaultPhysicalPlanner" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_planner/struct.DefaultPhysicalPlanner.html" class="struct" title="struct datafusion::physical_planner::DefaultPhysicalPlanner">DefaultPhysicalPlanner</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_planner/struct.DefaultPhysicalPlanner.html#method.with_extension_planners" class="fn">with_extension_planners</a>( extension_planners: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_planner/trait.ExtensionPlanner.html" class="trait" title="trait datafusion::physical_planner::ExtensionPlanner">ExtensionPlanner</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a>\>\>, ) -\> Self

Create a physical planner that uses `extension_planners` to plan user-defined logical nodes [`LogicalPlan::Extension`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html#variant.Extension "variant datafusion::logical_expr::LogicalPlan::Extension"). The planner uses the first [`ExtensionPlanner`](https://docs.rs/datafusion/50.2.0/datafusion/physical_planner/trait.ExtensionPlanner.html "trait datafusion::physical_planner::ExtensionPlanner") to return a non-`None` plan.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_planner/struct.DefaultPhysicalPlanner.html#impl-DefaultPhysicalPlanner-1" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_planner/struct.DefaultPhysicalPlanner.html" class="struct" title="struct datafusion::physical_planner::DefaultPhysicalPlanner">DefaultPhysicalPlanner</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_planner/struct.DefaultPhysicalPlanner.html#method.optimize_physical_plan" class="fn">optimize_physical_plan</a>\<F\>( &self, plan: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a>\>, session_state: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html" class="struct" title="struct datafusion::execution::session_state::SessionState">SessionState</a>, observer: F, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a>\>\>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(&dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a>, &dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/trait.PhysicalOptimizerRule.html" class="trait" title="trait datafusion::physical_optimizer::PhysicalOptimizerRule">PhysicalOptimizerRule</a>),

Optimize a physical plan by applying each physical optimizer, calling observer(plan, optimizer after each one)

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_planner/struct.DefaultPhysicalPlanner.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_planner/struct.DefaultPhysicalPlanner.html#impl-Default-for-DefaultPhysicalPlanner" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_planner/struct.DefaultPhysicalPlanner.html" class="struct" title="struct datafusion::physical_planner::DefaultPhysicalPlanner">DefaultPhysicalPlanner</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_planner/struct.DefaultPhysicalPlanner.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_planner/struct.DefaultPhysicalPlanner.html" class="struct" title="struct datafusion::physical_planner::DefaultPhysicalPlanner">DefaultPhysicalPlanner</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_planner/struct.DefaultPhysicalPlanner.html#impl-PhysicalPlanner-for-DefaultPhysicalPlanner" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_planner/trait.PhysicalPlanner.html" class="trait" title="trait datafusion::physical_planner::PhysicalPlanner">PhysicalPlanner</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_planner/struct.DefaultPhysicalPlanner.html" class="struct" title="struct datafusion::physical_planner::DefaultPhysicalPlanner">DefaultPhysicalPlanner</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_planner/struct.DefaultPhysicalPlanner.html#method.create_physical_plan" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_planner/trait.PhysicalPlanner.html#tymethod.create_physical_plan" class="fn">create_physical_plan</a>\<'life0, 'life1, 'life2, 'async_trait\>( &'life0 self, logical_plan: &'life1 <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html" class="enum" title="enum datafusion::logical_expr::LogicalPlan">LogicalPlan</a>, session_state: &'life2 <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html" class="struct" title="struct datafusion::execution::session_state::SessionState">SessionState</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a>\>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where Self: 'async_trait, 'life0: 'async_trait, 'life1: 'async_trait, 'life2: 'async_trait,

Create a physical plan from a logical plan

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_planner/struct.DefaultPhysicalPlanner.html#method.create_physical_expr" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_planner/trait.PhysicalPlanner.html#tymethod.create_physical_expr" class="fn">create_physical_expr</a>( &self, expr: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>, input_dfschema: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html" class="struct" title="struct datafusion::common::DFSchema">DFSchema</a>, session_state: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html" class="struct" title="struct datafusion::execution::session_state::SessionState">SessionState</a>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html" class="trait" title="trait datafusion::physical_expr::PhysicalExpr">PhysicalExpr</a>\>\>

Create a physical expression from a logical expression suitable for evaluation

`e`: the expression to convert

`input_dfschema`: the logical plan schema for evaluating `e`

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_planner/struct.DefaultPhysicalPlanner.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_planner/struct.DefaultPhysicalPlanner.html#blanket-implementations" class="anchor">§</a>
