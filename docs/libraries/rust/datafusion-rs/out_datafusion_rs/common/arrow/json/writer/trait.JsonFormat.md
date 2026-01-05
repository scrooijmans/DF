# Trait JsonFormat Copy item path

<a href="https://docs.rs/arrow-json/56.0.0/x86_64-unknown-linux-gnu/src/arrow_json/writer/mod.rs.html#119" class="src">Source</a>

``` rust
pub trait JsonFormat: Debug + Default {
    // Provided methods
    fn start_stream<W>(&self, _writer: &mut W) -> Result<(), ArrowError>
       where W: Write { ... }
    fn start_row<W>(
        &self,
        _writer: &mut W,
        _is_first_row: bool,
    ) -> Result<(), ArrowError>
       where W: Write { ... }
    fn end_row<W>(&self, _writer: &mut W) -> Result<(), ArrowError>
       where W: Write { ... }
    fn end_stream<W>(&self, _writer: &mut W) -> Result<(), ArrowError>
       where W: Write { ... }
}
```

Expand description

This trait defines how to format a sequence of JSON objects to a byte stream.

## Provided Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/writer/trait.JsonFormat.html#provided-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/writer/trait.JsonFormat.html#method.start_stream" class="fn">start_stream</a>\<W\>(&self, \_writer: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut W</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/error/enum.ArrowError.html" class="enum" title="enum datafusion::common::arrow::error::ArrowError">ArrowError</a>\>

where W: <a href="https://doc.rust-lang.org/nightly/std/io/trait.Write.html" class="trait" title="trait std::io::Write">Write</a>,

write any bytes needed at the start of the file to the writer

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/writer/trait.JsonFormat.html#method.start_row" class="fn">start_row</a>\<W\>( &self, \_writer: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut W</a>, \_is_first_row: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/error/enum.ArrowError.html" class="enum" title="enum datafusion::common::arrow::error::ArrowError">ArrowError</a>\>

where W: <a href="https://doc.rust-lang.org/nightly/std/io/trait.Write.html" class="trait" title="trait std::io::Write">Write</a>,

write any bytes needed for the start of each row

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/writer/trait.JsonFormat.html#method.end_row" class="fn">end_row</a>\<W\>(&self, \_writer: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut W</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/error/enum.ArrowError.html" class="enum" title="enum datafusion::common::arrow::error::ArrowError">ArrowError</a>\>

where W: <a href="https://doc.rust-lang.org/nightly/std/io/trait.Write.html" class="trait" title="trait std::io::Write">Write</a>,

write any bytes needed for the end of each row

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/writer/trait.JsonFormat.html#method.end_stream" class="fn">end_stream</a>\<W\>(&self, \_writer: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut W</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/error/enum.ArrowError.html" class="enum" title="enum datafusion::common::arrow::error::ArrowError">ArrowError</a>\>

where W: <a href="https://doc.rust-lang.org/nightly/std/io/trait.Write.html" class="trait" title="trait std::io::Write">Write</a>,

write any bytes needed for the start of each row

## Dyn Compatibility<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/writer/trait.JsonFormat.html#dyn-compatibility" class="anchor">§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementors<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/writer/trait.JsonFormat.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/writer/trait.JsonFormat.html#impl-JsonFormat-for-JsonArray" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/writer/trait.JsonFormat.html" class="trait" title="trait datafusion::common::arrow::json::writer::JsonFormat">JsonFormat</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/writer/struct.JsonArray.html" class="struct" title="struct datafusion::common::arrow::json::writer::JsonArray">JsonArray</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/writer/trait.JsonFormat.html#impl-JsonFormat-for-LineDelimited" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/writer/trait.JsonFormat.html" class="trait" title="trait datafusion::common::arrow::json::writer::JsonFormat">JsonFormat</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/writer/struct.LineDelimited.html" class="struct" title="struct datafusion::common::arrow::json::writer::LineDelimited">LineDelimited</a>
