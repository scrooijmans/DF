# Struct LoggingLayer Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/layers/logging.rs.html#112-114" class="src">Source</a>

``` rust
pub struct LoggingLayer<I = DefaultLoggingInterceptor> { /* private fields */ }
```

Expand description

Add [log](https://docs.rs/log/) for every operation.

## <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.LoggingLayer.html#logging" class="doc-anchor">Â§</a>Logging

- OpenDAL will log in structural way.
- Every operation will start with a `started` log entry.
- Every operation will finish with the following status:
  - `succeeded`: the operation is successful, but might have more to take.
  - `finished`: the whole operation is finished.
  - `failed`: the operation returns an unexpected error.
- The default log level while expected error happened is `Warn`.
- The default log level while unexpected failure happened is `Error`.

## <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.LoggingLayer.html#examples" class="doc-anchor">Â§</a>Examples

``` rust

let _ = Operator::new(services::Memory::default())?
    .layer(LoggingLayer::default())
    .finish();
Ok(())
```

## <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.LoggingLayer.html#output" class="doc-anchor">Â§</a>Output

OpenDAL is using [`log`](https://docs.rs/log/latest/log/) for logging internally.

To enable logging output, please set `RUST_LOG`:

``` shell
RUST_LOG=debug ./app
```

To config logging output, please refer to [Configure Logging](https://rust-lang-nursery.github.io/rust-cookbook/development_tools/debugging/config_log.html):

``` shell
RUST_LOG="info,opendal::services=debug" ./app
```

## <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.LoggingLayer.html#logging-interceptor" class="doc-anchor">Â§</a>Logging Interceptor

You can implement your own logging interceptor to customize the logging behavior.

``` rust

#[derive(Debug, Clone)]
struct MyLoggingInterceptor;

impl LoggingInterceptor for MyLoggingInterceptor {
    fn log(
        &self,
        info: &raw::AccessorInfo,
        operation: raw::Operation,
        context: &[(&str, &str)],
        message: &str,
        err: Option<&Error>,
    ) {
        // log something
    }
}

let _ = Operator::new(services::Memory::default())?
    .layer(LoggingLayer::new(MyLoggingInterceptor))
    .finish();
Ok(())
```

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.LoggingLayer.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.LoggingLayer.html#impl-LoggingLayer" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.LoggingLayer.html" class="struct" title="struct opendal::layers::LoggingLayer">LoggingLayer</a>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.LoggingLayer.html#method.new" class="fn">new</a>\<I: <a href="https://opendal.apache.org/docs/rust/opendal/layers/trait.LoggingInterceptor.html" class="trait" title="trait opendal::layers::LoggingInterceptor">LoggingInterceptor</a>\>(logger: I) -\> <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.LoggingLayer.html" class="struct" title="struct opendal::layers::LoggingLayer">LoggingLayer</a>\<I\>

Create the layer with specific logging interceptor.

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.LoggingLayer.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.LoggingLayer.html#impl-Debug-for-LoggingLayer%3CI%3E" class="anchor">Â§</a>

### impl\<I: <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a>\> <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.LoggingLayer.html" class="struct" title="struct opendal::layers::LoggingLayer">LoggingLayer</a>\<I\>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.LoggingLayer.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.LoggingLayer.html#impl-Default-for-LoggingLayer" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.LoggingLayer.html" class="struct" title="struct opendal::layers::LoggingLayer">LoggingLayer</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.LoggingLayer.html#method.default" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> Self

Returns the â€œdefault valueâ€? for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.LoggingLayer.html#impl-Layer%3CA%3E-for-LoggingLayer%3CI%3E" class="anchor">Â§</a>

### impl\<A: <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>, I: <a href="https://opendal.apache.org/docs/rust/opendal/layers/trait.LoggingInterceptor.html" class="trait" title="trait opendal::layers::LoggingInterceptor">LoggingInterceptor</a>\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html" class="trait" title="trait opendal::raw::Layer">Layer</a>\<A\> for <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.LoggingLayer.html" class="struct" title="struct opendal::layers::LoggingLayer">LoggingLayer</a>\<I\>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.LoggingLayer.html#associatedtype.LayeredAccess" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#associatedtype.LayeredAccess" class="associatedtype">LayeredAccess</a> = LoggingAccessor\<A, I\>

The layered accessor that returned by this layer.

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.LoggingLayer.html#method.layer" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#tymethod.layer" class="fn">layer</a>(&self, inner: A) -\> Self::<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#associatedtype.LayeredAccess" class="associatedtype" title="type opendal::raw::Layer::LayeredAccess">LayeredAccess</a>

Intercept the operations on the underlying storage.

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.LoggingLayer.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.LoggingLayer.html#blanket-implementations" class="anchor">Â§</a>
