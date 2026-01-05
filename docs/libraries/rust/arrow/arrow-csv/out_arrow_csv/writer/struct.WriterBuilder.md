# Struct WriterBuilder Copy item path

<a href="https://arrow.apache.org/rust/src/arrow_csv/writer.rs.html#191-214" class="src">Source</a>

``` rust
pub struct WriterBuilder {
    delimiter: u8,
    has_header: bool,
    quote: u8,
    escape: u8,
    double_quote: bool,
    date_format: Option<String>,
    datetime_format: Option<String>,
    timestamp_format: Option<String>,
    timestamp_tz_format: Option<String>,
    time_format: Option<String>,
    null_value: Option<String>,
}
```

Expand description

A CSV writer builder

## Fields<a href="https://arrow.apache.org/rust/arrow_csv/writer/struct.WriterBuilder.html#fields" class="anchor">Â§</a>

<a href="https://arrow.apache.org/rust/arrow_csv/writer/struct.WriterBuilder.html#structfield.delimiter" class="anchor field">Â§</a>`delimiter: `<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive"><code>u8</code></a>

Optional column delimiter. Defaults to `b','`

<a href="https://arrow.apache.org/rust/arrow_csv/writer/struct.WriterBuilder.html#structfield.has_header" class="anchor field">Â§</a>`has_header: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Whether to write column names as file headers. Defaults to `true`

<a href="https://arrow.apache.org/rust/arrow_csv/writer/struct.WriterBuilder.html#structfield.quote" class="anchor field">Â§</a>`quote: `<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive"><code>u8</code></a>

Optional quote character. Defaults to `b'"'`

<a href="https://arrow.apache.org/rust/arrow_csv/writer/struct.WriterBuilder.html#structfield.escape" class="anchor field">Â§</a>`escape: `<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive"><code>u8</code></a>

Optional escape character. Defaults to `b'\\'`

<a href="https://arrow.apache.org/rust/arrow_csv/writer/struct.WriterBuilder.html#structfield.double_quote" class="anchor field">Â§</a>`double_quote: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Enable double quote escapes. Defaults to `true`

<a href="https://arrow.apache.org/rust/arrow_csv/writer/struct.WriterBuilder.html#structfield.date_format" class="anchor field">Â§</a>`date_format: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

Optional date format for date arrays

<a href="https://arrow.apache.org/rust/arrow_csv/writer/struct.WriterBuilder.html#structfield.datetime_format" class="anchor field">Â§</a>`datetime_format: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

Optional datetime format for datetime arrays

<a href="https://arrow.apache.org/rust/arrow_csv/writer/struct.WriterBuilder.html#structfield.timestamp_format" class="anchor field">Â§</a>`timestamp_format: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

Optional timestamp format for timestamp arrays

<a href="https://arrow.apache.org/rust/arrow_csv/writer/struct.WriterBuilder.html#structfield.timestamp_tz_format" class="anchor field">Â§</a>`timestamp_tz_format: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

Optional timestamp format for timestamp with timezone arrays

<a href="https://arrow.apache.org/rust/arrow_csv/writer/struct.WriterBuilder.html#structfield.time_format" class="anchor field">Â§</a>`time_format: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

Optional time format for time arrays

<a href="https://arrow.apache.org/rust/arrow_csv/writer/struct.WriterBuilder.html#structfield.null_value" class="anchor field">Â§</a>`null_value: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

Optional value to represent null

## Implementations<a href="https://arrow.apache.org/rust/arrow_csv/writer/struct.WriterBuilder.html#implementations" class="anchor">Â§</a>

<a href="https://arrow.apache.org/rust/arrow_csv/writer/struct.WriterBuilder.html#impl-WriterBuilder" class="anchor">Â§</a>

### impl <a href="https://arrow.apache.org/rust/arrow_csv/writer/struct.WriterBuilder.html" class="struct" title="struct arrow_csv::writer::WriterBuilder">WriterBuilder</a>

#### pub fn <a href="https://arrow.apache.org/rust/arrow_csv/writer/struct.WriterBuilder.html#method.new" class="fn">new</a>() -\> Self

Create a new builder for configuring CSV writing options.

To convert a builder into a writer, call `WriterBuilder::build`

##### <a href="https://arrow.apache.org/rust/arrow_csv/writer/struct.WriterBuilder.html#example" class="doc-anchor">Â§</a>Example

