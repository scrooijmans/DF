# Struct LimitUpload Copy item path

<a href="https://docs.rs/object_store/latest/src/object_store/limit.rs.html#240-243" class="src">Source</a>

``` rust
pub struct LimitUpload { /* private fields */ }
```

Expand description

An [`MultipartUpload`](https://docs.rs/object_store/latest/object_store/trait.MultipartUpload.html "trait object_store::MultipartUpload") wrapper that limits the maximum number of concurrent requests

## Implementations<a href="https://docs.rs/object_store/latest/object_store/limit/struct.LimitUpload.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/limit/struct.LimitUpload.html#impl-LimitUpload" class="anchor">§</a>

### impl <a href="https://docs.rs/object_store/latest/object_store/limit/struct.LimitUpload.html" class="struct" title="struct object_store::limit::LimitUpload">LimitUpload</a>

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/limit/struct.LimitUpload.html#method.new" class="fn">new</a>(upload: <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/object_store/latest/object_store/trait.MultipartUpload.html" class="trait" title="trait object_store::MultipartUpload">MultipartUpload</a>\>, max_concurrency: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> Self

Create a new [`LimitUpload`](https://docs.rs/object_store/latest/object_store/limit/struct.LimitUpload.html "struct object_store::limit::LimitUpload") limiting `upload` to `max_concurrency` concurrent requests

## Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/limit/struct.LimitUpload.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/limit/struct.LimitUpload.html#impl-Debug-for-LimitUpload" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/object_store/latest/object_store/limit/struct.LimitUpload.html" class="struct" title="struct object_store::limit::LimitUpload">LimitUpload</a>

<a href="https://docs.rs/object_store/latest/object_store/limit/struct.LimitUpload.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/object_store/latest/object_store/limit/struct.LimitUpload.html#impl-MultipartUpload-for-LimitUpload" class="anchor">§</a>

### impl <a href="https://docs.rs/object_store/latest/object_store/trait.MultipartUpload.html" class="trait" title="trait object_store::MultipartUpload">MultipartUpload</a> for <a href="https://docs.rs/object_store/latest/object_store/limit/struct.LimitUpload.html" class="struct" title="struct object_store::limit::LimitUpload">LimitUpload</a>

<a href="https://docs.rs/object_store/latest/object_store/limit/struct.LimitUpload.html#method.put_part" class="anchor">§</a>

#### fn <a href="https://docs.rs/object_store/latest/object_store/trait.MultipartUpload.html#tymethod.put_part" class="fn">put_part</a>(&mut self, data: <a href="https://docs.rs/object_store/latest/object_store/struct.PutPayload.html" class="struct" title="struct object_store::PutPayload">PutPayload</a>) -\> <a href="https://docs.rs/object_store/latest/object_store/type.UploadPart.html" class="type" title="type object_store::UploadPart">UploadPart</a>

Upload the next part [Read more](https://docs.rs/object_store/latest/object_store/trait.MultipartUpload.html#tymethod.put_part)

<a href="https://docs.rs/object_store/latest/object_store/limit/struct.LimitUpload.html#method.complete" class="anchor">§</a>

#### fn <a href="https://docs.rs/object_store/latest/object_store/trait.MultipartUpload.html#tymethod.complete" class="fn">complete</a>\<'life0, 'async_trait\>( &'life0 mut self, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://docs.rs/object_store/latest/object_store/type.Result.html" class="type" title="type object_store::Result">Result</a>\<<a href="https://docs.rs/object_store/latest/object_store/struct.PutResult.html" class="struct" title="struct object_store::PutResult">PutResult</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where Self: 'async_trait, 'life0: 'async_trait,

Complete the multipart upload [Read more](https://docs.rs/object_store/latest/object_store/trait.MultipartUpload.html#tymethod.complete)

<a href="https://docs.rs/object_store/latest/object_store/limit/struct.LimitUpload.html#method.abort" class="anchor">§</a>

#### fn <a href="https://docs.rs/object_store/latest/object_store/trait.MultipartUpload.html#tymethod.abort" class="fn">abort</a>\<'life0, 'async_trait\>( &'life0 mut self, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://docs.rs/object_store/latest/object_store/type.Result.html" class="type" title="type object_store::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where Self: 'async_trait, 'life0: 'async_trait,

Abort the multipart upload [Read more](https://docs.rs/object_store/latest/object_store/trait.MultipartUpload.html#tymethod.abort)

## Auto Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/limit/struct.LimitUpload.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/object_store/latest/object_store/limit/struct.LimitUpload.html#blanket-implementations" class="anchor">§</a>
