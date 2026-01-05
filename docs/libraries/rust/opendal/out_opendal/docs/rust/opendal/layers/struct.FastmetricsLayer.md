# Struct FastmetricsLayer Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/layers/fastmetrics.rs.html#129-131" class="src">Source</a>

``` rust
pub struct FastmetricsLayer { /* private fields */ }
```

Available on **crate feature `layers-fastmetrics`** only.

Expand description

Add [fastmetrics](https://docs.rs/fastmetrics/) for every operation.

## <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.FastmetricsLayer.html#examples" class="doc-anchor">Â§</a>Examples

### <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.FastmetricsLayer.html#basic-usage" class="doc-anchor">Â§</a>Basic Usage

``` rust

let mut registry = fastmetrics::registry::Registry::default();
let op = Operator::new(services::Memory::default())?
    .layer(FastmetricsLayer::builder().register(&mut registry)?)
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
let mut output = String::new();
text::encode(&mut output, &registry).unwrap();
println!("{}", output);
```

### <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.FastmetricsLayer.html#global-instance" class="doc-anchor">Â§</a>Global Instance

`FastmetricsLayer` needs to be registered before instantiation.

If there are multiple operators in an application that need the `FastmetricsLayer`, we could instantiate it once and pass it to multiple operators. But we cannot directly call `.layer(FastmetricsLayer::builder().register(&mut registry)?)` for different services, because registering the same metrics multiple times to the same registry will cause register errors. Therefore, we can provide a global instance for the `FastmetricsLayer`.

``` rust

fn global_fastmetrics_layer() -> &'static FastmetricsLayer {
    static GLOBAL: OnceLock<FastmetricsLayer> = OnceLock::new();
    GLOBAL.get_or_init(|| {
        FastmetricsLayer::builder()
            .register_global()
            .expect("Failed to register with the global registry")
    })
}

let op = Operator::new(services::Memory::default())?
    .layer(global_fastmetrics_layer().clone())
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
let mut output = String::new();
with_global_registry(|registry| text::encode(&mut output, &registry).unwrap());
println!("{}", output);
```

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.FastmetricsLayer.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.FastmetricsLayer.html#impl-FastmetricsLayer" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.FastmetricsLayer.html" class="struct" title="struct opendal::layers::FastmetricsLayer">FastmetricsLayer</a>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.FastmetricsLayer.html#method.builder" class="fn">builder</a>() -\> <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.FastmetricsLayerBuilder.html" class="struct" title="struct opendal::layers::FastmetricsLayerBuilder">FastmetricsLayerBuilder</a>

Create a [`FastmetricsLayerBuilder`](https://opendal.apache.org/docs/rust/opendal/layers/struct.FastmetricsLayerBuilder.html "struct opendal::layers::FastmetricsLayerBuilder") to set the configuration of metrics.

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.FastmetricsLayer.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.FastmetricsLayer.html#impl-Clone-for-FastmetricsLayer" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.FastmetricsLayer.html" class="struct" title="struct opendal::layers::FastmetricsLayer">FastmetricsLayer</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.FastmetricsLayer.html#method.clone" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.FastmetricsLayer.html" class="struct" title="struct opendal::layers::FastmetricsLayer">FastmetricsLayer</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.FastmetricsLayer.html#method.clone_from" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.FastmetricsLayer.html#impl-Debug-for-FastmetricsLayer" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.FastmetricsLayer.html" class="struct" title="struct opendal::layers::FastmetricsLayer">FastmetricsLayer</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.FastmetricsLayer.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.FastmetricsLayer.html#impl-Layer%3CA%3E-for-FastmetricsLayer" class="anchor">Â§</a>

### impl\<A: <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html" class="trait" title="trait opendal::raw::Layer">Layer</a>\<A\> for <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.FastmetricsLayer.html" class="struct" title="struct opendal::layers::FastmetricsLayer">FastmetricsLayer</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.FastmetricsLayer.html#associatedtype.LayeredAccess" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#associatedtype.LayeredAccess" class="associatedtype">LayeredAccess</a> = <a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/struct.MetricsAccessor.html" class="struct" title="struct opendal::layers::observe::MetricsAccessor">MetricsAccessor</a>\<A, FastmetricsInterceptor\>

The layered accessor that returned by this layer.

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.FastmetricsLayer.html#method.layer" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#tymethod.layer" class="fn">layer</a>(&self, inner: A) -\> Self::<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#associatedtype.LayeredAccess" class="associatedtype" title="type opendal::raw::Layer::LayeredAccess">LayeredAccess</a>

Intercept the operations on the underlying storage.

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.FastmetricsLayer.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.FastmetricsLayer.html#blanket-implementations" class="anchor">Â§</a>
