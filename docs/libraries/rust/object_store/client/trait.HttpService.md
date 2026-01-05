# Trait HttpService Copy item path

<a href="https://docs.rs/object_store/latest/src/object_store/client/http/connection.rs.html#143-146" class="src">Source</a>

``` rust
pub trait HttpService:
    Debug
    + Send
    + Sync
    + 'static {
    // Required method
    fn call<'life0, 'async_trait>(
        &'life0 self,
        req: HttpRequest,
    ) -> Pin<Box<dyn Future<Output = Result<HttpResponse, HttpError>> + Send + 'async_trait>>
       where Self: 'async_trait,
             'life0: 'async_trait;
}
```

Available on **crate feature `cloud`** only.

Expand description

An asynchronous function from a [`HttpRequest`](https://docs.rs/object_store/latest/object_store/client/type.HttpRequest.html "type object_store::client::HttpRequest") to a [`HttpResponse`](https://docs.rs/object_store/latest/object_store/client/type.HttpResponse.html "type object_store::client::HttpResponse").

## Required Methods<a href="https://docs.rs/object_store/latest/object_store/client/trait.HttpService.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/object_store/latest/object_store/client/trait.HttpService.html#tymethod.call" class="fn">call</a>\<'life0, 'async_trait\>( &'life0 self, req: <a href="https://docs.rs/object_store/latest/object_store/client/type.HttpRequest.html" class="type" title="type object_store::client::HttpRequest">HttpRequest</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/object_store/latest/object_store/client/type.HttpResponse.html" class="type" title="type object_store::client::HttpResponse">HttpResponse</a>, <a href="https://docs.rs/object_store/latest/object_store/client/struct.HttpError.html" class="struct" title="struct object_store::client::HttpError">HttpError</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where Self: 'async_trait, 'life0: 'async_trait,

Perform [`HttpRequest`](https://docs.rs/object_store/latest/object_store/client/type.HttpRequest.html "type object_store::client::HttpRequest") returning [`HttpResponse`](https://docs.rs/object_store/latest/object_store/client/type.HttpResponse.html "type object_store::client::HttpResponse")

## Implementations on Foreign Types<a href="https://docs.rs/object_store/latest/object_store/client/trait.HttpService.html#foreign-impls" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/client/trait.HttpService.html#impl-HttpService-for-Client" class="anchor">§</a>

### impl <a href="https://docs.rs/object_store/latest/object_store/client/trait.HttpService.html" class="trait" title="trait object_store::client::HttpService">HttpService</a> for <a href="https://docs.rs/reqwest/0.12.23/x86_64-unknown-linux-gnu/reqwest/async_impl/client/struct.Client.html" class="struct" title="struct reqwest::async_impl::client::Client">Client</a>

<a href="https://docs.rs/object_store/latest/object_store/client/trait.HttpService.html#method.call" class="anchor">§</a>

#### fn <a href="https://docs.rs/object_store/latest/object_store/client/trait.HttpService.html#tymethod.call" class="fn">call</a>\<'life0, 'async_trait\>( &'life0 self, req: <a href="https://docs.rs/object_store/latest/object_store/client/type.HttpRequest.html" class="type" title="type object_store::client::HttpRequest">HttpRequest</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/object_store/latest/object_store/client/type.HttpResponse.html" class="type" title="type object_store::client::HttpResponse">HttpResponse</a>, <a href="https://docs.rs/object_store/latest/object_store/client/struct.HttpError.html" class="struct" title="struct object_store::client::HttpError">HttpError</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where Self: 'async_trait, 'life0: 'async_trait,

## Implementors<a href="https://docs.rs/object_store/latest/object_store/client/trait.HttpService.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/client/trait.HttpService.html#impl-HttpService-for-SpawnService%3CT%3E" class="anchor">§</a>

### impl\<T: <a href="https://docs.rs/object_store/latest/object_store/client/trait.HttpService.html" class="trait" title="trait object_store::client::HttpService">HttpService</a> + <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a>\> <a href="https://docs.rs/object_store/latest/object_store/client/trait.HttpService.html" class="trait" title="trait object_store::client::HttpService">HttpService</a> for <a href="https://docs.rs/object_store/latest/object_store/client/struct.SpawnService.html" class="struct" title="struct object_store::client::SpawnService">SpawnService</a>\<T\>
