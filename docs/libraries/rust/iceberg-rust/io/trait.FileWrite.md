# Trait FileWrite Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/io/file_io.rs.html#369-379" class="src">Source</a>

``` rust
pub trait FileWrite:
    Send
    + Unpin
    + 'static {
    // Required methods
    fn write<'life0, 'async_trait>(
        &'life0 mut self,
        bs: Bytes,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + Send + 'async_trait>>
       where Self: 'async_trait,
             'life0: 'async_trait;
    fn close<'life0, 'async_trait>(
        &'life0 mut self,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + Send + 'async_trait>>
       where Self: 'async_trait,
             'life0: 'async_trait;
}
```

Expand description

Trait for writing file.

## <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/trait.FileWrite.html#todo" class="doc-anchor">§</a>TODO

It’s possible for us to remove the async_trait, but we need to figure out how to handle the object safety.

## Required Methods<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/trait.FileWrite.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/trait.FileWrite.html#tymethod.write" class="fn">write</a>\<'life0, 'async_trait\>( &'life0 mut self, bs: <a href="https://docs.rs/bytes/1.10.1/x86_64-unknown-linux-gnu/bytes/bytes/struct.Bytes.html" class="struct" title="struct bytes::bytes::Bytes">Bytes</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where Self: 'async_trait, 'life0: 'async_trait,

Write bytes to file.

TODO: we can support writing non-contiguous bytes in the future.

#### fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/trait.FileWrite.html#tymethod.close" class="fn">close</a>\<'life0, 'async_trait\>( &'life0 mut self, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where Self: 'async_trait, 'life0: 'async_trait,

Close file.

Calling close on closed file will generate an error.

## Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/trait.FileWrite.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/trait.FileWrite.html#impl-FileWrite-for-Box%3Cdyn+FileWrite%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/trait.FileWrite.html" class="trait" title="trait iceberg::io::FileWrite">FileWrite</a> for <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/trait.FileWrite.html" class="trait" title="trait iceberg::io::FileWrite">FileWrite</a>\>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/trait.FileWrite.html#method.write" class="anchor">§</a>

#### fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/trait.FileWrite.html#tymethod.write" class="fn">write</a>\<'life0, 'async_trait\>( &'life0 mut self, bs: <a href="https://docs.rs/bytes/1.10.1/x86_64-unknown-linux-gnu/bytes/bytes/struct.Bytes.html" class="struct" title="struct bytes::bytes::Bytes">Bytes</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where Self: 'async_trait, 'life0: 'async_trait,

Write bytes to file. [Read more](https://docs.rs/iceberg/0.7.0/iceberg/io/trait.FileWrite.html#tymethod.write)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/trait.FileWrite.html#method.close" class="anchor">§</a>

#### fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/trait.FileWrite.html#tymethod.close" class="fn">close</a>\<'life0, 'async_trait\>( &'life0 mut self, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where Self: 'async_trait, 'life0: 'async_trait,

Close file. [Read more](https://docs.rs/iceberg/0.7.0/iceberg/io/trait.FileWrite.html#tymethod.close)

## Implementations on Foreign Types<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/trait.FileWrite.html#foreign-impls" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/trait.FileWrite.html#impl-FileWrite-for-Box%3Cdyn+FileWrite%3E-1" class="anchor">§</a>

### impl <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/trait.FileWrite.html" class="trait" title="trait iceberg::io::FileWrite">FileWrite</a> for <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/trait.FileWrite.html" class="trait" title="trait iceberg::io::FileWrite">FileWrite</a>\>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/trait.FileWrite.html#method.write-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/trait.FileWrite.html#tymethod.write" class="fn">write</a>\<'life0, 'async_trait\>( &'life0 mut self, bs: <a href="https://docs.rs/bytes/1.10.1/x86_64-unknown-linux-gnu/bytes/bytes/struct.Bytes.html" class="struct" title="struct bytes::bytes::Bytes">Bytes</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where Self: 'async_trait, 'life0: 'async_trait,

<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/trait.FileWrite.html#method.close-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/trait.FileWrite.html#tymethod.close" class="fn">close</a>\<'life0, 'async_trait\>( &'life0 mut self, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where Self: 'async_trait, 'life0: 'async_trait,

<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/trait.FileWrite.html#impl-FileWrite-for-Writer" class="anchor">§</a>

### impl <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/trait.FileWrite.html" class="trait" title="trait iceberg::io::FileWrite">FileWrite</a> for <a href="https://docs.rs/opendal/0.54.0/x86_64-unknown-linux-gnu/opendal/types/write/writer/struct.Writer.html" class="struct" title="struct opendal::types::write::writer::Writer">Writer</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/trait.FileWrite.html#method.write-2" class="anchor">§</a>

#### fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/trait.FileWrite.html#tymethod.write" class="fn">write</a>\<'life0, 'async_trait\>( &'life0 mut self, bs: <a href="https://docs.rs/bytes/1.10.1/x86_64-unknown-linux-gnu/bytes/bytes/struct.Bytes.html" class="struct" title="struct bytes::bytes::Bytes">Bytes</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where Self: 'async_trait, 'life0: 'async_trait,

<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/trait.FileWrite.html#method.close-2" class="anchor">§</a>

#### fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/trait.FileWrite.html#tymethod.close" class="fn">close</a>\<'life0, 'async_trait\>( &'life0 mut self, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where Self: 'async_trait, 'life0: 'async_trait,

## Implementors<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/trait.FileWrite.html#implementors" class="anchor">§</a>
