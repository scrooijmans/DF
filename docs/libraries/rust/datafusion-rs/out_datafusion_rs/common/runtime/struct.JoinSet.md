# Struct JoinSet Copy item path

<a href="https://docs.rs/datafusion-common-runtime/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_common_runtime/join_set.rs.html#29" class="src">Source</a>

``` rust
pub struct JoinSet<T> { /* private fields */ }
```

Expand description

A wrapper around Tokio’s JoinSet that forwards all API calls while optionally instrumenting spawned tasks and blocking closures with custom tracing behavior. If no tracer is injected via `trace_utils::set_tracer`, tasks and closures are executed without any instrumentation.

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/runtime/struct.JoinSet.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/runtime/struct.JoinSet.html#impl-JoinSet%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/runtime/struct.JoinSet.html" class="struct" title="struct datafusion::common::runtime::JoinSet">JoinSet</a>\<T\>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/runtime/struct.JoinSet.html#method.new" class="fn">new</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/runtime/struct.JoinSet.html" class="struct" title="struct datafusion::common::runtime::JoinSet">JoinSet</a>\<T\>

[JoinSet::new](https://docs.rs/tokio/1.47.1/x86_64-unknown-linux-gnu/tokio/task/join_set/struct.JoinSet.html#method.new "associated function tokio::task::join_set::JoinSet::new") - Create a new JoinSet.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/runtime/struct.JoinSet.html#method.len" class="fn">len</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

[JoinSet::len](https://docs.rs/tokio/1.47.1/x86_64-unknown-linux-gnu/tokio/task/join_set/struct.JoinSet.html#method.len "method tokio::task::join_set::JoinSet::len") - Return the number of tasks.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/runtime/struct.JoinSet.html#method.is_empty" class="fn">is_empty</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

[JoinSet::is_empty](https://docs.rs/tokio/1.47.1/x86_64-unknown-linux-gnu/tokio/task/join_set/struct.JoinSet.html#method.is_empty "method tokio::task::join_set::JoinSet::is_empty") - Check if the JoinSet is empty.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/runtime/struct.JoinSet.html#impl-JoinSet%3CT%3E-1" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/runtime/struct.JoinSet.html" class="struct" title="struct datafusion::common::runtime::JoinSet">JoinSet</a>\<T\>

where T: 'static,

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/runtime/struct.JoinSet.html#method.spawn" class="fn">spawn</a>\<F\>(&mut self, task: F) -\> <a href="https://docs.rs/tokio/1.47.1/x86_64-unknown-linux-gnu/tokio/runtime/task/abort/struct.AbortHandle.html" class="struct" title="struct tokio::runtime::task::abort::AbortHandle">AbortHandle</a>

where F: <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = T\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'static, T: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>,

[JoinSet::spawn](https://docs.rs/tokio/1.47.1/x86_64-unknown-linux-gnu/tokio/task/join_set/struct.JoinSet.html#method.spawn "method tokio::task::join_set::JoinSet::spawn") - Spawn a new task.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/runtime/struct.JoinSet.html#method.spawn_on" class="fn">spawn_on</a>\<F\>(&mut self, task: F, handle: &<a href="https://docs.rs/tokio/1.47.1/x86_64-unknown-linux-gnu/tokio/runtime/handle/struct.Handle.html" class="struct" title="struct tokio::runtime::handle::Handle">Handle</a>) -\> <a href="https://docs.rs/tokio/1.47.1/x86_64-unknown-linux-gnu/tokio/runtime/task/abort/struct.AbortHandle.html" class="struct" title="struct tokio::runtime::task::abort::AbortHandle">AbortHandle</a>

where F: <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = T\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'static, T: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>,

[JoinSet::spawn_on](https://docs.rs/tokio/1.47.1/x86_64-unknown-linux-gnu/tokio/task/join_set/struct.JoinSet.html#method.spawn_on "method tokio::task::join_set::JoinSet::spawn_on") - Spawn a task on a provided runtime.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/runtime/struct.JoinSet.html#method.spawn_local" class="fn">spawn_local</a>\<F\>(&mut self, task: F) -\> <a href="https://docs.rs/tokio/1.47.1/x86_64-unknown-linux-gnu/tokio/runtime/task/abort/struct.AbortHandle.html" class="struct" title="struct tokio::runtime::task::abort::AbortHandle">AbortHandle</a>

where F: <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = T\> + 'static,

[JoinSet::spawn_local](https://docs.rs/tokio/1.47.1/x86_64-unknown-linux-gnu/tokio/task/join_set/struct.JoinSet.html#method.spawn_local "method tokio::task::join_set::JoinSet::spawn_local") - Spawn a local task.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/runtime/struct.JoinSet.html#method.spawn_local_on" class="fn">spawn_local_on</a>\<F\>( &mut self, task: F, local_set: &<a href="https://docs.rs/tokio/1.47.1/x86_64-unknown-linux-gnu/tokio/task/local/struct.LocalSet.html" class="struct" title="struct tokio::task::local::LocalSet">LocalSet</a>, ) -\> <a href="https://docs.rs/tokio/1.47.1/x86_64-unknown-linux-gnu/tokio/runtime/task/abort/struct.AbortHandle.html" class="struct" title="struct tokio::runtime::task::abort::AbortHandle">AbortHandle</a>

where F: <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = T\> + 'static,

[JoinSet::spawn_local_on](https://docs.rs/tokio/1.47.1/x86_64-unknown-linux-gnu/tokio/task/join_set/struct.JoinSet.html#method.spawn_local_on "method tokio::task::join_set::JoinSet::spawn_local_on") - Spawn a local task on a provided LocalSet.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/runtime/struct.JoinSet.html#method.spawn_blocking" class="fn">spawn_blocking</a>\<F\>(&mut self, f: F) -\> <a href="https://docs.rs/tokio/1.47.1/x86_64-unknown-linux-gnu/tokio/runtime/task/abort/struct.AbortHandle.html" class="struct" title="struct tokio::runtime::task::abort::AbortHandle">AbortHandle</a>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html" class="trait" title="trait core::ops::function::FnOnce">FnOnce</a>() -\> T + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'static, T: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>,

[JoinSet::spawn_blocking](https://docs.rs/tokio/1.47.1/x86_64-unknown-linux-gnu/tokio/task/join_set/struct.JoinSet.html#method.spawn_blocking "method tokio::task::join_set::JoinSet::spawn_blocking") - Spawn a blocking task.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/runtime/struct.JoinSet.html#method.spawn_blocking_on" class="fn">spawn_blocking_on</a>\<F\>(&mut self, f: F, handle: &<a href="https://docs.rs/tokio/1.47.1/x86_64-unknown-linux-gnu/tokio/runtime/handle/struct.Handle.html" class="struct" title="struct tokio::runtime::handle::Handle">Handle</a>) -\> <a href="https://docs.rs/tokio/1.47.1/x86_64-unknown-linux-gnu/tokio/runtime/task/abort/struct.AbortHandle.html" class="struct" title="struct tokio::runtime::task::abort::AbortHandle">AbortHandle</a>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html" class="trait" title="trait core::ops::function::FnOnce">FnOnce</a>() -\> T + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'static, T: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>,

[JoinSet::spawn_blocking_on](https://docs.rs/tokio/1.47.1/x86_64-unknown-linux-gnu/tokio/task/join_set/struct.JoinSet.html#method.spawn_blocking_on "method tokio::task::join_set::JoinSet::spawn_blocking_on") - Spawn a blocking task on a provided runtime.

#### pub async fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/runtime/struct.JoinSet.html#method.join_next" class="fn">join_next</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<T, <a href="https://docs.rs/tokio/1.47.1/x86_64-unknown-linux-gnu/tokio/runtime/task/error/struct.JoinError.html" class="struct" title="struct tokio::runtime::task::error::JoinError">JoinError</a>\>\>

[JoinSet::join_next](https://docs.rs/tokio/1.47.1/x86_64-unknown-linux-gnu/tokio/task/join_set/struct.JoinSet.html#method.join_next "method tokio::task::join_set::JoinSet::join_next") - Await the next completed task.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/runtime/struct.JoinSet.html#method.try_join_next" class="fn">try_join_next</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<T, <a href="https://docs.rs/tokio/1.47.1/x86_64-unknown-linux-gnu/tokio/runtime/task/error/struct.JoinError.html" class="struct" title="struct tokio::runtime::task::error::JoinError">JoinError</a>\>\>

[JoinSet::try_join_next](https://docs.rs/tokio/1.47.1/x86_64-unknown-linux-gnu/tokio/task/join_set/struct.JoinSet.html#method.try_join_next "method tokio::task::join_set::JoinSet::try_join_next") - Try to join the next completed task.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/runtime/struct.JoinSet.html#method.abort_all" class="fn">abort_all</a>(&mut self)

[JoinSet::abort_all](https://docs.rs/tokio/1.47.1/x86_64-unknown-linux-gnu/tokio/task/join_set/struct.JoinSet.html#method.abort_all "method tokio::task::join_set::JoinSet::abort_all") - Abort all tasks.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/runtime/struct.JoinSet.html#method.detach_all" class="fn">detach_all</a>(&mut self)

[JoinSet::detach_all](https://docs.rs/tokio/1.47.1/x86_64-unknown-linux-gnu/tokio/task/join_set/struct.JoinSet.html#method.detach_all "method tokio::task::join_set::JoinSet::detach_all") - Detach all tasks.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/runtime/struct.JoinSet.html#method.poll_join_next" class="fn">poll_join_next</a>( &mut self, cx: &mut <a href="https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html" class="struct" title="struct core::task::wake::Context">Context</a>\<'\_\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html" class="enum" title="enum core::task::poll::Poll">Poll</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<T, <a href="https://docs.rs/tokio/1.47.1/x86_64-unknown-linux-gnu/tokio/runtime/task/error/struct.JoinError.html" class="struct" title="struct tokio::runtime::task::error::JoinError">JoinError</a>\>\>\>

[JoinSet::poll_join_next](https://docs.rs/tokio/1.47.1/x86_64-unknown-linux-gnu/tokio/task/join_set/struct.JoinSet.html#method.poll_join_next "method tokio::task::join_set::JoinSet::poll_join_next") - Poll for the next completed task.

#### pub async fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/runtime/struct.JoinSet.html#method.join_next_with_id" class="fn">join_next_with_id</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<(<a href="https://docs.rs/tokio/1.47.1/x86_64-unknown-linux-gnu/tokio/runtime/task/id/struct.Id.html" class="struct" title="struct tokio::runtime::task::id::Id">Id</a>, T), <a href="https://docs.rs/tokio/1.47.1/x86_64-unknown-linux-gnu/tokio/runtime/task/error/struct.JoinError.html" class="struct" title="struct tokio::runtime::task::error::JoinError">JoinError</a>\>\>

[JoinSet::join_next_with_id](https://docs.rs/tokio/1.47.1/x86_64-unknown-linux-gnu/tokio/task/join_set/struct.JoinSet.html#method.join_next_with_id "method tokio::task::join_set::JoinSet::join_next_with_id") - Await the next completed task with its ID.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/runtime/struct.JoinSet.html#method.try_join_next_with_id" class="fn">try_join_next_with_id</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<(<a href="https://docs.rs/tokio/1.47.1/x86_64-unknown-linux-gnu/tokio/runtime/task/id/struct.Id.html" class="struct" title="struct tokio::runtime::task::id::Id">Id</a>, T), <a href="https://docs.rs/tokio/1.47.1/x86_64-unknown-linux-gnu/tokio/runtime/task/error/struct.JoinError.html" class="struct" title="struct tokio::runtime::task::error::JoinError">JoinError</a>\>\>

[JoinSet::try_join_next_with_id](https://docs.rs/tokio/1.47.1/x86_64-unknown-linux-gnu/tokio/task/join_set/struct.JoinSet.html#method.try_join_next_with_id "method tokio::task::join_set::JoinSet::try_join_next_with_id") - Try to join the next completed task with its ID.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/runtime/struct.JoinSet.html#method.poll_join_next_with_id" class="fn">poll_join_next_with_id</a>( &mut self, cx: &mut <a href="https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html" class="struct" title="struct core::task::wake::Context">Context</a>\<'\_\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html" class="enum" title="enum core::task::poll::Poll">Poll</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<(<a href="https://docs.rs/tokio/1.47.1/x86_64-unknown-linux-gnu/tokio/runtime/task/id/struct.Id.html" class="struct" title="struct tokio::runtime::task::id::Id">Id</a>, T), <a href="https://docs.rs/tokio/1.47.1/x86_64-unknown-linux-gnu/tokio/runtime/task/error/struct.JoinError.html" class="struct" title="struct tokio::runtime::task::error::JoinError">JoinError</a>\>\>\>

[JoinSet::poll_join_next_with_id](https://docs.rs/tokio/1.47.1/x86_64-unknown-linux-gnu/tokio/task/join_set/struct.JoinSet.html#method.poll_join_next_with_id "method tokio::task::join_set::JoinSet::poll_join_next_with_id") - Poll for the next completed task with its ID.

#### pub async fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/runtime/struct.JoinSet.html#method.shutdown" class="fn">shutdown</a>(&mut self)

[JoinSet::shutdown](https://docs.rs/tokio/1.47.1/x86_64-unknown-linux-gnu/tokio/task/join_set/struct.JoinSet.html#method.shutdown "method tokio::task::join_set::JoinSet::shutdown") - Abort all tasks and wait for shutdown.

#### pub async fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/runtime/struct.JoinSet.html#method.join_all" class="fn">join_all</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<T\>

[JoinSet::join_all](https://docs.rs/tokio/1.47.1/x86_64-unknown-linux-gnu/tokio/task/join_set/struct.JoinSet.html#method.join_all "method tokio::task::join_set::JoinSet::join_all") - Await all tasks.

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/runtime/struct.JoinSet.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/runtime/struct.JoinSet.html#impl-Debug-for-JoinSet%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/runtime/struct.JoinSet.html" class="struct" title="struct datafusion::common::runtime::JoinSet">JoinSet</a>\<T\>

where T: <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/runtime/struct.JoinSet.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/runtime/struct.JoinSet.html#impl-Default-for-JoinSet%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/runtime/struct.JoinSet.html" class="struct" title="struct datafusion::common::runtime::JoinSet">JoinSet</a>\<T\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/runtime/struct.JoinSet.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/runtime/struct.JoinSet.html" class="struct" title="struct datafusion::common::runtime::JoinSet">JoinSet</a>\<T\>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/runtime/struct.JoinSet.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/runtime/struct.JoinSet.html#blanket-implementations" class="anchor">§</a>
