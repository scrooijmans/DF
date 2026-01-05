# Struct LimitStore Copy item path

<a href="https://docs.rs/object_store/latest/src/object_store/limit.rs.html#47-51" class="src">Source</a>

``` rust
pub struct LimitStore<T: ObjectStore> { /* private fields */ }
```

Expand description

Store wrapper that wraps an inner store and limits the maximum number of concurrent object store operations. Where each call to an [`ObjectStore`](https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html "trait object_store::ObjectStore") member function is considered a single operation, even if it may result in more than one network call

``` rust

// Create an in-memory `ObjectStore` limited to 20 concurrent requests
let store = LimitStore::new(InMemory::new(), 20);
```

## Implementations<a href="https://docs.rs/object_store/latest/object_store/limit/struct.LimitStore.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/limit/struct.LimitStore.html#impl-LimitStore%3CT%3E" class="anchor">§</a>

### impl\<T: <a href="https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html" class="trait" title="trait object_store::ObjectStore">ObjectStore</a>\> <a href="https://docs.rs/object_store/latest/object_store/limit/struct.LimitStore.html" class="struct" title="struct object_store::limit::LimitStore">LimitStore</a>\<T\>

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/limit/struct.LimitStore.html#method.new" class="fn">new</a>(inner: T, max_requests: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> Self

Create new limit store that will limit the maximum number of outstanding concurrent requests to `max_requests`

## Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/limit/struct.LimitStore.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/limit/struct.LimitStore.html#impl-Debug-for-LimitStore%3CT%3E" class="anchor">§</a>

### impl\<T: <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> + <a href="https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html" class="trait" title="trait object_store::ObjectStore">ObjectStore</a>\> <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/object_store/latest/object_store/limit/struct.LimitStore.html" class="struct" title="struct object_store::limit::LimitStore">LimitStore</a>\<T\>

<a href="https://docs.rs/object_store/latest/object_store/limit/struct.LimitStore.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/object_store/latest/object_store/limit/struct.LimitStore.html#impl-Display-for-LimitStore%3CT%3E" class="anchor">§</a>

### impl\<T: <a href="https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html" class="trait" title="trait object_store::ObjectStore">ObjectStore</a>\> <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> for <a href="https://docs.rs/object_store/latest/object_store/limit/struct.LimitStore.html" class="struct" title="struct object_store::limit::LimitStore">LimitStore</a>\<T\>

<a href="https://docs.rs/object_store/latest/object_store/limit/struct.LimitStore.html#method.fmt-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

<a href="https://docs.rs/object_store/latest/object_store/limit/struct.LimitStore.html#impl-ObjectStore-for-LimitStore%3CT%3E" class="anchor">§</a>

### impl\<T: <a href="https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html" class="trait" title="trait object_store::ObjectStore">ObjectStore</a>\> <a href="https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html" class="trait" title="trait object_store::ObjectStore">ObjectStore</a> for <a href="https://docs.rs/object_store/latest/object_store/limit/struct.LimitStore.html" class="struct" title="struct object_store::limit::LimitStore">LimitStore</a>\<T\>

<a href="https://docs.rs/object_store/latest/object_store/limit/struct.LimitStore.html#method.put" class="anchor">§</a>

#### fn <a href="https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html#method.put" class="fn">put</a>\<'life0, 'life1, 'async_trait\>( &'life0 self, location: &'life1 <a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>, payload: <a href="https://docs.rs/object_store/latest/object_store/struct.PutPayload.html" class="struct" title="struct object_store::PutPayload">PutPayload</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://docs.rs/object_store/latest/object_store/type.Result.html" class="type" title="type object_store::Result">Result</a>\<<a href="https://docs.rs/object_store/latest/object_store/struct.PutResult.html" class="struct" title="struct object_store::PutResult">PutResult</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where Self: 'async_trait, 'life0: 'async_trait, 'life1: 'async_trait,

