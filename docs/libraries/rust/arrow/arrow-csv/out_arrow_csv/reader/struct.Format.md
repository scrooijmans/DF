# Struct Format Copy item path

<a href="https://arrow.apache.org/rust/src/arrow_csv/reader/mod.rs.html#234-243" class="src">Source</a>

``` rust
pub struct Format {
    header: bool,
    delimiter: Option<u8>,
    escape: Option<u8>,
    quote: Option<u8>,
    terminator: Option<u8>,
    comment: Option<u8>,
    null_regex: NullRegex,
    truncated_rows: bool,
}
```

Expand description

The format specification for the CSV file

## Fields<a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.Format.html#fields" class="anchor">Â§</a>

<a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.Format.html#structfield.header" class="anchor field">Â§</a>`header: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a><a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.Format.html#structfield.delimiter" class="anchor field">Â§</a>`delimiter: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive"><code>u8</code></a>`>`<a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.Format.html#structfield.escape" class="anchor field">Â§</a>`escape: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive"><code>u8</code></a>`>`<a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.Format.html#structfield.quote" class="anchor field">Â§</a>`quote: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive"><code>u8</code></a>`>`<a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.Format.html#structfield.terminator" class="anchor field">Â§</a>`terminator: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive"><code>u8</code></a>`>`<a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.Format.html#structfield.comment" class="anchor field">Â§</a>`comment: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive"><code>u8</code></a>`>`<a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.Format.html#structfield.null_regex" class="anchor field">Â§</a>`null_regex: `<a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.NullRegex.html" class="struct" title="struct arrow_csv::reader::NullRegex"><code>NullRegex</code></a><a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.Format.html#structfield.truncated_rows" class="anchor field">Â§</a>`truncated_rows: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

## Implementations<a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.Format.html#implementations" class="anchor">Â§</a>

<a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.Format.html#impl-Format" class="anchor">Â§</a>

### impl <a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.Format.html" class="struct" title="struct arrow_csv::reader::Format">Format</a>

#### pub fn <a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.Format.html#method.with_header" class="fn">with_header</a>(self, has_header: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> Self

Specify whether the CSV file has a header, defaults to `false`

When `true`, the first row of the CSV file is treated as a header row

#### pub fn <a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.Format.html#method.with_delimiter" class="fn">with_delimiter</a>(self, delimiter: <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>) -\> Self

Specify a custom delimiter character, defaults to comma `','`

#### pub fn <a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.Format.html#method.with_escape" class="fn">with_escape</a>(self, escape: <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>) -\> Self

Specify an escape character, defaults to `None`

#### pub fn <a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.Format.html#method.with_quote" class="fn">with_quote</a>(self, quote: <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>) -\> Self

Specify a custom quote character, defaults to double quote `'"'`

#### pub fn <a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.Format.html#method.with_terminator" class="fn">with_terminator</a>(self, terminator: <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>) -\> Self

Specify a custom terminator character, defaults to CRLF

#### pub fn <a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.Format.html#method.with_comment" class="fn">with_comment</a>(self, comment: <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>) -\> Self

Specify a comment character, defaults to `None`

Lines starting with this character will be ignored

#### pub fn <a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.Format.html#method.with_null_regex" class="fn">with_null_regex</a>(self, null_regex: Regex) -\> Self

Provide a regex to match null values, defaults to `^$`

#### pub fn <a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.Format.html#method.with_truncated_rows" class="fn">with_truncated_rows</a>(self, allow: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> Self

Whether to allow truncated rows when parsing.

By default this is set to `false` and will error if the CSV rows have different lengths. When set to true then it will allow records with less than the expected number of columns and fill the missing columns with nulls. If the recordâ€™s schema is not nullable, then it will still return an error.

#### pub fn <a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.Format.html#method.infer_schema" class="fn">infer_schema</a>\<R: <a href="https://doc.rust-lang.org/nightly/std/io/trait.Read.html" class="trait" title="trait std::io::Read">Read</a>\>( &self, reader: R, max_records: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<(Schema, <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>), ArrowError\>

Infer schema of CSV records from the provided `reader`

If `max_records` is `None`, all records will be read, otherwise up to `max_records` records are read to infer the schema

Returns inferred schema and number of records read

#### fn <a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.Format.html#method.build_reader" class="fn">build_reader</a>\<R: <a href="https://doc.rust-lang.org/nightly/std/io/trait.Read.html" class="trait" title="trait std::io::Read">Read</a>\>(&self, reader: R) -\> Reader\<R\>

Build a \[`csv::Reader`\] for this [`Format`](https://arrow.apache.org/rust/arrow_csv/reader/struct.Format.html "struct arrow_csv::reader::Format")

#### fn <a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.Format.html#method.build_parser" class="fn">build_parser</a>(&self) -\> Reader

Build a \[`csv_core::Reader`\] for this [`Format`](https://arrow.apache.org/rust/arrow_csv/reader/struct.Format.html "struct arrow_csv::reader::Format")

## Trait Implementations<a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.Format.html#trait-implementations" class="anchor">Â§</a>

<a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.Format.html#impl-Clone-for-Format" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.Format.html" class="struct" title="struct arrow_csv::reader::Format">Format</a>

<a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.Format.html#method.clone" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.Format.html" class="struct" title="struct arrow_csv::reader::Format">Format</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.Format.html#method.clone_from" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.Format.html#impl-Debug-for-Format" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.Format.html" class="struct" title="struct arrow_csv::reader::Format">Format</a>

<a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.Format.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.Format.html#impl-Default-for-Format" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.Format.html" class="struct" title="struct arrow_csv::reader::Format">Format</a>

<a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.Format.html#method.default" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.Format.html" class="struct" title="struct arrow_csv::reader::Format">Format</a>

Returns the â€œdefault valueâ€? for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.Format.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.Format.html#blanket-implementations" class="anchor">Â§</a>
