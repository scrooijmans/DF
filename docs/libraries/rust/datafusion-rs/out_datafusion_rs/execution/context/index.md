# Module context Copy item path

<a href="https://docs.rs/datafusion/50.2.0/src/datafusion/execution/context/mod.rs.html#18-2447" class="src">Source</a>

Expand description

[`SessionContext`](https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html "struct datafusion::execution::context::SessionContext") API for registering data sources and executing queries

## Re-exports<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/index.html#reexports" class="anchor">§</a>

`pub use crate::execution::session_state::`<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html" class="struct" title="struct datafusion::execution::session_state::SessionState"><code>SessionState</code></a>`;`

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.EmptySerializerRegistry.html" class="struct" title="struct datafusion::execution::context::EmptySerializerRegistry">EmptySerializerRegistry</a>  
Default implementation of [SerializerRegistry](https://docs.rs/datafusion/50.2.0/datafusion/execution/registry/trait.SerializerRegistry.html "trait datafusion::execution::registry::SerializerRegistry") that throws unimplemented error for all requests.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.ExecutionProps.html" class="struct" title="struct datafusion::execution::context::ExecutionProps">ExecutionProps</a>  
Holds per-query execution properties and data (such as statement starting timestamps).

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SQLOptions.html" class="struct" title="struct datafusion::execution::context::SQLOptions">SQLOptions</a>  
Describes which SQL statements can be run.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionConfig.html" class="struct" title="struct datafusion::execution::context::SessionConfig">SessionConfig</a>  
Configuration options for [`SessionContext`](https://docs.rs/datafusion/latest/datafusion/execution/context/struct.SessionContext.html).

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html" class="struct" title="struct datafusion::execution::context::SessionContext">SessionContext</a>  
Main interface for executing queries with DataFusion. Maintains the state of the connection between a user and an instance of the DataFusion engine.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.TaskContext.html" class="struct" title="struct datafusion::execution::context::TaskContext">TaskContext</a>  
Task Execution Context

## Enums<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/index.html#enums" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/enum.RegisterFunction.html" class="enum" title="enum datafusion::execution::context::RegisterFunction">RegisterFunction</a>  
Type of function to create

## Traits<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/index.html#traits" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/trait.DataFilePaths.html" class="trait" title="trait datafusion::execution::context::DataFilePaths">DataFilePaths</a>  
DataFilePaths adds a method to convert strings and vector of strings to vector of [`ListingTableUrl`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableUrl.html "struct datafusion::datasource::listing::ListingTableUrl") URLs. This allows methods such [`SessionContext::read_csv`](https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.read_csv "method datafusion::execution::context::SessionContext::read_csv") and [`SessionContext::read_avro`](https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html#method.read_avro "method datafusion::execution::context::SessionContext::read_avro") to take either a single file or multiple files.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/trait.FunctionFactory.html" class="trait" title="trait datafusion::execution::context::FunctionFactory">FunctionFactory</a>  
A pluggable interface to handle `CREATE FUNCTION` statements and interact with [SessionState](https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html "struct datafusion::execution::session_state::SessionState") to registers new udf, udaf or udwf.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/trait.QueryPlanner.html" class="trait" title="trait datafusion::execution::context::QueryPlanner">QueryPlanner</a>  
A planner used to add extensions to DataFusion logical and physical plans.
