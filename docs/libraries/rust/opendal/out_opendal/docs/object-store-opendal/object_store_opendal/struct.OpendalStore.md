# Struct OpendalStore Copy item path

<a href="https://opendal.apache.org/docs/object-store-opendal/src/object_store_opendal/store.rs.html#97-100" class="src">Source</a>

``` rust
pub struct OpendalStore { /* private fields */ }
```

Expand description

OpendalStore implements ObjectStore trait by using opendal.

This allows users to use opendal as an object store without extra cost.

Visit \[`opendal::services`\] for more information about supported services.

``` rust
use std::sync::Arc;

use bytes::Bytes;
use object_store::path::Path;
use object_store::ObjectStore;
use object_store_opendal::OpendalStore;
use opendal::services::S3;
use opendal::{Builder, Operator};

#[tokio::main]
async fn main() {
   let builder = S3::default()
    .access_key_id("my_access_key")
    .secret_access_key("my_secret_key")
    .endpoint("my_endpoint")
    .region("my_region");

    // Create a new operator
    let operator = Operator::new(builder).unwrap().finish();

    // Create a new object store
    let object_store = Arc::new(OpendalStore::new(operator));

    let path = Path::from("data/nested/test.txt");
    let bytes = Bytes::from_static(b"hello, world! I am nested.");

    object_store.put(&path, bytes.clone().into()).await.unwrap();

    let content = object_store
        .get(&path)
        .await
        .unwrap()
        .bytes()
        .await
        .unwrap();

    assert_eq!(content, bytes);
}
```

## Implementations<a href="https://opendal.apache.org/docs/object-store-opendal/object_store_opendal/struct.OpendalStore.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/object-store-opendal/object_store_opendal/struct.OpendalStore.html#impl-OpendalStore" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/object-store-opendal/object_store_opendal/struct.OpendalStore.html" class="struct" title="struct object_store_opendal::OpendalStore">OpendalStore</a>

#### pub fn <a href="https://opendal.apache.org/docs/object-store-opendal/object_store_opendal/struct.OpendalStore.html#method.new" class="fn">new</a>(op: Operator) -\> Self

Create OpendalStore by given Operator.

#### pub fn <a href="https://opendal.apache.org/docs/object-store-opendal/object_store_opendal/struct.OpendalStore.html#method.info" class="fn">info</a>(&self) -\> &OperatorInfo

Get the Operator info.

<a href="https://opendal.apache.org/docs/object-store-opendal/object_store_opendal/struct.OpendalStore.html#impl-OpendalStore-1" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/object-store-opendal/object_store_opendal/struct.OpendalStore.html" class="struct" title="struct object_store_opendal::OpendalStore">OpendalStore</a>

#### pub fn <a href="https://opendal.apache.org/docs/object-store-opendal/object_store_opendal/struct.OpendalStore.html#method.new_amazon_s3" class="fn">new_amazon_s3</a>(builder: AmazonS3Builder) -\> Result\<<a href="https://opendal.apache.org/docs/object-store-opendal/object_store_opendal/struct.OpendalStore.html" class="struct" title="struct object_store_opendal::OpendalStore">OpendalStore</a>\>

Create OpendalStore from object_store Amazon S3 builder.

## Trait Implementations<a href="https://opendal.apache.org/docs/object-store-opendal/object_store_opendal/struct.OpendalStore.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/object-store-opendal/object_store_opendal/struct.OpendalStore.html#impl-Clone-for-OpendalStore" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://opendal.apache.org/docs/object-store-opendal/object_store_opendal/struct.OpendalStore.html" class="struct" title="struct object_store_opendal::OpendalStore">OpendalStore</a>

<a href="https://opendal.apache.org/docs/object-store-opendal/object_store_opendal/struct.OpendalStore.html#method.clone" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://opendal.apache.org/docs/object-store-opendal/object_store_opendal/struct.OpendalStore.html" class="struct" title="struct object_store_opendal::OpendalStore">OpendalStore</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://opendal.apache.org/docs/object-store-opendal/object_store_opendal/struct.OpendalStore.html#method.clone_from" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://opendal.apache.org/docs/object-store-opendal/object_store_opendal/struct.OpendalStore.html#impl-Debug-for-OpendalStore" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://opendal.apache.org/docs/object-store-opendal/object_store_opendal/struct.OpendalStore.html" class="struct" title="struct object_store_opendal::OpendalStore">OpendalStore</a>

<a href="https://opendal.apache.org/docs/object-store-opendal/object_store_opendal/struct.OpendalStore.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://opendal.apache.org/docs/object-store-opendal/object_store_opendal/struct.OpendalStore.html#impl-Display-for-OpendalStore" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> for <a href="https://opendal.apache.org/docs/object-store-opendal/object_store_opendal/struct.OpendalStore.html" class="struct" title="struct object_store_opendal::OpendalStore">OpendalStore</a>

