# Module session_state Copy item path

<a href="https://docs.rs/datafusion/50.2.0/src/datafusion/execution/session_state.rs.html#18-2303" class="src">Source</a>

Expand description

[`SessionState`](https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html "struct datafusion::execution::session_state::SessionState"): information required to run queries in a session

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html" class="struct" title="struct datafusion::execution::session_state::SessionState">SessionState</a>  
`SessionState` contains all the necessary state to plan and execute queries, such as configuration, functions, and runtime environment. Please see the documentation on [`SessionContext`](https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html "struct datafusion::execution::context::SessionContext") for more information.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionStateBuilder.html" class="struct" title="struct datafusion::execution::session_state::SessionStateBuilder">SessionStateBuilder</a>  
A builder to be used for building [`SessionState`](https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html "struct datafusion::execution::session_state::SessionState")’s. Defaults will be used for all values unless explicitly provided.
