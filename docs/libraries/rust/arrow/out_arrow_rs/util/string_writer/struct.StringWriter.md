# Struct StringWriter Copy item path

<a href="https://docs.rs/arrow/latest/src/arrow/util/string_writer.rs.html#73-75" class="src">Source</a>

``` rust
pub struct StringWriter { /* private fields */ }
```

Expand description

A writer that allows writing to a `String` like an `std::io::Write` object.

## Implementations<a href="https://docs.rs/arrow/latest/arrow/util/string_writer/struct.StringWriter.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/util/string_writer/struct.StringWriter.html#impl-StringWriter" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/util/string_writer/struct.StringWriter.html" class="struct" title="struct arrow::util::string_writer::StringWriter">StringWriter</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/util/string_writer/struct.StringWriter.html#method.new" class="fn">new</a>() -\> Self

Create a new `StringWriter`

## Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/util/string_writer/struct.StringWriter.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/util/string_writer/struct.StringWriter.html#impl-Debug-for-StringWriter" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/arrow/latest/arrow/util/string_writer/struct.StringWriter.html" class="struct" title="struct arrow::util::string_writer::StringWriter">StringWriter</a>

<a href="https://docs.rs/arrow/latest/arrow/util/string_writer/struct.StringWriter.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/arrow/latest/arrow/util/string_writer/struct.StringWriter.html#impl-Default-for-StringWriter" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/arrow/latest/arrow/util/string_writer/struct.StringWriter.html" class="struct" title="struct arrow::util::string_writer::StringWriter">StringWriter</a>

<a href="https://docs.rs/arrow/latest/arrow/util/string_writer/struct.StringWriter.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/arrow/latest/arrow/util/string_writer/struct.StringWriter.html" class="struct" title="struct arrow::util::string_writer::StringWriter">StringWriter</a> <a href="https://docs.rs/arrow/latest/arrow/util/string_writer/struct.StringWriter.html#" class="tooltip" data-notable-ty="StringWriter">ⓘ</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/arrow/latest/arrow/util/string_writer/struct.StringWriter.html#impl-Display-for-StringWriter" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> for <a href="https://docs.rs/arrow/latest/arrow/util/string_writer/struct.StringWriter.html" class="struct" title="struct arrow::util::string_writer::StringWriter">StringWriter</a>

<a href="https://docs.rs/arrow/latest/arrow/util/string_writer/struct.StringWriter.html#method.fmt-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

<a href="https://docs.rs/arrow/latest/arrow/util/string_writer/struct.StringWriter.html#impl-Write-for-StringWriter" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/std/io/trait.Write.html" class="trait" title="trait std::io::Write">Write</a> for <a href="https://docs.rs/arrow/latest/arrow/util/string_writer/struct.StringWriter.html" class="struct" title="struct arrow::util::string_writer::StringWriter">StringWriter</a>

<a href="https://docs.rs/arrow/latest/arrow/util/string_writer/struct.StringWriter.html#method.write" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/std/io/trait.Write.html#tymethod.write" class="fn">write</a>(&mut self, buf: &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]) -\> <a href="https://doc.rust-lang.org/nightly/std/io/error/type.Result.html" class="type" title="type std::io::error::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>

Writes a buffer into this writer, returning how many bytes were written. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Write.html#tymethod.write)

<a href="https://docs.rs/arrow/latest/arrow/util/string_writer/struct.StringWriter.html#method.flush" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/std/io/trait.Write.html#tymethod.flush" class="fn">flush</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/std/io/error/type.Result.html" class="type" title="type std::io::error::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

Flushes this output stream, ensuring that all intermediately buffered contents reach their destination. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Write.html#tymethod.flush)

1.36.0 · <a href="https://doc.rust-lang.org/nightly/src/std/io/mod.rs.html#1758" class="src">Source</a><a href="https://docs.rs/arrow/latest/arrow/util/string_writer/struct.StringWriter.html#method.write_vectored" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/std/io/trait.Write.html#method.write_vectored" class="fn">write_vectored</a>(&mut self, bufs: &\[<a href="https://doc.rust-lang.org/nightly/std/io/struct.IoSlice.html" class="struct" title="struct std::io::IoSlice">IoSlice</a>\<'\_\>\]) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, <a href="https://doc.rust-lang.org/nightly/std/io/error/struct.Error.html" class="struct" title="struct std::io::error::Error">Error</a>\>

Like [`write`](https://doc.rust-lang.org/nightly/std/io/trait.Write.html#tymethod.write "method std::io::Write::write"), except that it writes from a slice of buffers. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Write.html#method.write_vectored)

<a href="https://docs.rs/arrow/latest/arrow/util/string_writer/struct.StringWriter.html#method.is_write_vectored" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/std/io/trait.Write.html#method.is_write_vectored" class="fn">is_write_vectored</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

🔬This is a nightly-only experimental API. (`can_vector`)

Determines if this `Write`r has an efficient [`write_vectored`](https://doc.rust-lang.org/nightly/std/io/trait.Write.html#method.write_vectored "method std::io::Write::write_vectored") implementation. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Write.html#method.is_write_vectored)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/std/io/mod.rs.html#1835" class="src">Source</a><a href="https://docs.rs/arrow/latest/arrow/util/string_writer/struct.StringWriter.html#method.write_all" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/std/io/trait.Write.html#method.write_all" class="fn">write_all</a>(&mut self, buf: &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/std/io/error/struct.Error.html" class="struct" title="struct std::io::error::Error">Error</a>\>

Attempts to write an entire buffer into this writer. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Write.html#method.write_all)

<a href="https://docs.rs/arrow/latest/arrow/util/string_writer/struct.StringWriter.html#method.write_all_vectored" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/std/io/trait.Write.html#method.write_all_vectored" class="fn">write_all_vectored</a>(&mut self, bufs: &mut \[<a href="https://doc.rust-lang.org/nightly/std/io/struct.IoSlice.html" class="struct" title="struct std::io::IoSlice">IoSlice</a>\<'\_\>\]) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/std/io/error/struct.Error.html" class="struct" title="struct std::io::error::Error">Error</a>\>

🔬This is a nightly-only experimental API. (`write_all_vectored`)

Attempts to write multiple buffers into this writer. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Write.html#method.write_all_vectored)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/std/io/mod.rs.html#1950" class="src">Source</a><a href="https://docs.rs/arrow/latest/arrow/util/string_writer/struct.StringWriter.html#method.write_fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/std/io/trait.Write.html#method.write_fmt" class="fn">write_fmt</a>(&mut self, args: <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Arguments.html" class="struct" title="struct core::fmt::Arguments">Arguments</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/std/io/error/struct.Error.html" class="struct" title="struct std::io::error::Error">Error</a>\>

Writes a formatted string into this writer, returning any error encountered. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Write.html#method.write_fmt)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/std/io/mod.rs.html#1980-1982" class="src">Source</a><a href="https://docs.rs/arrow/latest/arrow/util/string_writer/struct.StringWriter.html#method.by_ref" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/std/io/trait.Write.html#method.by_ref" class="fn">by_ref</a>(&mut self) -\> &mut Self

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Creates a “by reference” adapter for this instance of `Write`. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Write.html#method.by_ref)

## Auto Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/util/string_writer/struct.StringWriter.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/arrow/latest/arrow/util/string_writer/struct.StringWriter.html#blanket-implementations" class="anchor">§</a>