<a href="https://opendal.apache.org/docs/object-store-opendal/object_store_opendal/struct.OpendalStore.html#method.fmt-1" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

<a href="https://opendal.apache.org/docs/object-store-opendal/object_store_opendal/struct.OpendalStore.html#impl-From%3COperator%3E-for-OpendalStore" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<Operator\> for <a href="https://opendal.apache.org/docs/object-store-opendal/object_store_opendal/struct.OpendalStore.html" class="struct" title="struct object_store_opendal::OpendalStore">OpendalStore</a>

<a href="https://opendal.apache.org/docs/object-store-opendal/object_store_opendal/struct.OpendalStore.html#method.from" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: Operator) -\> Self

Converts to this type from the input type.

<a href="https://opendal.apache.org/docs/object-store-opendal/object_store_opendal/struct.OpendalStore.html#impl-ObjectStore-for-OpendalStore" class="anchor">Â§</a>

### impl ObjectStore for <a href="https://opendal.apache.org/docs/object-store-opendal/object_store_opendal/struct.OpendalStore.html" class="struct" title="struct object_store_opendal::OpendalStore">OpendalStore</a>

<a href="https://opendal.apache.org/docs/object-store-opendal/object_store_opendal/struct.OpendalStore.html#method.put_opts" class="anchor">Â§</a>

#### fn put_opts\<'life0, 'life1, 'async_trait\>( &'life0 self, location: &'life1 Path, bytes: PutPayload, opts: PutOptions, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = Result\<PutResult\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where Self: 'async_trait, 'life0: 'async_trait, 'life1: 'async_trait,

Save the provided `payload` to `location` with the given options

<a href="https://opendal.apache.org/docs/object-store-opendal/object_store_opendal/struct.OpendalStore.html#method.put_multipart" class="anchor">Â§</a>

#### fn put_multipart\<'life0, 'life1, 'async_trait\>( &'life0 self, location: &'life1 Path, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = Result\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn MultipartUpload\>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where Self: 'async_trait, 'life0: 'async_trait, 'life1: 'async_trait,

Perform a multipart upload Read more

<a href="https://opendal.apache.org/docs/object-store-opendal/object_store_opendal/struct.OpendalStore.html#method.put_multipart_opts" class="anchor">Â§</a>

#### fn put_multipart_opts\<'life0, 'life1, 'async_trait\>( &'life0 self, location: &'life1 Path, opts: PutMultipartOptions, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = Result\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn MultipartUpload\>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where Self: 'async_trait, 'life0: 'async_trait, 'life1: 'async_trait,

Perform a multipart upload with options Read more

<a href="https://opendal.apache.org/docs/object-store-opendal/object_store_opendal/struct.OpendalStore.html#method.get_opts" class="anchor">Â§</a>

#### fn get_opts\<'life0, 'life1, 'async_trait\>( &'life0 self, location: &'life1 Path, options: GetOptions, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = Result\<GetResult\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where Self: 'async_trait, 'life0: 'async_trait, 'life1: 'async_trait,

Perform a get request with options

<a href="https://opendal.apache.org/docs/object-store-opendal/object_store_opendal/struct.OpendalStore.html#method.delete" class="anchor">Â§</a>

#### fn delete\<'life0, 'life1, 'async_trait\>( &'life0 self, location: &'life1 Path, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = Result\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where Self: 'async_trait, 'life0: 'async_trait, 'life1: 'async_trait,

Delete the object at the specified location.

<a href="https://opendal.apache.org/docs/object-store-opendal/object_store_opendal/struct.OpendalStore.html#method.list" class="anchor">Â§</a>

#### fn list(&self, prefix: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&Path\>) -\> BoxStream\<'static, Result\<ObjectMeta\>\>

List all the objects with the given prefix. Read more

<a href="https://opendal.apache.org/docs/object-store-opendal/object_store_opendal/struct.OpendalStore.html#method.list_with_offset" class="anchor">Â§</a>

#### fn list_with_offset( &self, prefix: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&Path\>, offset: &Path, ) -\> BoxStream\<'static, Result\<ObjectMeta\>\>

List all the objects with the given prefix and a location greater than `offset` Read more

<a href="https://opendal.apache.org/docs/object-store-opendal/object_store_opendal/struct.OpendalStore.html#method.list_with_delimiter" class="anchor">Â§</a>

#### fn list_with_delimiter\<'life0, 'life1, 'async_trait\>( &'life0 self, prefix: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&'life1 Path\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = Result\<ListResult\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where Self: 'async_trait, 'life0: 'async_trait, 'life1: 'async_trait,

List objects with the given prefix and an implementation specific delimiter. Returns common prefixes (directories) in addition to object metadata. Read more

<a href="https://opendal.apache.org/docs/object-store-opendal/object_store_opendal/struct.OpendalStore.html#method.copy" class="anchor">Â§</a>

