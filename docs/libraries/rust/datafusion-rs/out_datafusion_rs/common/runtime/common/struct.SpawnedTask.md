# Struct SpawnedTask Copy item path

<a href="https://docs.rs/datafusion-common-runtime/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_common_runtime/common.rs.html#35" class="src">Source</a>

``` rust
pub struct SpawnedTask<R> { /* private fields */ }
```

Expand description

Helper that provides a simple API to spawn a single task and join it. Provides guarantees of aborting on `Drop` to keep it cancel-safe. Note that if the task was spawned with `spawn_blocking`, it will only be aborted if it hasn’t started yet.

Technically, it’s just a wrapper of a `JoinHandle` overriding drop.

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/runtime/common/struct.SpawnedTask.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/runtime/common/struct.SpawnedTask.html#impl-SpawnedTask%3CR%3E" class="anchor">§</a>

### impl\<R\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/runtime/struct.SpawnedTask.html" class="struct" title="struct datafusion::common::runtime::SpawnedTask">SpawnedTask</a>\<R\>

where R: 'static,

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/runtime/common/struct.SpawnedTask.html#method.spawn" class="fn">spawn</a>\<T\>(task: T) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/runtime/struct.SpawnedTask.html" class="struct" title="struct datafusion::common::runtime::SpawnedTask">SpawnedTask</a>\<R\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/runtime/common/struct.SpawnedTask.html#" class="tooltip" data-notable-ty="SpawnedTask&lt;R&gt;">ⓘ</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = R\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'static, R: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>,

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/runtime/common/struct.SpawnedTask.html#method.spawn_blocking" class="fn">spawn_blocking</a>\<T\>(task: T) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/runtime/struct.SpawnedTask.html" class="struct" title="struct datafusion::common::runtime::SpawnedTask">SpawnedTask</a>\<R\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/runtime/common/struct.SpawnedTask.html#" class="tooltip" data-notable-ty="SpawnedTask&lt;R&gt;">ⓘ</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html" class="trait" title="trait core::ops::function::FnOnce">FnOnce</a>() -\> R + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'static, R: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>,

#### pub async fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/runtime/common/struct.SpawnedTask.html#method.join" class="fn">join</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<R, <a href="https://docs.rs/tokio/1.47.1/x86_64-unknown-linux-gnu/tokio/runtime/task/error/struct.JoinError.html" class="struct" title="struct tokio::runtime::task::error::JoinError">JoinError</a>\>

Joins the task, returning the result of join (`Result<R, JoinError>`). Same as awaiting the spawned task, but left for backwards compatibility.

#### pub async fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/runtime/common/struct.SpawnedTask.html#method.join_unwind" class="fn">join_unwind</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<R, <a href="https://docs.rs/tokio/1.47.1/x86_64-unknown-linux-gnu/tokio/runtime/task/error/struct.JoinError.html" class="struct" title="struct tokio::runtime::task::error::JoinError">JoinError</a>\>

Joins the task and unwinds the panic if it happens.

#### pub async fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/runtime/common/struct.SpawnedTask.html#method.join_unwind_mut" class="fn">join_unwind_mut</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<R, <a href="https://docs.rs/tokio/1.47.1/x86_64-unknown-linux-gnu/tokio/runtime/task/error/struct.JoinError.html" class="struct" title="struct tokio::runtime::task::error::JoinError">JoinError</a>\>

Joins the task using a mutable reference and unwinds the panic if it happens.

This method is similar to [`join_unwind`](https://docs.rs/datafusion/50.2.0/datafusion/common/runtime/struct.SpawnedTask.html#method.join_unwind "method datafusion::common::runtime::SpawnedTask::join_unwind"), but takes a mutable reference instead of consuming `self`. This allows the `SpawnedTask` to remain usable after the call.

If called multiple times on the same task:

- If the task is still running, it will continue waiting for completion
- If the task has already completed successfully, subsequent calls will continue to return the same `JoinError` indicating the task is finished
- If the task panicked, the first call will resume the panic, and the program will not reach subsequent calls

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/runtime/common/struct.SpawnedTask.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/runtime/common/struct.SpawnedTask.html#impl-Debug-for-SpawnedTask%3CR%3E" class="anchor">§</a>

### impl\<R\> <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/runtime/struct.SpawnedTask.html" class="struct" title="struct datafusion::common::runtime::SpawnedTask">SpawnedTask</a>\<R\>

where R: <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/runtime/common/struct.SpawnedTask.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/runtime/common/struct.SpawnedTask.html#impl-Drop-for-SpawnedTask%3CR%3E" class="anchor">§</a>

### impl\<R\> <a href="https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html" class="trait" title="trait core::ops::drop::Drop">Drop</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/runtime/struct.SpawnedTask.html" class="struct" title="struct datafusion::common::runtime::SpawnedTask">SpawnedTask</a>\<R\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/runtime/common/struct.SpawnedTask.html#method.drop" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html#tymethod.drop" class="fn">drop</a>(&mut self)

Executes the destructor for this type. [Read more](https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html#tymethod.drop)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/runtime/common/struct.SpawnedTask.html#impl-Future-for-SpawnedTask%3CR%3E" class="anchor">§</a>

### impl\<R\> <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/runtime/struct.SpawnedTask.html" class="struct" title="struct datafusion::common::runtime::SpawnedTask">SpawnedTask</a>\<R\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/runtime/common/struct.SpawnedTask.html#associatedtype.Output" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<R, <a href="https://docs.rs/tokio/1.47.1/x86_64-unknown-linux-gnu/tokio/runtime/task/error/struct.JoinError.html" class="struct" title="struct tokio::runtime::task::error::JoinError">JoinError</a>\>

The type of value produced on completion.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/runtime/common/struct.SpawnedTask.html#method.poll" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html#tymethod.poll" class="fn">poll</a>( self: <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<&mut <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/runtime/struct.SpawnedTask.html" class="struct" title="struct datafusion::common::runtime::SpawnedTask">SpawnedTask</a>\<R\>\>, cx: &mut <a href="https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html" class="struct" title="struct core::task::wake::Context">Context</a>\<'\_\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html" class="enum" title="enum core::task::poll::Poll">Poll</a>\<\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/runtime/struct.SpawnedTask.html" class="struct" title="struct datafusion::common::runtime::SpawnedTask">SpawnedTask</a>\<R\> as <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\>::<a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html#associatedtype.Output" class="associatedtype" title="type core::future::future::Future::Output">Output</a>\>

Attempts to resolve the future to a final value, registering the current task for wakeup if the value is not yet available. [Read more](https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html#tymethod.poll)

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/runtime/common/struct.SpawnedTask.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/runtime/common/struct.SpawnedTask.html#blanket-implementations" class="anchor">§</a>
