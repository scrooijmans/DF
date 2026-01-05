# Struct ChaosLayer Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/layers/chaos.rs.html#61-63" class="src">Source</a>

``` rust
pub struct ChaosLayer { /* private fields */ }
```

Available on **crate feature `layers-chaos`** only.

Expand description

Inject chaos into underlying services for robustness test.

## <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.ChaosLayer.html#chaos" class="doc-anchor">Â§</a>Chaos

Chaos tests is a part of stress test. By generating errors at specified error ratio, we can reproduce underlying services error more reliable.

Running tests under ChaosLayer will make your application more robust.

For example: If we specify an error rate of 0.5, there is a 50% chance of an EOF error for every read operation.

## <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.ChaosLayer.html#note" class="doc-anchor">Â§</a>Note

For now, ChaosLayer only injects read operations. More operations may be added in the future.

## <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.ChaosLayer.html#examples" class="doc-anchor">Â§</a>Examples

``` rust

let _ = Operator::new(services::Memory::default())?
    .layer(ChaosLayer::new(0.1))
    .finish();
Ok(())
```

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.ChaosLayer.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.ChaosLayer.html#impl-ChaosLayer" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.ChaosLayer.html" class="struct" title="struct opendal::layers::ChaosLayer">ChaosLayer</a>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.ChaosLayer.html#method.new" class="fn">new</a>(error_ratio: <a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>) -\> Self

Create a new chaos layer with specified error ratio.

##### <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.ChaosLayer.html#panics" class="doc-anchor">Â§</a>Panics

Input error_ratio must in \[0.0..=1.0\]

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.ChaosLayer.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.ChaosLayer.html#impl-Clone-for-ChaosLayer" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.ChaosLayer.html" class="struct" title="struct opendal::layers::ChaosLayer">ChaosLayer</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.ChaosLayer.html#method.clone" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.ChaosLayer.html" class="struct" title="struct opendal::layers::ChaosLayer">ChaosLayer</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.ChaosLayer.html#method.clone_from" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.ChaosLayer.html#impl-Debug-for-ChaosLayer" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.ChaosLayer.html" class="struct" title="struct opendal::layers::ChaosLayer">ChaosLayer</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.ChaosLayer.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.ChaosLayer.html#impl-Layer%3CA%3E-for-ChaosLayer" class="anchor">Â§</a>

### impl\<A: <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html" class="trait" title="trait opendal::raw::Layer">Layer</a>\<A\> for <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.ChaosLayer.html" class="struct" title="struct opendal::layers::ChaosLayer">ChaosLayer</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.ChaosLayer.html#associatedtype.LayeredAccess" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#associatedtype.LayeredAccess" class="associatedtype">LayeredAccess</a> = ChaosAccessor\<A\>

The layered accessor that returned by this layer.

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.ChaosLayer.html#method.layer" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#tymethod.layer" class="fn">layer</a>(&self, inner: A) -\> Self::<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#associatedtype.LayeredAccess" class="associatedtype" title="type opendal::raw::Layer::LayeredAccess">LayeredAccess</a>

Intercept the operations on the underlying storage.

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.ChaosLayer.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.ChaosLayer.html#blanket-implementations" class="anchor">Â§</a>
