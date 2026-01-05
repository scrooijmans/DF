# Struct Writer Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/blocking/write/writer.rs.html#24-27" class="src">Source</a>

``` rust
pub struct Writer { /* private fields */ }
```

Available on **crate feature `blocking`** only.

Expand description

BlockingWriter is designed to write data into given path in an blocking manner.

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Writer.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Writer.html#impl-Writer" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Writer.html" class="struct" title="struct opendal::blocking::Writer">Writer</a>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Writer.html#method.write" class="fn">write</a>(&mut self, bs: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html" class="struct" title="struct opendal::Buffer">Buffer</a>\>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

Write [`Buffer`](https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html "struct opendal::Buffer") into writer.

This operation will write all data in given buffer into writer.

###### <a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Writer.html#examples" class="doc-anchor">Â§</a>Examples

``` rust
use bytes::Bytes;
use opendal::blocking;
use opendal::blocking::Operator;
use opendal::Result;

fn test(op: blocking::Operator) -> Result<()> {
    let mut w = op.writer("hello.txt")?;
    // Buffer can be created from continues bytes.
    w.write("hello, world")?;
    // Buffer can also be created from non-continues bytes.
    w.write(vec![Bytes::from("hello,"), Bytes::from("world!")])?;

    // Make sure file has been written completely.
    w.close()?;
    Ok(())
}
```

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Writer.html#method.close" class="fn">close</a>(&mut self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/struct.Metadata.html" class="struct" title="struct opendal::Metadata">Metadata</a>\>

Close the writer and make sure all data have been committed.

###### <a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Writer.html#notes" class="doc-anchor">Â§</a>Notes

Close should only be called when the writer is not closed or aborted, otherwise an unexpected error could be returned.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Writer.html#method.into_std_write" class="fn">into_std_write</a>(self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.StdWriter.html" class="struct" title="struct opendal::blocking::StdWriter">StdWriter</a> <a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Writer.html#" class="tooltip" data-notable-ty="StdWriter">â“˜</a>

Convert writer into [`StdWriter`](https://opendal.apache.org/docs/rust/opendal/blocking/struct.StdWriter.html "struct opendal::blocking::StdWriter") which implements [`std::io::Write`](https://doc.rust-lang.org/nightly/std/io/trait.Write.html "trait std::io::Write"),

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Writer.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Writer.html#impl-Drop-for-Writer" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html" class="trait" title="trait core::ops::drop::Drop">Drop</a> for <a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Writer.html" class="struct" title="struct opendal::blocking::Writer">Writer</a>

Make sure the inner writer is dropped in async context.

<a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Writer.html#method.drop" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html#tymethod.drop" class="fn">drop</a>(&mut self)

Executes the destructor for this type. [Read more](https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html#tymethod.drop)

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Writer.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Writer.html#blanket-implementations" class="anchor">Â§</a>
