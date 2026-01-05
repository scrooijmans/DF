# Module blocking Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/blocking/mod.rs.html#18-33" class="src">Source</a>

Available on **crate feature `blocking`** only.

Expand description

blocking module provides blocking APIs for OpenDAL.

## Structs<a href="https://opendal.apache.org/docs/rust/opendal/blocking/index.html#structs" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.BufferIterator.html" class="struct" title="struct opendal::blocking::BufferIterator">BufferIterator</a>  
BufferIterator is an iterator of buffers.

<a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Deleter.html" class="struct" title="struct opendal::blocking::Deleter">Deleter</a>  
BlockingDeleter is designed to continuously remove content from storage.

<a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Lister.html" class="struct" title="struct opendal::blocking::Lister">Lister</a>  
BlockingLister is designed to list entries at given path in a blocking manner.

<a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Operator.html" class="struct" title="struct opendal::blocking::Operator">Operator</a>  
Use OpenDAL in blocking context.

<a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Reader.html" class="struct" title="struct opendal::blocking::Reader">Reader</a>  
BlockingReader is designed to read data from given path in an blocking manner.

<a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.StdBytesIterator.html" class="struct" title="struct opendal::blocking::StdBytesIterator">StdBytesIterator</a>  
StdIterator is the adapter of [`Iterator`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator") for \[`BlockingReader`\]\[crate::BlockingReader\].

<a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.StdReader.html" class="struct" title="struct opendal::blocking::StdReader">StdReader</a>  
StdReader is the adapter of [`Read`](https://doc.rust-lang.org/nightly/std/io/trait.Read.html "trait std::io::Read"), [`Seek`](https://doc.rust-lang.org/nightly/std/io/trait.Seek.html "trait std::io::Seek") and [`BufRead`](https://doc.rust-lang.org/nightly/std/io/trait.BufRead.html "trait std::io::BufRead") for \[`BlockingReader`\]\[crate::BlockingReader\].

<a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.StdWriter.html" class="struct" title="struct opendal::blocking::StdWriter">StdWriter</a>  
StdWriter is the adapter of [`std::io::Write`](https://doc.rust-lang.org/nightly/std/io/trait.Write.html "trait std::io::Write") for \[`BlockingWriter`\].

<a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Writer.html" class="struct" title="struct opendal::blocking::Writer">Writer</a>  
BlockingWriter is designed to write data into given path in an blocking manner.
