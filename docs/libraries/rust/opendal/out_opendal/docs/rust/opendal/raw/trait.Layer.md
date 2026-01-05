# Trait Layer Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/raw/layer.rs.html#98-104" class="src">Source</a>

``` rust
pub trait Layer<A: Access> {
    type LayeredAccess: Access;

    // Required method
    fn layer(&self, inner: A) -> Self::LayeredAccess;
}
```

Expand description

Layer is used to intercept the operations on the underlying storage.

Struct that implement this trait must accept input `Arc<dyn Accessor>` as inner, and returns a new `Arc<dyn Accessor>` as output.

All functions in `Accessor` requires `&self`, so itâ€™s implementerâ€™s responsibility to maintain the internal mutability. Please also keep in mind that `Accessor` requires `Send` and `Sync`.

## <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#notes" class="doc-anchor">Â§</a>Notes

### <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#inner" class="doc-anchor">Â§</a>Inner

Itâ€™s required to implement `fn inner() -> Option<Arc<dyn Accessor>>` for layerâ€™s accessors.

By implement this method, all API calls will be forwarded to inner accessor instead.

## <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#examples" class="doc-anchor">Â§</a>Examples

``` rust
use std::sync::Arc;

use opendal::raw::*;
use opendal::*;

/// Implement the real accessor logic here.
#[derive(Debug)]
struct TraceAccessor<A: Access> {
    inner: A,
}

impl<A: Access> LayeredAccess for TraceAccessor<A> {
    type Inner = A;
    type Reader = A::Reader;
    type Writer = A::Writer;
    type Lister = A::Lister;
    type Deleter = A::Deleter;

    fn inner(&self) -> &Self::Inner {
        &self.inner
    }

    async fn read(&self, path: &str, args: OpRead) -> Result<(RpRead, Self::Reader)> {
        self.inner.read(path, args).await
    }

    async fn write(&self, path: &str, args: OpWrite) -> Result<(RpWrite, Self::Writer)> {
        self.inner.write(path, args).await
    }

    async fn list(&self, path: &str, args: OpList) -> Result<(RpList, Self::Lister)> {
        self.inner.list(path, args).await
    }

    async fn delete(&self) -> Result<(RpDelete, Self::Deleter)> {
        self.inner.delete().await
    }
}

/// The public struct that exposed to users.
///
/// Will be used like `op.layer(TraceLayer)`
struct TraceLayer;

impl<A: Access> Layer<A> for TraceLayer {
    type LayeredAccess = TraceAccessor<A>;

    fn layer(&self, inner: A) -> Self::LayeredAccess {
        TraceAccessor { inner }
    }
}
```

## Required Associated Types<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#required-associated-types" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#associatedtype.LayeredAccess" class="associatedtype">LayeredAccess</a>: <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>

The layered accessor that returned by this layer.

## Required Methods<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#required-methods" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#tymethod.layer" class="fn">layer</a>(&self, inner: A) -\> Self::<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#associatedtype.LayeredAccess" class="associatedtype" title="type opendal::raw::Layer::LayeredAccess">LayeredAccess</a>

Intercept the operations on the underlying storage.

## Implementors<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#implementors" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#impl-Layer%3CA%3E-for-AsyncBacktraceLayer" class="anchor">Â§</a>

### impl\<A: <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html" class="trait" title="trait opendal::raw::Layer">Layer</a>\<A\> for <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.AsyncBacktraceLayer.html" class="struct" title="struct opendal::layers::AsyncBacktraceLayer">AsyncBacktraceLayer</a>

Available on **crate feature `layers-async-backtrace`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#associatedtype.LayeredAccess-1" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#associatedtype.LayeredAccess" class="associatedtype">LayeredAccess</a> = AsyncBacktraceAccessor\<A\>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#impl-Layer%3CA%3E-for-AwaitTreeLayer" class="anchor">Â§</a>

### impl\<A: <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html" class="trait" title="trait opendal::raw::Layer">Layer</a>\<A\> for <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.AwaitTreeLayer.html" class="struct" title="struct opendal::layers::AwaitTreeLayer">AwaitTreeLayer</a>

Available on **crate feature `layers-await-tree`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#associatedtype.LayeredAccess-2" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#associatedtype.LayeredAccess" class="associatedtype">LayeredAccess</a> = AwaitTreeAccessor\<A\>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#impl-Layer%3CA%3E-for-CapabilityCheckLayer" class="anchor">Â§</a>

