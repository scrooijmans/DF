# Trait HttpFetch Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/raw/http_util/client.rs.html#125-131" class="src">Source</a>

``` rust
pub trait HttpFetch:
    Send
    + Sync
    + Unpin
    + 'static {
    // Required method
    fn fetch(
        &self,
        req: Request<Buffer>,
    ) -> impl Future<Output = Result<Response<HttpBody>>> + MaybeSend;
}
```

Expand description

HttpFetch is the trait to fetch a request in async way. User should implement this trait to provide their own http client.

## Required Methods<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.HttpFetch.html#required-methods" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.HttpFetch.html#tymethod.fetch" class="fn">fetch</a>( &self, req: Request\<<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html" class="struct" title="struct opendal::Buffer">Buffer</a>\>, ) -\> impl <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<Response\<<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.HttpBody.html" class="struct" title="struct opendal::raw::HttpBody">HttpBody</a>\>\>\> + <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.MaybeSend.html" class="trait" title="trait opendal::raw::MaybeSend">MaybeSend</a>

Fetch a request in async way.

## Dyn Compatibility<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.HttpFetch.html#dyn-compatibility" class="anchor">Â§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementations on Foreign Types<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.HttpFetch.html#foreign-impls" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.HttpFetch.html#impl-HttpFetch-for-Client" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.HttpFetch.html" class="trait" title="trait opendal::raw::HttpFetch">HttpFetch</a> for Client

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.HttpFetch.html#method.fetch" class="anchor">Â§</a>

#### async fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.HttpFetch.html#tymethod.fetch" class="fn">fetch</a>(&self, req: Request\<<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html" class="struct" title="struct opendal::Buffer">Buffer</a>\>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<Response\<<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.HttpBody.html" class="struct" title="struct opendal::raw::HttpBody">HttpBody</a>\>\>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.HttpFetch.html#impl-HttpFetch-for-Arc%3CT%3E" class="anchor">Â§</a>

### impl\<T: HttpFetchDyn + ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.HttpFetch.html" class="trait" title="trait opendal::raw::HttpFetch">HttpFetch</a> for <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<T\>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.HttpFetch.html#method.fetch-1" class="anchor">Â§</a>

#### async fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.HttpFetch.html#tymethod.fetch" class="fn">fetch</a>(&self, req: Request\<<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html" class="struct" title="struct opendal::Buffer">Buffer</a>\>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<Response\<<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.HttpBody.html" class="struct" title="struct opendal::raw::HttpBody">HttpBody</a>\>\>

## Implementors<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.HttpFetch.html#implementors" class="anchor">Â§</a>
