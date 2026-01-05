# Trait MultipartUpload Copy item path

<a href="https://docs.rs/object_store/latest/src/object_store/upload.rs.html#41-93" class="src">Source</a>

``` rust
pub trait MultipartUpload: Send + Debug {
    // Required methods
    fn put_part(&mut self, data: PutPayload) -> UploadPart;
    fn complete<'life0, 'async_trait>(
        &'life0 mut self,
    ) -> Pin<Box<dyn Future<Output = Result<PutResult>> + Send + 'async_trait>>
       where Self: 'async_trait,
             'life0: 'async_trait;
    fn abort<'life0, 'async_trait>(
        &'life0 mut self,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + Send + 'async_trait>>
       where Self: 'async_trait,
             'life0: 'async_trait;
}
```

Expand description

A trait allowing writing an object in fixed size chunks

Consecutive chunks of data can be written by calling [`MultipartUpload::put_part`](https://docs.rs/object_store/latest/object_store/trait.MultipartUpload.html#tymethod.put_part "method object_store::MultipartUpload::put_part") and polling the returned futures to completion. Multiple futures returned by [`MultipartUpload::put_part`](https://docs.rs/object_store/latest/object_store/trait.MultipartUpload.html#tymethod.put_part "method object_store::MultipartUpload::put_part") may be polled in parallel, allowing for concurrent uploads.

Once all part uploads have been polled to completion, the upload can be completed by calling [`MultipartUpload::complete`](https://docs.rs/object_store/latest/object_store/trait.MultipartUpload.html#tymethod.complete "method object_store::MultipartUpload::complete"). This will make the entire uploaded object visible as an atomic operation.It is implementation behind behaviour if [`MultipartUpload::complete`](https://docs.rs/object_store/latest/object_store/trait.MultipartUpload.html#tymethod.complete "method object_store::MultipartUpload::complete") is called before all [`UploadPart`](https://docs.rs/object_store/latest/object_store/type.UploadPart.html "type object_store::UploadPart") have been polled to completion.

## Required Methods<a href="https://docs.rs/object_store/latest/object_store/trait.MultipartUpload.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/object_store/latest/object_store/trait.MultipartUpload.html#tymethod.put_part" class="fn">put_part</a>(&mut self, data: <a href="https://docs.rs/object_store/latest/object_store/struct.PutPayload.html" class="struct" title="struct object_store::PutPayload">PutPayload</a>) -\> <a href="https://docs.rs/object_store/latest/object_store/type.UploadPart.html" class="type" title="type object_store::UploadPart">UploadPart</a>

Upload the next part

Most stores require that all parts excluding the last are at least 5 MiB, and some further require that all parts excluding the last be the same size, e.g. [R2](https://developers.cloudflare.com/r2/objects/multipart-objects/#limitations). Clients wanting to maximise compatibility should therefore perform writes in fixed size blocks larger than 5 MiB.

Implementations may invoke this method multiple times and then await on the returned futures in parallel

``` rust
let mut upload: Box<&dyn MultipartUpload> = todo!();
let p1 = upload.put_part(vec![0; 10 * 1024 * 1024].into());
let p2 = upload.put_part(vec![1; 10 * 1024 * 1024].into());
futures::future::try_join(p1, p2).await.unwrap();
upload.complete().await.unwrap();
```

#### fn <a href="https://docs.rs/object_store/latest/object_store/trait.MultipartUpload.html#tymethod.complete" class="fn">complete</a>\<'life0, 'async_trait\>( &'life0 mut self, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://docs.rs/object_store/latest/object_store/type.Result.html" class="type" title="type object_store::Result">Result</a>\<<a href="https://docs.rs/object_store/latest/object_store/struct.PutResult.html" class="struct" title="struct object_store::PutResult">PutResult</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where Self: 'async_trait, 'life0: 'async_trait,

Complete the multipart upload

It is implementation defined behaviour if this method is called before polling all [`UploadPart`](https://docs.rs/object_store/latest/object_store/type.UploadPart.html "type object_store::UploadPart") returned by [`MultipartUpload::put_part`](https://docs.rs/object_store/latest/object_store/trait.MultipartUpload.html#tymethod.put_part "method object_store::MultipartUpload::put_part") to completion. Additionally, it is implementation defined behaviour to call [`MultipartUpload::complete`](https://docs.rs/object_store/latest/object_store/trait.MultipartUpload.html#tymethod.complete "method object_store::MultipartUpload::complete") on an already completed or aborted [`MultipartUpload`](https://docs.rs/object_store/latest/object_store/trait.MultipartUpload.html "trait object_store::MultipartUpload").

#### fn <a href="https://docs.rs/object_store/latest/object_store/trait.MultipartUpload.html#tymethod.abort" class="fn">abort</a>\<'life0, 'async_trait\>( &'life0 mut self, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://docs.rs/object_store/latest/object_store/type.Result.html" class="type" title="type object_store::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where Self: 'async_trait, 'life0: 'async_trait,

Abort the multipart upload

If a [`MultipartUpload`](https://docs.rs/object_store/latest/object_store/trait.MultipartUpload.html "trait object_store::MultipartUpload") is dropped without calling [`MultipartUpload::complete`](https://docs.rs/object_store/latest/object_store/trait.MultipartUpload.html#tymethod.complete "method object_store::MultipartUpload::complete"), some object stores will automatically clean up any previously uploaded parts. However, some stores, such as S3 and GCS, cannot perform cleanup on drop. As such [`MultipartUpload::abort`](https://docs.rs/object_store/latest/object_store/trait.MultipartUpload.html#tymethod.abort "method object_store::MultipartUpload::abort") can be invoked to perform this cleanup.

It will not be possible to call `abort` in all failure scenarios, for example non-graceful shutdown of the calling application. It is therefore recommended object stores are configured with lifecycle rules to automatically cleanup unused parts older than some threshold. See [crate::aws](https://docs.rs/object_store/latest/object_store/aws/index.html "mod object_store::aws") and [crate::gcp](https://docs.rs/object_store/latest/object_store/gcp/index.html "mod object_store::gcp") for more information.

It is implementation defined behaviour to call [`MultipartUpload::abort`](https://docs.rs/object_store/latest/object_store/trait.MultipartUpload.html#tymethod.abort "method object_store::MultipartUpload::abort") on an already completed or aborted [`MultipartUpload`](https://docs.rs/object_store/latest/object_store/trait.MultipartUpload.html "trait object_store::MultipartUpload")

## Implementations on Foreign Types<a href="https://docs.rs/object_store/latest/object_store/trait.MultipartUpload.html#foreign-impls" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/trait.MultipartUpload.html#impl-MultipartUpload-for-Box%3CW%3E" class="anchor">§</a>

### impl\<W: <a href="https://docs.rs/object_store/latest/object_store/trait.MultipartUpload.html" class="trait" title="trait object_store::MultipartUpload">MultipartUpload</a> + ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>\> <a href="https://docs.rs/object_store/latest/object_store/trait.MultipartUpload.html" class="trait" title="trait object_store::MultipartUpload">MultipartUpload</a> for <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<W\>

<a href="https://docs.rs/object_store/latest/object_store/trait.MultipartUpload.html#method.put_part" class="anchor">§</a>

#### fn <a href="https://docs.rs/object_store/latest/object_store/trait.MultipartUpload.html#tymethod.put_part" class="fn">put_part</a>(&mut self, data: <a href="https://docs.rs/object_store/latest/object_store/struct.PutPayload.html" class="struct" title="struct object_store::PutPayload">PutPayload</a>) -\> <a href="https://docs.rs/object_store/latest/object_store/type.UploadPart.html" class="type" title="type object_store::UploadPart">UploadPart</a>

<a href="https://docs.rs/object_store/latest/object_store/trait.MultipartUpload.html#method.complete" class="anchor">§</a>

#### fn <a href="https://docs.rs/object_store/latest/object_store/trait.MultipartUpload.html#tymethod.complete" class="fn">complete</a>\<'life0, 'async_trait\>( &'life0 mut self, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://docs.rs/object_store/latest/object_store/type.Result.html" class="type" title="type object_store::Result">Result</a>\<<a href="https://docs.rs/object_store/latest/object_store/struct.PutResult.html" class="struct" title="struct object_store::PutResult">PutResult</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where Self: 'async_trait, 'life0: 'async_trait,

<a href="https://docs.rs/object_store/latest/object_store/trait.MultipartUpload.html#method.abort" class="anchor">§</a>

#### fn <a href="https://docs.rs/object_store/latest/object_store/trait.MultipartUpload.html#tymethod.abort" class="fn">abort</a>\<'life0, 'async_trait\>( &'life0 mut self, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://docs.rs/object_store/latest/object_store/type.Result.html" class="type" title="type object_store::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where Self: 'async_trait, 'life0: 'async_trait,

## Implementors<a href="https://docs.rs/object_store/latest/object_store/trait.MultipartUpload.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/trait.MultipartUpload.html#impl-MultipartUpload-for-LimitUpload" class="anchor">§</a>

### impl <a href="https://docs.rs/object_store/latest/object_store/trait.MultipartUpload.html" class="trait" title="trait object_store::MultipartUpload">MultipartUpload</a> for <a href="https://docs.rs/object_store/latest/object_store/limit/struct.LimitUpload.html" class="struct" title="struct object_store::limit::LimitUpload">LimitUpload</a>
