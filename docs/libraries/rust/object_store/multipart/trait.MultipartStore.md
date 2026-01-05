# Trait MultipartStore Copy item path

<a href="https://docs.rs/object_store/latest/src/object_store/multipart.rs.html#45-84" class="src">Source</a>

``` rust
pub trait MultipartStore:
    Send
    + Sync
    + 'static {
    // Required methods
    fn create_multipart<'life0, 'life1, 'async_trait>(
        &'life0 self,
        path: &'life1 Path,
    ) -> Pin<Box<dyn Future<Output = Result<MultipartId>> + Send + 'async_trait>>
       where Self: 'async_trait,
             'life0: 'async_trait,
             'life1: 'async_trait;
    fn put_part<'life0, 'life1, 'life2, 'async_trait>(
        &'life0 self,
        path: &'life1 Path,
        id: &'life2 MultipartId,
        part_idx: usize,
        data: PutPayload,
    ) -> Pin<Box<dyn Future<Output = Result<PartId>> + Send + 'async_trait>>
       where Self: 'async_trait,
             'life0: 'async_trait,
             'life1: 'async_trait,
             'life2: 'async_trait;
    fn complete_multipart<'life0, 'life1, 'life2, 'async_trait>(
        &'life0 self,
        path: &'life1 Path,
        id: &'life2 MultipartId,
        parts: Vec<PartId>,
    ) -> Pin<Box<dyn Future<Output = Result<PutResult>> + Send + 'async_trait>>
       where Self: 'async_trait,
             'life0: 'async_trait,
             'life1: 'async_trait,
             'life2: 'async_trait;
    fn abort_multipart<'life0, 'life1, 'life2, 'async_trait>(
        &'life0 self,
        path: &'life1 Path,
        id: &'life2 MultipartId,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + Send + 'async_trait>>
       where Self: 'async_trait,
             'life0: 'async_trait,
             'life1: 'async_trait,
             'life2: 'async_trait;
}
```

Expand description

A low-level interface for interacting with multipart upload APIs

