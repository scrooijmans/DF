# Struct WriterBuilder Copy item path

<a href="https://docs.rs/arrow-csv/56.0.0/x86_64-unknown-linux-gnu/src/arrow_csv/writer.rs.html#191" class="src">Source</a>

``` rust
pub struct WriterBuilder { /* private fields */ }
```

Expand description

A CSV writer builder

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/csv/writer/struct.WriterBuilder.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/csv/writer/struct.WriterBuilder.html#impl-WriterBuilder" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/csv/struct.WriterBuilder.html" class="struct" title="struct datafusion::common::arrow::csv::WriterBuilder">WriterBuilder</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/csv/writer/struct.WriterBuilder.html#method.new" class="fn">new</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/csv/struct.WriterBuilder.html" class="struct" title="struct datafusion::common::arrow::csv::WriterBuilder">WriterBuilder</a>

Create a new builder for configuring CSV writing options.

To convert a builder into a writer, call `WriterBuilder::build`

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/csv/writer/struct.WriterBuilder.html#example" class="doc-anchor">§</a>Example

``` rust

fn example() -> Writer<File> {
    let file = File::create("target/out.csv").unwrap();

    // create a builder that doesn't write headers
    let builder = WriterBuilder::new().with_header(false);
    let writer = builder.build(file);

    writer
}
```

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/csv/writer/struct.WriterBuilder.html#method.with_header" class="fn">with_header</a>(self, header: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/csv/struct.WriterBuilder.html" class="struct" title="struct datafusion::common::arrow::csv::WriterBuilder">WriterBuilder</a>

Set whether to write the CSV file with a header

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/csv/writer/struct.WriterBuilder.html#method.header" class="fn">header</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns `true` if this writer is configured to write a header

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/csv/writer/struct.WriterBuilder.html#method.with_delimiter" class="fn">with_delimiter</a>(self, delimiter: <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/csv/struct.WriterBuilder.html" class="struct" title="struct datafusion::common::arrow::csv::WriterBuilder">WriterBuilder</a>

Set the CSV file’s column delimiter as a byte character

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/csv/writer/struct.WriterBuilder.html#method.delimiter" class="fn">delimiter</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>

Get the CSV file’s column delimiter as a byte character

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/csv/writer/struct.WriterBuilder.html#method.with_quote" class="fn">with_quote</a>(self, quote: <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/csv/struct.WriterBuilder.html" class="struct" title="struct datafusion::common::arrow::csv::WriterBuilder">WriterBuilder</a>

Set the CSV file’s quote character as a byte character

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/csv/writer/struct.WriterBuilder.html#method.quote" class="fn">quote</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>

Get the CSV file’s quote character as a byte character

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/csv/writer/struct.WriterBuilder.html#method.with_escape" class="fn">with_escape</a>(self, escape: <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/csv/struct.WriterBuilder.html" class="struct" title="struct datafusion::common::arrow::csv::WriterBuilder">WriterBuilder</a>

Set the CSV file’s escape character as a byte character

In some variants of CSV, quotes are escaped using a special escape character like `\` (instead of escaping quotes by doubling them).

By default, writing these idiosyncratic escapes is disabled, and is only used when `double_quote` is disabled.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/csv/writer/struct.WriterBuilder.html#method.escape" class="fn">escape</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>

Get the CSV file’s escape character as a byte character

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/csv/writer/struct.WriterBuilder.html#method.with_double_quote" class="fn">with_double_quote</a>(self, double_quote: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/csv/struct.WriterBuilder.html" class="struct" title="struct datafusion::common::arrow::csv::WriterBuilder">WriterBuilder</a>

Set whether to enable double quote escapes

When enabled (which is the default), quotes are escaped by doubling them. e.g., `"` escapes to `""`.

When disabled, quotes are escaped with the escape character (which is `\\` by default).

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/csv/writer/struct.WriterBuilder.html#method.double_quote" class="fn">double_quote</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Get whether double quote escapes are enabled

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/csv/writer/struct.WriterBuilder.html#method.with_date_format" class="fn">with_date_format</a>(self, format: <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/csv/struct.WriterBuilder.html" class="struct" title="struct datafusion::common::arrow::csv::WriterBuilder">WriterBuilder</a>

Set the CSV file’s date format

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/csv/writer/struct.WriterBuilder.html#method.date_format" class="fn">date_format</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>