``` rust

fn example() -> Writer<File> {
    let file = File::create("target/out.csv").unwrap();

    // create a builder that doesn't write headers
    let builder = WriterBuilder::new().with_header(false);
    let writer = builder.build(file);

    writer
}
```

#### pub fn <a href="https://arrow.apache.org/rust/arrow_csv/writer/struct.WriterBuilder.html#method.with_header" class="fn">with_header</a>(self, header: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> Self

Set whether to write the CSV file with a header

#### pub fn <a href="https://arrow.apache.org/rust/arrow_csv/writer/struct.WriterBuilder.html#method.header" class="fn">header</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns `true` if this writer is configured to write a header

#### pub fn <a href="https://arrow.apache.org/rust/arrow_csv/writer/struct.WriterBuilder.html#method.with_delimiter" class="fn">with_delimiter</a>(self, delimiter: <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>) -\> Self

Set the CSV fileâ€™s column delimiter as a byte character

#### pub fn <a href="https://arrow.apache.org/rust/arrow_csv/writer/struct.WriterBuilder.html#method.delimiter" class="fn">delimiter</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>

Get the CSV fileâ€™s column delimiter as a byte character

#### pub fn <a href="https://arrow.apache.org/rust/arrow_csv/writer/struct.WriterBuilder.html#method.with_quote" class="fn">with_quote</a>(self, quote: <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>) -\> Self

Set the CSV fileâ€™s quote character as a byte character

#### pub fn <a href="https://arrow.apache.org/rust/arrow_csv/writer/struct.WriterBuilder.html#method.quote" class="fn">quote</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>

Get the CSV fileâ€™s quote character as a byte character

#### pub fn <a href="https://arrow.apache.org/rust/arrow_csv/writer/struct.WriterBuilder.html#method.with_escape" class="fn">with_escape</a>(self, escape: <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>) -\> Self

Set the CSV fileâ€™s escape character as a byte character

In some variants of CSV, quotes are escaped using a special escape character like `\` (instead of escaping quotes by doubling them).

By default, writing these idiosyncratic escapes is disabled, and is only used when `double_quote` is disabled.

#### pub fn <a href="https://arrow.apache.org/rust/arrow_csv/writer/struct.WriterBuilder.html#method.escape" class="fn">escape</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>

Get the CSV fileâ€™s escape character as a byte character

#### pub fn <a href="https://arrow.apache.org/rust/arrow_csv/writer/struct.WriterBuilder.html#method.with_double_quote" class="fn">with_double_quote</a>(self, double_quote: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> Self

Set whether to enable double quote escapes

When enabled (which is the default), quotes are escaped by doubling them. e.g., `"` escapes to `""`.

When disabled, quotes are escaped with the escape character (which is `\\` by default).

#### pub fn <a href="https://arrow.apache.org/rust/arrow_csv/writer/struct.WriterBuilder.html#method.double_quote" class="fn">double_quote</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Get whether double quote escapes are enabled

#### pub fn <a href="https://arrow.apache.org/rust/arrow_csv/writer/struct.WriterBuilder.html#method.with_date_format" class="fn">with_date_format</a>(self, format: <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>) -\> Self

Set the CSV fileâ€™s date format

#### pub fn <a href="https://arrow.apache.org/rust/arrow_csv/writer/struct.WriterBuilder.html#method.date_format" class="fn">date_format</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>

Get the CSV fileâ€™s date format if set, defaults to RFC3339

#### pub fn <a href="https://arrow.apache.org/rust/arrow_csv/writer/struct.WriterBuilder.html#method.with_datetime_format" class="fn">with_datetime_format</a>(self, format: <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>) -\> Self

Set the CSV fileâ€™s datetime format

#### pub fn <a href="https://arrow.apache.org/rust/arrow_csv/writer/struct.WriterBuilder.html#method.datetime_format" class="fn">datetime_format</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>

Get the CSV fileâ€™s datetime format if set, defaults to RFC3339

#### pub fn <a href="https://arrow.apache.org/rust/arrow_csv/writer/struct.WriterBuilder.html#method.with_time_format" class="fn">with_time_format</a>(self, format: <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>) -\> Self

Set the CSV fileâ€™s time format