### impl\<A: <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html" class="trait" title="trait opendal::raw::Layer">Layer</a>\<A\> for <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.CapabilityCheckLayer.html" class="struct" title="struct opendal::layers::CapabilityCheckLayer">CapabilityCheckLayer</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#associatedtype.LayeredAccess-3" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#associatedtype.LayeredAccess" class="associatedtype">LayeredAccess</a> = CapabilityAccessor\<A\>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#impl-Layer%3CA%3E-for-ChaosLayer" class="anchor">Â§</a>

### impl\<A: <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html" class="trait" title="trait opendal::raw::Layer">Layer</a>\<A\> for <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.ChaosLayer.html" class="struct" title="struct opendal::layers::ChaosLayer">ChaosLayer</a>

Available on **crate feature `layers-chaos`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#associatedtype.LayeredAccess-4" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#associatedtype.LayeredAccess" class="associatedtype">LayeredAccess</a> = ChaosAccessor\<A\>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#impl-Layer%3CA%3E-for-ConcurrentLimitLayer" class="anchor">Â§</a>

### impl\<A: <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html" class="trait" title="trait opendal::raw::Layer">Layer</a>\<A\> for <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.ConcurrentLimitLayer.html" class="struct" title="struct opendal::layers::ConcurrentLimitLayer">ConcurrentLimitLayer</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#associatedtype.LayeredAccess-5" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#associatedtype.LayeredAccess" class="associatedtype">LayeredAccess</a> = ConcurrentLimitAccessor\<A\>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#impl-Layer%3CA%3E-for-DtraceLayer" class="anchor">Â§</a>

### impl\<A: <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html" class="trait" title="trait opendal::raw::Layer">Layer</a>\<A\> for <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.DtraceLayer.html" class="struct" title="struct opendal::layers::DtraceLayer">DtraceLayer</a>

Available on **Linux and crate feature `layers-dtrace`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#associatedtype.LayeredAccess-6" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#associatedtype.LayeredAccess" class="associatedtype">LayeredAccess</a> = DTraceAccessor\<A\>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#impl-Layer%3CA%3E-for-FastmetricsLayer" class="anchor">Â§</a>

### impl\<A: <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html" class="trait" title="trait opendal::raw::Layer">Layer</a>\<A\> for <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.FastmetricsLayer.html" class="struct" title="struct opendal::layers::FastmetricsLayer">FastmetricsLayer</a>

Available on **crate feature `layers-fastmetrics`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#associatedtype.LayeredAccess-7" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#associatedtype.LayeredAccess" class="associatedtype">LayeredAccess</a> = <a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/struct.MetricsAccessor.html" class="struct" title="struct opendal::layers::observe::MetricsAccessor">MetricsAccessor</a>\<A, FastmetricsInterceptor\>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#impl-Layer%3CA%3E-for-FastraceLayer" class="anchor">Â§</a>

### impl\<A: <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html" class="trait" title="trait opendal::raw::Layer">Layer</a>\<A\> for <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.FastraceLayer.html" class="struct" title="struct opendal::layers::FastraceLayer">FastraceLayer</a>

Available on **crate feature `layers-fastrace`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#associatedtype.LayeredAccess-8" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#associatedtype.LayeredAccess" class="associatedtype">LayeredAccess</a> = FastraceAccessor\<A\>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#impl-Layer%3CA%3E-for-HttpClientLayer" class="anchor">Â§</a>

### impl\<A: <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html" class="trait" title="trait opendal::raw::Layer">Layer</a>\<A\> for <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.HttpClientLayer.html" class="struct" title="struct opendal::layers::HttpClientLayer">HttpClientLayer</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#associatedtype.LayeredAccess-9" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#associatedtype.LayeredAccess" class="associatedtype">LayeredAccess</a> = HttpClientAccessor\<A\>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#impl-Layer%3CA%3E-for-ImmutableIndexLayer" class="anchor">Â§</a>