Get the CSV file’s date format if set, defaults to RFC3339

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/csv/writer/struct.WriterBuilder.html#method.with_datetime_format" class="fn">with_datetime_format</a>(self, format: <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/csv/struct.WriterBuilder.html" class="struct" title="struct datafusion::common::arrow::csv::WriterBuilder">WriterBuilder</a>

Set the CSV file’s datetime format

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/csv/writer/struct.WriterBuilder.html#method.datetime_format" class="fn">datetime_format</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>

Get the CSV file’s datetime format if set, defaults to RFC3339

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/csv/writer/struct.WriterBuilder.html#method.with_time_format" class="fn">with_time_format</a>(self, format: <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/csv/struct.WriterBuilder.html" class="struct" title="struct datafusion::common::arrow::csv::WriterBuilder">WriterBuilder</a>

Set the CSV file’s time format

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/csv/writer/struct.WriterBuilder.html#method.time_format" class="fn">time_format</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>

Get the CSV file’s datetime time if set, defaults to RFC3339

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/csv/writer/struct.WriterBuilder.html#method.with_timestamp_format" class="fn">with_timestamp_format</a>(self, format: <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/csv/struct.WriterBuilder.html" class="struct" title="struct datafusion::common::arrow::csv::WriterBuilder">WriterBuilder</a>

Set the CSV file’s timestamp format

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/csv/writer/struct.WriterBuilder.html#method.timestamp_format" class="fn">timestamp_format</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>

Get the CSV file’s timestamp format if set, defaults to RFC3339

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/csv/writer/struct.WriterBuilder.html#method.with_timestamp_tz_format" class="fn">with_timestamp_tz_format</a>(self, tz_format: <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/csv/struct.WriterBuilder.html" class="struct" title="struct datafusion::common::arrow::csv::WriterBuilder">WriterBuilder</a>

Set the CSV file’s timestamp tz format

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/csv/writer/struct.WriterBuilder.html#method.timestamp_tz_format" class="fn">timestamp_tz_format</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>

Get the CSV file’s timestamp tz format if set, defaults to RFC3339

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/csv/writer/struct.WriterBuilder.html#method.with_null" class="fn">with_null</a>(self, null_value: <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/csv/struct.WriterBuilder.html" class="struct" title="struct datafusion::common::arrow::csv::WriterBuilder">WriterBuilder</a>

Set the value to represent null in output

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/csv/writer/struct.WriterBuilder.html#method.null" class="fn">null</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

Get the value to represent null in output

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/csv/writer/struct.WriterBuilder.html#method.build" class="fn">build</a>\<W\>(self, writer: W) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/csv/struct.Writer.html" class="struct" title="struct datafusion::common::arrow::csv::Writer">Writer</a>\<W\>

where W: <a href="https://doc.rust-lang.org/nightly/std/io/trait.Write.html" class="trait" title="trait std::io::Write">Write</a>,

Create a new `Writer`

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/csv/writer/struct.WriterBuilder.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/csv/writer/struct.WriterBuilder.html#impl-Clone-for-WriterBuilder" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/csv/struct.WriterBuilder.html" class="struct" title="struct datafusion::common::arrow::csv::WriterBuilder">WriterBuilder</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/csv/writer/struct.WriterBuilder.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/csv/struct.WriterBuilder.html" class="struct" title="struct datafusion::common::arrow::csv::WriterBuilder">WriterBuilder</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/csv/writer/struct.WriterBuilder.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/csv/writer/struct.WriterBuilder.html#impl-Debug-for-WriterBuilder" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/csv/struct.WriterBuilder.html" class="struct" title="struct datafusion::common::arrow::csv::WriterBuilder">WriterBuilder</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/csv/writer/struct.WriterBuilder.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/csv/writer/struct.WriterBuilder.html#impl-Default-for-WriterBuilder" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/csv/struct.WriterBuilder.html" class="struct" title="struct datafusion::common::arrow::csv::WriterBuilder">WriterBuilder</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/csv/writer/struct.WriterBuilder.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/csv/struct.WriterBuilder.html" class="struct" title="struct datafusion::common::arrow::csv::WriterBuilder">WriterBuilder</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/csv/writer/struct.WriterBuilder.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/csv/writer/struct.WriterBuilder.html#blanket-implementations" class="anchor">§</a>
