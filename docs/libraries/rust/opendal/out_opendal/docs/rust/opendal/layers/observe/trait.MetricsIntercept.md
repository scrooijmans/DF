# Trait MetricsIntercept Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/layers/observe/metrics.rs.html#372-377" class="src">Source</a>

``` rust
pub trait MetricsIntercept:
    Debug
    + Clone
    + Send
    + Sync
    + Unpin
    + 'static {
    // Provided method
    fn observe(&self, labels: MetricLabels, value: MetricValue) { ... }
}
```

Expand description

The interceptor for metrics.

All metrics related libs should implement this trait to observe opendalâ€™s internal operations.

## Provided Methods<a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/trait.MetricsIntercept.html#provided-methods" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/trait.MetricsIntercept.html#method.observe" class="fn">observe</a>(&self, labels: <a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/struct.MetricLabels.html" class="struct" title="struct opendal::layers::observe::MetricLabels">MetricLabels</a>, value: <a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/enum.MetricValue.html" class="enum" title="enum opendal::layers::observe::MetricValue">MetricValue</a>)

Observe the metric value.

## Dyn Compatibility<a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/trait.MetricsIntercept.html#dyn-compatibility" class="anchor">Â§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementors<a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/trait.MetricsIntercept.html#implementors" class="anchor">Â§</a>
