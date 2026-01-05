# Struct SpawnService Copy item path

<a href="https://docs.rs/object_store/latest/src/object_store/client/http/spawn.rs.html#49-52" class="src">Source</a>

``` rust
pub struct SpawnService<T: HttpService + Clone> { /* private fields */ }
```

Available on **crate feature `cloud`** only.

Expand description

Wraps a provided [`HttpService`](https://docs.rs/object_store/latest/object_store/client/trait.HttpService.html "trait object_store::client::HttpService") and runs it on a separate tokio runtime

See example on [`SpawnedReqwestConnector`](https://docs.rs/object_store/latest/object_store/client/struct.SpawnedReqwestConnector.html "struct object_store::client::SpawnedReqwestConnector")

## Implementations<a href="https://docs.rs/object_store/latest/object_store/client/struct.SpawnService.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/client/struct.SpawnService.html#impl-SpawnService%3CT%3E" class="anchor">§</a>

### impl\<T: <a href="https://docs.rs/object_store/latest/object_store/client/trait.HttpService.html" class="trait" title="trait object_store::client::HttpService">HttpService</a> + <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a>\> <a href="https://docs.rs/object_store/latest/object_store/client/struct.SpawnService.html" class="struct" title="struct object_store::client::SpawnService">SpawnService</a>\<T\>

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/client/struct.SpawnService.html#method.new" class="fn">new</a>(inner: T, runtime: <a href="https://docs.rs/tokio/1.47.1/x86_64-unknown-linux-gnu/tokio/runtime/handle/struct.Handle.html" class="struct" title="struct tokio::runtime::handle::Handle">Handle</a>) -\> Self

Creates a new [`SpawnService`](https://docs.rs/object_store/latest/object_store/client/struct.SpawnService.html "struct object_store::client::SpawnService") from the provided

## Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/client/struct.SpawnService.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/client/struct.SpawnService.html#impl-Debug-for-SpawnService%3CT%3E" class="anchor">§</a>

### impl\<T: <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> + <a href="https://docs.rs/object_store/latest/object_store/client/trait.HttpService.html" class="trait" title="trait object_store::client::HttpService">HttpService</a> + <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a>\> <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/object_store/latest/object_store/client/struct.SpawnService.html" class="struct" title="struct object_store::client::SpawnService">SpawnService</a>\<T\>

<a href="https://docs.rs/object_store/latest/object_store/client/struct.SpawnService.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/object_store/latest/object_store/client/struct.SpawnService.html#impl-HttpService-for-SpawnService%3CT%3E" class="anchor">§</a>

### impl\<T: <a href="https://docs.rs/object_store/latest/object_store/client/trait.HttpService.html" class="trait" title="trait object_store::client::HttpService">HttpService</a> + <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a>\> <a href="https://docs.rs/object_store/latest/object_store/client/trait.HttpService.html" class="trait" title="trait object_store::client::HttpService">HttpService</a> for <a href="https://docs.rs/object_store/latest/object_store/client/struct.SpawnService.html" class="struct" title="struct object_store::client::SpawnService">SpawnService</a>\<T\>

<a href="https://docs.rs/object_store/latest/object_store/client/struct.SpawnService.html#method.call" class="anchor">§</a>

#### fn <a href="https://docs.rs/object_store/latest/object_store/client/trait.HttpService.html#tymethod.call" class="fn">call</a>\<'life0, 'async_trait\>( &'life0 self, req: <a href="https://docs.rs/object_store/latest/object_store/client/type.HttpRequest.html" class="type" title="type object_store::client::HttpRequest">HttpRequest</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/object_store/latest/object_store/client/type.HttpResponse.html" class="type" title="type object_store::client::HttpResponse">HttpResponse</a>, <a href="https://docs.rs/object_store/latest/object_store/client/struct.HttpError.html" class="struct" title="struct object_store::client::HttpError">HttpError</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where Self: 'async_trait, 'life0: 'async_trait,

Perform [`HttpRequest`](https://docs.rs/object_store/latest/object_store/client/type.HttpRequest.html "type object_store::client::HttpRequest") returning [`HttpResponse`](https://docs.rs/object_store/latest/object_store/client/type.HttpResponse.html "type object_store::client::HttpResponse")

## Auto Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/client/struct.SpawnService.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/object_store/latest/object_store/client/struct.SpawnService.html#blanket-implementations" class="anchor">§</a>
