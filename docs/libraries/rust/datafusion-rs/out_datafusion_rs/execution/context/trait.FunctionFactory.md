# Trait FunctionFactory Copy item path

<a href="https://docs.rs/datafusion/50.2.0/src/datafusion/execution/context/mod.rs.html#1774-1781" class="src">Source</a>

``` rust
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

A pluggable interface to handle `CREATE FUNCTION` statements and interact with [SessionState](https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html "struct datafusion::execution::session_state::SessionState") to registers new udf, udaf or udwf.

## Required Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/trait.FunctionFactory.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/trait.FunctionFactory.html#tymethod.create" class="fn">create</a>\<'life0, 'life1, 'async_trait\>( &'life0 self, state: &'life1 <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html" class="struct" title="struct datafusion::execution::session_state::SessionState">SessionState</a>, statement: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.CreateFunction.html" class="struct" title="struct datafusion::logical_expr::CreateFunction">CreateFunction</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/enum.RegisterFunction.html" class="enum" title="enum datafusion::execution::context::RegisterFunction">RegisterFunction</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where Self: 'async_trait, 'life0: 'async_trait, 'life1: 'async_trait,

Handles creation of user defined function specified in [CreateFunction](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.CreateFunction.html "struct datafusion::logical_expr::CreateFunction") statement

## Implementors<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/trait.FunctionFactory.html#implementors" class="anchor">§</a>
