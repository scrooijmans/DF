# Struct ReaderBuilder Copy item path

<a href="https://arrow.apache.org/rust/src/arrow_csv/reader/mod.rs.html#1058-1071" class="src">Source</a>

``` rust
pub struct ReaderBuilder {
    schema: SchemaRef,
    format: Format,
    batch_size: usize,
    bounds: Option<(usize, usize)>,
    projection: Option<Vec<usize>>,
}
```

Expand description

CSV file reader builder

## Fields<a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.ReaderBuilder.html#fields" class="anchor">Â§</a>

<a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.ReaderBuilder.html#structfield.schema" class="anchor field">Â§</a>`schema: SchemaRef`

Schema of the CSV file

<a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.ReaderBuilder.html#structfield.format" class="anchor field">Â§</a>`format: `<a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.Format.html" class="struct" title="struct arrow_csv::reader::Format"><code>Format</code></a>

Format of the CSV file

<a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.ReaderBuilder.html#structfield.batch_size" class="anchor field">Â§</a>`batch_size: `<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a>

Batch size (number of records to load each time)

The default batch size when using the `ReaderBuilder` is 1024 records

<a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.ReaderBuilder.html#structfield.bounds" class="anchor field">Â§</a>`bounds: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<(`<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a>`, `<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a>`)>`

The bounds over which to scan the reader. `None` starts from 0 and runs until EOF.

<a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.ReaderBuilder.html#structfield.projection" class="anchor field">Â§</a>`projection: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a>`>>`

Optional projection for which columns to load (zero-based column indices)

## Implementations<a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.ReaderBuilder.html#implementations" class="anchor">Â§</a>

<a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.ReaderBuilder.html#impl-ReaderBuilder" class="anchor">Â§</a>

### impl <a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.ReaderBuilder.html" class="struct" title="struct arrow_csv::reader::ReaderBuilder">ReaderBuilder</a>

#### pub fn <a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.ReaderBuilder.html#method.new" class="fn">new</a>(schema: SchemaRef) -\> <a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.ReaderBuilder.html" class="struct" title="struct arrow_csv::reader::ReaderBuilder">ReaderBuilder</a>

Create a new builder for configuring CSV parsing options.

To convert a builder into a reader, call `ReaderBuilder::build`

##### <a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.ReaderBuilder.html#example" class="doc-anchor">Â§</a>Example

``` rust
let mut file = File::open("test/data/uk_cities_with_headers.csv").unwrap();
// Infer the schema with the first 100 records
let (schema, _) = Format::default().infer_schema(&mut file, Some(100)).unwrap();
file.rewind().unwrap();

// create a builder
ReaderBuilder::new(Arc::new(schema)).build(file).unwrap();
```

#### pub fn <a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.ReaderBuilder.html#method.with_header" class="fn">with_header</a>(self, has_header: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> Self

Set whether the CSV file has a header

#### pub fn <a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.ReaderBuilder.html#method.with_format" class="fn">with_format</a>(self, format: <a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.Format.html" class="struct" title="struct arrow_csv::reader::Format">Format</a>) -\> Self

