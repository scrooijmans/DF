# Struct HttpBody Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/raw/http_util/body.rs.html#31-38" class="src">Source</a>

``` rust
pub struct HttpBody { /* private fields */ }
```

Expand description

The streaming body that OpenDALâ€™s HttpClient returned.

We implement [`oio::Read`](https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Read.html "trait opendal::raw::oio::Read") for the `HttpBody`. Services can use `HttpBody` as \[`Access::Read`\].

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.HttpBody.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.HttpBody.html#impl-HttpBody" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.HttpBody.html" class="struct" title="struct opendal::raw::HttpBody">HttpBody</a>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.HttpBody.html#method.new" class="fn">new</a>\<S\>(stream: S, size: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>) -\> Self

where S: Stream\<Item = <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html" class="struct" title="struct opendal::Buffer">Buffer</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html" class="trait" title="trait core::marker::Unpin">Unpin</a> + 'static,

Available on **non-WebAssembly** only.

Create a new `HttpBody` with given stream and optional size.

#### pub async fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.HttpBody.html#method.to_buffer" class="fn">to_buffer</a>(&mut self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html" class="struct" title="struct opendal::Buffer">Buffer</a>\>

Read all data from the stream.

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.HttpBody.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.HttpBody.html#impl-Read-for-HttpBody" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Read.html" class="trait" title="trait opendal::raw::oio::Read">Read</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.HttpBody.html" class="struct" title="struct opendal::raw::HttpBody">HttpBody</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.HttpBody.html#method.read" class="anchor">Â§</a>

#### async fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Read.html#tymethod.read" class="fn">read</a>(&mut self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html" class="struct" title="struct opendal::Buffer">Buffer</a>\>

Read at the given offset with the given size.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.HttpBody.html#method.read_all" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Read.html#method.read_all" class="fn">read_all</a>(&mut self) -\> impl <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html" class="struct" title="struct opendal::Buffer">Buffer</a>\>\> + <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.MaybeSend.html" class="trait" title="trait opendal::raw::MaybeSend">MaybeSend</a>

Read all data from the reader.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.HttpBody.html#impl-Send-for-HttpBody" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.HttpBody.html" class="struct" title="struct opendal::raw::HttpBody">HttpBody</a>

#### <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.HttpBody.html#safety" class="doc-anchor">Â§</a>Safety

HttpBody is `Send` on non wasm32 targets.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.HttpBody.html#impl-Sync-for-HttpBody" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.HttpBody.html" class="struct" title="struct opendal::raw::HttpBody">HttpBody</a>

#### <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.HttpBody.html#safety-1" class="doc-anchor">Â§</a>Safety

HttpBody is sync on non wasm32 targets.

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.HttpBody.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.HttpBody.html#blanket-implementations" class="anchor">Â§</a>
