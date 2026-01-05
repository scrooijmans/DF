# Struct WriteMultipart Copy item path

<a href="https://docs.rs/object_store/latest/src/object_store/upload.rs.html#120-128" class="src">Source</a>

``` rust
pub struct WriteMultipart { /* private fields */ }
```

Expand description

A synchronous write API for uploading data in parallel in fixed size chunks

Uses multiple tokio tasks in a [`JoinSet`](https://docs.rs/tokio/1.47.1/x86_64-unknown-linux-gnu/tokio/task/join_set/struct.JoinSet.html "struct tokio::task::join_set::JoinSet") to multiplex upload tasks in parallel

The design also takes inspiration from [`Sink`](https://docs.rs/futures-sink/0.3.31/x86_64-unknown-linux-gnu/futures_sink/trait.Sink.html "trait futures_sink::Sink") with [`WriteMultipart::wait_for_capacity`](https://docs.rs/object_store/latest/object_store/struct.WriteMultipart.html#method.wait_for_capacity "method object_store::WriteMultipart::wait_for_capacity") allowing back pressure on producers, prior to buffering the next part. However, unlike [`Sink`](https://docs.rs/futures-sink/0.3.31/x86_64-unknown-linux-gnu/futures_sink/trait.Sink.html "trait futures_sink::Sink") this back pressure is optional, allowing integration with synchronous producers

## Implementations<a href="https://docs.rs/object_store/latest/object_store/struct.WriteMultipart.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/struct.WriteMultipart.html#impl-WriteMultipart" class="anchor">§</a>

### impl <a href="https://docs.rs/object_store/latest/object_store/struct.WriteMultipart.html" class="struct" title="struct object_store::WriteMultipart">WriteMultipart</a>

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/struct.WriteMultipart.html#method.new" class="fn">new</a>(upload: <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/object_store/latest/object_store/trait.MultipartUpload.html" class="trait" title="trait object_store::MultipartUpload">MultipartUpload</a>\>) -\> Self

Create a new [`WriteMultipart`](https://docs.rs/object_store/latest/object_store/struct.WriteMultipart.html "struct object_store::WriteMultipart") that will upload using 5MB chunks

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/struct.WriteMultipart.html#method.new_with_chunk_size" class="fn">new_with_chunk_size</a>( upload: <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/object_store/latest/object_store/trait.MultipartUpload.html" class="trait" title="trait object_store::MultipartUpload">MultipartUpload</a>\>, chunk_size: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> Self

Create a new [`WriteMultipart`](https://docs.rs/object_store/latest/object_store/struct.WriteMultipart.html "struct object_store::WriteMultipart") that will upload in fixed `chunk_size` sized chunks

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/struct.WriteMultipart.html#method.poll_for_capacity" class="fn">poll_for_capacity</a>( &mut self, cx: &mut <a href="https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html" class="struct" title="struct core::task::wake::Context">Context</a>\<'\_\>, max_concurrency: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html" class="enum" title="enum core::task::poll::Poll">Poll</a>\<<a href="https://docs.rs/object_store/latest/object_store/type.Result.html" class="type" title="type object_store::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>\>

Polls for there to be less than `max_concurrency` [`UploadPart`](https://docs.rs/object_store/latest/object_store/type.UploadPart.html "type object_store::UploadPart") in progress

See [`Self::wait_for_capacity`](https://docs.rs/object_store/latest/object_store/struct.WriteMultipart.html#method.wait_for_capacity "method object_store::WriteMultipart::wait_for_capacity") for an async version of this function

#### pub async fn <a href="https://docs.rs/object_store/latest/object_store/struct.WriteMultipart.html#method.wait_for_capacity" class="fn">wait_for_capacity</a>(&mut self, max_concurrency: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/object_store/latest/object_store/type.Result.html" class="type" title="type object_store::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

Wait until there are less than `max_concurrency` [`UploadPart`](https://docs.rs/object_store/latest/object_store/type.UploadPart.html "type object_store::UploadPart") in progress

See [`Self::poll_for_capacity`](https://docs.rs/object_store/latest/object_store/struct.WriteMultipart.html#method.poll_for_capacity "method object_store::WriteMultipart::poll_for_capacity") for a [`Poll`](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll") version of this function

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/struct.WriteMultipart.html#method.write" class="fn">write</a>(&mut self, buf: &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\])

Write data to this [`WriteMultipart`](https://docs.rs/object_store/latest/object_store/struct.WriteMultipart.html "struct object_store::WriteMultipart")

Data is buffered using [`PutPayloadMut::extend_from_slice`](https://docs.rs/object_store/latest/object_store/struct.PutPayloadMut.html#method.extend_from_slice "method object_store::PutPayloadMut::extend_from_slice"). Implementations looking to write data from owned buffers may prefer [`Self::put`](https://docs.rs/object_store/latest/object_store/struct.WriteMultipart.html#method.put "method object_store::WriteMultipart::put") as this avoids copying.

Note this method is synchronous (not `async`) and will immediately start new uploads as soon as the internal `chunk_size` is hit, regardless of how many outstanding uploads are already in progress.

Back pressure can optionally be applied to producers by calling [`Self::wait_for_capacity`](https://docs.rs/object_store/latest/object_store/struct.WriteMultipart.html#method.wait_for_capacity "method object_store::WriteMultipart::wait_for_capacity") prior to calling this method

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/struct.WriteMultipart.html#method.put" class="fn">put</a>(&mut self, bytes: <a href="https://docs.rs/bytes/1.10.1/x86_64-unknown-linux-gnu/bytes/bytes/struct.Bytes.html" class="struct" title="struct bytes::bytes::Bytes">Bytes</a>)

Put a chunk of data into this [`WriteMultipart`](https://docs.rs/object_store/latest/object_store/struct.WriteMultipart.html "struct object_store::WriteMultipart") without copying

Data is buffered using [`PutPayloadMut::push`](https://docs.rs/object_store/latest/object_store/struct.PutPayloadMut.html#method.push "method object_store::PutPayloadMut::push"). Implementations looking to perform writes from non-owned buffers should prefer [`Self::write`](https://docs.rs/object_store/latest/object_store/struct.WriteMultipart.html#method.write "method object_store::WriteMultipart::write") as this will allow multiple calls to share the same underlying allocation.

See [`Self::write`](https://docs.rs/object_store/latest/object_store/struct.WriteMultipart.html#method.write "method object_store::WriteMultipart::write") for information on backpressure

#### pub async fn <a href="https://docs.rs/object_store/latest/object_store/struct.WriteMultipart.html#method.abort" class="fn">abort</a>(self) -\> <a href="https://docs.rs/object_store/latest/object_store/type.Result.html" class="type" title="type object_store::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

Abort this upload, attempting to clean up any successfully uploaded parts

#### pub async fn <a href="https://docs.rs/object_store/latest/object_store/struct.WriteMultipart.html#method.finish" class="fn">finish</a>(self) -\> <a href="https://docs.rs/object_store/latest/object_store/type.Result.html" class="type" title="type object_store::Result">Result</a>\<<a href="https://docs.rs/object_store/latest/object_store/struct.PutResult.html" class="struct" title="struct object_store::PutResult">PutResult</a>\>

Flush final chunk, and await completion of all in-flight requests

## Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/struct.WriteMultipart.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/struct.WriteMultipart.html#impl-Debug-for-WriteMultipart" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/object_store/latest/object_store/struct.WriteMultipart.html" class="struct" title="struct object_store::WriteMultipart">WriteMultipart</a>

<a href="https://docs.rs/object_store/latest/object_store/struct.WriteMultipart.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/struct.WriteMultipart.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/object_store/latest/object_store/struct.WriteMultipart.html#blanket-implementations" class="anchor">§</a>
