# Module execution Copy item path

<a href="https://docs.rs/datafusion/50.2.0/src/datafusion/execution/mod.rs.html#18-30" class="src">Source</a>

Expand description

Shared state for query planning and execution.

## Re-exports<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/index.html#reexports" class="anchor">§</a>

`pub use session_state::`<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html" class="struct" title="struct datafusion::execution::session_state::SessionState"><code>SessionState</code></a>`;`

`pub use session_state::`<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionStateBuilder.html" class="struct" title="struct datafusion::execution::session_state::SessionStateBuilder"><code>SessionStateBuilder</code></a>`;`

`pub use crate::datasource::file_format::`<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/index.html" class="mod" title="mod datafusion::datasource::file_format::options"><code>options</code></a>`;`

## Modules<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/index.html#modules" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/cache/index.html" class="mod" title="mod datafusion::execution::cache">cache</a>  
<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/config/index.html" class="mod" title="mod datafusion::execution::config">config</a>  
<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/index.html" class="mod" title="mod datafusion::execution::context">context</a>  
[`SessionContext`](https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html "struct datafusion::execution::context::SessionContext") API for registering data sources and executing queries

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/disk_manager/index.html" class="mod" title="mod datafusion::execution::disk_manager">disk_manager</a>  
[`DiskManager`](https://docs.rs/datafusion/50.2.0/datafusion/execution/struct.DiskManager.html "struct datafusion::execution::DiskManager"): Manages files generated during query execution

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/index.html" class="mod" title="mod datafusion::execution::memory_pool">memory_pool</a>  
[`MemoryPool`](https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/trait.MemoryPool.html "trait datafusion::execution::memory_pool::MemoryPool") for memory management during query execution, [`proxy`](https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/proxy/index.html "mod datafusion::execution::memory_pool::proxy") for help with allocation accounting.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/object_store/index.html" class="mod" title="mod datafusion::execution::object_store">object_store</a>  
ObjectStoreRegistry holds all the object stores at Runtime with a scheme for each store. This allows the user to extend DataFusion with different storage systems such as S3 or HDFS and query data inside these systems.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/parquet_encryption/index.html" class="mod" title="mod datafusion::execution::parquet_encryption">parquet_encryption</a>  
<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/registry/index.html" class="mod" title="mod datafusion::execution::registry">registry</a>  
<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/runtime_env/index.html" class="mod" title="mod datafusion::execution::runtime_env">runtime_env</a>  
Execution [`RuntimeEnv`](https://docs.rs/datafusion/50.2.0/datafusion/execution/runtime_env/struct.RuntimeEnv.html "struct datafusion::execution::runtime_env::RuntimeEnv") environment that manages access to object store, memory manager, disk manager.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/index.html" class="mod" title="mod datafusion::execution::session_state">session_state</a>  
[`SessionState`](https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html "struct datafusion::execution::session_state::SessionState"): information required to run queries in a session

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/struct.DiskManager.html" class="struct" title="struct datafusion::execution::DiskManager">DiskManager</a>  
Manages files generated during query execution, e.g. spill files generated while processing dataset larger than available memory.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/struct.SessionStateDefaults.html" class="struct" title="struct datafusion::execution::SessionStateDefaults">SessionStateDefaults</a>  
Defaults that are used as part of creating a SessionState such as table providers, file formats, registering of builtin functions, etc.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/struct.TaskContext.html" class="struct" title="struct datafusion::execution::TaskContext">TaskContext</a>  
Task Execution Context

## Traits<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/index.html#traits" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.FunctionRegistry.html" class="trait" title="trait datafusion::execution::FunctionRegistry">FunctionRegistry</a>  
A registry knows how to build logical expressions out of user-defined function’ names

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.RecordBatchStream.html" class="trait" title="trait datafusion::execution::RecordBatchStream">RecordBatchStream</a>  
Trait for types that stream [RecordBatch](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html "struct datafusion::common::arrow::array::RecordBatch")

## Type Aliases<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/index.html#types" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/type.SendableRecordBatchStream.html" class="type" title="type datafusion::execution::SendableRecordBatchStream">SendableRecordBatchStream</a>  
Trait for a [`Stream`](https://docs.rs/futures-core/0.3.31/x86_64-unknown-linux-gnu/futures_core/stream/trait.Stream.html "trait futures_core::stream::Stream") of [`RecordBatch`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html "struct datafusion::common::arrow::array::RecordBatch")es that can be passed between threads
