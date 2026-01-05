# QueryPlanner in datafusion::execution::context - Rust

## Trait QueryPlanner 

[Source](about:blank/src/datafusion/execution/context/mod.rs.html#1761-1768)

```
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