Most use-cases should prefer [`ObjectStore::put_multipart`](https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html#method.put_multipart "method object_store::ObjectStore::put_multipart") as this is supported by more backends, including [`LocalFileSystem`](https://docs.rs/object_store/latest/object_store/local/struct.LocalFileSystem.html "struct object_store::local::LocalFileSystem"), and automatically handles uploading fixed size parts of sufficient size in parallel

## Required Methods<a href="https://docs.rs/object_store/latest/object_store/multipart/trait.MultipartStore.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/object_store/latest/object_store/multipart/trait.MultipartStore.html#tymethod.create_multipart" class="fn">create_multipart</a>\<'life0, 'life1, 'async_trait\>( &'life0 self, path: &'life1 <a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://docs.rs/object_store/latest/object_store/type.Result.html" class="type" title="type object_store::Result">Result</a>\<<a href="https://docs.rs/object_store/latest/object_store/type.MultipartId.html" class="type" title="type object_store::MultipartId">MultipartId</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where Self: 'async_trait, 'life0: 'async_trait, 'life1: 'async_trait,

Creates a new multipart upload, returning the [`MultipartId`](https://docs.rs/object_store/latest/object_store/type.MultipartId.html "type object_store::MultipartId")

#### fn <a href="https://docs.rs/object_store/latest/object_store/multipart/trait.MultipartStore.html#tymethod.put_part" class="fn">put_part</a>\<'life0, 'life1, 'life2, 'async_trait\>( &'life0 self, path: &'life1 <a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>, id: &'life2 <a href="https://docs.rs/object_store/latest/object_store/type.MultipartId.html" class="type" title="type object_store::MultipartId">MultipartId</a>, part_idx: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, data: <a href="https://docs.rs/object_store/latest/object_store/struct.PutPayload.html" class="struct" title="struct object_store::PutPayload">PutPayload</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://docs.rs/object_store/latest/object_store/type.Result.html" class="type" title="type object_store::Result">Result</a>\<<a href="https://docs.rs/object_store/latest/object_store/multipart/struct.PartId.html" class="struct" title="struct object_store::multipart::PartId">PartId</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where Self: 'async_trait, 'life0: 'async_trait, 'life1: 'async_trait, 'life2: 'async_trait,

Uploads a new part with index `part_idx`

`part_idx` should be an integer in the range `0..N` where `N` is the number of parts in the upload. Parts may be uploaded concurrently and in any order.

Most stores require that all parts excluding the last are at least 5 MiB, and some further require that all parts excluding the last be the same size, e.g. [R2](https://developers.cloudflare.com/r2/objects/multipart-objects/#limitations). [`WriteMultipart`](https://docs.rs/object_store/latest/object_store/struct.WriteMultipart.html "struct object_store::WriteMultipart") performs writes in fixed size blocks of 5 MiB, and clients wanting to maximise compatibility should look to do likewise.

#### fn <a href="https://docs.rs/object_store/latest/object_store/multipart/trait.MultipartStore.html#tymethod.complete_multipart" class="fn">complete_multipart</a>\<'life0, 'life1, 'life2, 'async_trait\>( &'life0 self, path: &'life1 <a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>, id: &'life2 <a href="https://docs.rs/object_store/latest/object_store/type.MultipartId.html" class="type" title="type object_store::MultipartId">MultipartId</a>, parts: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/object_store/latest/object_store/multipart/struct.PartId.html" class="struct" title="struct object_store::multipart::PartId">PartId</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://docs.rs/object_store/latest/object_store/type.Result.html" class="type" title="type object_store::Result">Result</a>\<<a href="https://docs.rs/object_store/latest/object_store/struct.PutResult.html" class="struct" title="struct object_store::PutResult">PutResult</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where Self: 'async_trait, 'life0: 'async_trait, 'life1: 'async_trait, 'life2: 'async_trait,

Completes a multipart upload

The `i`’th value of `parts` must be a [`PartId`](https://docs.rs/object_store/latest/object_store/multipart/struct.PartId.html "struct object_store::multipart::PartId") returned by a call to [`Self::put_part`](https://docs.rs/object_store/latest/object_store/multipart/trait.MultipartStore.html#tymethod.put_part) with a `part_idx` of `i`, and the same `path` and `id` as provided to this method. Calling this method with out of sequence or repeated [`PartId`](https://docs.rs/object_store/latest/object_store/multipart/struct.PartId.html "struct object_store::multipart::PartId"), or [`PartId`](https://docs.rs/object_store/latest/object_store/multipart/struct.PartId.html "struct object_store::multipart::PartId") returned for other values of `path` or `id`, will result in implementation-defined behaviour

#### fn <a href="https://docs.rs/object_store/latest/object_store/multipart/trait.MultipartStore.html#tymethod.abort_multipart" class="fn">abort_multipart</a>\<'life0, 'life1, 'life2, 'async_trait\>( &'life0 self, path: &'life1 <a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>, id: &'life2 <a href="https://docs.rs/object_store/latest/object_store/type.MultipartId.html" class="type" title="type object_store::MultipartId">MultipartId</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://docs.rs/object_store/latest/object_store/type.Result.html" class="type" title="type object_store::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where Self: 'async_trait, 'life0: 'async_trait, 'life1: 'async_trait, 'life2: 'async_trait,

Aborts a multipart upload

## Implementors<a href="https://docs.rs/object_store/latest/object_store/multipart/trait.MultipartStore.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/multipart/trait.MultipartStore.html#impl-MultipartStore-for-AmazonS3" class="anchor">§</a>

### impl <a href="https://docs.rs/object_store/latest/object_store/multipart/trait.MultipartStore.html" class="trait" title="trait object_store::multipart::MultipartStore">MultipartStore</a> for <a href="https://docs.rs/object_store/latest/object_store/aws/struct.AmazonS3.html" class="struct" title="struct object_store::aws::AmazonS3">AmazonS3</a>

Available on **crate feature `aws`** only.

<a href="https://docs.rs/object_store/latest/object_store/multipart/trait.MultipartStore.html#impl-MultipartStore-for-MicrosoftAzure" class="anchor">§</a>

### impl <a href="https://docs.rs/object_store/latest/object_store/multipart/trait.MultipartStore.html" class="trait" title="trait object_store::multipart::MultipartStore">MultipartStore</a> for <a href="https://docs.rs/object_store/latest/object_store/azure/struct.MicrosoftAzure.html" class="struct" title="struct object_store::azure::MicrosoftAzure">MicrosoftAzure</a>

Available on **crate feature `azure`** only.

<a href="https://docs.rs/object_store/latest/object_store/multipart/trait.MultipartStore.html#impl-MultipartStore-for-GoogleCloudStorage" class="anchor">§</a>

### impl <a href="https://docs.rs/object_store/latest/object_store/multipart/trait.MultipartStore.html" class="trait" title="trait object_store::multipart::MultipartStore">MultipartStore</a> for <a href="https://docs.rs/object_store/latest/object_store/gcp/struct.GoogleCloudStorage.html" class="struct" title="struct object_store::gcp::GoogleCloudStorage">GoogleCloudStorage</a>

Available on **crate feature `gcp`** only.

<a href="https://docs.rs/object_store/latest/object_store/multipart/trait.MultipartStore.html#impl-MultipartStore-for-InMemory" class="anchor">§</a>

### impl <a href="https://docs.rs/object_store/latest/object_store/multipart/trait.MultipartStore.html" class="trait" title="trait object_store::multipart::MultipartStore">MultipartStore</a> for <a href="https://docs.rs/object_store/latest/object_store/memory/struct.InMemory.html" class="struct" title="struct object_store::memory::InMemory">InMemory</a>

<a href="https://docs.rs/object_store/latest/object_store/multipart/trait.MultipartStore.html#impl-MultipartStore-for-ThrottledStore%3CT%3E" class="anchor">§</a>

### impl\<T: <a href="https://docs.rs/object_store/latest/object_store/multipart/trait.MultipartStore.html" class="trait" title="trait object_store::multipart::MultipartStore">MultipartStore</a>\> <a href="https://docs.rs/object_store/latest/object_store/multipart/trait.MultipartStore.html" class="trait" title="trait object_store::multipart::MultipartStore">MultipartStore</a> for <a href="https://docs.rs/object_store/latest/object_store/throttle/struct.ThrottledStore.html" class="struct" title="struct object_store::throttle::ThrottledStore">ThrottledStore</a>\<T\>
