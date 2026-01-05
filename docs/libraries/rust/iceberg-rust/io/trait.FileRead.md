# Trait FileRead Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/io/file_io.rs.html#299-304" class="src">Source</a>

``` rust
pub trait FileRead:
    Send
    + Sync
    + Unpin
    + 'static {
    // Required method
    fn read<'life0, 'async_trait>(
        &'life0 self,
        range: Range<u64>,
    ) -> Pin<Box<dyn Future<Output = Result<Bytes>> + Send + 'async_trait>>
       where Self: 'async_trait,
             'life0: 'async_trait;
}
```

Expand description

Trait for reading file.

## <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/trait.FileRead.html#todo" class="doc-anchor">§</a>TODO

It’s possible for us to remove the async_trait, but we need to figure out how to handle the object safety.

## Required Methods<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/trait.FileRead.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/trait.FileRead.html#tymethod.read" class="fn">read</a>\<'life0, 'async_trait\>( &'life0 self, range: <a href="https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html" class="struct" title="struct core::ops::range::Range">Range</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<<a href="https://docs.rs/bytes/1.10.1/x86_64-unknown-linux-gnu/bytes/bytes/struct.Bytes.html" class="struct" title="struct bytes::bytes::Bytes">Bytes</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where Self: 'async_trait, 'life0: 'async_trait,

Read file content with given range.

TODO: we can support reading non-contiguous bytes in the future.

## Implementations on Foreign Types<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/trait.FileRead.html#foreign-impls" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/trait.FileRead.html#impl-FileRead-for-Reader" class="anchor">§</a>

### impl <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/trait.FileRead.html" class="trait" title="trait iceberg::io::FileRead">FileRead</a> for <a href="https://docs.rs/opendal/0.54.0/x86_64-unknown-linux-gnu/opendal/types/read/reader/struct.Reader.html" class="struct" title="struct opendal::types::read::reader::Reader">Reader</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/trait.FileRead.html#method.read" class="anchor">§</a>

#### fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/trait.FileRead.html#tymethod.read" class="fn">read</a>\<'life0, 'async_trait\>( &'life0 self, range: <a href="https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html" class="struct" title="struct core::ops::range::Range">Range</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<<a href="https://docs.rs/bytes/1.10.1/x86_64-unknown-linux-gnu/bytes/bytes/struct.Bytes.html" class="struct" title="struct bytes::bytes::Bytes">Bytes</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where Self: 'async_trait, 'life0: 'async_trait,

## Implementors<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/trait.FileRead.html#implementors" class="anchor">§</a>
