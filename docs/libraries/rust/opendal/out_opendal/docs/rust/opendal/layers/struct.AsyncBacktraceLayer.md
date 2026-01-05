# Struct AsyncBacktraceLayer Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/layers/async_backtrace.rs.html#45" class="src">Source</a>

``` rust
pub struct AsyncBacktraceLayer;
```

Available on **crate feature `layers-async-backtrace`** only.

Expand description

Add Efficient, logical â€˜stackâ€™ traces of async functions for the underlying services.

## <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.AsyncBacktraceLayer.html#async-backtrace" class="doc-anchor">Â§</a>Async Backtrace

async-backtrace allows developers to get a stack trace of the async functions. Read more about [async-backtrace](https://docs.rs/async-backtrace/latest/async_backtrace/)

## <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.AsyncBacktraceLayer.html#examples" class="doc-anchor">Â§</a>Examples

``` rust

let _ = Operator::new(services::Memory::default())?
    .layer(AsyncBacktraceLayer::default())
    .finish();
Ok(())
```

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.AsyncBacktraceLayer.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.AsyncBacktraceLayer.html#impl-Clone-for-AsyncBacktraceLayer" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.AsyncBacktraceLayer.html" class="struct" title="struct opendal::layers::AsyncBacktraceLayer">AsyncBacktraceLayer</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.AsyncBacktraceLayer.html#method.clone" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.AsyncBacktraceLayer.html" class="struct" title="struct opendal::layers::AsyncBacktraceLayer">AsyncBacktraceLayer</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.AsyncBacktraceLayer.html#method.clone_from" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.AsyncBacktraceLayer.html#impl-Default-for-AsyncBacktraceLayer" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.AsyncBacktraceLayer.html" class="struct" title="struct opendal::layers::AsyncBacktraceLayer">AsyncBacktraceLayer</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.AsyncBacktraceLayer.html#method.default" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.AsyncBacktraceLayer.html" class="struct" title="struct opendal::layers::AsyncBacktraceLayer">AsyncBacktraceLayer</a>

Returns the â€œdefault valueâ€? for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.AsyncBacktraceLayer.html#impl-Layer%3CA%3E-for-AsyncBacktraceLayer" class="anchor">Â§</a>

### impl\<A: <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html" class="trait" title="trait opendal::raw::Layer">Layer</a>\<A\> for <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.AsyncBacktraceLayer.html" class="struct" title="struct opendal::layers::AsyncBacktraceLayer">AsyncBacktraceLayer</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.AsyncBacktraceLayer.html#associatedtype.LayeredAccess" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#associatedtype.LayeredAccess" class="associatedtype">LayeredAccess</a> = AsyncBacktraceAccessor\<A\>

The layered accessor that returned by this layer.

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.AsyncBacktraceLayer.html#method.layer" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#tymethod.layer" class="fn">layer</a>(&self, accessor: A) -\> Self::<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#associatedtype.LayeredAccess" class="associatedtype" title="type opendal::raw::Layer::LayeredAccess">LayeredAccess</a>

Intercept the operations on the underlying storage.

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.AsyncBacktraceLayer.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.AsyncBacktraceLayer.html#blanket-implementations" class="anchor">Â§</a>
