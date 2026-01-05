# Module runtime Copy item path

<a href="https://docs.rs/datafusion/50.2.0/src/datafusion/lib.rs.html#766" class="src">Source</a>

Expand description

re-export of [`datafusion_common_runtime`](https://docs.rs/datafusion-common-runtime/50.2.0/x86_64-unknown-linux-gnu/datafusion_common_runtime/index.html "mod datafusion_common_runtime") crate

## Modules<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/runtime/index.html#modules" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/runtime/common/index.html" class="mod" title="mod datafusion::common::runtime::common">common</a>

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/runtime/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/runtime/struct.JoinSet.html" class="struct" title="struct datafusion::common::runtime::JoinSet">JoinSet</a>  
A wrapper around Tokio’s JoinSet that forwards all API calls while optionally instrumenting spawned tasks and blocking closures with custom tracing behavior. If no tracer is injected via `trace_utils::set_tracer`, tasks and closures are executed without any instrumentation.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/runtime/struct.SpawnedTask.html" class="struct" title="struct datafusion::common::runtime::SpawnedTask">SpawnedTask</a>  
Helper that provides a simple API to spawn a single task and join it. Provides guarantees of aborting on `Drop` to keep it cancel-safe. Note that if the task was spawned with `spawn_blocking`, it will only be aborted if it hasn’t started yet.

## Traits<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/runtime/index.html#traits" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/runtime/trait.JoinSetTracer.html" class="trait" title="trait datafusion::common::runtime::JoinSetTracer">JoinSetTracer</a>  
A trait for injecting instrumentation into either asynchronous futures or blocking closures at runtime.

## Functions<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/runtime/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/runtime/fn.set_join_set_tracer.html" class="fn" title="fn datafusion::common::runtime::set_join_set_tracer">set_join_set_tracer</a>  
Set the custom tracer for both futures and blocking closures.