Overrides the [Format](https://arrow.apache.org/rust/arrow_csv/reader/struct.Format.html "struct arrow_csv::reader::Format") of this [ReaderBuilder](https://arrow.apache.org/rust/arrow_csv/reader/struct.ReaderBuilder.html "struct arrow_csv::reader::ReaderBuilder")

#### pub fn <a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.ReaderBuilder.html#method.with_delimiter" class="fn">with_delimiter</a>(self, delimiter: <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>) -\> Self

Set the CSV fileâ€™s column delimiter as a byte character

#### pub fn <a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.ReaderBuilder.html#method.with_escape" class="fn">with_escape</a>(self, escape: <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>) -\> Self

Set the given character as the CSV fileâ€™s escape character

#### pub fn <a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.ReaderBuilder.html#method.with_quote" class="fn">with_quote</a>(self, quote: <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>) -\> Self

Set the given character as the CSV fileâ€™s quote character, by default it is double quote

#### pub fn <a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.ReaderBuilder.html#method.with_terminator" class="fn">with_terminator</a>(self, terminator: <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>) -\> Self

Provide a custom terminator character, defaults to CRLF

#### pub fn <a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.ReaderBuilder.html#method.with_comment" class="fn">with_comment</a>(self, comment: <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>) -\> Self

Provide a comment character, lines starting with this character will be ignored

#### pub fn <a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.ReaderBuilder.html#method.with_null_regex" class="fn">with_null_regex</a>(self, null_regex: Regex) -\> Self

Provide a regex to match null values, defaults to `^$`

#### pub fn <a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.ReaderBuilder.html#method.with_batch_size" class="fn">with_batch_size</a>(self, batch_size: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> Self

Set the batch size (number of records to load at one time)

#### pub fn <a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.ReaderBuilder.html#method.with_bounds" class="fn">with_bounds</a>(self, start: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, end: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> Self

Set the bounds over which to scan the reader. `start` and `end` are line numbers.

#### pub fn <a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.ReaderBuilder.html#method.with_projection" class="fn">with_projection</a>(self, projection: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>) -\> Self

Set the readerâ€™s column projection

#### pub fn <a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.ReaderBuilder.html#method.with_truncated_rows" class="fn">with_truncated_rows</a>(self, allow: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> Self

Whether to allow truncated rows when parsing.

By default this is set to `false` and will error if the CSV rows have different lengths. When set to true then it will allow records with less than the expected number of columns and fill the missing columns with nulls. If the recordâ€™s schema is not nullable, then it will still return an error.

#### pub fn <a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.ReaderBuilder.html#method.build" class="fn">build</a>\<R: <a href="https://doc.rust-lang.org/nightly/std/io/trait.Read.html" class="trait" title="trait std::io::Read">Read</a>\>(self, reader: R) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://arrow.apache.org/rust/arrow_csv/reader/type.Reader.html" class="type" title="type arrow_csv::reader::Reader">Reader</a>\<R\>, ArrowError\>

Create a new `Reader` from a non-buffered reader

If `R: BufRead` consider using [`Self::build_buffered`](https://arrow.apache.org/rust/arrow_csv/reader/struct.ReaderBuilder.html#method.build_buffered "method arrow_csv::reader::ReaderBuilder::build_buffered") to avoid unnecessary additional buffering, as internally this method wraps `reader` in [`std::io::BufReader`](https://doc.rust-lang.org/nightly/std/io/buffered/bufreader/struct.BufReader.html "struct std::io::buffered::bufreader::BufReader")

#### pub fn <a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.ReaderBuilder.html#method.build_buffered" class="fn">build_buffered</a>\<R: <a href="https://doc.rust-lang.org/nightly/std/io/trait.BufRead.html" class="trait" title="trait std::io::BufRead">BufRead</a>\>( self, reader: R, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.BufReader.html" class="struct" title="struct arrow_csv::reader::BufReader">BufReader</a>\<R\>, ArrowError\>

Create a new `BufReader` from a buffered reader

#### pub fn <a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.ReaderBuilder.html#method.build_decoder" class="fn">build_decoder</a>(self) -\> <a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.Decoder.html" class="struct" title="struct arrow_csv::reader::Decoder">Decoder</a>

Builds a decoder that can be used to decode CSV from an arbitrary byte stream

## Trait Implementations<a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.ReaderBuilder.html#trait-implementations" class="anchor">Â§</a>

<a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.ReaderBuilder.html#impl-Debug-for-ReaderBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.ReaderBuilder.html" class="struct" title="struct arrow_csv::reader::ReaderBuilder">ReaderBuilder</a>

<a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.ReaderBuilder.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations<a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.ReaderBuilder.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.ReaderBuilder.html#blanket-implementations" class="anchor">Â§</a>
