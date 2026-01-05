# Struct MetricsLayer Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/layers/observe/metrics.rs.html#381-383" class="src">Source</a>

``` rust
pub struct MetricsLayer<I: MetricsIntercept> { /* private fields */ }
```

Expand description

The metrics layer for opendal.

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/struct.MetricsLayer.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/struct.MetricsLayer.html#impl-MetricsLayer%3CI%3E" class="anchor">Â§</a>

### impl\<I: <a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/trait.MetricsIntercept.html" class="trait" title="trait opendal::layers::observe::MetricsIntercept">MetricsIntercept</a>\> <a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/struct.MetricsLayer.html" class="struct" title="struct opendal::layers::observe::MetricsLayer">MetricsLayer</a>\<I\>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/struct.MetricsLayer.html#method.new" class="fn">new</a>(interceptor: I) -\> Self

Create a new metrics layer.

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/struct.MetricsLayer.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/struct.MetricsLayer.html#impl-Clone-for-MetricsLayer%3CI%3E" class="anchor">Â§</a>

### impl\<I: <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> + <a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/trait.MetricsIntercept.html" class="trait" title="trait opendal::layers::observe::MetricsIntercept">MetricsIntercept</a>\> <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/struct.MetricsLayer.html" class="struct" title="struct opendal::layers::observe::MetricsLayer">MetricsLayer</a>\<I\>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/struct.MetricsLayer.html#method.clone" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/struct.MetricsLayer.html" class="struct" title="struct opendal::layers::observe::MetricsLayer">MetricsLayer</a>\<I\>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/struct.MetricsLayer.html#method.clone_from" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/struct.MetricsLayer.html#impl-Debug-for-MetricsLayer%3CI%3E" class="anchor">Â§</a>

### impl\<I: <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> + <a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/trait.MetricsIntercept.html" class="trait" title="trait opendal::layers::observe::MetricsIntercept">MetricsIntercept</a>\> <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/struct.MetricsLayer.html" class="struct" title="struct opendal::layers::observe::MetricsLayer">MetricsLayer</a>\<I\>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/struct.MetricsLayer.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/struct.MetricsLayer.html#impl-Layer%3CA%3E-for-MetricsLayer%3CI%3E" class="anchor">Â§</a>

### impl\<A: <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>, I: <a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/trait.MetricsIntercept.html" class="trait" title="trait opendal::layers::observe::MetricsIntercept">MetricsIntercept</a>\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html" class="trait" title="trait opendal::raw::Layer">Layer</a>\<A\> for <a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/struct.MetricsLayer.html" class="struct" title="struct opendal::layers::observe::MetricsLayer">MetricsLayer</a>\<I\>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/struct.MetricsLayer.html#associatedtype.LayeredAccess" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#associatedtype.LayeredAccess" class="associatedtype">LayeredAccess</a> = <a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/struct.MetricsAccessor.html" class="struct" title="struct opendal::layers::observe::MetricsAccessor">MetricsAccessor</a>\<A, I\>

The layered accessor that returned by this layer.

<a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/struct.MetricsLayer.html#method.layer" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#tymethod.layer" class="fn">layer</a>(&self, inner: A) -\> Self::<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#associatedtype.LayeredAccess" class="associatedtype" title="type opendal::raw::Layer::LayeredAccess">LayeredAccess</a>

Intercept the operations on the underlying storage.

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/struct.MetricsLayer.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/struct.MetricsLayer.html#blanket-implementations" class="anchor">Â§</a>
