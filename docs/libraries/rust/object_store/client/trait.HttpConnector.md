# Trait HttpConnector Copy item path

<a href="https://docs.rs/object_store/latest/src/object_store/client/http/connection.rs.html#285-288" class="src">Source</a>

``` rust
pub trait HttpConnector:
    Debug
    + Send
    + Sync
    + 'static {
    // Required method
    fn connect(&self, options: &ClientOptions) -> Result<HttpClient>;
}
```

Available on **crate feature `cloud`** only.

Expand description

A factory for [`HttpClient`](https://docs.rs/object_store/latest/object_store/client/struct.HttpClient.html "struct object_store::client::HttpClient")

## Required Methods<a href="https://docs.rs/object_store/latest/object_store/client/trait.HttpConnector.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/object_store/latest/object_store/client/trait.HttpConnector.html#tymethod.connect" class="fn">connect</a>(&self, options: &<a href="https://docs.rs/object_store/latest/object_store/client/struct.ClientOptions.html" class="struct" title="struct object_store::client::ClientOptions">ClientOptions</a>) -\> <a href="https://docs.rs/object_store/latest/object_store/type.Result.html" class="type" title="type object_store::Result">Result</a>\<<a href="https://docs.rs/object_store/latest/object_store/client/struct.HttpClient.html" class="struct" title="struct object_store::client::HttpClient">HttpClient</a>\>

Create a new [`HttpClient`](https://docs.rs/object_store/latest/object_store/client/struct.HttpClient.html "struct object_store::client::HttpClient") with the provided [`ClientOptions`](https://docs.rs/object_store/latest/object_store/client/struct.ClientOptions.html "struct object_store::client::ClientOptions")

## Implementors<a href="https://docs.rs/object_store/latest/object_store/client/trait.HttpConnector.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/client/trait.HttpConnector.html#impl-HttpConnector-for-ReqwestConnector" class="anchor">§</a>

### impl <a href="https://docs.rs/object_store/latest/object_store/client/trait.HttpConnector.html" class="trait" title="trait object_store::client::HttpConnector">HttpConnector</a> for <a href="https://docs.rs/object_store/latest/object_store/client/struct.ReqwestConnector.html" class="struct" title="struct object_store::client::ReqwestConnector">ReqwestConnector</a>

Available on **not (WebAssembly and WASI)**.

<a href="https://docs.rs/object_store/latest/object_store/client/trait.HttpConnector.html#impl-HttpConnector-for-SpawnedReqwestConnector" class="anchor">§</a>

### impl <a href="https://docs.rs/object_store/latest/object_store/client/trait.HttpConnector.html" class="trait" title="trait object_store::client::HttpConnector">HttpConnector</a> for <a href="https://docs.rs/object_store/latest/object_store/client/struct.SpawnedReqwestConnector.html" class="struct" title="struct object_store::client::SpawnedReqwestConnector">SpawnedReqwestConnector</a>

Available on **non-WebAssembly** only.
