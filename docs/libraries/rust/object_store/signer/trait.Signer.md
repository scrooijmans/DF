# Trait Signer Copy item path

<a href="https://docs.rs/object_store/latest/src/object_store/signer.rs.html#28-50" class="src">Source</a>

``` rust
pub trait Signer:
    Send
    + Sync
    + Debug
    + 'static {
    // Required method
    fn signed_url<'life0, 'life1, 'async_trait>(
        &'life0 self,
        method: Method,
        path: &'life1 Path,
        expires_in: Duration,
    ) -> Pin<Box<dyn Future<Output = Result<Url>> + Send + 'async_trait>>
       where Self: 'async_trait,
             'life0: 'async_trait,
             'life1: 'async_trait;

    // Provided method
    fn signed_urls<'life0, 'life1, 'async_trait>(
        &'life0 self,
        method: Method,
        paths: &'life1 [Path],
        expires_in: Duration,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<Url>>> + Send + 'async_trait>>
       where Self: 'async_trait,
             'life0: 'async_trait,
             'life1: 'async_trait { ... }
}
```

Available on **crate feature `cloud`** only.

Expand description

Universal API to generate presigned URLs from multiple object store services.

## Required Methods<a href="https://docs.rs/object_store/latest/object_store/signer/trait.Signer.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/object_store/latest/object_store/signer/trait.Signer.html#tymethod.signed_url" class="fn">signed_url</a>\<'life0, 'life1, 'async_trait\>( &'life0 self, method: <a href="https://docs.rs/http/1.3.1/x86_64-unknown-linux-gnu/http/method/struct.Method.html" class="struct" title="struct http::method::Method">Method</a>, path: &'life1 <a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>, expires_in: <a href="https://doc.rust-lang.org/nightly/core/time/struct.Duration.html" class="struct" title="struct core::time::Duration">Duration</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://docs.rs/object_store/latest/object_store/type.Result.html" class="type" title="type object_store::Result">Result</a>\<<a href="https://docs.rs/url/2.5.7/x86_64-unknown-linux-gnu/url/struct.Url.html" class="struct" title="struct url::Url">Url</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where Self: 'async_trait, 'life0: 'async_trait, 'life1: 'async_trait,

Given the intended [`Method`](https://docs.rs/http/1.3.1/x86_64-unknown-linux-gnu/http/method/struct.Method.html "struct http::method::Method") and [`Path`](https://docs.rs/object_store/latest/object_store/path/struct.Path.html "struct object_store::path::Path") to use and the desired length of time for which the URL should be valid, return a signed [`Url`](https://docs.rs/url/2.5.7/x86_64-unknown-linux-gnu/url/struct.Url.html "struct url::Url") created with the object store implementation’s credentials such that the URL can be handed to something that doesn’t have access to the object store’s credentials, to allow limited access to the object store.

## Provided Methods<a href="https://docs.rs/object_store/latest/object_store/signer/trait.Signer.html#provided-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/object_store/latest/object_store/signer/trait.Signer.html#method.signed_urls" class="fn">signed_urls</a>\<'life0, 'life1, 'async_trait\>( &'life0 self, method: <a href="https://docs.rs/http/1.3.1/x86_64-unknown-linux-gnu/http/method/struct.Method.html" class="struct" title="struct http::method::Method">Method</a>, paths: &'life1 \[<a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>\], expires_in: <a href="https://doc.rust-lang.org/nightly/core/time/struct.Duration.html" class="struct" title="struct core::time::Duration">Duration</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://docs.rs/object_store/latest/object_store/type.Result.html" class="type" title="type object_store::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/url/2.5.7/x86_64-unknown-linux-gnu/url/struct.Url.html" class="struct" title="struct url::Url">Url</a>\>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where Self: 'async_trait, 'life0: 'async_trait, 'life1: 'async_trait,

Generate signed urls for multiple paths.

See [`Signer::signed_url`](https://docs.rs/object_store/latest/object_store/signer/trait.Signer.html#tymethod.signed_url "method object_store::signer::Signer::signed_url") for more details.

## Implementors<a href="https://docs.rs/object_store/latest/object_store/signer/trait.Signer.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/signer/trait.Signer.html#impl-Signer-for-AmazonS3" class="anchor">§</a>

### impl <a href="https://docs.rs/object_store/latest/object_store/signer/trait.Signer.html" class="trait" title="trait object_store::signer::Signer">Signer</a> for <a href="https://docs.rs/object_store/latest/object_store/aws/struct.AmazonS3.html" class="struct" title="struct object_store::aws::AmazonS3">AmazonS3</a>

Available on **crate feature `aws`** only.

<a href="https://docs.rs/object_store/latest/object_store/signer/trait.Signer.html#impl-Signer-for-MicrosoftAzure" class="anchor">§</a>

### impl <a href="https://docs.rs/object_store/latest/object_store/signer/trait.Signer.html" class="trait" title="trait object_store::signer::Signer">Signer</a> for <a href="https://docs.rs/object_store/latest/object_store/azure/struct.MicrosoftAzure.html" class="struct" title="struct object_store::azure::MicrosoftAzure">MicrosoftAzure</a>

Available on **crate feature `azure`** only.

<a href="https://docs.rs/object_store/latest/object_store/signer/trait.Signer.html#impl-Signer-for-GoogleCloudStorage" class="anchor">§</a>

### impl <a href="https://docs.rs/object_store/latest/object_store/signer/trait.Signer.html" class="trait" title="trait object_store::signer::Signer">Signer</a> for <a href="https://docs.rs/object_store/latest/object_store/gcp/struct.GoogleCloudStorage.html" class="struct" title="struct object_store::gcp::GoogleCloudStorage">GoogleCloudStorage</a>

Available on **crate feature `gcp`** only.