### impl\<A: <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html" class="trait" title="trait opendal::raw::Layer">Layer</a>\<A\> for <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.ImmutableIndexLayer.html" class="struct" title="struct opendal::layers::ImmutableIndexLayer">ImmutableIndexLayer</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#associatedtype.LayeredAccess-10" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#associatedtype.LayeredAccess" class="associatedtype">LayeredAccess</a> = ImmutableIndexAccessor\<A\>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#impl-Layer%3CA%3E-for-MetricsLayer" class="anchor">Â§</a>

### impl\<A: <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html" class="trait" title="trait opendal::raw::Layer">Layer</a>\<A\> for opendal::layers::<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.MetricsLayer.html" class="struct" title="struct opendal::layers::MetricsLayer">MetricsLayer</a>

Available on **crate feature `layers-metrics`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#associatedtype.LayeredAccess-11" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#associatedtype.LayeredAccess" class="associatedtype">LayeredAccess</a> = <a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/struct.MetricsAccessor.html" class="struct" title="struct opendal::layers::observe::MetricsAccessor">MetricsAccessor</a>\<A, MetricsInterceptor\>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#impl-Layer%3CA%3E-for-MimeGuessLayer" class="anchor">Â§</a>

### impl\<A: <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html" class="trait" title="trait opendal::raw::Layer">Layer</a>\<A\> for <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.MimeGuessLayer.html" class="struct" title="struct opendal::layers::MimeGuessLayer">MimeGuessLayer</a>

Available on **crate feature `layers-mime-guess`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#associatedtype.LayeredAccess-12" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#associatedtype.LayeredAccess" class="associatedtype">LayeredAccess</a> = MimeGuessAccessor\<A\>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#impl-Layer%3CA%3E-for-OtelMetricsLayer" class="anchor">Â§</a>

### impl\<A: <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html" class="trait" title="trait opendal::raw::Layer">Layer</a>\<A\> for <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.OtelMetricsLayer.html" class="struct" title="struct opendal::layers::OtelMetricsLayer">OtelMetricsLayer</a>

Available on **crate feature `layers-otel-metrics`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#associatedtype.LayeredAccess-13" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#associatedtype.LayeredAccess" class="associatedtype">LayeredAccess</a> = <a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/struct.MetricsAccessor.html" class="struct" title="struct opendal::layers::observe::MetricsAccessor">MetricsAccessor</a>\<A, OtelMetricsInterceptor\>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#impl-Layer%3CA%3E-for-OtelTraceLayer" class="anchor">Â§</a>

### impl\<A: <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html" class="trait" title="trait opendal::raw::Layer">Layer</a>\<A\> for <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.OtelTraceLayer.html" class="struct" title="struct opendal::layers::OtelTraceLayer">OtelTraceLayer</a>

Available on **crate feature `layers-otel-trace`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#associatedtype.LayeredAccess-14" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#associatedtype.LayeredAccess" class="associatedtype">LayeredAccess</a> = OtelTraceAccessor\<A\>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#impl-Layer%3CA%3E-for-PrometheusClientLayer" class="anchor">Â§</a>

### impl\<A: <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html" class="trait" title="trait opendal::raw::Layer">Layer</a>\<A\> for <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.PrometheusClientLayer.html" class="struct" title="struct opendal::layers::PrometheusClientLayer">PrometheusClientLayer</a>

Available on **crate feature `layers-prometheus-client`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#associatedtype.LayeredAccess-15" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#associatedtype.LayeredAccess" class="associatedtype">LayeredAccess</a> = <a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/struct.MetricsAccessor.html" class="struct" title="struct opendal::layers::observe::MetricsAccessor">MetricsAccessor</a>\<A, PrometheusClientInterceptor\>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#impl-Layer%3CA%3E-for-PrometheusLayer" class="anchor">Â§</a>

### impl\<A: <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html" class="trait" title="trait opendal::raw::Layer">Layer</a>\<A\> for <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.PrometheusLayer.html" class="struct" title="struct opendal::layers::PrometheusLayer">PrometheusLayer</a>

Available on **crate feature `layers-prometheus`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#associatedtype.LayeredAccess-16" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#associatedtype.LayeredAccess" class="associatedtype">LayeredAccess</a> = <a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/struct.MetricsAccessor.html" class="struct" title="struct opendal::layers::observe::MetricsAccessor">MetricsAccessor</a>\<A, PrometheusInterceptor\>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#impl-Layer%3CA%3E-for-TailCutLayer" class="anchor">Â§</a>

