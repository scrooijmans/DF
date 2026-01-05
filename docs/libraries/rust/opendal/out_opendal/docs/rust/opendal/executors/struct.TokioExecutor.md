# Struct TokioExecutor Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/types/execute/executors/tokio_executor.rs.html#23" class="src">Source</a>

``` rust
pub struct TokioExecutor {}
```

Available on **crate feature `executors-tokio`** only.

Expand description

Executor that uses the \[`tokio::task::spawn`\] to execute futures.

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/executors/struct.TokioExecutor.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/executors/struct.TokioExecutor.html#impl-Default-for-TokioExecutor" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://opendal.apache.org/docs/rust/opendal/executors/struct.TokioExecutor.html" class="struct" title="struct opendal::executors::TokioExecutor">TokioExecutor</a>

<a href="https://opendal.apache.org/docs/rust/opendal/executors/struct.TokioExecutor.html#method.default" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://opendal.apache.org/docs/rust/opendal/executors/struct.TokioExecutor.html" class="struct" title="struct opendal::executors::TokioExecutor">TokioExecutor</a>

Returns the â€œdefault valueâ€? for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://opendal.apache.org/docs/rust/opendal/executors/struct.TokioExecutor.html#impl-Execute-for-TokioExecutor" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Execute.html" class="trait" title="trait opendal::Execute">Execute</a> for <a href="https://opendal.apache.org/docs/rust/opendal/executors/struct.TokioExecutor.html" class="struct" title="struct opendal::executors::TokioExecutor">TokioExecutor</a>

<a href="https://opendal.apache.org/docs/rust/opendal/executors/struct.TokioExecutor.html#method.execute" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/trait.Execute.html#tymethod.execute" class="fn">execute</a>(&self, f: <a href="https://opendal.apache.org/docs/rust/opendal/raw/type.BoxedStaticFuture.html" class="type" title="type opendal::raw::BoxedStaticFuture">BoxedStaticFuture</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>)

Tokioâ€™s JoinHandle has its own `abort` support, so dropping handle wonâ€™t cancel the task.

<a href="https://opendal.apache.org/docs/rust/opendal/executors/struct.TokioExecutor.html#method.timeout" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/trait.Execute.html#method.timeout" class="fn">timeout</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/raw/type.BoxedStaticFuture.html" class="type" title="type opendal::raw::BoxedStaticFuture">BoxedStaticFuture</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>\>

Return a future that will be resolved after the given timeout. [Read more](https://opendal.apache.org/docs/rust/opendal/trait.Execute.html#method.timeout)

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/executors/struct.TokioExecutor.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/executors/struct.TokioExecutor.html#blanket-implementations" class="anchor">Â§</a>
