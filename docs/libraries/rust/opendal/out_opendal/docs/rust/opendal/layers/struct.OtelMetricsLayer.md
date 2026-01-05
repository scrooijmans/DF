# Struct OtelMetricsLayer Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/layers/otelmetrics.rs.html#48-50" class="src">Source</a>

``` rust
pub struct OtelMetricsLayer { /* private fields */ }
```

Available on **crate feature `layers-otel-metrics`** only.

Expand description

Add [opentelemetry::metrics](https://docs.rs/opentelemetry/latest/opentelemetry/metrics/index.html) for every operation.

## <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.OtelMetricsLayer.html#examples" class="doc-anchor">Â§</a>Examples

``` rust

let meter = opentelemetry::global::meter("opendal");
let _ = Operator::new(services::Memory::default())?
    .layer(OtelMetricsLayer::builder().register(&meter))
    .finish();
Ok(())
```

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.OtelMetricsLayer.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.OtelMetricsLayer.html#impl-OtelMetricsLayer" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.OtelMetricsLayer.html" class="struct" title="struct opendal::layers::OtelMetricsLayer">OtelMetricsLayer</a>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.OtelMetricsLayer.html#method.builder" class="fn">builder</a>() -\> OtelMetricsLayerBuilder

Create a \[`OtelMetricsLayerBuilder`\] to set the configuration of metrics.

##### <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.OtelMetricsLayer.html#examples-1" class="doc-anchor">Â§</a>Examples

``` rust

let meter = opentelemetry::global::meter("opendal");
let op = Operator::new(services::Memory::default())?
    .layer(OtelMetricsLayer::builder().register(&meter))
    .finish();

Ok(())
```

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.OtelMetricsLayer.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.OtelMetricsLayer.html#impl-Clone-for-OtelMetricsLayer" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.OtelMetricsLayer.html" class="struct" title="struct opendal::layers::OtelMetricsLayer">OtelMetricsLayer</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.OtelMetricsLayer.html#method.clone" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.OtelMetricsLayer.html" class="struct" title="struct opendal::layers::OtelMetricsLayer">OtelMetricsLayer</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.OtelMetricsLayer.html#method.clone_from" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.OtelMetricsLayer.html#impl-Debug-for-OtelMetricsLayer" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.OtelMetricsLayer.html" class="struct" title="struct opendal::layers::OtelMetricsLayer">OtelMetricsLayer</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.OtelMetricsLayer.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.OtelMetricsLayer.html#impl-Layer%3CA%3E-for-OtelMetricsLayer" class="anchor">Â§</a>

### impl\<A: <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html" class="trait" title="trait opendal::raw::Layer">Layer</a>\<A\> for <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.OtelMetricsLayer.html" class="struct" title="struct opendal::layers::OtelMetricsLayer">OtelMetricsLayer</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.OtelMetricsLayer.html#associatedtype.LayeredAccess" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#associatedtype.LayeredAccess" class="associatedtype">LayeredAccess</a> = <a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/struct.MetricsAccessor.html" class="struct" title="struct opendal::layers::observe::MetricsAccessor">MetricsAccessor</a>\<A, OtelMetricsInterceptor\>

The layered accessor that returned by this layer.

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.OtelMetricsLayer.html#method.layer" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#tymethod.layer" class="fn">layer</a>(&self, inner: A) -\> Self::<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#associatedtype.LayeredAccess" class="associatedtype" title="type opendal::raw::Layer::LayeredAccess">LayeredAccess</a>

Intercept the operations on the underlying storage.

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.OtelMetricsLayer.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.OtelMetricsLayer.html#blanket-implementations" class="anchor">Â§</a>
