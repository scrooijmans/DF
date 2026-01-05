# Struct PrometheusLayer Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/layers/prometheus.rs.html#141-143" class="src">Source</a>

``` rust
pub struct PrometheusLayer { /* private fields */ }
```

Available on **crate feature `layers-prometheus`** only.

Expand description

Add [prometheus](https://docs.rs/prometheus) for every operation.

## <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.PrometheusLayer.html#prometheus-metrics" class="doc-anchor">Â§</a>Prometheus Metrics

We provide several metrics, please see the documentation of [`observe`](https://opendal.apache.org/docs/rust/opendal/layers/observe/index.html "mod opendal::layers::observe") module. For a more detailed explanation of these metrics and how they are used, please refer to the [Prometheus documentation](https://prometheus.io/docs/introduction/overview/).

## <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.PrometheusLayer.html#examples" class="doc-anchor">Â§</a>Examples

### <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.PrometheusLayer.html#basic-usage" class="doc-anchor">Â§</a>Basic Usage

``` rust

let registry = prometheus::default_registry();

let op = Operator::new(services::Memory::default())?
    .layer(
        PrometheusLayer::builder()
            .register(registry)
            .expect("register metrics successfully"),
    )
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
let mut buffer = Vec::<u8>::new();
let encoder = prometheus::TextEncoder::new();
encoder.encode(&prometheus::gather(), &mut buffer).unwrap();
println!("## Prometheus Metrics");
println!("{}", String::from_utf8(buffer.clone()).unwrap());
```

### <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.PrometheusLayer.html#global-instance" class="doc-anchor">Â§</a>Global Instance

`PrometheusLayer` needs to be registered before instantiation.

If there are multiple operators in an application that need the `PrometheusLayer`, we could instantiate it once and pass it to multiple operators. But we cannot directly call `.layer(PrometheusLayer::builder().register(&registry)?)` for different services, because registering the same metrics multiple times to the same registry will cause register errors. Therefore, we can provide a global instance for the `PrometheusLayer`.

``` rust

fn global_prometheus_layer() -> &'static PrometheusLayer {
    static GLOBAL: OnceLock<PrometheusLayer> = OnceLock::new();
    GLOBAL.get_or_init(|| {
        PrometheusLayer::builder()
            .register_default()
            .expect("Failed to register with the global registry")
    })
}

let op = Operator::new(services::Memory::default())?
    .layer(global_prometheus_layer().clone())
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
let mut buffer = Vec::<u8>::new();
let encoder = prometheus::TextEncoder::new();
encoder.encode(&prometheus::gather(), &mut buffer).unwrap();
println!("## Prometheus Metrics");
println!("{}", String::from_utf8(buffer.clone()).unwrap());
```

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.PrometheusLayer.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.PrometheusLayer.html#impl-PrometheusLayer" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.PrometheusLayer.html" class="struct" title="struct opendal::layers::PrometheusLayer">PrometheusLayer</a>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.PrometheusLayer.html#method.builder" class="fn">builder</a>() -\> <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.PrometheusLayerBuilder.html" class="struct" title="struct opendal::layers::PrometheusLayerBuilder">PrometheusLayerBuilder</a>

Create a [`PrometheusLayerBuilder`](https://opendal.apache.org/docs/rust/opendal/layers/struct.PrometheusLayerBuilder.html "struct opendal::layers::PrometheusLayerBuilder") to set the configuration of metrics.

##### <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.PrometheusLayer.html#example" class="doc-anchor">Â§</a>Example

``` rust
// Pick a builder and configure it.
let builder = services::Memory::default();
let registry = prometheus::default_registry();

let duration_seconds_buckets = prometheus::exponential_buckets(0.01, 2.0, 16).unwrap();
let bytes_buckets = prometheus::exponential_buckets(1.0, 2.0, 16).unwrap();
let _ = Operator::new(builder)?
    .layer(
        PrometheusLayer::builder()
            .duration_seconds_buckets(duration_seconds_buckets)
            .bytes_buckets(bytes_buckets)
            .register(registry)
            .expect("register metrics successfully"),
    )
    .finish();
```

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.PrometheusLayer.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.PrometheusLayer.html#impl-Clone-for-PrometheusLayer" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.PrometheusLayer.html" class="struct" title="struct opendal::layers::PrometheusLayer">PrometheusLayer</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.PrometheusLayer.html#method.clone" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.PrometheusLayer.html" class="struct" title="struct opendal::layers::PrometheusLayer">PrometheusLayer</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.PrometheusLayer.html#method.clone_from" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.PrometheusLayer.html#impl-Debug-for-PrometheusLayer" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.PrometheusLayer.html" class="struct" title="struct opendal::layers::PrometheusLayer">PrometheusLayer</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.PrometheusLayer.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.PrometheusLayer.html#impl-Layer%3CA%3E-for-PrometheusLayer" class="anchor">Â§</a>

### impl\<A: <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html" class="trait" title="trait opendal::raw::Layer">Layer</a>\<A\> for <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.PrometheusLayer.html" class="struct" title="struct opendal::layers::PrometheusLayer">PrometheusLayer</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.PrometheusLayer.html#associatedtype.LayeredAccess" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#associatedtype.LayeredAccess" class="associatedtype">LayeredAccess</a> = <a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/struct.MetricsAccessor.html" class="struct" title="struct opendal::layers::observe::MetricsAccessor">MetricsAccessor</a>\<A, PrometheusInterceptor\>

The layered accessor that returned by this layer.

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.PrometheusLayer.html#method.layer" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#tymethod.layer" class="fn">layer</a>(&self, inner: A) -\> Self::<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#associatedtype.LayeredAccess" class="associatedtype" title="type opendal::raw::Layer::LayeredAccess">LayeredAccess</a>

Intercept the operations on the underlying storage.

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.PrometheusLayer.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.PrometheusLayer.html#blanket-implementations" class="anchor">Â§</a>
