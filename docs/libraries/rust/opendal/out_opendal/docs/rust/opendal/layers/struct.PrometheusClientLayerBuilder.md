# Struct PrometheusClientLayerBuilder Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/layers/prometheus_client.rs.html#100-108" class="src">Source</a>

``` rust
pub struct PrometheusClientLayerBuilder { /* private fields */ }
```

Available on **crate feature `layers-prometheus-client`** only.

Expand description

[`PrometheusClientLayerBuilder`](https://opendal.apache.org/docs/rust/opendal/layers/struct.PrometheusClientLayerBuilder.html "struct opendal::layers::PrometheusClientLayerBuilder") is a config builder to build a [`PrometheusClientLayer`](https://opendal.apache.org/docs/rust/opendal/layers/struct.PrometheusClientLayer.html "struct opendal::layers::PrometheusClientLayer").

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.PrometheusClientLayerBuilder.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.PrometheusClientLayerBuilder.html#impl-PrometheusClientLayerBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.PrometheusClientLayerBuilder.html" class="struct" title="struct opendal::layers::PrometheusClientLayerBuilder">PrometheusClientLayerBuilder</a>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.PrometheusClientLayerBuilder.html#method.bytes_buckets" class="fn">bytes_buckets</a>(self, buckets: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>\>) -\> Self

Set buckets for bytes related histogram like `operation_bytes`.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.PrometheusClientLayerBuilder.html#method.bytes_rate_buckets" class="fn">bytes_rate_buckets</a>(self, buckets: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>\>) -\> Self

Set buckets for bytes rate related histogram like `operation_bytes_rate`.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.PrometheusClientLayerBuilder.html#method.entries_buckets" class="fn">entries_buckets</a>(self, buckets: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>\>) -\> Self

Set buckets for entries related histogram like `operation_entries`.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.PrometheusClientLayerBuilder.html#method.entries_rate_buckets" class="fn">entries_rate_buckets</a>(self, buckets: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>\>) -\> Self

Set buckets for entries rate related histogram like `operation_entries_rate`.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.PrometheusClientLayerBuilder.html#method.duration_seconds_buckets" class="fn">duration_seconds_buckets</a>(self, buckets: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>\>) -\> Self

Set buckets for duration seconds related histogram like `operation_duration_seconds`.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.PrometheusClientLayerBuilder.html#method.ttfb_buckets" class="fn">ttfb_buckets</a>(self, buckets: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>\>) -\> Self

Set buckets for ttfb related histogram like `operation_ttfb_seconds`.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.PrometheusClientLayerBuilder.html#method.disable_label_root" class="fn">disable_label_root</a>(self, disable: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> Self

The â€˜rootâ€™ label might have risks of being high cardinality, users can choose to disable it when they found itâ€™s not useful for their metrics.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.PrometheusClientLayerBuilder.html#method.register" class="fn">register</a>(self, registry: &mut Registry) -\> <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.PrometheusClientLayer.html" class="struct" title="struct opendal::layers::PrometheusClientLayer">PrometheusClientLayer</a>

Register the metrics into the registry and return a [`PrometheusClientLayer`](https://opendal.apache.org/docs/rust/opendal/layers/struct.PrometheusClientLayer.html "struct opendal::layers::PrometheusClientLayer").

##### <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.PrometheusClientLayerBuilder.html#example" class="doc-anchor">Â§</a>Example

``` rust

// Pick a builder and configure it.
let builder = services::Memory::default();
let mut registry = prometheus_client::registry::Registry::default();

let _ = Operator::new(builder)?
    .layer(PrometheusClientLayer::builder().register(&mut registry))
    .finish();
```

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.PrometheusClientLayerBuilder.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.PrometheusClientLayerBuilder.html#impl-Default-for-PrometheusClientLayerBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.PrometheusClientLayerBuilder.html" class="struct" title="struct opendal::layers::PrometheusClientLayerBuilder">PrometheusClientLayerBuilder</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.PrometheusClientLayerBuilder.html#method.default" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> Self

Returns the â€œdefault valueâ€? for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.PrometheusClientLayerBuilder.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.PrometheusClientLayerBuilder.html#blanket-implementations" class="anchor">Â§</a>
