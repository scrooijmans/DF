# Struct FastmetricsLayerBuilder Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/layers/fastmetrics.rs.html#149-157" class="src">Source</a>

``` rust
pub struct FastmetricsLayerBuilder { /* private fields */ }
```

Available on **crate feature `layers-fastmetrics`** only.

Expand description

[`FastmetricsLayerBuilder`](https://opendal.apache.org/docs/rust/opendal/layers/struct.FastmetricsLayerBuilder.html "struct opendal::layers::FastmetricsLayerBuilder") is a config builder to build a [`FastmetricsLayer`](https://opendal.apache.org/docs/rust/opendal/layers/struct.FastmetricsLayer.html "struct opendal::layers::FastmetricsLayer").

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.FastmetricsLayerBuilder.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.FastmetricsLayerBuilder.html#impl-FastmetricsLayerBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.FastmetricsLayerBuilder.html" class="struct" title="struct opendal::layers::FastmetricsLayerBuilder">FastmetricsLayerBuilder</a>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.FastmetricsLayerBuilder.html#method.bytes_buckets" class="fn">bytes_buckets</a>(self, buckets: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>\>) -\> Self

Set buckets for bytes related histogram like `operation_bytes`.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.FastmetricsLayerBuilder.html#method.bytes_rate_buckets" class="fn">bytes_rate_buckets</a>(self, buckets: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>\>) -\> Self

Set buckets for bytes rate related histogram like `operation_bytes_rate`.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.FastmetricsLayerBuilder.html#method.entries_buckets" class="fn">entries_buckets</a>(self, buckets: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>\>) -\> Self

Set buckets for entries related histogram like `operation_entries`.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.FastmetricsLayerBuilder.html#method.entries_rate_buckets" class="fn">entries_rate_buckets</a>(self, buckets: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>\>) -\> Self

Set buckets for entries rate related histogram like `operation_entries_rate`.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.FastmetricsLayerBuilder.html#method.duration_seconds_buckets" class="fn">duration_seconds_buckets</a>(self, buckets: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>\>) -\> Self

Set buckets for duration seconds related histogram like `operation_duration_seconds`.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.FastmetricsLayerBuilder.html#method.ttfb_buckets" class="fn">ttfb_buckets</a>(self, buckets: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>\>) -\> Self

Set buckets for ttfb related histogram like `operation_ttfb_seconds`.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.FastmetricsLayerBuilder.html#method.disable_label_root" class="fn">disable_label_root</a>(self, disable: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> Self

The â€˜rootâ€™ label might have risks of being high cardinality; users can choose to disable it when they found itâ€™s not useful for their metrics.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.FastmetricsLayerBuilder.html#method.register" class="fn">register</a>(self, registry: &mut Registry) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.FastmetricsLayer.html" class="struct" title="struct opendal::layers::FastmetricsLayer">FastmetricsLayer</a>\>

Register the metrics into the registry and return a [`FastmetricsLayer`](https://opendal.apache.org/docs/rust/opendal/layers/struct.FastmetricsLayer.html "struct opendal::layers::FastmetricsLayer").

##### <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.FastmetricsLayerBuilder.html#example" class="doc-anchor">Â§</a>Example

``` rust

let mut registry = fastmetrics::registry::Registry::default();

// Pick a builder and configure it.
let builder = services::Memory::default();
let _ = Operator::new(builder)?
    .layer(FastmetricsLayer::builder().register(&mut registry)?)
    .finish();
```

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.FastmetricsLayerBuilder.html#method.register_global" class="fn">register_global</a>(self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.FastmetricsLayer.html" class="struct" title="struct opendal::layers::FastmetricsLayer">FastmetricsLayer</a>\>

Register the metrics into the global registry and return a [`FastmetricsLayer`](https://opendal.apache.org/docs/rust/opendal/layers/struct.FastmetricsLayer.html "struct opendal::layers::FastmetricsLayer").

##### <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.FastmetricsLayerBuilder.html#example-1" class="doc-anchor">Â§</a>Example

``` rust

// Pick a builder and configure it.
let builder = services::Memory::default();
let _ = Operator::new(builder)?
    .layer(FastmetricsLayer::builder().register_global()?)
    .finish();
```

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.FastmetricsLayerBuilder.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.FastmetricsLayerBuilder.html#impl-Default-for-FastmetricsLayerBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.FastmetricsLayerBuilder.html" class="struct" title="struct opendal::layers::FastmetricsLayerBuilder">FastmetricsLayerBuilder</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.FastmetricsLayerBuilder.html#method.default" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> Self

Returns the â€œdefault valueâ€? for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.FastmetricsLayerBuilder.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.FastmetricsLayerBuilder.html#blanket-implementations" class="anchor">Â§</a>