Save the provided bytes to the specified location [Read more](https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html#method.put)

<a href="https://docs.rs/object_store/latest/object_store/limit/struct.LimitStore.html#method.put_opts" class="anchor">§</a>

#### fn <a href="https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html#tymethod.put_opts" class="fn">put_opts</a>\<'life0, 'life1, 'async_trait\>( &'life0 self, location: &'life1 <a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>, payload: <a href="https://docs.rs/object_store/latest/object_store/struct.PutPayload.html" class="struct" title="struct object_store::PutPayload">PutPayload</a>, opts: <a href="https://docs.rs/object_store/latest/object_store/struct.PutOptions.html" class="struct" title="struct object_store::PutOptions">PutOptions</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://docs.rs/object_store/latest/object_store/type.Result.html" class="type" title="type object_store::Result">Result</a>\<<a href="https://docs.rs/object_store/latest/object_store/struct.PutResult.html" class="struct" title="struct object_store::PutResult">PutResult</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where Self: 'async_trait, 'life0: 'async_trait, 'life1: 'async_trait,

Save the provided `payload` to `location` with the given options

<a href="https://docs.rs/object_store/latest/object_store/limit/struct.LimitStore.html#method.put_multipart" class="anchor">§</a>

#### fn <a href="https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html#method.put_multipart" class="fn">put_multipart</a>\<'life0, 'life1, 'async_trait\>( &'life0 self, location: &'life1 <a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://docs.rs/object_store/latest/object_store/type.Result.html" class="type" title="type object_store::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/object_store/latest/object_store/trait.MultipartUpload.html" class="trait" title="trait object_store::MultipartUpload">MultipartUpload</a>\>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where Self: 'async_trait, 'life0: 'async_trait, 'life1: 'async_trait,

Perform a multipart upload [Read more](https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html#method.put_multipart)

<a href="https://docs.rs/object_store/latest/object_store/limit/struct.LimitStore.html#method.put_multipart_opts" class="anchor">§</a>

#### fn <a href="https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html#tymethod.put_multipart_opts" class="fn">put_multipart_opts</a>\<'life0, 'life1, 'async_trait\>( &'life0 self, location: &'life1 <a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>, opts: <a href="https://docs.rs/object_store/latest/object_store/struct.PutMultipartOptions.html" class="struct" title="struct object_store::PutMultipartOptions">PutMultipartOptions</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://docs.rs/object_store/latest/object_store/type.Result.html" class="type" title="type object_store::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/object_store/latest/object_store/trait.MultipartUpload.html" class="trait" title="trait object_store::MultipartUpload">MultipartUpload</a>\>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where Self: 'async_trait, 'life0: 'async_trait, 'life1: 'async_trait,

Perform a multipart upload with options [Read more](https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html#tymethod.put_multipart_opts)

<a href="https://docs.rs/object_store/latest/object_store/limit/struct.LimitStore.html#method.get" class="anchor">§</a>

#### fn <a href="https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html#method.get" class="fn">get</a>\<'life0, 'life1, 'async_trait\>( &'life0 self, location: &'life1 <a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://docs.rs/object_store/latest/object_store/type.Result.html" class="type" title="type object_store::Result">Result</a>\<<a href="https://docs.rs/object_store/latest/object_store/struct.GetResult.html" class="struct" title="struct object_store::GetResult">GetResult</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where Self: 'async_trait, 'life0: 'async_trait, 'life1: 'async_trait,

Return the bytes that are stored at the specified location.

<a href="https://docs.rs/object_store/latest/object_store/limit/struct.LimitStore.html#method.get_opts" class="anchor">§</a>

#### fn <a href="https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html#tymethod.get_opts" class="fn">get_opts</a>\<'life0, 'life1, 'async_trait\>( &'life0 self, location: &'life1 <a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>, options: <a href="https://docs.rs/object_store/latest/object_store/struct.GetOptions.html" class="struct" title="struct object_store::GetOptions">GetOptions</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://docs.rs/object_store/latest/object_store/type.Result.html" class="type" title="type object_store::Result">Result</a>\<<a href="https://docs.rs/object_store/latest/object_store/struct.GetResult.html" class="struct" title="struct object_store::GetResult">GetResult</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where Self: 'async_trait, 'life0: 'async_trait, 'life1: 'async_trait,

Perform a get request with options

<a href="https://docs.rs/object_store/latest/object_store/limit/struct.LimitStore.html#method.get_range" class="anchor">§</a>

#### fn <a href="https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html#method.get_range" class="fn">get_range</a>\<'life0, 'life1, 'async_trait\>( &'life0 self, location: &'life1 <a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>, range: <a href="https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html" class="struct" title="struct core::ops::range::Range">Range</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://docs.rs/object_store/latest/object_store/type.Result.html" class="type" title="type object_store::Result">Result</a>\<<a href="https://docs.rs/bytes/1.10.1/x86_64-unknown-linux-gnu/bytes/bytes/struct.Bytes.html" class="struct" title="struct bytes::bytes::Bytes">Bytes</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where Self: 'async_trait, 'life0: 'async_trait, 'life1: 'async_trait,

Return the bytes that are stored at the specified location in the given byte range. [Read more](https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html#method.get_range)

<a href="https://docs.rs/object_store/latest/object_store/limit/struct.LimitStore.html#method.get_ranges" class="anchor">§</a>

#### fn <a href="https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html#method.get_ranges" class="fn">get_ranges</a>\<'life0, 'life1, 'life2, 'async_trait\>( &'life0 self, location: &'life1 <a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>, ranges: &'life2 \[<a href="https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html" class="struct" title="struct core::ops::range::Range">Range</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>\], ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://docs.rs/object_store/latest/object_store/type.Result.html" class="type" title="type object_store::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/bytes/1.10.1/x86_64-unknown-linux-gnu/bytes/bytes/struct.Bytes.html" class="struct" title="struct bytes::bytes::Bytes">Bytes</a>\>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where Self: 'async_trait, 'life0: 'async_trait, 'life1: 'async_trait, 'life2: 'async_trait,

Return the bytes that are stored at the specified location in the given byte ranges

<a href="https://docs.rs/object_store/latest/object_store/limit/struct.LimitStore.html#method.head" class="anchor">§</a>

#### fn <a href="https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html#method.head" class="fn">head</a>\<'life0, 'life1, 'async_trait\>( &'life0 self, location: &'life1 <a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://docs.rs/object_store/latest/object_store/type.Result.html" class="type" title="type object_store::Result">Result</a>\<<a href="https://docs.rs/object_store/latest/object_store/struct.ObjectMeta.html" class="struct" title="struct object_store::ObjectMeta">ObjectMeta</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where Self: 'async_trait, 'life0: 'async_trait, 'life1: 'async_trait,

Return the metadata for the specified location

<a href="https://docs.rs/object_store/latest/object_store/limit/struct.LimitStore.html#method.delete" class="anchor">§</a>

#### fn <a href="https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html#tymethod.delete" class="fn">delete</a>\<'life0, 'life1, 'async_trait\>( &'life0 self, location: &'life1 <a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://docs.rs/object_store/latest/object_store/type.Result.html" class="type" title="type object_store::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where Self: 'async_trait, 'life0: 'async_trait, 'life1: 'async_trait,

Delete the object at the specified location.

<a href="https://docs.rs/object_store/latest/object_store/limit/struct.LimitStore.html#method.delete_stream" class="anchor">§</a>

#### fn <a href="https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html#method.delete_stream" class="fn">delete_stream</a>\<'a\>( &'a self, locations: <a href="https://docs.rs/futures-core/0.3.31/x86_64-unknown-linux-gnu/futures_core/stream/type.BoxStream.html" class="type" title="type futures_core::stream::BoxStream">BoxStream</a>\<'a, <a href="https://docs.rs/object_store/latest/object_store/type.Result.html" class="type" title="type object_store::Result">Result</a>\<<a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>\>\>, ) -\> <a href="https://docs.rs/futures-core/0.3.31/x86_64-unknown-linux-gnu/futures_core/stream/type.BoxStream.html" class="type" title="type futures_core::stream::BoxStream">BoxStream</a>\<'a, <a href="https://docs.rs/object_store/latest/object_store/type.Result.html" class="type" title="type object_store::Result">Result</a>\<<a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>\>\>

Delete all the objects at the specified locations [Read more](https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html#method.delete_stream)

<a href="https://docs.rs/object_store/latest/object_store/limit/struct.LimitStore.html#method.list" class="anchor">§</a>

#### fn <a href="https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html#tymethod.list" class="fn">list</a>(&self, prefix: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>\>) -\> <a href="https://docs.rs/futures-core/0.3.31/x86_64-unknown-linux-gnu/futures_core/stream/type.BoxStream.html" class="type" title="type futures_core::stream::BoxStream">BoxStream</a>\<'static, <a href="https://docs.rs/object_store/latest/object_store/type.Result.html" class="type" title="type object_store::Result">Result</a>\<<a href="https://docs.rs/object_store/latest/object_store/struct.ObjectMeta.html" class="struct" title="struct object_store::ObjectMeta">ObjectMeta</a>\>\>

List all the objects with the given prefix. [Read more](https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html#tymethod.list)

<a href="https://docs.rs/object_store/latest/object_store/limit/struct.LimitStore.html#method.list_with_offset" class="anchor">§</a>

#### fn <a href="https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html#method.list_with_offset" class="fn">list_with_offset</a>( &self, prefix: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>\>, offset: &<a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>, ) -\> <a href="https://docs.rs/futures-core/0.3.31/x86_64-unknown-linux-gnu/futures_core/stream/type.BoxStream.html" class="type" title="type futures_core::stream::BoxStream">BoxStream</a>\<'static, <a href="https://docs.rs/object_store/latest/object_store/type.Result.html" class="type" title="type object_store::Result">Result</a>\<<a href="https://docs.rs/object_store/latest/object_store/struct.ObjectMeta.html" class="struct" title="struct object_store::ObjectMeta">ObjectMeta</a>\>\>

List all the objects with the given prefix and a location greater than `offset` [Read more](https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html#method.list_with_offset)

<a href="https://docs.rs/object_store/latest/object_store/limit/struct.LimitStore.html#method.list_with_delimiter" class="anchor">§</a>

#### fn <a href="https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html#tymethod.list_with_delimiter" class="fn">list_with_delimiter</a>\<'life0, 'life1, 'async_trait\>( &'life0 self, prefix: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&'life1 <a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://docs.rs/object_store/latest/object_store/type.Result.html" class="type" title="type object_store::Result">Result</a>\<<a href="https://docs.rs/object_store/latest/object_store/struct.ListResult.html" class="struct" title="struct object_store::ListResult">ListResult</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where Self: 'async_trait, 'life0: 'async_trait, 'life1: 'async_trait,

List objects with the given prefix and an implementation specific delimiter. Returns common prefixes (directories) in addition to object metadata. [Read more](https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html#tymethod.list_with_delimiter)

<a href="https://docs.rs/object_store/latest/object_store/limit/struct.LimitStore.html#method.copy" class="anchor">§</a>

#### fn <a href="https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html#tymethod.copy" class="fn">copy</a>\<'life0, 'life1, 'life2, 'async_trait\>( &'life0 self, from: &'life1 <a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>, to: &'life2 <a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://docs.rs/object_store/latest/object_store/type.Result.html" class="type" title="type object_store::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where Self: 'async_trait, 'life0: 'async_trait, 'life1: 'async_trait, 'life2: 'async_trait,

Copy an object from one path to another in the same object store. [Read more](https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html#tymethod.copy)

<a href="https://docs.rs/object_store/latest/object_store/limit/struct.LimitStore.html#method.rename" class="anchor">§</a>

#### fn <a href="https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html#method.rename" class="fn">rename</a>\<'life0, 'life1, 'life2, 'async_trait\>( &'life0 self, from: &'life1 <a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>, to: &'life2 <a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://docs.rs/object_store/latest/object_store/type.Result.html" class="type" title="type object_store::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where Self: 'async_trait, 'life0: 'async_trait, 'life1: 'async_trait, 'life2: 'async_trait,

Move an object from one path to another in the same object store. [Read more](https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html#method.rename)

<a href="https://docs.rs/object_store/latest/object_store/limit/struct.LimitStore.html#method.copy_if_not_exists" class="anchor">§</a>

#### fn <a href="https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html#tymethod.copy_if_not_exists" class="fn">copy_if_not_exists</a>\<'life0, 'life1, 'life2, 'async_trait\>( &'life0 self, from: &'life1 <a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>, to: &'life2 <a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://docs.rs/object_store/latest/object_store/type.Result.html" class="type" title="type object_store::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where Self: 'async_trait, 'life0: 'async_trait, 'life1: 'async_trait, 'life2: 'async_trait,

Copy an object from one path to another, only if destination is empty. [Read more](https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html#tymethod.copy_if_not_exists)

<a href="https://docs.rs/object_store/latest/object_store/limit/struct.LimitStore.html#method.rename_if_not_exists" class="anchor">§</a>

#### fn <a href="https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html#method.rename_if_not_exists" class="fn">rename_if_not_exists</a>\<'life0, 'life1, 'life2, 'async_trait\>( &'life0 self, from: &'life1 <a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>, to: &'life2 <a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://docs.rs/object_store/latest/object_store/type.Result.html" class="type" title="type object_store::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where Self: 'async_trait, 'life0: 'async_trait, 'life1: 'async_trait, 'life2: 'async_trait,

Move an object from one path to another in the same object store. [Read more](https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html#method.rename_if_not_exists)

## Auto Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/limit/struct.LimitStore.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/object_store/latest/object_store/limit/struct.LimitStore.html#blanket-implementations" class="anchor">§</a>