#### pub fn <a href="https://arrow.apache.org/rust/arrow_csv/writer/struct.WriterBuilder.html#method.time_format" class="fn">time_format</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>

Get the CSV fileâ€™s datetime time if set, defaults to RFC3339

#### pub fn <a href="https://arrow.apache.org/rust/arrow_csv/writer/struct.WriterBuilder.html#method.with_timestamp_format" class="fn">with_timestamp_format</a>(self, format: <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>) -\> Self

Set the CSV fileâ€™s timestamp format

#### pub fn <a href="https://arrow.apache.org/rust/arrow_csv/writer/struct.WriterBuilder.html#method.timestamp_format" class="fn">timestamp_format</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>

Get the CSV fileâ€™s timestamp format if set, defaults to RFC3339

#### pub fn <a href="https://arrow.apache.org/rust/arrow_csv/writer/struct.WriterBuilder.html#method.with_timestamp_tz_format" class="fn">with_timestamp_tz_format</a>(self, tz_format: <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>) -\> Self

Set the CSV fileâ€™s timestamp tz format

#### pub fn <a href="https://arrow.apache.org/rust/arrow_csv/writer/struct.WriterBuilder.html#method.timestamp_tz_format" class="fn">timestamp_tz_format</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>

Get the CSV fileâ€™s timestamp tz format if set, defaults to RFC3339

#### pub fn <a href="https://arrow.apache.org/rust/arrow_csv/writer/struct.WriterBuilder.html#method.with_null" class="fn">with_null</a>(self, null_value: <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>) -\> Self

Set the value to represent null in output

#### pub fn <a href="https://arrow.apache.org/rust/arrow_csv/writer/struct.WriterBuilder.html#method.null" class="fn">null</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

Get the value to represent null in output

#### pub fn <a href="https://arrow.apache.org/rust/arrow_csv/writer/struct.WriterBuilder.html#method.build" class="fn">build</a>\<W: <a href="https://doc.rust-lang.org/nightly/std/io/trait.Write.html" class="trait" title="trait std::io::Write">Write</a>\>(self, writer: W) -\> <a href="https://arrow.apache.org/rust/arrow_csv/writer/struct.Writer.html" class="struct" title="struct arrow_csv::writer::Writer">Writer</a>\<W\>

Create a new `Writer`

## Trait Implementations<a href="https://arrow.apache.org/rust/arrow_csv/writer/struct.WriterBuilder.html#trait-implementations" class="anchor">Â§</a>

<a href="https://arrow.apache.org/rust/arrow_csv/writer/struct.WriterBuilder.html#impl-Clone-for-WriterBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://arrow.apache.org/rust/arrow_csv/writer/struct.WriterBuilder.html" class="struct" title="struct arrow_csv::writer::WriterBuilder">WriterBuilder</a>

<a href="https://arrow.apache.org/rust/arrow_csv/writer/struct.WriterBuilder.html#method.clone" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://arrow.apache.org/rust/arrow_csv/writer/struct.WriterBuilder.html" class="struct" title="struct arrow_csv::writer::WriterBuilder">WriterBuilder</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://arrow.apache.org/rust/arrow_csv/writer/struct.WriterBuilder.html#method.clone_from" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://arrow.apache.org/rust/arrow_csv/writer/struct.WriterBuilder.html#impl-Debug-for-WriterBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://arrow.apache.org/rust/arrow_csv/writer/struct.WriterBuilder.html" class="struct" title="struct arrow_csv::writer::WriterBuilder">WriterBuilder</a>

<a href="https://arrow.apache.org/rust/arrow_csv/writer/struct.WriterBuilder.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://arrow.apache.org/rust/arrow_csv/writer/struct.WriterBuilder.html#impl-Default-for-WriterBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://arrow.apache.org/rust/arrow_csv/writer/struct.WriterBuilder.html" class="struct" title="struct arrow_csv::writer::WriterBuilder">WriterBuilder</a>

<a href="https://arrow.apache.org/rust/arrow_csv/writer/struct.WriterBuilder.html#method.default" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> Self

Returns the â€œdefault valueâ€? for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://arrow.apache.org/rust/arrow_csv/writer/struct.WriterBuilder.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://arrow.apache.org/rust/arrow_csv/writer/struct.WriterBuilder.html#blanket-implementations" class="anchor">Â§</a>
