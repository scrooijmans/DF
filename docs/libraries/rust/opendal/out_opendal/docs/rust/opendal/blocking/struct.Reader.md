# Struct Reader Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/blocking/read/reader.rs.html#31-34" class="src">Source</a>

``` rust
pub struct Reader { /* private fields */ }
```

Available on **crate feature `blocking`** only.

Expand description

BlockingReader is designed to read data from given path in an blocking manner.

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Reader.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Reader.html#impl-Reader" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Reader.html" class="struct" title="struct opendal::blocking::Reader">Reader</a>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Reader.html#method.read" class="fn">read</a>(&self, range: impl <a href="https://doc.rust-lang.org/nightly/core/ops/range/trait.RangeBounds.html" class="trait" title="trait core::ops::range::RangeBounds">RangeBounds</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html" class="struct" title="struct opendal::Buffer">Buffer</a>\>

Read give range from reader into [`Buffer`](https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html "struct opendal::Buffer").

This operation is zero-copy, which means it keeps the \[`bytes::Bytes`\] returned by underlying storage services without any extra copy or intensive memory allocations.

##### <a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Reader.html#notes" class="doc-anchor">Â§</a>Notes

- Buffer length smaller than range means we have reached the end of file.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Reader.html#method.read_into" class="fn">read_into</a>( &self, buf: &mut impl BufMut, range: impl <a href="https://doc.rust-lang.org/nightly/core/ops/range/trait.RangeBounds.html" class="trait" title="trait core::ops::range::RangeBounds">RangeBounds</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>, ) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>

This operation will copy and write bytes into given \[`BufMut`\]. Allocation happens while \[`BufMut`\] doesnâ€™t have enough space.

##### <a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Reader.html#notes-1" class="doc-anchor">Â§</a>Notes

- Returning length smaller than range means we have reached the end of file.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Reader.html#method.into_iterator" class="fn">into_iterator</a>( self, range: impl <a href="https://doc.rust-lang.org/nightly/core/ops/range/trait.RangeBounds.html" class="trait" title="trait core::ops::range::RangeBounds">RangeBounds</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>, ) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.BufferIterator.html" class="struct" title="struct opendal::blocking::BufferIterator">BufferIterator</a>\>

Create a buffer iterator to read specific range from given reader.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Reader.html#method.into_std_read" class="fn">into_std_read</a>(self, range: impl <a href="https://doc.rust-lang.org/nightly/core/ops/range/trait.RangeBounds.html" class="trait" title="trait core::ops::range::RangeBounds">RangeBounds</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.StdReader.html" class="struct" title="struct opendal::blocking::StdReader">StdReader</a>\>

Convert reader into [`StdReader`](https://opendal.apache.org/docs/rust/opendal/blocking/struct.StdReader.html "struct opendal::blocking::StdReader") which implements \[`futures::AsyncRead`\], \[`futures::AsyncSeek`\] and \[`futures::AsyncBufRead`\].

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Reader.html#method.into_bytes_iterator" class="fn">into_bytes_iterator</a>( self, range: impl <a href="https://doc.rust-lang.org/nightly/core/ops/range/trait.RangeBounds.html" class="trait" title="trait core::ops::range::RangeBounds">RangeBounds</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>, ) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.StdBytesIterator.html" class="struct" title="struct opendal::blocking::StdBytesIterator">StdBytesIterator</a>\>

Convert reader into [`StdBytesIterator`](https://opendal.apache.org/docs/rust/opendal/blocking/struct.StdBytesIterator.html "struct opendal::blocking::StdBytesIterator") which implements [`Iterator`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator").

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Reader.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Reader.html#impl-Clone-for-Reader" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Reader.html" class="struct" title="struct opendal::blocking::Reader">Reader</a>

<a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Reader.html#method.clone" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Reader.html" class="struct" title="struct opendal::blocking::Reader">Reader</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Reader.html#method.clone_from" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Reader.html#impl-Drop-for-Reader" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html" class="trait" title="trait core::ops::drop::Drop">Drop</a> for <a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Reader.html" class="struct" title="struct opendal::blocking::Reader">Reader</a>

Make sure the inner reader is dropped in async context.

<a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Reader.html#method.drop" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html#tymethod.drop" class="fn">drop</a>(&mut self)

Executes the destructor for this type. [Read more](https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html#tymethod.drop)

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Reader.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Reader.html#blanket-implementations" class="anchor">Â§</a>
