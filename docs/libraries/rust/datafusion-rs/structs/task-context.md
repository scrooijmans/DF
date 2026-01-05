# TaskContext in datafusion::execution::context - Rust

## Struct TaskContext 

[Source](https://docs.rs/datafusion-execution/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_execution/task.rs.html#36)

```
pub struct TaskContext { /* private fields */ }
```

Expand description

Task Execution Context

A [`TaskContext`](../struct.TaskContext.html "struct datafusion::execution::TaskContext") contains the state required during a single query’s execution. Please see the documentation on [`SessionContext`](https://docs.rs/datafusion/latest/datafusion/execution/context/struct.SessionContext.html) for more information.

[Source](https://docs.rs/datafusion-execution/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_execution/task.rs.html#35)
[§](#impl-Debug-for-TaskContext)

[Source](https://docs.rs/datafusion-execution/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_execution/task.rs.html#53)
[§](#impl-Default-for-TaskContext)

[Source](about:blank/src/datafusion/execution/context/mod.rs.html#1741-1745)
[§](#impl-From%3C%26SessionContext%3E-for-TaskContext)

Create a new task context instance from SessionContext

[Source](about:blank/src/datafusion/execution/context/mod.rs.html#1742-1744)
[§](#method.from-1)

Converts to this type from the input type.

[Source](about:blank/src/datafusion/execution/session_state.rs.html#1907-1920)
[§](#impl-From%3C%26SessionState%3E-for-TaskContext)

Create a new task context instance from SessionState

[Source](about:blank/src/datafusion/execution/session_state.rs.html#1908-1919)
[§](#method.from-2)

Converts to this type from the input type.

[Source](https://docs.rs/datafusion-session/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_session/session.rs.html#141)
[§](#impl-From%3C%26dyn+Session%3E-for-TaskContext)

Create a new task context instance from Session

[Source](https://docs.rs/datafusion-session/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_session/session.rs.html#142)
[§](#method.from)

Converts to this type from the input type.

[Source](https://docs.rs/datafusion-execution/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_execution/task.rs.html#146)
[§](#impl-FunctionRegistry-for-TaskContext)

[§](#impl-Freeze-for-TaskContext)

[§](#impl-RefUnwindSafe-for-TaskContext)

[§](#impl-Send-for-TaskContext)

[§](#impl-Sync-for-TaskContext)

[§](#impl-Unpin-for-TaskContext)

[§](#impl-UnwindSafe-for-TaskContext)
