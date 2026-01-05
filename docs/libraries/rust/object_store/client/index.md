# Module client Copy item path

<a href="https://docs.rs/object_store/latest/src/object_store/client/mod.rs.html#18-1047" class="src">Source</a>

Available on **crate feature `cloud`** only.

Expand description

Generic utilities for [`reqwest`](https://docs.rs/reqwest/0.12.23/x86_64-unknown-linux-gnu/reqwest/index.html "mod reqwest") based [`ObjectStore`](https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html "trait object_store::ObjectStore") implementations

## Structs<a href="https://docs.rs/object_store/latest/object_store/client/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/client/struct.Certificate.html" class="struct" title="struct object_store::client::Certificate">Certificate</a>Non-WebAssembly  
Represents a CA certificate provided by the user.

<a href="https://docs.rs/object_store/latest/object_store/client/struct.ClientOptions.html" class="struct" title="struct object_store::client::ClientOptions">ClientOptions</a>  
HTTP client configuration for remote object stores

<a href="https://docs.rs/object_store/latest/object_store/client/struct.HttpClient.html" class="struct" title="struct object_store::client::HttpClient">HttpClient</a>  
An HTTP client

<a href="https://docs.rs/object_store/latest/object_store/client/struct.HttpError.html" class="struct" title="struct object_store::client::HttpError">HttpError</a>  
An HTTP protocol error

<a href="https://docs.rs/object_store/latest/object_store/client/struct.HttpRequestBody.html" class="struct" title="struct object_store::client::HttpRequestBody">HttpRequestBody</a>  
The [`Body`](https://docs.rs/http-body/1.0.1/x86_64-unknown-linux-gnu/http_body/trait.Body.html "trait http_body::Body") of an [`HttpRequest`](https://docs.rs/object_store/latest/object_store/client/type.HttpRequest.html "type object_store::client::HttpRequest")

<a href="https://docs.rs/object_store/latest/object_store/client/struct.HttpResponseBody.html" class="struct" title="struct object_store::client::HttpResponseBody">HttpResponseBody</a>  
The body of an [`HttpResponse`](https://docs.rs/object_store/latest/object_store/client/type.HttpResponse.html "type object_store::client::HttpResponse")

<a href="https://docs.rs/object_store/latest/object_store/client/struct.ReqwestConnector.html" class="struct" title="struct object_store::client::ReqwestConnector">ReqwestConnector</a>Not (WebAssembly and WASI)  
[`HttpConnector`](https://docs.rs/object_store/latest/object_store/client/trait.HttpConnector.html "trait object_store::client::HttpConnector") using [`reqwest::Client`](https://docs.rs/reqwest/0.12.23/x86_64-unknown-linux-gnu/reqwest/async_impl/client/struct.Client.html "struct reqwest::async_impl::client::Client")

<a href="https://docs.rs/object_store/latest/object_store/client/struct.SpawnService.html" class="struct" title="struct object_store::client::SpawnService">SpawnService</a>  
Wraps a provided [`HttpService`](https://docs.rs/object_store/latest/object_store/client/trait.HttpService.html "trait object_store::client::HttpService") and runs it on a separate tokio runtime

<a href="https://docs.rs/object_store/latest/object_store/client/struct.SpawnedReqwestConnector.html" class="struct" title="struct object_store::client::SpawnedReqwestConnector">SpawnedReqwestConnector</a>Non-WebAssembly  
[`reqwest::Client`](https://docs.rs/reqwest/0.12.23/x86_64-unknown-linux-gnu/reqwest/async_impl/client/struct.Client.html "struct reqwest::async_impl::client::Client") connector that performs all I/O on the provided tokio [`Runtime`](https://docs.rs/tokio/1.47.1/x86_64-unknown-linux-gnu/tokio/runtime/runtime/struct.Runtime.html "struct tokio::runtime::runtime::Runtime") (thread pool).

<a href="https://docs.rs/object_store/latest/object_store/client/struct.StaticCredentialProvider.html" class="struct" title="struct object_store::client::StaticCredentialProvider">StaticCredentialProvider</a>  
A static set of credentials

## Enums<a href="https://docs.rs/object_store/latest/object_store/client/index.html#enums" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/client/enum.ClientConfigKey.html" class="enum" title="enum object_store::client::ClientConfigKey">ClientConfigKey</a>  
Configuration keys for [`ClientOptions`](https://docs.rs/object_store/latest/object_store/client/struct.ClientOptions.html "struct object_store::client::ClientOptions")

<a href="https://docs.rs/object_store/latest/object_store/client/enum.HttpErrorKind.html" class="enum" title="enum object_store::client::HttpErrorKind">HttpErrorKind</a>  
Identifies the kind of [`HttpError`](https://docs.rs/object_store/latest/object_store/client/struct.HttpError.html "struct object_store::client::HttpError")

## Traits<a href="https://docs.rs/object_store/latest/object_store/client/index.html#traits" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/client/trait.CredentialProvider.html" class="trait" title="trait object_store::client::CredentialProvider">CredentialProvider</a>  
Provides credentials for use when signing requests

<a href="https://docs.rs/object_store/latest/object_store/client/trait.HttpConnector.html" class="trait" title="trait object_store::client::HttpConnector">HttpConnector</a>  
A factory for [`HttpClient`](https://docs.rs/object_store/latest/object_store/client/struct.HttpClient.html "struct object_store::client::HttpClient")

<a href="https://docs.rs/object_store/latest/object_store/client/trait.HttpService.html" class="trait" title="trait object_store::client::HttpService">HttpService</a>  
An asynchronous function from a [`HttpRequest`](https://docs.rs/object_store/latest/object_store/client/type.HttpRequest.html "type object_store::client::HttpRequest") to a [`HttpResponse`](https://docs.rs/object_store/latest/object_store/client/type.HttpResponse.html "type object_store::client::HttpResponse").

## Type Aliases<a href="https://docs.rs/object_store/latest/object_store/client/index.html#types" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/client/type.HttpRequest.html" class="type" title="type object_store::client::HttpRequest">HttpRequest</a>  
An HTTP Request

<a href="https://docs.rs/object_store/latest/object_store/client/type.HttpResponse.html" class="type" title="type object_store::client::HttpResponse">HttpResponse</a>  
An HTTP response
