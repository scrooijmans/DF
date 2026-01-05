# Trait QueryPlanner Copy item path

<a href="https://docs.rs/datafusion/50.2.0/src/datafusion/execution/context/mod.rs.html#1761-1768" class="src">Source</a>

``` rust
pub trait QueryPlanner: Debug {
    // Required method
    fn create_physical_plan<'life0, 'life1, 'life2, 'async_trait>(
        &'life0 self,
        logical_plan: &'life1 LogicalPlan,
        session_state: &'life2 SessionState,
    ) -> Pin<Box<dyn Future<Output = Result<Arc<dyn ExecutionPlan>>> + Send + 'async_trait>>
       where Self: 'async_trait,
             'life0: 'async_trait,
             'life1: 'async_trait,
             'life2: 'async_trait;
}
```

Expand description

A planner used to add extensions to DataFusion logical and physical plans.

## Required Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/trait.QueryPlanner.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/trait.QueryPlanner.html#tymethod.create_physical_plan" class="fn">create_physical_plan</a>\<'life0, 'life1, 'life2, 'async_trait\>( &'life0 self, logical_plan: &'life1 <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html" class="enum" title="enum datafusion::logical_expr::LogicalPlan">LogicalPlan</a>, session_state: &'life2 <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html" class="struct" title="struct datafusion::execution::session_state::SessionState">SessionState</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a>\>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where Self: 'async_trait, 'life0: 'async_trait, 'life1: 'async_trait, 'life2: 'async_trait,

Given a `LogicalPlan`, create an [`ExecutionPlan`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html "trait datafusion::physical_plan::ExecutionPlan") suitable for execution

## Implementors<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/trait.QueryPlanner.html#implementors" class="anchor">§</a>
