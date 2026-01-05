# Struct FastraceLayer Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/layers/fastrace.rs.html#106" class="src">Source</a>

``` rust
pub struct FastraceLayer;
```

Available on **crate feature `layers-fastrace`** only.

Expand description

Add [fastrace](https://docs.rs/fastrace/) for every operation.

## <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.FastraceLayer.html#examples" class="doc-anchor">Â§</a>Examples

### <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.FastraceLayer.html#basic-setup" class="doc-anchor">Â§</a>Basic Setup

``` rust

let _ = Operator::new(services::Memory::default())?
    .layer(FastraceLayer)
    .finish();
Ok(())
```

### <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.FastraceLayer.html#real-usage" class="doc-anchor">Â§</a>Real usage

``` rust

let reporter =
    fastrace_jaeger::JaegerReporter::new("127.0.0.1:6831".parse()?, "opendal").unwrap();
fastrace::set_reporter(reporter, fastrace::collector::Config::default());

{
    let root = Span::root("op", SpanContext::random());
    let runtime = tokio::runtime::Runtime::new()?;
    runtime.block_on(
        async {
            let _ = dotenvy::dotenv();
            let op = Operator::new(services::Memory::default())?
                .layer(FastraceLayer)
                .finish();
            op.write("test", "0".repeat(16 * 1024 * 1024).into_bytes())
                .await?;
            op.stat("test").await?;
            op.read("test").await?;
            Ok::<(), opendal::Error>(())
        }
        .in_span(Span::enter_with_parent("test", &root)),
    )?;
}

fastrace::flush();

Ok(())
```

## <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.FastraceLayer.html#output" class="doc-anchor">Â§</a>Output

OpenDAL is using [`fastrace`](https://docs.rs/fastrace/latest/fastrace/) for tracing internally.

To enable fastrace output, please init one of the reporter that `fastrace` supports.

For example:

``` rust

let reporter =
    fastrace_jaeger::JaegerReporter::new("127.0.0.1:6831".parse()?, "opendal").unwrap();
fastrace::set_reporter(reporter, fastrace::collector::Config::default());
Ok(())
```

For real-world usage, please take a look at [`fastrace-datadog`](https://crates.io/crates/fastrace-datadog) or [`fastrace-jaeger`](https://crates.io/crates/fastrace-jaeger) .

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.FastraceLayer.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.FastraceLayer.html#impl-Layer%3CA%3E-for-FastraceLayer" class="anchor">Â§</a>

### impl\<A: <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html" class="trait" title="trait opendal::raw::Layer">Layer</a>\<A\> for <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.FastraceLayer.html" class="struct" title="struct opendal::layers::FastraceLayer">FastraceLayer</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.FastraceLayer.html#associatedtype.LayeredAccess" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#associatedtype.LayeredAccess" class="associatedtype">LayeredAccess</a> = FastraceAccessor\<A\>

The layered accessor that returned by this layer.

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.FastraceLayer.html#method.layer" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#tymethod.layer" class="fn">layer</a>(&self, inner: A) -\> Self::<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#associatedtype.LayeredAccess" class="associatedtype" title="type opendal::raw::Layer::LayeredAccess">LayeredAccess</a>

Intercept the operations on the underlying storage.

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.FastraceLayer.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.FastraceLayer.html#blanket-implementations" class="anchor">Â§</a>
