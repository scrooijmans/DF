# Struct MetricsLayer Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/layers/metrics.rs.html#80" class="src">Source</a>

``` rust
pub struct MetricsLayer {}
```

Available on **crate feature `layers-metrics`** only.

Expand description

Add [metrics](https://docs.rs/metrics/) for every operation.

## <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.MetricsLayer.html#metrics" class="doc-anchor">Â§</a>Metrics

We provide several metrics, please see the documentation of [`observe`](https://opendal.apache.org/docs/rust/opendal/layers/observe/index.html "mod opendal::layers::observe") module.

## <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.MetricsLayer.html#notes" class="doc-anchor">Â§</a>Notes

Please make sure the exporter has been pulled in regular time. Otherwise, the histogram data collected by `requests_duration_seconds` could result in OOM.

## <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.MetricsLayer.html#examples" class="doc-anchor">Â§</a>Examples

``` rust

let _ = Operator::new(services::Memory::default())?
    .layer(MetricsLayer::default())
    .finish();
Ok(())
```

## <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.MetricsLayer.html#output" class="doc-anchor">Â§</a>Output

OpenDAL is using [`metrics`](https://docs.rs/metrics/latest/metrics/) for metrics internally.

To enable metrics output, please enable one of the exporters that `metrics` supports.

Take [`metrics_exporter_prometheus`](https://docs.rs/metrics-exporter-prometheus/latest/metrics_exporter_prometheus/) as an example:

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.MetricsLayer.html#" class="tooltip" title="This example is not tested">â“˜</a>

``` rust
let builder = PrometheusBuilder::new()
    .set_buckets_for_metric(
        Matcher::Suffix("bytes".into()),
        &observe::DEFAULT_BYTES_BUCKETS,
    )
    .set_buckets_for_metric(
        Matcher::Suffix("duration_seconds".into()),
        &observe::DEFAULT_DURATION_SECONDS_BUCKETS,
    )
    // ..
    .expect("failed to create builder");
builder.install().expect("failed to install recorder/exporter");
let handle = builder.install_recorder().expect("failed to install recorder");
let (recorder, exporter) = builder.build().expect("failed to build recorder/exporter");
let recorder = builder.build_recorder().expect("failed to build recorder");
```

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.MetricsLayer.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.MetricsLayer.html#impl-Clone-for-MetricsLayer" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.MetricsLayer.html" class="struct" title="struct opendal::layers::MetricsLayer">MetricsLayer</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.MetricsLayer.html#method.clone" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.MetricsLayer.html" class="struct" title="struct opendal::layers::MetricsLayer">MetricsLayer</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.MetricsLayer.html#method.clone_from" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.MetricsLayer.html#impl-Debug-for-MetricsLayer" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.MetricsLayer.html" class="struct" title="struct opendal::layers::MetricsLayer">MetricsLayer</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.MetricsLayer.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.MetricsLayer.html#impl-Default-for-MetricsLayer" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.MetricsLayer.html" class="struct" title="struct opendal::layers::MetricsLayer">MetricsLayer</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.MetricsLayer.html#method.default" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.MetricsLayer.html" class="struct" title="struct opendal::layers::MetricsLayer">MetricsLayer</a>

Returns the â€œdefault valueâ€? for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.MetricsLayer.html#impl-Layer%3CA%3E-for-MetricsLayer" class="anchor">Â§</a>

### impl\<A: <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html" class="trait" title="trait opendal::raw::Layer">Layer</a>\<A\> for <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.MetricsLayer.html" class="struct" title="struct opendal::layers::MetricsLayer">MetricsLayer</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.MetricsLayer.html#associatedtype.LayeredAccess" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#associatedtype.LayeredAccess" class="associatedtype">LayeredAccess</a> = <a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/struct.MetricsAccessor.html" class="struct" title="struct opendal::layers::observe::MetricsAccessor">MetricsAccessor</a>\<A, MetricsInterceptor\>

The layered accessor that returned by this layer.

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.MetricsLayer.html#method.layer" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#tymethod.layer" class="fn">layer</a>(&self, inner: A) -\> Self::<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#associatedtype.LayeredAccess" class="associatedtype" title="type opendal::raw::Layer::LayeredAccess">LayeredAccess</a>

Intercept the operations on the underlying storage.

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.MetricsLayer.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.MetricsLayer.html#blanket-implementations" class="anchor">Â§</a>
