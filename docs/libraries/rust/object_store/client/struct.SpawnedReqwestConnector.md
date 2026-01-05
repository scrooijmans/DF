# Struct SpawnedReqwestConnector Copy item path

<a href="https://docs.rs/object_store/latest/src/object_store/client/http/connection.rs.html#342-344" class="src">Source</a>

``` rust
pub struct SpawnedReqwestConnector { /* private fields */ }
```

Available on **crate feature `cloud` and non-WebAssembly** only.

Expand description

[`reqwest::Client`](https://docs.rs/reqwest/0.12.23/x86_64-unknown-linux-gnu/reqwest/async_impl/client/struct.Client.html "struct reqwest::async_impl::client::Client") connector that performs all I/O on the provided tokio [`Runtime`](https://docs.rs/tokio/1.47.1/x86_64-unknown-linux-gnu/tokio/runtime/runtime/struct.Runtime.html "struct tokio::runtime::runtime::Runtime") (thread pool).

This adapter is most useful when you wish to segregate I/O from CPU bound work that may be happening on the [`Runtime`](https://docs.rs/tokio/1.47.1/x86_64-unknown-linux-gnu/tokio/runtime/runtime/struct.Runtime.html "struct tokio::runtime::runtime::Runtime").

## <a href="https://docs.rs/object_store/latest/object_store/client/struct.SpawnedReqwestConnector.html#example-spawning-requests-on-separate-runtime" class="doc-anchor">§</a>Example: Spawning requests on separate runtime

``` rust
// create a tokio runtime for I/O.
let io_runtime: Runtime = get_io_runtime();
// configure a store using the runtime.
let handle = io_runtime.handle().clone(); // get a handle to the same runtime
let store: Arc<dyn ObjectStore> = Arc::new(
  MicrosoftAzureBuilder::new()
    .with_http_connector(SpawnedReqwestConnector::new(handle))
    .with_container_name("my_container")
    .with_account("my_account")
    .build()?
 );
// any requests made using store will be spawned on the io_runtime
```

## Implementations<a href="https://docs.rs/object_store/latest/object_store/client/struct.SpawnedReqwestConnector.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/client/struct.SpawnedReqwestConnector.html#impl-SpawnedReqwestConnector" class="anchor">§</a>

### impl <a href="https://docs.rs/object_store/latest/object_store/client/struct.SpawnedReqwestConnector.html" class="struct" title="struct object_store::client::SpawnedReqwestConnector">SpawnedReqwestConnector</a>

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/client/struct.SpawnedReqwestConnector.html#method.new" class="fn">new</a>(runtime: <a href="https://docs.rs/tokio/1.47.1/x86_64-unknown-linux-gnu/tokio/runtime/handle/struct.Handle.html" class="struct" title="struct tokio::runtime::handle::Handle">Handle</a>) -\> Self

Create a new [`SpawnedReqwestConnector`](https://docs.rs/object_store/latest/object_store/client/struct.SpawnedReqwestConnector.html "struct object_store::client::SpawnedReqwestConnector") with the provided [`Handle`](https://docs.rs/tokio/1.47.1/x86_64-unknown-linux-gnu/tokio/runtime/handle/struct.Handle.html "struct tokio::runtime::handle::Handle") to a tokio [`Runtime`](https://docs.rs/tokio/1.47.1/x86_64-unknown-linux-gnu/tokio/runtime/runtime/struct.Runtime.html "struct tokio::runtime::runtime::Runtime")

## Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/client/struct.SpawnedReqwestConnector.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/client/struct.SpawnedReqwestConnector.html#impl-Debug-for-SpawnedReqwestConnector" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/object_store/latest/object_store/client/struct.SpawnedReqwestConnector.html" class="struct" title="struct object_store::client::SpawnedReqwestConnector">SpawnedReqwestConnector</a>

<a href="https://docs.rs/object_store/latest/object_store/client/struct.SpawnedReqwestConnector.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/object_store/latest/object_store/client/struct.SpawnedReqwestConnector.html#impl-HttpConnector-for-SpawnedReqwestConnector" class="anchor">§</a>

### impl <a href="https://docs.rs/object_store/latest/object_store/client/trait.HttpConnector.html" class="trait" title="trait object_store::client::HttpConnector">HttpConnector</a> for <a href="https://docs.rs/object_store/latest/object_store/client/struct.SpawnedReqwestConnector.html" class="struct" title="struct object_store::client::SpawnedReqwestConnector">SpawnedReqwestConnector</a>

<a href="https://docs.rs/object_store/latest/object_store/client/struct.SpawnedReqwestConnector.html#method.connect" class="anchor">§</a>

#### fn <a href="https://docs.rs/object_store/latest/object_store/client/trait.HttpConnector.html#tymethod.connect" class="fn">connect</a>(&self, options: &<a href="https://docs.rs/object_store/latest/object_store/client/struct.ClientOptions.html" class="struct" title="struct object_store::client::ClientOptions">ClientOptions</a>) -\> <a href="https://docs.rs/object_store/latest/object_store/type.Result.html" class="type" title="type object_store::Result">Result</a>\<<a href="https://docs.rs/object_store/latest/object_store/client/struct.HttpClient.html" class="struct" title="struct object_store::client::HttpClient">HttpClient</a>\>

Create a new [`HttpClient`](https://docs.rs/object_store/latest/object_store/client/struct.HttpClient.html "struct object_store::client::HttpClient") with the provided [`ClientOptions`](https://docs.rs/object_store/latest/object_store/client/struct.ClientOptions.html "struct object_store::client::ClientOptions")

## Auto Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/client/struct.SpawnedReqwestConnector.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/object_store/latest/object_store/client/struct.SpawnedReqwestConnector.html#blanket-implementations" class="anchor">§</a>
