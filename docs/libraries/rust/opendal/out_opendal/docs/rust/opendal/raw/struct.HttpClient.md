# Struct HttpClient Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/raw/http_util/client.rs.html#62-64" class="src">Source</a>

``` rust
pub struct HttpClient { /* private fields */ }
```

Expand description

A HTTP client instance for OpenDALâ€™s services.

## <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.HttpClient.html#notes" class="doc-anchor">Â§</a>Notes

- A http client must support redirections that follows 3xx response.

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.HttpClient.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.HttpClient.html#impl-HttpClient" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.HttpClient.html" class="struct" title="struct opendal::raw::HttpClient">HttpClient</a>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.HttpClient.html#method.new" class="fn">new</a>() -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<Self\>

Create a new http client in async context.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.HttpClient.html#method.with" class="fn">with</a>(client: impl <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.HttpFetch.html" class="trait" title="trait opendal::raw::HttpFetch">HttpFetch</a>) -\> Self

Construct `Self` with given \[`reqwest::Client`\]

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.HttpClient.html#method.build" class="fn">build</a>(builder: ClientBuilder) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<Self\>

ðŸ‘ŽDeprecated

Build a new http client in async context.

#### pub async fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.HttpClient.html#method.send" class="fn">send</a>(&self, req: Request\<<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html" class="struct" title="struct opendal::Buffer">Buffer</a>\>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<Response\<<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html" class="struct" title="struct opendal::Buffer">Buffer</a>\>\>

Send a request and consume response.

#### pub async fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.HttpClient.html#method.fetch" class="fn">fetch</a>(&self, req: Request\<<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html" class="struct" title="struct opendal::Buffer">Buffer</a>\>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<Response\<<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.HttpBody.html" class="struct" title="struct opendal::raw::HttpBody">HttpBody</a>\>\>

Fetch a request and return a streamable [`HttpBody`](https://opendal.apache.org/docs/rust/opendal/raw/struct.HttpBody.html "struct opendal::raw::HttpBody").

Services can use [`HttpBody`](https://opendal.apache.org/docs/rust/opendal/raw/struct.HttpBody.html "struct opendal::raw::HttpBody") as \[`Access::Read`\].

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.HttpClient.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.HttpClient.html#impl-Clone-for-HttpClient" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.HttpClient.html" class="struct" title="struct opendal::raw::HttpClient">HttpClient</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.HttpClient.html#method.clone" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.HttpClient.html" class="struct" title="struct opendal::raw::HttpClient">HttpClient</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.HttpClient.html#method.clone_from" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.HttpClient.html#impl-Debug-for-HttpClient" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.HttpClient.html" class="struct" title="struct opendal::raw::HttpClient">HttpClient</a>

We donâ€™t want users to know details about our clients.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.HttpClient.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.HttpClient.html#impl-Default-for-HttpClient" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.HttpClient.html" class="struct" title="struct opendal::raw::HttpClient">HttpClient</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.HttpClient.html#method.default" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> Self

Returns the â€œdefault valueâ€? for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.HttpClient.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.HttpClient.html#blanket-implementations" class="anchor">Â§</a>
