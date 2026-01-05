# Struct HttpClientLayer Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/layers/http_client.rs.html#68-70" class="src">Source</a>

``` rust
pub struct HttpClientLayer { /* private fields */ }
```

Expand description

Layer for replacing the default HTTP client with a custom one.

This layer allows users to provide their own HTTP client implementation by implementing the [`HttpFetch`](https://opendal.apache.org/docs/rust/opendal/raw/trait.HttpFetch.html "trait opendal::raw::HttpFetch") trait. This is useful when you need to customize HTTP behavior, add authentication, use a different HTTP client library, or apply custom middleware.

## <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.HttpClientLayer.html#examples" class="doc-anchor">Â§</a>Examples

``` rust
use opendal::layers::HttpClientLayer;
use opendal::services;
use opendal::Operator;
use opendal::Result;
use opendal::raw::HttpClient;

// Create a custom HTTP client
let custom_client = HttpClient::new()?;

let op = Operator::new(services::S3::default())?
    .layer(HttpClientLayer::new(custom_client))
    .finish();
```

## <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.HttpClientLayer.html#custom-http-client-implementation" class="doc-anchor">Â§</a>Custom HTTP Client Implementation

``` rust
use opendal::raw::{HttpFetch, HttpBody};
use opendal::Buffer;
use http::{Request, Response};
use opendal::Result;

struct CustomHttpClient {
    // Your custom HTTP client fields
}

impl HttpFetch for CustomHttpClient {
    async fn fetch(&self, req: Request<Buffer>) -> Result<Response<HttpBody>> {
        // Your custom HTTP client implementation
        todo!()
    }
}
```

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.HttpClientLayer.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.HttpClientLayer.html#impl-HttpClientLayer" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.HttpClientLayer.html" class="struct" title="struct opendal::layers::HttpClientLayer">HttpClientLayer</a>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.HttpClientLayer.html#method.new" class="fn">new</a>(client: <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.HttpClient.html" class="struct" title="struct opendal::raw::HttpClient">HttpClient</a>) -\> Self

Create a new `HttpClientLayer` with the given HTTP client.

##### <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.HttpClientLayer.html#arguments" class="doc-anchor">Â§</a>Arguments

- `client` - The HTTP client to use for all HTTP requests

##### <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.HttpClientLayer.html#examples-1" class="doc-anchor">Â§</a>Examples

``` rust
use opendal::layers::HttpClientLayer;
use opendal::raw::HttpClient;
use opendal::Result;

let client = HttpClient::new()?;
let layer = HttpClientLayer::new(client);
```

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.HttpClientLayer.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.HttpClientLayer.html#impl-Clone-for-HttpClientLayer" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.HttpClientLayer.html" class="struct" title="struct opendal::layers::HttpClientLayer">HttpClientLayer</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.HttpClientLayer.html#method.clone" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.HttpClientLayer.html" class="struct" title="struct opendal::layers::HttpClientLayer">HttpClientLayer</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.HttpClientLayer.html#method.clone_from" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.HttpClientLayer.html#impl-Layer%3CA%3E-for-HttpClientLayer" class="anchor">Â§</a>

### impl\<A: <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html" class="trait" title="trait opendal::raw::Layer">Layer</a>\<A\> for <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.HttpClientLayer.html" class="struct" title="struct opendal::layers::HttpClientLayer">HttpClientLayer</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.HttpClientLayer.html#associatedtype.LayeredAccess" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#associatedtype.LayeredAccess" class="associatedtype">LayeredAccess</a> = HttpClientAccessor\<A\>

The layered accessor that returned by this layer.

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.HttpClientLayer.html#method.layer" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#tymethod.layer" class="fn">layer</a>(&self, inner: A) -\> Self::<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#associatedtype.LayeredAccess" class="associatedtype" title="type opendal::raw::Layer::LayeredAccess">LayeredAccess</a>

Intercept the operations on the underlying storage.

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.HttpClientLayer.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.HttpClientLayer.html#blanket-implementations" class="anchor">Â§</a>
