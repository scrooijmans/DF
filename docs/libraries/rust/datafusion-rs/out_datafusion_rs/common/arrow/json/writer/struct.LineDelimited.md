# Struct LineDelimited Copy item path

<a href="https://docs.rs/arrow-json/56.0.0/x86_64-unknown-linux-gnu/src/arrow_json/writer/mod.rs.html#154" class="src">Source</a>

``` rust
pub struct LineDelimited {}
```

Expand description

Produces JSON output with one record per line.

For example:

``` json
{"foo":1}
{"bar":1}
```

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/writer/struct.LineDelimited.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/writer/struct.LineDelimited.html#impl-Debug-for-LineDelimited" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/writer/struct.LineDelimited.html" class="struct" title="struct datafusion::common::arrow::json::writer::LineDelimited">LineDelimited</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/writer/struct.LineDelimited.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/writer/struct.LineDelimited.html#impl-Default-for-LineDelimited" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/writer/struct.LineDelimited.html" class="struct" title="struct datafusion::common::arrow::json::writer::LineDelimited">LineDelimited</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/writer/struct.LineDelimited.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/writer/struct.LineDelimited.html" class="struct" title="struct datafusion::common::arrow::json::writer::LineDelimited">LineDelimited</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/writer/struct.LineDelimited.html#impl-JsonFormat-for-LineDelimited" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/writer/trait.JsonFormat.html" class="trait" title="trait datafusion::common::arrow::json::writer::JsonFormat">JsonFormat</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/writer/struct.LineDelimited.html" class="struct" title="struct datafusion::common::arrow::json::writer::LineDelimited">LineDelimited</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/writer/struct.LineDelimited.html#method.end_row" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/writer/trait.JsonFormat.html#method.end_row" class="fn">end_row</a>\<W\>(&self, writer: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut W</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/error/enum.ArrowError.html" class="enum" title="enum datafusion::common::arrow::error::ArrowError">ArrowError</a>\>

where W: <a href="https://doc.rust-lang.org/nightly/std/io/trait.Write.html" class="trait" title="trait std::io::Write">Write</a>,

write any bytes needed for the end of each row

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/writer/struct.LineDelimited.html#method.start_stream" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/writer/trait.JsonFormat.html#method.start_stream" class="fn">start_stream</a>\<W\>(&self, \_writer: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut W</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/error/enum.ArrowError.html" class="enum" title="enum datafusion::common::arrow::error::ArrowError">ArrowError</a>\>

where W: <a href="https://doc.rust-lang.org/nightly/std/io/trait.Write.html" class="trait" title="trait std::io::Write">Write</a>,

write any bytes needed at the start of the file to the writer

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/writer/struct.LineDelimited.html#method.start_row" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/writer/trait.JsonFormat.html#method.start_row" class="fn">start_row</a>\<W\>( &self, \_writer: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut W</a>, \_is_first_row: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/error/enum.ArrowError.html" class="enum" title="enum datafusion::common::arrow::error::ArrowError">ArrowError</a>\>

where W: <a href="https://doc.rust-lang.org/nightly/std/io/trait.Write.html" class="trait" title="trait std::io::Write">Write</a>,

write any bytes needed for the start of each row

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/writer/struct.LineDelimited.html#method.end_stream" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/writer/trait.JsonFormat.html#method.end_stream" class="fn">end_stream</a>\<W\>(&self, \_writer: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut W</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/error/enum.ArrowError.html" class="enum" title="enum datafusion::common::arrow::error::ArrowError">ArrowError</a>\>

where W: <a href="https://doc.rust-lang.org/nightly/std/io/trait.Write.html" class="trait" title="trait std::io::Write">Write</a>,

write any bytes needed for the start of each row

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/writer/struct.LineDelimited.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/writer/struct.LineDelimited.html#blanket-implementations" class="anchor">§</a>
