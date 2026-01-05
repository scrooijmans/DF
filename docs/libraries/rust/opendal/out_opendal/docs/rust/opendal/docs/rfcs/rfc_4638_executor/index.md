# Module rfc_4638_executor Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/docs/rfcs/mod.rs.html#238" class="src">Source</a>

Available on **`docsrs`** only.

Expand description

Executor API

- Proposal Name: `executor`
- Start Date: 2024-05-23
- RFC PR: [apache/opendal#4638](https://github.com/apache/opendal/pull/4638)
- Tracking Issue: [apache/opendal#4639](https://github.com/apache/opendal/issues/4639)

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_4638_executor/index.html#summary" class="doc-anchor">Â§</a>Summary

Add executor in opendal to allow running tasks concurrently in background.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_4638_executor/index.html#motivation" class="doc-anchor">Â§</a>Motivation

OpenDAL offers top-tier support for concurrent execution, allowing tasks to run simultaneously in the background. Users can easily enable concurrent file read/write operations with just one line of code:

``` diff
 let mut w = op
     .writer_with(path)
     .chunk(8 * 1024 * 1024) // 8 MiB per chunk
+    .concurrent(16) // 16 concurrent tasks
     .await?;
     
 w.write(bs).await?;
 w.write(bs).await?; // The submitted tasks only be executed while user calling `write`.
 ...
 sleep(Duration::from_secs(10)).await; // The submitted tasks make no progress during `sleep`.
 ...
 w.close().await?;
```

However, the execution of those tasks relies on users continuously calling `write`. They cannot run tasks concurrently in the background. (I explained the technical details in the `Rationale and alternatives` section.)

This can result in the following issues:

- Task latency may increase as tasks are not executed until the task queue is full.
- Memory usage may be high because all chunks must be held in memory until the task is completed.

I propose introducing an executor abstraction in OpenDAL to enable concurrent background task execution. The executor will automatically manage the tasks in the background without requiring users to drive the progress manually.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_4638_executor/index.html#guide-level-explanation" class="doc-anchor">Â§</a>Guide-level explanation

OpenDAL will add a new `Executor` struct to manage concurrent tasks.

``` rust
pub struct Executor {
    ...
}

pub struct Task {
    ...
}

impl Executor {
    /// Create a new tokio based executor.
    pub fn new() -> Self { ... }
    
    /// Create a new executor with given execute impl.
    pub fn with(exec: Arc<dyn Execute>) -> Self { ... }

    /// Run given future in background immediately.
    pub fn execute<F>(&self, f: F) -> Task<F::Output>
    where
        F: Future + Send + 'static,
    {
        ...
    }
}
```

The `Executor` uses the `tokio` runtime by default but users can also provide their own runtime by:

``` rust
pub trait Execute {
    fn execute(&self, f: BoxedFuture<()>) -> Result<()>;
}
```

Users can set executor in `OpWrite` / `OpRead` to enable concurrent background task execution:

``` rust
+ let exec = Executor::new();
  let w = op
      .writer_with(path)
      .chunk(8 * 1024 * 1024) // 8 MiB per chunk
      .concurrent(16) // 16 concurrent tasks
+     .executor(exec) // Use specified executor
      .await?;
```

Specifying an executor every time is cumbersome. Users can also set a global executor for given operator:

``` rust
+ let exec = Executor::new();
+ let op = op.with_default_executor(exec);

  let w = op
      .writer_with(path)
      .chunk(8 * 1024 * 1024) // 8 MiB per chunk
      .concurrent(16) // 16 concurrent tasks
      .await?;
```

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_4638_executor/index.html#reference-level-explanation" class="doc-anchor">Â§</a>Reference-level explanation

As mentioned in the `Guide-level explanation`, the `Executor` struct will manage concurrent tasks in the background. `Executor` will be powered by trait `Execute` to support different underlying runtimes. To make trait `Execute` object safe, we only accept `BoxedFuture<()>` as input. `Executor` will handle the future output and return the result to the caller.

Operations that supporting concurrent execution will add a new field:

``` rust
pub struct OpXxx {
    ...
    executor: Option<Executor>,
}
```

Operator will add a new field to store the default executor:

``` rust
pub struct Operator {
    ...
    default_executor: Option<Executor>,
}
```

The `Task` spawned by `Executor` will be a future that can be awaited to fetch the result:

``` rust
let res = task.await;
```

The task will be executed immediately after calling `execute`. Users can also cancel the task by dropping the `Task` object. Users donâ€™t need to poll those `Task` object to make progress.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_4638_executor/index.html#drawbacks" class="doc-anchor">Â§</a>Drawbacks

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_4638_executor/index.html#complexity" class="doc-anchor">Â§</a>Complexity

To support concurrent execution, we need to introduce:

- a new `Executor` struct
- a new `Task` struct
- a new `Execute` trait

This may increase the complexity of the codebase.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_4638_executor/index.html#rationale-and-alternatives" class="doc-anchor">Â§</a>Rationale and alternatives

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_4638_executor/index.html#why-introducing-so-many-new-abstractions" class="doc-anchor">Â§</a>Why introducing so many new abstractions?

We need to introduce new abstractions to support concurrent execution across different runtimes. Unfortunately, this is the current reality of async rust.

Supporting just one or two runtimes by adding features is much easier. Supporting only Tokio is extremely simple, requiring about 10 lines of changes. However, this violates our vision of free data access.

Firstly, we donâ€™t want to force our users to use Tokio. We aim to support all runtimes, including async-std, smol, and others.

Secondly, OpenDAL should be capable of running in any environment, including embedded systems. We donâ€™t want to restrict our users to a specific runtime.

Finally, users may have their own preferences for observability and performance in their runtime. We intend to accommodate these needs effortlessly.

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_4638_executor/index.html#why-concurrentfutures-doesnt-work" class="doc-anchor">Â§</a>Why `ConcurrentFutures` doesnâ€™t work?

`ConcurrentFutures` is a `Vec<impl Future>`, users need to keep calling `poll_next` to make progress. This is not suitable for our use case. We need a way to run tasks in the background without user intervention.

> Iâ€™ve heard that futures will wake up when theyâ€™re ready, and itâ€™s the runtimeâ€™s job to poll them, right?

No, itâ€™s partially correct. The runtime will wake up the future when itâ€™s ready, but itâ€™s the userâ€™s job to poll the future. The runtime will not poll the future automatically unless itâ€™s managed by the runtime.

For tokio, that means all futures provided by tokio, like `tokio::time::Sleep`, will be polled by tokio runtime. However, if you create a future by yourself, you need to poll it manually.

I have an example to explain this:

*Try it at [playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=628e67adef90128151e175d22c87808e)*

``` rust
use futures::stream::FuturesUnordered;
use futures::StreamExt;
use std::time::Duration;
use tokio::time::{sleep, Instant};

#[tokio::main]
async fn main() {
    let now = Instant::now();
    let mut cf = FuturesUnordered::new();

    // cf.push(Box::pin(sleep(Duration::from_secs(3))));
    cf.push(Box::pin(async move {
        sleep(Duration::from_secs(3)).await;
        println!("async task finished at {}s", now.elapsed().as_secs_f64());
    }));
    sleep(Duration::from_secs(4)).await;
    println!("outer sleep finished at {}s", now.elapsed().as_secs_f64());

    let _: Vec<()> = cf.collect().await;
    println!("consumed: {}s", now.elapsed().as_secs_f64())
}
```

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_4638_executor/index.html#prior-art" class="doc-anchor">Â§</a>Prior art

None.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_4638_executor/index.html#unresolved-questions" class="doc-anchor">Â§</a>Unresolved questions

None.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_4638_executor/index.html#future-possibilities" class="doc-anchor">Â§</a>Future possibilities

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_4638_executor/index.html#blocking-executor" class="doc-anchor">Â§</a>Blocking Executor

This proposal mainly focuses on async tasks. However, we can also consider adding blocking support to `Executor`. Users can use concurrent tasks in blocking context too:

``` rust
 let w = op
     .writer_with(path)
     .chunk(8 * 1024 * 1024) // 8 MiB per chunk
+    .concurrent(16) // 16 concurrent tasks
     .do()?;
```
