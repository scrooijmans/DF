# Struct HttpResponseBody Copy item path

<a href="https://docs.rs/object_store/latest/src/object_store/client/http/body.rs.html#168" class="src">Source</a>

``` rust
pub struct HttpResponseBody(/* private fields */);
```

Available on **crate feature `cloud`** only.

Expand description

The body of an [`HttpResponse`](https://docs.rs/object_store/latest/object_store/client/type.HttpResponse.html "type object_store::client::HttpResponse")

## Implementations<a href="https://docs.rs/object_store/latest/object_store/client/struct.HttpResponseBody.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/client/struct.HttpResponseBody.html#impl-HttpResponseBody" class="anchor">§</a>

### impl <a href="https://docs.rs/object_store/latest/object_store/client/struct.HttpResponseBody.html" class="struct" title="struct object_store::client::HttpResponseBody">HttpResponseBody</a>

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/client/struct.HttpResponseBody.html#method.new" class="fn">new</a>\<B\>(body: B) -\> Self

where B: <a href="https://docs.rs/http-body/1.0.1/x86_64-unknown-linux-gnu/http_body/trait.Body.html" class="trait" title="trait http_body::Body">Body</a>\<Data = <a href="https://docs.rs/bytes/1.10.1/x86_64-unknown-linux-gnu/bytes/bytes/struct.Bytes.html" class="struct" title="struct bytes::bytes::Bytes">Bytes</a>, Error = <a href="https://docs.rs/object_store/latest/object_store/client/struct.HttpError.html" class="struct" title="struct object_store::client::HttpError">HttpError</a>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + 'static,

Create an [`HttpResponseBody`](https://docs.rs/object_store/latest/object_store/client/struct.HttpResponseBody.html "struct object_store::client::HttpResponseBody") from the provided [`Body`](https://docs.rs/http-body/1.0.1/x86_64-unknown-linux-gnu/http_body/trait.Body.html "trait http_body::Body")

Note: [`BodyExt::map_err`](https://docs.rs/http-body-util/0.1.3/x86_64-unknown-linux-gnu/http_body_util/trait.BodyExt.html#method.map_err "method http_body_util::BodyExt::map_err") can be used to alter error variants

#### pub async fn <a href="https://docs.rs/object_store/latest/object_store/client/struct.HttpResponseBody.html#method.bytes" class="fn">bytes</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/bytes/1.10.1/x86_64-unknown-linux-gnu/bytes/bytes/struct.Bytes.html" class="struct" title="struct bytes::bytes::Bytes">Bytes</a>, <a href="https://docs.rs/object_store/latest/object_store/client/struct.HttpError.html" class="struct" title="struct object_store::client::HttpError">HttpError</a>\>

Collects this response into a [`Bytes`](https://docs.rs/bytes/1.10.1/x86_64-unknown-linux-gnu/bytes/bytes/struct.Bytes.html "struct bytes::bytes::Bytes")

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/client/struct.HttpResponseBody.html#method.bytes_stream" class="fn">bytes_stream</a>(self) -\> <a href="https://docs.rs/futures-core/0.3.31/x86_64-unknown-linux-gnu/futures_core/stream/type.BoxStream.html" class="type" title="type futures_core::stream::BoxStream">BoxStream</a>\<'static, <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/bytes/1.10.1/x86_64-unknown-linux-gnu/bytes/bytes/struct.Bytes.html" class="struct" title="struct bytes::bytes::Bytes">Bytes</a>, <a href="https://docs.rs/object_store/latest/object_store/client/struct.HttpError.html" class="struct" title="struct object_store::client::HttpError">HttpError</a>\>\>

Returns a stream of this response data

## Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/client/struct.HttpResponseBody.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/client/struct.HttpResponseBody.html#impl-Body-for-HttpResponseBody" class="anchor">§</a>

### impl <a href="https://docs.rs/http-body/1.0.1/x86_64-unknown-linux-gnu/http_body/trait.Body.html" class="trait" title="trait http_body::Body">Body</a> for <a href="https://docs.rs/object_store/latest/object_store/client/struct.HttpResponseBody.html" class="struct" title="struct object_store::client::HttpResponseBody">HttpResponseBody</a>

<a href="https://docs.rs/object_store/latest/object_store/client/struct.HttpResponseBody.html#associatedtype.Data" class="anchor">§</a>

#### type <a href="https://docs.rs/http-body/1.0.1/x86_64-unknown-linux-gnu/http_body/trait.Body.html#associatedtype.Data" class="associatedtype">Data</a> = <a href="https://docs.rs/bytes/1.10.1/x86_64-unknown-linux-gnu/bytes/bytes/struct.Bytes.html" class="struct" title="struct bytes::bytes::Bytes">Bytes</a>

Values yielded by the `Body`.

<a href="https://docs.rs/object_store/latest/object_store/client/struct.HttpResponseBody.html#associatedtype.Error" class="anchor">§</a>

#### type <a href="https://docs.rs/http-body/1.0.1/x86_64-unknown-linux-gnu/http_body/trait.Body.html#associatedtype.Error" class="associatedtype">Error</a> = <a href="https://docs.rs/object_store/latest/object_store/client/struct.HttpError.html" class="struct" title="struct object_store::client::HttpError">HttpError</a>

The error type this `Body` might generate.

<a href="https://docs.rs/object_store/latest/object_store/client/struct.HttpResponseBody.html#method.poll_frame" class="anchor">§</a>

#### fn <a href="https://docs.rs/http-body/1.0.1/x86_64-unknown-linux-gnu/http_body/trait.Body.html#tymethod.poll_frame" class="fn">poll_frame</a>( self: <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<&mut Self\>, cx: &mut <a href="https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html" class="struct" title="struct core::task::wake::Context">Context</a>\<'\_\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html" class="enum" title="enum core::task::poll::Poll">Poll</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/http-body/1.0.1/x86_64-unknown-linux-gnu/http_body/frame/struct.Frame.html" class="struct" title="struct http_body::frame::Frame">Frame</a>\<Self::<a href="https://docs.rs/http-body/1.0.1/x86_64-unknown-linux-gnu/http_body/trait.Body.html#associatedtype.Data" class="associatedtype" title="type http_body::Body::Data">Data</a>\>, Self::<a href="https://docs.rs/http-body/1.0.1/x86_64-unknown-linux-gnu/http_body/trait.Body.html#associatedtype.Error" class="associatedtype" title="type http_body::Body::Error">Error</a>\>\>\>

Attempt to pull out the next data buffer of this stream.

<a href="https://docs.rs/object_store/latest/object_store/client/struct.HttpResponseBody.html#method.is_end_stream" class="anchor">§</a>

#### fn <a href="https://docs.rs/http-body/1.0.1/x86_64-unknown-linux-gnu/http_body/trait.Body.html#method.is_end_stream" class="fn">is_end_stream</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns `true` when the end of stream has been reached. [Read more](https://docs.rs/http-body/1.0.1/x86_64-unknown-linux-gnu/http_body/trait.Body.html#method.is_end_stream)

<a href="https://docs.rs/object_store/latest/object_store/client/struct.HttpResponseBody.html#method.size_hint" class="anchor">§</a>

#### fn <a href="https://docs.rs/http-body/1.0.1/x86_64-unknown-linux-gnu/http_body/trait.Body.html#method.size_hint" class="fn">size_hint</a>(&self) -\> <a href="https://docs.rs/http-body/1.0.1/x86_64-unknown-linux-gnu/http_body/size_hint/struct.SizeHint.html" class="struct" title="struct http_body::size_hint::SizeHint">SizeHint</a>

Returns the bounds on the remaining length of the stream. [Read more](https://docs.rs/http-body/1.0.1/x86_64-unknown-linux-gnu/http_body/trait.Body.html#method.size_hint)

<a href="https://docs.rs/object_store/latest/object_store/client/struct.HttpResponseBody.html#impl-Debug-for-HttpResponseBody" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/object_store/latest/object_store/client/struct.HttpResponseBody.html" class="struct" title="struct object_store::client::HttpResponseBody">HttpResponseBody</a>

<a href="https://docs.rs/object_store/latest/object_store/client/struct.HttpResponseBody.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/object_store/latest/object_store/client/struct.HttpResponseBody.html#impl-From%3CBytes%3E-for-HttpResponseBody" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/bytes/1.10.1/x86_64-unknown-linux-gnu/bytes/bytes/struct.Bytes.html" class="struct" title="struct bytes::bytes::Bytes">Bytes</a>\> for <a href="https://docs.rs/object_store/latest/object_store/client/struct.HttpResponseBody.html" class="struct" title="struct object_store::client::HttpResponseBody">HttpResponseBody</a>

<a href="https://docs.rs/object_store/latest/object_store/client/struct.HttpResponseBody.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://docs.rs/bytes/1.10.1/x86_64-unknown-linux-gnu/bytes/bytes/struct.Bytes.html" class="struct" title="struct bytes::bytes::Bytes">Bytes</a>) -\> Self

Converts to this type from the input type.

<a href="https://docs.rs/object_store/latest/object_store/client/struct.HttpResponseBody.html#impl-From%3CString%3E-for-HttpResponseBody" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\> for <a href="https://docs.rs/object_store/latest/object_store/client/struct.HttpResponseBody.html" class="struct" title="struct object_store::client::HttpResponseBody">HttpResponseBody</a>

<a href="https://docs.rs/object_store/latest/object_store/client/struct.HttpResponseBody.html#method.from-2" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>) -\> Self

Converts to this type from the input type.

<a href="https://docs.rs/object_store/latest/object_store/client/struct.HttpResponseBody.html#impl-From%3CVec%3Cu8%3E%3E-for-HttpResponseBody" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>\> for <a href="https://docs.rs/object_store/latest/object_store/client/struct.HttpResponseBody.html" class="struct" title="struct object_store::client::HttpResponseBody">HttpResponseBody</a>

<a href="https://docs.rs/object_store/latest/object_store/client/struct.HttpResponseBody.html#method.from-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>) -\> Self

Converts to this type from the input type.

## Auto Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/client/struct.HttpResponseBody.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/object_store/latest/object_store/client/struct.HttpResponseBody.html#blanket-implementations" class="anchor">§</a>
