# Struct HttpClient Copy item path

<a href="https://docs.rs/object_store/latest/src/object_store/client/http/connection.rs.html#150" class="src">Source</a>

``` rust
pub struct HttpClient(/* private fields */);
```

Available on **crate feature `cloud`** only.

Expand description

An HTTP client

## Implementations<a href="https://docs.rs/object_store/latest/object_store/client/struct.HttpClient.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/client/struct.HttpClient.html#impl-HttpClient" class="anchor">§</a>

### impl <a href="https://docs.rs/object_store/latest/object_store/client/struct.HttpClient.html" class="struct" title="struct object_store::client::HttpClient">HttpClient</a>

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/client/struct.HttpClient.html#method.new" class="fn">new</a>(service: impl <a href="https://docs.rs/object_store/latest/object_store/client/trait.HttpService.html" class="trait" title="trait object_store::client::HttpService">HttpService</a> + 'static) -\> Self

Create a new [`HttpClient`](https://docs.rs/object_store/latest/object_store/client/struct.HttpClient.html "struct object_store::client::HttpClient") from an [`HttpService`](https://docs.rs/object_store/latest/object_store/client/trait.HttpService.html "trait object_store::client::HttpService")

#### pub async fn <a href="https://docs.rs/object_store/latest/object_store/client/struct.HttpClient.html#method.execute" class="fn">execute</a>( &self, request: <a href="https://docs.rs/object_store/latest/object_store/client/type.HttpRequest.html" class="type" title="type object_store::client::HttpRequest">HttpRequest</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/object_store/latest/object_store/client/type.HttpResponse.html" class="type" title="type object_store::client::HttpResponse">HttpResponse</a>, <a href="https://docs.rs/object_store/latest/object_store/client/struct.HttpError.html" class="struct" title="struct object_store::client::HttpError">HttpError</a>\>

Performs [`HttpRequest`](https://docs.rs/object_store/latest/object_store/client/type.HttpRequest.html "type object_store::client::HttpRequest") using this client

## Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/client/struct.HttpClient.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/client/struct.HttpClient.html#impl-Clone-for-HttpClient" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/object_store/latest/object_store/client/struct.HttpClient.html" class="struct" title="struct object_store::client::HttpClient">HttpClient</a>

<a href="https://docs.rs/object_store/latest/object_store/client/struct.HttpClient.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/object_store/latest/object_store/client/struct.HttpClient.html" class="struct" title="struct object_store::client::HttpClient">HttpClient</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/client/struct.HttpClient.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/object_store/latest/object_store/client/struct.HttpClient.html#impl-Debug-for-HttpClient" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/object_store/latest/object_store/client/struct.HttpClient.html" class="struct" title="struct object_store::client::HttpClient">HttpClient</a>

<a href="https://docs.rs/object_store/latest/object_store/client/struct.HttpClient.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/client/struct.HttpClient.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/object_store/latest/object_store/client/struct.HttpClient.html#blanket-implementations" class="anchor">§</a>
