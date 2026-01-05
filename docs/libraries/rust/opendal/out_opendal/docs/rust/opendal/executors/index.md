# Module executors Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/types/execute/executors/mod.rs.html#18-27" class="src">Source</a>

Expand description

executors module provides implementations for the [`Execute`](https://opendal.apache.org/docs/rust/opendal/trait.Execute.html "trait opendal::Execute") trait for widely used runtimes.

Every executor will be hide behind the feature like `executors-xxx`. Users can switch or enable the executors they want by enabling the corresponding feature. Also, users can provide their own executor by implementing the [`Execute`](https://opendal.apache.org/docs/rust/opendal/trait.Execute.html "trait opendal::Execute") trait directly.

## Structs<a href="https://opendal.apache.org/docs/rust/opendal/executors/index.html#structs" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/executors/struct.TokioExecutor.html" class="struct" title="struct opendal::executors::TokioExecutor">TokioExecutor</a>`executors-tokio`  
Executor that uses the \[`tokio::task::spawn`\] to execute futures.
