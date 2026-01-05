# Struct StdReader Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/blocking/read/std_reader.rs.html#35-38" class="src">Source</a>

``` rust
pub struct StdReader { /* private fields */ }
```

Available on **crate feature `blocking`** only.

Expand description

StdReader is the adapter of [`Read`](https://doc.rust-lang.org/nightly/std/io/trait.Read.html "trait std::io::Read"), [`Seek`](https://doc.rust-lang.org/nightly/std/io/trait.Seek.html "trait std::io::Seek") and [`BufRead`](https://doc.rust-lang.org/nightly/std/io/trait.BufRead.html "trait std::io::BufRead") for \[`BlockingReader`\]\[crate::BlockingReader\].

Users can use this adapter in cases where they need to use [`Read`](https://doc.rust-lang.org/nightly/std/io/trait.Read.html "trait std::io::Read") or [`BufRead`](https://doc.rust-lang.org/nightly/std/io/trait.BufRead.html "trait std::io::BufRead") trait.

StdReader also implements [`Send`](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") and [`Sync`](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync").

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.StdReader.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.StdReader.html#impl-BufRead-for-StdReader" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/std/io/trait.BufRead.html" class="trait" title="trait std::io::BufRead">BufRead</a> for <a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.StdReader.html" class="struct" title="struct opendal::blocking::StdReader">StdReader</a>

<a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.StdReader.html#method.fill_buf" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/std/io/trait.BufRead.html#tymethod.fill_buf" class="fn">fill_buf</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/std/io/error/type.Result.html" class="type" title="type std::io::error::Result">Result</a>\<&\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\>

Returns the contents of the internal buffer, filling it with more data, via `Read` methods, if empty. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.BufRead.html#tymethod.fill_buf)

<a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.StdReader.html#method.consume" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/std/io/trait.BufRead.html#tymethod.consume" class="fn">consume</a>(&mut self, amt: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

Marks the given `amount` of additional bytes from the internal buffer as having been read. Subsequent calls to `read` only return bytes that have not been marked as read. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.BufRead.html#tymethod.consume)

<a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.StdReader.html#method.has_data_left" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/std/io/trait.BufRead.html#method.has_data_left" class="fn">has_data_left</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, <a href="https://doc.rust-lang.org/nightly/std/io/error/struct.Error.html" class="struct" title="struct std::io::error::Error">Error</a>\>

ðŸ”¬This is a nightly-only experimental API. (`buf_read_has_data_left`)

Checks if there is any data left to be `read`. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.BufRead.html#method.has_data_left)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/std/io/mod.rs.html#2454" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.StdReader.html#method.read_until" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/std/io/trait.BufRead.html#method.read_until" class="fn">read_until</a>(&mut self, byte: <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>, buf: &mut <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, <a href="https://doc.rust-lang.org/nightly/std/io/error/struct.Error.html" class="struct" title="struct std::io::error::Error">Error</a>\>

Reads all bytes into `buf` until the delimiter `byte` or EOF is reached. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.BufRead.html#method.read_until)

1.83.0 Â· <a href="https://doc.rust-lang.org/nightly/src/std/io/mod.rs.html#2519" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.StdReader.html#method.skip_until" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/std/io/trait.BufRead.html#method.skip_until" class="fn">skip_until</a>(&mut self, byte: <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, <a href="https://doc.rust-lang.org/nightly/std/io/error/struct.Error.html" class="struct" title="struct std::io::error::Error">Error</a>\>

Skips all bytes until the delimiter `byte` or EOF is reached. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.BufRead.html#method.skip_until)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/std/io/mod.rs.html#2587" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.StdReader.html#method.read_line" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/std/io/trait.BufRead.html#method.read_line" class="fn">read_line</a>(&mut self, buf: &mut <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, <a href="https://doc.rust-lang.org/nightly/std/io/error/struct.Error.html" class="struct" title="struct std::io::error::Error">Error</a>\>

Reads all bytes until a newline (the `0xA` byte) is reached, and append them to the provided `String` buffer. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.BufRead.html#method.read_line)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/std/io/mod.rs.html#2625-2627" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.StdReader.html#method.split" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/std/io/trait.BufRead.html#method.split" class="fn">split</a>(self, byte: <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/io/struct.Split.html" class="struct" title="struct std::io::Split">Split</a>\<Self\>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Returns an iterator over the contents of this reader split on the byte `byte`. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.BufRead.html#method.split)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/std/io/mod.rs.html#2662-2664" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.StdReader.html#method.lines" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/std/io/trait.BufRead.html#method.lines" class="fn">lines</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/std/io/struct.Lines.html" class="struct" title="struct std::io::Lines">Lines</a>\<Self\>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Returns an iterator over the lines of this reader. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.BufRead.html#method.lines)

<a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.StdReader.html#impl-Drop-for-StdReader" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html" class="trait" title="trait core::ops::drop::Drop">Drop</a> for <a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.StdReader.html" class="struct" title="struct opendal::blocking::StdReader">StdReader</a>

Make sure the inner reader is dropped in async context.

<a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.StdReader.html#method.drop" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html#tymethod.drop" class="fn">drop</a>(&mut self)

Executes the destructor for this type. [Read more](https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html#tymethod.drop)

<a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.StdReader.html#impl-Read-for-StdReader" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/std/io/trait.Read.html" class="trait" title="trait std::io::Read">Read</a> for <a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.StdReader.html" class="struct" title="struct opendal::blocking::StdReader">StdReader</a>

<a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.StdReader.html#method.read" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/std/io/trait.Read.html#tymethod.read" class="fn">read</a>(&mut self, buf: &mut \[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]) -\> <a href="https://doc.rust-lang.org/nightly/std/io/error/type.Result.html" class="type" title="type std::io::error::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>

Pull some bytes from this source into the specified buffer, returning how many bytes were read. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Read.html#tymethod.read)

1.36.0 Â· <a href="https://doc.rust-lang.org/nightly/src/std/io/mod.rs.html#825" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.StdReader.html#method.read_vectored" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/std/io/trait.Read.html#method.read_vectored" class="fn">read_vectored</a>(&mut self, bufs: &mut \[<a href="https://doc.rust-lang.org/nightly/std/io/struct.IoSliceMut.html" class="struct" title="struct std::io::IoSliceMut">IoSliceMut</a>\<'\_\>\]) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, <a href="https://doc.rust-lang.org/nightly/std/io/error/struct.Error.html" class="struct" title="struct std::io::error::Error">Error</a>\>

Like `read`, except that it reads into a slice of buffers. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Read.html#method.read_vectored)

<a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.StdReader.html#method.is_read_vectored" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/std/io/trait.Read.html#method.is_read_vectored" class="fn">is_read_vectored</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

ðŸ”¬This is a nightly-only experimental API. (`can_vector`)

Determines if this `Read`er has an efficient `read_vectored` implementation. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Read.html#method.is_read_vectored)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/std/io/mod.rs.html#935" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.StdReader.html#method.read_to_end" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/std/io/trait.Read.html#method.read_to_end" class="fn">read_to_end</a>(&mut self, buf: &mut <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, <a href="https://doc.rust-lang.org/nightly/std/io/error/struct.Error.html" class="struct" title="struct std::io::error::Error">Error</a>\>

Reads all bytes until EOF in this source, placing them into `buf`. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Read.html#method.read_to_end)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/std/io/mod.rs.html#991" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.StdReader.html#method.read_to_string" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/std/io/trait.Read.html#method.read_to_string" class="fn">read_to_string</a>(&mut self, buf: &mut <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, <a href="https://doc.rust-lang.org/nightly/std/io/error/struct.Error.html" class="struct" title="struct std::io::error::Error">Error</a>\>

Reads all bytes until EOF in this source, appending them to `buf`. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Read.html#method.read_to_string)

1.6.0 Â· <a href="https://doc.rust-lang.org/nightly/src/std/io/mod.rs.html#1044" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.StdReader.html#method.read_exact" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/std/io/trait.Read.html#method.read_exact" class="fn">read_exact</a>(&mut self, buf: &mut \[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/std/io/error/struct.Error.html" class="struct" title="struct std::io::error::Error">Error</a>\>

Reads the exact number of bytes required to fill `buf`. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Read.html#method.read_exact)

<a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.StdReader.html#method.read_buf" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/std/io/trait.Read.html#method.read_buf" class="fn">read_buf</a>(&mut self, buf: <a href="https://doc.rust-lang.org/nightly/core/io/borrowed_buf/struct.BorrowedCursor.html" class="struct" title="struct core::io::borrowed_buf::BorrowedCursor">BorrowedCursor</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/std/io/error/struct.Error.html" class="struct" title="struct std::io::error::Error">Error</a>\>

ðŸ”¬This is a nightly-only experimental API. (`read_buf`)

Pull some bytes from this source into the specified buffer. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Read.html#method.read_buf)

<a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.StdReader.html#method.read_buf_exact" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/std/io/trait.Read.html#method.read_buf_exact" class="fn">read_buf_exact</a>(&mut self, cursor: <a href="https://doc.rust-lang.org/nightly/core/io/borrowed_buf/struct.BorrowedCursor.html" class="struct" title="struct core::io::borrowed_buf::BorrowedCursor">BorrowedCursor</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/std/io/error/struct.Error.html" class="struct" title="struct std::io::error::Error">Error</a>\>

ðŸ”¬This is a nightly-only experimental API. (`read_buf`)

Reads the exact number of bytes required to fill `cursor`. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Read.html#method.read_buf_exact)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/std/io/mod.rs.html#1119-1121" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.StdReader.html#method.by_ref" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/std/io/trait.Read.html#method.by_ref" class="fn">by_ref</a>(&mut self) -\> &mut Self

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Creates a â€œby referenceâ€? adapter for this instance of `Read`. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Read.html#method.by_ref)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/std/io/mod.rs.html#1162-1164" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.StdReader.html#method.bytes" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/std/io/trait.Read.html#method.bytes" class="fn">bytes</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/std/io/struct.Bytes.html" class="struct" title="struct std::io::Bytes">Bytes</a>\<Self\>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Transforms this `Read` instance to an [`Iterator`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator") over its bytes. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Read.html#method.bytes)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/std/io/mod.rs.html#1200-1202" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.StdReader.html#method.chain" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/std/io/trait.Read.html#method.chain" class="fn">chain</a>\<R\>(self, next: R) -\> <a href="https://doc.rust-lang.org/nightly/std/io/struct.Chain.html" class="struct" title="struct std::io::Chain">Chain</a>\<Self, R\>

where R: <a href="https://doc.rust-lang.org/nightly/std/io/trait.Read.html" class="trait" title="trait std::io::Read">Read</a>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Creates an adapter which will chain this stream with another. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Read.html#method.chain)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/std/io/mod.rs.html#1239-1241" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.StdReader.html#method.take" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/std/io/trait.Read.html#method.take" class="fn">take</a>(self, limit: <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/io/struct.Take.html" class="struct" title="struct std::io::Take">Take</a>\<Self\>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Creates an adapter which will read at most `limit` bytes from it. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Read.html#method.take)

<a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.StdReader.html#impl-Seek-for-StdReader" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/std/io/trait.Seek.html" class="trait" title="trait std::io::Seek">Seek</a> for <a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.StdReader.html" class="struct" title="struct opendal::blocking::StdReader">StdReader</a>

<a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.StdReader.html#method.seek" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/std/io/trait.Seek.html#tymethod.seek" class="fn">seek</a>(&mut self, pos: <a href="https://doc.rust-lang.org/nightly/std/io/enum.SeekFrom.html" class="enum" title="enum std::io::SeekFrom">SeekFrom</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/io/error/type.Result.html" class="type" title="type std::io::error::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>

Seek to an offset, in bytes, in a stream. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Seek.html#tymethod.seek)

1.55.0 Â· <a href="https://doc.rust-lang.org/nightly/src/std/io/mod.rs.html#2064" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.StdReader.html#method.rewind" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/std/io/trait.Seek.html#method.rewind" class="fn">rewind</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/std/io/error/struct.Error.html" class="struct" title="struct std::io::error::Error">Error</a>\>

Rewind to the beginning of a stream. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Seek.html#method.rewind)

<a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.StdReader.html#method.stream_len" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/std/io/trait.Seek.html#method.stream_len" class="fn">stream_len</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>, <a href="https://doc.rust-lang.org/nightly/std/io/error/struct.Error.html" class="struct" title="struct std::io::error::Error">Error</a>\>

ðŸ”¬This is a nightly-only experimental API. (`seek_stream_len`)

Returns the length of this stream (in bytes). [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Seek.html#method.stream_len)

1.51.0 Â· <a href="https://doc.rust-lang.org/nightly/src/std/io/mod.rs.html#2132" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.StdReader.html#method.stream_position" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/std/io/trait.Seek.html#method.stream_position" class="fn">stream_position</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>, <a href="https://doc.rust-lang.org/nightly/std/io/error/struct.Error.html" class="struct" title="struct std::io::error::Error">Error</a>\>

Returns the current seek position from the start of the stream. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Seek.html#method.stream_position)

1.80.0 Â· <a href="https://doc.rust-lang.org/nightly/src/std/io/mod.rs.html#2160" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.StdReader.html#method.seek_relative" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/std/io/trait.Seek.html#method.seek_relative" class="fn">seek_relative</a>(&mut self, offset: <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/std/io/error/struct.Error.html" class="struct" title="struct std::io::error::Error">Error</a>\>

Seeks relative to the current position. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Seek.html#method.seek_relative)

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.StdReader.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.StdReader.html#blanket-implementations" class="anchor">Â§</a>
