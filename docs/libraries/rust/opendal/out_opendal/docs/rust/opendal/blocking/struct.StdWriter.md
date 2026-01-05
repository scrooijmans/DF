# Struct StdWriter Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/blocking/write/std_writer.rs.html#32-35" class="src">Source</a>

``` rust
pub struct StdWriter { /* private fields */ }
```

Available on **crate feature `blocking`** only.

Expand description

StdWriter is the adapter of [`std::io::Write`](https://doc.rust-lang.org/nightly/std/io/trait.Write.html "trait std::io::Write") for \[`BlockingWriter`\].

Users can use this adapter in cases where they need to use [`std::io::Write`](https://doc.rust-lang.org/nightly/std/io/trait.Write.html "trait std::io::Write") related trait.

## <a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.StdWriter.html#notes" class="doc-anchor">Â§</a>Notes

Files are automatically closed when they go out of scope. Errors detected on closing are ignored by the implementation of Drop. Use the method `close` if these errors must be manually handled.

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.StdWriter.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.StdWriter.html#impl-StdWriter" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.StdWriter.html" class="struct" title="struct opendal::blocking::StdWriter">StdWriter</a>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.StdWriter.html#method.close" class="fn">close</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/std/io/error/type.Result.html" class="type" title="type std::io::error::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

Close the internal writer and make sure all data have been stored.

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.StdWriter.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.StdWriter.html#impl-Drop-for-StdWriter" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html" class="trait" title="trait core::ops::drop::Drop">Drop</a> for <a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.StdWriter.html" class="struct" title="struct opendal::blocking::StdWriter">StdWriter</a>

Make sure the inner writer is dropped in async context.

<a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.StdWriter.html#method.drop" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html#tymethod.drop" class="fn">drop</a>(&mut self)

Executes the destructor for this type. [Read more](https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html#tymethod.drop)

<a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.StdWriter.html#impl-Write-for-StdWriter" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/std/io/trait.Write.html" class="trait" title="trait std::io::Write">Write</a> for <a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.StdWriter.html" class="struct" title="struct opendal::blocking::StdWriter">StdWriter</a>

<a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.StdWriter.html#method.write" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/std/io/trait.Write.html#tymethod.write" class="fn">write</a>(&mut self, buf: &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]) -\> <a href="https://doc.rust-lang.org/nightly/std/io/error/type.Result.html" class="type" title="type std::io::error::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>

Writes a buffer into this writer, returning how many bytes were written. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Write.html#tymethod.write)

<a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.StdWriter.html#method.flush" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/std/io/trait.Write.html#tymethod.flush" class="fn">flush</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/std/io/error/type.Result.html" class="type" title="type std::io::error::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

Flushes this output stream, ensuring that all intermediately buffered contents reach their destination. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Write.html#tymethod.flush)

1.36.0 Â· <a href="https://doc.rust-lang.org/nightly/src/std/io/mod.rs.html#1758" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.StdWriter.html#method.write_vectored" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/std/io/trait.Write.html#method.write_vectored" class="fn">write_vectored</a>(&mut self, bufs: &\[<a href="https://doc.rust-lang.org/nightly/std/io/struct.IoSlice.html" class="struct" title="struct std::io::IoSlice">IoSlice</a>\<'\_\>\]) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, <a href="https://doc.rust-lang.org/nightly/std/io/error/struct.Error.html" class="struct" title="struct std::io::error::Error">Error</a>\>

Like [`write`](https://doc.rust-lang.org/nightly/std/io/trait.Write.html#tymethod.write "method std::io::Write::write"), except that it writes from a slice of buffers. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Write.html#method.write_vectored)

<a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.StdWriter.html#method.is_write_vectored" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/std/io/trait.Write.html#method.is_write_vectored" class="fn">is_write_vectored</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

ðŸ”¬This is a nightly-only experimental API. (`can_vector`)

Determines if this `Write`r has an efficient [`write_vectored`](https://doc.rust-lang.org/nightly/std/io/trait.Write.html#method.write_vectored "method std::io::Write::write_vectored") implementation. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Write.html#method.is_write_vectored)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/std/io/mod.rs.html#1835" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.StdWriter.html#method.write_all" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/std/io/trait.Write.html#method.write_all" class="fn">write_all</a>(&mut self, buf: &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/std/io/error/struct.Error.html" class="struct" title="struct std::io::error::Error">Error</a>\>

Attempts to write an entire buffer into this writer. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Write.html#method.write_all)

<a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.StdWriter.html#method.write_all_vectored" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/std/io/trait.Write.html#method.write_all_vectored" class="fn">write_all_vectored</a>(&mut self, bufs: &mut \[<a href="https://doc.rust-lang.org/nightly/std/io/struct.IoSlice.html" class="struct" title="struct std::io::IoSlice">IoSlice</a>\<'\_\>\]) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/std/io/error/struct.Error.html" class="struct" title="struct std::io::error::Error">Error</a>\>

ðŸ”¬This is a nightly-only experimental API. (`write_all_vectored`)

Attempts to write multiple buffers into this writer. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Write.html#method.write_all_vectored)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/std/io/mod.rs.html#1950" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.StdWriter.html#method.write_fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/std/io/trait.Write.html#method.write_fmt" class="fn">write_fmt</a>(&mut self, args: <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Arguments.html" class="struct" title="struct core::fmt::Arguments">Arguments</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/std/io/error/struct.Error.html" class="struct" title="struct std::io::error::Error">Error</a>\>

Writes a formatted string into this writer, returning any error encountered. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Write.html#method.write_fmt)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/std/io/mod.rs.html#1980-1982" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.StdWriter.html#method.by_ref" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/std/io/trait.Write.html#method.by_ref" class="fn">by_ref</a>(&mut self) -\> &mut Self

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Creates a â€œby referenceâ€? adapter for this instance of `Write`. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Write.html#method.by_ref)

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.StdWriter.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.StdWriter.html#blanket-implementations" class="anchor">Â§</a>
