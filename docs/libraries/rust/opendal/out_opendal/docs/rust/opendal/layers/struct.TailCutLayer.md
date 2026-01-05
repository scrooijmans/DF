# Struct TailCutLayer Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/layers/tail_cut.rs.html#260-263" class="src">Source</a>

``` rust
pub struct TailCutLayer { /* private fields */ }
```

Expand description

Layer that automatically cancels long-tail requests.

This layer monitors request latency distribution and cancels requests that are significantly slower than the historical baseline (e.g., slower than P95).

This layer should be created via [`TailCutLayer::builder()`](https://opendal.apache.org/docs/rust/opendal/layers/struct.TailCutLayer.html#method.builder "associated function opendal::layers::TailCutLayer::builder") and can be cloned to share statistics across multiple operators.

## <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.TailCutLayer.html#examples" class="doc-anchor">Â§</a>Examples

``` rust
use opendal::layers::TailCutLayer;
use std::time::Duration;

let layer = TailCutLayer::builder()
    .percentile(95)
    .safety_factor(1.3)
    .window(Duration::from_secs(60))
    .build();

let op = Operator::new(services::Memory::default())?
    .layer(layer)
    .finish();
```

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.TailCutLayer.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.TailCutLayer.html#impl-TailCutLayer" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.TailCutLayer.html" class="struct" title="struct opendal::layers::TailCutLayer">TailCutLayer</a>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.TailCutLayer.html#method.builder" class="fn">builder</a>() -\> <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.TailCutLayerBuilder.html" class="struct" title="struct opendal::layers::TailCutLayerBuilder">TailCutLayerBuilder</a>

Create a builder to configure the layer.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.TailCutLayer.html#method.new" class="fn">new</a>() -\> Self

Create a layer with default settings.

This is equivalent to `TailCutLayer::builder().build()`.

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.TailCutLayer.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.TailCutLayer.html#impl-Clone-for-TailCutLayer" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.TailCutLayer.html" class="struct" title="struct opendal::layers::TailCutLayer">TailCutLayer</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.TailCutLayer.html#method.clone" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.TailCutLayer.html" class="struct" title="struct opendal::layers::TailCutLayer">TailCutLayer</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.TailCutLayer.html#method.clone_from" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.TailCutLayer.html#impl-Default-for-TailCutLayer" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.TailCutLayer.html" class="struct" title="struct opendal::layers::TailCutLayer">TailCutLayer</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.TailCutLayer.html#method.default" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> Self

Returns the â€œdefault valueâ€? for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.TailCutLayer.html#impl-Layer%3CA%3E-for-TailCutLayer" class="anchor">Â§</a>

### impl\<A: <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html" class="trait" title="trait opendal::raw::Layer">Layer</a>\<A\> for <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.TailCutLayer.html" class="struct" title="struct opendal::layers::TailCutLayer">TailCutLayer</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.TailCutLayer.html#associatedtype.LayeredAccess" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#associatedtype.LayeredAccess" class="associatedtype">LayeredAccess</a> = TailCutAccessor\<A\>

The layered accessor that returned by this layer.

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.TailCutLayer.html#method.layer" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#tymethod.layer" class="fn">layer</a>(&self, inner: A) -\> Self::<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#associatedtype.LayeredAccess" class="associatedtype" title="type opendal::raw::Layer::LayeredAccess">LayeredAccess</a>

Intercept the operations on the underlying storage.

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.TailCutLayer.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.TailCutLayer.html#blanket-implementations" class="anchor">Â§</a>
