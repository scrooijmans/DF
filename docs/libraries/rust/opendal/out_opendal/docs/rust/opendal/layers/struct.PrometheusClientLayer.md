# Struct PrometheusClientLayer Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/layers/prometheus_client.rs.html#80-82" class="src">Source</a>

``` rust
pub struct PrometheusClientLayer { /* private fields */ }
```

Available on **crate feature `layers-prometheus-client`** only.

Expand description

Add [prometheus-client](https://docs.rs/prometheus-client) for every operation.

## <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.PrometheusClientLayer.html#prometheus-metrics" class="doc-anchor">Â§</a>Prometheus Metrics

We provide several metrics, please see the documentation of [`observe`](https://opendal.apache.org/docs/rust/opendal/layers/observe/index.html "mod opendal::layers::observe") module. For a more detailed explanation of these metrics and how they are used, please refer to the [Prometheus documentation](https://prometheus.io/docs/introduction/overview/).

## <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.PrometheusClientLayer.html#examples" class="doc-anchor">Â§</a>Examples

``` rust

let mut registry = prometheus_client::registry::Registry::default();

let op = Operator::new(services::Memory::default())?
    .layer(PrometheusClientLayer::builder().register(&mut registry))
    .finish();

// Write data into object test.
op.write("test", "Hello, World!").await?;
// Read data from the object.
let bs = op.read("test").await?;
info!("content: {}", String::from_utf8_lossy(&bs.to_bytes()));

// Get object metadata.
let meta = op.stat("test").await?;
info!("meta: {:?}", meta);

// Export prometheus metrics.
let mut buf = String::new();
prometheus_client::encoding::text::encode(&mut buf, &registry).unwrap();
println!("## Prometheus Metrics");
println!("{}", buf);
```

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.PrometheusClientLayer.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.PrometheusClientLayer.html#impl-PrometheusClientLayer" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.PrometheusClientLayer.html" class="struct" title="struct opendal::layers::PrometheusClientLayer">PrometheusClientLayer</a>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.PrometheusClientLayer.html#method.builder" class="fn">builder</a>() -\> <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.PrometheusClientLayerBuilder.html" class="struct" title="struct opendal::layers::PrometheusClientLayerBuilder">PrometheusClientLayerBuilder</a>

Create a [`PrometheusClientLayerBuilder`](https://opendal.apache.org/docs/rust/opendal/layers/struct.PrometheusClientLayerBuilder.html "struct opendal::layers::PrometheusClientLayerBuilder") to set the configuration of metrics.

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.PrometheusClientLayer.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.PrometheusClientLayer.html#impl-Clone-for-PrometheusClientLayer" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.PrometheusClientLayer.html" class="struct" title="struct opendal::layers::PrometheusClientLayer">PrometheusClientLayer</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.PrometheusClientLayer.html#method.clone" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.PrometheusClientLayer.html" class="struct" title="struct opendal::layers::PrometheusClientLayer">PrometheusClientLayer</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.PrometheusClientLayer.html#method.clone_from" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.PrometheusClientLayer.html#impl-Debug-for-PrometheusClientLayer" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.PrometheusClientLayer.html" class="struct" title="struct opendal::layers::PrometheusClientLayer">PrometheusClientLayer</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.PrometheusClientLayer.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.PrometheusClientLayer.html#impl-Layer%3CA%3E-for-PrometheusClientLayer" class="anchor">Â§</a>

### impl\<A: <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html" class="trait" title="trait opendal::raw::Layer">Layer</a>\<A\> for <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.PrometheusClientLayer.html" class="struct" title="struct opendal::layers::PrometheusClientLayer">PrometheusClientLayer</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.PrometheusClientLayer.html#associatedtype.LayeredAccess" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#associatedtype.LayeredAccess" class="associatedtype">LayeredAccess</a> = <a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/struct.MetricsAccessor.html" class="struct" title="struct opendal::layers::observe::MetricsAccessor">MetricsAccessor</a>\<A, PrometheusClientInterceptor\>

The layered accessor that returned by this layer.

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.PrometheusClientLayer.html#method.layer" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#tymethod.layer" class="fn">layer</a>(&self, inner: A) -\> Self::<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#associatedtype.LayeredAccess" class="associatedtype" title="type opendal::raw::Layer::LayeredAccess">LayeredAccess</a>

Intercept the operations on the underlying storage.

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.PrometheusClientLayer.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.PrometheusClientLayer.html#blanket-implementations" class="anchor">Â§</a>
