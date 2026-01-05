# Struct TracingLayer Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/layers/tracing.rs.html#149" class="src">Source</a>

``` rust
pub struct TracingLayer;
```

Available on **crate feature `layers-tracing`** only.

Expand description

Add [tracing](https://docs.rs/tracing/) for every operation.

## <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.TracingLayer.html#examples" class="doc-anchor">Â§</a>Examples

### <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.TracingLayer.html#basic-setup" class="doc-anchor">Â§</a>Basic Setup

``` rust

let _ = Operator::new(services::Memory::default())?
    .layer(TracingLayer)
    .finish();
Ok(())
```

### <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.TracingLayer.html#real-usage" class="doc-anchor">Â§</a>Real usage

``` rust

use opentelemetry::trace::TracerProvider;
let tracer_provider = opentelemetry_sdk::trace::SdkTracerProvider::builder()
    .with_simple_exporter(
        opentelemetry_otlp::SpanExporter::builder()
            .with_tonic()
            .build()?,
    )
    .with_resource(
        Resource::builder()
            .with_attributes(vec![KeyValue::new("service.name", "opendal_example")])
            .build(),
    )
    .build();
let tracer = tracer_provider.tracer("opendal_tracer");
let opentelemetry = tracing_opentelemetry::layer().with_tracer(tracer);

tracing_subscriber::registry()
    .with(EnvFilter::from_default_env())
    .with(opentelemetry)
    .try_init()?;

{
    let runtime = tokio::runtime::Runtime::new()?;
    runtime.block_on(async {
        let root = tracing::span!(tracing::Level::INFO, "app_start", work_units = 2);
        let _enter = root.enter();

        let _ = dotenvy::dotenv();
        let op = Operator::new(services::Memory::default())?
            .layer(TracingLayer)
            .finish();

        op.write("test", "0".repeat(16 * 1024 * 1024).into_bytes())
            .await?;
        op.stat("test").await?;
        op.read("test").await?;
        Ok::<(), opendal::Error>(())
    })?;
}

// Shut down the current tracer provider.
// This will invoke the shutdown method on all span processors.
// span processors should export remaining spans before return.
tracer_provider.shutdown()?;

Ok(())
```

## <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.TracingLayer.html#output" class="doc-anchor">Â§</a>Output

OpenDAL is using [`tracing`](https://docs.rs/tracing/latest/tracing/) for tracing internally.

To enable tracing output, please init one of the subscribers that `tracing` supports.

For example:

``` rust


let my_subscriber = FooSubscriber::new();
tracing::subscriber::set_global_default(my_subscriber).expect("setting tracing default failed");
```

For real-world usage, please take a look at [`tracing-opentelemetry`](https://crates.io/crates/tracing-opentelemetry).

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.TracingLayer.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.TracingLayer.html#impl-Layer%3CA%3E-for-TracingLayer" class="anchor">Â§</a>

### impl\<A: <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html" class="trait" title="trait opendal::raw::Layer">Layer</a>\<A\> for <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.TracingLayer.html" class="struct" title="struct opendal::layers::TracingLayer">TracingLayer</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.TracingLayer.html#associatedtype.LayeredAccess" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#associatedtype.LayeredAccess" class="associatedtype">LayeredAccess</a> = TracingAccessor\<A\>

The layered accessor that returned by this layer.

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.TracingLayer.html#method.layer" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#tymethod.layer" class="fn">layer</a>(&self, inner: A) -\> Self::<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#associatedtype.LayeredAccess" class="associatedtype" title="type opendal::raw::Layer::LayeredAccess">LayeredAccess</a>

Intercept the operations on the underlying storage.

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.TracingLayer.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.TracingLayer.html#blanket-implementations" class="anchor">Â§</a>
