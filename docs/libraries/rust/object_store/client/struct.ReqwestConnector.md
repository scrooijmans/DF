# Struct ReqwestConnector Copy item path

<a href="https://docs.rs/object_store/latest/src/object_store/client/http/connection.rs.html#294" class="src">Source</a>

``` rust
pub struct ReqwestConnector {}
```

Available on **crate feature `cloud` and not (WebAssembly and WASI)** only.

Expand description

[`HttpConnector`](https://docs.rs/object_store/latest/object_store/client/trait.HttpConnector.html "trait object_store::client::HttpConnector") using [`reqwest::Client`](https://docs.rs/reqwest/0.12.23/x86_64-unknown-linux-gnu/reqwest/async_impl/client/struct.Client.html "struct reqwest::async_impl::client::Client")

## Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/client/struct.ReqwestConnector.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/client/struct.ReqwestConnector.html#impl-Debug-for-ReqwestConnector" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/object_store/latest/object_store/client/struct.ReqwestConnector.html" class="struct" title="struct object_store::client::ReqwestConnector">ReqwestConnector</a>

<a href="https://docs.rs/object_store/latest/object_store/client/struct.ReqwestConnector.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/object_store/latest/object_store/client/struct.ReqwestConnector.html#impl-Default-for-ReqwestConnector" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/object_store/latest/object_store/client/struct.ReqwestConnector.html" class="struct" title="struct object_store::client::ReqwestConnector">ReqwestConnector</a>

<a href="https://docs.rs/object_store/latest/object_store/client/struct.ReqwestConnector.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/object_store/latest/object_store/client/struct.ReqwestConnector.html" class="struct" title="struct object_store::client::ReqwestConnector">ReqwestConnector</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/object_store/latest/object_store/client/struct.ReqwestConnector.html#impl-HttpConnector-for-ReqwestConnector" class="anchor">§</a>

### impl <a href="https://docs.rs/object_store/latest/object_store/client/trait.HttpConnector.html" class="trait" title="trait object_store::client::HttpConnector">HttpConnector</a> for <a href="https://docs.rs/object_store/latest/object_store/client/struct.ReqwestConnector.html" class="struct" title="struct object_store::client::ReqwestConnector">ReqwestConnector</a>

<a href="https://docs.rs/object_store/latest/object_store/client/struct.ReqwestConnector.html#method.connect" class="anchor">§</a>

#### fn <a href="https://docs.rs/object_store/latest/object_store/client/trait.HttpConnector.html#tymethod.connect" class="fn">connect</a>(&self, options: &<a href="https://docs.rs/object_store/latest/object_store/client/struct.ClientOptions.html" class="struct" title="struct object_store::client::ClientOptions">ClientOptions</a>) -\> <a href="https://docs.rs/object_store/latest/object_store/type.Result.html" class="type" title="type object_store::Result">Result</a>\<<a href="https://docs.rs/object_store/latest/object_store/client/struct.HttpClient.html" class="struct" title="struct object_store::client::HttpClient">HttpClient</a>\>

Create a new [`HttpClient`](https://docs.rs/object_store/latest/object_store/client/struct.HttpClient.html "struct object_store::client::HttpClient") with the provided [`ClientOptions`](https://docs.rs/object_store/latest/object_store/client/struct.ClientOptions.html "struct object_store::client::ClientOptions")

## Auto Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/client/struct.ReqwestConnector.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/object_store/latest/object_store/client/struct.ReqwestConnector.html#blanket-implementations" class="anchor">§</a>