#### fn copy\<'life0, 'life1, 'life2, 'async_trait\>( &'life0 self, from: &'life1 Path, to: &'life2 Path, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = Result\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where Self: 'async_trait, 'life0: 'async_trait, 'life1: 'async_trait, 'life2: 'async_trait,

Copy an object from one path to another in the same object store. Read more

<a href="https://opendal.apache.org/docs/object-store-opendal/object_store_opendal/struct.OpendalStore.html#method.copy_if_not_exists" class="anchor">Â§</a>

#### fn copy_if_not_exists\<'life0, 'life1, 'life2, 'async_trait\>( &'life0 self, from: &'life1 Path, to: &'life2 Path, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = Result\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where Self: 'async_trait, 'life0: 'async_trait, 'life1: 'async_trait, 'life2: 'async_trait,

Copy an object from one path to another, only if destination is empty. Read more

<a href="https://opendal.apache.org/docs/object-store-opendal/object_store_opendal/struct.OpendalStore.html#method.rename" class="anchor">Â§</a>

#### fn rename\<'life0, 'life1, 'life2, 'async_trait\>( &'life0 self, from: &'life1 Path, to: &'life2 Path, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = Result\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where Self: 'async_trait, 'life0: 'async_trait, 'life1: 'async_trait, 'life2: 'async_trait,

Move an object from one path to another in the same object store. Read more

<a href="https://opendal.apache.org/docs/object-store-opendal/object_store_opendal/struct.OpendalStore.html#method.put" class="anchor">Â§</a>

#### fn put\<'life0, 'life1, 'async_trait\>( &'life0 self, location: &'life1 Path, payload: PutPayload, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<PutResult, Error\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where 'life0: 'async_trait, 'life1: 'async_trait, Self: 'async_trait,

Save the provided bytes to the specified location Read more

<a href="https://opendal.apache.org/docs/object-store-opendal/object_store_opendal/struct.OpendalStore.html#method.get" class="anchor">Â§</a>

#### fn get\<'life0, 'life1, 'async_trait\>( &'life0 self, location: &'life1 Path, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<GetResult, Error\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where 'life0: 'async_trait, 'life1: 'async_trait, Self: 'async_trait,

Return the bytes that are stored at the specified location.

<a href="https://opendal.apache.org/docs/object-store-opendal/object_store_opendal/struct.OpendalStore.html#method.get_range" class="anchor">Â§</a>

#### fn get_range\<'life0, 'life1, 'async_trait\>( &'life0 self, location: &'life1 Path, range: <a href="https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html" class="struct" title="struct core::ops::range::Range">Range</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<Bytes, Error\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where 'life0: 'async_trait, 'life1: 'async_trait, Self: 'async_trait,

Return the bytes that are stored at the specified location in the given byte range. Read more

<a href="https://opendal.apache.org/docs/object-store-opendal/object_store_opendal/struct.OpendalStore.html#method.get_ranges" class="anchor">Â§</a>

#### fn get_ranges\<'life0, 'life1, 'life2, 'async_trait\>( &'life0 self, location: &'life1 Path, ranges: &'life2 \[<a href="https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html" class="struct" title="struct core::ops::range::Range">Range</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>\], ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<Bytes\>, Error\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where 'life0: 'async_trait, 'life1: 'async_trait, 'life2: 'async_trait, Self: 'async_trait,

Return the bytes that are stored at the specified location in the given byte ranges

<a href="https://opendal.apache.org/docs/object-store-opendal/object_store_opendal/struct.OpendalStore.html#method.head" class="anchor">Â§</a>

#### fn head\<'life0, 'life1, 'async_trait\>( &'life0 self, location: &'life1 Path, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<ObjectMeta, Error\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where 'life0: 'async_trait, 'life1: 'async_trait, Self: 'async_trait,

Return the metadata for the specified location

<a href="https://opendal.apache.org/docs/object-store-opendal/object_store_opendal/struct.OpendalStore.html#method.delete_stream" class="anchor">Â§</a>

#### fn delete_stream\<'a\>( &'a self, locations: <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn Stream\<Item = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<Path, Error\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'a\>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn Stream\<Item = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<Path, Error\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'a\>\>

Delete all the objects at the specified locations Read more

<a href="https://opendal.apache.org/docs/object-store-opendal/object_store_opendal/struct.OpendalStore.html#method.rename_if_not_exists" class="anchor">Â§</a>

#### fn rename_if_not_exists\<'life0, 'life1, 'life2, 'async_trait\>( &'life0 self, from: &'life1 Path, to: &'life2 Path, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, Error\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where 'life0: 'async_trait, 'life1: 'async_trait, 'life2: 'async_trait, Self: 'async_trait,

Move an object from one path to another in the same object store. Read more

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/object-store-opendal/object_store_opendal/struct.OpendalStore.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/object-store-opendal/object_store_opendal/struct.OpendalStore.html#blanket-implementations" class="anchor">Â§</a>
