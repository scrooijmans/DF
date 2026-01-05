# Struct HttpRequestBody Copy item path

<a href="https://docs.rs/object_store/latest/src/object_store/client/http/body.rs.html#34" class="src">Source</a>

``` rust
pub struct HttpRequestBody(/* private fields */);
```

Available on **crate feature `cloud`** only.

Expand description

The [`Body`](https://docs.rs/http-body/1.0.1/x86_64-unknown-linux-gnu/http_body/trait.Body.html "trait http_body::Body") of an [`HttpRequest`](https://docs.rs/object_store/latest/object_store/client/type.HttpRequest.html "type object_store::client::HttpRequest")

## Implementations<a href="https://docs.rs/object_store/latest/object_store/client/struct.HttpRequestBody.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/client/struct.HttpRequestBody.html#impl-HttpRequestBody" class="anchor">§</a>

### impl <a href="https://docs.rs/object_store/latest/object_store/client/struct.HttpRequestBody.html" class="struct" title="struct object_store::client::HttpRequestBody">HttpRequestBody</a>

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/client/struct.HttpRequestBody.html#method.empty" class="fn">empty</a>() -\> Self

An empty [`HttpRequestBody`](https://docs.rs/object_store/latest/object_store/client/struct.HttpRequestBody.html "struct object_store::client::HttpRequestBody")

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/client/struct.HttpRequestBody.html#method.is_empty" class="fn">is_empty</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if this body is empty

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/client/struct.HttpRequestBody.html#method.content_length" class="fn">content_length</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the total length of the [`Bytes`](https://docs.rs/bytes/1.10.1/x86_64-unknown-linux-gnu/bytes/bytes/struct.Bytes.html "struct bytes::bytes::Bytes") in this body

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/client/struct.HttpRequestBody.html#method.as_bytes" class="fn">as_bytes</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/bytes/1.10.1/x86_64-unknown-linux-gnu/bytes/bytes/struct.Bytes.html" class="struct" title="struct bytes::bytes::Bytes">Bytes</a>\>

If this body consists of a single contiguous [`Bytes`](https://docs.rs/bytes/1.10.1/x86_64-unknown-linux-gnu/bytes/bytes/struct.Bytes.html "struct bytes::bytes::Bytes"), returns it

## Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/client/struct.HttpRequestBody.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/client/struct.HttpRequestBody.html#impl-Body-for-HttpRequestBody" class="anchor">§</a>

### impl <a href="https://docs.rs/http-body/1.0.1/x86_64-unknown-linux-gnu/http_body/trait.Body.html" class="trait" title="trait http_body::Body">Body</a> for <a href="https://docs.rs/object_store/latest/object_store/client/struct.HttpRequestBody.html" class="struct" title="struct object_store::client::HttpRequestBody">HttpRequestBody</a>

<a href="https://docs.rs/object_store/latest/object_store/client/struct.HttpRequestBody.html#associatedtype.Data" class="anchor">§</a>

#### type <a href="https://docs.rs/http-body/1.0.1/x86_64-unknown-linux-gnu/http_body/trait.Body.html#associatedtype.Data" class="associatedtype">Data</a> = <a href="https://docs.rs/bytes/1.10.1/x86_64-unknown-linux-gnu/bytes/bytes/struct.Bytes.html" class="struct" title="struct bytes::bytes::Bytes">Bytes</a>

Values yielded by the `Body`.

<a href="https://docs.rs/object_store/latest/object_store/client/struct.HttpRequestBody.html#associatedtype.Error" class="anchor">§</a>

#### type <a href="https://docs.rs/http-body/1.0.1/x86_64-unknown-linux-gnu/http_body/trait.Body.html#associatedtype.Error" class="associatedtype">Error</a> = <a href="https://docs.rs/object_store/latest/object_store/client/struct.HttpError.html" class="struct" title="struct object_store::client::HttpError">HttpError</a>

The error type this `Body` might generate.

<a href="https://docs.rs/object_store/latest/object_store/client/struct.HttpRequestBody.html#method.poll_frame" class="anchor">§</a>

#### fn <a href="https://docs.rs/http-body/1.0.1/x86_64-unknown-linux-gnu/http_body/trait.Body.html#tymethod.poll_frame" class="fn">poll_frame</a>( self: <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<&mut Self\>, \_cx: &mut <a href="https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html" class="struct" title="struct core::task::wake::Context">Context</a>\<'\_\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html" class="enum" title="enum core::task::poll::Poll">Poll</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/http-body/1.0.1/x86_64-unknown-linux-gnu/http_body/frame/struct.Frame.html" class="struct" title="struct http_body::frame::Frame">Frame</a>\<Self::<a href="https://docs.rs/http-body/1.0.1/x86_64-unknown-linux-gnu/http_body/trait.Body.html#associatedtype.Data" class="associatedtype" title="type http_body::Body::Data">Data</a>\>, Self::<a href="https://docs.rs/http-body/1.0.1/x86_64-unknown-linux-gnu/http_body/trait.Body.html#associatedtype.Error" class="associatedtype" title="type http_body::Body::Error">Error</a>\>\>\>

Attempt to pull out the next data buffer of this stream.

<a href="https://docs.rs/object_store/latest/object_store/client/struct.HttpRequestBody.html#method.is_end_stream" class="anchor">§</a>

#### fn <a href="https://docs.rs/http-body/1.0.1/x86_64-unknown-linux-gnu/http_body/trait.Body.html#method.is_end_stream" class="fn">is_end_stream</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns `true` when the end of stream has been reached. [Read more](https://docs.rs/http-body/1.0.1/x86_64-unknown-linux-gnu/http_body/trait.Body.html#method.is_end_stream)

<a href="https://docs.rs/object_store/latest/object_store/client/struct.HttpRequestBody.html#method.size_hint" class="anchor">§</a>

#### fn <a href="https://docs.rs/http-body/1.0.1/x86_64-unknown-linux-gnu/http_body/trait.Body.html#method.size_hint" class="fn">size_hint</a>(&self) -\> <a href="https://docs.rs/http-body/1.0.1/x86_64-unknown-linux-gnu/http_body/size_hint/struct.SizeHint.html" class="struct" title="struct http_body::size_hint::SizeHint">SizeHint</a>

Returns the bounds on the remaining length of the stream. [Read more](https://docs.rs/http-body/1.0.1/x86_64-unknown-linux-gnu/http_body/trait.Body.html#method.size_hint)

<a href="https://docs.rs/object_store/latest/object_store/client/struct.HttpRequestBody.html#impl-Clone-for-HttpRequestBody" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/object_store/latest/object_store/client/struct.HttpRequestBody.html" class="struct" title="struct object_store::client::HttpRequestBody">HttpRequestBody</a>

<a href="https://docs.rs/object_store/latest/object_store/client/struct.HttpRequestBody.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/object_store/latest/object_store/client/struct.HttpRequestBody.html" class="struct" title="struct object_store::client::HttpRequestBody">HttpRequestBody</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/client/struct.HttpRequestBody.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/object_store/latest/object_store/client/struct.HttpRequestBody.html#impl-Debug-for-HttpRequestBody" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/object_store/latest/object_store/client/struct.HttpRequestBody.html" class="struct" title="struct object_store::client::HttpRequestBody">HttpRequestBody</a>

<a href="https://docs.rs/object_store/latest/object_store/client/struct.HttpRequestBody.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/object_store/latest/object_store/client/struct.HttpRequestBody.html#impl-From%3CBytes%3E-for-HttpRequestBody" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/bytes/1.10.1/x86_64-unknown-linux-gnu/bytes/bytes/struct.Bytes.html" class="struct" title="struct bytes::bytes::Bytes">Bytes</a>\> for <a href="https://docs.rs/object_store/latest/object_store/client/struct.HttpRequestBody.html" class="struct" title="struct object_store::client::HttpRequestBody">HttpRequestBody</a>

<a href="https://docs.rs/object_store/latest/object_store/client/struct.HttpRequestBody.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://docs.rs/bytes/1.10.1/x86_64-unknown-linux-gnu/bytes/bytes/struct.Bytes.html" class="struct" title="struct bytes::bytes::Bytes">Bytes</a>) -\> Self

Converts to this type from the input type.

<a href="https://docs.rs/object_store/latest/object_store/client/struct.HttpRequestBody.html#impl-From%3CPutPayload%3E-for-HttpRequestBody" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/object_store/latest/object_store/struct.PutPayload.html" class="struct" title="struct object_store::PutPayload">PutPayload</a>\> for <a href="https://docs.rs/object_store/latest/object_store/client/struct.HttpRequestBody.html" class="struct" title="struct object_store::client::HttpRequestBody">HttpRequestBody</a>

<a href="https://docs.rs/object_store/latest/object_store/client/struct.HttpRequestBody.html#method.from-3" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://docs.rs/object_store/latest/object_store/struct.PutPayload.html" class="struct" title="struct object_store::PutPayload">PutPayload</a>) -\> Self

Converts to this type from the input type.

<a href="https://docs.rs/object_store/latest/object_store/client/struct.HttpRequestBody.html#impl-From%3CString%3E-for-HttpRequestBody" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\> for <a href="https://docs.rs/object_store/latest/object_store/client/struct.HttpRequestBody.html" class="struct" title="struct object_store::client::HttpRequestBody">HttpRequestBody</a>

<a href="https://docs.rs/object_store/latest/object_store/client/struct.HttpRequestBody.html#method.from-2" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>) -\> Self

Converts to this type from the input type.

<a href="https://docs.rs/object_store/latest/object_store/client/struct.HttpRequestBody.html#impl-From%3CVec%3Cu8%3E%3E-for-HttpRequestBody" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>\> for <a href="https://docs.rs/object_store/latest/object_store/client/struct.HttpRequestBody.html" class="struct" title="struct object_store::client::HttpRequestBody">HttpRequestBody</a>

<a href="https://docs.rs/object_store/latest/object_store/client/struct.HttpRequestBody.html#method.from-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>) -\> Self

Converts to this type from the input type.

## Auto Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/client/struct.HttpRequestBody.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/object_store/latest/object_store/client/struct.HttpRequestBody.html#blanket-implementations" class="anchor">§</a>
