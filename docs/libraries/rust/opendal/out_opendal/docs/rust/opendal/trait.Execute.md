# Trait Execute Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/types/execute/api.rs.html#52-67" class="src">Source</a>

``` rust
pub trait Execute:
    Send
    + Sync
    + 'static {
    // Required method
    fn execute(&self, f: BoxedStaticFuture<()>);

    // Provided method
    fn timeout(&self) -> Option<BoxedStaticFuture<()>> { ... }
}
```

Expand description

Execute trait is used to execute task in background.

## <a href="https://opendal.apache.org/docs/rust/opendal/trait.Execute.html#notes-about-timeout-implementation" class="doc-anchor">Â§</a>Notes about Timeout Implementation

Implementing a correct and elegant timeout mechanism is challenging for us.

The `Execute` trait must be object safe, allowing us to use `Arc<dyn Execute>`. Consequently, we cannot introduce a generic type parameter to `Execute`. We utilize \[`RemoteHandle`\] to implement the [`Execute::execute`](https://opendal.apache.org/docs/rust/opendal/trait.Execute.html#tymethod.execute "method opendal::Execute::execute") method. \[`RemoteHandle`\] operates by transmitting `Future::Output` through a channel, enabling the spawning of [`BoxedStaticFuture<()>`](https://opendal.apache.org/docs/rust/opendal/raw/type.BoxedStaticFuture.html "type opendal::raw::BoxedStaticFuture").

However, for timeouts, we need to spawn a future that resolves after a specified duration. Simply wrapping the future within another timeout future is not feasible because if the timeout is reached and the original future has not completed, it will be droppedâ€”causing any held `Task` to panic.

As an alternative solution, we developed a `timeout` API. Users of the `Executor` should invoke this API when they require a timeout and combine it with their own futures using \[`futures::select`\].

This approach may seem inelegant but it allows us flexibility without being tied specifically to the Tokio runtime.

PLEASE raising an issue if you have a better solution.

## Required Methods<a href="https://opendal.apache.org/docs/rust/opendal/trait.Execute.html#required-methods" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/trait.Execute.html#tymethod.execute" class="fn">execute</a>(&self, f: <a href="https://opendal.apache.org/docs/rust/opendal/raw/type.BoxedStaticFuture.html" class="type" title="type opendal::raw::BoxedStaticFuture">BoxedStaticFuture</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>)

Execute async task in background.

##### <a href="https://opendal.apache.org/docs/rust/opendal/trait.Execute.html#behavior" class="doc-anchor">Â§</a>Behavior

- Implementor MUST manage the executing futures and keep making progress.
- Implementor MUST NOT drop futures until itâ€™s resolved.

## Provided Methods<a href="https://opendal.apache.org/docs/rust/opendal/trait.Execute.html#provided-methods" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/trait.Execute.html#method.timeout" class="fn">timeout</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/raw/type.BoxedStaticFuture.html" class="type" title="type opendal::raw::BoxedStaticFuture">BoxedStaticFuture</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>\>

Return a future that will be resolved after the given timeout.

Default implementation returns None.

## Implementations on Foreign Types<a href="https://opendal.apache.org/docs/rust/opendal/trait.Execute.html#foreign-impls" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Execute.html#impl-Execute-for-()" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Execute.html" class="trait" title="trait opendal::Execute">Execute</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Execute.html#method.execute" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/trait.Execute.html#tymethod.execute" class="fn">execute</a>(&self, \_: <a href="https://opendal.apache.org/docs/rust/opendal/raw/type.BoxedStaticFuture.html" class="type" title="type opendal::raw::BoxedStaticFuture">BoxedStaticFuture</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>)

## Implementors<a href="https://opendal.apache.org/docs/rust/opendal/trait.Execute.html#implementors" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Execute.html#impl-Execute-for-TokioExecutor" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Execute.html" class="trait" title="trait opendal::Execute">Execute</a> for <a href="https://opendal.apache.org/docs/rust/opendal/executors/struct.TokioExecutor.html" class="struct" title="struct opendal::executors::TokioExecutor">TokioExecutor</a>

Available on **crate feature `executors-tokio`** only.
