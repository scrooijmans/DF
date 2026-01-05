# Struct PrometheusLayerBuilder Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/layers/prometheus.rs.html#190-197" class="src">Source</a>

``` rust
pub struct PrometheusLayerBuilder { /* private fields */ }
```

Available on **crate feature `layers-prometheus`** only.

Expand description

[`PrometheusLayerBuilder`](https://opendal.apache.org/docs/rust/opendal/layers/struct.PrometheusLayerBuilder.html "struct opendal::layers::PrometheusLayerBuilder") is a config builder to build a [`PrometheusLayer`](https://opendal.apache.org/docs/rust/opendal/layers/struct.PrometheusLayer.html "struct opendal::layers::PrometheusLayer").

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.PrometheusLayerBuilder.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.PrometheusLayerBuilder.html#impl-PrometheusLayerBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.PrometheusLayerBuilder.html" class="struct" title="struct opendal::layers::PrometheusLayerBuilder">PrometheusLayerBuilder</a>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.PrometheusLayerBuilder.html#method.bytes_buckets" class="fn">bytes_buckets</a>(self, buckets: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>\>) -\> Self

Set buckets for bytes related histogram like `operation_bytes`.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.PrometheusLayerBuilder.html#method.bytes_rate_buckets" class="fn">bytes_rate_buckets</a>(self, buckets: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>\>) -\> Self

Set buckets for bytes rate related histogram like `operation_bytes_rate`.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.PrometheusLayerBuilder.html#method.entries_buckets" class="fn">entries_buckets</a>(self, buckets: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>\>) -\> Self

Set buckets for entries related histogram like `operation_entries`.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.PrometheusLayerBuilder.html#method.entries_rate_buckets" class="fn">entries_rate_buckets</a>(self, buckets: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>\>) -\> Self

Set buckets for entries rate related histogram like `operation_entries_rate`.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.PrometheusLayerBuilder.html#method.duration_seconds_buckets" class="fn">duration_seconds_buckets</a>(self, buckets: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>\>) -\> Self

Set buckets for duration seconds related histogram like `operation_duration_seconds`.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.PrometheusLayerBuilder.html#method.ttfb_buckets" class="fn">ttfb_buckets</a>(self, buckets: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>\>) -\> Self

Set buckets for ttfb related histogram like `operation_ttfb_seconds`.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.PrometheusLayerBuilder.html#method.register" class="fn">register</a>(self, registry: &Registry) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.PrometheusLayer.html" class="struct" title="struct opendal::layers::PrometheusLayer">PrometheusLayer</a>\>

Register the metrics into the given registry and return a [`PrometheusLayer`](https://opendal.apache.org/docs/rust/opendal/layers/struct.PrometheusLayer.html "struct opendal::layers::PrometheusLayer").

##### <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.PrometheusLayerBuilder.html#example" class="doc-anchor">Â§</a>Example

``` rust
// Pick a builder and configure it.
let builder = services::Memory::default();
let _ = Operator::new(builder)?
    .layer(
        PrometheusLayer::builder()
            .register(prometheus::default_registry())
            .expect("register metrics successfully"),
    )
    .finish();
```

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.PrometheusLayerBuilder.html#method.register_default" class="fn">register_default</a>(self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.PrometheusLayer.html" class="struct" title="struct opendal::layers::PrometheusLayer">PrometheusLayer</a>\>

Register the metrics into the default registry and return a [`PrometheusLayer`](https://opendal.apache.org/docs/rust/opendal/layers/struct.PrometheusLayer.html "struct opendal::layers::PrometheusLayer").

##### <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.PrometheusLayerBuilder.html#example-1" class="doc-anchor">Â§</a>Example

``` rust
// Pick a builder and configure it.
let builder = services::Memory::default();
let _ = Operator::new(builder)?
    .layer(
        PrometheusLayer::builder()
            .register_default()
            .expect("register metrics successfully"),
    )
    .finish();
```

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.PrometheusLayerBuilder.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.PrometheusLayerBuilder.html#impl-Default-for-PrometheusLayerBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.PrometheusLayerBuilder.html" class="struct" title="struct opendal::layers::PrometheusLayerBuilder">PrometheusLayerBuilder</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.PrometheusLayerBuilder.html#method.default" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> Self

Returns the â€œdefault valueâ€? for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.PrometheusLayerBuilder.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.PrometheusLayerBuilder.html#blanket-implementations" class="anchor">Â§</a>