### impl\<A: <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html" class="trait" title="trait opendal::raw::Layer">Layer</a>\<A\> for <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.TailCutLayer.html" class="struct" title="struct opendal::layers::TailCutLayer">TailCutLayer</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#associatedtype.LayeredAccess-17" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#associatedtype.LayeredAccess" class="associatedtype">LayeredAccess</a> = TailCutAccessor\<A\>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#impl-Layer%3CA%3E-for-ThrottleLayer" class="anchor">Â§</a>

### impl\<A: <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html" class="trait" title="trait opendal::raw::Layer">Layer</a>\<A\> for <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.ThrottleLayer.html" class="struct" title="struct opendal::layers::ThrottleLayer">ThrottleLayer</a>

Available on **crate feature `layers-throttle`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#associatedtype.LayeredAccess-18" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#associatedtype.LayeredAccess" class="associatedtype">LayeredAccess</a> = ThrottleAccessor\<A\>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#impl-Layer%3CA%3E-for-TimeoutLayer" class="anchor">Â§</a>

### impl\<A: <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html" class="trait" title="trait opendal::raw::Layer">Layer</a>\<A\> for <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.TimeoutLayer.html" class="struct" title="struct opendal::layers::TimeoutLayer">TimeoutLayer</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#associatedtype.LayeredAccess-19" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#associatedtype.LayeredAccess" class="associatedtype">LayeredAccess</a> = TimeoutAccessor\<A\>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#impl-Layer%3CA%3E-for-TracingLayer" class="anchor">Â§</a>

### impl\<A: <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html" class="trait" title="trait opendal::raw::Layer">Layer</a>\<A\> for <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.TracingLayer.html" class="struct" title="struct opendal::layers::TracingLayer">TracingLayer</a>

Available on **crate feature `layers-tracing`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#associatedtype.LayeredAccess-20" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#associatedtype.LayeredAccess" class="associatedtype">LayeredAccess</a> = TracingAccessor\<A\>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#impl-Layer%3CA%3E-for-MetricsLayer%3CI%3E" class="anchor">Â§</a>

### impl\<A: <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>, I: <a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/trait.MetricsIntercept.html" class="trait" title="trait opendal::layers::observe::MetricsIntercept">MetricsIntercept</a>\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html" class="trait" title="trait opendal::raw::Layer">Layer</a>\<A\> for opendal::layers::observe::<a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/struct.MetricsLayer.html" class="struct" title="struct opendal::layers::observe::MetricsLayer">MetricsLayer</a>\<I\>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#associatedtype.LayeredAccess-21" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#associatedtype.LayeredAccess" class="associatedtype">LayeredAccess</a> = <a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/struct.MetricsAccessor.html" class="struct" title="struct opendal::layers::observe::MetricsAccessor">MetricsAccessor</a>\<A, I\>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#impl-Layer%3CA%3E-for-LoggingLayer%3CI%3E" class="anchor">Â§</a>

### impl\<A: <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>, I: <a href="https://opendal.apache.org/docs/rust/opendal/layers/trait.LoggingInterceptor.html" class="trait" title="trait opendal::layers::LoggingInterceptor">LoggingInterceptor</a>\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html" class="trait" title="trait opendal::raw::Layer">Layer</a>\<A\> for <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.LoggingLayer.html" class="struct" title="struct opendal::layers::LoggingLayer">LoggingLayer</a>\<I\>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#associatedtype.LayeredAccess-22" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#associatedtype.LayeredAccess" class="associatedtype">LayeredAccess</a> = LoggingAccessor\<A, I\>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#impl-Layer%3CA%3E-for-RetryLayer%3CI%3E" class="anchor">Â§</a>

### impl\<A: <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>, I: <a href="https://opendal.apache.org/docs/rust/opendal/layers/trait.RetryInterceptor.html" class="trait" title="trait opendal::layers::RetryInterceptor">RetryInterceptor</a>\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html" class="trait" title="trait opendal::raw::Layer">Layer</a>\<A\> for <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.RetryLayer.html" class="struct" title="struct opendal::layers::RetryLayer">RetryLayer</a>\<I\>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#associatedtype.LayeredAccess-23" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#associatedtype.LayeredAccess" class="associatedtype">LayeredAccess</a> = RetryAccessor\<A, I\>
