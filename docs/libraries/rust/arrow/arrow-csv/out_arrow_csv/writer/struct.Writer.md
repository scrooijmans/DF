# Struct Writer Copy item path

<a href="https://arrow.apache.org/rust/src/arrow_csv/writer.rs.html#77-96" class="src">Source</a>

``` rust
pub struct Writer<W: Write> {
    writer: Writer<W>,
    has_headers: bool,
    date_format: Option<String>,
    datetime_format: Option<String>,
    timestamp_format: Option<String>,
    timestamp_tz_format: Option<String>,
    time_format: Option<String>,
    beginning: bool,
    null_value: Option<String>,
}
```

Expand description

A CSV writer

## Fields<a href="https://arrow.apache.org/rust/arrow_csv/writer/struct.Writer.html#fields" class="anchor">Â§</a>

<a href="https://arrow.apache.org/rust/arrow_csv/writer/struct.Writer.html#structfield.writer" class="anchor field">Â§</a>`writer: Writer<W>`

The object to write to

<a href="https://arrow.apache.org/rust/arrow_csv/writer/struct.Writer.html#structfield.has_headers" class="anchor field">Â§</a>`has_headers: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Whether file should be written with headers, defaults to `true`

<a href="https://arrow.apache.org/rust/arrow_csv/writer/struct.Writer.html#structfield.date_format" class="anchor field">Â§</a>`date_format: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

The date format for date arrays, defaults to RFC3339

<a href="https://arrow.apache.org/rust/arrow_csv/writer/struct.Writer.html#structfield.datetime_format" class="anchor field">Â§</a>`datetime_format: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

The datetime format for datetime arrays, defaults to RFC3339

<a href="https://arrow.apache.org/rust/arrow_csv/writer/struct.Writer.html#structfield.timestamp_format" class="anchor field">Â§</a>`timestamp_format: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

The timestamp format for timestamp arrays, defaults to RFC3339

<a href="https://arrow.apache.org/rust/arrow_csv/writer/struct.Writer.html#structfield.timestamp_tz_format" class="anchor field">Â§</a>`timestamp_tz_format: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

The timestamp format for timestamp (with timezone) arrays, defaults to RFC3339

<a href="https://arrow.apache.org/rust/arrow_csv/writer/struct.Writer.html#structfield.time_format" class="anchor field">Â§</a>`time_format: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

The time format for time arrays, defaults to RFC3339

<a href="https://arrow.apache.org/rust/arrow_csv/writer/struct.Writer.html#structfield.beginning" class="anchor field">Â§</a>`beginning: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Is the beginning-of-writer

<a href="https://arrow.apache.org/rust/arrow_csv/writer/struct.Writer.html#structfield.null_value" class="anchor field">Â§</a>`null_value: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

The value to represent null entries, defaults to [`DEFAULT_NULL_VALUE`](https://arrow.apache.org/rust/arrow_csv/writer/constant.DEFAULT_NULL_VALUE.html "constant arrow_csv::writer::DEFAULT_NULL_VALUE")

## Implementations<a href="https://arrow.apache.org/rust/arrow_csv/writer/struct.Writer.html#implementations" class="anchor">Â§</a>

<a href="https://arrow.apache.org/rust/arrow_csv/writer/struct.Writer.html#impl-Writer%3CW%3E" class="anchor">Â§</a>

### impl\<W: <a href="https://doc.rust-lang.org/nightly/std/io/trait.Write.html" class="trait" title="trait std::io::Write">Write</a>\> <a href="https://arrow.apache.org/rust/arrow_csv/writer/struct.Writer.html" class="struct" title="struct arrow_csv::writer::Writer">Writer</a>\<W\>

#### pub fn <a href="https://arrow.apache.org/rust/arrow_csv/writer/struct.Writer.html#method.new" class="fn">new</a>(writer: W) -\> Self

Create a new CsvWriter from a writable object, with default options

#### pub fn <a href="https://arrow.apache.org/rust/arrow_csv/writer/struct.Writer.html#method.write" class="fn">write</a>(&mut self, batch: &RecordBatch) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, ArrowError\>

Write a RecordBatch to a writable object

#### pub fn <a href="https://arrow.apache.org/rust/arrow_csv/writer/struct.Writer.html#method.into_inner" class="fn">into_inner</a>(self) -\> W

Unwraps this `Writer<W>`, returning the underlying writer.

## Trait Implementations<a href="https://arrow.apache.org/rust/arrow_csv/writer/struct.Writer.html#trait-implementations" class="anchor">Â§</a>

<a href="https://arrow.apache.org/rust/arrow_csv/writer/struct.Writer.html#impl-Debug-for-Writer%3CW%3E" class="anchor">Â§</a>

### impl\<W: <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> + <a href="https://doc.rust-lang.org/nightly/std/io/trait.Write.html" class="trait" title="trait std::io::Write">Write</a>\> <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://arrow.apache.org/rust/arrow_csv/writer/struct.Writer.html" class="struct" title="struct arrow_csv::writer::Writer">Writer</a>\<W\>

<a href="https://arrow.apache.org/rust/arrow_csv/writer/struct.Writer.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://arrow.apache.org/rust/arrow_csv/writer/struct.Writer.html#impl-RecordBatchWriter-for-Writer%3CW%3E" class="anchor">Â§</a>

### impl\<W: <a href="https://doc.rust-lang.org/nightly/std/io/trait.Write.html" class="trait" title="trait std::io::Write">Write</a>\> RecordBatchWriter for <a href="https://arrow.apache.org/rust/arrow_csv/writer/struct.Writer.html" class="struct" title="struct arrow_csv::writer::Writer">Writer</a>\<W\>

<a href="https://arrow.apache.org/rust/arrow_csv/writer/struct.Writer.html#method.write-1" class="anchor">Â§</a>

#### fn write(&mut self, batch: &RecordBatch) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, ArrowError\>

Write a single batch to the writer.

<a href="https://arrow.apache.org/rust/arrow_csv/writer/struct.Writer.html#method.close" class="anchor">Â§</a>

#### fn close(self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, ArrowError\>

Write footer or termination data, then mark the writer as done.

## Auto Trait Implementations<a href="https://arrow.apache.org/rust/arrow_csv/writer/struct.Writer.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://arrow.apache.org/rust/arrow_csv/writer/struct.Writer.html#blanket-implementations" class="anchor">Â§</a>
