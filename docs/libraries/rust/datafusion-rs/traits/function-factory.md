# FunctionFactory in datafusion::execution::context - Rust

## Trait FunctionFactory 

[Source](about:blank/src/datafusion/execution/context/mod.rs.html#1774-1781)

```
pub trait FunctionFactory:
    Debug
    + Sync
    + Send {
    // Required method
    fn create<'life0, 'life1, 'async_trait>(
        &'life0 self,
        state: &'life1 SessionState,
        statement: CreateFunction,
    ) -> Pin<Box<dyn Future<Output = Result<RegisterFunction>> + Send + 'async_trait>>
       where Self: 'async_trait,
             'life0: 'async_trait,
             'life1: 'async_trait;
}
```

Expand description

A pluggable interface to handle `CREATE FUNCTION` statements and interact with [SessionState](../session_state/struct.SessionState.html "struct datafusion::execution::session_state::SessionState") to registers new udf, udaf or udwf.
