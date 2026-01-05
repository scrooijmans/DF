# Trait ExtensionPlanner Copy item path

<a href="https://docs.rs/datafusion/50.2.0/src/datafusion/physical_planner.rs.html#133-152" class="src">Source</a>

``` rust
pub trait ExtensionPlanner {
    // Required method
    fn plan_extension<'life0, 'life1, 'life2, 'life3, 'life4, 'life5, 'life6, 'async_trait>(
        &'life0 self,
        planner: &'life1 dyn PhysicalPlanner,
        node: &'life2 dyn UserDefinedLogicalNode,
        logical_inputs: &'life3 [&'life4 LogicalPlan],
        physical_inputs: &'life5 [Arc<dyn ExecutionPlan>],
        session_state: &'life6 SessionState,
    ) -> Pin<Box<dyn Future<Output = Result<Option<Arc<dyn ExecutionPlan>>>> + Send + 'async_trait>>
       where Self: 'async_trait,
             'life0: 'async_trait,
             'life1: 'async_trait,
             'life2: 'async_trait,
             'life3: 'async_trait,
             'life4: 'async_trait,
             'life5: 'async_trait,
             'life6: 'async_trait;
}
```

Expand description

This trait exposes the ability to plan an [`ExecutionPlan`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html "trait datafusion::physical_plan::ExecutionPlan") out of a [`LogicalPlan`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html "enum datafusion::logical_expr::LogicalPlan").

## Required Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_planner/trait.ExtensionPlanner.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_planner/trait.ExtensionPlanner.html#tymethod.plan_extension" class="fn">plan_extension</a>\<'life0, 'life1, 'life2, 'life3, 'life4, 'life5, 'life6, 'async_trait\>( &'life0 self, planner: &'life1 dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_planner/trait.PhysicalPlanner.html" class="trait" title="trait datafusion::physical_planner::PhysicalPlanner">PhysicalPlanner</a>, node: &'life2 dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.UserDefinedLogicalNode.html" class="trait" title="trait datafusion::logical_expr::UserDefinedLogicalNode">UserDefinedLogicalNode</a>, logical_inputs: &'life3 \[&'life4 <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html" class="enum" title="enum datafusion::logical_expr::LogicalPlan">LogicalPlan</a>\], physical_inputs: &'life5 \[<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a>\>\], session_state: &'life6 <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html" class="struct" title="struct datafusion::execution::session_state::SessionState">SessionState</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a>\>\>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where Self: 'async_trait, 'life0: 'async_trait, 'life1: 'async_trait, 'life2: 'async_trait, 'life3: 'async_trait, 'life4: 'async_trait, 'life5: 'async_trait, 'life6: 'async_trait,

Create a physical plan for a [`UserDefinedLogicalNode`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.UserDefinedLogicalNode.html "trait datafusion::logical_expr::UserDefinedLogicalNode").

`input_dfschema`: the logical plan schema for the inputs to this node

Returns an error when the planner knows how to plan the concrete implementation of `node` but errors while doing so.

Returns `None` when the planner does not know how to plan the `node` and wants to delegate the planning to another [`ExtensionPlanner`](https://docs.rs/datafusion/50.2.0/datafusion/physical_planner/trait.ExtensionPlanner.html "trait datafusion::physical_planner::ExtensionPlanner").

## Implementors<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_planner/trait.ExtensionPlanner.html#implementors" class="anchor">§</a>
