# Struct BufferStream Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/types/read/buffer_stream.rs.html#132-141" class="src">Source</a>

``` rust
pub struct BufferStream { /* private fields */ }
```

Expand description

BufferStream is a stream of buffers, created by [`Reader::into_stream`](https://opendal.apache.org/docs/rust/opendal/struct.Reader.html#method.into_stream "method opendal::Reader::into_stream")

`BufferStream` implements `Stream` trait.

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/struct.BufferStream.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/struct.BufferStream.html#impl-Stream-for-BufferStream" class="anchor">Â§</a>

### impl Stream for <a href="https://opendal.apache.org/docs/rust/opendal/struct.BufferStream.html" class="struct" title="struct opendal::BufferStream">BufferStream</a>

<a href="https://opendal.apache.org/docs/rust/opendal/struct.BufferStream.html#associatedtype.Item" class="anchor">Â§</a>

#### type Item = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html" class="struct" title="struct opendal::Buffer">Buffer</a>, <a href="https://opendal.apache.org/docs/rust/opendal/struct.Error.html" class="struct" title="struct opendal::Error">Error</a>\>

Values yielded by the stream.

<a href="https://opendal.apache.org/docs/rust/opendal/struct.BufferStream.html#method.poll_next" class="anchor">Â§</a>

#### fn poll_next( self: <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<&mut Self\>, cx: &mut <a href="https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html" class="struct" title="struct core::task::wake::Context">Context</a>\<'\_\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html" class="enum" title="enum core::task::poll::Poll">Poll</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<Self::Item\>\>

Attempt to pull out the next value of this stream, registering the current task for wakeup if the value is not yet available, and returning `None` if the stream is exhausted. Read more

<a href="https://opendal.apache.org/docs/rust/opendal/struct.BufferStream.html#method.size_hint" class="anchor">Â§</a>

#### fn size_hint(&self) -\> (<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>)

Returns the bounds on the remaining length of the stream. Read more

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/struct.BufferStream.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/struct.BufferStream.html#blanket-implementations" class="anchor">Â§</a>
