# Struct OtelTraceLayer Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/layers/oteltrace.rs.html#52" class="src">Source</a>

``` rust
pub struct OtelTraceLayer;
```

Available on **crate feature `layers-otel-trace`** only.

Expand description

Add [opentelemetry::trace](https://docs.rs/opentelemetry/latest/opentelemetry/trace/index.html) for every operation.

Examples

### <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.OtelTraceLayer.html#basic-setup" class="doc-anchor">Â§</a>Basic Setup

``` rust

let _ = Operator::new(services::Memory::default())?
    .layer(OtelTraceLayer)
    .finish();
Ok(())
```

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.OtelTraceLayer.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.OtelTraceLayer.html#impl-Layer%3CA%3E-for-OtelTraceLayer" class="anchor">Â§</a>

### impl\<A: <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html" class="trait" title="trait opendal::raw::Layer">Layer</a>\<A\> for <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.OtelTraceLayer.html" class="struct" title="struct opendal::layers::OtelTraceLayer">OtelTraceLayer</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.OtelTraceLayer.html#associatedtype.LayeredAccess" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#associatedtype.LayeredAccess" class="associatedtype">LayeredAccess</a> = OtelTraceAccessor\<A\>

The layered accessor that returned by this layer.

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.OtelTraceLayer.html#method.layer" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#tymethod.layer" class="fn">layer</a>(&self, inner: A) -\> Self::<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#associatedtype.LayeredAccess" class="associatedtype" title="type opendal::raw::Layer::LayeredAccess">LayeredAccess</a>

Intercept the operations on the underlying storage.

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.OtelTraceLayer.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.OtelTraceLayer.html#blanket-implementations" class="anchor">Â§</a>
